"""Identity-safe cleanup helper: exact invocation/container-generation checks."""
import hashlib
import json
import pathlib
import subprocess
import sys
import tempfile
import time
import unittest

HERE = pathlib.Path(__file__).resolve().parent
SCRIPTS = HERE.parents[1] / "scripts"
for entry in (str(HERE), str(SCRIPTS)):
    if entry not in sys.path:
        sys.path.insert(0, entry)

import fake_services  # noqa: E402
from moriarty_dev import loan_cleanup  # noqa: E402

CONTAINER = "d" * 64
INVOCATION = "e" * 64


def intent_for(tmp):
    return {
        "schema": loan_cleanup.INTENT_SCHEMA, "allocationId": "alloc-01", "unit": "fixture.service",
        "timerUnit": "fixture-cleanup", "containerId": CONTAINER, "invocationSha256": INVOCATION,
        "chargeId": "c", "reservationId": "r", "evidenceDirectory": str(tmp), "proverReceiptPath": str(tmp / "prover-ownership.json"),
        "unitReceiptPath": str(tmp / "unit-ownership.json"),
    }


def prover_receipt(started):
    return {"schema": loan_cleanup.PROVER_RECEIPT_SCHEMA, "allocationId": "alloc-01", "invocationSha256": INVOCATION,
            "chargeId": "c", "reservationId": "r", "containerId": CONTAINER, "containerStartedAt": started,
            "startArgv": [], "startReturnCode": 0, "observedAtUtc": "2026-09-13T00:00:00+00:00"}


def unit_receipt(invocation_id, cgroup="/user.slice/user-1000.slice/user@1000.service/app.slice/fixture.service"):
    return {"schema": loan_cleanup.UNIT_RECEIPT_SCHEMA, "allocationId": "alloc-01", "invocationSha256": INVOCATION,
            "chargeId": "c", "reservationId": "r", "unit": "fixture.service", "unitInvocationId": invocation_id,
            "unitControlGroup": cgroup, "unitMainPid": "4242",
            "observedAtUtc": "2026-09-13T00:00:00+00:00"}


