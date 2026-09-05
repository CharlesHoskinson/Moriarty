from __future__ import annotations
import argparse
import hashlib
import importlib.util
import json
import os
import posixpath
import re
import shutil
import subprocess
import sys
import time
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[1]
if __package__ in (None, ""):
    sys.path.insert(0, str(ROOT))
from scripts.a4_json_stream import hash_stream

STAGES = ROOT / ".superpowers/sdd/a4-producer-receipts"
DISPATCH = ROOT / ".superpowers/sdd/a4-producer-dispatch.json"
SHARED_DISPATCH = ROOT / ".superpowers/sdd/a5-factoring-receipts/dispatch.json"
TOOL_STORE = ROOT / ".superpowers/sdd/a5-factoring-receipts/tool-store"
RUNTIME_HELPER = ROOT / "scripts/run_s02_candidate_a_factoring_pilot.py"
PYTHON_MANIFEST = ROOT / ".superpowers/sdd/a4-checker-task1-receipts/python-environment.json"
PYTHON_ARCHIVE = PYTHON_MANIFEST.with_suffix(".tar.gz")
PYTHON_MANIFEST_SHA = "cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4"
PYTHON_ARCHIVE_SHA = "7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac"
ROOTS = tuple(f"specs/quint/s02/candidate_a_integrated_case_{index:03d}.qnt" for index in range(78)) + (
    "specs/quint/s02/candidate_a_integrated_wrappers_typecheck.qnt",
    "specs/quint/s02/candidate_a_integrated_export_test.qnt",
    "specs/quint/s02/candidate_a_authority_installment_test.qnt",
    "specs/quint/s02/candidate_a_authority_swap_test.qnt",
)
PYTHON = (
    "moriarty/__init__.py", "moriarty/core.py", "moriarty/swap.py", "scripts/check_s02_candidate_a_correspondence.py",
    "scripts/a4_carrier.py", "scripts/a4_agreement.py", "scripts/a4_authority.py", "scripts/a4_cases.py",
    "scripts/a4_inventory.py", "scripts/check_s02_candidate_a_integrated.py",
    "scripts/s02_candidate_a_integrated_inventory.py", "scripts/export_s02_candidate_a_integrated.py",
    "scripts/record_s02_candidate_a_integrated.py", "tests/test_s02_candidate_a_integrated.py",
    "tests/test_s02_candidate_a_integrated_export.py",
    "scripts/a4_json_stream.py", "tests/test_a4_json_stream.py",
)
IMPORTS = re.compile(r'^\s*import\s+[^\n]*?\s+from\s+"([^"]+)"', re.MULTILINE)

def sha(data): return hashlib.sha256(data).hexdigest()

def artifact_hash(path):
    with path.open("rb") as stream:
        return hash_stream(stream)[0]

def write_json(path, value):
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, sort_keys=True, indent=2)
        stream.write("\n")

def snapshot(destination):
    pending = [name for name in ROOTS if (ROOT / name).is_file()]
    paths = {name for name in PYTHON if (ROOT / name).is_file()}
    missing = [name for name in (*ROOTS, *PYTHON) if not (ROOT / name).is_file()]
    while pending:
        name = pending.pop()
        if name in paths: continue
        paths.add(name)
        path = ROOT / name
        if path.is_symlink(): raise ValueError("source symlink")
        source = path.read_text(encoding="utf-8")
        for imported in IMPORTS.findall(source):
            if not imported.startswith("./"): raise ValueError("nonlocal source import")
            target = posixpath.normpath(str(PurePosixPath(name).parent / (imported + ".qnt")))
            if target.startswith("../"): raise ValueError("source escape")
            if not (ROOT / target).is_file(): raise ValueError(f"missing actual import {target}")
            pending.append(target)
    hashes = {}
    for name in sorted(paths):
        source = ROOT / name
        if source.is_symlink(): raise ValueError("source symlink")
        data = source.read_bytes()
        target = destination / "source" / name
        target.parent.mkdir(parents=True, exist_ok=True)
        with target.open("xb") as stream: stream.write(data)
        hashes[name] = sha(data)
    write_json(destination / "closure.json", {"sources": hashes, "not_yet_created": missing})
    return hashes, missing

