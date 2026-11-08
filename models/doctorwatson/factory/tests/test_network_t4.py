"""E5/E6 (SWF-008, SWF-009): the `network:` row field, tier T4 and the `qa/tiers/t4.sh` refusal."""

import json
import os
import shutil
import subprocess
import tempfile
import unittest

from .helpers import FakeRepo, pred, yaml_of

REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))))
RECEIPT = "qa/sherlock/receipt-traces/OP-PV-ROLLBACK/receipt.json"


class NetworkField(FakeRepo):
    def test_networks_and_t4_accepted(self):
        self.predicates(yaml_of(
            pred("A6p", tier="T2", network="preview"),
            pred("A8p", tier="T3", network="preview", depends=["A6p"]),
            pred("A8m", tier="T4", network="mainnet", depends=["A8p"]),
            pred("A6", tier="T2", network="undeployed"),
            pred("A8", tier="T3", network="stagenet", retired="stagenet retired"),
        ))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_unknown_network_refused(self):
        self.predicates(yaml_of(pred("A8p", tier="T3", network="devnet")))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A8p: network must be one of undeployed, stagenet, preview, mainnet (got 'devnet')", r.stdout)

    def test_stagenet_only_on_retired_rows(self):
        self.predicates(yaml_of(pred("A8", tier="T3", network="stagenet")))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A8: network stagenet is retired; only a retired row may carry it", r.stdout)

    def test_network_checked_against_pins_file(self):
        self.write("qa/sherlock/target-pins.json", json.dumps({"networks": {"undeployed": {}, "preview": {}}}))
        self.predicates(yaml_of(pred("A8p", tier="T3", network="preview"),
                                pred("A8m", tier="T4", network="mainnet", depends=["A8p"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A8m: network 'mainnet' has no row in qa/sherlock/target-pins.json", r.stdout)
        self.assertNotIn("A8p:", r.stdout)

    def test_t4_needs_mainnet_and_mainnet_needs_t4(self):
        self.predicates(yaml_of(pred("A8p", tier="T3", network="preview"),
                                pred("A8m", tier="T4", network="preview", depends=["A8p"]),
                                pred("A9m", tier="T3", network="mainnet", depends=["A8p"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A8m: a T4 row must carry network: mainnet", r.stdout)
        self.assertIn("A9m: a network: mainnet row must be tier T4", r.stdout)

    def test_t4_without_t3_dependency_refused(self):
        self.predicates(yaml_of(pred("A6p", tier="T2", network="preview"),
                                pred("A8m", tier="T4", network="mainnet", depends=["A6p"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A8m: a T4 row must depend on its T3 Preview counterpart 'A8p' (same key lineage)", r.stdout)

    def test_t4_on_other_lineage_refused(self):
        self.predicates(yaml_of(pred("B4p", set="B", tier="T3", network="preview"),
                                pred("B5m", set="B", tier="T4", network="mainnet", depends=["B4p"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("B5m: a T4 row must depend on its T3 Preview counterpart 'B5p'", r.stdout)

    def test_counterpart_must_be_t3_preview(self):
        self.predicates(yaml_of(pred("B5p", set="B", tier="T2", network="preview"),
                                pred("B5m", set="B", tier="T4", network="mainnet", depends=["B5p"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("B5m: a T4 row must depend on its T3 Preview counterpart 'B5p'", r.stdout)

    def test_check_tier_t4(self):
        self.predicates(yaml_of(pred("A8p", tier="T3", network="preview"),
                                pred("A8m", tier="T4", network="mainnet", depends=["A8p"])))
        self.assertEqual(self.run_cli("check", "A8p").returncode, 0)
        r = self.run_cli("check", "--tier", "T4")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("A8m: PASS", r.stdout)
        self.assertNotIn("A8p:", r.stdout)


class RealCatalog(unittest.TestCase):
    def test_repo_mainnet_rows_are_t4(self):
        import yaml
        with open(os.path.join(REPO, "qa/sherlock/predicates.yaml")) as fh:
            rows = {r["id"]: r for r in yaml.safe_load(fh)["predicates"]}
        for i in ("B4m", "B5m", "B8m", "B9m"):
            self.assertEqual((rows[i]["tier"], rows[i].get("network")), ("T4", "mainnet"), i)
        for i in ("A2p", "A3p", "A4p", "A6p", "A8p", "A9p", "A10p", "B4p", "B5p", "B8p", "B9p"):
            self.assertEqual(rows[i].get("network"), "preview", i)
        for i in ("A8", "A9", "A10", "B4", "B5", "B8", "B9"):
            self.assertEqual(rows[i].get("network"), "stagenet", i)
            self.assertIn("retired", rows[i], i)


class T4Runner(unittest.TestCase):
    """qa/tiers/t4.sh refuses unless the Preview rollback rehearsal receipt is committed (SWF-009)."""

    def setUp(self):
        self._tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self._tmp.cleanup)
        self.d = os.path.join(self._tmp.name, "repo")
        os.makedirs(os.path.join(self.d, "qa", "tiers"))
        os.makedirs(os.path.join(self.d, "scripts"))
        for f in ("t4.sh", "lib.sh"):
            shutil.copy(os.path.join(REPO, "qa", "tiers", f), os.path.join(self.d, "qa", "tiers", f))
        # stub doctorwatson: records its arguments and passes
        stub = os.path.join(self.d, "scripts", "doctorwatson")
        with open(stub, "w") as fh:
            fh.write('#!/usr/bin/env bash\necho "$@" >> "$(git rev-parse --show-toplevel)/.dw-calls"\nexit 0\n')
        os.chmod(stub, 0o755)
        with open(os.path.join(self.d, ".gitignore"), "w") as fh:
            fh.write(".dw-calls\n")
        self.git("init", "-q")
        self.git("add", ".")
        self.git("commit", "-q", "-m", "init")

    def git(self, *args):
        env = dict(os.environ, GIT_AUTHOR_NAME="t", GIT_AUTHOR_EMAIL="t@example.invalid",
                   GIT_COMMITTER_NAME="t", GIT_COMMITTER_EMAIL="t@example.invalid")
        for k in ("GIT_DIR", "GIT_INDEX_FILE", "GIT_WORK_TREE"):
            env.pop(k, None)
        subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", *args],
                       cwd=self.d, env=env, check=True, capture_output=True)

    def run_t4(self):
        env = {k: v for k, v in os.environ.items() if not k.startswith("GIT_")}
        return subprocess.run(["bash", "qa/tiers/t4.sh"], cwd=self.d, env=env, capture_output=True,
                              text=True, timeout=60)

    def calls(self):
        p = os.path.join(self.d, ".dw-calls")
        return open(p).read() if os.path.exists(p) else ""

    def write_receipt(self):
        path = os.path.join(self.d, RECEIPT)
        os.makedirs(os.path.dirname(path), exist_ok=True)
        with open(path, "w") as fh:
            fh.write('{"op":"OP-PV-ROLLBACK-REHEARSAL"}\n')

    def test_refuses_without_receipt(self):
        r = self.run_t4()
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("REFUSED", r.stderr)
        self.assertIn(RECEIPT, r.stderr)
        self.assertNotIn("check", self.calls())

    def test_refuses_with_uncommitted_receipt(self):
        self.write_receipt()
        r = self.run_t4()
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("not committed", r.stderr)
        self.assertNotIn("check", self.calls())

    def test_runs_t4_checks_with_committed_receipt(self):
        self.write_receipt()
        self.git("add", RECEIPT)
        self.git("commit", "-q", "-m", "receipt")
        r = self.run_t4()
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("check --tier T4", self.calls())


if __name__ == "__main__":
    unittest.main()
