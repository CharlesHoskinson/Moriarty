#!/usr/bin/env python3
"""Immutable, finite offline pilot receipts. No service/listener or retry loop."""
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import signal
import shutil
import re
import subprocess
import sys
import tarfile
import time

ROOT = Path(__file__).resolve().parents[1]
MODEL = ROOT / "specs/quint/s02/factored_verification"
RECEIPTS = ROOT / ".superpowers/sdd/a5-factoring-receipts"
QUINT = Path("/home/charl/.npm-global/bin/quint")
RUST = Path("/home/charl/.quint/rust-evaluator-v0.6.0/quint_evaluator")
AP = Path("/home/charl/.quint/apalache-dist-0.56.1/apalache/bin/apalache-mc")
JAR = AP.parent.parent / "lib/apalache.jar"
JAVA = Path(subprocess.check_output(["which", "java"], text=True).strip()).resolve()
PINNED = {
 str(QUINT.resolve()): "ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501",
 str(RUST): "b2efdeac5713d153e41bf2143b94ed75d888fdd5637f4a5d61a04c695313510a",
 str(AP): "bda52d2dbdbc7f6e95289a69dfe7ddeb162493ddd3501898d33ea7d1da3a8cd7",
 str(JAR): "4753c0ebb2cbb266e2c6ac19ab5ca3827d726cc80fd1fc5d7c1eeb64736cd60b",
}
WITNESSES = ("prepared", "signed", "proposed", "verified", "committed", "rejected",
             "afterCompleted", "beforeCompleted")

def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


NODE = Path(shutil.which("node")).resolve()
QUINT_PACKAGE = QUINT.resolve().parents[2]
PYTHON_MANIFEST = ROOT / ".superpowers/sdd/a4-checker-task1-receipts/python-environment.json"
PYTHON_ARCHIVE = PYTHON_MANIFEST.with_suffix(".tar.gz")
PYTHON_MANIFEST_SHA = "cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4"
PYTHON_ARCHIVE_SHA = "7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac"

def controlled_environment(heap):
    env = dict(os.environ)
    for key in ("NODE_PATH", "NODE_COMPILE_CACHE", "PYTHONPATH", "JAVA_TOOL_OPTIONS", "_JAVA_OPTIONS",
                "JDK_JAVA_OPTIONS", "LD_PRELOAD", "LD_LIBRARY_PATH"):
        env.pop(key, None)
    env.update(PATH=os.pathsep.join((str(JAVA.parent), str(NODE.parent), "/usr/bin", "/bin")),
        APALACHE_JAR=str(JAR), JVM_ARGS="-Xmx" + str(heap) + "m",
        JVM_GC_ARGS="-XX:+UseG1GC -XX:G1PeriodicGCInterval=600000 -XX:+G1PeriodicGCInvokesConcurrent",
        NODE_OPTIONS="--max-old-space-size=4096", NODE_DISABLE_COMPILE_CACHE="1",
        PYTHONNOUSERSITE="1", PYTHONDONTWRITEBYTECODE="1")
    return env

def verify_python_snapshot():
    assert sha(PYTHON_MANIFEST) == PYTHON_MANIFEST_SHA
    assert sha(PYTHON_ARCHIVE) == PYTHON_ARCHIVE_SHA
    manifest = json.loads(PYTHON_MANIFEST.read_text())
    assert manifest["archiveSha256"] == PYTHON_ARCHIVE_SHA
    assert Path(manifest["pythonResolved"]) == Path(sys.executable).resolve()
    records = manifest["sourceBytes"]
    expected = {r["archivePath"]: r for r in records}
    assert len(expected) == len(records)
    with tarfile.open(PYTHON_ARCHIVE, "r:gz") as archive:
        actual = {m.name: m for m in archive.getmembers() if m.isfile()}
        assert actual.keys() == expected.keys()
        for name, item in expected.items():
            assert hashlib.sha256(archive.extractfile(actual[name]).read()).hexdigest() == item["sha256"]
            assert sha(item["path"]) == item["sha256"]
    return records

