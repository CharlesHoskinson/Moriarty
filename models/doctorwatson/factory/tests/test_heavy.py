import fcntl
import os
import time

from .helpers import FakeRepo, pred, yaml_of


class PreflightTests(FakeRepo):
    # A fitness test must not depend on the host's live memory (another heavy run can hold
    # tens of GiB); the memory rule itself is covered by the MemoryGuard admission tests.
    HOST_MEM_NEUTRAL = {"DW_DEFAULT_RSS_CAP": "1", "DW_MEM_RESERVE_GIB": "0", "DW_MAX_SWAP_GIB": "100000"}

    def test_ok(self):
        r = self.run_cli("preflight", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000",
                                              **self.HOST_MEM_NEUTRAL})
        self.assertEqual(r.returncode, 0, r.stdout)
        self.assertIn("disk free", r.stdout)
        self.assertIn("load 1m", r.stdout)
        self.assertIn("PREFLIGHT OK", r.stdout)

    def test_json(self):
        import json
        r = self.run_cli("preflight", "--json", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000",
                                              **self.HOST_MEM_NEUTRAL})
        d = json.loads(r.stdout)
        self.assertTrue(d["ok"])
        self.assertIn("diskFreeGiB", d)
        self.assertIn("load1", d)

    def test_unfit_disk(self):
        r = self.run_cli("preflight", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "99999999"})
        self.assertEqual(r.returncode, 3)
        self.assertIn("UNFIT", r.stdout)

    def test_unfit_load(self):
        r = self.run_cli("preflight", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "0"})
        self.assertEqual(r.returncode, 3)
        self.assertIn("load", r.stdout)

    def test_unfit_lock_held(self):
        fd = os.open(self.lock, os.O_RDWR | os.O_CREAT)
        self.addCleanup(os.close, fd)
        fcntl.flock(fd, fcntl.LOCK_EX)
        r = self.run_cli("preflight", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000",
                                              **self.HOST_MEM_NEUTRAL})
        self.assertEqual(r.returncode, 3)
        self.assertIn("HELD", r.stdout)


