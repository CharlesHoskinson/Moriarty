import datetime
import json
import os

from .helpers import FakeRepo


def iso(delta_hours=0):
    t = datetime.datetime.now(datetime.timezone.utc) - datetime.timedelta(hours=delta_hours)
    return t.strftime("%Y-%m-%dT%H:%M:%SZ")


class StopRuleTests(FakeRepo):
    def put_status(self, iters, flip_hours_ago=1):
        self.write("qa/sherlock/status.json", json.dumps({
            "iterationsSinceFlip": iters,
            "lastFlip": {"id": "A1", "at": iso(flip_hours_ago)}}))

    def test_continue(self):
        self.put_status(2)
        r = self.run_cli("stop-rule")
        self.assertEqual(r.returncode, 0)
        self.assertIn("CONTINUE", r.stdout)

    def test_stop_on_iterations_with_trace(self):
        self.put_status(3)
        self.write("qa/sherlock/runs.jsonl",
                   json.dumps({"id": "A1", "exit": 1, "trace": "qa/sherlock/traces/old.log"}) + "\n" +
                   json.dumps({"id": "A1", "exit": 0, "trace": "qa/sherlock/traces/ok.log"}) + "\n" +
                   json.dumps({"id": "A2", "exit": 4, "trace": "qa/sherlock/traces/last-fail.log"}) + "\n" +
                   json.dumps({"id": "A3", "exit": 0, "trace": "qa/sherlock/traces/ok2.log"}) + "\n")
        r = self.run_cli("stop-rule")
        self.assertEqual(r.returncode, 6)
        self.assertIn("STOP: escalate to owner", r.stdout)
        self.assertIn("qa/sherlock/traces/last-fail.log", r.stdout)

    def test_stop_on_24h(self):
        self.put_status(0, flip_hours_ago=25)
        r = self.run_cli("stop-rule")
        self.assertEqual(r.returncode, 6)
        self.assertIn("STOP", r.stdout)

    def test_23h_continues(self):
        self.put_status(1, flip_hours_ago=23)
        self.assertEqual(self.run_cli("stop-rule").returncode, 0)

    def test_json(self):
        self.put_status(3)
        r = self.run_cli("stop-rule", "--json")
        self.assertEqual(r.returncode, 6)
        d = json.loads(r.stdout)
        self.assertTrue(d["stop"])
        self.assertEqual(d["iterationsSinceFlip"], 3)

    def test_no_status_continues(self):
        self.assertEqual(self.run_cli("stop-rule").returncode, 0)

    def test_end_to_end_with_check(self):
        from .helpers import pred, yaml_of
        self.predicates(yaml_of(pred("A1", check="exit 1")))
        for _ in range(2):
            self.run_cli("check")
        self.assertEqual(self.run_cli("stop-rule").returncode, 0)
        self.run_cli("check")
        r = self.run_cli("stop-rule")
        self.assertEqual(r.returncode, 6)
        self.assertIn("qa/sherlock/traces/", r.stdout)


class HoldoutTests(FakeRepo):
    def seal(self):
        return self.run_cli("holdout", "seal", env={"DW_ROLE": "evaluator"})

    def make(self):
        self.write("qa/holdout/s1/in.json", "{}")
        self.write("qa/holdout/s2.txt", "hello")
        r = self.seal()
        self.assertEqual(r.returncode, 0, r.stdout)

    def test_seal_refused_without_role(self):
        self.write("qa/holdout/s.txt", "x")
        r = self.run_cli("holdout", "seal")
        self.assertEqual(r.returncode, 8)
        self.assertFalse(os.path.exists(self.p("qa/holdout/MANIFEST.sha256")))
        r = self.run_cli("holdout", "seal", env={"DW_ROLE": "driver"})
        self.assertEqual(r.returncode, 8)

    def test_manifest_format(self):
        self.make()
        with open(self.p("qa/holdout/MANIFEST.sha256")) as fh:
            lines = fh.read().splitlines()
        self.assertEqual([l.split("  ", 1)[1] for l in lines], ["s1/in.json", "s2.txt"])
        self.assertTrue(all(len(l.split("  ", 1)[0]) == 64 for l in lines))

    def test_verify_ok(self):
        self.make()
        r = self.run_cli("holdout", "verify")
        self.assertEqual(r.returncode, 0, r.stdout)

    def test_changed(self):
        self.make()
        self.write("qa/holdout/s2.txt", "tampered")
        r = self.run_cli("holdout", "verify")
        self.assertEqual(r.returncode, 7)
        self.assertIn("CHANGED s2.txt", r.stdout)

    def test_missing(self):
        self.make()
        os.remove(self.p("qa/holdout/s1/in.json"))
        r = self.run_cli("holdout", "verify")
        self.assertEqual(r.returncode, 7)
        self.assertIn("MISSING s1/in.json", r.stdout)

    def test_unlisted(self):
        self.make()
        self.write("qa/holdout/extra.txt", "new")
        r = self.run_cli("holdout", "verify")
        self.assertEqual(r.returncode, 7)
        self.assertIn("UNLISTED extra.txt", r.stdout)

    def test_no_manifest(self):
        self.write("qa/holdout/s.txt", "x")
        self.assertEqual(self.run_cli("holdout", "verify").returncode, 7)

    def test_reseal_after_change(self):
        self.make()
        self.write("qa/holdout/s2.txt", "legit change")
        self.assertEqual(self.seal().returncode, 0)
        self.assertEqual(self.run_cli("holdout", "verify").returncode, 0)

    def test_manifest_edit_detected(self):
        self.make()
        mp = self.p("qa/holdout/MANIFEST.sha256")
        with open(mp) as fh:
            text = fh.read()
        with open(mp, "w") as fh:
            fh.write("0" * 64 + text[64:])
        r = self.run_cli("holdout", "verify")
        self.assertEqual(r.returncode, 7)
        self.assertIn("CHANGED", r.stdout)
