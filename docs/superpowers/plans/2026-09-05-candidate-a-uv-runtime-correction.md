# A5 original-suite uv runtime correction implementation plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans. Execute only after root adopts this amendment.

**Goal:** Repair executable discovery for the unchanged historical441 suite, preserving the failed original receipt and every later A4/final gate.

**Architecture:** Reuse the admitted Python/runtime and2691-file fixture archives. A separate pinned supplement retains the real uv executable and its seven loaded libraries; a private one-entry bin and a recorded launcher select the already-admitted main Python environment. No shared helper, historical wrapper, tests, or fixture bytes change.

**Tech Stack:** Installed uv0.12.5, CPython3.13.14, existing immutable A5 recorder.

## Global constraints and evidence

The original stage `task2-original-python-regression` remains immutable: exit1,440passed/1failed,65.51s pytest time. The failure is `FileNotFoundError: b'uv'` at the unchanged `test_sprint_evidence_validator_accepts_instruction_manifest`. Its subprocess argv is literally `uv run python scripts/validate_sprint_evidence.py --manifest openspec/work-packages.json --instructions-only`. Recorder PATH excludes `/home/charl/.local/bin`.

Original wrapper SHA256 is `6d0eb0ea76a0baab1d7544cc83c091602e3a0e2ebc45cf5d3b1fd839ebbb213f`; fixture manifest SHA256 is `7e8e5297a333dd91cd5aef8a9aa755355b2941b62cdd4aad185f95a86a5fa04c`; archive SHA256 is `e0bbb3ca31de72834921cce6e30fc225bceeef478be6b5a7bfe7b50d437f240a`. Original fresh441 collection SHA256 is `adb5b3c1e75ee020ef7192ca4aea6cc0a7c5f367133a786db2116792978d52c2`. These are reused, never rewritten.

### EARS / OpenSpec delta

- UV001: When the unchanged test requests uv, the retry SHALL execute retained real uv0.12.5 bytes, not a replacement implementation.
- UV002: While uv runs, the launch SHALL disable dependency sync, network, Python downloads, env-file/user configuration loading, and SHALL select `/home/charl/Moriarty/.venv`, whose interpreter/package bytes are already admitted. It SHALL not select the worktree's distinct .venv.
- UV003: Before each retry command, the launcher SHALL run and retain an offline/no-sync uv child identity probe. A mismatch SHALL stop before pytest/Core.
- UV004: Each command SHALL retain original stdout/stderr, parent and nested requested argv, effective controlled environment, fixed runtime/source archives, before/after byte checks, and terminal status. Supplement validation SHALL check actual private uv copy and exact regular tar membership.
- UV005: Fresh collection SHALL equal the prior441 ordered node IDs. Runtime SHALL pass all441 with no skips/errors and exact node-ID set equality. Core53 SHALL run only after both gates. Any failure stops this sequence.
- UV006: All new A4 tests and the final unrestricted full suite SHALL remain mandatory. This retry does not close Task2 alone, Task3, or A5.

## Ownership and dispatch

Create only ignored files under `.superpowers/sdd/a5-factoring-receipts/`: `a5_uv_runtime.py`, `capture-task2-python-uv.py`, `task2-uv-dispatch.json`, `task2-uv-runtime/`, and fresh stages named below. Do not edit the shared recorder, original capture, runtime tool-store, old manifests/receipts, frozen semantic files, or tests. No commit or native job is authorized by this amendment.

Root records a new full40-hex amendment dispatch commit in `task2-uv-dispatch.json` before execution. It also states actualTask2Base=d824fa3381ecda846be7e8dd027732f1a7f62e91 and runtimeBootstrapBase=900bb2051225b4a3d99bf422c3b2e5e386e3e7bc. The helper's historical beforeDispatchCommit field remains the bootstrap base; sourceCommit remains independent currentHEAD.

## Task1: Add the isolated runtime supplement

- [ ] Create `a5_uv_runtime.py` with the complete block below using apply_patch. This is a genuine executable, not a uv shim. Only the private bin directory contains a copied real uv.

