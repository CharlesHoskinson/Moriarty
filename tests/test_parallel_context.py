"""Behavioral checks for the read-only author context assembler."""
import copy
import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPT = Path(__file__).resolve().parents[1] / "scripts/build_task_context.py"


def module():
    spec = importlib.util.spec_from_file_location("task_context", SCRIPT)
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


class ParallelContextTests(unittest.TestCase):
    def setUp(self):
        self.api = module()
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.base = "a" * 40
        self.manifest = {
            "schema": "moriarty.parallel-language-workflow/1",
            "candidate_base": self.base,
            "shared_owner": "integrator",
            "shared_files": ["crates/core/src/ir.rs"],
            "requirements": [{"path": "ROADMAP.md", "role": "requirement", "lines": [1, 4]}],
            "closure_relevant_inputs": ["crates/core/src/ir.rs"],
            "acceptance_graph": {"U0": [], "U1": ["U0"]},
            "integration_reviewers": [
                {"id": "R-S", "model": "gpt-6.1-sol", "effort": "high", "role": "integration-reviewer", "read_only": True, "implementation_assignments": [], "authored_current_composed_change": False},
                {"id": "R-G", "model": "grok-4.7", "effort": "high", "role": "integration-reviewer", "read_only": True, "implementation_assignments": [], "authored_current_composed_change": False},
            ],
            "lanes": [],
        }
        inventory = []
        nodes = []
        for i, lane in enumerate(["S1", "S2", "S3", "G1", "G2", "G3"]):
            path = f"crates/core/tests/lane{i}.rs"
            model = "gpt-6.1-sol" if lane.startswith("S") else "grok-4.7"
            self.manifest["lanes"].append({
                "id": lane, "author": {"model": model, "effort": "high"},
                "objective": "Exercise a scoped public consumer", "milestones": ["U0"],
                "write_set": [path], "source_dependencies": [],
                "acceptance_dependencies": ["U0"], "acceptance_blockers": ["native unrun"],
                "inputs": [{"path": path, "role": "test", "lines": [1, 4]}],
                "verification": [{"argv": ["cargo", "test", "--locked"], "applicability": "after-implementation"}],
            })
            nodes.append({"id": lane, "source_file": path, "source_location": "L1", "label": lane, "_origin": "ast"})
        self.manifest["review_round1"] = {
            "S1": ["S2", "G1"], "S2": ["S3", "G2"], "S3": ["S1", "G3"],
            "G1": ["S1", "G2"], "G2": ["S2", "G3"], "G3": ["S3", "G1"],
        }
        paths = ["ROADMAP.md", "crates/core/src/ir.rs"] + [x["write_set"][0] for x in self.manifest["lanes"]]
        for path in paths:
            dest = self.root / path
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_text("public source\n")
            raw = dest.read_bytes()
            oid = hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest()
            inventory.append({"path": path, "blob_oid": oid, "coverage_class": "candidate_content", "mode": "100644"})
        self.bundle = {
            "commit": self.base,
            "graph": {"built_at_commit": self.base, "nodes": nodes, "links": [{"source": "S1", "target": "S2", "relation": "calls", "source_file": paths[2], "source_location": "L1"}]},
            "coverage": {"commit": self.base, "inventory_complete": True, "coverage_classes": {"candidate_content": len(paths)}},
            "inventory": {"commit": self.base, "tracked_files": inventory},
            "summary": {"commit": self.base, "nodes": len(nodes), "edges": 1},
            "digests": {"graph.json": "f" * 64},
        }

    def assemble(self, lane="S1", round_number=1):
        return self.api.assemble(self.root, self.manifest, self.bundle, lane, self.base, round_number)

    def test_real_consumer_sources_and_dual_non_author_reviews(self):
        packet = self.assemble()
        self.assertEqual(packet["routing"]["reviewers"], ["S2", "G1"])
        self.assertEqual(packet["native_gate"]["status"], "not-admitted-by-context")
        self.assertTrue(packet["source_set"][0]["sha256"])
        self.assertEqual(packet["graph"]["qualification"], "advisory-scoped-snapshot")

    def test_exact_and_directory_write_overlaps_fail(self):
        for second in ["crates/core/tests/lane0.rs", "crates/core/tests/"]:
            with self.subTest(second=second):
                manifest = copy.deepcopy(self.manifest)
                manifest["lanes"][1]["write_set"] = [second]
                with self.assertRaisesRegex(self.api.ContextError, "write-overlap"):
                    self.api.validate_manifest(self.root, manifest)

    def test_shared_owner_collision_fails(self):
        self.manifest["lanes"][0]["write_set"] = ["crates/core/src/ir.rs"]
        with self.assertRaisesRegex(self.api.ContextError, "shared-owner"):
            self.assemble()

    def test_source_and_acceptance_cycles_fail(self):
        self.manifest["lanes"][0]["source_dependencies"] = ["S2"]
        self.manifest["lanes"][1]["source_dependencies"] = ["S1"]
        with self.assertRaisesRegex(self.api.ContextError, "dependency-cycle"):
            self.assemble()
        self.manifest["lanes"][0]["source_dependencies"] = []
        self.manifest["lanes"][1]["source_dependencies"] = []
        self.manifest["acceptance_graph"]["U0"] = ["U1"]
        with self.assertRaisesRegex(self.api.ContextError, "dependency-cycle"):
            self.assemble()

    def test_outside_secret_binary_archive_and_symlink_paths_fail(self):
        for path in ["../outside.md", "/etc/passwd", "raw/input.md", "crates/core/actualPK.json", "crates/core/privatePK.json", "crates/core/marker.json", "crates/core/private-marker.json", "crates/core/SRS.dat", "crates/core/srs_14.json", "crates/core/credentials.json", "crates/core/state.bin", "target/cache.rs"]:
            with self.subTest(path=path):
                with self.assertRaises(self.api.ContextError):
                    self.api.safe_source(self.root, path)
        (self.root / "docs").mkdir()
        (self.root / "docs/link.md").symlink_to(self.root / "ROADMAP.md")
        with self.assertRaisesRegex(self.api.ContextError, "symlink"):
            self.api.safe_source(self.root, "docs/link.md")

    def test_stale_commit_and_code_fail_but_current_workflow_docs_are_scoped(self):
        with self.assertRaisesRegex(self.api.ContextError, "stale-graph-commit"):
            self.api.assemble(self.root, self.manifest, self.bundle, "S1", "b" * 40)
        (self.root / "ROADMAP.md").write_text("current workflow steering\n")
        packet = self.assemble()
        source = next(x for x in packet["source_set"] if x["path"] == "ROADMAP.md")
        self.assertEqual(source["graph_freshness"], "current-doc-overlay-graph-hints-omitted")
        (self.root / "crates/core/src/ir.rs").write_text("changed interface\n")
        with self.assertRaisesRegex(self.api.ContextError, "stale-graph-source"):
            self.assemble()

    def test_inventory_only_mori_is_not_ast_or_semantic_evidence(self):
        path = "examples/core/reference.mori"
        dest = self.root / path
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_text('language "moriarty/1";\n')
        raw = dest.read_bytes()
        self.bundle["inventory"]["tracked_files"].append({"path": path, "blob_oid": hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest(), "coverage_class": "candidate_content"})
        self.manifest["lanes"][0]["inputs"].append({"path": path, "role": "code", "lines": [1, 4]})
        source = next(x for x in self.assemble()["source_set"] if x["path"] == path)
        self.assertEqual(source["graph_layer"], "inventory-only-direct-source")

    def test_review_rotation_excludes_author_and_keeps_both_providers(self):
        for round_number in range(1, 7):
            for lane in self.manifest["lanes"]:
                packet = self.assemble(lane["id"], round_number)
                reviewers = packet["routing"]["reviewers"]
                self.assertNotIn(lane["id"], reviewers)
                models = {next(x["author"]["model"] for x in self.manifest["lanes"] if x["id"] == r) for r in reviewers}
                self.assertEqual(models, {"gpt-6.1-sol", "grok-4.7"})

    def test_changed_context_snapshot_is_stale(self):
        old = self.assemble()
        (self.root / "ROADMAP.md").write_text("updated scope\n")
        with self.assertRaisesRegex(self.api.ContextError, "stale-context"):
            self.api.check_context(old, self.assemble())

    def test_saved_packet_body_tampering_rejects_even_with_unchanged_snapshot(self):
        original = self.assemble()
        for field in ["excerpt", "routing", "objective", "write_set", "neighbors"]:
            changed = copy.deepcopy(original)
            if field == "excerpt":
                changed["source_set"][0]["excerpt"]["text"] = "unreviewed source text"
            elif field == "routing":
                changed["routing"]["reviewers"] = ["S1", "G1"]
            elif field == "objective":
                changed["objective"] = "unapproved objective"
            elif field == "write_set":
                changed["write_set"] = ["crates/core/src/ir.rs"]
            else:
                changed["graph"]["neighbors"].append({"label": "fabricated source pointer"})
            self.assertEqual(changed["freshness"], original["freshness"])
            with self.subTest(field=field):
                with self.assertRaisesRegex(self.api.ContextError, "stale-context"):
                    self.api.check_context(changed, original)
        reordered = json.loads(json.dumps(original, indent=3, sort_keys=True))
        self.api.check_context(reordered, original)

    def test_protected_inventory_rejects_before_any_source_inspection(self):
        for path in ["crates/core/src/provider_public.rs", "docs/native-parameters.md", "experiments/moriarty-language/formal/quint/language/current.qnt"]:
            manifest = copy.deepcopy(self.manifest)
            bundle = copy.deepcopy(self.bundle)
            manifest["lanes"][0]["inputs"].append({"path": path, "role": "code", "lines": [1, 4]})
            bundle["inventory"]["tracked_files"].append({"path": path, "blob_oid": "0" * 40, "coverage_class": "protected_excluded"})
            with self.subTest(path=path):
                with mock.patch.object(self.api, "safe_source", side_effect=AssertionError("filesystem inspection preceded protected inventory refusal")):
                    with self.assertRaisesRegex(self.api.ContextError, "excluded-graph-coverage"):
                        self.api.assemble(self.root, manifest, bundle, "S1", self.base)

    def test_protected_shared_path_refuses_before_any_safe_source(self):
        path = "docs/native-parameters.md"
        self.manifest["shared_files"].append(path)
        self.bundle["inventory"]["tracked_files"].append({
            "path": path, "blob_oid": "0" * 40, "coverage_class": "protected_excluded",
        })
        with mock.patch.object(self.api, "safe_source", side_effect=AssertionError("protected shared path reached filesystem inspection")):
            with self.assertRaisesRegex(self.api.ContextError, "excluded-graph-coverage"):
                self.assemble()

    def test_public_source_with_nonprotected_inventory_can_be_inlined(self):
        for path in ["crates/core/src/wallet_provider.rs", "docs/wallet-provider-interface.md"]:
            dest = self.root / path
            dest.parent.mkdir(parents=True, exist_ok=True)
            raw = b"Public wallet/provider interface source\n"
            dest.write_bytes(raw)
            self.bundle["inventory"]["tracked_files"].append({"path": path, "blob_oid": hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest(), "coverage_class": "candidate_content"})
            self.manifest["lanes"][0]["inputs"].append({"path": path, "role": "code", "lines": [1, 4]})
        packet = self.assemble()
        for path in ["crates/core/src/wallet_provider.rs", "docs/wallet-provider-interface.md"]:
            entry = next(x for x in packet["source_set"] if x["path"] == path)
            self.assertIn("Public wallet/provider", entry["excerpt"]["text"])

    def test_composed_review_seats_are_readonly_and_outside_all_authors(self):
        packet = self.assemble()
        plan = packet["integration_reviewers"]
        self.assertEqual({x["id"] for x in plan}, {"R-S", "R-G"})
        authors = {x["id"] for x in self.manifest["lanes"]} | {self.manifest["shared_owner"]}
        self.assertFalse({x["id"] for x in plan} & authors)
        mutations = [{"id": "S1"}, {"model": "grok-4.7"}, {"effort": "medium"}, {"read_only": False}, {"implementation_assignments": ["S1"]}, {"authored_current_composed_change": True}]
        for delta in mutations:
            manifest = copy.deepcopy(self.manifest)
            manifest["integration_reviewers"][0].update(delta)
            with self.subTest(delta=delta):
                with self.assertRaisesRegex(self.api.ContextError, "integration-review"):
                    self.api.validate_manifest(self.root, manifest)

    def test_provider_metadata_roots_reject_before_read(self):
        for path in ["/tmp/.codex/parallel-language-workflow.json", "/tmp/provider-cache/graph", "/tmp/privatePK/context.json"]:
            with self.subTest(path=path):
                with self.assertRaisesRegex(self.api.ContextError, "protected-metadata"):
                    self.api.metadata_path(Path(path))

    def test_public_wallet_provider_source_is_allowed_but_private_storage_is_not(self):
        for path in ["crates/core/src/provider_api.rs", "crates/core/src/wallet_adapter.rs", "docs/wallet-provider-interface.md"]:
            with self.subTest(path=path):
                self.assertEqual(self.api.safe_source(self.root, path), self.root / path)
        for path in ["crates/core/private/wallet.json", "crates/core/cache/provider.json", "crates/core/wallet-secret.json"]:
            with self.subTest(path=path):
                with self.assertRaises(self.api.ContextError):
                    self.api.safe_source(self.root, path)

    def test_explicit_current_quint_model_is_direct_read_despite_archive_inventory(self):
        path = "experiments/moriarty-language/formal/quint/language/lifecycle.qnt"
        dest = self.root / path
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_text("module lifecycle { pure val safety = true }\n")
        raw = dest.read_bytes()
        self.bundle["inventory"]["tracked_files"].append({"path": path, "blob_oid": hashlib.sha1(b"blob " + str(len(raw)).encode() + b"\0" + raw).hexdigest(), "coverage_class": "archival_inventory_only"})
        self.manifest["lanes"][0]["inputs"].append({"path": path, "role": "formal-qa", "lines": [1, 4]})
        item = next(x for x in self.assemble()["source_set"] if x["path"] == path)
        self.assertEqual(item["graph_layer"], "inventory-only-direct-source")
        self.assertEqual(item["coverage_class"], "archival_inventory_only")
        self.assertEqual(item["sha256"], hashlib.sha256(raw).hexdigest())
        with self.assertRaises(self.api.ContextError):
            self.api.safe_source(self.root, "experiments/moriarty-language/formal/quint/language/results/qa.log")

    def test_future_verification_inputs_do_not_become_green(self):
        self.manifest["lanes"][0]["verification"][0]["requires_paths"] = ["crates/core/tests/new_case.rs"]
        check = self.assemble()["verification"][0]
        self.assertEqual(check["current_applicability"], "specified-only-missing-inputs")
        self.assertEqual(check["missing_inputs"], ["crates/core/tests/new_case.rs"])

    def test_graph_bundle_digest_and_metadata_commit_fail_closed(self):
        graph_dir = self.root / "graph"
        graph_dir.mkdir()
        payloads = {"graph.json": self.bundle["graph"], "coverage.json": self.bundle["coverage"], "inventory.json": self.bundle["inventory"], "build-summary.json": self.bundle["summary"]}
        ready = {"ready": True, "commit": self.base, "files": {}}
        for name, value in payloads.items():
            raw = json.dumps(value).encode()
            (graph_dir / name).write_bytes(raw)
            ready["files"][name] = {"sha256": hashlib.sha256(raw).hexdigest(), "bytes": len(raw)}
        (graph_dir / "graph-ready.json").write_text(json.dumps(ready))
        self.api.load_graph_bundle(graph_dir)
        (graph_dir / "graph.json").write_text("{}")
        with self.assertRaisesRegex(self.api.ContextError, "stale-graph-digest"):
            self.api.load_graph_bundle(graph_dir)
        payloads["graph.json"]["built_at_commit"] = "b" * 40
        raw = json.dumps(payloads["graph.json"]).encode()
        (graph_dir / "graph.json").write_bytes(raw)
        ready["files"]["graph.json"] = {"sha256": hashlib.sha256(raw).hexdigest(), "bytes": len(raw)}
        (graph_dir / "graph-ready.json").write_text(json.dumps(ready))
        with self.assertRaisesRegex(self.api.ContextError, "stale-graph-metadata"):
            self.api.load_graph_bundle(graph_dir)


if __name__ == "__main__":
    unittest.main()