def bootstrap(before_dispatch_base):
    assert re.fullmatch(r"[0-9a-f]{40}", before_dispatch_base)
    assert subprocess.check_output(["git","rev-parse","HEAD"], cwd=ROOT, text=True).strip() == before_dispatch_base
    for path, wanted in PINNED.items():
        assert sha(path) == wanted, "Tool pin mismatch: " + path
    store = RECEIPTS / "tool-store"
    store.mkdir(parents=True, exist_ok=False)
    python_records = verify_python_snapshot()
    paths = {NODE, QUINT.resolve(), RUST, AP, JAR, JAVA, Path(sys.executable).resolve(), Path("/usr/bin/time")}
    paths.update(Path(shutil.which(n)).resolve() for n in ("bash","env","dirname","readlink","mkdir","mktemp","ldd"))
    java_root = JAVA.parent.parent
    assert (java_root/"lib/modules").is_file() and (QUINT_PACKAGE/"package.json").is_file()
    # Conservative installed dependency closure, not unrelated global npm/JDK packages.
    tree_roots = (QUINT_PACKAGE/"dist", QUINT_PACKAGE/"node_modules", java_root/"lib", java_root/"conf")
    tree_members = {}
    for root in tree_roots:
        tree_members[str(root)] = []
        for p in root.rglob("*"):
            assert not(p.is_symlink() and p.is_dir()), "Review external directory dependency: " + str(p)
            if p.is_file():
                paths.add(p.resolve())
                tree_members[str(root)].append(str(p.resolve()))
        tree_members[str(root)] = sorted(set(tree_members[str(root)]))
    paths.update((QUINT_PACKAGE/"package.json", java_root/"release"))
    native = paths.union(Path(r["path"]) for r in python_records if r["path"].endswith(".so"))
    ldd_receipts = []
    for p in sorted(native):
        with p.open("rb") as stream:
            elf = stream.read(4) == b"\x7fELF"
        if not elf:
            continue
        result = subprocess.run(["ldd",str(p)], capture_output=True, timeout=30,
                                env=controlled_environment(4096))
        ldd_receipts.append({"argv":["ldd",str(p)],"exitCode":result.returncode,
            "stdout":result.stdout.decode(errors="replace"),"stderr":result.stderr.decode(errors="replace")})
        for path in re.findall(r"(?:=>\s*)?(/[^\s()]+)", result.stdout.decode(errors="replace")):
            dependency = Path(path)
            if dependency.is_file():
                paths.add(dependency.resolve())
    records = [{"path":str(p), "archivePath":str(p).lstrip("/"), "sha256":sha(p)} for p in sorted(paths)]
    with tarfile.open(store/"runtime.tar.gz","x:gz",dereference=True) as archive:
        for item in records:
            archive.add(item["path"],arcname=item["archivePath"],recursive=False)
    with tarfile.open(store/"runtime.tar.gz","r:gz") as archive:
        members = archive.getmembers()
        assert all(m.isfile() for m in members)
        expected = {item["archivePath"]:item for item in records}
        assert {m.name for m in members} == expected.keys()
        for member in members:
            assert hashlib.sha256(archive.extractfile(member).read()).hexdigest() == expected[member.name]["sha256"]
    versions = []
    for name,argv in (
        ("node",[str(NODE),"--version"]),
        ("quint",[str(NODE),str(QUINT.resolve()),"--version"]),
        ("java",[str(JAVA),"-version"]),
        ("python",[sys.executable,"--version"]),
        ("rust",[str(RUST),"--version"]),
        ("libc",["ldd","--version"])):
        result = subprocess.run(argv,capture_output=True,timeout=30,env=controlled_environment(4096))
        (store/(name+".stdout")).write_bytes(result.stdout)
        (store/(name+".stderr")).write_bytes(result.stderr)
        assert result.returncode == (1 if name == "rust" else 0), "Unexpected version command result: "+name
        versions.append({"name":name,"argv":argv,"exitCode":result.returncode,
            "stdoutSha256":sha(store/(name+".stdout")),"stderrSha256":sha(store/(name+".stderr"))})
    manifest = {"files":records,"treeMembers":tree_members,"archiveSha256":sha(store/"runtime.tar.gz"),
        "reusedPythonManifest":str(PYTHON_MANIFEST),"reusedPythonManifestSha256":PYTHON_MANIFEST_SHA,
        "reusedPythonArchive":str(PYTHON_ARCHIVE),"reusedPythonArchiveSha256":PYTHON_ARCHIVE_SHA,
        "pythonFiles":python_records,"versions":versions,"dynamicLibraryReceipts":ldd_receipts,
        "quintPackage":json.loads((QUINT_PACKAGE/"package.json").read_text()),
        "javaRelease":(java_root/"release").read_text(),
        "scope":"actual launchers, installed Quint dependency tree, Java runtime, native library closure; reused admitted Python source/native closure; not an OS image"}
    (store/"manifest.json").write_text(json.dumps(manifest,indent=2)+"\n")
    dispatch={"beforeDispatchCommit":before_dispatch_base,"runtimeManifestSha256":sha(store/"manifest.json"),
        "runtimeArchiveSha256":sha(store/"runtime.tar.gz")}
    (RECEIPTS/"dispatch.json").write_text(json.dumps(dispatch,indent=2)+"\n")
    verify_runtime(before_dispatch_base)
    print(json.dumps(dispatch),flush=True)