```python
import hashlib
import io
import json
import os
import subprocess
import sys
import tarfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
R = ROOT / ".superpowers/sdd/a5-factoring-receipts"
STORE = R / "task2-uv-runtime"
LAUNCHER = Path(__file__).resolve()
PYTHON = "/home/charl/Moriarty/.venv/bin/python"
PREFIX = "/home/charl/Moriarty/.venv"
RESOLVED = "/home/charl/.local/share/uv/python/cpython-3.13.14-linux-x86_64-gnu/bin/python3.13"
DISPATCH = R / "task2-uv-dispatch.json"
PINS = {
 "/home/charl/.local/bin/uv": "b65f23a420c4acc96427efb30e5ed9bc0f7e25d2d712000f6ede77c1a0de5f46",
 "/usr/lib/x86_64-linux-gnu/libgcc_s.so.1": "9d339ecb409578d6a5d587e6c537a8f9589b8a13fefba30d167433a4b5758bee",
 "/usr/lib/x86_64-linux-gnu/librt.so.1": "5df2508a1ef33bd8024d77271e2a4a2607cb63e59dd753dd46f8e7a2ed44962d",
 "/usr/lib/x86_64-linux-gnu/libpthread.so.0": "f93acb6e78dcf0213c8a85f922d21916249148e24de426079c40b6304c42085d",
 "/usr/lib/x86_64-linux-gnu/libm.so.6": "beea4eeacfcfa2cd96011b959a826c97cf4a774017e214f6a34d7eea3d49cd88",
 "/usr/lib/x86_64-linux-gnu/libdl.so.2": "7d293f8361fcead4f9691561adc0413f724f3607b959abe0d4fb243072956079",
 "/usr/lib/x86_64-linux-gnu/libc.so.6": "a3947513a02831ec692ebf13053c07614882ab54a2101fb91a1b15724062ed0c",
 "/usr/lib/x86_64-linux-gnu/ld-linux-x86-64.so.2": "c5e80a563850d6ab5c2f2482e4202d9c1b71fbf44854b8c399e63527202c64e1",
 "/home/charl/Moriarty/.venv/pyvenv.cfg": "a5da72403fdf1e84b671e943656baef2770bb8c9f16bdcfe04c323582ad61451",
}
def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def write(p, value):
    with p.open("x") as stream:
        json.dump(value, stream, indent=2)
        stream.write("\n")

def checks():
    assert all(sha(p) == h for p,h in PINS.items())
    assert str(Path(PYTHON).resolve()) == RESOLVED
    assert sha(STORE/"bin/uv") == PINS["/home/charl/.local/bin/uv"]
    assert sorted(p.name for p in (STORE/"bin").iterdir()) == ["uv"]
    m = json.loads((STORE/"manifest.json").read_text())
    assert sha(STORE/"runtime.tar.gz") == m["archiveSha256"]
    expected = {e["archivePath"]:e for e in m["files"]}
    assert len(expected) == len(m["files"])
    with tarfile.open(STORE/"runtime.tar.gz", "r:gz") as archive:
        members = archive.getmembers()
        assert len(members) == len(expected)
        assert {a.name for a in members} == expected.keys()
        for member in members:
            assert member.isfile()
            e = expected[member.name]
            assert hashlib.sha256(archive.extractfile(member).read()).hexdigest() == e["sha256"]
            assert sha(e["path"]) == e["sha256"]
    return sha(STORE/"manifest.json")

def prepare():
    sys.path.insert(0, str(ROOT))
    from scripts.run_s02_candidate_a_factoring_pilot import verify_runtime
    bootstrap = "900bb2051225b4a3d99bf422c3b2e5e386e3e7bc"
    before = verify_runtime(bootstrap)
    assert sys.dont_write_bytecode and sys.pycache_prefix
    d = json.loads(DISPATCH.read_text())
    assert subprocess.check_output(["git","rev-parse","HEAD"],cwd=ROOT,text=True).strip() == d["amendmentDispatchBase"]
    assert d["actualTask2Base"] == "d824fa3381ecda846be7e8dd027732f1a7f62e91"
    assert d["runtimeBootstrapBase"] == bootstrap
    assert all(sha(p) == h for p,h in PINS.items())
    STORE.mkdir(exist_ok=False)
    (STORE/"bin").mkdir()
    # Exact immutable executable copy, not a search-path symlink or shell shim.
    with (STORE/"bin/uv").open("xb") as stream:
        stream.write(Path("/home/charl/.local/bin/uv").read_bytes())
    (STORE/"bin/uv").chmod(0o755)
    paths = [Path(p) for p in PINS] + [
        LAUNCHER, R/"capture-task2-python-uv.py", DISPATCH, STORE/"bin/uv"]
    entries = []
    with tarfile.open(STORE/"runtime.tar.gz","x:gz") as archive:
        for p in sorted(paths):
            raw = p.read_bytes()
            name = str(p).lstrip("/")
            entry = tarfile.TarInfo(name)
            entry.size = len(raw)
            archive.addfile(entry, io.BytesIO(raw))
            entries.append({"path":str(p),"archivePath":name,
                "sha256":hashlib.sha256(raw).hexdigest(),"bytes":len(raw)})
    write(STORE/"manifest.json", {"files":entries,
        "archiveSha256":sha(STORE/"runtime.tar.gz"),
        "selectedPython":PYTHON,"selectedPrefix":PREFIX,
        "resolvedInterpreter":RESOLVED,"dispatch":d,
        "sharedRuntime":before,"parentArgv":sys.orig_argv,
        "parentCachePrefix":sys.pycache_prefix,
        "parentBytecodeDisabled":sys.dont_write_bytecode})
    # Version and dynamic-loader inventory are original raw streams.
    for name,argv in (
        ("version",[str(STORE/"bin/uv"),"--version"]),
        ("loader",["/usr/bin/ldd",str(STORE/"bin/uv")])):
        with (STORE/(name+".stdout")).open("xb") as out, (STORE/(name+".stderr")).open("xb") as err:
            p = subprocess.run(argv,stdout=out,stderr=err,timeout=15)
        write(STORE/(name+".json"),{"argv":argv,"exitCode":p.returncode,
            "stdoutSha256":sha(STORE/(name+".stdout")),
            "stderrSha256":sha(STORE/(name+".stderr"))})
        assert p.returncode == 0
    loader = (STORE/"loader.stdout").read_text()
    loaded = set()
    for line in loader.splitlines():
        words = line.split()
        if "=>" in words:
            loaded.add(str(Path(words[words.index("=>")+1]).resolve()))
        elif words and words[0].startswith("/"):
            loaded.add(str(Path(words[0]).resolve()))
    assert loaded == {p for p in PINS if p.startswith("/usr/lib/")}
    assert (STORE/"version.stdout").read_text().strip() == "uv 0.12.5 (x86_64-unknown-linux-gnu)"
    assert checks()
    assert verify_runtime(bootstrap) == before
    write(STORE/"result.json", {"ok":True,"runtimeUnchanged":True,
        "manifestSha256":sha(STORE/"manifest.json")})

def launch(stage, argv):
    assert stage in ("task2-original-python-uv-collect",
        "task2-original-python-uv-regression","core-comparison-uv")
    assert argv[0] == PYTHON
    assert sys.dont_write_bytecode and sys.pycache_prefix
    digest = checks()
    out = R/stage
    env = dict(os.environ)
    for key in list(env):
        if key.startswith("UV_") or key == "VIRTUAL_ENV":
            env.pop(key)
    env.update(PATH=str(STORE/"bin")+os.pathsep+env["PATH"],
        VIRTUAL_ENV=PREFIX, UV_PROJECT_ENVIRONMENT=PREFIX, UV_PYTHON=PYTHON,
        UV_NO_SYNC="1", UV_OFFLINE="1", UV_PYTHON_DOWNLOADS="never",
        UV_NO_CONFIG="1", UV_NO_ENV_FILE="1",
        UV_CACHE_DIR=str(out/"fresh-uv-cache"))
    (out/"fresh-uv-cache").mkdir(exist_ok=False)
    identity = ("import json,sys,pathlib;print(json.dumps({"
        "'executable':sys.executable,'prefix':sys.prefix,"
        "'resolved':str(pathlib.Path(sys.executable).resolve()),"
        "'version':list(sys.version_info[:3])},sort_keys=True))")
    probe = ["uv","run","python","-c",identity]
    write(out/"launch.json", {"parentArgv":sys.orig_argv,"requestedChildArgv":argv,
        "probeArgv":probe,"environment":{k:v for k,v in env.items()
            if k.startswith(("UV_","PYTHON")) or k in
            ("PATH","VIRTUAL_ENV","NODE_OPTIONS","NODE_DISABLE_COMPILE_CACHE")},
        "supplementSha256":digest})
    with (out/"uv-identity.stdout").open("xb") as stdout, (out/"uv-identity.stderr").open("xb") as stderr:
        p = subprocess.run(probe,cwd=ROOT,env=env,stdout=stdout,stderr=stderr,timeout=30)
    write(out/"uv-identity.json", {"exitCode":p.returncode,
        "stdoutSha256":sha(out/"uv-identity.stdout"),"stderrSha256":sha(out/"uv-identity.stderr")})
    assert p.returncode == 0
    actual = json.loads((out/"uv-identity.stdout").read_text())
    assert actual == {"executable":PYTHON,"prefix":PREFIX,"resolved":RESOLVED,"version":[3,13,14]}, actual
    assert checks() == digest
    os.execve(PYTHON,argv,env)

if __name__ == "__main__":
    assert sys.argv[1] == "launch"
    launch(sys.argv[2],sys.argv[3:])
```