class CleanupIdentity(unittest.TestCase):
    def setUp(self):
        self.tmp = pathlib.Path(tempfile.mkdtemp(prefix="moriarty-cleanup-"))
        self.services = fake_services.FakeServices(self.tmp / "state")
        self.run = fake_services.run_command_for(self.services)
        self.intent = intent_for(self.tmp)
        self.services.seed_container(fake_services.container_config(CONTAINER, "sha256:" + "0" * 64, 1, "/w", "/c",
                                                                     "/w", "/c", "/c/x"))
        self.services._save("script.json", {"dockerStartedAt": "2026-09-13T20:00:00Z", "invocationId": "f" * 32})
        self.run(["/usr/bin/docker", "start", CONTAINER])
        self.run(["/usr/bin/systemd-run", "--user", "--unit=fixture.service", "--", "/bin/true"])

    def kills(self):
        return [row for row in self.services.calls() if "kill" in row]

    def test_exact_identities_are_killed_and_contained(self):
        facts = loan_cleanup.contain_owned(self.intent, prover_receipt("2026-09-13T20:00:00Z"), unit_receipt("f" * 32),
                                           self.run, time.monotonic_ns() + 25 * 10 ** 9)
        self.assertTrue(facts["complete"], facts)
        self.assertEqual(facts["unit"]["disposition"], "killed")
        self.assertEqual(facts["prover"]["disposition"], "killed")
        self.assertEqual(len(self.kills()), 2)
        self.assertEqual(facts["outstandingOwners"], [])
        for row in facts["commands"]:
            self.assertEqual(row["argv"][:2], ["/usr/bin/timeout", "--signal=KILL"], "every argv carries its bound")

    def test_changed_invocation_id_is_never_killed(self):
        facts = loan_cleanup.contain_owned(self.intent, prover_receipt("2026-09-13T20:00:00Z"), unit_receipt("a" * 32),
                                           self.run, time.monotonic_ns() + 25 * 10 ** 9)
        self.assertFalse(facts["complete"])
        self.assertEqual(facts["unit"]["disposition"], "invocation-mismatch-not-killed")
        self.assertEqual([row for row in self.kills() if "systemctl" in row[3]], [])
        self.assertEqual(facts["outstandingOwners"][0]["resource"], "unit:fixture.service")

    def test_replacement_container_generation_is_never_killed(self):
        facts = loan_cleanup.contain_owned(self.intent, prover_receipt("2026-09-13T19:00:00Z"), unit_receipt("f" * 32),
                                           self.run, time.monotonic_ns() + 25 * 10 ** 9)
        self.assertFalse(facts["complete"])
        self.assertEqual(facts["prover"]["disposition"], "replacement-generation-not-killed")
        self.assertEqual([row for row in self.kills() if "docker" in row[3]], [])
        self.assertTrue(any(o["resource"].startswith("container:") for o in facts["outstandingOwners"]))

    def test_missing_receipts_leave_ownership_unresolved_without_guessing(self):
        facts = loan_cleanup.contain_owned(self.intent, None, None, self.run, time.monotonic_ns() + 25 * 10 ** 9)
        self.assertFalse(facts["complete"])
        self.assertEqual(facts["unit"]["disposition"], "no-ownership-receipt")
        self.assertEqual(facts["prover"]["disposition"], "no-ownership-receipt")
        self.assertEqual(self.kills(), [])

    def test_in_process_startup_observation_may_stand_in_for_a_lost_receipt(self):
        facts = loan_cleanup.contain_owned(self.intent, None, None, self.run, time.monotonic_ns() + 25 * 10 ** 9,
                                           unit_started={"InvocationID": "f" * 32,
                                                         "ControlGroup": "/user.slice/user-1000.slice/user@1000.service/app.slice/fixture.service"},
                                           prover_started={"StartedAt": "2026-09-13T20:00:00Z"})
        self.assertTrue(facts["complete"])
        self.assertEqual(len(self.kills()), 2)

    def test_absent_resources_are_contained_without_commands(self):
        self.run(["/usr/bin/systemctl", "--user", "stop", "fixture.service"])
        self.run(["/usr/bin/docker", "kill", CONTAINER])
        facts = loan_cleanup.contain_owned(self.intent, prover_receipt("2026-09-13T20:00:00Z"), unit_receipt("f" * 32),
                                           self.run, time.monotonic_ns() + 25 * 10 ** 9)
        self.assertTrue(facts["complete"])
        self.assertEqual(facts["unit"]["disposition"], "absent-or-inactive")
        self.assertEqual(facts["prover"]["disposition"], "not-running")

    def test_unavailable_observation_is_unresolved(self):
        self.services._save("script.json", {"identityShowTimeout": True})
        facts = loan_cleanup.contain_owned(self.intent, prover_receipt("2026-09-13T20:00:00Z"), unit_receipt("f" * 32),
                                           self.run, time.monotonic_ns() + 25 * 10 ** 9)
        self.assertFalse(facts["unit"]["contained"])
        self.assertEqual(facts["unit"]["disposition"], "observation-unavailable")

    def test_intent_loading_requires_exact_digest_and_closed_fields(self):
        path = self.tmp / "intent.json"
        raw = json.dumps(self.intent).encode()
        path.write_bytes(raw)
        digest = hashlib.sha256(raw).hexdigest()
        self.assertEqual(loan_cleanup.load_intent(path, digest)["unit"], "fixture.service")
        with self.assertRaisesRegex(ValueError, "INTENT_DIGEST_MISMATCH"):
            loan_cleanup.load_intent(path, "0" * 64)
        link = self.tmp / "link.json"
        link.symlink_to(path)
        with self.assertRaisesRegex(ValueError, "INTENT_PATH_REQUIRED"):
            loan_cleanup.load_intent(link, digest)
        extra = dict(self.intent, extra=1)
        with self.assertRaisesRegex(ValueError, "INTENT_FIELDS"):
            loan_cleanup.validate_intent(extra)
        with self.assertRaisesRegex(ValueError, "INTENT_CONTAINER_ID"):
            loan_cleanup.validate_intent(dict(self.intent, containerId="short"))

    def test_main_refuses_bad_intent_and_help_is_inert(self):
        script = SCRIPTS / "moriarty_dev" / "loan_cleanup.py"
        helped = subprocess.run([sys.executable, str(script), "--help"], capture_output=True, text=True)
        self.assertEqual(helped.returncode, 0)
        self.assertIn("--intent", helped.stdout)
        self.assertEqual(self.services.calls()[-1][1], "--user")  # only setUp's fake commands were issued
        bad = subprocess.run([sys.executable, str(script), "--intent", str(self.tmp / "missing.json"),
                              "--sha256", "0" * 64], capture_output=True, text=True)
        self.assertEqual(bad.returncode, 2)
        self.assertIn("cleanup refused", bad.stderr)

    def test_timer_argv_builders(self):
        argv = loan_cleanup.timer_arm_argv("fixture-cleanup", 1600, "/usr/bin/python3", "/x/loan_cleanup.py",
                                           "/tmp/intent.json", "a" * 64)
        self.assertEqual(argv[:5], ["/usr/bin/systemd-run", "--user", "--unit=fixture-cleanup", "--on-active=1600s",
                                    "--timer-property=AccuracySec=1s"])
        self.assertEqual(loan_cleanup.timer_cancel_argv("fixture-cleanup", 40)[2], "25s")
        with self.assertRaisesRegex(ValueError, "TIMER_UNIT_REQUIRED"):
            loan_cleanup.require_timer_name("bad.service")


if __name__ == "__main__":
    unittest.main()
