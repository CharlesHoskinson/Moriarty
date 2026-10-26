import fcntl
import json
import os
import tempfile
import threading
import time
import unittest

from factory import preflight, queue

from .helpers import FakeRepo


class QueueCase(FakeRepo):
    def setUp(self):
        super().setUp()
        self.qdir = os.path.join(self._tmp.name, "q")
        self.env.update(DW_QUEUE_DIR=self.qdir, DW_ROLE="coordinator", DW_AGENT="tester")
        self._saved = {k: os.environ.get(k) for k in ("DW_QUEUE_DIR", "DW_LOCK", "DW_ROLE", "DW_AGENT")}
        os.environ.update(DW_QUEUE_DIR=self.qdir, DW_LOCK=self.lock, DW_ROLE="coordinator", DW_AGENT="tester")
        self.addCleanup(self._restore)

    def _restore(self):
        for k, v in self._saved.items():
            if v is None:
                os.environ.pop(k, None)
            else:
                os.environ[k] = v

    def q(self, *args, **kw):
        return self.run_cli("q", *args, **kw)

    def submit(self, ident, *extra, wall="60"):
        r = self.q("submit", "--id", ident, "--max-wall", wall, *extra)
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        return r.stdout.strip().split()[0]

    def order(self):
        r = self.q("status", "--json")
        self.assertEqual(r.returncode, 0, r.stderr)
        return [t["label"] for t in json.loads(r.stdout)["queue"]]


class CliTests(QueueCase):
    def test_admission_requires_max_wall(self):
        r = self.q("submit", "--id", "A")
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("max-wall", r.stderr + r.stdout)

    def test_fifo_and_status(self):
        self.submit("A")
        self.submit("B")
        self.submit("C")
        self.assertEqual(self.order(), ["A", "B", "C"])
        self.assertIn("A", self.q("status").stdout)

    def test_priority_is_coordinator_only(self):
        r = self.q("submit", "--id", "A", "--max-wall", "5", "--prio", "5", env={"DW_ROLE": "agent"})
        self.assertNotEqual(r.returncode, 0)
        self.assertIn("coordinator", r.stderr + r.stdout)
        self.submit("A")
        self.submit("B", "--prio", "5")
        self.assertEqual(self.order(), ["B", "A"])

    def test_reorder_hold_release_cancel(self):
        a = self.submit("A")
        b = self.submit("B")
        self.submit("C")
        self.assertEqual(self.q("reorder", "C", "--first").returncode, 0)
        self.assertEqual(self.order()[0], "C")
        self.assertEqual(self.q("hold", b).returncode, 0)
        st = json.loads(self.q("status", "--json").stdout)
        self.assertTrue([t for t in st["queue"] if t["label"] == "B"][0]["held"])
        self.assertEqual(self.q("release", b).returncode, 0)
        self.assertEqual(self.q("cancel", a).returncode, 0)
        self.assertNotIn("A", self.order())
        r = self.q("reorder", "B", "--first", env={"DW_ROLE": "agent"})
        self.assertNotEqual(r.returncode, 0)

    def test_agent_may_cancel_only_own(self):
        r = self.q("submit", "--id", "A", "--max-wall", "5", env={"DW_ROLE": "agent", "DW_AGENT": "x"})
        tid = r.stdout.split()[0]
        self.assertNotEqual(self.q("cancel", tid, env={"DW_ROLE": "agent", "DW_AGENT": "y"}).returncode, 0)
        self.assertEqual(self.q("cancel", tid, env={"DW_ROLE": "agent", "DW_AGENT": "x"}).returncode, 0)

    def test_after_blocks_until_dep_done(self):
        a = self.submit("A")
        self.submit("B", "--after", a)
        st = json.loads(self.q("status", "--json").stdout)
        self.assertEqual(st["head"], a)
        self.q("cancel", a)
        st = json.loads(self.q("status", "--json").stdout)
        b = [x for x in st["queue"] if x["label"] == "B"][0]
        self.assertEqual(b["blocked"], "dependency %s cancelled" % a)

    def test_wait_returns_when_head_and_times_out(self):
        a = self.submit("A")
        b = self.submit("B")
        r = self.q("wait", b, "--timeout", "1")
        self.assertEqual(r.returncode, 124)
        r = self.q("wait", a, "--timeout", "5")
        self.assertEqual(r.returncode, 0)


