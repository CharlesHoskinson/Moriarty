import json
import os
import signal
import subprocess
import sys
import time
import unittest

from .helpers import FakeRepo


def proc_starttime(pid):
    with open(f"/proc/{pid}/stat") as fh:
        return int(fh.read().rsplit(")", 1)[1].split()[19])


class ReapTest(FakeRepo):
    def setUp(self):
        super().setUp()
        self.gpu = os.path.join(self._tmp.name, "gpu.lock")
        self.env["DW_GPU_LOCK"] = self.gpu
        self.enable = os.path.join(self._tmp.name, "reap-enabled")
        self.env["DW_REAP_ENABLE"] = self.enable
        self.kids = []
        self.addCleanup(self._cleanup)

    def _cleanup(self):
        for k in self.kids:
            try:
                os.killpg(k.pid, signal.SIGKILL)
            except OSError:
                pass
            k.wait()

    def job(self):
        k = subprocess.Popen(["sleep", "300"], start_new_session=True)
        self.kids.append(k)
        return k

    def lease(self, k, deadline, starttime=None, sid=None, lock="heavy"):
        path = (self.lock if lock == "heavy" else self.gpu) + ".lease"
        with open(path, "w") as fh:
            json.dump({"v": 1, "id": "T", "lock": lock, "pid": k.pid,
                       "starttime": starttime if starttime is not None else proc_starttime(k.pid),
                       "sid": sid if sid is not None else k.pid, "deadline": deadline}, fh)

    def alive(self, k):
        return k.poll() is None

    def test_dry_run_default_reports_expired_and_kills_nothing(self):
        k = self.job()
        self.lease(k, time.time() - 5)
        r = self.run_cli("reap")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("EXPIRED", r.stdout)
        self.assertIn("would", r.stdout)
        self.assertTrue(self.alive(k))

    def test_unexpired_lease_is_not_a_target(self):
        k = self.job()
        self.lease(k, time.time() + 600)
        r = self.run_cli("reap", "--execute")
        self.assertIn("ok", r.stdout)
        self.assertNotIn("EXPIRED", r.stdout)
        self.assertTrue(self.alive(k))

    def test_execute_refused_without_owner_enable_file(self):
        k = self.job()
        self.lease(k, time.time() - 5)
        r = self.run_cli("reap", "--execute")
        self.assertEqual(r.returncode, 3)
        self.assertIn("not enabled", r.stdout + r.stderr)
        self.assertTrue(self.alive(k))

    def test_execute_kills_expired_session_when_enabled(self):
        k = self.job()
        self.lease(k, time.time() - 5)
        open(self.enable, "w").close()
        r = self.run_cli("reap", "--execute", "--grace", "1")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        k.wait(timeout=10)
        self.assertFalse(self.alive(k))

    def test_starttime_mismatch_refuses(self):
        k = self.job()
        self.lease(k, time.time() - 5, starttime=1)
        open(self.enable, "w").close()
        r = self.run_cli("reap", "--execute", "--grace", "1")
        self.assertIn("starttime", r.stdout)
        self.assertTrue(self.alive(k))

    def test_sid_mismatch_refuses(self):
        k = self.job()
        self.lease(k, time.time() - 5, sid=os.getsid(0))
        open(self.enable, "w").close()
        r = self.run_cli("reap", "--execute", "--grace", "1")
        self.assertIn("session", r.stdout)
        self.assertTrue(self.alive(k))

    def test_refuses_own_session_even_if_lease_says_so(self):
        # never signal the caller's own session
        k = self.job()
        self.lease(k, time.time() - 5, sid=os.getsid(0))
        open(self.enable, "w").close()
        r = self.run_cli("reap", "--execute")
        self.assertTrue(self.alive(k))

    def test_dead_pid_reports_stale_lease(self):
        k = self.job()
        self.lease(k, time.time() - 5)
        os.killpg(k.pid, signal.SIGKILL)
        k.wait()
        r = self.run_cli("reap")
        self.assertIn("STALE", r.stdout)

    def test_no_lease_files(self):
        r = self.run_cli("reap")
        self.assertEqual(r.returncode, 0)
        self.assertIn("no lease", r.stdout)

    def test_gpu_lease_also_scanned(self):
        k = self.job()
        self.lease(k, time.time() - 5, lock="gpu")
        r = self.run_cli("reap")
        self.assertIn("gpu", r.stdout)
        self.assertIn("EXPIRED", r.stdout)

    def test_json_output(self):
        k = self.job()
        self.lease(k, time.time() - 5)
        r = self.run_cli("reap", "--json")
        d = json.loads(r.stdout)
        self.assertEqual(d[0]["state"], "expired")
        self.assertFalse(d[0]["killed"])


if __name__ == "__main__":
    unittest.main()


class ScriptsTest(unittest.TestCase):
    ROOT = os.path.abspath(os.path.join(os.path.dirname(__file__), "..", "..", "..", ".."))

    def test_hostcommit_refuses_attribution_and_commits_paths(self):
        import tempfile
        d = tempfile.mkdtemp()
        sh = lambda *a: subprocess.run(a, cwd=d, capture_output=True, text=True)
        sh("git", "init", "-q"); sh("git", "config", "user.email", "a@b"); sh("git", "config", "user.name", "t")
        open(os.path.join(d, "f"), "w").write("x")
        hc = os.path.join(self.ROOT, "scripts", "hostcommit")
        bad = sh(hc, d, "Co-Authored-By: x", "f")
        self.assertEqual(bad.returncode, 3)
        ok = sh(hc, d, "add f", "f")
        self.assertEqual(ok.returncode, 0, ok.stderr)
        self.assertIn("add f", sh("git", "log", "--oneline").stdout)

    def test_blocked_notify_due_after_threshold_then_repeat_gate(self):
        import tempfile
        f = os.path.join(tempfile.mkdtemp(), "b.json")
        env = dict(os.environ, DW_BLOCKED_FILE=f, DW_BLOCKED_NO_REAP="1")
        bn = os.path.join(self.ROOT, "scripts", "blocked-notify")
        run = lambda *a: subprocess.run([bn, *a], env=env, capture_output=True, text=True)
        run("add", "k", "owner decision")
        self.assertEqual(run("due").stdout, "")
        self.assertIn("owner decision", run("due", "--after-min", "0").stdout)
        self.assertEqual(run("due", "--after-min", "0").stdout, "")
        run("clear", "k")
        self.assertEqual(run("due", "--after-min", "0", "--repeat-min", "0").stdout, "")