class HeavyTests(FakeRepo):
    def heavy(self, *args, cmd=("true",), env=None, timeout=60):
        return self.run_cli("heavy", *args, "--", *cmd, env=env, timeout=timeout)

    def test_pipe_stdin_is_explicit_and_never_teed(self):
        cmd = ("python3", "-c", "import sys; print(len(sys.stdin.read()))")
        payload = "private-opening-test-sentinel"
        r = self.run_cli("heavy", "--id", "P0", "--", *cmd, input=payload)
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(r.stdout.strip(), "0")
        r = self.run_cli("heavy", "--id", "P1", "--pipe-stdin", "--", *cmd, input=payload)
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(r.stdout.strip(), str(len(payload)))
        for run in self.runs():
            with open(self.p(run["trace"])) as stream:
                self.assertNotIn(payload, stream.read())
            self.assertNotIn(payload, str(run))
        self.assertNotIn(payload, r.stdout + r.stderr)

    def test_pipe_stdin_refuses_regular_files_before_running(self):
        import subprocess
        import sys
        from .helpers import PKG_PARENT
        self.write("input", "test-input")
        with open(self.p("input")) as stream:
            r = subprocess.run([sys.executable, "-m", "factory", "heavy", "--id", "P1",
                                "--pipe-stdin", "--", "true"], cwd=PKG_PARENT, env=self.env,
                               stdin=stream, capture_output=True, text=True)
        self.assertEqual(r.returncode, 2, r.stderr)
        self.assertIn("pipe or socket", r.stderr)
        self.assertEqual(self.runs(), [])

    def test_node_socket_stdin_reaches_only_the_proving_child(self):
        import subprocess
        import sys
        from .helpers import PKG_PARENT
        self.write("forward.mjs", "import {spawn} from 'node:child_process';\n"
                   "const p=spawn(process.argv[2],['-m','factory','heavy','--id','N1','--pipe-stdin','--',"
                   "'python3','-c','import sys; print(len(sys.stdin.read()))'],{stdio:['pipe','inherit','inherit']});\n"
                   "process.stdin.pipe(p.stdin);p.on('close',code=>{process.exitCode=code;});\n")
        payload = "node-pipe-opening-test"
        r = subprocess.run(["node", self.p("forward.mjs"), sys.executable], cwd=PKG_PARENT,
                           env=self.env, input=payload, capture_output=True, text=True)
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(r.stdout.strip(), str(len(payload)))
        with open(self.p(self.runs()[0]["trace"])) as stream:
            self.assertNotIn(payload, stream.read())

    def test_passthrough_exit_and_tee(self):
        r = self.heavy("--id", "X1", cmd=("bash", "-c", "echo out; echo err >&2; exit 7"))
        self.assertEqual(r.returncode, 7)
        self.assertIn("out", r.stdout)
        run = self.runs()[0]
        self.assertEqual(run["exit"], 7)
        self.assertEqual(run["id"], "X1")
        self.assertGreater(run["maxRssBytes"], 0)
        with open(self.p(run["trace"])) as fh:
            text = fh.read()
        self.assertIn("out", text)
        self.assertIn("err", text)
        self.assertTrue(run["run"].endswith("-X1"))

    def test_signal_exit_code(self):
        r = self.heavy("--id", "X1", cmd=("bash", "-c", "kill -9 $$"))
        self.assertEqual(r.returncode, 137)

    def test_needs_command(self):
        r = self.run_cli("heavy", "--id", "X1")
        self.assertEqual(r.returncode, 2)

    def test_unfit_exit_3_nothing_logged(self):
        r = self.heavy("--id", "X1", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "99999999"})
        self.assertEqual(r.returncode, 3)
        self.assertEqual(self.runs(), [])

    def test_lock_held_exit_3(self):
        fd = os.open(self.lock, os.O_RDWR | os.O_CREAT)
        self.addCleanup(os.close, fd)
        fcntl.flock(fd, fcntl.LOCK_EX)
        r = self.heavy("--id", "X1")
        self.assertEqual(r.returncode, 3)
        self.assertIn("lock", r.stderr)

    def test_lock_released_after_run(self):
        self.assertEqual(self.heavy("--id", "X1").returncode, 0)
        fd = os.open(self.lock, os.O_RDWR)
        self.addCleanup(os.close, fd)
        fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)  # would raise if still held

    def test_retry_rule(self):
        cmd = ("bash", "-c", "exit 1")
        self.assertEqual(self.heavy("--id", "R1", cmd=cmd).returncode, 1)
        r = self.heavy("--id", "R1", cmd=cmd)
        self.assertEqual(r.returncode, 5)
        self.assertIn("hypothesis", r.stderr)
        self.assertEqual(len(self.runs()), 1)  # the refused retry is not a run
        r = self.heavy("--id", "R1", "--hypothesis", "off by one", cmd=cmd)
        self.assertEqual(r.returncode, 1)
        runs = self.runs()
        self.assertEqual(len(runs), 2)
        self.assertEqual(runs[1]["hypothesis"], "off by one")
        # never consumed: still runnable with a hypothesis, any number of times
        self.assertEqual(self.heavy("--id", "R1", "--hypothesis", "again", cmd=cmd).returncode, 1)
        self.assertEqual(len(self.runs()), 3)

    def test_retry_rule_not_triggered_by_different_cmd_or_success(self):
        self.assertEqual(self.heavy("--id", "R1", cmd=("bash", "-c", "exit 1")).returncode, 1)
        self.assertEqual(self.heavy("--id", "R1", cmd=("bash", "-c", "exit 3")).returncode, 3)
        self.assertEqual(self.heavy("--id", "R1", cmd=("true",)).returncode, 0)
        self.assertEqual(self.heavy("--id", "R1", cmd=("true",)).returncode, 0)
        self.assertEqual(self.heavy("--id", "R2", cmd=("bash", "-c", "exit 1")).returncode, 1)

    def test_retry_rule_new_head(self):
        import subprocess
        git = ["git", "-C", self.repo, "-c", "user.name=t", "-c", "user.email=t@t", "-c", "commit.gpgsign=false"]
        subprocess.run(git + ["init", "-q"], check=True)
        self.write("f", "1")
        subprocess.run(git + ["add", "f"], check=True)
        subprocess.run(git + ["commit", "-q", "-m", "one"], check=True)
        cmd = ("bash", "-c", "exit 1")
        self.assertEqual(self.heavy("--id", "R1", cmd=cmd).returncode, 1)
        self.assertEqual(self.heavy("--id", "R1", cmd=cmd).returncode, 5)
        self.write("f", "2")
        subprocess.run(git + ["commit", "-q", "-am", "two"], check=True)
        self.assertEqual(self.heavy("--id", "R1", cmd=cmd).returncode, 1)  # new head: no hypothesis needed
        self.assertNotEqual(self.runs()[0]["head"], self.runs()[1]["head"])

    def test_measure_then_cap(self):
        self.seed_budget("k1")
        r = self.heavy("--id", "M1", "--budget", "k1", cmd=("bash", "-c", "sleep 0.4"))
        self.assertEqual(r.returncode, 0, r.stderr)
        b = self.budgets()["budgets"]["k1"]
        self.assertGreaterEqual(b["measured"]["wallSeconds"], 0.4)
        self.assertGreater(b["measured"]["maxRssBytes"], 0)
        self.assertEqual(b["measured"]["k"], 20)  # seeded k kept
        self.assertTrue(b["measured"]["run"].endswith("-M1"))
        self.assertEqual(b["cap"]["wallSeconds"], -(-2 * b["measured"]["wallSeconds"] // 1))
        self.assertEqual(b["cap"]["maxRssBytes"], 2 * b["measured"]["maxRssBytes"])

    def test_unseeded_budget_key_created(self):
        self.assertEqual(self.heavy("--id", "M1", "--budget", "new", cmd=("true",)).returncode, 0)
        self.assertIn("new", self.budgets()["budgets"])

    def test_failed_run_does_not_measure(self):
        self.seed_budget("k1")
        self.heavy("--id", "M1", "--budget", "k1", cmd=("bash", "-c", "exit 1"))
        self.assertIsNone(self.budgets()["budgets"]["k1"]["measured"]["wallSeconds"])

    def test_budget_from_predicate(self):
        self.predicates(yaml_of(pred("A3", heavy=True, budget="a3")))
        self.seed_budget("a3")
        self.assertEqual(self.heavy("--id", "A3").returncode, 0)
        self.assertIsNotNone(self.budgets()["budgets"]["a3"]["measured"]["wallSeconds"])

    def test_wall_cap_exceeded_exit_4(self):
        self.seed_budget("k1", measured_wall=0.1, cap_wall=0.4, cap_rss=10 ** 12, rss=1000)
        t0 = time.monotonic()
        r = self.heavy("--id", "W1", "--budget", "k1", cmd=("bash", "-c", "sleep 30"))
        self.assertEqual(r.returncode, 4, r.stderr)
        self.assertLess(time.monotonic() - t0, 10)
        run = self.runs()[0]
        self.assertEqual(run["exit"], 4)
        self.assertEqual(run["capExceeded"], "wall")
        self.assertIn("BUDGET EXCEEDED", r.stderr)

    def test_wall_cap_kills_process_group(self):
        self.seed_budget("k1", measured_wall=0.1, cap_wall=0.4, cap_rss=10 ** 12, rss=1000)
        marker = self.p("late")
        r = self.heavy("--id", "W1", "--budget", "k1",
                       cmd=("bash", "-c", f"(sleep 1.5; touch {marker}) & wait"))
        self.assertEqual(r.returncode, 4)
        time.sleep(2)
        self.assertFalse(os.path.exists(marker))

    def test_rss_cap_exceeded_exit_4(self):
        self.seed_budget("k1", measured_wall=100, cap_wall=1000, cap_rss=30 * 1024 * 1024, rss=1000)
        code = "import time; b = bytearray(150*1024*1024); time.sleep(0.6)"
        r = self.heavy("--id", "S1", "--budget", "k1", cmd=("python3", "-c", code))
        self.assertEqual(r.returncode, 4, r.stderr)
        self.assertEqual(self.runs()[0]["capExceeded"], "rss")

    def test_within_cap_passes(self):
        self.seed_budget("k1", measured_wall=1, cap_wall=60, cap_rss=10 ** 12, rss=1000)
        r = self.heavy("--id", "OK", "--budget", "k1", cmd=("true",))
        self.assertEqual(r.returncode, 0)
        # cap is left alone once measured
        self.assertEqual(self.budgets()["budgets"]["k1"]["cap"]["wallSeconds"], 60)

    def test_status_updated_for_known_predicate(self):
        self.predicates(yaml_of(pred("A1")))
        self.heavy("--id", "A1", cmd=("true",))
        st = self.status()
        self.assertTrue(st["predicates"]["A1"]["passes"])
        self.assertEqual(st["lastFlip"]["id"], "A1")
        self.assertEqual(st["iterationsSinceFlip"], 0)

    def test_limits_for(self):
        import sys
        from .helpers import PKG_PARENT
        self.seed_budget("k1")
        sys.path.insert(0, PKG_PARENT)
        self.addCleanup(sys.path.remove, PKG_PARENT)
        os.environ["DW_REPO"] = self.repo
        self.addCleanup(os.environ.pop, "DW_REPO")
        from factory import budgets
        self.assertIsNone(budgets.limits_for("k1"))
        self.assertIsNone(budgets.limits_for("missing"))
        self.seed_budget("k1", measured_wall=2, cap_wall=4, cap_rss=99, rss=50)
        self.assertEqual(budgets.limits_for("k1"), {"wallSeconds": 4, "maxRssBytes": 99})


class MemoryGuardTests(FakeRepo):
    """SRC-016 / task 2.0: every heavy run has a kernel memory cap <= 48 GiB; preflight checks memory."""

    GIB = 1024 ** 3
    ALLOC = ("python3", "-c", "b = bytearray(400 * 1024 * 1024); b[::4096] = b'x' * len(b[::4096]); print('survived')")

    def heavy(self, *cmd, env=None):
        return self.run_cli("heavy", "--id", "M0", "--", *cmd, env=env, timeout=120)

    def guard_available(self):
        import subprocess as sp
        if os.environ.get("DW_MEM_GUARD") == "poll":
            return False
        try:
            return sp.run(["systemd-run", "--user", "--scope", "--collect", "-q", "-p", "MemoryMax=64M",
                           "--", "true"], capture_output=True, timeout=15).returncode == 0
        except (OSError, sp.SubprocessError):
            return False

    def test_default_cap_applied_without_budget(self):
        r = self.heavy("true")
        self.assertEqual(r.returncode, 0, r.stderr)
        run = self.runs()[-1]
        self.assertEqual(run["memCapBytes"], 48 * self.GIB)
        self.assertIn(run["memGuard"], ("cgroup", "poll"))

    def test_cap_never_above_48_gib(self):
        r = self.heavy("true", env={"DW_DEFAULT_RSS_CAP": "200"})
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(self.runs()[-1]["memCapBytes"], 48 * self.GIB)

    def test_budget_cap_clamped(self):
        self.seed_budget("B", measured_wall=1, cap_wall=600, cap_rss=100 * self.GIB, rss=1)
        r = self.run_cli("heavy", "--id", "M1", "--budget", "B", "--", "true")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(self.runs()[-1]["memCapBytes"], 48 * self.GIB)

    def test_cgroup_scope_kills_job_over_cap(self):
        if not self.guard_available():
            self.skipTest("systemd-run --user --scope unavailable")
        r = self.heavy(*self.ALLOC, env={"DW_DEFAULT_RSS_CAP": "0.0625"})
        self.assertEqual(r.returncode, 4, r.stdout + r.stderr)
        self.assertNotIn("survived", r.stdout)
        run = self.runs()[-1]
        self.assertEqual(run["memGuard"], "cgroup")
        self.assertEqual(run["capExceeded"], "rss")
        self.assertTrue(run["memScope"].startswith("dw-") and run["memScope"].endswith(".scope"))

    def test_job_runs_inside_scope(self):
        if not self.guard_available():
            self.skipTest("systemd-run --user --scope unavailable")
        r = self.heavy("cat", "/proc/self/cgroup")
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn(self.runs()[-1]["memScope"], r.stdout)

    def test_poll_fallback_kills_job_over_cap(self):
        r = self.heavy(*self.ALLOC, env={"DW_DEFAULT_RSS_CAP": "0.0625", "DW_MEM_GUARD": "poll"})
        self.assertEqual(r.returncode, 4, r.stdout + r.stderr)
        run = self.runs()[-1]
        self.assertEqual(run["memGuard"], "poll")
        self.assertEqual(run["capExceeded"], "rss")

    def test_preflight_refuses_low_mem_available(self):
        env = {"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000",
               "DW_MEM_RESERVE_GIB": "100000"}
        r = self.run_cli("preflight", env=env)
        self.assertEqual(r.returncode, 3, r.stdout)
        self.assertIn("MemAvailable", r.stdout)
        r = self.heavy("true", env=env)
        self.assertEqual(r.returncode, 3, r.stderr)
        self.assertEqual(self.runs(), [])

    def test_preflight_refuses_swap_in_use(self):
        env = {"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000",
               "DW_MEM_RESERVE_GIB": "0", "DW_DEFAULT_RSS_CAP": "0.001", "DW_MAX_SWAP_GIB": "-1"}
        r = self.run_cli("preflight", env=env)
        self.assertEqual(r.returncode, 3, r.stdout)
        self.assertIn("swap", r.stdout)

    def test_cargo_test_gets_rust_test_threads(self):
        bindir = os.path.join(self._tmp.name, "bin")
        os.makedirs(bindir)
        fake = os.path.join(bindir, "cargo")
        with open(fake, "w") as fh:
            fh.write("#!/bin/sh\necho threads=${RUST_TEST_THREADS:-unset}\n")
        os.chmod(fake, 0o755)
        env = {"RUST_TEST_THREADS": ""}
        r = self.heavy(fake, "test", "-p", "x", env=env)
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIn("threads=4", r.stdout)
        r = self.heavy(fake, "test", env={"RUST_TEST_THREADS": "2"})
        self.assertIn("threads=2", r.stdout)
        r = self.heavy(fake, "build", env=env)
        self.assertIn("threads=unset", r.stdout)


class ConcurrentHeavyTests(FakeRepo):
    """E11: heavy --concurrent P under one lock, per-process caps and a <= 100 GiB aggregate cap."""

    def read_text(self, rel):
        with open(self.p(rel)) as fh:
            return fh.read()

    GIB = 1024 ** 3
    # The hold must outlast the slower child's start-up: under host load the second child can
    # allocate more than 4 s after the first, which then printed 'survived' before the aggregate
    # crossed the cap (seen three times on 2026-11-08). The guard kills both as soon as the sum
    # exceeds the cap, so a long hold does not slow a passing run.
    HOLD = ("python3", "-c", "import time; b = bytearray(100 * 1024 * 1024); b[::4096] = b'x' * len(b[::4096]); "
            "time.sleep(60); print('survived')")

    def heavy(self, *args, cmd=None, env=None):
        tail = ("--", *cmd) if cmd else ()
        return self.run_cli("heavy", *args, *tail, env=env, timeout=120)

    def test_copies_run_at_once_with_slots(self):
        r = self.heavy("--id", "C1", "--concurrent", "2", cmd=("sh", "-c", "echo slot=$DW_SLOT of $DW_SLOTS; sleep 1"))
        self.assertEqual(r.returncode, 0, r.stderr)
        run = self.runs()[-1]
        self.assertEqual(run["concurrent"], 2)
        self.assertLess(run["wallSeconds"], 1.9)  # makespan, not the sum
        self.assertEqual(run["aggregateCapBytes"], 100 * self.GIB)
        outs = sorted(self.read_text(c["trace"]).strip() for c in run["children"])
        self.assertEqual(outs, ["slot=0 of 2", "slot=1 of 2"])
        self.assertIn("makespan=", self.read_text(run["trace"]))

    def test_admission_refuses_sum_of_caps_over_aggregate(self):
        r = self.heavy("--id", "C2", "--concurrent", "3", cmd=("true",))  # 3 x 48 GiB > 100 GiB
        self.assertEqual(r.returncode, 3, r.stderr)
        self.assertIn("aggregate cap", r.stderr)
        self.assertEqual(self.runs(), [])
        r = self.heavy("--id", "C2", "--concurrent", "3", cmd=("true",), env={"DW_DEFAULT_RSS_CAP": "33"})
        self.assertEqual(r.returncode, 0, r.stderr)

    def test_aggregate_cap_clamped_to_100_gib(self):
        r = self.heavy("--id", "C3", "--concurrent", "2", "--aggregate-cap", "500G", cmd=("true",))
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertEqual(self.runs()[-1]["aggregateCapBytes"], 100 * self.GIB)

    def _aggregate_kill(self, mode):
        """Second layer: the slice / summed-RSS guard, past admission (which already keeps P x cap <= aggregate)."""
        import sys
        sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__)))))
        from factory import heavy
        trace = self.p("qa/sherlock/traces/agg.log")
        os.makedirs(os.path.dirname(trace), exist_ok=True)
        marker = "agg%d%d" % (os.getpid(), time.time_ns() % 100000)
        res = heavy.run_concurrent([list(self.HOLD), list(self.HOLD)], self.repo, trace, False,
                                   {"maxRssBytes": 128 * 1024 ** 2}, 150 * 1024 ** 2, marker, mode)
        self.assertEqual(res["exceeded"], "rss", res)
        self.assertTrue(res["aggregateExceeded"], res)
        self.assertNotEqual(res["exit"], 0)
        for c in res["children"]:
            self.assertNotIn("survived", self.read_text(c["trace"]))
        return res

    def test_aggregate_kill_poll(self):
        res = self._aggregate_kill("poll")
        self.assertIsNone(res["memSlice"])

    def test_aggregate_kill_cgroup_slice(self):
        if not MemoryGuardTests.guard_available(self):
            self.skipTest("systemd-run --user --scope unavailable")
        res = self._aggregate_kill("cgroup")
        self.assertTrue(res["memSlice"].startswith("heavy-dw_") and res["memSlice"].endswith(".slice"))

    def test_commands_file(self):
        self.write("cmds.txt", "# two different jobs\necho one\n\necho two\n")
        r = self.heavy("--id", "C5", "--commands", self.p("cmds.txt"))
        self.assertEqual(r.returncode, 0, r.stderr)
        run = self.runs()[-1]
        self.assertEqual([c["cmd"] for c in run["children"]], ["echo one", "echo two"])
        r = self.heavy("--id", "C5", "--commands", self.p("cmds.txt"), "--concurrent", "3")
        self.assertEqual(r.returncode, 2)

    def test_failing_child_fails_the_run(self):
        r = self.heavy("--id", "C6", "--concurrent", "2", cmd=("sh", "-c", 'test "$DW_SLOT" = 0'))
        self.assertEqual(r.returncode, 1, r.stderr)
        exits = sorted(c["exit"] for c in self.runs()[-1]["children"])
        self.assertEqual(exits, [0, 1])