def runtime_expected():
    dispatch = json.loads(DISPATCH.read_text())
    if set(dispatch) != {"beforeDispatchCommit", "sharedDispatchSha256", "runtimeManifestSha256", "runtimeArchiveSha256", "runtimeHelperSha256"}:
        raise ValueError("producer dispatch fields")
    if not re.fullmatch(r"[0-9a-f]{40}", dispatch["beforeDispatchCommit"]):
        raise ValueError("explicit producer pre-dispatch base missing")
    shared = json.loads(SHARED_DISPATCH.read_text())
    if sha(SHARED_DISPATCH.read_bytes()) != dispatch["sharedDispatchSha256"]:
        raise ValueError("unadmitted shared dispatch")
    if shared["runtimeManifestSha256"] != dispatch["runtimeManifestSha256"] or shared["runtimeArchiveSha256"] != dispatch["runtimeArchiveSha256"]:
        raise ValueError("shared runtime admission mismatch")
    if sha(RUNTIME_HELPER.read_bytes()) != dispatch["runtimeHelperSha256"]:
        raise ValueError("shared runtime verifier source pin")
    spec = importlib.util.spec_from_file_location("a4_shared_runtime", RUNTIME_HELPER)
    helper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(helper)
    verification = helper.verify_runtime(shared["beforeDispatchCommit"])
    manifest_path = TOOL_STORE / "manifest.json"
    if sha(manifest_path.read_bytes()) != dispatch["runtimeManifestSha256"]:
        raise ValueError("runtime manifest pin")
    manifest = json.loads(manifest_path.read_text())
    if manifest["archiveSha256"] != dispatch["runtimeArchiveSha256"]:
        raise ValueError("runtime archive declaration")
    if (manifest["reusedPythonManifest"] != str(PYTHON_MANIFEST)
        or manifest["reusedPythonArchive"] != str(PYTHON_ARCHIVE)
        or manifest["reusedPythonManifestSha256"] != PYTHON_MANIFEST_SHA
        or manifest["reusedPythonArchiveSha256"] != PYTHON_ARCHIVE_SHA):
        raise ValueError("unadmitted Python archive")
    python = json.loads(PYTHON_MANIFEST.read_text())
    if python["archiveSha256"] != PYTHON_ARCHIVE_SHA or python["sourceBytes"] != manifest["pythonFiles"]:
        raise ValueError("Python source inventory")
    if Path(python["pythonResolved"]) != Path(sys.executable).resolve():
        raise ValueError("wrong Python interpreter")
    pins = {str(DISPATCH): sha(DISPATCH.read_bytes()), str(SHARED_DISPATCH): dispatch["sharedDispatchSha256"],
            str(RUNTIME_HELPER): dispatch["runtimeHelperSha256"],
            str(manifest_path): dispatch["runtimeManifestSha256"],
            str(TOOL_STORE / "runtime.tar.gz"): dispatch["runtimeArchiveSha256"],
            str(PYTHON_MANIFEST): PYTHON_MANIFEST_SHA, str(PYTHON_ARCHIVE): PYTHON_ARCHIVE_SHA}
    for item in manifest["files"] + manifest["pythonFiles"]:
        if item["path"] in pins and pins[item["path"]] != item["sha256"]:
            raise ValueError("conflicting runtime pin")
        pins[item["path"]] = item["sha256"]
    for version in manifest["versions"]:
        for suffix in ("stdout", "stderr"):
            pins[str(TOOL_STORE / (version["name"] + "." + suffix))] = version[suffix + "Sha256"]
    return dispatch, shared, manifest, pins, helper, verification

def runtime_observed(manifest, expected):
    files = {}
    for name in expected:
        try: files[name] = sha(Path(name).read_bytes())
        except OSError: files[name] = None
    trees = {}
    for name in manifest["treeMembers"]:
        directory = Path(name)
        members = []
        if directory.is_dir():
            for path in directory.rglob("*"):
                if path.is_symlink() and path.is_dir():
                    members.append("UNADMITTED_DIRECTORY_LINK:" + str(path))
                elif path.is_file():
                    members.append(str(path.resolve()))
        trees[name] = sorted(set(members))
    return {"files": files, "treeMembers": trees}

def controlled_environment(cache):
    env = dict(os.environ)
    for name in ("PYTHONPATH", "PYTHONHOME", "PYTHONSTARTUP", "PYTHONINSPECT", "PYTEST_ADDOPTS", "PYTEST_PLUGINS",
                 "NODE_PATH", "NODE_OPTIONS", "NODE_COMPILE_CACHE", "LD_PRELOAD", "LD_LIBRARY_PATH"):
        env.pop(name, None)
    env.update(PYTHONNOUSERSITE="1", PYTHONDONTWRITEBYTECODE="1", PYTHONPYCACHEPREFIX=str(cache),
               NODE_DISABLE_COMPILE_CACHE="1")
    return env

def tool(name, path, version_command, env):
    actual = Path(path).resolve(strict=True)
    result = subprocess.run(version_command, cwd=ROOT, capture_output=True, check=False, env=env)
    return {"name": name, "path": str(actual), "sha256": sha(actual.read_bytes()),
            "version_command": version_command, "version_exit": result.returncode,
            "version_stdout": result.stdout.decode("utf-8", errors="replace"),
            "version_stderr": result.stderr.decode("utf-8", errors="replace")}

