# A5 exact uv interpreter-alias amendment

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans. Assembly is authorized; execution still requires root admission.

**Goal:** Accept the one independently observed and verified uv executable alias without changing the selected Python runtime, tests, or failed evidence.

**Architecture:** Create two companion copies with a narrow exact diff. Reuse the admitted13-member uv supplement and all shared runtime/fixture archives. Preserve both failed stages and their source bytes.

**Tech Stack:** Existing CPython3.13.14, uv0.12.5, immutable recorder.

## Evidence and EARS delta

The admitted uv environment probe returned executable `/home/charl/Moriarty/.venv/bin/python3`, prefix `/home/charl/Moriarty/.venv`, resolved interpreter `/home/charl/.local/share/uv/python/cpython-3.13.14-linux-x86_64-gnu/bin/python3.13`, version[3,13,14]. The actual python3 path is a symlink whose exact text is `python`. The prior assertion expected the lexical path ending in python and stopped before pytest. Preserve `task2-original-python-uv-collect` exit1 and its full original probe/launch/archive/fixture receipts; do not recategorize it as a test failure.

- UA001: Before and after each command, validation SHALL require that exact python3 path is a symlink, its readlink text is exactly python, and its resolved path equals the admitted interpreter. No alias allowlist.
- UA002: Probe equality SHALL differ only in expected executable spelling. Prefix, resolved interpreter, version, environment isolation, offline/no-sync controls, and archive checks remain unchanged.
- UA003: All fresh command receipts SHALL pin new sources, a separate root alias dispatch, and all original failed-probe source/output/launch evidence. The existing supplement/archive/launcher/capture bytes SHALL remain unchanged.
- UA004: Fresh collection SHALL match the original ordered441 IDs; runtime SHALL pass all441 exactJUnit; only then Core53 SHALL run. Every mismatch stops, without implicit retry.
- UA005: This amendment SHALL not alter test or requested child argv, bypass uv, prepare another runtime archive, launch native jobs, close expanded A4/final unrestricted gates, or claim full A5.

## Exact files and assembly

Create only `.superpowers/sdd/a5-factoring-receipts/a5_uv_runtime_alias.py` and `capture-task2-python-uv-alias.py`. Root owns the new `task2-uv-alias-dispatch.json`, which records the actual alias dispatch commit separately from original amendment3a7d32d3a812207e860e340851aa17c1a8b3494a, Task2d824fa3381ecda846be7e8dd027732f1a7f62e91, and bootstrap900bb2051225b4a3d99bf422c3b2e5e386e3e7bc. It pins the two new sources and preserved failed-probe files before dispatch.

- [ ] Apply the two patches emitted by this exact generator with apply_patch. The generator does not write source itself. Compile both assembled sources without importing/executing them. Root reviews complete generated diffs and hashes.

```python
from pathlib import Path
import hashlib
R = Path("/home/charl/Moriarty/.worktrees/s01-audit-start/.superpowers/sdd/a5-factoring-receipts")
runtime = (R/"a5_uv_runtime.py").read_text()
capture = (R/"capture-task2-python-uv.py").read_text()
assert hashlib.sha256(runtime.encode()).hexdigest() == "6dc316f9ac9b615c3f36013d1328f3f0d33765cb7e1a445a95c0a5168e86141a"
assert hashlib.sha256(capture.encode()).hexdigest() == "2380e9f74945390ddc07215328175b86a50551461ba3fbb38e618f9f149e45e4"
def one(s,a,b):
    assert s.count(a) == 1, a
    return s.replace(a,b)
runtime = one(runtime, '    assert str(Path(PYTHON).resolve()) == RESOLVED',
    '    assert str(Path(PYTHON).resolve()) == RESOLVED\n'
    '    alias = Path(PREFIX)/"bin/python3"\n'
    '    assert alias.is_symlink() and os.readlink(alias) == "python"\n'
    '    assert str(alias.resolve()) == RESOLVED')
runtime = one(runtime,
    '"executable":PYTHON,"prefix":PREFIX,"resolved":RESOLVED,"version":[3,13,14]',
    '"executable":PREFIX+"/bin/python3","prefix":PREFIX,"resolved":RESOLVED,"version":[3,13,14]')
capture = one(capture,
    'from a5_uv_runtime import checks as check_uv, prepare as prepare_uv, STORE, LAUNCHER, DISPATCH',
    'from a5_uv_runtime_alias import checks as check_uv, STORE, LAUNCHER, DISPATCH')
capture = one(capture,
    'if mode == "prepare-uv":\n    prepare_uv()\nelif mode in ("collect", "python", "core-comparison"):',
    'if mode in ("collect", "python", "core-comparison"):')
for before,after in (
    ("task2-original-python-uv-collect","task2-original-python-uv-alias-collect"),
    ("task2-original-python-uv-regression","task2-original-python-uv-alias-regression"),
    ("core-comparison-uv","core-comparison-uv-alias")):
    assert runtime.count(before) == 1 and capture.count(before) in (1,2)
    runtime = runtime.replace(before,after)
    capture = capture.replace(before,after)
capture = one(capture, '    requested_child_argv = argv',
    '    extra += [RECEIPTS/"task2-uv-alias-dispatch.json"]\n'
    '    failed_probe = RECEIPTS/"task2-original-python-uv-collect"\n'
    '    extra += [failed_probe/name for name in ("input.json","result.json","stdout.txt","stderr.txt",\n'
    '        "fixture-checks.json","launch.json","uv-identity.stdout","uv-identity.stderr",\n'
    '        "uv-identity.json","source-and-runner.tar.gz","resources.txt")]\n'
    '    requested_child_argv = argv')
for name,text in (("a5_uv_runtime_alias.py",runtime),("capture-task2-python-uv-alias.py",capture)):
    compile(text,name,"exec")
    print("*** Begin Patch")
    print("*** Add File: "+str(R/name))
    print("\n".join("+"+line for line in text.splitlines()))
    print("*** End Patch")

```

The new alias launcher retains an unused copied prepare function only to keep the review diff narrow; neither its main entry nor the new capture exposes prepare. No prepare call is authorized. Its validation continues to check the old supplement's original source bytes, while per-command extras pin the new source bytes.

## Sequential commands after separate root source/dispatch admission

No prepare step. Each command must have terminal0, untimedout, and unchanged source/fixture/runtime/supplement/alias checks before proceeding.

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-alias-collect-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv-alias.py collect
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-alias-regression-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv-alias.py python
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a5-factoring-receipts/uv-alias-core-parent-cache .superpowers/sdd/a5-factoring-receipts/capture-task2-python-uv-alias.py core-comparison
```

- [ ] Require new collection441 and ordered equality with prior original collection.
- [ ] Require all441passing, no skips/errors, exactJUnit node-ID set equality.
- [ ] Require Core53 exact report `{"ok":true,"differences":[],"scope":"complete-inventory"}`.
- [ ] Retain all fresh source archives, raw probe/launch/output streams and metadata alongside both original environmental failure stages. Root independently audits before Task2 admission.

The failed original441 run and failed uv identity probe remain two distinct observations. This correction does not weaken semantic assertions or permit arbitrary aliases; it replaces a guessed lexical identity with one verified exact symlink relation. A further mismatch stops for diagnosis, not a broader third attempt.

