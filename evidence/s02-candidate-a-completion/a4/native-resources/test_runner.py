import importlib.util
from pathlib import Path


spec = importlib.util.spec_from_file_location("native_resource_runner", Path(__file__).with_name("runner.py"))
runner = importlib.util.module_from_spec(spec)
spec.loader.exec_module(runner)


def test_present_empty_resource_is_incomplete(tmp_path):
    resource = tmp_path / "resources.txt"
    resource.write_bytes(b"")
    assert runner.resource_complete(resource, ["python", "child.py"]) is False


import hashlib
import json
import os
import subprocess
import sys
import time
import pytest


def child(tmp_path, body, name="child"):
    source = tmp_path / (name + ".py")
    source.write_text(body)
    return [sys.executable, "-B", "-X", f"pycache_prefix={tmp_path / (name + '-cache')}", str(source)]


def run_child(tmp_path, body=None, **kwargs):
    inner = tmp_path / "inner.json"
    if body is None:
        body = f"from pathlib import Path\nPath({str(inner)!r}).write_text('{{}}')\nprint('honest child stdout')\n"
    argv = child(tmp_path, body)
    folder = tmp_path / "capture"
    code = runner.capture(argv, folder, inner, **kwargs)
    terminal = json.loads((folder / "terminal.json").read_text())
    return argv, folder, code, terminal


def test_honest_measured_child_and_all_sidecar_hashes(tmp_path):
    argv, folder, code, terminal = run_child(tmp_path)
    assert code == 0 and terminal["eligible"] is True
    assert terminal["inner_receipt_complete"] is False
    assert terminal["inner_receipt_independent_validation_required"] is True
    assert terminal["resource_complete"] is True
    assert len(terminal["resource"]["fields"]) == 23
    assert terminal["resource"]["maximum_rss_kib"] > 0
    assert terminal["resource"]["exit_status"] == 0
    assert terminal["cleanup"] == {"forced": False, "signals": [], "errors": [], "complete": True}
    assert terminal["source_runtime_before"] == terminal["source_runtime_after"]
    for name, digest in terminal["sidecar_sha256"].items():
        assert digest == hashlib.sha256(Path(name).read_bytes()).hexdigest()
    assert terminal["inner_receipt_sha256"] == hashlib.sha256((tmp_path / "inner.json").read_bytes()).hexdigest()
    assert (folder / "stdout.bin").read_bytes() == b"honest child stdout\n"
    launch = json.loads((folder / "launch.json").read_text())
    assert launch["argv"] == ["/usr/bin/time", "-v", "-o", str(folder / "resources.txt"), *argv]
    assert launch["child_argv"] == argv and launch["original_parent_argv"] == sys.orig_argv
    assert launch["fixed_environment"]["LC_ALL"] == "C"
    assert runner.parse_resource(folder / "resources.txt", argv)["fields"]["Command being timed"] == '"' + " ".join(argv) + '"'


def test_missing_inner_is_not_eligible(tmp_path):
    _, _, code, terminal = run_child(tmp_path, "print('no receipt')\n")
    assert code == 2 and terminal["actual_exit"] == 0
    assert terminal["resource_complete"] and not terminal["inner_receipt_present"]
    assert not terminal["eligible"]


def test_nonzero_child_preserves_actual_status(tmp_path):
    inner = tmp_path / "inner.json"
    body = f"from pathlib import Path\nPath({str(inner)!r}).write_text('{{}}')\nraise SystemExit(7)\n"
    _, _, code, terminal = run_child(tmp_path, body)
    assert code == terminal["actual_exit"] == terminal["resource"]["exit_status"] == 7
    assert terminal["resource_complete"] and not terminal["eligible"]
    assert terminal["resource"]["diagnostics"] == ["Command exited with non-zero status 7"]


