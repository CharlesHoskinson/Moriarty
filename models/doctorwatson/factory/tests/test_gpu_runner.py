import importlib.util
import os
import subprocess
import tempfile
import time
import unittest

HERE = os.path.dirname(os.path.abspath(__file__))
RUNNER = os.path.join(HERE, "..", "..", "..", "..", "scripts", "qa", "gpu-runner.py")


def load():
    spec = importlib.util.spec_from_file_location("gpu_runner", RUNNER)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def alive(pid):
    try:
        with open(f"/proc/{pid}/stat") as fh:
            return fh.read().rsplit(")", 1)[1].split()[0] != "Z"
    except OSError:
        return False


class RunJobTests(unittest.TestCase):
    def setUp(self):
        self.mod = load()
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)

    def run_job(self, code, timeout, grace=2):
        pidf = os.path.join(self.tmp.name, "gc.pid")
        with open(os.path.join(self.tmp.name, "out.log"), "w") as out:
            t0 = time.time()
            rc = self.mod.run_job(["python3", "-c", code.replace("PIDF", pidf)], self.tmp.name,
                                  dict(os.environ), out, timeout, marker="t%d" % time.time_ns(), grace=grace)
        gc = int(open(pidf).read()) if os.path.exists(pidf) else None
        return rc, gc, time.time() - t0

    def test_timeout_kills_nested_setsid_descendant(self):
        code = ("import subprocess,time;"
                "p=subprocess.Popen(['sleep','300'],start_new_session=True);"
                "open('PIDF','w').write(str(p.pid)); time.sleep(300)")
        rc, gc, took = self.run_job(code, 1)
        self.assertEqual(rc, 124)
        self.assertLess(took, 15)
        end = time.time() + 5
        while alive(gc) and time.time() < end:
            time.sleep(0.1)
        self.assertFalse(alive(gc))

    def test_sigterm_ignoring_job_is_sigkilled_after_grace(self):
        code = ("import signal,time;signal.signal(signal.SIGTERM,signal.SIG_IGN);"
                "open('PIDF','w').write('1');time.sleep(300)")
        rc, _, took = self.run_job(code, 1, grace=1)
        self.assertEqual(rc, 124)
        self.assertLess(took, 10)

    def test_clean_exit_reaps_leftover_daemon(self):
        code = ("import subprocess;"
                "p=subprocess.Popen(['sleep','300'],start_new_session=True);"
                "open('PIDF','w').write(str(p.pid))")
        rc, gc, _ = self.run_job(code, 30)
        self.assertEqual(rc, 0)
        end = time.time() + 5
        while alive(gc) and time.time() < end:
            time.sleep(0.1)
        self.assertFalse(alive(gc))

    def test_heavy_route_is_in_request_schema(self):
        self.assertIn("heavy", open(RUNNER).read())


if __name__ == "__main__":
    unittest.main()


class HeavyRouteTests(unittest.TestCase):
    def test_timeout_through_heavy_releases_both_locks(self):
        from .helpers import PKG_PARENT
        from .test_lease import locked
        mod = load()
        with tempfile.TemporaryDirectory() as tmp:
            repo = os.path.join(tmp, "repo")
            os.makedirs(os.path.join(repo, "qa", "sherlock"))
            lock, glock = os.path.join(tmp, "heavy.lock"), os.path.join(tmp, "gpu.lock")
            env = dict(os.environ, DW_REPO=repo, DW_LOCK=lock, DW_GPU_LOCK=glock, DW_SKIP_PREFLIGHT="1",
                       PYTHONPATH=PKG_PARENT)
            argv = [os.sys.executable, "-m", "factory", "heavy", "--id", "R1", "--gpu", "--",
                    "bash", "-c", "setsid sleep 300 & sleep 300"]
            with open(os.path.join(tmp, "o.log"), "w") as out:
                rc = mod.run_job(argv, PKG_PARENT, env, out, 3, marker="rt%d" % time.time_ns(), grace=10)
            self.assertEqual(rc, 124)
            self.assertFalse(locked(lock))
            self.assertFalse(locked(glock))
