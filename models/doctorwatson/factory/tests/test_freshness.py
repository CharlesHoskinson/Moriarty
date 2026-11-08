"""E7 (SAS-002): freshness by declared dependency closure, in temporary git repos."""

import json
import subprocess

from .helpers import FakeRepo, pred, yaml_of

PINS = {
    "schema": "moriarty.target-pins/1",
    "networks": {
        "preview": {"ledger": "9.1.0.1", "compact": "0.36.0",
                    "chain": {"networkId": "preview", "genesisHash": "0xaa", "specVersion": 2001100}},
        "mainnet": {"ledger": "9.1.0.1", "compact": "0.36.0",
                    "chain": {"networkId": "mainnet", "genesisHash": "0xbb", "specVersion": 2001100}},
    },
}


def row(id, deps=None, **kw):
    text = pred(id, **kw)
    if deps is not None:
        text += f"\n    deps: {deps}"
    return text


class GitRepo(FakeRepo):
    def setUp(self):
        super().setUp()
        self.git("init", "-q", "-b", "main")
        self.git("config", "user.email", "t@example.invalid")
        self.git("config", "user.name", "t")
        self.git("config", "commit.gpgsign", "false")
        self.write(".gitignore", "qa/sherlock/traces/\nqa/sherlock/.status.lock\ntarget/\n")
        self.write("src/lib.rs", "fn a() {}\n")
        self.write("openspec/spec.md", "spec v1\n")
        self.write("other/notes.txt", "n1\n")
        self.write("target/keys/node.manifest.json", '{"vk":"1"}\n')
        self.write("qa/sherlock/receipt-traces/OP-A/receipt.json", '{"tx":"1"}\n')
        self.write("qa/sherlock/receipt-traces/OP-B/receipt.json", '{"tx":"1"}\n')
        self.put_pins(PINS)

    def git(self, *args):
        r = subprocess.run(["git", "-C", self.repo, *args], capture_output=True, text=True)
        self.assertEqual(r.returncode, 0, r.stderr)
        return r.stdout

    def put_pins(self, pins):
        self.write("qa/sherlock/target-pins.json", json.dumps(pins, indent=1) + "\n")

    def commit(self, msg, *paths):
        self.git("add", "--", *(paths or ["."]))
        self.git("commit", "-q", "-m", msg)

    def setup_rows(self, *rows):
        self.predicates(yaml_of(*rows))
        self.commit("init")
        r = self.run_cli("check")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def state_commit(self):
        self.commit("chore(factory-state)", "qa/sherlock/status.json", "qa/sherlock/runs.jsonl")

    def next_json(self):
        r = self.run_cli("next", "--json")
        self.assertEqual(r.returncode, 0, r.stderr)
        return json.loads(r.stdout)

    def stale_ids(self):
        return {s["id"]: s["reasons"] for s in self.next_json()["stale"]}

    def assert_eligible(self, pid):
        d = self.next_json()
        self.assertNotIn(pid, {s["id"] for s in d["stale"]}, d["stale"])
        self.assertIsNone(d["firstFailing"], d)

    def assert_stale(self, pid, reason_prefix):
        stale = self.stale_ids()
        self.assertIn(pid, stale)
        self.assertTrue(any(r.startswith(reason_prefix) for r in stale[pid]), stale[pid])
        d = self.next_json()
        self.assertEqual(d["firstFailing"]["id"], pid)
        self.assertTrue(d["firstFailing"]["stale"])


