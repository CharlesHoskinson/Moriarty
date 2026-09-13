"""Actual runner.execute -> real installed main over a private AF_UNIX channel.

The runner runs in the test process. A fake inherited-environment launcher
execs the bound argv exactly as the pinned Foreman launcher would after
containment. The child is the real loan_executor.main with production
transports routed to the fake service executables. Forged, replayed, sibling
and stale-store cases deny without a second launch or debit.
"""
import json
import os
import pathlib
import signal
import socket
import sqlite3
import subprocess
import sys
import time
import unittest
from contextlib import closing

HERE = pathlib.Path(__file__).resolve().parent
if str(HERE) not in sys.path:
    sys.path.insert(0, str(HERE))

import harness  # noqa: E402
from harness import LoanFixture  # noqa: E402
from moriarty_dev import accounting, runner, store  # noqa: E402
from moriarty_dev.records import load_snapshot  # noqa: E402
from moriarty_dev.store import make_history_reader  # noqa: E402

FAKE_LAUNCHER = """import os, sys
args = sys.argv[1:]
assert args[:2] == ["--timeout", "1800"] and args[2:4] == ["--grace", "5"]
assert args[4:6] == ["--require-containment", "strong"] and args[6] == "--"
argv = args[7:]
os.execv(argv[0], argv)
"""


def store_rows(db_path, query):
    with closing(sqlite3.connect(db_path)) as conn:
        return conn.execute(query).fetchall()


class RunnerCase(unittest.TestCase):
    def setUp(self):
        self.fx = LoanFixture().build()
        self.addCleanup(self.fx.cleanup)
        self.fake = self.fx.root / "fake-launch.py"
        self.fake.write_text(FAKE_LAUNCHER)
        self.fx.write_runner(launcher=[self.fx.python, str(self.fake)])

    def reserve(self):
        snap = load_snapshot(str(self.fx.root), self.fx.action["id"], history_reader=make_history_reader(self.fx.db_path))
        return store.reserve(self.fx.db_path, str(self.fx.root), self.fx.action, snap, charge_id=harness.RUNTIME_CHARGE)

    def execute(self, argv=None):
        if argv is not None:
            self.fx.write_runner(launcher=[self.fx.python, str(self.fake)], argv=argv)
        rid = self.reserve()
        receipt = runner.execute(self.fx.root, self.fx.runner, self.fx.runner_digest, rid)
        return rid, receipt, receipt["runnerReceipt"]["loanResult"]

    def custom_child(self, name, body):
        script = self.fx.root / name
        script.write_text(body)
        return [self.fx.python, name, "--plan", self.fx.plan_rel, "--sha256", self.fx.plan_sha,
                "--admission", self.fx.admission_rel]