The launch receipt deliberately records only the named controlled/UV/Python/PATH keys, not unrelated inherited environment values. The executed environment remains unchanged except the explicit listed controls. This is an environment-selection receipt, not a claim that every inherited variable was retained.

## Task2: Create the companion capture without modifying the original

- [ ] Run this deterministic generator after adoption; it emits an apply_patch payload, which the implementer applies with apply_patch. It does not write source itself. Each replacement is exact and count-checked; root reviews the resulting companion diff before dispatch.

```python
from pathlib import Path
import hashlib
root = Path("/home/charl/Moriarty/.worktrees/s01-audit-start")
relative = ".superpowers/sdd/a5-factoring-receipts/"
old = root/relative/"capture-task2-python.py"
text = old.read_text()
assert hashlib.sha256(old.read_bytes()).hexdigest() == "6d0eb0ea76a0baab1d7544cc83c091602e3a0e2ebc45cf5d3b1fd839ebbb213f"
def replace(a,b):
    global text
    assert text.count(a) == 1, a
    text = text.replace(a,b)
replace('import xml.etree.ElementTree as ET',
    'import xml.etree.ElementTree as ET\nfrom a5_uv_runtime import checks as check_uv, prepare as prepare_uv, STORE, LAUNCHER, DISPATCH')
replace('paths.update((FINAL/"cases.json", Path(__file__).resolve(), TASK_DISPATCH, OLD, ROOT/"pyproject.toml"))',
    'paths.update((FINAL/"cases.json", RECEIPTS/"capture-task2-python.py", TASK_DISPATCH, OLD, ROOT/"pyproject.toml"))')
start = text.index('if mode == "prepare":')
end = text.index('elif mode in ("collect", "python", "core-comparison"):')
text = text[:start] + 'if mode == "prepare-uv":\n    prepare_uv()\nelif mode in ("collect", "python", "core-comparison"):' + text[end+len('elif mode in ("collect", "python", "core-comparison"):'):]
# Stage names and collection dependency are changed consistently, in this new file only.
assert text.count('task2-original-python-collect') == 2
text = text.replace('task2-original-python-collect','task2-original-python-uv-collect')
replace('task2-original-python-regression','task2-original-python-uv-regression')
replace('"core-comparison":"core-comparison"', '"core-comparison":"core-comparison-uv"')
replace('    check(manifest)\n    stage =',
    '    check(manifest)\n'
    '    assert sha(BASE/"manifest.json") == "7e8e5297a333dd91cd5aef8a9aa755355b2941b62cdd4aad185f95a86a5fa04c"\n'
    '    assert sha(RECEIPTS/"task2-original-python-collect/collected-nodeids.json") == "adb5b3c1e75ee020ef7192ca4aea6cc0a7c5f367133a786db2116792978d52c2"\n'
    '    uv_before = check_uv()\n    stage =')
replace('    code = record(stage,argv,1200,4096,',
    '    extra += [LAUNCHER,DISPATCH,STORE/"manifest.json",STORE/"runtime.tar.gz",STORE/"bin/uv"]\n'
    '    extra += [RECEIPTS/"task2-original-python-collect"/name for name in ("input.json","result.json","stdout.txt","stderr.txt","collected-nodeids.json","fixture-checks.json","inventory-result.json")]\n'
    '    requested_child_argv = argv\n'
    '    extra = list(dict.fromkeys(extra))\n'
    '    argv = [PYTHON,"-B","-X","pycache_prefix="+str(RECEIPTS/stage/"launcher-cache"),str(LAUNCHER),"launch",stage,*argv]\n'
    '    code = record(stage,argv,1200,4096,')
replace('        check(manifest)\n    except (AssertionError, OSError):',
    '        check(manifest)\n        assert check_uv() == uv_before\n    except (AssertionError, OSError):')
replace('        "parentBytecodeDisabled":sys.dont_write_bytecode,"originalCaseCount":53})',
    '        "parentBytecodeDisabled":sys.dont_write_bytecode,"originalCaseCount":53,\n'
    '        "supplementSha256":uv_before,"requestedChildArgv":requested_child_argv})')
replace('        write_json(RECEIPTS/stage/"collected-nodeids.json", ids)',
    '        prior_ids = json.loads((RECEIPTS/"task2-original-python-collect/collected-nodeids.json").read_text())\n'
    '        assert ids == prior_ids, "Fresh collection must equal original ordered441 IDs"\n'
    '        write_json(RECEIPTS/stage/"collected-nodeids.json", ids)')
replace('    else:\n        argv = [PYTHON,"scripts/check_s02_candidate_a_correspondence.py"',
    '    else:\n'
    '        passed = RECEIPTS/"task2-original-python-uv-regression"\n'
    '        result = json.loads((passed/"result.json").read_text())\n'
    '        inventory = json.loads((passed/"inventory-result.json").read_text())\n'
    '        assert result["exitCode"] == 0 and result["sourceAndToolsUnchanged"] and result["runtimeUnchanged"]\n'
    '        assert inventory["count"] == 441 and inventory["matchesFreshCollection"]\n'
    '        extra += [passed/name for name in ("result.json","inventory-result.json","junit.xml","fixture-checks.json")]\n'
    '        argv = [PYTHON,"scripts/check_s02_candidate_a_correspondence.py"')
print("*** Begin Patch")
print("*** Add File: "+relative+"capture-task2-python-uv.py")
print("\n".join("+"+line for line in text.splitlines()))
print("*** End Patch")
```

