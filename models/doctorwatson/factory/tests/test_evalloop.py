import json
import subprocess

from .helpers import FakeRepo


def git(repo, *a):
    return subprocess.run(["git", "-C", repo, *a], capture_output=True, text=True, check=True).stdout


class EvalLoopBase(FakeRepo):
    def setUp(self):
        super().setUp()
        git(self.repo, "init", "-q")
        git(self.repo, "config", "user.email", "t@t")
        git(self.repo, "config", "user.name", "t")
        self.write("qa/sherlock/status.json", "{}\n")
        self.write("qa/sherlock/runs.jsonl", "")
        self.write("a.txt", "a\n")
        git(self.repo, "add", "-A")
        git(self.repo, "commit", "-qm", "init")


class CleanTests(EvalLoopBase):
    def test_clean(self):
        self.assertEqual(self.run_cli("clean").returncode, 0)

    def test_state_files_exempt(self):
        self.write("qa/sherlock/status.json", '{"x":1}\n')
        self.write("qa/sherlock/runs.jsonl", "{}\n")
        r = self.run_cli("clean")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_other_dirty_fails(self):
        self.write("a.txt", "changed\n")
        r = self.run_cli("clean")
        self.assertEqual(r.returncode, 1)
        self.assertIn("a.txt", r.stdout)

    def test_untracked_fails(self):
        self.write("new.rs", "x")
        self.assertEqual(self.run_cli("clean").returncode, 1)

    def test_state_commit_only_touches_state(self):
        self.write("qa/sherlock/status.json", '{"x":1}\n')
        self.write("a.txt", "changed\n")
        r = self.run_cli("state-commit")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)
        self.assertIn("chore(factory-state)", git(self.repo, "log", "-1", "--format=%s"))
        self.assertEqual(git(self.repo, "show", "--name-only", "--format=", "HEAD").split(),
                         ["qa/sherlock/status.json"])
        self.assertIn("a.txt", git(self.repo, "status", "--porcelain"))
        body = git(self.repo, "log", "-1", "--format=%B")
        self.assertNotIn("Co-Authored", body)

    def test_state_commit_noop(self):
        self.assertEqual(self.run_cli("state-commit").returncode, 0)


class ContractTests(EvalLoopBase):
    def contract(self, **over):
        c = {"format": "doctorwatson-contract/1", "id": "U999",
             "entrypoint": {"argv": ["python3", "-c", "import json;print(json.dumps({'chunks':[{'charges':{'events':'1'}}],'top':1}))"]},
             "expect": {"exit": 0, "json": True, "max_stdout_bytes": 1048576,
                        "required_paths": ["top", "chunks[].charges.events"],
                        "forbidden_paths": ["chunks[].accounting"]}}
        for k, v in over.items():
            c["expect"][k] = v
        self.write("qa/holdout/contracts/U999.json", json.dumps(c))
        return c

    def test_pass(self):
        self.contract()
        r = self.run_cli("contract-check", "U999")
        self.assertEqual(r.returncode, 0, r.stdout + r.stderr)

    def test_missing_path(self):
        self.contract(required_paths=["top", "chunks[].view"])
        r = self.run_cli("contract-check", "U999")
        self.assertEqual(r.returncode, 1)
        self.assertIn("chunks[].view", r.stdout)

    def test_forbidden_path(self):
        self.contract(forbidden_paths=["top"])
        self.assertEqual(self.run_cli("contract-check", "U999").returncode, 1)

    def test_size_cap(self):
        self.contract(max_stdout_bytes=10)
        r = self.run_cli("contract-check", "U999")
        self.assertEqual(r.returncode, 1)
        self.assertIn("stdout", r.stdout)

    def test_exit_mismatch(self):
        self.contract(exit=3)
        self.assertEqual(self.run_cli("contract-check", "U999").returncode, 1)

    def test_bad_contract(self):
        self.write("qa/holdout/contracts/U998.json", "{}")
        self.assertEqual(self.run_cli("contract-check", "U998").returncode, 2)

    def test_unknown_id(self):
        self.assertEqual(self.run_cli("contract-check", "NOPE").returncode, 2)
