"""Local, serialized package upgrades with recovery before return to the host."""
import argparse
from contextlib import contextmanager
import fcntl
import hashlib
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parent.parent))
from moriarty_dev.bootstrap import UNAVAILABLE, runtime_digest, inventory

REQUIRED_MODULES = {"__init__.py", "bootstrap.py", "deployment.py", "hook.py", "hook_protocol.py", "cli.py",
                    "compatibility.py", "policy.py", "records.py", "store.py", "runner.py", "notifications.py"}


@contextmanager
def locked(runtime_home):
    runtime_home.mkdir(parents=True, exist_ok=True)
    with (runtime_home / "upgrade.lock").open("a") as handle:
        fcntl.flock(handle, fcntl.LOCK_EX)
        yield


def files(root):
    result = {}
    for path in sorted(root.rglob("*")):
        if "__pycache__" in path.parts:
            continue
        if path.is_symlink():
            raise ValueError("package symlink: " + str(path))
        if path.is_file():
            result[path.relative_to(root).as_posix()] = hashlib.sha256(path.read_bytes()).hexdigest()
    return result


def copy_atomic(source, destination):
    destination.parent.mkdir(parents=True, exist_ok=True)
    stage = Path(tempfile.mkdtemp(prefix=".stage-", dir=destination.parent))
    try:
        shutil.copytree(source, stage / "package", ignore=shutil.ignore_patterns("__pycache__"))
        if files(source) != files(stage / "package"):
            raise ValueError("copy changed during staging")
        os.rename(stage / "package", destination)
    finally:
        shutil.rmtree(stage)


def definition(plugin, digest):
    bootstrap = (plugin / "scripts/moriarty_dev/bootstrap.py").read_text()
    fallback = shlex.quote(json.dumps(UNAVAILABLE, separators=(",", ":")))
    hooks = {}
    for event in ("SessionStart", "PreToolUse", "PostToolUse", "Stop"):
        command = "if command -v python3 >/dev/null 2>&1; then exec python3 -I -c " + shlex.quote(bootstrap)
        command += " " + digest + " " + event + "; else printf '%s\\n' " + fallback + "; fi"
        hooks[event] = [{"matcher": "startup|resume|clear|compact" if event == "SessionStart" else ".*",
                         "hooks": [{"type": "command", "command": command, "timeout": 1}]}]
    return {"description": "Repository workflow hooks with a pinned runtime independent of cache retention.", "hooks": hooks}


def stage_runtime(plugin, runtime_home, digest):
    target = runtime_home / "runtimes" / digest
    if not target.exists():
        copy_atomic(plugin, target)
    if runtime_digest(target) != digest:
        raise ValueError("retained runtime conflict")


def validate_source(plugin):
    rows = inventory(plugin)
    if not REQUIRED_MODULES.issubset(rows):
        raise ValueError("source runtime incomplete")
    for name in rows:
        path = plugin / "scripts/moriarty_dev" / name
        try:
            compile(path.read_bytes(), str(path), "exec")
        except SyntaxError as error:
            raise ValueError("source syntax invalid: " + name) from error
    generated = json.loads((plugin / "hooks/hooks.json").read_text())
    with tempfile.TemporaryDirectory(prefix="moriarty-source-check-") as directory:
        env = {**os.environ, "HOME": directory, "PLUGIN_ROOT": str(plugin), "CLAUDE_PLUGIN_ROOT": ""}
        for event in ("Stop", "PreToolUse"):
            command = generated["hooks"][event][0]["hooks"][0]["command"]
            result = subprocess.run(["sh", "-c", command], input=json.dumps({"cwd": directory,
                "tool_name":"exec_command", "tool_input":{"cmd":"pwd"}}), env=env, capture_output=True, text=True, timeout=3)
            expected = {} if event == "Stop" else {"hookSpecificOutput":{"hookEventName":event}}
            if result.returncode != 0 or result.stderr or json.loads(result.stdout) != expected:
                raise ValueError("source protocol startup failed")


def retained_roots(codex_home, extra=()):
    roots = list((codex_home / "plugins/cache").glob("*/moriarty-dev/*"))
    legacy = codex_home / "plugins/moriarty-dev"
    if legacy.exists():
        roots.append(legacy)
    roots += [Path(path) for path in extra]
    unique = sorted(set(path.absolute() for path in roots))
    for path in unique:
        if path.is_symlink() or not path.is_dir():
            raise ValueError("retained root unavailable: " + str(path))
    return unique


def prepare(plugin, runtime_home):
    plugin = plugin.resolve()
    with locked(runtime_home):
        digest = runtime_digest(plugin)
        generated = definition(plugin, digest)
        target = plugin / "hooks/hooks.json"
        target.parent.mkdir(parents=True, exist_ok=True)
        temporary = target.with_suffix(".json.tmp")
        temporary.write_text(json.dumps(generated, indent=2) + "\n")
        os.replace(temporary, target)
        stage_runtime(plugin, runtime_home, digest)
        return digest