@pytest.mark.parametrize("mutation", ["truncated", "duplicate", "unknown", "negative", "nan", "inf", "malformed-wall", "zero-rss", "command"])
def test_malformed_or_substituted_resource_report(tmp_path, mutation):
    argv, folder, _, _ = run_child(tmp_path)
    text = (folder / "resources.txt").read_text()
    if mutation == "truncated": text = "\n".join(text.splitlines()[:-1]) + "\n"
    elif mutation == "duplicate": text += "\tExit status: 0\n"
    elif mutation == "unknown": text += "\tForeign counter: 0\n"
    elif mutation == "negative": text = text.replace("\tSwaps: 0", "\tSwaps: -1")
    elif mutation in ("nan", "inf"):
        text = "\n".join("\tUser time (seconds): " + mutation if line.startswith("\tUser time") else line for line in text.splitlines()) + "\n"
    elif mutation == "malformed-wall":
        text = "\n".join("\tElapsed (wall clock) time (h:mm:ss or m:ss): 1:99.0" if line.startswith("\tElapsed") else line for line in text.splitlines()) + "\n"
    elif mutation == "zero-rss":
        text = "\n".join("\tMaximum resident set size (kbytes): 0" if line.startswith("\tMaximum") else line for line in text.splitlines()) + "\n"
    elif mutation == "command":
        text = text.replace('"' + " ".join(argv) + '"', '"python substituted.py"')
    changed = tmp_path / "changed-resources.txt"
    changed.write_text(text)
    assert not runner.resource_complete(changed, argv)
    with pytest.raises(runner.ResourceError): runner.parse_resource(changed, argv)


def test_resource_limit_uses_bounded_read(tmp_path, monkeypatch):
    path = tmp_path / "oversize.txt"
    path.write_bytes(b"x" * 65537)
    original = Path.open
    requests = []
    class Guarded:
        def __init__(self, stream): self.stream = stream
        def __enter__(self): return self
        def __exit__(self, *args): self.stream.close()
        def read(self, size=-1):
            requests.append(size)
            assert 0 < size <= 65537
            return self.stream.read(size)
    def opened(self, *args, **kwargs):
        stream = original(self, *args, **kwargs)
        return Guarded(stream) if self == path else stream
    monkeypatch.setattr(Path, "open", opened)
    with pytest.raises(runner.ResourceError, match="64 KiB"):
        runner.parse_resource(path, ["python"])
    assert requests == [65537]


@pytest.mark.parametrize("wall", [True, False, 0, -1, 1.5, "1", None])
def test_invalid_wall_does_not_create_destination(tmp_path, wall):
    with pytest.raises(ValueError, match="wall"):
        runner.capture(["unused"], tmp_path / "capture", tmp_path / "inner", wall_seconds=wall)
    assert not (tmp_path / "capture").exists()


@pytest.mark.parametrize("argv", [[], "python", [""], [True], [1]])
def test_invalid_argv_fails_before_launch(tmp_path, argv):
    with pytest.raises(ValueError, match="argv"):
        runner.capture(argv, tmp_path / "capture", tmp_path / "inner")
    assert not (tmp_path / "capture").exists()


def test_exclusive_destinations_and_existing_inner(tmp_path):
    destination = tmp_path / "capture"
    destination.mkdir()
    original = destination / "original"
    original.write_bytes(b"do not overwrite")
    with pytest.raises(ValueError, match="already exists"):
        runner.capture(["unused"], destination, tmp_path / "inner")
    assert original.read_bytes() == b"do not overwrite"
    inner = tmp_path / "inner"
    inner.write_bytes(b"original inner")
    with pytest.raises(ValueError, match="already exists"):
        runner.capture(["unused"], tmp_path / "new", inner)
    assert inner.read_bytes() == b"original inner" and not (tmp_path / "new").exists()


@pytest.mark.parametrize("target", ["outer", "ancestor", "inner"])
def test_symlink_destination_rejected(tmp_path, target):
    real = tmp_path / "real"
    real.mkdir()
    alias = tmp_path / "alias"
    alias.symlink_to(real, target_is_directory=True)
    destination, inner = tmp_path / "capture", tmp_path / "inner"
    if target == "outer": destination = alias
    elif target == "ancestor": destination = alias / "capture"
    else: inner = alias / "inner"
    with pytest.raises(ValueError, match="symlink"):
        runner.capture(["unused"], destination, inner)
    assert list(real.iterdir()) == []