def dispatch_base():
    return json.loads((RECEIPTS/"dispatch.json").read_text())["beforeDispatchCommit"]

def verify_runtime(before_dispatch_base):
    dispatch=json.loads((RECEIPTS/"dispatch.json").read_text())
    assert before_dispatch_base == dispatch["beforeDispatchCommit"]
    store=RECEIPTS/"tool-store"
    assert sha(store/"manifest.json") == dispatch["runtimeManifestSha256"]
    manifest=json.loads((store/"manifest.json").read_text())
    for root, expected in manifest["treeMembers"].items():
        assert sorted({str(p.resolve()) for p in Path(root).rglob("*") if p.is_file()}) == expected
    assert sha(store/"runtime.tar.gz") == manifest["archiveSha256"] == dispatch["runtimeArchiveSha256"]
    assert sha(PYTHON_MANIFEST) == PYTHON_MANIFEST_SHA and sha(PYTHON_ARCHIVE) == PYTHON_ARCHIVE_SHA
    for item in manifest["files"] + manifest["pythonFiles"]:
        assert sha(item["path"]) == item["sha256"], "Runtime moved: "+item["path"]
    assert NODE == Path(shutil.which("node")).resolve() and JAVA == Path(shutil.which("java")).resolve()
    for item in manifest["versions"]:
        assert sha(store/(item["name"]+".stdout")) == item["stdoutSha256"]
        assert sha(store/(item["name"]+".stderr")) == item["stderrSha256"]
    return {"dispatchSha256":sha(RECEIPTS/"dispatch.json"), **dispatch}

def record(stage, argv, wall, heap, property_name, depth, compile_output=False, extra=(), *, before_dispatch_base, domain="two-profile actual Alice funding and rebound forgery pilot"):
    runtime_before = verify_runtime(before_dispatch_base)
    requested_argv = list(argv)
    if argv[0] == str(QUINT):
        argv = [str(NODE), str(QUINT.resolve()), *argv[1:]]
    out = RECEIPTS / stage
    out.mkdir(parents=True, exist_ok=False)
    derivation = ROOT / "scripts/derive_s02_candidate_a_factored.py"
    spec = importlib.util.spec_from_file_location("a5_derivation", derivation)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    base = ROOT / "specs/quint/s02"
    sources = [base / (name + ".qnt") for name in module.PINNED_ORIGINALS]
    sources += [base / (name + ".qnt") for name in
                ("effects", "consumption", "observations", "policies", "authorization", "execution")]
    sources += sorted(MODEL.glob("*.qnt"))
    sources += [Path(__file__).resolve(), derivation]
    if (MODEL / "derivation.json").exists():
        sources.append(MODEL / "derivation.json")
    for name, wanted in module.PINNED_ORIGINALS.items():
        assert sha(base / (name + ".qnt")) == wanted, "Original pin mismatch: " + name
    tools = [NODE, QUINT.resolve(), RUST, AP, JAR, JAVA, Path(sys.executable).resolve(), Path("/usr/bin/time")]
    for path, wanted in PINNED.items():
        assert sha(path) == wanted, "Tool pin mismatch: " + path
    pins = [{"path": str(p), "sha256": sha(p)} for p in sources + tools + list(extra)]
    with tarfile.open(out / "source-and-runner.tar.gz", "x:gz") as archive:
        for path in sources + list(extra):
            archive.add(path, arcname=str(path.relative_to(ROOT)), recursive=False)
    metadata = {"argv": argv, "requestedArgv": requested_argv, "beforeDispatchCommit": before_dispatch_base,
      "runtimeSnapshot": runtime_before, "cwd": str(ROOT), "property": property_name, "depth": depth,
      "sourceCommit": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip(),
      "pins": pins, "sourceArchiveSha256": sha(out / "source-and-runner.tar.gz"),
      "domains": domain,
      "fairness": "none", "deadlock": "checker deadlock checking disabled by --no-deadlock; pilotSafety requires ready at every nonterminal and disabled step at terminal",
      "limits": {"wallSeconds": wall, "jvmHeapMiB": heap, "nodeHeapMiB": 4096},
      "toolVersions": {"quint": "0.32.0 (pinned earlier raw version receipt)",
        "rust": "v0.6.0 directory label; --version unsupported",
        "apalache": "0.56.1/build70cdaf4 pinned jar"},
      "classification": "pilot only; not full Candidate A A5"}
    (out / "input.json").write_text(json.dumps(metadata, indent=2) + "\n")
    with (out / "java-version.stdout").open("xb") as stdout, (out / "java-version.stderr").open("xb") as stderr:
        version = subprocess.run([str(JAVA), "-version"], stdout=stdout, stderr=stderr, timeout=10)
    assert version.returncode == 0
    output = out / ("input.qnt.json" if compile_output else "stdout.txt")
    wrapped = ["/usr/bin/time", "-v", "-o", str(out / "resources.txt"), *argv]
    env = controlled_environment(heap)
    env["PYTHONPYCACHEPREFIX"] = str(out/"fresh-python-cache")
    started = time.monotonic()
    timed_out = False
    with output.open("xb") as stdout, (out / "stderr.txt").open("xb") as stderr:
        proc = subprocess.Popen(wrapped, cwd=ROOT, env=env, stdout=stdout, stderr=stderr, start_new_session=True)
        try:
            exit_code = proc.wait(timeout=wall)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(proc.pid, signal.SIGTERM)
            try:
                exit_code = proc.wait(timeout=5)
            except subprocess.TimeoutExpired:
                os.killpg(proc.pid, signal.SIGKILL)
                exit_code = proc.wait()
    try:
        runtime_after = verify_runtime(before_dispatch_base)
        runtime_unchanged = runtime_after == runtime_before
    except (AssertionError, OSError):
        runtime_unchanged = False
    result = {"exitCode": exit_code, "timedOut": timed_out, "durationSeconds": time.monotonic()-started,
      "stdoutSha256": sha(output), "stderrSha256": sha(out/"stderr.txt"),
      "runtimeUnchanged": runtime_unchanged,
      "sourceAndToolsUnchanged": runtime_unchanged and all(sha(p["path"]) == p["sha256"] for p in pins)}
    if compile_output and exit_code == 0 and not timed_out:
        try:
            document = json.loads(output.read_text())
            counts = {"objects": 0, "letObjects": 0, "appObjects": 0}
            stack = [document]
            while stack:
                item = stack.pop()
                if isinstance(item, dict):
                    counts["objects"] += 1
                    counts["letObjects"] += item.get("kind") == "let"
                    counts["appObjects"] += item.get("kind") == "app"
                    stack.extend(item.values())
                elif isinstance(item, list):
                    stack.extend(item)
            result["generated"] = {"bytes": output.stat().st_size, **counts}
        except (ValueError, MemoryError) as error:
            result["measurementError"] = type(error).__name__
    (out / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"stage": stage, **result}), flush=True)
    if not result["sourceAndToolsUnchanged"]:
        raise SystemExit("Source/tool movement invalidates stage")
    return 124 if timed_out else exit_code

