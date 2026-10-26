"""scripts/tick-check and scripts/wait-event against fixtures (no live locks, no live bus)."""

import calendar
import json
import os
import subprocess
import sys
import tempfile
import threading
import time
import unittest

SCRIPTS = os.path.abspath(os.path.join(os.path.dirname(__file__), "../../../../scripts"))
TICK, WAIT = f"{SCRIPTS}/tick-check", f"{SCRIPTS}/wait-event"


def msg(i, frm, subj, ts="2026-10-25T10:00:00.0Z", kind="msg", to="coordinator"):
    return json.dumps({"id": i, "ts": ts, "from_name": frm, "to": to, "kind": kind, "subj": subj, "body": ""})


class Fx(unittest.TestCase):
    def setUp(self):
        self.d = tempfile.mkdtemp()
        j = lambda n: os.path.join(self.d, n)  # noqa: E731
        self.inbox = j("inbox.jsonl")
        self.recv = j("recv.sh")
        open(self.recv, "w").write(f"#!/bin/sh\ncat {self.inbox}\n")
        os.chmod(self.recv, 0o755)
        for n in ("heavy.lock", "gpu.lock"):
            open(j(n), "w").close()
        self.proc_locks = j("locks")
        os.makedirs(j("run"))
        open(j("run/agent-a.log"), "w").write("working\n")
        open(j("run/agent-b.log"), "w").write("exit 0\n")
        self.env = dict(os.environ, TC_RECV_CMD=self.recv, TC_STATE=j("state.json"), DW_LOCK=j("heavy.lock"),
                        DW_GPU_LOCK=j("gpu.lock"), TC_PROC_LOCKS=self.proc_locks, TC_RUN_DIR=j("run"),
                        TC_Q_CMD="/nonexistent-q", TC_REAP_CMD="/nonexistent-reap",
                        TC_NEXT_CMD="echo A: 1/10", TC_STOP_CMD="echo CONTINUE")
        self.set_locks(held=[])

    def set_locks(self, held):
        lines = []
        for i, n in enumerate(held):
            ino = os.stat(os.path.join(self.d, f"{n}.lock")).st_ino
            lines.append(f"{i + 1}: FLOCK  ADVISORY  WRITE {os.getpid()} 08:30:{ino} 0 EOF")
        open(self.proc_locks, "w").write("\n".join(lines) + "\n")

    def tick(self, *args, env=None):
        return subprocess.run([sys.executable, TICK, "--peek", *args], capture_output=True, text=True,
                              env=dict(self.env, **(env or {})), timeout=30).stdout


class TickCheck(Fx):
    def test_sections_and_stubs(self):
        open(self.inbox, "w").write(msg("1", "sherlock-u", "QUESTION U023 heavy slot") + "\n")
        out = self.tick()
        for s in ("== bus recv", "== open asks", "== locks / leases", "== agents", "== host",
                  "== heavy queue head", "== sherlock"):
            self.assertIn(s, out)
        self.assertIn("stub", out)  # q status + reap not landed
        self.assertIn("A: 1/10", out)
        self.assertIn("CONTINUE", out)

    def test_open_ask_age(self):
        ts = "2026-10-25T10:00:00.0Z"
        open(self.inbox, "w").write(msg("1", "p7", "BLOCKED waiting heavy", ts) + "\n")
        out = self.tick("--skip", "agents", env={"TC_NOW": str(calendar.timegm((2026, 10, 25, 11, 0, 0)))})
        self.assertRegex(out, r"1h00m\s+p7: BLOCKED waiting heavy")

    def test_lock_holder_and_free(self):
        self.set_locks(held=["heavy"])
        out = self.tick()
        self.assertRegex(out, r"heavy: HELD pid=\d+")
        self.assertIn("gpu: free", out)

    def test_agent_liveness_and_section_failure_isolated(self):
        out = self.tick(env={"TC_RECV_CMD": "/nonexistent-bus"})
        self.assertIn("unavailable", out)  # bus section degraded
        self.assertIn("exited   agent-b", out)  # later sections still ran
        self.assertIn("== sherlock", out)

    def test_fast(self):
        t = time.time()
        self.tick()
        self.assertLess(time.time() - t, 30)


class WaitEvent(Fx):
    def run_wait(self, *args, timeout=20):
        return subprocess.run([sys.executable, WAIT, "--interval", "0.2", *args], capture_output=True,
                              text=True, env=self.env, timeout=timeout)

    def test_lock_release(self):
        self.set_locks(held=["heavy", "gpu"])
        open(self.inbox, "w").close()
        threading.Timer(1.0, lambda: self.set_locks(held=["gpu"])).start()
        r = self.run_wait("--no-bus", "--timeout", "10")
        self.assertEqual(r.returncode, 0)
        self.assertEqual(r.stdout.strip(), "EVENT lock-release heavy")

    def test_bus_blocked_only(self):
        open(self.inbox, "w").write(msg("1", "x", "STATUS chatter") + "\n")
        threading.Timer(1.0, lambda: open(self.inbox, "a").write(msg("2", "sherlock-b", "BLOCKED B4") + "\n")).start()
        r = self.run_wait("--no-locks", "--timeout", "10")
        self.assertEqual(r.returncode, 0)
        self.assertIn("BLOCKED B4", r.stdout)
        self.assertNotIn("chatter", r.stdout)

    def test_since_now_ignores_existing(self):
        open(self.inbox, "w").write(msg("1", "x", "BLOCKED old") + "\n")
        r = self.run_wait("--no-locks", "--since-now", "--timeout", "1")
        self.assertEqual(r.returncode, 3)

    def test_timeout_and_no_lock_taken(self):
        r = self.run_wait("--timeout", "1")
        self.assertEqual(r.returncode, 3)
        import fcntl
        with open(os.path.join(self.d, "heavy.lock")) as fh:  # we can still take it: script never locked
            fcntl.flock(fh, fcntl.LOCK_EX | fcntl.LOCK_NB)


if __name__ == "__main__":
    unittest.main()