def test_environment_values_are_not_leaked(tmp_path, monkeypatch):
    poison = {"PYTHON_FAKE": "private-poison-value", "PYTEST_FAKE": "private-poison-value",
              "NODE_FAKE": "private-poison-value", "LD_FAKE": "private-poison-value"}
    for key, value in poison.items(): monkeypatch.setenv(key, value)
    inner = tmp_path / "inner.json"
    selected = [*poison, "PYTHONNOUSERSITE", "PYTHONDONTWRITEBYTECODE", "NODE_DISABLE_COMPILE_CACHE", "LC_ALL"]
    body = f"import os,json\nfrom pathlib import Path\nPath({str(inner)!r}).write_text('{{}}')\nprint(json.dumps({{k:os.environ[k] for k in {selected!r} if k in os.environ}}))\n"
    _, folder, code, _ = run_child(tmp_path, body)
    assert code == 0
    observed = json.loads((folder / "stdout.bin").read_text())
    for key in poison: assert key not in observed
    for key, value in {"PYTHONNOUSERSITE": "1", "PYTHONDONTWRITEBYTECODE": "1", "NODE_DISABLE_COMPILE_CACHE": "1", "LC_ALL": "C"}.items(): assert observed[key] == value
    launch = (folder / "launch.json").read_text()
    assert "private-poison-value" not in launch
    assert set(poison) <= set(json.loads(launch)["removed_environment_keys"])


def test_time_pin_mismatch_is_prelaunch(tmp_path, monkeypatch):
    monkeypatch.setattr(runner, "TIME_SHA", "0" * 64)
    with pytest.raises(ValueError, match="unadmitted"):
        runner.capture(["unused"], tmp_path / "capture", tmp_path / "inner")
    assert not (tmp_path / "capture").exists()


def test_inner_cannot_alias_outer_sidecars_or_dangling_link(tmp_path):
    destination = tmp_path / "capture"
    for inner in [destination / "terminal.json", destination / ".." / "capture" / "stdout.bin"]:
        with pytest.raises(ValueError, match="outside"):
            runner.capture(["unused"], destination, inner)
    dangling = tmp_path / "dangling-inner"
    dangling.symlink_to(tmp_path / "missing")
    with pytest.raises(ValueError, match="symlink"):
        runner.capture(["unused"], destination, dangling)
    assert not destination.exists()


def test_real_launch_error_retains_terminal(tmp_path):
    _, folder, code, terminal = run_child(tmp_path, cwd=tmp_path / "missing-working-directory")
    assert code == 2 and terminal["launch_error"]["type"] == "FileNotFoundError"
    assert terminal["actual_exit"] is None and not terminal["eligible"]
    assert not terminal["inner_receipt_present"] and not terminal["resource_complete"]
    assert (folder / "stdout.bin").read_bytes() == (folder / "stderr.bin").read_bytes() == b""


def test_simple_timeout_is_incomplete(tmp_path):
    _, _, code, terminal = run_child(tmp_path, "import time\ntime.sleep(60)\n", wall_seconds=1, grace_seconds=0.1, cleanup_seconds=0.1)
    assert code == 124 and terminal["timed_out"]
    assert not terminal["resource_complete"] and not terminal["eligible"]
    assert not terminal["inner_receipt_complete"]
    assert [s["signal"] for s in terminal["cleanup"]["signals"]] == ["SIGTERM", "SIGKILL"]


def test_monitor_error_still_cleans_actual_owned_process(tmp_path, monkeypatch):
    # Inject a monitor failure, not a fabricated process. Popen still starts the child.
    original = runner.subprocess.Popen
    def launch(*args, **kwargs):
        process = original(*args, **kwargs)
        wait = process.wait
        calls = {"count": 0}
        def broken_once(*args, **kwargs):
            calls["count"] += 1
            if calls["count"] == 1: raise RuntimeError("controlled monitor error")
            return wait(*args, **kwargs)
        process.wait = broken_once
        return process
    monkeypatch.setattr(runner.subprocess, "Popen", launch)
    _, _, code, terminal = run_child(tmp_path, "import time\ntime.sleep(60)\n", grace_seconds=0.1, cleanup_seconds=0.1)
    assert code == 2 and terminal["monitor_error"]["message"] == "controlled monitor error"
    assert terminal["cleanup"]["forced"] and not terminal["eligible"]
    assert [item["signal"] for item in terminal["cleanup"]["signals"]] == ["SIGTERM", "SIGKILL"]