- [ ] Pin the prior original collection files and failed-stage input/result/stdout/stderr/JUnit in the supplement dispatch receipt by exact hashes before prepare. Include dispatch itself in every fresh stage (as above). Root verifies prior441 collection SHA and failed-stage stdout SHA `1dfb47e9d145cf10f88e6d735c303b30189a0a7c4f623beb0f79b8bb48129830`.
- [ ] Parse both newly assembled Python sources using compile(..., "exec") without executing them. Review generated diff and validate no original file changed. Capture exact new source hashes in dispatch. These checks are syntax/provenance, not a runtime GREEN.

## Task3: Review-gated sequential commands

- [ ] Root admits the supplement source and dispatch before prepare. Use the exact commands below, sequentially, each with a fresh parent cache. Keep the active command until terminal and stop on any mismatch.

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-prepare-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv.py prepare-uv
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-collect-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv.py collect
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-regression-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv.py python
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-core-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv.py core-comparison
```

- [ ] Prepare must return terminal0 with original version/loader streams and exact supplement archive bytes; root may byte-admit it before collection. Its original command output is retained in the execution transcript and result.json; there is no invented historical native command receipt.
- [ ] Fresh collection must report441 and equal prior ordered IDs. All source/fixture/runtime/supplement checks must remain true.
- [ ] Runtime must report441passed, JUnit441 with no failures/errors/skips, and exact fresh collection set equality. The unchanged failing test is included with identical bytes/argv; this is the environmental RED→GREEN, not a semantic fix.
- [ ] Only then execute Core53. Require exact report `{"ok":true,"differences":[],"scope":"complete-inventory"}` against existing14ITFs/cases53. No case omission or filter.
- [ ] Root audits all original raw output/archive byte closure, launch/probe evidence,441inventory, Core53report, actual amendment dispatch vs bootstrap/taskbase. No task admission is claimed until that review and the remaining native corpus/equivalence/samples succeed.

## Remaining risks and stop conditions

UV_PROJECT_ENVIRONMENT selection is not claimed from a guessed default: the retained probe must establish the actual admitted prefix/executable before each command. UV_NO_SYNC implies frozen lock behavior per installed uv help; UV_OFFLINE disables uv network access, not an OS-wide network sandbox for arbitrary test code. No network package acquisition is authorized.

The supplement captures real uv and its current loader closure, not a new whole environment. The admitted Python3329-file archive and shared tool4758-file archive remain authoritative, with existing before/after validators. New unrelated A4 changes may continue only outside the2691selected source/input set.

A probe, loader, inventory, source, tool, timeout, or pytest mismatch is terminal for this dispatch. Do not alter a test, broaden PATH to the user's bin, fake uv behavior, synchronize environments, rerun automatically, or proceed to Core53 after failure.