class DeclaredClosure(GitRepo):
    DEPS = ('{paths: ["src/**", "openspec/spec.md"], pins: preview, chain: preview, '
            'manifests: ["target/keys/node.manifest.json"], '
            'receipts: ["qa/sherlock/receipt-traces/OP-A/receipt.json"]}')

    def setUp(self):
        super().setUp()
        self.setup_rows(row("A1", deps=self.DEPS))

    def test_result_records_head_and_closure(self):
        e = self.status()["predicates"]["A1"]
        self.assertEqual(e["head"], self.git("rev-parse", "HEAD").strip())
        self.assertEqual(set(e["closure"]), {
            "row", "paths", "pins:preview", "chain:preview",
            "manifest:target/keys/node.manifest.json",
            "receipt:qa/sherlock/receipt-traces/OP-A/receipt.json"})

    def test_state_only_commit_eligible(self):
        self.state_commit()
        self.assert_eligible("A1")
        self.assertEqual(self.next_json()["counts"]["A"], [1, 1])

    def test_receipt_only_commit_of_undeclared_receipt_eligible(self):
        self.state_commit()
        self.write("qa/sherlock/receipt-traces/OP-B/receipt.json", '{"tx":"2"}\n')
        self.write("qa/sherlock/receipt-traces/OP-C/receipt.json", '{"tx":"3"}\n')
        self.commit("receipts", "qa/sherlock/receipt-traces")
        self.assert_eligible("A1")

    def test_undeclared_path_change_eligible(self):
        self.write("other/notes.txt", "n2\n")
        self.commit("notes", "other/notes.txt")
        self.assert_eligible("A1")

    def test_source_change_stale(self):
        self.write("src/lib.rs", "fn a() { 1 }\n")
        self.commit("src", "src/lib.rs")
        self.assert_stale("A1", "paths")

    def test_new_source_file_stale(self):
        self.write("src/deep/new.rs", "fn b() {}\n")
        self.assert_stale("A1", "paths")

    def test_spec_change_stale(self):
        self.write("openspec/spec.md", "spec v2\n")
        self.commit("spec", "openspec/spec.md")
        self.assert_stale("A1", "paths")

    def test_key_manifest_change_at_unchanged_head_stale(self):
        head = self.git("rev-parse", "HEAD")
        self.write("target/keys/node.manifest.json", '{"vk":"2"}\n')
        self.assertEqual(self.git("rev-parse", "HEAD"), head)
        self.assert_stale("A1", "manifest:")

    def test_pins_row_change_stale(self):
        pins = json.loads(json.dumps(PINS))
        pins["networks"]["preview"]["compact"] = "0.37.0"
        self.put_pins(pins)
        self.commit("pins", "qa/sherlock/target-pins.json")
        self.assert_stale("A1", "pins:preview")

    def test_other_pins_row_change_eligible(self):
        pins = json.loads(json.dumps(PINS))
        pins["networks"]["mainnet"]["compact"] = "0.37.0"
        pins["updated"] = "later"
        self.put_pins(pins)
        self.commit("pins", "qa/sherlock/target-pins.json")
        self.assert_eligible("A1")

    def test_chain_instance_change_stale(self):
        pins = json.loads(json.dumps(PINS))
        pins["networks"]["preview"]["chain"]["genesisHash"] = "0xcc"
        self.put_pins(pins)
        stale = self.stale_ids()
        self.assertIn("chain:preview", stale["A1"])

    def test_declared_receipt_change_stale(self):
        self.write("qa/sherlock/receipt-traces/OP-A/receipt.json", '{"tx":"2"}\n')
        self.commit("receipt", "qa/sherlock/receipt-traces")
        self.assert_stale("A1", "receipt:")

    def test_rerun_restores_eligibility(self):
        self.write("src/lib.rs", "fn a() { 2 }\n")
        self.commit("src", "src/lib.rs")
        self.assert_stale("A1", "paths")
        self.assertEqual(self.run_cli("check", "A1").returncode, 0)
        self.assert_eligible("A1")

    def test_dirty_run_then_commit_eligible(self):
        self.write("src/lib.rs", "fn a() { 3 }\n")
        self.assertEqual(self.run_cli("check", "A1").returncode, 0)
        self.commit("src", "src/lib.rs")
        self.state_commit()
        self.assert_eligible("A1")