def restore_snapshot(snapshot, destination, expected):
    if not destination.exists() and not destination.is_symlink():
        copy_atomic(snapshot, destination)
    conflicts = []
    for relative, digest in expected.items():
        target = destination / relative
        parents = [destination, *list(target.parents)[:len(Path(relative).parts) - 1]]
        if any(parent.is_symlink() or (parent.exists() and not parent.is_dir()) for parent in parents):
            conflicts.append(relative)
            continue
        if target.exists() or target.is_symlink():
            if target.is_symlink() or not target.is_file() or hashlib.sha256(target.read_bytes()).hexdigest() != digest:
                conflicts.append(relative)
            continue
        target.parent.mkdir(parents=True, exist_ok=True)
        with tempfile.NamedTemporaryFile(dir=target.parent, prefix=".restore-", delete=False) as stream:
            stage = Path(stream.name)
        try:
            shutil.copy2(snapshot / relative, stage)
            # No overwrites: a concurrently created target is a recovery conflict.
            try:
                os.link(stage, target)
            except FileExistsError:
                conflicts.append(relative)
        finally:
            stage.unlink()
    if conflicts:
        raise RuntimeError("retained cache conflict: " + ", ".join(conflicts))
    actual = {name: hashlib.sha256((destination / name).read_bytes()).hexdigest() for name in expected}
    if any(actual.get(name) != digest for name, digest in expected.items()):
        raise RuntimeError("retained cache restoration incomplete")


def upgrade(plugin, cache_root, backup_root, runtime_home, installer_argv, extra_roots=()):
    plugin = plugin.resolve()
    with locked(runtime_home):
        digest = runtime_digest(plugin)
        if json.loads((plugin / "hooks/hooks.json").read_text()) != definition(plugin, digest):
            raise ValueError("stale hook registration; run prepare first")
        manifest = json.loads((plugin / ".codex-plugin/plugin.json").read_text())
        version = manifest["version"]
        if manifest["name"] != "moriarty-dev" or not re.fullmatch(r"[A-Za-z0-9_.+-]+", version):
            raise ValueError("invalid plugin identity")
        validate_source(plugin)
        expected_source = files(plugin)
        stage_runtime(plugin, runtime_home, digest)
        backup_root.mkdir(parents=True, exist_ok=True)
        backup = Path(tempfile.mkdtemp(prefix="upgrade-", dir=backup_root))
        retained = []
        roots = sorted(set((list(cache_root.iterdir()) if cache_root.exists() else []) + list(extra_roots)))
        for index, old in enumerate(roots):
            if not old.is_dir() or old.is_symlink():
                raise ValueError("invalid retained cache root")
            expected = files(old)
            snapshot = backup / (str(index) + "-" + old.name)
            copy_atomic(old, snapshot)
            if files(snapshot) != expected:
                raise ValueError("snapshot changed before install")
            retained.append((old, snapshot, expected))
        report = {"runtimeDigest": digest, "backup": str(backup), "installerReturnCode": None,
                  "restorationComplete": False, "installedVerified": False,
                  "retained": [{"path": str(old), "snapshot": str(snapshot), "files": expected} for old, snapshot, expected in retained]}
        receipt = backup / "receipt.json"
        receipt.write_text(json.dumps(report, indent=2) + "\n")
        try:
            if files(plugin) != expected_source:
                raise ValueError("source changed before install")
            result = subprocess.run(installer_argv, stdin=subprocess.DEVNULL, capture_output=True, text=True)
            report["installerReturnCode"] = result.returncode
            (backup / "installer-stdout.txt").write_text(result.stdout)
            (backup / "installer-stderr.txt").write_text(result.stderr)
        finally:
            errors = []
            for old, snapshot, expected in retained:
                try:
                    restore_snapshot(snapshot, old, expected)
                except Exception as error:
                    errors.append(str(error))
            report["restorationComplete"] = not errors
            report["restorationErrors"] = errors
            try:
                report["installedVerified"] = files(cache_root / version) == expected_source
            except Exception as error:
                report["installedVerified"] = False
                report["installedVerificationError"] = str(error)
            receipt.write_text(json.dumps(report, indent=2) + "\n")
            if errors:
                raise RuntimeError("recovery conflict or failure: " + "; ".join(errors))
        return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("operation", choices=("prepare", "install"))
    parser.add_argument("--plugin", type=Path, required=True)
    parser.add_argument("--marketplace", default="personal")
    parser.add_argument("--retain-root", type=Path, action="append", default=[])
    args = parser.parse_args()
    runtime_home = Path.home() / ".local/share/moriarty-dev"
    if args.operation == "prepare":
        print(json.dumps({"runtimeDigest": prepare(args.plugin, runtime_home)}))
        return
    if not re.fullmatch(r"[A-Za-z0-9_-]+", args.marketplace):
        parser.error("invalid marketplace name")
    probe = subprocess.run(["codex", "plugin", "list", "--marketplace", args.marketplace, "--json"],
                           stdin=subprocess.DEVNULL, capture_output=True, text=True, check=True)
    catalog = json.loads(probe.stdout)
    matches = [row for key in ("installed", "available") for row in catalog.get(key, [])
               if row.get("pluginId") == "moriarty-dev@" + args.marketplace]
    if not matches or any(row.get("source", {}).get("source") != "local" or
                         Path(row["source"]["path"]).resolve() != args.plugin.resolve() for row in matches):
        parser.error("plugin is not the confirmed local marketplace source")
    codex_home = Path(os.environ.get("CODEX_HOME") or Path.home() / ".codex")
    extra = args.retain_root + [Path(os.environ[key]) for key in ("PLUGIN_ROOT", "CLAUDE_PLUGIN_ROOT") if os.environ.get(key)]
    roots = retained_roots(codex_home, extra)
    report = upgrade(args.plugin, codex_home / "plugins/cache" / args.marketplace / "moriarty-dev",
                     runtime_home / "backups", runtime_home, ["codex", "plugin", "add", "moriarty-dev@" + args.marketplace], roots)
    print(json.dumps(report, indent=2))
    raise SystemExit(report["installerReturnCode"] or (0 if report["installedVerified"] else 1))


if __name__ == "__main__":
    main()
