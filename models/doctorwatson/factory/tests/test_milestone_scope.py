"""E15 (SWF-012, SWF-013): milestone-scoped tier runners.

The Sherlock M6 gate is the C4 dependency closure; the Fenrir gates are the B rows and
their closures. An unimplemented mandatory B4p leaves the Sherlock M6 selection eligible
and blocks the Fenrir Preview selection.
"""

import json
import os
import unittest

from .helpers import FakeRepo, pred, yaml_of

UNIMPL = 'echo "UNIMPLEMENTED $1"; exit 2\n'
REPO = os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))))


class MilestoneScope(FakeRepo):
    def setUp(self):
        super().setUp()
        self.write("scripts/qa/unimplemented.sh", UNIMPL, 0o755)
        self.predicates(yaml_of(
            pred("A1", check="true"),
            pred("A8p", tier="T3", network="preview", check="true", depends=["A1"]),
            pred("U023", set="U", tier="T2", check="true"),
            pred("C4", set="C", tier="T1", check="true", depends=["U023", "A8p"]),
            pred("B3", set="B", tier="T2", check="true"),
            pred("B4p", set="B", tier="T3", network="preview", check="scripts/qa/unimplemented.sh B4p",
                 depends=["B3", "A8p"]),
            pred("B9p", set="B", tier="T3", network="preview", check="true", depends=["B4p"]),
            pred("B4m", set="B", tier="T4", network="mainnet", check="true", depends=["B4p"]),
            pred("B9m", set="B", tier="T4", network="mainnet", check="true", depends=["B9p", "B4m"]),
            pred("A8", tier="T3", network="stagenet", retired="stagenet", check="touch ran-a8; exit 1"),
            pred("A99", tier="T3", optional=True, check="touch ran-a99; exit 1", depends=["A8p"]),
        ))
        self.assertEqual(self.run_cli("check", "A1", "U023", "B3").returncode, 0)

    def test_sherlock_m6_eligible_despite_b4p(self):
        for flag in (["--milestone", "sherlock-m6"], ["--closure", "C4"]):
            with self.subTest(flag=flag):
                r = self.run_cli("check", "--tier", "T3", *flag)
                self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
                self.assertIn("A8p: PASS", r.stdout)
                self.assertNotIn("B4p:", r.stdout)
                self.assertNotIn("B9p:", r.stdout)
        r = self.run_cli("check", "--milestone", "sherlock-m6")
        self.assertEqual(r.returncode, 0, r.stdout)
        self.assertIn("C4: PASS", r.stdout)
        d = json.loads(self.run_cli("next", "--json", "--milestone", "sherlock-m6").stdout)
        self.assertIsNone(d["firstFailing"])

    def test_fenrir_preview_blocked_by_b4p(self):
        r = self.run_cli("check", "--tier", "T3", "--milestone", "fenrir-preview")
        self.assertEqual(r.returncode, 1, r.stdout)
        self.assertIn("B4p: FAIL exit=2", r.stdout)
        self.assertIn("B9p: BLOCKED-BY B4p", r.stdout)
        self.assertNotIn("B4m:", r.stdout)  # T4, outside this tier and this gate
        d = json.loads(self.run_cli("next", "--json", "--milestone", "fenrir-preview").stdout)
        self.assertEqual(d["firstFailing"]["id"], "B4p")

    def test_fenrir_mainnet_closure(self):
        r = self.run_cli("check", "--tier", "T4", "--milestone", "fenrir-mainnet")
        self.assertEqual(r.returncode, 1, r.stdout)
        self.assertIn("B4m: BLOCKED-BY B4p", r.stdout)
        self.assertIn("B9m: BLOCKED-BY B9p", r.stdout)

    def test_closure_skips_retired_and_optional(self):
        self.predicates(yaml_of(
            pred("A8p", tier="T3", check="true"),
            pred("A99", tier="T3", optional=True, check="touch ran-a99; exit 1"),
            pred("A98", tier="T3", optional=True, check="true", depends=["A99"]),
            pred("C4", set="C", tier="T1", check="true", depends=["A8p"]),
        ))
        r = self.run_cli("check", "--closure", "A98")
        self.assertEqual(r.returncode, 0, r.stdout)
        self.assertEqual(r.stdout.strip(), "")
        self.assertFalse(os.path.exists(self.p("ran-a99")))
        r = self.run_cli("check", "--closure", "A98", "--include-optional")
        self.assertIn("A99: FAIL", r.stdout)

    def test_unknown_closure_and_milestone(self):
        r = self.run_cli("check", "--closure", "Z9")
        self.assertEqual(r.returncode, 2)
        self.assertIn("unknown predicate id(s): Z9", r.stderr)
        r = self.run_cli("check", "--milestone", "nope")
        self.assertEqual(r.returncode, 2)


class RealCatalogClosures(unittest.TestCase):
    """SWF-013 on the real catalog: the C4 closure has no B row; the Fenrir gates are their B rows."""

    def closure(self, roots):
        import sys
        sys.path.insert(0, os.path.join(REPO, "models", "doctorwatson"))
        from factory import predicates as pm
        preds = pm.load(REPO)
        return {p.id: p for p in pm.closure(preds, roots)}

    def test_c4_closure_has_no_b_row(self):
        from factory import predicates as pm
        c = self.closure(pm.MILESTONES["sherlock-m6"])
        self.assertIn("C4", c)
        self.assertIn("A8p", c)
        self.assertEqual([i for i, p in c.items() if p.set == "B"], [])

    def test_fenrir_gates_contain_their_b_rows(self):
        from factory import predicates as pm
        pv = self.closure(pm.MILESTONES["fenrir-preview"])
        for i in ("B1", "B2", "B3", "B6", "B7", "B4p", "B5p", "B8p", "B9p"):
            self.assertIn(i, pv)
        self.assertFalse(any(i.endswith("m") and i.startswith("B") for i in pv))
        mn = self.closure(pm.MILESTONES["fenrir-mainnet"])
        for i in ("B4m", "B5m", "B8m", "B9m", "B9p"):
            self.assertIn(i, mn)


class TierRunnersScoped(unittest.TestCase):
    def test_t2_t3_pass_the_milestone(self):
        for t in ("t2.sh", "t3.sh"):
            with open(os.path.join(REPO, "qa", "tiers", t)) as fh:
                text = fh.read()
            self.assertIn("--milestone", text, t)
            self.assertIn("sherlock-m6", text, t)


if __name__ == "__main__":
    unittest.main()
