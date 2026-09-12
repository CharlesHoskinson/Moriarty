"""Package-local contracts: runnable from a copied plugin without repo fixtures."""
import contextlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

PLUGIN = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(PLUGIN / "scripts"))
from moriarty_dev import cli, hook, records, store


class WireCompatibilityTests(unittest.TestCase):
    def invoke(self, raw, event="PreToolUse"):
        args = [sys.executable, str(PLUGIN / "scripts/moriarty_dev/hook.py")]
        if event is not None:
            args.append(event)
        result = subprocess.run(args, input=raw, capture_output=True, timeout=2)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, b"")
        self.assertLessEqual(len(result.stdout), 2048)
        return json.loads(result.stdout)

    def test_invalid_inputs_produce_common_diagnostics(self):
        for raw in (b"not-json", b"[]", b"null", b"\xff", b'{"x":"' + b"x" * 8192 + b'"}',
                    json.dumps({"cwd": 3}).encode(), json.dumps({"cwd": ""}).encode()):
            with self.subTest(raw=raw[:60]):
                response = self.invoke(raw)
                self.assertIn("systemMessage", response)
                self.assertNotIn("permissionDecision", response.get("hookSpecificOutput", {}))

    def test_unknown_or_invalid_events_never_leak_into_envelope(self):
        for event in (None, 1, {}, [], "FutureEvent", ""):
            with self.subTest(event=event):
                response = self.invoke(json.dumps({"hook_event_name": event}).encode(), event=None)
                self.assertIn("systemMessage", response)
                self.assertNotIn("hookSpecificOutput", response)

    def test_input_limit_counts_utf8_bytes(self):
        raw = json.dumps({"x": "界" * 3000}, ensure_ascii=False).encode()
        self.assertLess(len(raw.decode()), 8192)
        self.assertIn("systemMessage", self.invoke(raw))

    def test_malformed_input_never_calls_repository_handler(self):
        captured = io.StringIO()
        with patch.object(sys, "stdin", io.StringIO("[broken")), patch.object(sys, "argv", ["hook.py", "PreToolUse"]), patch.object(hook, "handle_event") as handler, contextlib.redirect_stdout(captured):
            hook.main()
        handler.assert_not_called()
        self.assertIn("systemMessage", json.loads(captured.getvalue()))

    def test_surrogate_in_payload_does_not_break_stdout_encoding(self):
        response = self.invoke(b'{"cwd":"\\ud800"}')
        self.assertIn("systemMessage", response)

    def test_stop_and_permitted_pretool_closed_contract(self):
        with tempfile.TemporaryDirectory() as directory:
            raw = json.dumps({"cwd": directory, "tool_name": "Bash", "tool_input": {"command": "pwd"}}).encode()
            self.assertEqual(self.invoke(raw, "Stop"), {})
            self.assertEqual(self.invoke(raw), {"hookSpecificOutput": {"hookEventName": "PreToolUse"}})


class DispatchRecognitionTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.action = {"id": "blocked-action", "commandRef": "commands.json#run"}

    def output(self, command):
        with patch.object(hook, "read_actions", return_value=[self.action]), patch.object(hook, "load_snapshot", return_value={}), patch.object(hook, "assess", return_value={"allow": False, "reasonCode": "AUTHORITY_UNAVAILABLE"}):
            return hook.handle_pre_tool_use({"tool_name": "Bash", "tool_input": {"command": command}}, self.root)

    def test_mentions_and_non_run_commands_are_not_dispatches(self):
        for command in ('rg blocked-action actions.json', 'printf "%s\\n" blocked-action',
                        'echo python3 cli.py run --action blocked-action',
                        'python3 cli.py review --action blocked-action --receipt review.json',
                        'python3 cli.py --repo blocked-action status'):
            with self.subTest(command=command):
                self.assertNotIn("permissionDecision", self.output(command)["hookSpecificOutput"])

    def test_supported_run_forms_retain_denials(self):
        for command in ('python3 cli.py run --action blocked-action',
                        'python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . run --action=blocked-action',
                        'python3 -m moriarty_dev.cli --json run --action blocked-action',
                        ['python3', 'cli.py', 'run', '--action', 'blocked-action']):
            with self.subTest(command=command):
                self.assertEqual(self.output(command)["hookSpecificOutput"]["permissionDecision"], "deny")

    def test_exact_raw_argv_denial_does_not_require_action_id(self):
        (self.root / "commands.json").write_text(json.dumps({"commands": {"run": {"argv": ["python3", "driver.py"]}}}))
        self.assertEqual(self.output("python3 driver.py")["hookSpecificOutput"]["permissionDecision"], "deny")


class InstallationCompatibilityTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="moriarty compatibility ")
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.home = self.root / "codex home"
        self.repo = self.root / "repo"
        self.repo.mkdir()
        (self.repo / ".git").mkdir()

    def run_cli(self, command, plugin=PLUGIN, before=False, extra=()):
        args = [sys.executable, str(plugin / "scripts/moriarty_dev/cli.py"), "--repo", str(self.repo)]
        args += ["--json", command] if before else [command, "--json"]
        result = subprocess.run(args + list(extra), env={**os.environ, "CODEX_HOME": str(self.home)}, capture_output=True, text=True, timeout=5)
        self.assertEqual(result.returncode, 0, result.stderr)
        return json.loads(result.stdout)

    def copy_plugin(self, version):
        dest = self.home / "plugins/cache/personal/moriarty-dev" / version
        shutil.copytree(PLUGIN, dest, ignore=shutil.ignore_patterns("__pycache__"))
        return dest

    def test_source_cached_report_and_doctor_agree(self):
        cached = self.copy_plugin("old-version")
        source = self.run_cli("doctor")
        installed = self.run_cli("doctor", cached)
        report = self.run_cli("report")
        self.assertTrue(source["hostInstalled"])
        self.assertEqual(source["hostCoverage"], installed["hostCoverage"])
        self.assertEqual(source["hostCoverage"], report["hostCoverage"])
        self.assertNotEqual(source["hostCoverage"], "host-verified")
        self.assertEqual(source["compatibility"]["activation"], "unknown")

    def test_json_flag_works_before_and_after_subcommand(self):
        self.assertEqual(self.run_cli("doctor"), self.run_cli("doctor", before=True))

    def test_expected_old_root_is_diagnosed_without_repair(self):
        missing = self.home / "plugins/cache/personal/moriarty-dev/missing-version"
        result = self.run_cli("doctor", extra=("--expected-plugin-root", str(missing)))
        observation = result["compatibility"]["expectedRoots"][0]
        self.assertEqual(observation["path"], str(missing))
        self.assertFalse(observation["usable"])
        self.assertFalse(missing.exists())

    def test_broken_cached_package_is_not_usable(self):
        cached = self.copy_plugin("broken-version")
        (cached / "scripts/moriarty_dev/hook.py").unlink()
        result = self.run_cli("doctor")
        self.assertFalse(result["hostInstalled"])
        observation = result["compatibility"]["cachedRoots"][0]
        self.assertFalse(observation["usable"])
        self.assertIn("scripts/moriarty_dev/hook.py", observation["missingFiles"])

    def test_coexisting_versions_execute_registered_commands(self):
        old = self.copy_plugin("old-version")
        old_bytes = (old / "scripts/moriarty_dev/hook.py").read_bytes()
        new = self.copy_plugin("new-version")
        for plugin in (old, new):
            definition = json.loads((plugin / "hooks/hooks.json").read_text())
            for event, groups in definition["hooks"].items():
                command = groups[0]["hooks"][0]["command"]
                result = subprocess.run(["bash", "-c", command], input=json.dumps({"cwd": str(self.repo)}), env={**os.environ, "PLUGIN_ROOT": str(plugin), "CLAUDE_PLUGIN_ROOT": "/stale"}, capture_output=True, text=True, timeout=2)
                self.assertEqual(result.returncode, 0, result.stderr)
                output = json.loads(result.stdout)
                self.assertEqual(output, {} if event == "Stop" else {"hookSpecificOutput": {"hookEventName": event}})
        self.assertEqual(old_bytes, (old / "scripts/moriarty_dev/hook.py").read_bytes())
        result = self.run_cli("doctor")
        self.assertEqual(len(result["compatibility"]["cachedRoots"]), 2)


class StoreHistoryCompatibilityTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory()
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / "main"
        (self.root / ".git").mkdir(parents=True)
        self.linked = Path(self.tmp.name) / "linked"
        self.linked.mkdir()
        gitdir = self.root / ".git/worktrees/linked"
        gitdir.mkdir(parents=True)
        (gitdir / "commondir").write_text("../..")
        (self.linked / ".git").write_text("gitdir: " + str(gitdir))
        self.db = store.get_db_path(self.root)

    def record_failure(self, root, name):
        store.record_event(self.db, str(root), "SP01", "cap", "a" * 64, "action", "defect_failure", {"findingId": name})

    def history(self, root):
        return store.make_history_reader(self.db)({"repository": records._repository_identity(root, []), "requirement": "SP01", "capability": "cap"})

    def test_historical_worktree_rows_and_admin_intervals_share_identity(self):
        self.record_failure(self.root, "F1")
        self.record_failure(self.linked, "F2")
        store.record_admin_interval(self.db, str(self.root), 0, 20, "review")
        store.record_admin_interval(self.db, str(self.linked), 10, 30, "review")
        for root in (self.root, self.linked):
            with self.subTest(root=root):
                history = self.history(root)
                self.assertIsNotNone(history)
                self.assertEqual(history["sameDefectFailures"], 2)
                self.assertEqual(history["adminSeconds"], 30)

    def test_removed_worktree_history_remains_and_other_repo_stays_isolated(self):
        self.record_failure(self.root, "F1")
        self.record_failure(self.linked, "F2")
        (self.linked / ".git").unlink()
        self.assertEqual(store.get_history(self.db, str(self.root), "SP01", "cap")["sameDefectFailures"], 2)
        other = Path(self.tmp.name) / "other"
        (other / ".git").mkdir(parents=True)
        self.assertIsNone(store.get_history(self.db, str(other), "SP01", "cap"))

    def receipt(self, **extra):
        return {"requirement": "SP01", "capability": "cap", "candidateHash": "b" * 64,
                "scope": "reviewed candidate", "author": "author", "reviewer": "independent", "verdict": "APPROVED", **extra}

    def test_approval_without_named_resolutions_cannot_clear_blockers(self):
        self.record_failure(self.root, "F1")
        for extra in ({}, {"resolvedFindings": []}):
            store.record_review(self.db, str(self.root), self.receipt(**extra))
            self.assertEqual(store.get_history(self.db, str(self.root), "SP01", "cap")["sameDefectFailures"], 1)

    def test_explicit_resolutions_clear_only_named_stable_ids(self):
        self.record_failure(self.root, "F1")
        self.record_failure(self.root, "F2")
        store.record_review(self.db, str(self.root), self.receipt(resolvedFindings=["F1"]))
        self.assertEqual(store.get_history(self.db, str(self.root), "SP01", "cap")["sameDefectFailures"], 1)
        store.record_review(self.db, str(self.root), self.receipt(resolved=["F2"]))
        self.assertEqual(store.get_history(self.db, str(self.root), "SP01", "cap")["sameDefectFailures"], 0)

    def test_malformed_resolution_list_rejected_before_state_write(self):
        self.record_failure(self.root, "F1")
        for value in ("F1", [None], [""], [" "], [1], {}):
            with self.subTest(value=value), self.assertRaises(store.StoreError):
                store.record_review(self.db, str(self.root), self.receipt(resolvedFindings=value))
        self.assertEqual(store.get_history(self.db, str(self.root), "SP01", "cap")["sameDefectFailures"], 1)

    def test_outbox_survives_removed_worktree_and_duplicate_observation(self):
        store.enqueue_tx(self.db, str(self.linked), "fixture-removed-worktree", "submitted", {"network": "preview"})
        (self.linked / ".git").unlink()
        pending = store.get_undelivered_txs(self.db, str(self.root))
        self.assertEqual([row["txId"] for row in pending], ["fixture-removed-worktree"])
        store.enqueue_tx(self.db, str(self.root), "fixture-removed-worktree", "submitted", {"network": "preview"})
        self.assertEqual(len(store.get_undelivered_txs(self.db, str(self.root))), 1)
        other = Path(self.tmp.name) / "other"
        (other / ".git").mkdir(parents=True)
        self.assertEqual(store.get_undelivered_txs(self.db, str(other)), [])
        with self.assertRaises(store.StoreError):
            store.enqueue_tx(self.db, str(other), "fixture-removed-worktree", "submitted", {"network": "preview"})

    def test_outbox_rejects_wrong_database_before_creating_it(self):
        wrong_db = Path(self.tmp.name) / "other/.git/moriarty-dev/state.sqlite3"
        with self.assertRaises(store.StoreError):
            store.enqueue_tx(wrong_db, str(self.root), "fixture-wrong-database", "submitted", {"network": "preview"})
        self.assertFalse(wrong_db.exists())


class SprintCompletionTests(unittest.TestCase):
    def reports(self, sprints):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "openspec/sprints").mkdir(parents=True)
            (root / "openspec/sprints/sprints.json").write_text(json.dumps({"sprints": sprints}))
            (root / "openspec/moriarty-completion-program.json").write_text(json.dumps({"reportReconciliation": {"stageAdmission": {"stages": [{"id": "done", "status": "complete"}]}}}))
            return {row["sprint"]: row for row in cli._derive_sprint_reports(root)}

    def sprint(self, name, required=(), stages=("done",)):
        return {"id": name, "title": name, "stages": list(stages), "completionRequires": list(required)}

    def test_dependencies_missing_and_cycles_stay_open(self):
        for sprints in ([self.sprint("SP05", ["SP01"])],
                        [self.sprint("SP05", ["SP01"]), self.sprint("SP01", stages=["open"])],
                        [self.sprint("SP05", ["SP01"]), self.sprint("SP01", ["SP05"])],
                        [self.sprint("SP05", ["SP05"])],
                        [self.sprint("SP05", stages=[])]):
            with self.subTest(sprints=sprints):
                result = self.reports(sprints)["SP05"]
                self.assertEqual(result["status"], "open")
                self.assertFalse(result["demonstrated"])

    def test_completion_propagates_independent_of_list_order(self):
        result = self.reports([self.sprint("SP05", ["SP02"]), self.sprint("SP02", ["SP01"]), self.sprint("SP01")])
        self.assertTrue(all(row["demonstrated"] for row in result.values()))
        self.assertNotIn("completion requirements:", result["SP05"]["predicate"])


if __name__ == "__main__":
    unittest.main()
