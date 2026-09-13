"""Exercise registrations across cache deletion, not just package coexistence."""
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
from moriarty_dev import deployment


class UpgradeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="moriarty upgrade 界 ")
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.source = self.root / "source"
        shutil.copytree(PLUGIN, self.source, ignore=shutil.ignore_patterns("__pycache__"))
        self.user = self.root / "user home"
        self.runtime = self.user / ".local/share/moriarty-dev"
        self.cache = self.root / "cache"
        self.backups = self.root / "backups"
        self.repo = self.root / "repo"
        (self.repo / ".git").mkdir(parents=True)

    def command(self, event):
        return json.loads((self.source / "hooks/hooks.json").read_text())["hooks"][event][0]["hooks"][0]["command"]

    def invoke(self, command, selected=None, payload=None):
        result = subprocess.run(["sh", "-c", command], input=json.dumps(payload or {"cwd": str(self.repo)}),
            capture_output=True, text=True, timeout=3,
            env={**os.environ, "HOME": str(self.user), "PLUGIN_ROOT": str(selected or self.cache / "gone"),
                 "CLAUDE_PLUGIN_ROOT": str(self.source), "PYTHONDONTWRITEBYTECODE": "1"})
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        self.assertLessEqual(len(result.stdout.encode()), 2048)
        return json.loads(result.stdout)

    def test_deleted_cache_keeps_permit_stop_and_registered_denial(self):
        deployment.prepare(self.source, self.runtime)
        (self.repo / ".moriarty-dev").mkdir()
        action = {"id":"blocked", "commandRef":"commands.json#run", "requirement":"SP01", "capability":"test",
                  "kind":"implement", "candidate":"test", "admissionRef":"campaign:test", "evidenceProfile":"test"}
        (self.repo / ".moriarty-dev/actions.json").write_text(json.dumps({"schema":"moriarty-dev.actions/1", "actions":[action]}))
        (self.repo / "commands.json").write_text(json.dumps({"commands":{"run":{"argv":["python3","driver.py"]}}}))
        payload = {"cwd":str(self.repo), "tool_name":"exec_command", "tool_input":{"cmd":"python3 driver.py"}}
        self.assertEqual(self.invoke(self.command("PreToolUse"), payload=payload)["hookSpecificOutput"]["permissionDecision"], "deny")
        payload["tool_input"]["cmd"] = "pwd"
        self.assertNotIn("permissionDecision", self.invoke(self.command("PreToolUse"), payload=payload)["hookSpecificOutput"])
        self.assertEqual(self.invoke(self.command("Stop")), {})

    def test_missing_and_tampered_runtime_never_executes_other_version(self):
        digest = deployment.prepare(self.source, self.runtime)
        command = self.command("Stop")
        target = self.runtime / "runtimes" / digest / "scripts/moriarty_dev/hook.py"
        marker = self.root / "tampered-ran"
        target.write_text("from pathlib import Path; Path(" + repr(str(marker)) + ").touch()\nraise RuntimeError('tampered code')")
        direct = subprocess.run([sys.executable, str(target)], capture_output=True)
        self.assertNotEqual(direct.returncode, 0)
        self.assertTrue(marker.exists())
        marker.unlink()
        self.assertEqual(set(self.invoke(command)), {"systemMessage"})
        self.assertFalse(marker.exists())
        shutil.rmtree(self.runtime / "runtimes" / digest)
        self.assertEqual(set(self.invoke(command)), {"systemMessage"})
        # A nonempty native root must not fall through to an unrelated legacy root.
        self.assertEqual(set(self.invoke(command, selected=self.cache / "missing")), {"systemMessage"})

    def test_two_pins_keep_exact_versions_after_cache_removal(self):
        hook = self.source / "scripts/moriarty_dev/hook.py"
        hook.write_text("print('{\"systemMessage\":\"first\"}')\n")
        first = deployment.prepare(self.source, self.runtime)
        command1 = self.command("Stop")
        hook.write_text("print('{\"systemMessage\":\"second\"}')\n")
        second = deployment.prepare(self.source, self.runtime)
        command2 = self.command("Stop")
        self.assertNotEqual(first, second)
        self.assertEqual(self.invoke(command1)["systemMessage"], "first")
        self.assertEqual(self.invoke(command2)["systemMessage"], "second")

    def fake_installer(self, code, conflict=False):
        script = self.root / "installer.py"
        script.write_text("import shutil, pathlib, sys\n"
            + ("pathlib.Path(sys.argv[1], 'old', 'kept.txt').write_text('concurrent edit')\n" if conflict else "shutil.rmtree(sys.argv[1])\n")
            + "shutil.copytree(sys.argv[2], pathlib.Path(sys.argv[1]) / sys.argv[3])\n"
            + f"raise SystemExit({code})\n")
        version = json.loads((self.source / ".codex-plugin/plugin.json").read_text())["version"]
        return [sys.executable, str(script), str(self.cache), str(self.source), version]

    def test_installer_success_and_failure_restore_before_return(self):
        for code in (0, 7):
            with self.subTest(code=code):
                old = self.cache / "old"
                old.mkdir(parents=True, exist_ok=True)
                (old / "kept.txt").write_text("original")
                deployment.prepare(self.source, self.runtime)
                result = deployment.upgrade(self.source, self.cache, self.backups, self.runtime, self.fake_installer(code))
                self.assertEqual((old / "kept.txt").read_text(), "original")
                self.assertEqual(result["installerReturnCode"], code)
                self.assertTrue(result["installedVerified"])

    def test_modified_surviving_cache_is_preserved_and_reported(self):
        old = self.cache / "old"
        old.mkdir(parents=True)
        (old / "kept.txt").write_text("original")
        deployment.prepare(self.source, self.runtime)
        with self.assertRaisesRegex(RuntimeError, "conflict"):
            deployment.upgrade(self.source, self.cache, self.backups, self.runtime, self.fake_installer(0, conflict=True))
        self.assertEqual((old / "kept.txt").read_text(), "concurrent edit")

    def test_stale_registration_refuses_installer(self):
        deployment.prepare(self.source, self.runtime)
        (self.source / "scripts/moriarty_dev/hook.py").write_text("print('{}')\n")
        marker = self.root / "installer-ran"
        with self.assertRaisesRegex(ValueError, "registration"):
            deployment.upgrade(self.source, self.cache, self.backups, self.runtime,
                [sys.executable,"-c", "from pathlib import Path; Path(" + repr(str(marker)) + ").touch()"])
        self.assertFalse(marker.exists())

    def test_partial_cache_removal_restores_missing_files(self):
        old = self.cache / "old"
        (old / "nested").mkdir(parents=True)
        (old / "nested/hook.py").write_text("original")
        (old / "kept.txt").write_text("keep")
        deployment.prepare(self.source, self.runtime)
        script = "from pathlib import Path; Path(" + repr(str(old / "nested/hook.py")) + ").unlink()"
        result = deployment.upgrade(self.source, self.cache, self.backups, self.runtime, [sys.executable,"-c",script])
        self.assertTrue(result["restorationComplete"])
        self.assertEqual((old / "nested/hook.py").read_text(), "original")
        self.assertEqual((old / "kept.txt").read_text(), "keep")

    def test_missing_interpreter_has_common_diagnostic(self):
        deployment.prepare(self.source, self.runtime)
        result = subprocess.run(["/bin/sh", "-c", self.command("Stop")], input="{}", text=True,
                                capture_output=True, env={"PATH":str(self.root / "no-tools")})
        self.assertEqual(result.returncode, 0)
        self.assertEqual(set(json.loads(result.stdout)), {"systemMessage"})

    def test_added_python_file_and_symlink_are_rejected(self):
        digest = deployment.prepare(self.source, self.runtime)
        target = self.runtime / "runtimes" / digest / "scripts/moriarty_dev"
        extra = target / "extra.py"
        extra.write_text("raise RuntimeError('unverified addition')")
        self.assertIn("systemMessage", self.invoke(self.command("Stop")))
        extra.unlink()
        extra.symlink_to(self.source / "scripts/moriarty_dev/hook.py")
        self.assertIn("systemMessage", self.invoke(self.command("Stop")))

    def test_snapshot_failure_prevents_install(self):
        old = self.cache / "old"
        old.mkdir(parents=True)
        (old / "kept.txt").write_text("original")
        deployment.prepare(self.source, self.runtime)
        actual_run = subprocess.run
        with patch.object(deployment, "copy_atomic", side_effect=OSError("snapshot failed")), patch.object(deployment.subprocess, "run", wraps=actual_run) as run:
            with self.assertRaisesRegex(OSError, "snapshot failed"):
                deployment.upgrade(self.source, self.cache, self.backups, self.runtime, ["must-not-run"])
            self.assertFalse(any(call.args[0] == ["must-not-run"] for call in run.call_args_list))

    def test_installer_exception_still_restores(self):
        old = self.cache / "old"
        old.mkdir(parents=True)
        (old / "kept.txt").write_text("original")
        deployment.prepare(self.source, self.runtime)
        actual_run = subprocess.run
        def fail(argv, *args, **kwargs):
            if argv == ["missing-installer"]:
                shutil.rmtree(old)
                raise OSError("installer failed to spawn")
            return actual_run(argv, *args, **kwargs)
        with patch.object(deployment.subprocess, "run", side_effect=fail), self.assertRaisesRegex(OSError, "failed to spawn"):
            deployment.upgrade(self.source, self.cache, self.backups, self.runtime, ["missing-installer"])
        self.assertEqual((old / "kept.txt").read_text(), "original")

    def test_prepare_reuses_matching_runtime_without_overwrite(self):
        digest = deployment.prepare(self.source, self.runtime)
        hook = self.runtime / "runtimes" / digest / "scripts/moriarty_dev/hook.py"
        stamp = hook.stat().st_mtime_ns
        self.assertEqual(deployment.prepare(self.source, self.runtime), digest)
        self.assertEqual(hook.stat().st_mtime_ns, stamp)
        hook.write_text("print('{}')\n")
        with self.assertRaisesRegex(ValueError, "runtime conflict"):
            deployment.prepare(self.source, self.runtime)
        self.assertEqual(hook.read_text(), "print('{}')\n")

    def test_isolated_python_ignores_shadow_modules(self):
        deployment.prepare(self.source, self.runtime)
        shadow = self.root / "shadow"
        shadow.mkdir()
        marker = self.root / "shadow-ran"
        (shadow / "json.py").write_text("from pathlib import Path; Path(" + repr(str(marker)) + ").touch()\n")
        with patch.dict(os.environ, {"PYTHONPATH":str(shadow)}):
            self.assertEqual(self.invoke(self.command("Stop")), {})
        self.assertFalse(marker.exists())

    def test_concurrent_prepare_publishes_one_complete_runtime(self):
        from concurrent.futures import ThreadPoolExecutor
        other = self.root / "other-source"
        shutil.copytree(self.source, other)
        with ThreadPoolExecutor(max_workers=2) as pool:
            futures = [pool.submit(deployment.prepare, source, self.runtime) for source in (self.source, other)]
            digests = [future.result(timeout=5) for future in futures]
        self.assertEqual(digests[0], digests[1])
        self.assertEqual(self.invoke(self.command("Stop")), {})

    def test_invalid_runtime_source_prevents_installer(self):
        for corruption in ("missing", "syntax"):
            with self.subTest(corruption=corruption):
                path = self.source / "scripts/moriarty_dev/policy.py"
                original = path.read_bytes()
                if corruption == "missing":
                    path.unlink()
                else:
                    path.write_text("invalid Python syntax !!!")
                deployment.prepare(self.source, self.runtime)
                marker = self.root / "invalid-source-installer-ran"
                with self.assertRaisesRegex(ValueError, "source"):
                    deployment.upgrade(self.source, self.cache, self.backups, self.runtime,
                        [sys.executable,"-c","from pathlib import Path; Path(" + repr(str(marker)) + ").touch()"])
                self.assertFalse(marker.exists())
                path.write_bytes(original)

    def test_all_marketplaces_and_legacy_roots_are_restored(self):
        codex_home = self.root / "codex"
        roots = [codex_home / "plugins/cache/personal/moriarty-dev/v1",
                 codex_home / "plugins/cache/other/moriarty-dev/v1", codex_home / "plugins/moriarty-dev"]
        for root in roots:
            root.mkdir(parents=True)
            (root / "hook.py").write_text(str(root))
        discovered = deployment.retained_roots(codex_home)
        self.assertEqual(set(discovered), set(roots))
        deployment.prepare(self.source, self.runtime)
        script = "import shutil; shutil.rmtree(" + repr(str(codex_home / "plugins")) + ")"
        result = deployment.upgrade(self.source, self.cache, self.backups, self.runtime,
            [sys.executable,"-c",script], discovered)
        self.assertTrue(result["restorationComplete"])
        for root in roots:
            self.assertEqual((root / "hook.py").read_text(), str(root))

    def test_invalid_installed_candidate_preserves_receipt_outcomes(self):
        deployment.prepare(self.source, self.runtime)
        version = json.loads((self.source / ".codex-plugin/plugin.json").read_text())["version"]
        installed = self.cache / version
        installed.mkdir(parents=True)
        # Create invalid candidate only after snapshots were captured.
        script = "from pathlib import Path; Path(" + repr(str(installed / "link")) + ").symlink_to('/missing')"
        result = deployment.upgrade(self.source, self.cache, self.backups, self.runtime, [sys.executable,"-c",script])
        receipt = json.loads((Path(result["backup"]) / "receipt.json").read_text())
        self.assertEqual(receipt["installerReturnCode"], 0)
        self.assertTrue(receipt["restorationComplete"])
        self.assertFalse(receipt["installedVerified"])
        self.assertIn("installedVerificationError", receipt)

    def test_doctor_observes_pin_without_promoting_host_coverage(self):
        from moriarty_dev.compatibility import inspect_compatibility, observed_coverage
        digest = deployment.prepare(self.source, self.runtime)
        with patch.dict(os.environ, {"HOME":str(self.user), "CODEX_HOME":str(self.root / "empty-codex")}):
            observed = inspect_compatibility(self.source)
        self.assertEqual(observed["executingPlugin"]["runtimePin"], digest)
        self.assertTrue(observed["executingPlugin"]["sourceRuntimeMatches"])
        self.assertTrue(observed["executingPlugin"]["retainedRuntime"]["digestMatches"])
        self.assertEqual(observed["hookTrust"], "unknown")
        self.assertEqual(observed_coverage(observed), "unverified")


if __name__ == "__main__":
    unittest.main()