class Handoff(RunnerCase):
    def test_complete_handoff_launches_once_and_reads_the_durable_result(self):
        os.environ["MORIARTY_INVOCATION_NONCE"] = "hostile-inherited-value"
        os.environ["MORIARTY_INVOCATION_SOCKET"] = "/nonexistent/hostile.sock"
        self.addCleanup(os.environ.pop, "MORIARTY_INVOCATION_NONCE", None)
        self.addCleanup(os.environ.pop, "MORIARTY_INVOCATION_SOCKET", None)
        rid, receipt, loan = self.execute()
        self.assertEqual(receipt["exitCode"], 0, receipt["stderr"])
        self.assertEqual(loan["disposition"], "success", loan)
        self.assertTrue(loan["handshake"]["completed"] and loan["containmentAcknowledged"] and loan["invocationAppended"])
        self.assertTrue(loan["ownershipReleased"])
        self.assertEqual([p["phase"] for p in loan["progress"]], ["validated", "startup", "launched", "contained", "result"])
        result = json.loads((self.fx.root / loan["resultPath"]).read_text())
        self.assertEqual(result["invocationSha256"], loan["invocationSha256"])
        self.assertEqual(harness.sha256_file(self.fx.root / loan["resultPath"]), loan["resultSha256"])
        event = store.find_invocation_event(self.fx.db_path, loan["invocationSha256"])
        self.assertEqual(event["reservationId"], rid)
        self.assertEqual(event["parentPid"], os.getpid())
        self.assertEqual(event["chargeId"], harness.RUNTIME_CHARGE)
        self.assertLessEqual(event["outerDeadlineMonotonic"], event["outerStartMonotonic"] + 1800 * 10 ** 9)
        starts = [row for row in self.fx.services.calls() if "start" in row]
        self.assertEqual(len(starts), 1, "exact own claimed debit launches once")
        self.assertFalse(any(name.startswith("channel-") for name in os.listdir(self.fx.evidence_dir)),
                         "runner removes its own socket directory")
        ownership = json.loads(self.fx.ownership_path.read_text())
        self.assertEqual(ownership["state"], "released")
        self.assertEqual(ownership["runtimeReservationId"], rid)
        self.assertEqual(json.loads(self.fx.master_path.read_text())["externalPackageCharges"][0]["launchClaim"], "claimed")

    def test_forged_nonce_is_denied_without_launch(self):
        argv = self.custom_child("forged.py", """import json, os, socket, sys
path = os.environ["MORIARTY_INVOCATION_SOCKET"]
fd = os.open(os.path.dirname(path), os.O_RDONLY | os.O_DIRECTORY)
sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
sock.connect("/proc/self/fd/%d/%s" % (fd, os.path.basename(path)))
sock.sendall((json.dumps({"schema": "moriarty.loan-handshake/1", "nonce": "0" * 64, "authoritySha256": "0" * 64}) + "\\n").encode())
print(sock.recv(4096).decode())
raise SystemExit(2)
""")
        rid, receipt, loan = self.execute(argv)
        self.assertEqual(loan["disposition"], "unresolved")
        self.assertFalse(loan["handshake"]["completed"])
        self.assertEqual(loan["handshake"]["denied"], "NONCE")
        self.assertEqual(self.fx.services.calls(), [], "no service command after a denied handshake")
        self.assertEqual(json.loads(self.fx.master_path.read_text())["externalPackageCharges"][0]["launchClaim"], "claimed",
                         "claimed debit is not refunded or reset")
        self.assertEqual(store_rows(self.fx.db_path, "SELECT status FROM reservations"), [("active",)])

    def test_second_handshake_and_replay_are_refused(self):
        argv = self.custom_child("twice.py", """import json, os, socket, sys, hashlib
def connect():
    path = os.environ["MORIARTY_INVOCATION_SOCKET"]
    fd = os.open(os.path.dirname(path), os.O_RDONLY | os.O_DIRECTORY)
    s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM); s.connect("/proc/self/fd/%%d/%%s" %% (fd, os.path.basename(path))); return s
first = connect()
first.sendall((json.dumps({"schema": "moriarty.loan-handshake/1", "nonce": os.environ["MORIARTY_INVOCATION_NONCE"], "authoritySha256": %r}) + "\\n").encode())
answer = json.loads(first.recv(65536).decode().split("\\n")[0])
assert "payload" in answer, answer
try:
    second = connect()
    second.sendall(b"x\\n")
    data = second.recv(10)
    print("SECOND", data)
    raise SystemExit(5)
except OSError as error:
    print("SECOND_REFUSED", type(error).__name__)
    raise SystemExit(2)
""" % self.fx.admission_sha)
        rid, receipt, loan = self.execute(argv)
        self.assertTrue(loan["handshake"]["completed"])
        self.assertIn("SECOND_REFUSED", receipt["stdout"])
        self.assertEqual(receipt["exitCode"], 2)
        self.assertEqual(loan["disposition"], "unresolved", "no result document: unresolved, not refused")
        self.assertEqual(store_rows(self.fx.db_path, "SELECT status FROM reservations"), [("active",)])

    def test_sibling_process_outside_the_launcher_is_denied(self):
        argv = self.custom_child("sleeper.py", """import os, sys, time
sock = os.environ["MORIARTY_INVOCATION_SOCKET"]
open(os.path.join(os.path.dirname(sock), "..", "sleeper-ready"), "w").close()
time.sleep(3)
raise SystemExit(2)
""")
        self.fx.write_runner(launcher=[self.fx.python, str(self.fake)], argv=argv)
        rid = self.reserve()
        import threading
        outcome = {}

        def intruder():
            deadline = time.monotonic() + 5
            while time.monotonic() < deadline:
                ready = self.fx.evidence_dir / "sleeper-ready"
                sockets = list(self.fx.evidence_dir.glob("channel-*/channel.sock"))
                if ready.exists() and sockets:
                    dir_fd = os.open(str(sockets[0].parent), os.O_RDONLY | os.O_DIRECTORY)
                    sock = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
                    try:
                        sock.connect("/proc/self/fd/%d/channel.sock" % dir_fd)
                        sock.sendall((json.dumps({"schema": "moriarty.loan-handshake/1",
                                                  "nonce": "0" * 64, "authoritySha256": "0" * 64}) + "\n").encode())
                        outcome["answer"] = sock.recv(4096).decode()
                    finally:
                        sock.close()
                        os.close(dir_fd)
                    return
                time.sleep(0.02)
        thread = threading.Thread(target=intruder)
        thread.start()
        receipt = runner.execute(self.fx.root, self.fx.runner, self.fx.runner_digest, rid)
        thread.join(10)
        loan = receipt["runnerReceipt"]["loanResult"]
        self.assertIn("PEER_ANCESTRY", loan["handshake"]["denied"])
        self.assertIn("PEER_ANCESTRY", outcome.get("answer", ""))
        self.assertFalse(loan["handshake"]["completed"])
        self.assertEqual(loan["disposition"], "unresolved")

    def test_stale_store_event_denies_the_handshake(self):
        original = store.find_invocation_event
        store.find_invocation_event = lambda db_path, digest: None
        self.addCleanup(setattr, store, "find_invocation_event", original)
        rid, receipt, loan = self.execute()
        self.assertIn("invocation event", loan["handshake"]["denied"])
        self.assertEqual(receipt["exitCode"], 2)
        # No completed handshake: the child's report is diagnostic only (R2-02).
        self.assertEqual(loan["disposition"], "unresolved")
        self.assertEqual(self.fx.services.calls(), [])
        self.assertEqual(json.loads(self.fx.master_path.read_text())["externalPackageCharges"][0]["launchClaim"], "claimed")

    def test_failed_or_hung_invocation_append_never_answers(self):
        original = store.record_invocation

        def failing(*args, **kwargs):
            raise store.StoreError("injected append failure")
        store.record_invocation = failing
        self.addCleanup(setattr, store, "record_invocation", original)
        rid, receipt, loan = self.execute()
        self.assertFalse(loan["invocationAppended"])
        self.assertFalse(loan["handshake"]["attempted"])
        self.assertEqual(receipt["exitCode"], 2, "child refuses: no channel answered")
        self.assertEqual(loan["disposition"], "unresolved")
        self.assertEqual(store_rows(self.fx.db_path, "SELECT status FROM reservations"), [("active",)])
        # A hung append is killed at its bound by the supervised worker.
        fx2 = LoanFixture().build()
        self.addCleanup(fx2.cleanup)
        fake2 = fx2.root / "fake-launch.py"
        fake2.write_text(FAKE_LAUNCHER)
        fx2.write_runner(launcher=[fx2.python, str(fake2)])
        store.record_invocation = lambda *a, **k: time.sleep(30)
        bound = runner.APPEND_BOUND_SECONDS
        runner.APPEND_BOUND_SECONDS = 1
        self.addCleanup(setattr, runner, "APPEND_BOUND_SECONDS", bound)
        snap = load_snapshot(str(fx2.root), fx2.action["id"], history_reader=make_history_reader(fx2.db_path))
        rid2 = store.reserve(fx2.db_path, str(fx2.root), fx2.action, snap, charge_id=harness.RUNTIME_CHARGE)
        started = time.monotonic()
        receipt2 = runner.execute(fx2.root, fx2.runner, fx2.runner_digest, rid2)
        self.assertLess(time.monotonic() - started, 20)
        loan2 = receipt2["runnerReceipt"]["loanResult"]
        self.assertFalse(loan2["invocationAppended"])
        self.assertEqual(loan2["disposition"], "unresolved")
        self.assertIsNone(store.find_invocation_event(fx2.db_path, loan2["invocationSha256"]))

    def test_boot_mismatch_is_refused_by_the_child(self):
        argv = self.custom_child("wrong-boot.py", """import sys
sys.path.insert(0, %r)
from pathlib import Path
from moriarty_dev import loan_executor
deps = loan_executor.production_dependencies(Path.cwd(), service_table=%r)
deps["boot_id"] = lambda: "00000000-0000-0000-0000-000000000000"
deps["exchange_invocation"] = lambda admission: loan_executor.connect_parent_channel(
    admission, dict(__import__("os").environ), deps["boot_id"], __import__("time").monotonic_ns)
raise SystemExit(loan_executor.main(sys.argv[1:], dependencies=deps))
""" % (str(harness.SCRIPTS), self.fx.service_table))
        rid, receipt, loan = self.execute(argv)
        self.assertEqual(receipt["exitCode"], 2)
        # The child rejected the answer; its null-identity report cannot close the reservation.
        self.assertEqual(loan["disposition"], "unresolved")
        self.assertEqual(loan["failureCode"], "HANDSHAKE_DENIED")
        self.assertEqual(self.fx.services.calls(), [])
        self.assertEqual(store_rows(self.fx.db_path, "SELECT status FROM reservations"), [("active",)])

    def test_parent_loss_after_startup_yields_owned_cleanup_and_unknown(self):
        # The runner lives in a separate process that is killed after the unit exists.
        driver = self.fx.root / "runner-driver.py"
        driver.write_text("""import sys, json
sys.path.insert(0, %r)
from pathlib import Path
from moriarty_dev import runner, store
from moriarty_dev.records import load_snapshot
from moriarty_dev.store import make_history_reader
root = Path(%r)
plan = json.loads(Path(%r).read_text())
db = Path(%r)
snap = load_snapshot(str(root), plan["action"]["id"], history_reader=make_history_reader(db))
rid = store.reserve(db, str(root), plan["action"], snap, charge_id=plan["chargeId"])
receipt = runner.execute(root, plan, %r, rid)
print(json.dumps(receipt["runnerReceipt"]["loanResult"]))
""" % (str(harness.SCRIPTS), str(self.fx.root), str(self.fx.root / "runner-plan.json"), str(self.fx.db_path),
       self.fx.runner_digest))
        (self.fx.root / "runner-plan.json").write_text(json.dumps(self.fx.runner))
        self.fx.services._save("script.json", {"runningPolls": 30})
        proc = subprocess.Popen([self.fx.python, str(driver)], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            deadline = time.monotonic() + 20
            while time.monotonic() < deadline and self.fx.services.unit(harness.UNIT) is None:
                time.sleep(0.05)
            self.assertIsNotNone(self.fx.services.unit(harness.UNIT), "financial unit must have launched")
            os.kill(proc.pid, signal.SIGKILL)
            proc.communicate(timeout=10)
        finally:
            if proc.poll() is None:
                proc.kill()
        result_path = self.fx.root / self.fx.plan["resultPath"]
        deadline = time.monotonic() + 30
        while time.monotonic() < deadline and not result_path.exists():
            time.sleep(0.1)
        self.assertTrue(result_path.exists(), "child finished on its own after parent loss")
        result = json.loads(result_path.read_text())
        self.assertEqual((result["status"], result["failureCode"]), ("PROCESS_UNKNOWN", "PARENT_LOST"))
        self.assertTrue(result["containmentComplete"])
        self.assertEqual(store_rows(self.fx.db_path, "SELECT status FROM reservations"), [("active",)])
        ownership = json.loads(self.fx.ownership_path.read_text())
        self.assertEqual(ownership["state"], "runtime-held", "durable ownership stays unresolved after parent loss")


class ForemanTransport(unittest.TestCase):
    def test_pinned_strong_launcher_inherits_the_channel_environment(self):
        node = pathlib.Path(__import__("shutil").which("node")).resolve()
        env = dict(os.environ, MORIARTY_INVOCATION_SOCKET="/tmp/probe.sock", MORIARTY_INVOCATION_NONCE="ab" * 32)
        probe = ("import os, json; print(json.dumps({k: os.environ.get(k) for k in "
                 "('MORIARTY_INVOCATION_SOCKET', 'MORIARTY_INVOCATION_NONCE')}))")
        result = subprocess.run([str(node), harness.FOREMAN_LAUNCHER, "--timeout", "20", "--grace", "2",
                                 "--require-containment", "strong", "--", sys.executable, "-c", probe],
                                capture_output=True, text=True, env=env, timeout=60)
        self.assertEqual(result.returncode, 0, result.stderr)
        seen = json.loads(result.stdout.strip().splitlines()[-1])
        self.assertEqual(seen, {"MORIARTY_INVOCATION_SOCKET": "/tmp/probe.sock", "MORIARTY_INVOCATION_NONCE": "ab" * 32})
        self.assertIn("posix_pidns_userns_strong", result.stderr)


if __name__ == "__main__":
    unittest.main()