def main():
    if len(sys.argv) == 3 and sys.argv[1] == "bootstrap":
        bootstrap(sys.argv[2])
        return
    mode, variant, prop, before_dispatch_base = sys.argv[1:5]
    def run(*args, **kwargs):
        return record(*args, before_dispatch_base=before_dispatch_base, **kwargs)
    assert variant in ("original", "factored")
    stem = "candidate_a_funding_pilot" + ("_f" if variant == "factored" else "")
    entry = MODEL / (stem + ".qnt")
    stage = mode + "-" + variant + "-" + prop
    if mode == "compile":
        assert prop in ("pilotSafety", "neverPrepared")
        code = run(stage, [str(QUINT), "compile", str(entry), "--main="+stem, "--target=json",
          "--invariant="+prop, "--verbosity=0"], 900, 4096, prop, 5, True)
    elif mode == "check":
        assert prop in ("pilotSafety", "neverPrepared")
        compiled = RECEIPTS / ("compile-"+variant+"-"+prop)
        previous = json.loads((compiled / "result.json").read_text())
        assert previous["exitCode"] == 0 and not previous["timedOut"] and previous["sourceAndToolsUnchanged"]
        assert "measurementError" not in previous
        compiled_metadata = json.loads((compiled / "input.json").read_text())
        assert all(sha(p["path"]) == p["sha256"] for p in compiled_metadata["pins"])
        inp = compiled / "input.qnt.json"
        assert sha(inp) == previous["stdoutSha256"]
        code = run(stage, [str(AP), "--out-dir="+str(RECEIPTS/stage/"apalache"), "check",
          "--init=q::init", "--next=q::step", "--inv=q::inv", "--length=5", "--no-deadlock", str(inp)],
          600, 4096, prop, 5, extra=(inp,))
    elif mode == "sample":
        assert prop == "pilotSafety"
        code = run(stage, [str(QUINT), "run", str(entry), "--backend=rust", "--seed=42",
          "--max-samples=100", "--max-steps=5", "--invariant=pilotSafety", "--witnesses", *WITNESSES,
          "--verbosity=1"], 1200, 4096, prop, 5)
    else:
        raise SystemExit("Unsupported stage; no implicit retry")
    raise SystemExit(code)

if __name__ == "__main__":
    main()
