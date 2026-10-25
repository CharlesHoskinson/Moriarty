import unittest
from .helpers import FakeRepo, pred, yaml_of


class CheckTests(FakeRepo):
    def test_all_pass_exit_0_and_status(self):
        self.predicates(yaml_of(pred("A1"), pred("B1", set="B")))
        r = self.run_cli("check")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("A1: PASS", r.stdout)
        st = self.status()
        self.assertEqual(st["schema"], "doctorwatson.status/1")
        self.assertEqual(st["counts"]["A"], [1, 1])
        self.assertEqual(st["counts"]["B"], [1, 1])
        self.assertEqual(st["counts"]["C"], [0, 0])
        self.assertIsNone(st["firstFailing"])
        self.assertTrue(st["predicates"]["A1"]["passes"])
        self.assertEqual(st["predicates"]["A1"]["exit"], 0)
        self.assertEqual(len(self.runs()), 2)
        run = self.runs()[0]
        for key in ("run", "id", "cmd", "hypothesis", "exit", "wallSeconds", "maxRssBytes", "trace", "head"):
            self.assertIn(key, run)

    def test_fail_exit_1(self):
        self.predicates(yaml_of(pred("A1", check="echo boom; exit 1"), pred("A2")))
        r = self.run_cli("check")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A1: FAIL exit=1", r.stdout)
        st = self.status()
        self.assertEqual(st["firstFailing"], {"id": "A1", "check": "echo boom; exit 1", "exit": 1})
        trace = self.runs()[0]["trace"]
        with open(self.p(trace)) as fh:
            self.assertIn("boom", fh.read())

    def test_unimplemented_exit_2_recorded(self):
        self.write("scripts/qa/unimplemented.sh", 'echo "UNIMPLEMENTED $1"; exit 2\n', 0o755)
        self.predicates(yaml_of(pred("A1", check="scripts/qa/unimplemented.sh A1")))
        r = self.run_cli("check")
        self.assertEqual(r.returncode, 1)
        self.assertEqual(self.status()["firstFailing"]["exit"], 2)

    def test_blocked_by_dependency(self):
        self.predicates(yaml_of(pred("A1", check="exit 1"), pred("A2", depends=["A1"], check="touch ran-a2"),
                                pred("A3", depends=["A2"], check="touch ran-a3")))
        r = self.run_cli("check")
        self.assertEqual(r.returncode, 1)
        self.assertIn("A2: BLOCKED-BY A1", r.stdout)
        self.assertIn("A3: BLOCKED-BY A2", r.stdout)
        import os
        self.assertFalse(os.path.exists(self.p("ran-a2")))
        self.assertFalse(os.path.exists(self.p("ran-a3")))
        self.assertFalse(self.status()["predicates"]["A2"]["passes"])

    def test_explicit_id_is_still_blocked(self):
        # naming a predicate must not bypass its depends gate (it printed a false C2 PASS)
        import os
        self.predicates(yaml_of(pred("U1", set="U", check="exit 1"),
                                pred("C2", set="C", depends=["U1"], check="touch ran-c2")))
        r = self.run_cli("check", "C2")
        self.assertEqual(r.returncode, 1, r.stdout)
        self.assertIn("C2: BLOCKED-BY U1", r.stdout)
        self.assertNotIn("C2: PASS", r.stdout)
        self.assertFalse(os.path.exists(self.p("ran-c2")))
        st = self.status()["predicates"]["C2"]
        self.assertFalse(st["passes"])
        self.assertEqual(st.get("blockedBy"), "U1")

    def test_explicit_ids_run_dependency_first(self):
        import os
        self.predicates(yaml_of(pred("A1"), pred("A2", depends=["A1"], check="touch ran-a2")))
        r = self.run_cli("check", "A2", "A1")  # topo order: A1 passes in this run, then A2 runs
        self.assertEqual(r.returncode, 0, r.stdout)
        self.assertTrue(os.path.exists(self.p("ran-a2")))

    def test_dependency_status_from_previous_run(self):
        import os
        self.predicates(yaml_of(pred("A1"), pred("A2", depends=["A1"], check="touch ran-a2")))
        self.run_cli("check", "A1")
        r = self.run_cli("check", "--set", "A")  # A1 passes, then A2 runs
        self.assertEqual(r.returncode, 0)
        self.assertTrue(os.path.exists(self.p("ran-a2")))

    def test_selection_by_set_and_tier(self):
        self.predicates(yaml_of(pred("A1", tier="T0"), pred("A2", tier="T1"), pred("B1", set="B", tier="T1")))
        r = self.run_cli("check", "--tier", "T1")
        self.assertIn("A2:", r.stdout)
        self.assertIn("B1:", r.stdout)
        self.assertNotIn("A1:", r.stdout)
        r = self.run_cli("check", "--set", "B")
        self.assertIn("B1:", r.stdout)
        self.assertNotIn("A2:", r.stdout)

    def test_unknown_id(self):
        self.predicates(yaml_of(pred("A1")))
        r = self.run_cli("check", "ZZ")
        self.assertEqual(r.returncode, 2)

    def test_runs_from_repo_root(self):
        self.predicates(yaml_of(pred("A1", check="test -f marker")))
        self.write("marker", "x")
        self.assertEqual(self.run_cli("check").returncode, 0)

    def test_lastflip_and_iterations(self):
        self.predicates(yaml_of(pred("A1", check="test -f flag"), pred("A2")))
        self.run_cli("check", "A1")  # fails, no flip
        self.assertEqual(self.status()["iterationsSinceFlip"], 1)
        self.assertIsNone(self.status()["lastFlip"])
        self.run_cli("check", "A1")
        self.assertEqual(self.status()["iterationsSinceFlip"], 2)
        self.write("flag", "")
        self.run_cli("check", "A1")  # false -> true
        st = self.status()
        self.assertEqual(st["iterationsSinceFlip"], 0)
        self.assertEqual(st["lastFlip"]["id"], "A1")
        self.run_cli("check", "A1")  # pass -> pass, not a flip
        st = self.status()
        self.assertEqual(st["iterationsSinceFlip"], 1)
        self.assertEqual(st["lastFlip"]["id"], "A1")

    def test_status_atomic_no_tmp_left(self):
        import os
        self.predicates(yaml_of(pred("A1")))
        self.run_cli("check")
        self.assertEqual([f for f in os.listdir(self.p("qa/sherlock")) if ".tmp" in f], [])

    def test_hypothesis_recorded(self):
        self.predicates(yaml_of(pred("A1")))
        self.run_cli("check", "--hypothesis", "because")
        self.assertEqual(self.runs()[0]["hypothesis"], "because")

    def test_heavy_predicate_goes_through_heavy_and_measures(self):
        self.predicates(yaml_of(pred("A1", check="true", heavy=True, budget="k1")))
        self.seed_budget("k1")
        r = self.run_cli("check")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        run = self.runs()[0]
        self.assertIsNotNone(run["preflight"])
        self.assertIsNotNone(self.budgets()["budgets"]["k1"]["measured"]["wallSeconds"])

    def test_heavy_predicate_unfit_not_recorded_as_result(self):
        self.predicates(yaml_of(pred("A1", check="true", heavy=True, budget="k1")))
        r = self.run_cli("check", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "99999999"})
        self.assertEqual(r.returncode, 1)
        self.assertIn("exit=3", r.stdout)
        self.assertNotIn("A1", self.status()["predicates"])
        self.assertEqual(self.runs(), [])


class ConcurrentStatusUpdateTests(unittest.TestCase):
    def test_parallel_updates_lose_nothing(self):
        import tempfile
        import threading

        from factory import status
        from factory.predicates import Predicate

        with tempfile.TemporaryDirectory() as d:
            preds = [Predicate(id=f"F{i}", set="F", title="t", tier="T0", check="true")
                     for i in range(1, 41)]
            threads = [threading.Thread(target=status.update,
                                        args=(preds, {p.id: {"passes": True, "exit": 0}}, True),
                                        kwargs={"repo": d}) for p in preds]
            for t in threads:
                t.start()
            for t in threads:
                t.join()
            self.assertEqual(status.load(d)["counts"]["F"], [40, 40])