def test_cleanup_error_keeps_terminal_and_attempts_kill(tmp_path, monkeypatch):
    original = runner.os.killpg
    def fail_term(pgid, sig):
        if sig == runner.signal.SIGTERM: raise PermissionError("controlled TERM error")
        return original(pgid, sig)
    monkeypatch.setattr(runner.os, "killpg", fail_term)
    _, _, code, terminal = run_child(tmp_path, "import time\ntime.sleep(60)\n", wall_seconds=1, grace_seconds=0.1, cleanup_seconds=0.1)
    assert code == 124 and terminal["timed_out"]
    assert terminal["cleanup"]["errors"][0]["message"] == "controlled TERM error"
    assert any(item["signal"] == "SIGKILL" and item["sent"] for item in terminal["cleanup"]["signals"])
    assert not terminal["cleanup"]["complete"] and not terminal["eligible"]


@pytest.mark.parametrize("mode", ["timeout", "ordinary-exit"])
def test_leader_exit_never_hides_live_descendant(tmp_path, mode):
    pid_path, heartbeat = tmp_path / "descendant.pid", tmp_path / "heartbeat"
    descendant = child(tmp_path, f"import os,signal,time\nfrom pathlib import Path\nsignal.signal(signal.SIGTERM,signal.SIG_IGN)\nPath({str(pid_path)!r}).write_text(str(os.getpid()))\nwhile True:\n with Path({str(heartbeat)!r}).open('a') as f: f.write('x')\n time.sleep(0.02)\n", "descendant")
    inner = tmp_path / "inner.json"
    body = f"import subprocess,signal,time\nfrom pathlib import Path\nsubprocess.Popen({descendant!r})\nwhile not Path({str(pid_path)!r}).exists(): time.sleep(0.01)\nPath({str(inner)!r}).write_text('{{}}')\n"
    if mode == "timeout": body += "signal.signal(signal.SIGTERM,lambda *args:exit(0))\ntime.sleep(60)\n"
    _, _, code, terminal = run_child(tmp_path, body, wall_seconds=1 if mode == "timeout" else 10, grace_seconds=0.15, cleanup_seconds=0.15)
    assert code == (124 if mode == "timeout" else 2)
    assert terminal["cleanup"]["forced"] and not terminal["eligible"]
    assert any(item == {"signal": "SIGKILL", "sent": True} for item in terminal["cleanup"]["signals"])
    if mode == "ordinary-exit": assert terminal["actual_exit"] == 0 and not terminal["timed_out"]
    pid = int(pid_path.read_text())
    stat = Path(f"/proc/{pid}/stat")
    if stat.exists():
        assert stat.read_text().split(") ", 1)[1].split()[0] == "Z"
        assert not terminal["cleanup"]["complete"]
    count = heartbeat.stat().st_size
    time.sleep(0.1)
    assert heartbeat.stat().st_size == count


def test_source_instability_is_ineligible_without_editing_source(tmp_path, monkeypatch):
    original = runner.pins
    calls = {"count": 0}
    def changed(paths):
        observed = original(paths)
        calls["count"] += 1
        if calls["count"] == 2: observed[str(Path(runner.__file__).resolve())] = "0" * 64
        return observed
    monkeypatch.setattr(runner, "pins", changed)
    _, _, code, terminal = run_child(tmp_path)
    assert code == 2 and not terminal["source_runtime_stable"] and not terminal["eligible"]


def test_cli_uses_actual_short_process_and_hides_cleanup_options(tmp_path):
    inner = tmp_path / "inner.json"
    command = child(tmp_path, f"from pathlib import Path\nPath({str(inner)!r}).write_text('{{}}')\n")
    argv = [sys.executable, "-B", "-X", f"pycache_prefix={tmp_path / 'wrapper-cache'}", runner.__file__,
            "--receipt-dir", str(tmp_path / "capture"), "--inner-receipt", str(inner), "--wall-seconds", "10", "--", *command]
    (tmp_path / "parent-argv.json").write_text(json.dumps(argv))
    with (tmp_path / "parent.stdout").open("wb") as out, (tmp_path / "parent.stderr").open("wb") as err:
        result = subprocess.run(argv, stdout=out, stderr=err, check=False)
    assert result.returncode == 0
    terminal = json.loads((tmp_path / "capture/terminal.json").read_text())
    assert terminal["eligible"]
    with pytest.raises(SystemExit) as error: runner.main(["--grace-seconds", "0.1"])
    assert error.value.code == 2
