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
            "sources": [{"source_file": str(self.source), "sha256": self.pin}],
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
