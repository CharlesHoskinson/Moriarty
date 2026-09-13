"""Exec-shim tests: preflight, argv/env, ownership, fail-closed. No K/strace."""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
import tempfile
import unittest
from pathlib import Path

HERE = Path(__file__).resolve().parent
ADAPTER_PATH = HERE / "k-macro05-trace106-diagnostic.py"
PINS_PATH = HERE / "k-macro05-trace106-pins.json"
STACK_SOFT = 8388608
KROOT = "/nix/store/y63xkr8pk2bqd5lh4889rlwldw26v9f4-k-7.1.337-4a46d1231473b599c699160132fd6e76a5c46406"
FORBIDDEN = (
    "PYTHONPATH", "PYTHONHOME", "PYTHONSTARTUP", "LD_PRELOAD",
    "JAVA_TOOL_OPTIONS", "_JAVA_OPTIONS", "PERL5LIB", "BASH_ENV",
)


def load_adapter():
    spec = importlib.util.spec_from_file_location("k_macro05_trace106_diagnostic", ADAPTER_PATH)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class ShimContractTests(unittest.TestCase):
    def test_collector_heuristics_are_gone(self):
        adapter = load_adapter()
        for name in (
            "parse_process_evidence", "_native_spawn", "claim_output_dir",
            "retain_observation", "default_output_dir", "communicate",
        ):
            self.assertFalse(hasattr(adapter, name), name)
        text = ADAPTER_PATH.read_text()
        self.assertNotIn("selectors", text)
        self.assertNotIn("SIGSEGV", text)
        self.assertNotIn("krunCalls", text)
        self.assertIn("os.execve", text)

    def test_child_env_preserves_home_and_drops_injection(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        env = adapter.child_env(pins, home="/home/charl")
        self.assertEqual(env["HOME"], "/home/charl")
        self.assertEqual(env["PATH"], "/usr/bin:/bin")
        self.assertEqual(env["LANG"], "C.UTF-8")
        self.assertEqual(env["PYTHONNOUSERSITE"], "1")
        self.assertNotIn("NIX_USER_CONF_FILES", env)
        self.assertNotIn("NIX_PROFILES", env)
        for name in FORBIDDEN:
            self.assertNotIn(name, env)

    def test_query_env_suppresses_nix_user_config(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        env = adapter.query_env(pins, home="/home/charl")
        self.assertEqual(env["HOME"], "/home/charl")
        self.assertEqual(env["PATH"], "/usr/bin:/bin")
        self.assertEqual(env["NIX_USER_CONF_FILES"], "/dev/null")
        self.assertEqual(env["NIX_CONFIG"], "")
        self.assertNotIn("NIX_PROFILES", env)
        for name in FORBIDDEN:
            self.assertNotIn(name, env)

    def test_missing_home_fails_closed(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        with self.assertRaisesRegex(adapter.PreflightError, "HOME_MISSING"):
            adapter.child_env(pins, home="")

    def test_selected_hash_drift_fails_before_exec(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        pins["selectedExecutables"][0]["sha256"] = "0" * 64
        executed = []
        with self.assertRaisesRegex(adapter.PreflightError, "PIN_MISMATCH"):
            adapter.run_shim(pins=pins, execve=lambda *a, **k: executed.append(1), here=HERE)
        self.assertEqual(executed, [])

    def test_arbitrary_overflow_path_is_rejected(self):
        adapter = load_adapter()
        ownership = json.loads((HERE / "k-macro05-trace106-ownership.json").read_text())
        host = adapter.load_host_evidence(ownership)
        with tempfile.TemporaryDirectory() as raw:
            path = str(Path(raw) / "not-in-observation")
            Path(path).write_text("x")
            os.chmod(path, 0o555)
            with self.assertRaisesRegex(adapter.PreflightError, "ARBITRARY_OVERFLOW"):
                adapter.verify_ownership(path, host, 65534, 0)

    def test_mapped_host_root_nix_path_is_not_writable(self):
        adapter = load_adapter()
        ownership = json.loads((HERE / "k-macro05-trace106-ownership.json").read_text())
        host = adapter.load_host_evidence(ownership)
        uid_map = Path("/proc/self/uid_map").read_text()
        expected = adapter.namespace_uid_for_host(uid_map, ownership["hostRootUid"], ownership["overflowUid"])
        st = Path(KROOT).stat()
        self.assertIn(st.st_uid, (0, 65534))
        self.assertFalse(os.access(KROOT, os.W_OK))
        adapter.verify_ownership(KROOT, host, expected, ownership["hostRootUid"])
        self.assertEqual(host[KROOT]["hostUid"], 0)
        self.assertEqual(host[KROOT]["lstatInode"], Path(KROOT).lstat().st_ino)
        if st.st_uid == 65534:
            self.assertEqual(ownership["hostRootUid"], 0)
            self.assertNotEqual(st.st_uid, 0)
            self.assertEqual(expected, 65534)

    def test_uid_map_identity_and_map_current_user(self):
        adapter = load_adapter()
        self.assertEqual(
            adapter.namespace_uid_for_host("         0          0 4294967295\n", 0, 65534),
            0,
        )
        self.assertEqual(
            adapter.namespace_uid_for_host("      1000       1000          1\n", 0, 65534),
            65534,
        )
        live = Path("/proc/self/uid_map").read_text()
        expected = adapter.namespace_uid_for_host(live, 0, 65534)
        self.assertEqual(Path(KROOT).stat().st_uid, expected)

    def test_host_stat_drift_fails_closed(self):
        adapter = load_adapter()
        ownership = json.loads((HERE / "k-macro05-trace106-ownership.json").read_text())
        host = adapter.load_host_evidence(ownership)
        mutated = dict(host[KROOT])
        mutated["lstatInode"] = 1
        host = dict(host)
        host[KROOT] = mutated
        with self.assertRaisesRegex(adapter.PreflightError, "HOST_STAT_DRIFT"):
            adapter.verify_ownership(KROOT, host, 65534, 0)

    def test_old_rootuid_observation_is_rejected(self):
        adapter = load_adapter()
        ownership = json.loads((HERE / "k-macro05-trace106-ownership.json").read_text())
        ownership = dict(ownership)
        ownership["hostEvidencePath"] = ownership["nixMetadataObservationPath"]
        ownership["hostEvidenceSHA256"] = ownership["nixMetadataObservationSHA256"]
        with self.assertRaisesRegex(adapter.PreflightError, "HOST_EVIDENCE_SCHEMA"):
            adapter.load_host_evidence(ownership)

    def test_env_is_not_claimed_host_root(self):
        adapter = load_adapter()
        ownership = json.loads((HERE / "k-macro05-trace106-ownership.json").read_text())
        host = adapter.load_host_evidence(ownership)
        self.assertNotIn("/usr/bin/env", host)
        self.assertIn("/usr/bin/python3.14", host)
        self.assertEqual(
            adapter.bind_path_for_ownership("/usr/bin/python3", host),
            "/usr/bin/python3.14",
        )
        self.assertIsNone(adapter.bind_path_for_ownership("/usr/bin/env", host))
        with self.assertRaisesRegex(adapter.PreflightError, "ARBITRARY_OVERFLOW"):
            adapter.verify_ownership("/usr/bin/env", host, 65534, 0)
        adapter.verify_nonwritable("/usr/bin/env", Path("/usr/bin/env").lstat().st_uid)

    def test_wrapper_path_and_parser_python_are_pinned(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        adapter.verify_wrapper_and_parser(pins)
        shebang = pins["parserShebang"]
        self.assertEqual(shebang["argument"], "python3")
        self.assertIn("python3-3.11.9/bin/python3", shebang["resolvedOnWrapperPath"])
        self.assertEqual(
            hashlib.sha256(Path(shebang["resolvedOnWrapperPath"]).read_bytes()).hexdigest(),
            shebang["sha256"],
        )

    def test_real_preflight_then_injected_exec_does_not_invoke_k(self):
        adapter = load_adapter()
        pins = adapter.load_pins(PINS_PATH)
        recorded = []

        def stub(path, argv, env):
            recorded.append((path, list(argv), dict(env)))

        adapter.run_shim(
            pins=pins,
            execve=stub,
            getrlimit=lambda *_: (STACK_SOFT, -1),
            setrlimit=lambda *a: None,
            here=HERE,
            home=os.environ.get("HOME", "/home/charl"),
        )
        self.assertEqual(len(recorded), 1)
        path, argv, env = recorded[0]
        self.assertEqual(path, "/usr/bin/strace")
        self.assertEqual(argv, pins["diagnosticArgv"])
        self.assertEqual(env["PATH"], "/usr/bin:/bin")
        self.assertEqual(env["HOME"], os.environ.get("HOME", "/home/charl"))
        self.assertNotIn("NIX_USER_CONF_FILES", env)
        for name in FORBIDDEN:
            self.assertNotIn(name, env)
        self.assertNotIn("krun", path)

    def test_query_uses_controlled_env_and_requisite_set(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        seen = []

        def fake_run(argv, executable=None, env=None, **_k):
            seen.append((list(argv), executable, dict(env)))
            class Result:
                returncode = 1
                stderr = "injected-stop"
                stdout = ""
            return Result()

        with self.assertRaisesRegex(adapter.PreflightError, "NAR_QUERY"):
            adapter.verify_requisites(
                pins, HERE, home="/home/charl", nix_run=fake_run,
            )
        self.assertGreaterEqual(len(seen), 1)
        argv, executable, env = seen[0]
        self.assertEqual(argv[0], "nix-store")
        self.assertEqual(argv[1:3], ["--query", "--requisites"])
        self.assertTrue(executable.endswith("/bin/nix"))
        self.assertEqual(env["NIX_USER_CONF_FILES"], "/dev/null")
        self.assertEqual(env["NIX_CONFIG"], "")
        self.assertEqual(env["HOME"], "/home/charl")
        self.assertNotIn("NIX_PROFILES", env)

    def test_requisite_set_mismatch_fails_closed(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        calls = {"n": 0}

        def fake_run(argv, executable=None, env=None, **_k):
            calls["n"] += 1
            class Result:
                returncode = 0
                stderr = ""
                stdout = "/nix/store/not-the-pinned-set\n"
            return Result()

        with self.assertRaisesRegex(adapter.PreflightError, "REQUISITE_SET"):
            adapter.verify_requisites(pins, HERE, home="/home/charl", nix_run=fake_run)

    def test_cli_rejects_unexpected_args(self):
        adapter = load_adapter()
        self.assertEqual(adapter.main(["--verify-only"]), 2)
        self.assertEqual(adapter.main(["--exec-stub"]), 2)

    def test_stack_mismatch_fails_closed(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        with self.assertRaisesRegex(adapter.PreflightError, "STACK_LIMIT"):
            adapter.apply_limits(pins, getrlimit=lambda *_: (16777216, -1), setrlimit=lambda *a: None)

    def test_nar_drift_fails_closed(self):
        adapter = load_adapter()
        pins = json.loads(PINS_PATH.read_text())
        requisites = json.loads((HERE / pins["manifests"]["requisites"]).read_text())
        requisites["entries"][0]["narHash"] = "sha256:0000000000000000000000000000000000000000000000000000000000000000"
        with tempfile.TemporaryDirectory() as raw:
            tmp = Path(raw)
            (tmp / "k-macro05-trace106-requisites.json").write_text(json.dumps(requisites))
            (tmp / "k-macro05-trace106-ownership.json").write_bytes((HERE / "k-macro05-trace106-ownership.json").read_bytes())
            with self.assertRaisesRegex(adapter.PreflightError, "PIN_MISMATCH"):
                adapter.verify_requisites(pins, tmp)


if __name__ == "__main__":
    unittest.main()