class FixedBudgetTests(FakeRepo):
    """E17: `fixed: true` caps are enforced from the first run and never rewritten."""

    GIB = 1024 ** 3

    def seed_fixed(self, key, wall, rss, agg=None):
        import json
        cap = {"wallSeconds": wall, "maxRssBytes": rss}
        if agg:
            cap["aggregateRssBytes"] = agg
        with open(self.p("qa/sherlock/budgets.json"), "w") as fh:
            json.dump({"schema": "doctorwatson.budgets/1", "budgets": {key: {
                "measured": {"wallSeconds": None, "maxRssBytes": None, "k": None, "at": None, "run": None},
                "cap": cap, "fixed": True}}}, fh)

    def test_fixed_cap_enforced_on_first_run(self):
        self.seed_fixed("F", 1, 8 * self.GIB)
        r = self.run_cli("heavy", "--id", "F1", "--budget", "F", "--", "sleep", "5")
        self.assertEqual(r.returncode, 4, r.stderr)
        run = self.runs()[-1]
        self.assertEqual(run["capExceeded"], "wall")
        self.assertEqual(run["memCapBytes"], 8 * self.GIB)

    def test_fixed_cap_not_rewritten_but_measured(self):
        self.seed_fixed("F", 600, 8 * self.GIB)
        r = self.run_cli("heavy", "--id", "F2", "--budget", "F", "--", "true")
        self.assertEqual(r.returncode, 0, r.stderr)
        b = self.budgets()["budgets"]["F"]
        self.assertEqual(b["cap"], {"wallSeconds": 600, "maxRssBytes": 8 * self.GIB})
        self.assertIsNotNone(b["measured"]["wallSeconds"])
        self.assertTrue(b["fixed"])

    def test_fixed_aggregate_cap_and_concurrency_measured(self):
        self.seed_fixed("F", 600, 30 * self.GIB, agg=60 * self.GIB)
        r = self.run_cli("heavy", "--id", "F3", "--budget", "F", "--concurrent", "2", "--", "true")
        self.assertEqual(r.returncode, 0, r.stderr)
        run = self.runs()[-1]
        self.assertEqual(run["aggregateCapBytes"], 60 * self.GIB)
        self.assertEqual(run["concurrency"], 2)
        self.assertIn("aggregateRssBytes", run)
        self.assertEqual(self.budgets()["budgets"]["F"]["measured"]["concurrency"], 2)
        r = self.run_cli("heavy", "--id", "F4", "--budget", "F", "--concurrent", "3", "--", "true")
        self.assertEqual(r.returncode, 3, r.stderr)  # 3 x 30 GiB > 60 GiB

    def test_receipt_has_cgroup_flag(self):
        r = self.run_cli("heavy", "--id", "F5", "--", "true", env={"DW_MEM_GUARD": "poll"})
        self.assertEqual(r.returncode, 0, r.stderr)
        self.assertIs(self.runs()[-1]["cgroup"], False)
