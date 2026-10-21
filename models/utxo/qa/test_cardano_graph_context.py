"""Ordinary source-derived checks: no graph/model/native command is executed."""
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from cardano_graph_context import load_graph_context


def write_json(path, value):
    path.write_text(json.dumps(value))


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


class GraphContextTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / "repo"
        self.root.mkdir()
        subprocess.run(["git", "init", "-q", str(self.root)], check=True)
        subprocess.run(["git", "-C", str(self.root), "-c", "user.name=fixture",
                        "-c", "user.email=fixture@example.invalid", "commit", "--allow-empty",
                        "-qm", "fixture"], check=True)
        self.head = subprocess.check_output(["git", "-C", str(self.root), "rev-parse", "HEAD"]).decode().strip()
        self.rel = "docs/utxo/doctorwatson/astra/cardano-design.md"
        self.source = self.root / self.rel
        self.source.parent.mkdir(parents=True)
        self.source.write_text("# Cardano\nDated source, not a ledger receipt.\n")
        # Keep graph outputs outside inventory scope, as in the actual explicit graph.
        self.out = Path(self.tmp.name) / "external-graph"
        self.out.mkdir()
        self.pin = digest(self.source)
        self.make_graph()

    def make_graph(self):
        manifest = {
            "root": str(self.root),
            "snapshots": [{"source_root": str(self.root), "git_commit": self.head}],
            "policy": {"excluded_components": [], "excluded_prefixes": [],
                       "private_binary_extensions": [], "supported_primary_text_extensions": [".md"]},
            "files": [{"source_file": str(self.source), "source_root": str(self.root),
                       "relative_path": self.rel, "sha256": self.pin,
                       "snapshot": "active_utxo_ba21"}],
        }
        node = {"id": "exact_cardano_heading", "source_file": self.rel,
                "source_location": "L1", "source_namespace": "active_utxo_ba21",
                "source_sha256": self.pin, "source_snapshot": "dated-origin",
                "source_status": "active_committed", "_origin": "structural"}
        write_json(self.out / "graph.json", {"nodes": [node], "links": []})
        write_json(self.out / "source-manifest.json", manifest)
        write_json(self.out / "semantic-coverage.json", [{"coverage": "partial"}])
        self.bind_identity()

    def bind_identity(self):
        write_json(self.out / "graph-identity.json", {
            "graph_sha256": digest(self.out / "graph.json"),
            "source_manifest_sha256": digest(self.out / "source-manifest.json"),
            "semantic_coverage_sha256": digest(self.out / "semantic-coverage.json"),
        })

    def load(self):
        return load_graph_context(self.root, self.out, environ={})

    def test_fresh_scope_returns_exact_source_handoff_and_inert_query(self):
        result = self.load()
        self.assertEqual(result["status"], "FRESH_FOR_PINNED_SCOPE")
        self.assertEqual(result["firstStageQuery"]["seed"], "exact_cardano_heading")
        self.assertEqual(result["handoffReferences"][0]["sourceFile"], str(self.source))
        self.assertEqual(result["handoffReferences"][0]["sourceLocation"], "L1")
        self.assertEqual(result["handoffReferences"][0]["sourceCurrentness"], "MATCH")
        self.assertIn("partial", result["semanticCoverage"])
        self.assertFalse(result["ledgerAcceptance"])
        self.assertFalse(result["firstStageQuery"]["budgetIsHardCap"])

    def test_explicit_argument_precedes_environment_and_default_never_uses_parent(self):
        result = load_graph_context(self.root, self.out, environ={"GRAPHIFY_OUT": "/missing/other"})
        self.assertEqual(result["resolution"], "argument")
        self.assertEqual(result["graphifyOut"], str(self.out))
        result = load_graph_context(self.root, environ={"GRAPHIFY_OUT": str(self.out)})
        self.assertEqual(result["resolution"], "GRAPHIFY_OUT")
        result = load_graph_context(self.root, environ={})
        self.assertEqual(result["graphifyOut"], str(self.root / "graphify-out"))
        self.assertEqual(result["status"], "STALE")
        self.assertIsNone(result["firstStageQuery"])

    def test_changed_source_preserves_dated_origin_and_stale_qualification(self):
        self.source.write_text("Changed source\n")
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        self.assertEqual(result["firstStageQuery"]["mode"], "dated-discovery-only")
        ref = result["handoffReferences"][0]
        self.assertEqual(ref["sourceSnapshot"], "dated-origin")
        self.assertEqual(ref["sourceSha256"], self.pin)
        self.assertEqual(ref["sourceCurrentness"], "CHANGED_OR_MISSING")
        self.assertIn("body-changed", result["nextAction"])
        self.assertIn(str(self.source), result["nextAction"])

    def install_source_scope(self):
        ref = self.load()["handoffReferences"][0]
        graph = json.loads((self.out / "graph.json").read_text())
        # New scoped nodes carry absolute source locations, as the increment does.
        graph["nodes"][0]["source_file"] = str(self.source)
        write_json(self.out / "graph.json", graph)
        self.bind_identity()
        write_json(self.out / "cardano-first-proof-scope.json", {
            "format": "cardano-first-proof-source-scope/1", "namespace": "new-source-scope",
            "seed": ref["nodeId"], "handoff_references": [ref],
            "sources": [{"source_file": str(self.source), "source_root": str(self.root),
                         "sha256": self.pin}],
        })
        identity = json.loads((self.out / "graph-identity.json").read_text())
        identity["cardano_first_proof_scope_sha256"] = digest(self.out / "cardano-first-proof-scope.json")
        write_json(self.out / "graph-identity.json", identity)

    def test_fresh_explicit_scope_never_makes_inherited_graph_fresh(self):
        self.install_source_scope()
        (self.root / "uncovered-new.md").write_text("outside new scope")
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        self.assertEqual(result["sourceScope"]["status"], "FRESH_FOR_EXPLICIT_SOURCE_SCOPE")
        self.assertFalse(result["sourceScope"]["wholeGraphFresh"])
        self.assertEqual(result["firstStageQuery"]["mode"], "current-explicit-source-scope-only")

    def test_changed_scoped_source_reverts_to_dated_navigation(self):
        self.install_source_scope()
        self.source.write_text("changed scoped body")
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "STALE")
        self.assertIsNone(result["firstStageQuery"])

    def test_scope_node_mismatch_cannot_supply_current_query(self):
        self.install_source_scope()
        path = self.out / "cardano-first-proof-scope.json"
        scope = json.loads(path.read_text())
        scope["handoff_references"][0]["nodeId"] = "invented"
        write_json(path, scope)
        identity = json.loads((self.out / "graph-identity.json").read_text())
        identity["cardano_first_proof_scope_sha256"] = digest(path)
        write_json(self.out / "graph-identity.json", identity)
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "STALE")
        self.assertIsNone(result["firstStageQuery"])

    def install_named_scope(self, kind, source, source_root, node_id, pin=None):
        # Independent fixture declarations for the three published sidecars.
        names = {
            "legacy": ("cardano-first-proof-scope.json", "cardano_first_proof_scope_sha256",
                       "cardano-first-proof-source-scope/1"),
            "protocol": ("cardano-protocol-planner-evaluation-scope.json",
                         "cardano_protocol_planner_evaluation_scope_sha256",
                         "cardano-protocol-planner-evaluation-source-scope/1"),
            "current": ("cardano-preview-cli-filebinding-scope.json",
                        "cardano_preview_cli_filebinding_scope_sha256",
                        "cardano-preview-cli-filebinding-source-scope/1"),
        }
        name, key, format_name = names[kind]
        pin = digest(source) if pin is None else pin
        graph = json.loads((self.out / "graph.json").read_text())
        graph["nodes"].append({"id": node_id, "source_file": str(source),
                               "source_sha256": pin, "source_location": "L1",
                               "source_namespace": kind, "source_snapshot": kind,
                               "source_status": "fixture-source"})
        write_json(self.out / "graph.json", graph)
        ref = {"nodeId": node_id, "sourceFile": str(source), "sourceSha256": pin,
               "sourceLocation": "L1", "sourceSnapshot": kind, "sourceStatus": "fixture-source",
               "currentSourceSha256": pin, "sourceCurrentness": "MATCH", "readRequired": True}
        write_json(self.out / name, {"format": format_name, "namespace": kind,
                                    "seed": node_id, "handoff_references": [ref],
                                    "sources": [{"source_file": str(source),
                                                 "source_root": str(source_root), "sha256": pin}]})
        identity = json.loads((self.out / "graph-identity.json").read_text())
        identity["graph_sha256"] = digest(self.out / "graph.json")
        identity[key] = digest(self.out / name)
        write_json(self.out / "graph-identity.json", identity)

    def other_checkout_source(self):
        root = Path(self.tmp.name) / "other-checkout"
        root.mkdir()
        source = root / "source.md"
        source.write_text("Matching bytes in another checkout\n")
        return root, source

    def test_current_checkout_uses_newest_matching_scope_and_corrected_query(self):
        other_root, other_source = self.other_checkout_source()
        self.install_named_scope("legacy", other_source, other_root, "w17")
        self.install_named_scope("protocol", self.source, self.root, "i6-old", pin="0" * 64)
        self.install_named_scope("current", self.source, self.root, "i6-current")
        (self.root / "uncovered.md").write_text("outside the explicit source scope")
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        self.assertEqual(result["sourceScope"]["status"], "FRESH_FOR_EXPLICIT_SOURCE_SCOPE")
        self.assertTrue(result["sourceScope"]["matchesCheckout"])
        self.assertEqual(result["firstStageQuery"]["seed"], "i6-current")
        self.assertEqual(result["firstStageQuery"]["question"],
                         "inspect protocol prepare plan evaluation remote request")
        self.assertEqual(result["firstStageQuery"]["mode"], "current-explicit-source-scope-only")
        self.assertEqual(len(result["sourceScope"]["candidateScopes"]), 3)
        self.assertFalse(result["ledgerAcceptance"])
        self.assertFalse(result["sourceScope"]["wholeGraphFresh"])
        self.assertIn("partial", result["semanticCoverage"])

    def test_legacy_scope_remains_current_for_its_own_checkout(self):
        self.install_named_scope("legacy", self.source, self.root, "w17-own-root")
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "FRESH_FOR_EXPLICIT_SOURCE_SCOPE")
        self.assertEqual(result["firstStageQuery"]["seed"], "w17-own-root")
        self.assertTrue(result["firstStageQuery"]["question"].startswith("Cardano Groth16"))

    def test_fresh_other_checkout_scope_is_never_current_here(self):
        other_root, other_source = self.other_checkout_source()
        self.install_named_scope("legacy", other_source, other_root, "w17")
        (self.root / "uncovered.md").write_text("keep global scope explicitly stale")
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "STALE")
        self.assertFalse(result["sourceScope"]["matchesCheckout"])
        self.assertIn("source-scope-root-mismatch", result["nextAction"])
        self.assertEqual(result["firstStageQuery"]["mode"], "dated-discovery-only")

    def test_changed_current_source_cannot_fall_back_to_unrelated_fresh_legacy(self):
        other_root, other_source = self.other_checkout_source()
        self.install_named_scope("legacy", other_source, other_root, "w17")
        self.install_named_scope("current", self.source, self.root, "i6")
        self.source.write_text("changed current-checkout bytes\n")
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "STALE")
        self.assertTrue(result["sourceScope"]["matchesCheckout"])
        self.assertTrue(result["sourceScope"]["scopeFile"].endswith("cardano-preview-cli-filebinding-scope.json"))
        self.assertEqual(result["firstStageQuery"]["mode"], "dated-discovery-only")
        self.assertNotEqual(result["firstStageQuery"]["seed"], "w17")

    def test_null_scoped_digest_cannot_qualify_a_missing_source(self):
        original = self.source.read_bytes()
        for exists in (True, False):
            with self.subTest(source_exists=exists):
                self.source.write_bytes(original)
                node_id = "null-body-" + str(exists)
                self.install_named_scope("current", self.source, self.root, node_id)
                path = self.out / "cardano-preview-cli-filebinding-scope.json"
                scope = json.loads(path.read_text())
                scope["sources"][0]["sha256"] = None
                scope["handoff_references"][0]["sourceSha256"] = None
                write_json(path, scope)
                graph = json.loads((self.out / "graph.json").read_text())
                next(node for node in graph["nodes"] if node["id"] == node_id)["source_sha256"] = None
                write_json(self.out / "graph.json", graph)
                identity = json.loads((self.out / "graph-identity.json").read_text())
                identity["graph_sha256"] = digest(self.out / "graph.json")
                identity["cardano_preview_cli_filebinding_scope_sha256"] = digest(path)
                write_json(self.out / "graph-identity.json", identity)
                if not exists:
                    self.source.unlink()
                result = self.load()
                self.assertEqual(result["sourceScope"]["status"], "STALE")
                query = result["firstStageQuery"]
                self.assertTrue(query is None or query["mode"] != "current-explicit-source-scope-only")
                self.assertTrue(query is None or query["seed"] != node_id)

    def test_symlink_scope_with_null_identity_cannot_supply_current_scope(self):
        self.install_named_scope("current", self.source, self.root, "unbound-scope")
        path = self.out / "cardano-preview-cli-filebinding-scope.json"
        target = self.out / "linked-scope.json"
        path.rename(target)
        path.symlink_to(target.name)
        identity = json.loads((self.out / "graph-identity.json").read_text())
        identity["cardano_preview_cli_filebinding_scope_sha256"] = None
        write_json(self.out / "graph-identity.json", identity)
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "STALE")
        query = result["firstStageQuery"]
        self.assertTrue(query is None or query["mode"] != "current-explicit-source-scope-only")

    def test_symlink_artifact_with_null_identity_cannot_supply_bound_query(self):
        self.install_named_scope("current", self.source, self.root, "unbound-graph")
        for name, key in (("graph.json", "graph_sha256"),
                          ("source-manifest.json", "source_manifest_sha256"),
                          ("semantic-coverage.json", "semantic_coverage_sha256")):
            with self.subTest(artifact=name):
                path = self.out / name
                original = path.read_bytes()
                identity_path = self.out / "graph-identity.json"
                original_identity = identity_path.read_bytes()
                target = self.out / ("linked-" + name)
                path.rename(target)
                path.symlink_to(target.name)
                identity = json.loads(original_identity)
                identity[key] = None
                write_json(identity_path, identity)
                try:
                    result = self.load()
                    self.assertIsNone(result["firstStageQuery"])
                    self.assertEqual(result["handoffReferences"], [])
                finally:
                    path.unlink()
                    path.write_bytes(original)
                    target.unlink()
                    identity_path.write_bytes(original_identity)

    def test_missing_fallback_source_with_null_hash_is_not_a_match(self):
        manifest_path = self.out / "source-manifest.json"
        manifest = json.loads(manifest_path.read_text())
        manifest["files"][0]["sha256"] = None
        write_json(manifest_path, manifest)
        graph_path = self.out / "graph.json"
        graph = json.loads(graph_path.read_text())
        graph["nodes"][0]["source_sha256"] = None
        write_json(graph_path, graph)
        self.source.unlink()
        self.bind_identity()
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        ref = result["handoffReferences"][0]
        self.assertIsNone(ref["currentSourceSha256"])
        self.assertEqual(ref["sourceCurrentness"], "CHANGED_OR_MISSING")
        self.assertEqual(result["firstStageQuery"]["mode"], "dated-discovery-only")

    def test_scoped_named_reference_pins_require_nonblank_text(self):
        for field, value in (("id", None), ("id", ""), ("id", False), ("id", " "),
                             ("location", None), ("location", ""), ("location", 7), ("location", " ")):
            with self.subTest(field=field, value=value):
                self.make_graph()
                self.install_named_scope("current", self.source, self.root, "valid-reference")
                path = self.out / "cardano-preview-cli-filebinding-scope.json"
                scope = json.loads(path.read_text())
                graph = json.loads((self.out / "graph.json").read_text())
                if field == "id":
                    graph["nodes"][-1]["id"] = value
                    scope["handoff_references"][0]["nodeId"] = value
                    scope["seed"] = value
                else:
                    graph["nodes"][-1]["source_location"] = value
                    scope["handoff_references"][0]["sourceLocation"] = value
                write_json(self.out / "graph.json", graph)
                write_json(path, scope)
                self.bind_identity()
                identity = json.loads((self.out / "graph-identity.json").read_text())
                identity["cardano_preview_cli_filebinding_scope_sha256"] = digest(path)
                write_json(self.out / "graph-identity.json", identity)
                result = self.load()
                self.assertEqual(result["sourceScope"]["status"], "STALE")
                query = result["firstStageQuery"]
                self.assertTrue(query is None or query["mode"] != "current-explicit-source-scope-only")
                if query:
                    self.assertTrue(all(isinstance(arg, str) and arg.strip() for arg in query["argv"]))

    def test_fallback_named_reference_pins_require_nonblank_text(self):
        for field, value in (("id", None), ("id", ""), ("id", False), ("id", " "),
                             ("source_location", None), ("source_location", ""),
                             ("source_location", 7), ("source_location", " ")):
            with self.subTest(field=field, value=value):
                self.make_graph()
                path = self.out / "graph.json"
                graph = json.loads(path.read_text())
                graph["nodes"][0][field] = value
                write_json(path, graph)
                self.bind_identity()
                result = self.load()
                self.assertEqual(result["handoffReferences"], [])
                self.assertIsNone(result["firstStageQuery"])

    def test_scoped_reference_currentness_is_derived_from_validated_source(self):
        for fields_present in (True, False):
            with self.subTest(fields_present=fields_present):
                self.make_graph()
                self.install_named_scope("current", self.source, self.root, "valid-reference")
                path = self.out / "cardano-preview-cli-filebinding-scope.json"
                scope = json.loads(path.read_text())
                ref = scope["handoff_references"][0]
                derived = {"currentSourceSha256": None, "sourceCurrentness": "CHANGED_OR_MISSING",
                           "readRequired": False}
                if fields_present:
                    ref.update(derived)
                else:
                    for key in derived:
                        ref.pop(key)
                write_json(path, scope)
                identity = json.loads((self.out / "graph-identity.json").read_text())
                identity["cardano_preview_cli_filebinding_scope_sha256"] = digest(path)
                write_json(self.out / "graph-identity.json", identity)
                result = self.load()
                self.assertEqual(result["sourceScope"]["status"], "FRESH_FOR_EXPLICIT_SOURCE_SCOPE")
                self.assertEqual(result["handoffReferences"], result["sourceScope"]["handoffReferences"])
                actual = result["handoffReferences"][0]
                self.assertEqual(actual.get("currentSourceSha256"), self.pin)
                self.assertEqual(actual.get("sourceCurrentness"), "MATCH")
                self.assertIs(actual.get("readRequired"), True)
                self.assertEqual(json.loads(path.read_text()), scope)

    def test_older_same_checkout_scope_can_supply_exact_matching_bytes(self):
        self.install_named_scope("protocol", self.source, self.root, "matching-i6")
        self.install_named_scope("current", self.source, self.root, "nonmatching-i6", pin="0" * 64)
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "FRESH_FOR_EXPLICIT_SOURCE_SCOPE")
        self.assertEqual(result["firstStageQuery"]["seed"], "matching-i6")

    def test_declared_checkout_root_cannot_cover_a_source_outside_it(self):
        _, other_source = self.other_checkout_source()
        self.install_named_scope("current", other_source, self.root, "outside-root")
        result = self.load()
        self.assertEqual(result["sourceScope"]["status"], "STALE")
        self.assertFalse(result["sourceScope"]["matchesCheckout"])
        self.assertNotEqual(result["firstStageQuery"]["seed"], "outside-root")

    def test_unregistered_sidecar_is_not_discovered(self):
        write_json(self.out / "future-unregistered-scope.json", {"seed": "guessed"})
        result = self.load()
        self.assertIsNone(result["sourceScope"])
        self.assertEqual(result["firstStageQuery"]["seed"], "exact_cardano_heading")

    def test_artifact_changes_cannot_offer_unbound_node_query(self):
        (self.out / "graph.json").write_text('{"nodes": []}')
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        self.assertIsNone(result["firstStageQuery"])
        self.assertEqual(result["handoffReferences"], [])

    def test_source_inventory_changes_are_not_silently_fresh_and_output_is_bounded(self):
        for index in range(20):
            (self.root / (str(index) + ".md")).write_text("new primary source")
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        issue = next(i for i in result["freshness"]["issues"] if i["kind"] == "new-primary-files")
        self.assertEqual(issue["pathCount"], 20)
        self.assertEqual(len(issue["paths"]), 8)
        self.assertEqual(issue["omittedPathCount"], 12)

    def test_wrong_root_and_changed_head_remain_stale(self):
        manifest = json.loads((self.out / "source-manifest.json").read_text())
        manifest["root"] = str(self.root.parent)
        manifest["snapshots"][0]["git_commit"] = "0" * 40
        write_json(self.out / "source-manifest.json", manifest)
        self.bind_identity()
        result = self.load()
        self.assertEqual(result["status"], "STALE")
        kinds = {i["kind"] for i in result["freshness"]["issues"]}
        self.assertTrue({"wrong-root", "HEAD-changed"} <= kinds)

    def test_missing_matching_known_node_has_no_guessed_seed(self):
        write_json(self.out / "graph.json", {"nodes": [{"id": "cardano_guess"}], "links": []})
        self.bind_identity()
        result = self.load()
        self.assertIsNone(result["firstStageQuery"])
        self.assertEqual(result["handoffReferences"], [])


if __name__ == "__main__":
    unittest.main()
