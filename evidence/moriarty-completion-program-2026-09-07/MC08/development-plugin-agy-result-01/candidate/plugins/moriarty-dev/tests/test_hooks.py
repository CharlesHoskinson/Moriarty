import json
import os
import subprocess
import sys
import tempfile
import time
import unittest
from pathlib import Path

SCRIPTS_DIR = Path(__file__).resolve().parent.parent / "scripts"
TESTS_DIR = Path(__file__).resolve().parent
if str(SCRIPTS_DIR) not in sys.path:
    sys.path.insert(0, str(SCRIPTS_DIR))
if str(TESTS_DIR) not in sys.path:
    sys.path.insert(0, str(TESTS_DIR))

import test_records as tr
from moriarty_dev.store import get_db_path, init_db, record_event


class HookContractTestCase(unittest.TestCase):
    def setUp(self):
        self.temp_dir = tempfile.TemporaryDirectory()
        self.root = Path(self.temp_dir.name)
        subprocess.run(["git", "init"], cwd=str(self.root), capture_output=True, check=True)
        subprocess.run(["git", "config", "user.email", "test@moriarty.local"], cwd=str(self.root), check=True)
        subprocess.run(["git", "config", "user.name", "Test User"], cwd=str(self.root), check=True)
        self.db_path = get_db_path(self.root)
        init_db(self.db_path)
        self.hook_py = SCRIPTS_DIR / "moriarty_dev" / "hook.py"

    def tearDown(self):
        self.temp_dir.cleanup()

    def run_hook(self, event_name, payload):
        cmd = [sys.executable, str(self.hook_py), event_name]
        start = time.time()
        proc = subprocess.run(
            cmd,
            input=json.dumps(payload),
            capture_output=True,
            text=True,
            timeout=2.0,
        )
        elapsed = time.time() - start
        self.assertLessEqual(elapsed, 1.0, "Hook must execute within 1.0 second")
        self.assertLessEqual(len(proc.stdout.encode("utf-8")), 2048, "Hook output must be <= 2048 bytes")
        self.assertEqual(proc.returncode, 0)
        return json.loads(proc.stdout.strip() or "{}")

    def test_unrelated_repository_permits(self):
        # Empty git repo with no actions.json
        payload = {
            "hookEventName": "PreToolUse",
            "cwd": str(self.root),
            "toolName": "execute_command",
            "toolInput": {"command": "ls -la"},
        }
        res = self.run_hook("PreToolUse", payload)
        self.assertEqual(res.get("hookSpecificOutput", {}).get("permissionDecision"), "allow")

    def test_malformed_input_does_not_crash(self):
        cmd = [sys.executable, str(self.hook_py), "PreToolUse"]
        proc = subprocess.run(
            cmd,
            input="NOT_JSON_AT_ALL!!!",
            capture_output=True,
            text=True,
        )
        self.assertEqual(proc.returncode, 0)
        res = json.loads(proc.stdout.strip() or "{}")
        self.assertEqual(res.get("hookSpecificOutput", {}).get("permissionDecision"), "allow")

    def test_denied_known_dispatch(self):
        # Set up genuine registers in self.root with 2 failures
        tr.copy_genuine_registers(self.root)
        campaigns = tr.load_json(self.root / "evidence" / "moriarty-completion-program-2026-09-07" / "report-reconciliation" / "campaign-admission.json")
        tr.write_current_accounting(self.root)
        tr.attach_current_accounting(self.root, campaigns)
        tr.write_commands(self.root)

        implement_action = tr.loan_action("implement", "commands.json#driver")
        reproduce_action = tr.loan_action("reproduce", "commands.json#driver")
        tr.write_actions(self.root, [implement_action, reproduce_action, tr.loan_action("report")])

        for i in range(2):
            record_event(
                self.db_path,
                str(self.root),
                implement_action["requirement"],
                implement_action["capability"],
                implement_action["candidate"],
                implement_action["id"],
                "defect_failure",
                {"defectId": f"DEFECT-{i+1}"},
            )

        payload = {
            "hookEventName": "PreToolUse",
            "cwd": str(self.root),
            "toolName": "execute_command",
            "toolInput": {"command": "python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py run --action sp01-loan-implement"},
        }
        res = self.run_hook("PreToolUse", payload)
        hook_out = res.get("hookSpecificOutput", {})
        self.assertEqual(hook_out.get("permissionDecision"), "deny")
        self.assertIn("Repeated unresolved failure", hook_out.get("permissionDecisionReason", ""))
        self.assertIn("sp01-loan-reproduce", hook_out.get("permissionDecisionReason", ""))

    def test_permitted_diagnostic(self):
        payload = {
            "hookEventName": "PreToolUse",
            "cwd": str(self.root),
            "toolName": "execute_command",
            "toolInput": {"command": "python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py status"},
        }
        res = self.run_hook("PreToolUse", payload)
        self.assertEqual(res.get("hookSpecificOutput", {}).get("permissionDecision"), "allow")

    def test_stop_called_twice_has_no_auto_continuation(self):
        payload = {"hookEventName": "Stop", "cwd": str(self.root)}
        res1 = self.run_hook("Stop", payload)
        self.assertNotIn("autoContinuation", res1.get("hookSpecificOutput", {}))
        self.assertNotIn("continue", res1.get("hookSpecificOutput", {}))

        res2 = self.run_hook("Stop", payload)
        self.assertNotIn("autoContinuation", res2.get("hookSpecificOutput", {}))
        self.assertNotIn("continue", res2.get("hookSpecificOutput", {}))


if __name__ == "__main__":
    unittest.main()
