import fcntl
import json
import os
import signal
import subprocess
import sys
import time

from .helpers import FakeRepo, PKG_PARENT


def alive(pid):
    try:
        os.kill(pid, 0)
    except ProcessLookupError:
        return False
    try:
        with open(f"/proc/{pid}/stat") as fh:
            return fh.read().rsplit(")", 1)[1].split()[0] != "Z"
    except OSError:
        return False


def locked(path):
    fd = os.open(path, os.O_RDWR | os.O_CREAT)
    try:
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return False
    except OSError:
        return True
    finally:
        os.close(fd)


class LeaseTests(FakeRepo):
    def heavy(self, *args, cmd=("true",), env=None, timeout=60):
        return self.run_cli("heavy", *args, "--", *cmd, env=env, timeout=timeout)

    def popen_heavy(self, *args, cmd, env=None, **kw):
        e = dict(self.env)
        e.update(env or {})
        return subprocess.Popen([sys.executable, "-m", "factory", "heavy", *args, "--", *cmd],
                                cwd=PKG_PARENT, env=e, stdout=subprocess.DEVNULL,
                                stderr=subprocess.DEVNULL, **kw)

    def wait_for(self, pred, secs=15):
        end = time.time() + secs
        while time.time() < end:
            if pred():
                return True
            time.sleep(0.05)
        return False

    def test_refuses_without_wall_budget(self):
        r = self.heavy("--id", "N1", env={"DW_MAX_WALL": ""})
        self.assertEqual(r.returncode, 6)
        self.assertIn("wall", r.stderr)
        self.assertEqual(self.runs(), [])

    def test_explicit_max_wall_flag_allows(self):
        r = self.heavy("--id", "N1", "--max-wall", "30", env={"DW_MAX_WALL": ""})
        self.assertEqual(r.returncode, 0, r.stderr)

    def test_default_wall_cap_applies_unmeasured_budget(self):
        r = self.heavy("--id", "C1", "--budget", "new", cmd=("sleep", "30"), env={"DW_MAX_WALL": "1"})
        self.assertEqual(r.returncode, 4)
        self.assertIn("BUDGET EXCEEDED", r.stderr)

    def test_deadline_caps_job(self):
        t0 = time.time()
        r = self.heavy("--id", "D1", "--deadline", str(time.time() + 1), cmd=("sleep", "30"))
        self.assertEqual(r.returncode, 4)
        self.assertLess(time.time() - t0, 10)

    def test_lease_written_and_removed(self):
        marker = self.p("lease.json")
        code = f"import shutil; shutil.copy('{self.lock}.lease', '{marker}')"
        r = self.heavy("--id", "L1", cmd=("python3", "-c", code))
        self.assertEqual(r.returncode, 0, r.stderr)
        d = json.load(open(marker))
        self.assertEqual(d["schema"], "doctorwatson.lease/1")
        self.assertEqual((d["lock"], d["id"], d["locks"]), ("heavy", "L1", ["heavy"]))
        for k in ("holderPid", "holderStart", "jobPid", "jobSid", "startedAt", "deadline", "run", "cmd"):
            self.assertIn(k, d)
        self.assertEqual(d["jobSid"], d["jobPid"])
        self.assertFalse(os.path.exists(self.lock + ".lease"))

    def test_gpu_holds_both_locks_in_process(self):
        glock = self.env["DW_GPU_LOCK"]
        code = (f"import os,fcntl,json;"
                f"print(json.dumps([os.path.exists('{self.lock}.lease'), os.path.exists('{glock}.lease')]))")
        p = self.popen_heavy("--id", "G1", "--gpu", cmd=("sleep", "30"))
        self.addCleanup(p.kill)
        self.assertTrue(self.wait_for(lambda: locked(self.lock) and locked(glock)))
        lease = json.load(open(glock + ".lease"))
        self.assertEqual(lease["locks"], ["heavy", "gpu"])
        self.assertEqual(lease["holderPid"], p.pid)
        p.send_signal(signal.SIGTERM)
        p.wait(20)
        self.assertFalse(locked(self.lock))
        self.assertFalse(locked(glock))
        self.assertFalse(os.path.exists(glock + ".lease"))

    def test_gpu_busy_is_unfit_and_releases_heavy(self):
        glock = self.env["DW_GPU_LOCK"]
        fd = os.open(glock, os.O_RDWR | os.O_CREAT)
        self.addCleanup(os.close, fd)
        fcntl.flock(fd, fcntl.LOCK_EX)
        r = self.heavy("--id", "G2", "--gpu")
        self.assertEqual(r.returncode, 3)
        self.assertIn("gpu", r.stderr)
        self.assertFalse(locked(self.lock))

    def test_sigterm_kills_session_escapees(self):
        pidf = self.p("gc.pid")
        code = (f"import os,subprocess;"
                f"p=subprocess.Popen(['sleep','300'],start_new_session=True);"
                f"open('{pidf}','w').write(str(p.pid)); import time; time.sleep(300)")
        p = self.popen_heavy("--id", "S1", cmd=("python3", "-c", code))
        self.addCleanup(p.kill)
        self.assertTrue(self.wait_for(lambda: os.path.exists(pidf) and open(pidf).read()))
        gc = int(open(pidf).read())
        self.assertTrue(alive(gc))
        p.send_signal(signal.SIGTERM)
        p.wait(20)
        self.assertTrue(self.wait_for(lambda: not alive(gc), 10), "setsid grandchild survived")
        self.assertFalse(locked(self.lock))

    def test_wall_cap_kills_setsid_grandchild(self):
        pidf = self.p("gc2.pid")
        code = (f"import subprocess,time;"
                f"p=subprocess.Popen(['sleep','300'],start_new_session=True);"
                f"open('{pidf}','w').write(str(p.pid)); time.sleep(300)")
        r = self.heavy("--id", "S2", "--max-wall", "1", cmd=("python3", "-c", code))
        self.assertEqual(r.returncode, 4)
        gc = int(open(pidf).read())
        self.assertTrue(self.wait_for(lambda: not alive(gc), 10))
        self.assertFalse(locked(self.lock))

    def test_parent_death_watchdog(self):
        pidf = self.p("hv.pid")
        gcf = self.p("hvjob.pid")
        shell = (f"{sys.executable} -m factory heavy --id P1 --max-wall 300 -- "
                 f"bash -c 'echo $$ > {gcf}; sleep 300' & echo $! > {pidf}; sleep 300")
        parent = subprocess.Popen(["bash", "-c", shell], cwd=PKG_PARENT,
                                  env=dict(self.env, DW_WATCH_PARENT="1"), start_new_session=True,
                                  stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        self.addCleanup(lambda: os.killpg(parent.pid, signal.SIGKILL) if alive(parent.pid) else None)
        self.assertTrue(self.wait_for(lambda: os.path.exists(gcf) and open(gcf).read().strip()))
        job = int(open(gcf).read())
        self.assertTrue(locked(self.lock))
        os.kill(parent.pid, signal.SIGKILL)  # only the parent shell, not the heavy process
        self.assertTrue(self.wait_for(lambda: not locked(self.lock), 15), "lock still held after parent death")
        self.assertTrue(self.wait_for(lambda: not alive(job), 10))

    def test_lease_state_api(self):
        sys.path.insert(0, PKG_PARENT)
        from factory import heavy
        os.environ["DW_LOCK"] = self.lock
        os.environ["DW_GPU_LOCK"] = self.env["DW_GPU_LOCK"]
        self.addCleanup(os.environ.pop, "DW_LOCK", None)
        self.addCleanup(os.environ.pop, "DW_GPU_LOCK", None)
        self.assertIsNone(heavy.read_lease("heavy"))
        st = heavy.lease_state("heavy")
        self.assertFalse(st["locked"])
        p = self.popen_heavy("--id", "A1", cmd=("sleep", "30"))
        self.addCleanup(p.kill)
        self.assertTrue(self.wait_for(lambda: heavy.read_lease("heavy")))
        st = heavy.lease_state("heavy")
        self.assertTrue(st["holderAlive"] and st["locked"] and not st["expired"])
        self.assertEqual(st["lease"]["id"], "A1")
        p.terminate()
        p.wait(20)
        self.assertIsNone(heavy.read_lease("heavy"))


class ReapCrossTests(FakeRepo):
    def test_reap_dry_run_lists_expired_heavy_lease(self):
        code = (f"import subprocess,sys;sys.path.insert(0,'{PKG_PARENT}');"
                "subprocess.run(['sleep','0.1']);")
        # a fake job that outlives a 2 s cap while the lease deadline is already in the past
        p = subprocess.Popen([sys.executable, "-m", "factory", "heavy", "--id", "RP1", "--max-wall", "2", "--",
                              "sleep", "30"], cwd=PKG_PARENT, env=self.env, stdout=subprocess.DEVNULL,
                             stderr=subprocess.DEVNULL)
        self.addCleanup(p.kill)
        lease = self.lock + ".lease"
        end = time.time() + 15
        while time.time() < end:
            try:
                d = json.load(open(lease))
                if d.get("sid"):
                    break
            except (OSError, ValueError):
                pass
            time.sleep(0.05)
        d["deadline"] = time.time() - 5  # simulate a job that overran its 2 s cap
        json.dump(d, open(lease, "w"))
        self.assertEqual(d["sid"], d["pid"])
        for k in ("v", "pid", "starttime", "sid", "deadline", "id"):
            self.assertIn(k, d)
        r = self.run_cli("reap", "--json")
        out = json.loads(r.stdout)
        recs = out if isinstance(out, list) else out.get("leases", out.get("results", []))
        mine = [x for x in recs if x.get("lock") == "heavy"]
        self.assertTrue(mine, r.stdout)
        self.assertEqual(mine[0]["state"], "expired")
        self.assertEqual(mine[0]["sid"], d["sid"])
        self.assertFalse(mine[0]["killed"])
        p.terminate()
        p.wait(20)


class QueueIntegrationTests(FakeRepo):
    def test_heavy_queues_behind_holder_instead_of_exit3(self):
        a = subprocess.Popen([sys.executable, "-m", "factory", "heavy", "--id", "QA", "--max-wall", "60", "--",
                              "sleep", "2"], cwd=PKG_PARENT, env=self.env, stdout=subprocess.DEVNULL,
                             stderr=subprocess.DEVNULL)
        self.addCleanup(a.kill)
        end = time.time() + 15
        while time.time() < end and not os.path.exists(self.lock + ".lease"):
            time.sleep(0.05)
        r = self.run_cli("heavy", "--id", "QB", "--max-wall", "60", "--", "true",
                         env={"DW_QUEUE_TIMEOUT": "30"})
        self.assertEqual(r.returncode, 0, r.stderr)
        a.wait(20)
