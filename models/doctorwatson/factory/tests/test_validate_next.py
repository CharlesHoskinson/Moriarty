import json

from .helpers import FakeRepo, pred, yaml_of


class ValidateTests(FakeRepo):
    def test_ok(self):
        self.predicates(yaml_of(pred("A1"), pred("A2", depends=["A1"]),
                                pred("A3", heavy=True, budget="a3")))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_missing_file(self):
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("not found", r.stdout)

    def test_errors_listed(self):
        self.predicates(yaml_of(
            pred("A1"), pred("A1"),
            pred("A2", set="Z"), pred("A3", tier="T9"),
            pred("A4", depends=["NOPE"]),
            pred("A5", heavy=True),
            pred("C1", set="C", depends=["C2"]), pred("C2", set="C", depends=["C1"]),
        ))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        out = r.stdout
        self.assertIn("duplicate id", out)
        self.assertIn("set must be one of", out)
        self.assertIn("tier must be one of", out)
        self.assertIn("unknown id 'NOPE'", out)
        self.assertIn("heavy predicate needs a budget key", out)
        self.assertIn("dependency cycle among: C1, C2", out)

    def test_self_dependency(self):
        self.predicates(yaml_of(pred("A1", depends=["A1"])))
        r = self.run_cli("validate")
        self.assertEqual(r.returncode, 1)
        self.assertIn("depends on itself", r.stdout)


class NextTests(FakeRepo):
    def test_no_status(self):
        self.predicates(yaml_of(pred("A1", title="first one", check="scripts/qa/unimplemented.sh A1"),
                                pred("B1", set="B")))
        r = self.run_cli("next")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("A: 0/1", r.stdout)
        self.assertIn("B: 0/1", r.stdout)
        self.assertIn("(no results yet — run scripts/doctorwatson check --tier T0)", r.stdout)
        self.assertIn("FIRST FAILING: A1 — first one", r.stdout)
        self.assertIn("CHECK: scripts/qa/unimplemented.sh A1", r.stdout)

    def test_json_no_status(self):
        self.predicates(yaml_of(pred("A1")))
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertTrue(d["noResults"])
        self.assertEqual(d["firstFailing"]["id"], "A1")
        self.assertEqual(d["counts"]["A"], [0, 1])

    def test_topological_first_failing(self):
        # file order: A1 depends on A2, so A2 is first failing even though A1 is earlier in the file
        self.predicates(yaml_of(pred("A1", depends=["A2"]), pred("A2"), pred("A3")))
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertEqual(d["firstFailing"]["id"], "A2")

    def test_ties_by_file_order(self):
        self.predicates(yaml_of(pred("A2"), pred("A1"), pred("A3", depends=["A1"])))
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertEqual(d["firstFailing"]["id"], "A2")

    def test_uses_status_and_all_passing(self):
        self.predicates(yaml_of(pred("A1"), pred("A2", depends=["A1"])))
        self.assertEqual(self.run_cli("check").returncode, 0)
        r = self.run_cli("next")
        self.assertIn("A: 2/2", r.stdout)
        self.assertIn("ALL PASSING", r.stdout)
        self.assertNotIn("no results yet", r.stdout)

    def test_first_failing_after_partial(self):
        self.predicates(yaml_of(pred("A1"), pred("A2", check="exit 1"), pred("A3")))
        self.run_cli("check", "A1", "A2")
        d = json.loads(self.run_cli("next", "--json").stdout)
        self.assertEqual(d["firstFailing"]["id"], "A2")
        self.assertEqual(d["firstFailing"]["exit"], 1)
        self.assertEqual(d["counts"]["A"], [1, 3])

    def test_bad_yaml_exit_1(self):
        self.predicates(yaml_of(pred("A1", depends=["Q"])))
        self.assertEqual(self.run_cli("next").returncode, 1)