class ChainOnlyClosure(GitRepo):
    def test_chain_row_ignores_software_pin_change(self):
        self.setup_rows(row("A9p", deps="{chain: preview}"))
        pins = json.loads(json.dumps(PINS))
        pins["networks"]["preview"]["compact"] = "0.37.0"
        self.put_pins(pins)
        self.assert_eligible("A9p")
        pins["networks"]["preview"]["chain"]["specVersion"] = 2001200
        self.put_pins(pins)
        self.assert_stale("A9p", "chain:preview")


class DefaultClosure(GitRepo):
    """A row without `deps:` depends on the whole working tree except factory state."""

    def setUp(self):
        super().setUp()
        self.setup_rows(row("A1"), row("A2", depends=["A1"], deps='{paths: ["openspec/**"]}'))

    def test_state_only_commit_eligible(self):
        self.state_commit()
        self.assert_eligible("A1")
        self.assert_eligible("A2")

    def test_any_source_change_stale(self):
        self.write("other/notes.txt", "n2\n")
        self.commit("notes", "other/notes.txt")
        self.assert_stale("A1", "paths")

    def test_receipt_change_stale_without_declaration(self):
        self.write("qa/sherlock/receipt-traces/OP-B/receipt.json", '{"tx":"2"}\n')
        self.assert_stale("A1", "paths")

    def test_stale_dependency_propagates(self):
        self.write("other/notes.txt", "n2\n")
        stale = self.stale_ids()
        self.assertIn("depends:A1", stale["A2"])

    def test_own_row_change_stale_other_row_change_not(self):
        self.predicates(yaml_of(row("A1", check="true; true"),
                                row("A2", depends=["A1"], deps='{paths: ["openspec/**"]}', title="renamed")))
        stale = self.stale_ids()
        self.assertIn("row", stale["A1"])
        self.assertEqual(stale["A2"], ["depends:A1"])

    def test_legacy_entry_without_closure_stale(self):
        st = self.status()
        st["predicates"]["A1"].pop("closure")
        self.write("qa/sherlock/status.json", json.dumps(st))
        self.assert_stale("A1", "no recorded closure")


class NextText(GitRepo):
    def test_stale_line_printed(self):
        self.setup_rows(row("A1", deps='{paths: ["src/**"]}'))
        self.write("src/lib.rs", "changed\n")
        r = self.run_cli("next")
        self.assertIn("STALE: A1 (paths)", r.stdout)
        self.assertIn("FIRST FAILING: A1", r.stdout)
        self.assertIn("A: 0/1", r.stdout)


class DepsValidate(FakeRepo):
    def test_bad_deps_refused(self):
        self.predicates(yaml_of(row("A1", deps="[src]"),
                                row("A2", deps="{paths: src, bogus: 1}"),
                                row("A3", deps="{pins: [1], manifests: [\"\"]}")))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A1: deps must be a mapping", r.stdout)
        self.assertIn("A2: deps.paths must be a list of non-empty strings", r.stdout)
        self.assertIn("A2: deps has unknown key 'bogus'", r.stdout)
        self.assertIn("A3: deps.pins must be a network name or a list of them", r.stdout)
        self.assertIn("A3: deps.manifests must be a list of non-empty strings", r.stdout)

    def test_good_deps_accepted(self):
        self.predicates(yaml_of(row("A1", deps=DeclaredClosure.DEPS), row("A2", deps="{chain: [preview, mainnet]}")))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 0, r.stdout)

    def test_non_git_repo_check_then_next_eligible(self):
        self.write("src/lib.rs", "x\n")
        self.predicates(yaml_of(row("A1", deps='{paths: ["src/**"]}'), row("A2")))
        self.assertEqual(self.run_cli("check").returncode, 0)
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertEqual(d["stale"], [])
        self.write("src/lib.rs", "y\n")
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertEqual({s["id"] for s in d["stale"]}, {"A1", "A2"})
