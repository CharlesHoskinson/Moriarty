import fcntl
import os
import time

from .helpers import FakeRepo, pred, yaml_of


class PreflightTests(FakeRepo):
    def test_ok(self):
        r = self.run_cli("preflight", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000"})
        self.assertEqual(r.returncode, 0, r.stdout)
        self.assertIn("disk free", r.stdout)
        self.assertIn("load 1m", r.stdout)
        self.assertIn("PREFLIGHT OK", r.stdout)

    def test_json(self):
        import json
        r = self.run_cli("preflight", "--json", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000"})
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
        r = self.run_cli("preflight", env={"DW_SKIP_PREFLIGHT": "0", "DW_MIN_FREE_GIB": "0", "DW_MAX_LOAD": "100000"})
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
