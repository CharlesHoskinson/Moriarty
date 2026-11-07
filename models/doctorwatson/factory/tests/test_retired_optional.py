"""E10/E14 (SWF-011, SWF-012): retired and optional predicate rows. Predicate F16."""

import json
import os

from .helpers import FakeRepo, pred, yaml_of

UNIMPL = 'echo "UNIMPLEMENTED $1"; exit 2\n'


class RetiredOptionalValidate(FakeRepo):
    def test_fields_accepted(self):
        self.predicates(yaml_of(pred("A8", tier="T3", retired="stagenet row; see A8p"),
                                pred("B4p", set="B", tier="T3", optional=True),
                                pred("B5p", set="B", tier="T3", optional=True, depends=["B4p"]),
                                pred("B5", set="B", retired="stagenet", depends=["A8"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_field_types(self):
        self.predicates(yaml_of(pred("A1", retired=""), pred("A2", optional="yes")))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A1: retired must be a non-empty reason or anchor string", r.stdout)
        self.assertIn("A2: optional must be true or false", r.stdout)

    def test_mandatory_on_retired_refused(self):
        self.predicates(yaml_of(pred("A8", retired="stagenet"), pred("C4", set="C", depends=["A8"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("C4: active row depends on retired row 'A8'", r.stdout)

    def test_optional_on_retired_refused(self):
        self.predicates(yaml_of(pred("B4", set="B", retired="stagenet"),
                                pred("B5p", set="B", optional=True, depends=["B4"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("B5p: active row depends on retired row 'B4'", r.stdout)

    def test_mandatory_on_optional_refused(self):
        self.predicates(yaml_of(pred("B4p", set="B", optional=True), pred("A9p", depends=["B4p"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A9p: mandatory row depends on optional row 'B4p'", r.stdout)


class RetiredA8(FakeRepo):
    """SWF-011 scenario: A8 retired and failing, A8p unimplemented."""

    def setUp(self):
        super().setUp()
        self.write("scripts/qa/unimplemented.sh", UNIMPL, 0o755)
        self.predicates(yaml_of(
            pred("A1", check="true"),
            pred("A8", tier="T3", retired="stagenet row; replaced by A8p", check="touch ran-a8; exit 1",
                 depends=["A1"]),
            pred("A8p", tier="T3", check="scripts/qa/unimplemented.sh A8p", depends=["A1"]),
        ))
        self.assertEqual(self.run_cli("check", "A1").returncode, 0)

    def test_next_names_a8p_and_lists_retired(self):
        r = self.run_cli("next")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("FIRST FAILING: A8p", r.stdout)
        self.assertIn("RETIRED: 1", r.stdout)
        self.assertIn("RETIRED: A8 (stagenet row; replaced by A8p)", r.stdout)
        self.assertIn("A: 1/2", r.stdout)  # retired rows do not count
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertEqual(d["firstFailing"]["id"], "A8p")
        self.assertEqual(d["retired"], [{"id": "A8", "retired": "stagenet row; replaced by A8p"}])

    def test_check_tier_t3_ignores_a8(self):
        r = self.run_cli("check", "--tier", "T3")
        self.assertIn("A8p: FAIL exit=2", r.stdout)
        self.assertNotIn("A8:", r.stdout)
        self.assertFalse(os.path.exists(self.p("ran-a8")))
        self.assertNotIn("A8", self.status()["predicates"])

    def test_check_set_a_ignores_a8(self):
        r = self.run_cli("check", "--set", "A")
        self.assertIn("A1: PASS", r.stdout)
        self.assertIn("A8p: FAIL exit=2", r.stdout)
        self.assertNotIn("A8:", r.stdout)
        self.assertFalse(os.path.exists(self.p("ran-a8")))

    def test_include_optional_still_skips_retired(self):
        r = self.run_cli("check", "--include-optional")
        self.assertNotIn("A8:", r.stdout)
        self.assertFalse(os.path.exists(self.p("ran-a8")))

    def test_explicit_check_a8_runs_and_history_retained(self):
        r = self.run_cli("check", "A8")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A8: FAIL exit=1", r.stdout)
        self.assertTrue(os.path.exists(self.p("ran-a8")))
        self.assertFalse(self.status()["predicates"]["A8"]["passes"])
        # a later sweep leaves the A8 entry and its runs.jsonl line in place
        self.run_cli("check", "--set", "A")
        self.assertIn("A8", self.status()["predicates"])
        self.assertIn("A8", [rec["id"] for rec in self.runs()])
        self.assertEqual(self.status()["firstFailing"]["id"], "A8p")


class OptionalPreviewGate(FakeRepo):
    """SWF-012 scenario: optional B rows never block the Preview gate; mandatory rows do.

    The gate is `next` naming no FIRST FAILING row and `check --tier T3` exiting 0.
    """

    def catalog(self, mandatory_check="true", which="A8p"):
        self.write("scripts/qa/unimplemented.sh", UNIMPL, 0o755)
        rows = {"A8p": "A", "U023": "U", "F16": "F"}
        mand = [pred(i, set=s, tier="T3",
                     check=(mandatory_check if i == which else "true")) for i, s in rows.items()]
        self.predicates(yaml_of(
            *mand,
            pred("A8", tier="T3", retired="stagenet", check="exit 1"),
            pred("B4p", set="B", tier="T3", optional=True, check="scripts/qa/unimplemented.sh B4p"),
            pred("B6", set="B", tier="T3", optional=True, check="exit 1"),
            pred("B7", set="B", tier="T3", optional=True, check="exit 1"),
        ))

    def gate(self):
        r = self.run_cli("check", "--tier", "T3")
        d = json.loads(self.run_cli("next", "--json").stdout)
        return r, d

    def test_optional_failures_leave_gate_eligible(self):
        self.catalog()
        r, d = self.gate()
        self.assertEqual(r.returncode, 0, r.stdout)
        for i in ("B4p", "B6", "B7", "A8"):
            self.assertNotIn(f"{i}:", r.stdout)
        self.assertIsNone(d["firstFailing"])
        self.assertEqual(d["optional"], [0, 3])
        self.assertIsNone(self.status()["firstFailing"])
        self.assertIn("ALL PASSING", self.run_cli("next").stdout)

    def test_include_optional_reports_b_rows(self):
        self.catalog()
        self.assertEqual(self.run_cli("check", "--tier", "T3").returncode, 0)
        r = self.run_cli("check", "--include-optional")
        self.assertEqual(r.returncode, 1)
        self.assertIn("B4p: FAIL exit=2", r.stdout)
        self.assertIn("B6: FAIL exit=1", r.stdout)
        self.assertIn("B7: FAIL exit=1", r.stdout)
        self.assertNotIn("A8:", r.stdout)
        # still not gating by default; gating on request
        self.assertIsNone(json.loads(self.run_cli("next", "--json").stdout)["firstFailing"])
        d = json.loads(self.run_cli("next", "--json", "--include-optional").stdout)
        self.assertEqual(d["firstFailing"]["id"], "B4p")

    def test_explicit_optional_check_runs(self):
        self.catalog()
        r = self.run_cli("check", "B6")
        self.assertEqual(r.returncode, 1)
        self.assertIn("B6: FAIL exit=1", r.stdout)

    def test_unimplemented_mandatory_blocks(self):
        for row in ("A8p", "U023", "F16"):
            with self.subTest(row=row):
                self.catalog(mandatory_check=f"scripts/qa/unimplemented.sh {row}", which=row)
                if os.path.exists(self.p("qa/sherlock/status.json")):
                    os.remove(self.p("qa/sherlock/status.json"))
                r, d = self.gate()
                self.assertEqual(r.returncode, 1, r.stdout)
                self.assertIn(f"{row}: FAIL exit=2", r.stdout)
                self.assertEqual(d["firstFailing"]["id"], row)