class AcquireTests(QueueCase):
    def fd(self):
        return preflight.open_lock()

    def test_acquire_when_free_and_finish(self):
        fd = self.fd()
        ok, why = queue.acquire_turn(fd, "J1", "echo", max_wall=30, poll=0.05)
        self.assertTrue(ok, why)
        t = queue.load()["tickets"][0]
        self.assertEqual(t["state"], "running")
        self.assertEqual(t["pid"], os.getpid())
        self.assertEqual(t["deadline"] - t["started"], 30)
        queue.finish("J1", 0)
        self.assertEqual(queue.load()["tickets"][0]["state"], "done")
        fcntl.flock(fd, fcntl.LOCK_UN)
        os.close(fd)

    def test_fifo_admission_order_and_no_exit3(self):
        holder = self.fd()
        fcntl.flock(holder, fcntl.LOCK_EX)
        got = []

        def job(name, delay):
            time.sleep(delay)
            fd = self.fd()
            ok, why = queue.acquire_turn(fd, name, "x", max_wall=30, poll=0.05, timeout=20)
            got.append((name, ok))
            time.sleep(0.1)
            queue.finish(name, 0)
            fcntl.flock(fd, fcntl.LOCK_UN)
            os.close(fd)

        ths = [threading.Thread(target=job, args=(n, d)) for n, d in (("A", 0), ("B", 0.3), ("C", 0.6))]
        for t in ths:
            t.start()
        time.sleep(1.2)
        self.assertEqual(got, [])
        fcntl.flock(holder, fcntl.LOCK_UN)
        for t in ths:
            t.join(15)
        self.assertEqual(got, [("A", True), ("B", True), ("C", True)])

    def test_timeout_returns_reason_and_cancels_ticket(self):
        holder = self.fd()
        fcntl.flock(holder, fcntl.LOCK_EX)
        fd = self.fd()
        ok, why = queue.acquire_turn(fd, "J", "x", max_wall=30, poll=0.05, timeout=0.3)
        self.assertFalse(ok)
        self.assertIn("timeout", why)
        self.assertEqual(queue.load()["tickets"][0]["state"], "cancelled")

    def test_preticketed_label_is_reused(self):
        tid = self.submit("J")
        fd = self.fd()
        ok, why = queue.acquire_turn(fd, "J", "x", max_wall=None, poll=0.05)
        self.assertTrue(ok, why)
        self.assertEqual(len(queue.load()["tickets"]), 1)
        self.assertEqual(queue.load()["tickets"][0]["id"], tid)

    def test_stale_running_ticket_is_reaped(self):
        tid = self.submit("Dead")
        with queue._state() as st:
            t = st["tickets"][0]
            t.update(state="running", pid=2 ** 22 + 12345, pidStart=1, started=time.time(), deadline=time.time() + 99)
        fd = self.fd()
        ok, why = queue.acquire_turn(fd, "Next", "x", max_wall=5, poll=0.05, timeout=5)
        self.assertTrue(ok, why)
        self.assertEqual(queue.load()["tickets"][0]["state"], "crashed")

    def test_hold_blocks_admission(self):
        self.submit("H")
        self.q("hold", "--all")
        fd = self.fd()
        ok, why = queue.acquire_turn(fd, "H", "x", max_wall=5, poll=0.05, timeout=0.3)
        self.assertFalse(ok)
        self.assertIn("timeout", why)

    def test_overrun_flagged_in_status(self):
        fd = self.fd()
        queue.acquire_turn(fd, "O", "x", max_wall=1, poll=0.05)
        with queue._state() as st:
            st["tickets"][0]["deadline"] = time.time() - 5
        st = json.loads(self.q("status", "--json").stdout)
        self.assertTrue(st["running"]["overrun"])

    def test_queue_off_falls_back_to_nonblocking(self):
        os.environ["DW_QUEUE"] = "off"
        self.addCleanup(os.environ.pop, "DW_QUEUE", None)
        holder = self.fd()
        fcntl.flock(holder, fcntl.LOCK_EX)
        fd = self.fd()
        ok, why = queue.acquire_turn(fd, "Z", "x", max_wall=5, poll=0.05)
        self.assertFalse(ok)
        self.assertIn("holds the heavy lock", why)


class LockAllTests(QueueCase):
    def test_all_or_nothing(self):
        a = os.open(os.path.join(self._tmp.name, "a.lock"), os.O_RDWR | os.O_CREAT)
        b = os.open(os.path.join(self._tmp.name, "b.lock"), os.O_RDWR | os.O_CREAT)
        b2 = os.open(os.path.join(self._tmp.name, "b.lock"), os.O_RDWR)
        fcntl.flock(b2, fcntl.LOCK_EX)
        self.assertFalse(queue.lock_all([a, b]))
        a2 = os.open(os.path.join(self._tmp.name, "a.lock"), os.O_RDWR)
        self.assertTrue(preflight.try_lock(a2))