def main(argv=None):
    parser = argparse.ArgumentParser()
    parser.add_argument("--stage", required=True)
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args(argv)
    if not re.fullmatch(r"[a-z0-9][a-z0-9-]*", args.stage): raise ValueError("unsafe stage")
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not command: raise ValueError("empty command")
    stage = STAGES / args.stage
    cache = stage / "python-cache"
    if not sys.dont_write_bytecode or not sys.pycache_prefix or Path(sys.pycache_prefix).resolve() != cache:
        raise ValueError("launch recorder with -B and the exact fresh per-stage -X pycache_prefix")
    if stage.exists(): raise ValueError("stage/cache already exists")
    stage.mkdir(parents=True, exist_ok=False)
    cache.mkdir()
    env = controlled_environment(cache)
    dispatch, shared, runtime, runtime_pins, helper, shared_before = runtime_expected()
    with (stage / "runtime-helper.py").open("xb") as stream:
        stream.write(RUNTIME_HELPER.read_bytes())
    runtime_before = runtime_observed(runtime, runtime_pins)
    if runtime_before != {"files": runtime_pins, "treeMembers": runtime["treeMembers"]}:
        raise ValueError("runtime differs from root-admitted archive before command")
    before, missing_before = snapshot(stage / "before")
    quint = shutil.which("quint")
    if quint is None: raise ValueError("Quint missing")
    node = Path(shutil.which("node")).resolve()
    cli = Path(quint).resolve()
    rust = Path("/home/charl/.quint/rust-evaluator-v0.6.0/quint_evaluator")
    for path in (node, cli, rust, Path(sys.executable).resolve()):
        if str(path) not in runtime_pins: raise ValueError("unadmitted actual executable")
    tools = {"node": tool("node", node, [str(node), "--version"], env),
             "quint": tool("quint", cli, [str(node), str(cli), "--version"], env),
             "rust": tool("rust", rust, [str(rust), "--version"], env),
             "python": tool("python", sys.executable, [sys.executable, "--version"], env)}
    actual_command = [str(node), str(cli), *command[1:]] if command[0] == "quint" else command
    source_commit = subprocess.run(["git", "rev-parse", "HEAD"], cwd=ROOT, capture_output=True, check=True).stdout.decode().strip()
    dirty = subprocess.run(["git", "status", "--short"], cwd=ROOT, capture_output=True, check=True).stdout.decode()
    started = time.time_ns()
    with (stage / "stdout.bin").open("xb") as stdout, (stage / "stderr.bin").open("xb") as stderr:
        completed = subprocess.run(actual_command, cwd=ROOT, stdout=stdout, stderr=stderr, check=False, env=env)
    ended = time.time_ns()
    after, missing_after = snapshot(stage / "after")
    runtime_after = runtime_observed(runtime, runtime_pins)
    try:
        shared_after = helper.verify_runtime(shared["beforeDispatchCommit"])
    except (AssertionError, OSError, ValueError) as error:
        shared_after = {"verificationError": str(error)}
    runtime_stable = runtime_before == runtime_after and shared_before == shared_after
    source_stable = before == after and missing_before == missing_after
    recorder_exit = (completed.returncode if completed.returncode >= 0 else 128 - completed.returncode)
    if not source_stable or not runtime_stable: recorder_exit = 2
    artifacts = {path.name: artifact_hash(path) for path in sorted(stage.glob("*.itf.json"))}
    receipt = {"command": command, "executed_command": actual_command, "cwd": str(ROOT),
        "beforeDispatchCommit": dispatch["beforeDispatchCommit"], "sourceCommit": source_commit, "dirty_before": dirty,
        "sourceCommitAfter": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
        "started_ns": started, "ended_ns": ended, "elapsed_ns": ended - started,
        "exit_code": completed.returncode, "sources_before": before, "sources_after": after,
        "source_stable": source_stable, "runtime_stable": runtime_stable, "recorder_exit_code": recorder_exit,
        "runtime_before": runtime_before, "runtime_after": runtime_after,
        "shared_runtime_before": shared_before, "shared_runtime_after": shared_after,
        "runtime_manifest_sha256": dispatch["runtimeManifestSha256"], "runtime_archive_sha256": dispatch["runtimeArchiveSha256"],
        "python_cache_prefix": str(cache), "python_bytecode_writes": False,
        "not_yet_created_before": missing_before, "not_yet_created_after": missing_after,
        "tools": tools, "artifacts": artifacts,
        "stdout_sha256": sha((stage / "stdout.bin").read_bytes()),
        "stderr_sha256": sha((stage / "stderr.bin").read_bytes())}
    write_json(stage / "receipt.json", receipt)
    print(json.dumps({"stage": str(stage), "exit_code": completed.returncode,
                      "source_stable": source_stable, "runtime_stable": runtime_stable,
                      "recorder_exit_code": recorder_exit, "artifacts": artifacts}))
    return recorder_exit

if __name__ == "__main__":
    raise SystemExit(main())
