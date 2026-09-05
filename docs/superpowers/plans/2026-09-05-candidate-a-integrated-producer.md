# Candidate A Integrated Producer Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Root assigns the producer and independent checker separately. Do not dispatch additional implementers or modify the checker from this plan.

**Goal:** Produce the complete, fixed A4 authority-history inventory from actual guarded Candidate A installment and swap executions, with original ITF records, strict structural export and immutable command/source receipts.

**Architecture:** New schema-version-2 entry points leave schema 1 unchanged. A shared typed observer wraps the accepted lifecycle guards and updates; two deterministic batch drivers retain only the current authority state and latest event. A structural Python exporter preserves raw ITF values and exact adjacent positions; the separately authored checker supplies semantic replay, not this producer.

**Tech Stack:** Quint 0.32.0, Rust evaluator 0.6.0, Python 3 standard library and pytest, existing frozen Candidate A/common modules.

## Global Constraints

- Status: proposed implementation plan; all new commands and outcomes below are specified-only until actual terminal receipts exist.
- Adopted design: `docs/superpowers/specs/2026-09-05-candidate-a-integrated-export-design.md`, adoption `effb7af`.
- Controlling main XML: `/home/charl/Moriarty/deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`, A4, SHA256 `f185e1ec35dfff8c89c5eb962be6e46b9614b0ae603af8a1bdf232287c871c91`; main OpenSpec `openspec/changes/s02-candidate-a-a4/specs/s02-candidate-a-a4/spec.md`.
- Planning pre-dispatch A3 anchor: `926b350cc4f5943dcf5555c091c58b8782b797e9`. Before execution root must pin final admitted A2/A3 source and shared regression receipts. Do not infer admission from moving HEAD.
- Never edit frozen Python Core, common/A semantics, accepted lifecycle fixtures, lifecycle tests/harnesses, or schema-1 producer/checker.
- No expected fixture supplies an executed result. A second actual deterministic evaluation at the observation boundary is instrumentation, not an adapter-internal execution attestation.
- `EvidenceValid` is a finite external-verifier premise. Actor metadata is not an authenticated signer.
- Cancellation has `CancellationCallA`, `NoCoreProjection`, unchanged agreement, and `computations=[]`. It never calls Core.
- Actual agreement computations keep tagged `ATransactionEvaluation` and `ExtractionObservedA4(raw AEffectExtraction)`. An evaluator diagnostic uses `ExtractionNotApplicableA4`, is retained in a diagnostic event, and cannot satisfy the accepted event inventory.
- Denied probes and case-end preserve the entire authority state. A retained rejection requires a separate actual reject transition. Case-start is the only reset and is permitted only after the preceding case-end.
- Both profiles are present for every case family. No subset acceptance and no sample-count substitute for a missing fixed event.
- Root owns the adopted literal JSON inventory, final manifest, archive intake, commits and acceptance. Producer/checker authorship remains separate; no Council, integration, universal-proof or completed-A4 claim follows from this subplan alone.

## File responsibilities and interfaces

Create only these source units when root dispatches implementation:

| Path | Responsibility |
|---|---|
| `scripts/s02_candidate_a_integrated_inventory.py` | Producer-side independent enumeration, descriptor/event grammar and generation of literal Quint case tables. Root reviews its generated JSON before freezing the shared inventory. |
| `specs/quint/s02/candidate_a_integrated_observer.qnt` | Full typed command, guard, computation, exact mutation, observation and generic boundary helpers. No persistent state. |
| `specs/quint/s02/candidate_a_integrated_cases.qnt` | Generated, reviewed literal lists for exactly 32 installment and 46 swap cases. |
| `specs/quint/s02/candidate_a_integrated_driver.qnt` | Four-variable deterministic batch machine parameterized by its literal cases. |
| `specs/quint/s02/candidate_a_integrated_installment_export.qnt` | Installment-only driver instantiation. |
| `specs/quint/s02/candidate_a_integrated_swap_export.qnt` | Swap-only driver instantiation. |
| `specs/quint/s02/candidate_a_integrated_export_test.qnt` | Pure observer and actual transition/denial/derivation/reset tests. |
| `scripts/export_s02_candidate_a_integrated.py` | New strict schema-2 structural exporter and CLI. No authority or Python Core replay. |
| `scripts/record_s02_candidate_a_integrated.py` | Exclusive fresh-directory command recorder, transitive source snapshots, raw stdout/stderr, exit and tool/source hashes. |
| `tests/test_s02_candidate_a_integrated_export.py` | Parser, inventory, raw-record linkage, domain, reset and omission controls. |

Root-owned read-only acceptance input:
`evidence/s02-candidate-a-completion/a4/inventory.json`.
Independent checker entry point and tests belong to the separately adopted checker plan.

Export envelope has exactly:
`schema_version, inventory_sha256, source_pins, input_pins, receipt_pins, cases`.
Each case has exactly `case_id,lifecycle,profile,scenario,control,events`.
Each event has exactly `case_id,profile,sequence,kind,arguments,observed_guard,computations,before,after,provenance`.
Provenance has exactly `input_path,input_sha256,before_index,after_index`.
All descriptors and event kinds are JSON strings. `sequence` and positions are JSON integers, never booleans. Nested `arguments`, `computations`, `before`, and `after` remain original raw ITF representations.

Raw driver variables are exactly `authorityState,latestEvent,caseIndex,cursor`.
`latestEvent` has `caseId,profile,sequence,kind,arguments,observedGuard,computations`.
The first ITF state is case-start, sequence 0, before=after=unsigned. For every subsequent state at index k, before_index=k-1 and after_index=k. Do not search backward for an equal-looking state.

`arguments={guard:A4Guard,command:A4Command}` distinguishes a lifecycle guard from a direct A/common guard. Full generic arguments and the original lifecycle command must agree; e.g. recovery preparation is gated by `canCommandI`, including cancellation, not merely generic `canPrepareSigningAuthorityA`.

## Fixed case and event inventory

The independent checker planner agreed on **78 cases and 1,557 events** (case-start/end included), with no events deduplicated. Case order is installment then swap; enumerate ordinary scenarios, then swap verified-stale, then controls in the table order. Each descriptor is immediately expanded to SignAfterResolve then SignBeforeResolve. A case ID is `<lifecycle>/<scenario>/<control>/<profile>`, where lifecycle is `installment` or `swap` and profile is `SignAfterResolve` or `SignBeforeResolve`. Exported profile is the full literal `SignAfterResolve` or `SignBeforeResolve`.

| Family, each profile | Cases/profile | Events/case | Both-profile events |
|---|---:|---:|---:|
| I two-fills ordinary | 1 | 17 | 34 |
| I recover-r0-choice2 ordinary | 1 | 22 | 44 |
| I recover-r0-timeout100/101 ordinary | 2 | 23 | 92 |
| I recover-r0-refuse100/101 ordinary | 2 | 20 | 80 |
| I recover-r1-choice2 ordinary | 1 | 27 | 54 |
| I recover-r1-timeout100/101 ordinary | 2 | 28 | 112 |
| I recover-r1-refuse100/101 ordinary | 2 | 25 | 100 |
| I no-cancel | 1 | 9 | 18 |
| I unsigned | 1 | 14 | 28 |
| I old-nonce | 1 | 13 | 26 |
| I fresh-duplicate-cancel | 1 | 13 | 26 |
| I unused-successor | 1 | 12 | 24 |
| S funded2-settle/refund ordinary | 2 | 26 | 104 |
| S funded0-timeout100/101 ordinary | 2 | 7 | 28 |
| S funded1-timeout100/101 ordinary | 2 | 17 | 68 |
| S funded2-timeout100/101 ordinary | 2 | 27 | 108 |
| S funded2-refuse100/101-choice0/1 ordinary | 4 | 22 | 176 |
| S verified-stale | 1 | 22 | 44 |
| S stale-signing | 1 | 16 | 32 |
| S unused-successor/reversed-effects/reductions/neutral-chooser | 4 | 24 | 192 |
| S second-plan | 1 | after 9, before 8 | 17 |
| S wrong-Core-chooser | 1 | 14 | 28 |
| S wrong-signer/wrong-nonce | 2 | 20 | 80 |
| S stale-facts | 1 | 21 | 42 |

Installment totals: 32 cases, 638 events. Swap totals: 46 cases, 919 events. One whole batch per lifecycle therefore needs 637 and 918 steps respectively; a batch record contains its initial state plus all steps. These are finite export schedules, not arbitrary-interleaving model checks.

Planning correction disclosure: both planners initially added the swap-control rows as591 instead of391, reporting1,757 total events. Root executed the pure selector enumeration and exposed the arithmetic error. The author then independently executed that same finite enumerator:78cases, I638/S919, total1,557; no case or selector was removed. The corrected canonical inventory SHA256 is `8a1a6136afc9fda3c29e050df8b80144750cc21365e9194924401c155dc6402b`. Root preserves a separate correction addendum for the already-adopted checker plan; this producer plan does not edit its bytes.

Exact ordinary command lists are `routeI(s)` and `routeS(s)` from the admitted lifecycle modules. Append the following **statically enumerated** replay probes before case-end:

- I TwoFills: proposal then commit for FirstFillAttempt, then SecondFillAttempt.
- I recover-r0 success: CancelAttempt, RecoveryAttempt. I r0 refusal: CancelAttempt only.
- I recover-r1 success: FirstFillAttempt, FreshCancelAttempt, RecoveryAttempt. I r1 refusal: FirstFillAttempt, FreshCancelAttempt.
- Each I recover case additionally probes parent cancellation, slot1, slot2, using retained full `parent,entry,prepared` records.
- S every ordinary case: FundingOneAttempt if funded>=1; FundingTwoAttempt if funded=2; DispositionAttempt always, including refusals. For each ID proposal then commit, with full original attempted operation/observation/actor.
- S verified-stale: `staleRouteS` with an explicit denied Commit inserted between Advance100 and RejectVerified; no extra ordinary replay suffix.

Control step lists, after the named actual prefix (prefix length excludes case-start):

| Control | Prefix | Ordered remaining events, before case-end |
|---|---|---|
| I no-cancel | race prefix2 | denied PrepareRecovery; denied operationFinancialGuard recovery; ProposeRecovery; denied Verify; RejectProposed |
| I unsigned | cancellation-first route prefix8 | ProposeRecovery; denied Verify; RejectProposed; denied PrepareParent nonce0 |
| I old-nonce | cancellation-first route prefix8 | ProposeRecovery; denied Verify with parent nonce0 evidence; RejectProposed with same evidence |
| I fresh-duplicate-cancel | cancellation-first route prefix8 | Propose FreshCancelAttempt with actual cancel0 observation; denied Verify; RejectProposed |
| I unused-successor | TwoFills prefix4 | derive changed proposed FirstFillAttempt; denied Verify; RejectProposed; explicitly derive verified mutant from retained rejection; denied Commit; RejectVerified |
| S stale-signing | funding prefix11 | Prepare disposition Alice; Advance100; denied Sign |
| S four observation variants | settle prefix16 | derive changed proposed DispositionAttempt; denied Verify; RejectProposed; explicitly derive verified mutant from retained rejection; denied Commit; RejectVerified |
| S second-plan | unsigned | denied authorityPlanMatchesA; bad-policy Prepare (denied after, actual before); original-policy Prepare after only; Sign original; Propose actual funding observation with changed plan; denied Verify; RejectProposed |
| S wrong-Core-chooser | funding prefix11 | denied CoreAccepted probe of actual settle choice by Alice at Time2 |
| S wrong-signer/wrong-nonce | settle prefix16 | denied Verify with changed evidence; RejectProposed retaining that evidence |
| S stale-facts | settle prefix16 | derive proposed observation with changed planned predecessor environment100; denied Verify; RejectProposed |

Case-end statuses are exactly `financial-terminal`, `refusal-terminal`, and `negative-complete`. Ordinary financial/refusal outcomes come from retained actual states/attempts. An adversarial case ending with unrelated pending records (I unused-successor retains the other proposed race attempt) is `negative-complete`, never financial or no-pending. Stale-signing explicitly retains PreparedSigning. Constructed verified derivations preserve earlier proposed rejection in its earlier raw position; they are not advertised as reachable verification transitions. Derivation `baseSequence` is the original checked proposal: I unused-successor sequence3, S sequence16. Both derivation stages refer to that same original base, never to the post-rejection state as a newly legitimate origin.

## Task 0: Evidence bootstrap (execute before Tasks 1–3)

**Files:** Create `scripts/record_s02_candidate_a_integrated.py`; use only new `.superpowers/sdd/a4-producer-receipts/` stages. This nonbehavioral setup precedes every RED command referenced earlier. It does not depend on the exporter existing.

Runtime precondition: root first admits the **A5-owned** bootstrap store at `.superpowers/sdd/a5-factoring-receipts/tool-store/`. Its manifest and tar retain actual Node, installed Quint `dist/`, `node_modules/`, `package.json`, Rust and the shared A5 runtime superset. It references the existing Python3329-file archive. The producer never creates or duplicates either store. This is an explicit execution dependency, not a dependency on A5 model-checking success. Root coordinates bootstrap before producer Task0 commands.

Root also creates `.superpowers/sdd/a4-producer-dispatch.json` before producer source edits, with exactly `beforeDispatchCommit`, `sharedDispatchSha256`, `runtimeManifestSha256`, `runtimeArchiveSha256`, `runtimeHelperSha256`. The first field is the actual full40-character producer pre-dispatch commit; the remaining fields bind root's admitted A5 dispatch/store and `scripts/run_s02_candidate_a_factoring_pilot.py` helper bytes. Do not copy A5's pre-dispatch commit into the producer field. Each command separately records its current `sourceCommit` and full dirty source hashes; neither field replaces the other. The imported helper source belongs to runtime receipts, not semantic source_pins.

**Produces:** Exclusive stage directory with `before/source/`, `after/source/`, `stdout.bin`, `stderr.bin`, `receipt.json`, exact source hashes and bytes, before/after shared-runtime pins, actual Node/CLI/Rust/Python identities, requested and executed argv, producer pre-dispatch base versus sourceCommit, terminal code and elapsed time. Runtime pins are separate from the unchanged semantic source_pins set. Original runtime bytes are retained once in the shared store; receipts reference them. Preserve failed bootstrap commands separately; they are not semantic RED.

```python
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
STAGES = ROOT / ".superpowers/sdd/a4-producer-receipts"
DISPATCH = ROOT / ".superpowers/sdd/a4-producer-dispatch.json"
SHARED_DISPATCH = ROOT / ".superpowers/sdd/a5-factoring-receipts/dispatch.json"
TOOL_STORE = ROOT / ".superpowers/sdd/a5-factoring-receipts/tool-store"
RUNTIME_HELPER = ROOT / "scripts/run_s02_candidate_a_factoring_pilot.py"
PYTHON_MANIFEST = ROOT / ".superpowers/sdd/a4-checker-task1-receipts/python-environment.json"
PYTHON_ARCHIVE = PYTHON_MANIFEST.with_suffix(".tar.gz")
PYTHON_MANIFEST_SHA = "cd004057c4067bf7cc536d2cec5038add88d0852c1a7de5030e402ff2204f9c4"
PYTHON_ARCHIVE_SHA = "7363d106d464f976af2a0e7f1f56e3c7bed237c7a3396a60d073369add068dac"
ROOTS = (
    "specs/quint/s02/candidate_a_integrated_installment_export.qnt",
    "specs/quint/s02/candidate_a_integrated_swap_export.qnt",
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
)
IMPORTS = re.compile(r'^\s*import\s+[^\n]*?\s+from\s+"([^"]+)"', re.MULTILINE)

def sha(data): return hashlib.sha256(data).hexdigest()

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
    artifacts = {path.name: sha(path.read_bytes()) for path in sorted(stage.glob("*.itf.json"))}
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
```

Bootstrap checks: `/home/charl/Moriarty/.venv/bin/python -B -c 'import ast,pathlib; ast.parse(pathlib.Path("scripts/record_s02_candidate_a_integrated.py").read_text())'`, then the exact smoke command below. Every recorder launch uses `-B` and its exclusive stage's fresh cache prefix, including RED and GREEN stages described by name later. A justified rerun must change both stage and cache prefix. Children inherit that cache prefix with bytecode writes disabled. A bootstrap smoke is not model behavior or evidence of interpreter correctness. Preserve only specific stage paths; never overwrite a prior stage. Early stages list not-yet-created planned sources explicitly. Final exports require that list empty and exact source closure equal the admission, not a partial early-stage closure.

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/recorder-smoke/python-cache scripts/record_s02_candidate_a_integrated.py --stage recorder-smoke -- /home/charl/Moriarty/.venv/bin/python --version
```

Run long recorder processes as live jobs. Poll their existing handle with waits no longer than60seconds and send concise progress; do not start duplicate Quint compilers because stdout is written directly to the receipt file. Read terminal receipt and both raw output files before making a pass claim. An interrupted recorder lacking `receipt.json` is nonterminal evidence, not success; preserve it and use a new stage for any justified rerun.

## Task 1: Freeze the typed observer contract and finite inventory

**Files:** Create the producer inventory module, observer, literal case tables and test file listed above. Do not create acceptance traces yet.

**Consumes:** Frozen lifecycle `canCommandI/applyCommandI/routeI`, `canCommandS/applyCommandS/routeS/staleRouteS`; common/A guard and update functions.

**Produces:** `A4Command`, `A4Guard`, `A4Arguments`, `A4Computation`, `A4Event`, `A4Case`; full command cross-mapping; `observeA4`, `guardA4`, `applyA4`; literal case tables and root-reviewable JSON inventory.

- [ ] Add and typecheck the declarations below before behavior. Imports in every new pure module are the complete direct common/A and both lifecycle imports; imports do not confer authority to edit their sources.

```quint
type A4Signed = SignedPolicy[APredecessor,AContinuation,ACall,AResolvedPlan]
type A4Parent = Parent[A4Signed]
type A4Entry = Entry[A4Signed]
type A4Scenario = InstallmentScenarioA4(ScenarioI) | SwapScenarioA4(ScenarioS)
type A4CaseDescriptor = {caseId: str, lifecycle: str, profile: str,
  scenario: str, control: str}
type A4DerivationStage = ProposedDerivationA4 | VerifiedDerivationA4
type A4Diagnostic = ObserverIssueA4(str)
  | AdapterIssueA4({call: ACall, adaptation: AAuthorityAdaptation})
type A4Command = PrepareA4({policy: AAuthorityPolicy, signer: Principal})
  | SignA4({policy: AAuthorityPolicy, signer: Principal, token: int})
  | ProposeA4({id: AttemptId, operation: Operation,
      observation: AAuthorityObservation, actor: Principal})
  | VerifyA4({id: AttemptId, evidence: AAuthorityEvidence})
  | CommitA4({id: AttemptId})
  | RejectProposedA4({id: AttemptId, evidence: AAuthorityEvidence})
  | RejectVerifiedA4({id: AttemptId})
  | AdvanceA4({environment: Environment})
  | CancelParentProbeA4({parent: A4Parent, entry: A4Entry, prepared: A4Entry})
  | ConsumeSlotProbeA4({parent: A4Parent, entry: A4Entry, slot: int, prepared: A4Entry})
  | FinancialProbeA4({operation: Operation, effects: List[Transfer]})
  | CoreAcceptedProbeA4({request: AAuthorityRequest})
  | PlanMatchesProbeA4({plan: AResolvedPlan})
  | DeriveA4({mutationId: str, baseCaseId: str, baseSequence: int,
      attempt: AAuthorityAttempt, evidence: AAuthorityEvidence, stage: A4DerivationStage})
  | CaseStartA4(A4CaseDescriptor)
  | CaseEndA4({status: str})
  | DiagnosticA4(A4Diagnostic)
type A4Guard = InstallmentCommandGuardA4(CommandI) | SwapCommandGuardA4(CommandS)
  | PrepareGuardA4 | SignGuardA4 | ProposeGuardA4 | VerifyGuardA4 | CommitGuardA4
  | RejectProposedGuardA4 | RejectVerifiedGuardA4 | AdvanceGuardA4
  | CancelParentGuardA4 | ConsumeSlotGuardA4 | FinancialGuardA4
  | CoreAcceptedGuardA4 | PlanMatchesGuardA4 | NoGuardA4
type A4Arguments = {guard: A4Guard, command: A4Command}
type A4Extraction = ExtractionObservedA4(AEffectExtraction) | ExtractionNotApplicableA4
type A4Computation = {request: AAuthorityRequest, evaluation: ATransactionEvaluation,
  extraction: A4Extraction}
type A4Event = {caseId: str, profile: str, sequence: int, kind: str,
  arguments: A4Arguments, observedGuard: bool, computations: List[A4Computation]}
```

The final plan sections below define implementation and test blocks for these interfaces; a declaration/typecheck is not behavioral RED.

- [ ] Implement the finite enumeration in `scripts/s02_candidate_a_integrated_inventory.py`. `inventory()` returns JSON data; it never reads submitted cases to determine requirements. `instructions()` is producer-private generation input. Keep its string grammar literal and reject unknown selectors.

```python
from itertools import product

PROFILES = (("after", "SignAfterResolve"), ("before", "SignBeforeResolve"))
I_MODES = ("choice2", "timeout100", "timeout101", "refuse100", "refuse101")
I_CONTROLS = ("no-cancel", "unsigned", "old-nonce",
              "fresh-duplicate-cancel", "unused-successor")
S_MUTANTS = ("unused-successor", "reversed-effects", "reductions", "neutral-chooser")
S_CONTROLS = ("stale-signing", *S_MUTANTS, "second-plan",
              "wrong-Core-chooser", "wrong-signer", "wrong-nonce", "stale-facts")

def i_route(residual=None, mode=None):
    route = ["I:prepare-parent", "I:sign-parent", "I:propose:first",
             "I:propose:cancel", "I:verify:first", "I:verify:cancel"]
    if residual is None:
        return route + ["I:commit:first", "I:reject-stale:cancel",
                        "I:propose:second", "I:verify:second", "I:commit:second"]
    route += (["I:commit:first", "I:reject-stale:cancel"] if residual else
              ["I:commit:cancel", "I:reject-stale:first"])
    if residual:
        route += ["I:propose:fresh-cancel", "I:verify:fresh-cancel", "I:commit:fresh-cancel"]
    if mode != "choice2":
        route += ["I:advance"]
    route += ["I:prepare-recovery", "I:sign-recovery", "I:propose:recovery"]
    return route + (["I:reject-recovery"] if mode.startswith("refuse") else
                    ["I:verify:recovery", "I:commit:recovery"])

def s_funding(funded):
    route = []
    for number, owner in ((1, "Alice"), (2, "Bob")):
        if funded >= number:
            if number == 2:
                route += ["S:advance:2"]
            name = f"fund{number}"
            route += [f"S:prepare:{name}:{owner}", f"S:sign:{name}:{owner}",
                      f"S:propose:{name}", f"S:verify:{name}", f"S:commit:{name}"]
    return route

def s_route(funded, mode, time=2):
    route = s_funding(funded)
    if mode not in ("settle", "refund"):
        route += [f"S:advance:{time}"]
    if funded and mode != "refuse":
        for owner in ("Alice", "Bob")[:funded]:
            route += [f"S:prepare:disposition:{owner}", f"S:sign:disposition:{owner}"]
        return route + ["S:propose:disposition", "S:verify:disposition", "S:commit:disposition"]
    return route + ["S:propose:disposition", "S:reject-proposed:disposition"]

def replay(ids):
    return [f"P:{verb}:{name}" for name in ids for verb in ("propose", "commit")]

def scenarios():
    yield "installment", "two-fills", None, None, None
    for residual, mode in product((0, 1), I_MODES):
        yield "installment", f"recover-r{residual}-{mode}", residual, mode, None
    for mode in ("settle", "refund"):
        yield "swap", f"funded2-{mode}", 2, mode, 2
    for funded, time in product((0, 1, 2), (100, 101)):
        yield "swap", f"funded{funded}-timeout{time}", funded, "timeout", time
    for time, chosen in product((100, 101), (0, 1)):
        yield "swap", f"funded2-refuse{time}-choice{chosen}", 2, "refuse", time

def suffix(control, profile, lifecycle):
    if lifecycle == "installment":
        return {
            "no-cancel": ["D:prepare-recovery", "D:financial-recovery", "C:propose:recovery",
                                "D:verify:recovery:normal", "C:reject:recovery:normal"],
            "unsigned": ["C:propose:recovery", "D:verify:recovery:normal",
                                  "C:reject:recovery:normal", "D:prepare-parent"],
            "old-nonce": ["C:propose:recovery", "D:verify:recovery:parent",
                          "C:reject:recovery:parent"],
            "fresh-duplicate-cancel": ["C:duplicate-cancel", "D:verify:fresh-cancel:normal",
                                       "C:reject:fresh-cancel:normal"],
            "unused-successor": ["M:proposed", "D:verify:first:normal", "C:reject:first:normal",
                                 "M:verified", "P:commit:first", "C:reject-verified:first"],
        }[control]
    if control in S_MUTANTS:
        return ["M:proposed", "D:verify:disposition:normal", "C:reject:disposition:normal",
                "M:verified", "P:commit:disposition", "C:reject-verified:disposition"]
    return {
        "stale-signing": ["S:prepare:disposition:Alice", "S:advance:100", "D:sign-disposition"],
        "second-plan": ["D:plan", "D:bad-prepare" if profile == "after" else "C:bad-prepare"]
            + (["C:prepare:fund1:Alice"] if profile == "after" else [])
            + ["C:sign:fund1:Alice", "C:bad-plan-propose", "D:verify:fund1:normal", "C:reject:fund1:normal"],
        "wrong-Core-chooser": ["D:core"],
        "wrong-signer": ["D:verify:disposition:signer", "C:reject:disposition:signer"],
        "wrong-nonce": ["D:verify:disposition:nonce", "C:reject:disposition:nonce"],
        "stale-facts": ["M:proposed", "D:verify:disposition:normal", "C:reject:disposition:normal"],
    }[control]

def instructions():
    for lifecycle in ("installment", "swap"):
        for life, name, count, mode, time in scenarios():
            if life != lifecycle:
                continue
            route = i_route(count, mode) if life == "installment" else s_route(count, mode, time)
            if life == "installment":
                ids = (["first", "second"] if count is None else
                       (["first", "fresh-cancel"] if count else ["cancel"]) +
                       ([] if mode.startswith("refuse") else ["recovery"]))
                tail = replay(ids) + ([] if count is None else ["P:cancel-parent", "P:slot:1", "P:slot:2"])
            else:
                tail = replay([f"fund{i}" for i in range(1, count + 1)] + ["disposition"])
            for short, profile in PROFILES:
                yield descriptor(life, name, "ordinary", short, profile), ["B:start", *route, *tail, "B:end"]
        if lifecycle == "swap":
            stale = s_route(2, "settle")[:17] + ["S:advance:100", "P:commit:disposition", "S:reject-verified:disposition"]
            for short, profile in PROFILES:
                yield descriptor(lifecycle, "funded2-settle", "verified-stale", short, profile), ["B:start", *stale, "B:end"]
        for control in I_CONTROLS if lifecycle == "installment" else S_CONTROLS:
            scenario = ("two-fills" if control == "unused-successor" else "recover-r0-choice2") if lifecycle == "installment" else "funded2-settle"
            for short, profile in PROFILES:
                if lifecycle == "installment":
                    prefix = i_route()[:4] if control == "unused-successor" else i_route(0, "choice2")[:2 if control == "no-cancel" else 8]
                else:
                    count = 0 if control == "second-plan" else 11 if control in ("stale-signing", "wrong-Core-chooser") else 16
                    prefix = s_route(2, "settle")[:count]
                yield descriptor(lifecycle, scenario, control, short, profile), ["B:start", *prefix, *suffix(control, short, lifecycle), "B:end"]

def descriptor(lifecycle, scenario, control, short, profile):
    return {"case_id": f"{lifecycle}/{scenario}/{control}/{profile}", "lifecycle": lifecycle,
            "profile": profile, "scenario": scenario, "control": control}

def inventory():
    cases = [{**d, "event_count": len(steps)} for d, steps in instructions()]
    assert len(cases) == 78
    assert sum(c["event_count"] for c in cases) == 1557
    assert len({c["case_id"] for c in cases}) == len(cases)
    return {"schema_version": 2, "cases": cases}
```

`selector` is the fixed producer/checker event grammar, not a replacement for full raw command arguments. A selector cannot authorize extra events or alter the independently checked fullargs.

- [ ] Add these inventory tests before the enumerator body (initial typed/importable body returns `{"schema_version":2,"cases":[]}`). Preserve the resulting assertion RED, then implement the body above.

```python
from scripts.s02_candidate_a_integrated_inventory import inventory, instructions

def test_exact_inventory():
    cases = inventory()["cases"]
    assert len(cases) == 78
    assert sum(c["event_count"] for c in cases) == 1557
    assert sum(c["lifecycle"] == "installment" for c in cases) == 32
    assert sum(c["event_count"] for c in cases if c["lifecycle"] == "installment") == 638
    assert sum(c["event_count"] for c in cases if c["lifecycle"] == "swap") == 919
    for index in range(0, 78, 2):
        assert cases[index]["profile"] == "SignAfterResolve"
        assert cases[index + 1]["profile"] == "SignBeforeResolve"
        assert cases[index]["scenario"] == cases[index + 1]["scenario"]
        assert cases[index]["control"] == cases[index + 1]["control"]

def test_mutant_dual_boundaries_and_original_bases():
    selected = [(d, steps) for d, steps in instructions() if
                d["control"] in ("unused-successor", "reversed-effects", "reductions", "neutral-chooser")]
    assert len(selected) == 10
    for desc, steps in selected:
        assert steps.count("M:proposed") == steps.count("M:verified") == 1
        assert steps.index("M:proposed") < steps.index("M:verified")
        base = 3 if desc["lifecycle"] == "installment" else 16
        assert steps[base] in ("I:propose:first", "S:propose:disposition")
        assert steps[steps.index("M:verified") - 1].startswith("C:reject:")
```

Run RED and GREEN with the recorder defined in Task 0:
`/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/inventory-red/python-cache scripts/record_s02_candidate_a_integrated.py --stage inventory-red -- /home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -q -k 'exact_inventory or mutant_dual'`.
The red command must terminate nonzero on the assertions, not on import or syntax failure. Repeat as `--stage inventory-green` after implementation. Root compares the generated 78-case/1,557-selector manifest with the checker's independent enumeration and adopts the literal bytes before actual export.

### Observer computation implementation and behavioral test

Use these exact imports at the start of the observer and test modules. The Task1 test header additionally imports `candidate_a_integrated_observer.* from "./candidate_a_integrated_observer"` immediately, before its first tests; do not defer that dependency to Task2. The complete Task1 test file consists of the opening line `module candidate_a_integrated_export_test {`, the displayed imports plus that observer import, the four displayed Task1 run definitions, and the closing line `}`.

```quint
import effects.* from "./effects"
import consumption.* from "./consumption"
import observations.* from "./observations"
import policies.* from "./policies"
import authorization.* from "./authorization"
import execution.* from "./execution"
import candidate_a_types.* from "./candidate_a_types"
import candidate_a_programs.* from "./candidate_a_programs"
import candidate_a_core.* from "./candidate_a_core"
import candidate_a_projection.* from "./candidate_a_projection"
import candidate_a_authority_adapter.* from "./candidate_a_authority_adapter"
import candidate_a_authority_boundary.* from "./candidate_a_authority_boundary"
import candidate_a_authority_installment_fixtures.* from "./candidate_a_authority_installment_fixtures"
import candidate_a_authority_installment.* from "./candidate_a_authority_installment"
import candidate_a_authority_swap_fixtures.* from "./candidate_a_authority_swap_fixtures"
import candidate_a_authority_swap.* from "./candidate_a_authority_swap"
```

- [ ] Add typed stub `pure def callComputationsA4(call: ACall): List[A4Computation] = List()` and this test. Preserve full import closure and an actual compiling assertion RED before replacing the stub.

```quint
run actualDepositObservationTest = assert({
  val request = {before: beforeS(0), input: PresentAInput(DepositInputA({
    account: aliceA, depositor: Alice, quantity: 10})), now: Time1}
  val recorded = callComputationsA4(AgreementCallA(request))
  if (recorded.length() != 1) false else recorded.nth(0).request == request
    and recorded.nth(0).evaluation == computeTransaction(programS, request.before.state, request.input, Time1)
    and match recorded.nth(0).evaluation {
      | TransactionComputedA(raw) => raw.accepted and raw.payments == List()
          and recorded.nth(0).extraction == ExtractionObservedA4(EffectsExtractedA(List({
            source: Wallet(Alice), destination: Escrow(aliceA), asset: TokenA, quantity: 10})))
      | _ => false }
})
run cancellationHasNoComputationTest = assert(
  callComputationsA4(CancellationCallA({before: initialBeforeI, now: Time2})) == List())
```

- [ ] Implement the observer below. The exact record multiplicity is: Prepare/Sign AfterResolution `binding.identity.operations` in order, BeforeResolution none; Propose its supplied observation call; Verify/RejectProposed the retained proposed attempt call; Commit/RejectVerified the retained attempt call; Derive its resulting claimed attempt call; PlanMatches all plan operations in order; CoreAccepted exactly its supplied request; all other commands none. Skip cancellation calls. Do not deduplicate repeated requests. Outer plan lists and all evidence policy lists remain raw for the independent checker to compare separately.

```quint
type A4AttemptView = MissingAttemptA4 | PresentAttemptA4(AAuthorityAttempt)
pure def attemptA4(state: AAuthorityExecution, id: AttemptId): A4AttemptView = match state.attempts.get(id) {
  | NoAttempt => MissingAttemptA4
  | ProposedAttempt(a) => PresentAttemptA4(a)
  | VerifiedOperation(v) => PresentAttemptA4(v.attempt)
  | ExecutedOperation(v) => PresentAttemptA4(v.attempt)
  | RejectedOperation(r) => PresentAttemptA4(r.attempt)
}
pure def computeA4(request: AAuthorityRequest): A4Computation = {
  val evaluation = computeTransaction(request.before.program, request.before.state, request.input, request.now)
  {request: request, evaluation: evaluation, extraction: match evaluation {
    | TransactionComputedA(raw) => ExtractionObservedA4(extractCommittedEffects(request.before.program,
        request.before.state, request.input, request.now, raw))
    | _ => ExtractionNotApplicableA4 }}
}
pure def callComputationsA4(call: ACall): List[A4Computation] = match call {
  | CancellationCallA(_) => List()
  | AgreementCallA(request) => List(computeA4(request))
}
pure def planComputationsA4(plan: AResolvedPlan): List[A4Computation] =
  plan.operations.foldl(List(), (out, op) => out.concat(callComputationsA4(op.artifactAndCall)))
pure def policyComputationsA4(policy: AAuthorityPolicy): List[A4Computation] = match policy.binding {
  | BeforeResolution(_) => List()
  | AfterResolution(view) => planComputationsA4(view.identity)
}
pure def retainedComputationsA4(state: AAuthorityExecution, id: AttemptId): List[A4Computation] =
  match attemptA4(state, id) {
    | MissingAttemptA4 => List()
    | PresentAttemptA4(a) => callComputationsA4(a.observation.artifactAndCall)
  }
pure def computationsA4(state: AAuthorityExecution, command: A4Command): List[A4Computation] = match command {
  | PrepareA4(c) => policyComputationsA4(c.policy)
  | SignA4(c) => policyComputationsA4(c.policy)
  | ProposeA4(c) => callComputationsA4(c.observation.artifactAndCall)
  | VerifyA4(c) => retainedComputationsA4(state, c.id)
  | CommitA4(c) => retainedComputationsA4(state, c.id)
  | RejectProposedA4(c) => retainedComputationsA4(state, c.id)
  | RejectVerifiedA4(c) => retainedComputationsA4(state, c.id)
  | DeriveA4(c) => callComputationsA4(c.attempt.observation.artifactAndCall)
  | CoreAcceptedProbeA4(c) => List(computeA4(c.request))
  | PlanMatchesProbeA4(c) => planComputationsA4(c.plan)
  | DiagnosticA4(detail) => match detail {
      | ObserverIssueA4(_) => List() | AdapterIssueA4(issue) => callComputationsA4(issue.call) }
  | _ => List()
}
pure def computedA4(c: A4Computation): bool = match c.evaluation {
  | TransactionComputedA(_) => match c.extraction {
      | ExtractionObservedA4(EffectsExtractedA(_)) => true
      | _ => false }
  | _ => false
}
pure def directTagA4(command: A4Command): A4Guard = match command {
  | PrepareA4(_) => PrepareGuardA4 | SignA4(_) => SignGuardA4
  | ProposeA4(_) => ProposeGuardA4 | VerifyA4(_) => VerifyGuardA4
  | CommitA4(_) => CommitGuardA4 | RejectProposedA4(_) => RejectProposedGuardA4
  | RejectVerifiedA4(_) => RejectVerifiedGuardA4 | AdvanceA4(_) => AdvanceGuardA4
  | CancelParentProbeA4(_) => CancelParentGuardA4 | ConsumeSlotProbeA4(_) => ConsumeSlotGuardA4
  | FinancialProbeA4(_) => FinancialGuardA4 | CoreAcceptedProbeA4(_) => CoreAcceptedGuardA4
  | PlanMatchesProbeA4(_) => PlanMatchesGuardA4 | _ => NoGuardA4
}
pure def directA4(command: A4Command): A4Arguments = {guard: directTagA4(command), command: command}
pure def directGuardA4(state: AAuthorityExecution, command: A4Command): bool = match command {
  | PrepareA4(c) => canPrepareSigningAuthorityA(state.authority, c.policy, c.signer)
  | SignA4(c) => canSignAuthorityA(state.authority, c.policy, c.signer, c.token)
  | ProposeA4(c) => canPropose(state, c.id, c.operation, c.observation, c.actor)
  | VerifyA4(c) => canVerifyAuthorityA(state, c.id, c.evidence)
  | CommitA4(c) => canCommitAuthorityA(state, c.id)
  | RejectProposedA4(c) => canRejectProposedAuthorityA(state, c.id, c.evidence)
  | RejectVerifiedA4(c) => canRejectVerifiedAuthorityA(state, c.id)
  | AdvanceA4(c) => validAuthorityExecutionA(state) and validEnvironment(c.environment)
      and c.environment.physicalTime > state.authority.context.environment.physicalTime
  | CancelParentProbeA4(c) => canCancelParent(c.parent, c.entry, c.prepared)
  | ConsumeSlotProbeA4(c) => canConsumeSlot(c.parent, c.entry, c.slot, c.prepared)
  | FinancialProbeA4(c) => operationFinancialGuard(state.authority.context, c.operation, c.effects)
  | PlanMatchesProbeA4(c) => authorityPlanMatchesA(c.plan)
  | CoreAcceptedProbeA4(c) => match computeA4(c.request).evaluation {
      | TransactionComputedA(r) => r.accepted | _ => false }
  | DeriveA4(c) => c.attempt.id == (if (c.baseSequence == 3) FirstFillAttempt else DispositionAttempt)
      and match state.attempts.get(c.attempt.id) {
        | ProposedAttempt(_) => c.stage == ProposedDerivationA4
        | RejectedOperation(r) => c.stage == VerifiedDerivationA4
            and r.stage == VerificationBoundary and r.attempt == c.attempt and r.evidence == c.evidence
        | _ => false }
  | CaseStartA4(_) => true | CaseEndA4(_) => true | DiagnosticA4(_) => false
}
pure def directApplyA4(state: AAuthorityExecution, command: A4Command): AAuthorityExecution = match command {
  | PrepareA4(c) => {...state, authority: applyPrepareSigning(state.authority, c.policy, c.signer)}
  | SignA4(c) => {...state, authority: applySign(state.authority, c.policy, c.signer, c.token)}
  | ProposeA4(c) => applyPropose(state, c.id, c.operation, c.observation, c.actor)
  | VerifyA4(c) => applyVerify(state, c.id, c.evidence)
  | CommitA4(c) => applyCommit(state, c.id)
  | RejectProposedA4(c) => applyRejectProposedAuthorityA(state, c.id, c.evidence)
  | RejectVerifiedA4(c) => applyRejectVerifiedAuthorityA(state, c.id)
  | AdvanceA4(c) => {...state, authority: {...state.authority,
      context: {...state.authority.context, environment: c.environment}}}
  | DeriveA4(c) => {...state, attempts: state.attempts.put(c.attempt.id,
      if (c.stage == ProposedDerivationA4) ProposedAttempt(c.attempt)
      else VerifiedOperation({attempt: c.attempt, evidence: c.evidence}))}
  | _ => state
}
```

The direct derivation guard is a representability check, not an independent mutation validator. `mutatedA4` below constructs the exact finite mutation; the checker independently compares it to the original base position and rejects all other changes.

- [ ] Add diagnostic coverage without inventing Core failure:

```quint
run diagnosticRemainsDiagnosticTest = assert({
  val bad = {...beforeS(0), state: {...beforeS(0).state,
    accounts: beforeS(0).state.accounts.put(aliceA, -1)}}
  val c = computeA4({before: bad, input: NoAInput, now: Time1})
  c.evaluation == OutsideModelDomainA and c.extraction == ExtractionNotApplicableA4 and not(computedA4(c))
})
run planComputationOrderTest = assert({
  val records = planComputationsA4(parentPlanI)
  records.length() == 2
    and records.nth(0).request.before == initialBeforeI
    and records.nth(1).request.before == residualBeforeI
    and records.foldl(true, (ok, c) => ok and computedA4(c))
})
```

Run one recursive test-module typecheck and `quint test specs/quint/s02/candidate_a_integrated_export_test.qnt --backend=rust --seed=42 --match '.*Test'`, recorded as distinct immutable `observer-types` and `observer-green` stages. Do not call supplemental tests an original RED if they were first run after implementation.

## Task 2: Exact command lowering, controls and actual batch execution

**Files:** Complete observer; create literal cases, parameterized driver and two instantiations; extend Quint tests. No frozen module changes.

**Consumes:** Task1 typed observer, root-adopted 78-case inventory, accepted lifecycle route lists.

**Produces:** `buildA4(state,c,step):A4Arguments`, `guardA4`, `applyA4`, exact generated `INSTALLMENT_CASES_A4/SWAP_CASES_A4`, deterministic `init/step`, `completeA4/noDiagnosticA4`.

- [ ] Add these step/config types and exact helpers:

```quint
type A4EvidenceMode = NormalEvidenceA4 | ParentEvidenceA4 | WrongSignerEvidenceA4 | WrongNonceEvidenceA4
type A4Instruction = LifecycleIA4(CommandI) | LifecycleSA4(CommandS)
  | DirectLifecycleIA4(CommandI) | DirectLifecycleSA4(CommandS)
  | ReplayProposalA4(AttemptId) | ReplayCommitA4(AttemptId)
  | ParentCancellationA4 | ParentSlotA4(int) | RecoveryFinancialA4 | DuplicateCancellationA4
  | VerifyEvidenceA4({id: AttemptId, mode: A4EvidenceMode})
  | RejectEvidenceA4({id: AttemptId, mode: A4EvidenceMode})
  | RejectConstructedA4(AttemptId) | MutateA4(A4DerivationStage)
  | StaleSignA4 | BadPlanA4 | BadPrepareA4 | BadProposeA4 | WrongCoreA4
type A4Step = {kind: str, instruction: A4Instruction}
type A4Case = {descriptor: A4CaseDescriptor, scenario: A4Scenario,
  profile: SigningProfile, steps: List[A4Step]}
pure def diagnosticA4(message: str): A4Arguments = directA4(DiagnosticA4(ObserverIssueA4(message)))
pure def proposeAdaptedA4(id: AttemptId, op: Operation, actor: Principal,
  call: ACall, adaptation: AAuthorityAdaptation): A4Command = match adaptation {
    | AuthorityAdaptedA(obs) => ProposeA4({id: id, operation: op, observation: obs, actor: actor})
    | _ => DiagnosticA4(AdapterIssueA4({call: call, adaptation: adaptation}))
  }
pure def caseEvidenceA4(c: A4Case, a: AAuthorityAttempt, mode: A4EvidenceMode): AAuthorityEvidence = {
  val normal = match c.scenario {
    | InstallmentScenarioA4(s) => evidenceI(a, if (mode == ParentEvidenceA4)
        parentPolicyI(c.profile) else selectedPolicyI(a.id, s, c.profile))
    | SwapScenarioA4(s) => evidenceS(a, s, c.profile) }
  val key = keyS(Alice, 1)
  if (mode == WrongSignerEvidenceA4) {...normal, signatures: normal.signatures.put(key,
    {...normal.signatures.get(key), signed: {...normal.signatures.get(key).signed, signer: Mallory}})}
  else if (mode == WrongNonceEvidenceA4) {...normal, signatures:
    normal.signatures.keys().exclude(Set(key)).mapBy(k => normal.signatures.get(k)).put(keyS(Alice, 0),
      {...normal.signatures.get(key), signed: {policy: policyS(FundingOneAttempt, staleScenarioS, Alice, c.profile),
        signer: Alice, token: 0}})}
  else normal
}
pure def lowerIA4(state: AAuthorityExecution, c: A4Case, s: ScenarioI, cmd: CommandI): A4Command = match cmd {
  | PrepareParentI => PrepareA4({policy: parentPolicyI(c.profile), signer: Alice})
  | SignParentI => SignA4({policy: parentPolicyI(c.profile), signer: Alice, token: 0})
  | PrepareRecoveryI => PrepareA4({policy: recoveryPolicyI(s, c.profile), signer: Alice})
  | SignRecoveryI => SignA4({policy: recoveryPolicyI(s, c.profile), signer: Alice, token: 0})
  | ProposeI(id) => proposeAdaptedA4(id, plannedForI(id, s).operation,
      if (Set(FirstFillAttempt, SecondFillAttempt).contains(id)) Bob else Alice, plannedForI(id, s).artifactAndCall, actualI(id, s))
  | VerifyI(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => VerifyA4({id: id, evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing verify attempt")) }
  | CommitI(id) => CommitA4({id: id})
  | RejectStaleI(id) => RejectVerifiedA4({id: id})
  | AdvanceI => AdvanceA4({environment: environmentI(timeValue(timeI(modeI(s))))})
  | RejectRecoveryI => match attemptA4(state, RecoveryAttempt) {
      | PresentAttemptA4(a) => RejectProposedA4({id: RecoveryAttempt,
          evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing recovery attempt")) }
}
pure def lowerSA4(state: AAuthorityExecution, c: A4Case, s: ScenarioS, cmd: CommandS): A4Command = match cmd {
  | PrepareS(x) => PrepareA4({policy: policyS(x.id, s, x.principal, c.profile), signer: x.principal})
  | SignS(x) => SignA4({policy: policyS(x.id, s, x.principal, c.profile), signer: x.principal, token: 0})
  | ProposeS(id) => proposeAdaptedA4(id, plannedS(id, s).operation,
      if (id == FundingOneAttempt) Alice else Bob, plannedS(id, s).artifactAndCall, actualS(id, s))
  | VerifyS(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => VerifyA4({id: id, evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing verify attempt")) }
  | CommitS(id) => CommitA4({id: id})
  | AdvanceS(t) => AdvanceA4({environment: environmentS(timeValue(t))})
  | RejectProposedS(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => RejectProposedA4({id: id, evidence: caseEvidenceA4(c, a, NormalEvidenceA4)})
      | _ => DiagnosticA4(ObserverIssueA4("missing rejected attempt")) }
  | RejectVerifiedS(id) => RejectVerifiedA4({id: id})
}
pure val badPlanA4: AResolvedPlan = {
  val original = fundingPlannedS(Alice)
  {identity: SwapPlanA, operations: List(original, {...original, proposedSuccessor: beforeS(0)})}
}
pure def badPolicyA4(p: SigningProfile): AAuthorityPolicy = {
  val original = policyS(FundingOneAttempt, staleScenarioS, Alice, p)
  {...original, binding: if (p == SignAfterResolve)
    AfterResolution({identity: badPlanA4, operations: badPlanA4.operations})
    else BeforeResolution(AnyArtifactUnderMechanism)}
}
pure def mutatedA4(a: AAuthorityAttempt, mutation: str): AAuthorityAttempt = {
  val obs = a.observation
  val changed = if (mutation == "unused-successor") {...obs, proposedSuccessor: {...obs.proposedSuccessor,
    program: {...obs.proposedSuccessor.program, nodes: obs.proposedSuccessor.program.nodes.put(N15,
      PayA({account: aliceA, payee: Bob, amount: ConstantA(5), continuation: N0}))}}}
    else if (mutation == "reversed-effects") {...obs, effects: List(obs.effects.nth(1), obs.effects.nth(0))}
    else if (mutation == "reductions") {...obs, coreProjection: match obs.coreProjection {
      | CoreProjected(r) => CoreProjected({...r, reductions: r.reductions + 1})
      | NoCoreProjection => NoCoreProjection }}
    else if (mutation == "neutral-chooser") {...obs, input: ChoiceLike({id: "settle", chooser: Alice, chosen: 1})}
    else {...obs, resolvedPlan: {...obs.resolvedPlan, operations: List({
      ...obs.resolvedPlan.operations.nth(0), predecessorFacts: {
        ...obs.resolvedPlan.operations.nth(0).predecessorFacts, environment: environmentS(100)}})}}
  {...a, observation: changed}
}
```

`mutatedA4` is callable only for the five fixed literal control names (the last branch is stale-facts). The Python selector renderer rejects every other name. No mutation changes the honest artifact request; honest computations remain distinct from forged claims.

- [ ] Complete command lowering and observation with the following blocks:

```quint
pure def evidenceCommandA4(state: AAuthorityExecution, c: A4Case, id: AttemptId,
  mode: A4EvidenceMode, rejecting: bool): A4Command = match attemptA4(state, id) {
    | PresentAttemptA4(a) => if (rejecting) RejectProposedA4({id: id, evidence: caseEvidenceA4(c, a, mode)})
        else VerifyA4({id: id, evidence: caseEvidenceA4(c, a, mode)})
    | _ => DiagnosticA4(ObserverIssueA4("missing evidence attempt"))
  }
pure def buildA4(state: AAuthorityExecution, c: A4Case, instruction: A4Instruction): A4Arguments = match instruction {
  | LifecycleIA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => {guard: InstallmentCommandGuardA4(cmd), command: lowerIA4(state, c, s, cmd)}
      | _ => diagnosticA4("foreign installment command") }
  | LifecycleSA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => {guard: SwapCommandGuardA4(cmd), command: lowerSA4(state, c, s, cmd)}
      | _ => diagnosticA4("foreign swap command") }
  | DirectLifecycleIA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => directA4(lowerIA4(state, c, s, cmd))
      | _ => diagnosticA4("foreign direct installment command") }
  | DirectLifecycleSA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => directA4(lowerSA4(state, c, s, cmd))
      | _ => diagnosticA4("foreign direct swap command") }
  | ReplayProposalA4(id) => match attemptA4(state, id) {
      | PresentAttemptA4(a) => directA4(ProposeA4({id: id, operation: a.operation, observation: a.observation, actor: a.actor}))
      | _ => diagnosticA4("missing replay attempt") }
  | ReplayCommitA4(id) => directA4(CommitA4({id: id}))
  | ParentCancellationA4 => match state.authority.context.parents.get(parentKeyI) {
      | ParentLive(live) => directA4(CancelParentProbeA4({parent: live.parent, entry: live.entry, prepared: live.entry}))
      | _ => diagnosticA4("missing cancellation parent") }
  | ParentSlotA4(slot) => match state.authority.context.parents.get(parentKeyI) {
      | ParentLive(live) => directA4(ConsumeSlotProbeA4({parent: live.parent, entry: live.entry, slot: slot, prepared: live.entry}))
      | _ => diagnosticA4("missing slot parent") }
  | RecoveryFinancialA4 => match actualI(RecoveryAttempt, RecoverI({residual: false, mode: Choice2I})) {
      | AuthorityAdaptedA(obs) => directA4(FinancialProbeA4({operation: recoverI, effects: obs.effects}))
      | other => directA4(DiagnosticA4(AdapterIssueA4({call: plannedForI(RecoveryAttempt,
          RecoverI({residual: false, mode: Choice2I})).artifactAndCall, adaptation: other}))) }
  | DuplicateCancellationA4 => directA4(proposeAdaptedA4(FreshCancelAttempt, cancelI, Alice, cancel0PlannedI.artifactAndCall,
      actualI(CancelAttempt, RecoverI({residual: false, mode: Choice2I}))))
  | VerifyEvidenceA4(x) => directA4(evidenceCommandA4(state, c, x.id, x.mode, false))
  | RejectEvidenceA4(x) => directA4(evidenceCommandA4(state, c, x.id, x.mode, true))
  | RejectConstructedA4(id) => directA4(RejectVerifiedA4({id: id}))
  | MutateA4(stage) => {
      val id = if (c.descriptor.lifecycle == "installment") FirstFillAttempt else DispositionAttempt
      match attemptA4(state, id) {
        | PresentAttemptA4(a) => {
            val changed = if (stage == ProposedDerivationA4) mutatedA4(a, c.descriptor.control) else a
            directA4(DeriveA4({mutationId: c.descriptor.control, baseCaseId: c.descriptor.caseId,
              baseSequence: if (id == FirstFillAttempt) 3 else 16, attempt: changed,
              evidence: caseEvidenceA4(c, changed, NormalEvidenceA4), stage: stage}))
          }
        | _ => diagnosticA4("missing mutation base") }
    }
  | StaleSignA4 => directA4(SignA4({policy: policyS(DispositionAttempt, staleScenarioS, Alice, c.profile), signer: Alice, token: 0}))
  | BadPlanA4 => directA4(PlanMatchesProbeA4({plan: badPlanA4}))
  | BadPrepareA4 => directA4(PrepareA4({policy: badPolicyA4(c.profile), signer: Alice}))
  | BadProposeA4 => match actualS(FundingOneAttempt, staleScenarioS) {
      | AuthorityAdaptedA(obs) => directA4(ProposeA4({id: FundingOneAttempt, operation: OpFund,
          observation: {...obs, resolvedPlan: badPlanA4}, actor: Alice}))
      | other => directA4(DiagnosticA4(AdapterIssueA4({call: fundingPlannedS(Alice).artifactAndCall, adaptation: other}))) }
  | WrongCoreA4 => directA4(CoreAcceptedProbeA4({request: {before: beforeS(2),
      input: PresentAInput(ChoiceInputA({id: SettleId, chooser: Alice, chosen: 1})), now: Time2}}))
}
pure def guardA4(state: AAuthorityExecution, c: A4Case, args: A4Arguments): bool = match args.guard {
  | InstallmentCommandGuardA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => args.command == lowerIA4(state, c, s, cmd) and canCommandI(state, s, c.profile, cmd)
      | _ => false }
  | SwapCommandGuardA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => args.command == lowerSA4(state, c, s, cmd) and canCommandS(state, s, c.profile, cmd)
      | _ => false }
  | _ => args.guard == directTagA4(args.command) and directGuardA4(state, args.command)
}
pure def applyA4(state: AAuthorityExecution, c: A4Case, args: A4Arguments): AAuthorityExecution = match args.guard {
  | InstallmentCommandGuardA4(cmd) => match c.scenario {
      | InstallmentScenarioA4(s) => applyCommandI(state, s, c.profile, cmd) | _ => state }
  | SwapCommandGuardA4(cmd) => match c.scenario {
      | SwapScenarioA4(s) => applyCommandS(state, s, c.profile, cmd) | _ => state }
  | _ => directApplyA4(state, args.command)
}
pure def initialA4(c: A4Case): AAuthorityExecution = match c.scenario {
  | InstallmentScenarioA4(_) => unsignedI | SwapScenarioA4(_) => unsignedS }
pure def observeA4(state: AAuthorityExecution, c: A4Case, sequence: int, step: A4Step): A4Event = {
  val args = buildA4(state, c, step.instruction)
  val actualGuard = guardA4(state, c, args)
  val records = computationsA4(state, args.command)
  val diagnostic = match args.command { | DiagnosticA4(_) => true | _ => false }
  val expectedGuard = step.kind != "denied-probe"
  {caseId: c.descriptor.caseId, profile: c.descriptor.profile, sequence: sequence,
    kind: if (diagnostic or actualGuard != expectedGuard or not(records.foldl(true, (ok, x) => ok and computedA4(x))))
      "diagnostic" else step.kind, arguments: args, observedGuard: actualGuard, computations: records}
}
pure def startA4(c: A4Case): A4Event = {caseId: c.descriptor.caseId, profile: c.descriptor.profile,
  sequence: 0, kind: "case-start", arguments: directA4(CaseStartA4(c.descriptor)), observedGuard: true, computations: List()}
pure def financiallyDoneA4(state: AAuthorityExecution, c: A4Case): bool = match c.scenario {
  | InstallmentScenarioA4(_) => financialTerminalI(state)
  | SwapScenarioA4(_) => financialTerminalS(state) }
pure def retainedRefusalA4(state: AAuthorityExecution, c: A4Case): bool = {
  val id = if (c.descriptor.lifecycle == "installment") RecoveryAttempt else DispositionAttempt
  val noPending = if (c.descriptor.lifecycle == "installment") noPendingI(state) else noPendingS(state)
  noPending and match state.attempts.get(id) {
    | RejectedOperation(r) => r.stage == VerificationBoundary and
        (match r.reason { | CoreRejected(_) => true | UnauthorizedEffect => c.descriptor.lifecycle == "swap" | _ => false })
    | _ => false }
}
pure def negativeDoneA4(state: AAuthorityExecution, previous: A4Event): bool =
  (previous.kind == "denied-probe" and not(previous.observedGuard))
    or (previous.kind == "transition" and previous.observedGuard and match previous.arguments.command {
      | RejectProposedA4(c) => match state.attempts.get(c.id) { | RejectedOperation(_) => true | _ => false }
      | RejectVerifiedA4(c) => match state.attempts.get(c.id) { | RejectedOperation(_) => true | _ => false }
      | _ => false })
pure def endA4(state: AAuthorityExecution, c: A4Case, previous: A4Event): A4Event = {
  val ordinary = c.descriptor.control == "ordinary"
  val financial = financiallyDoneA4(state, c)
  val ready = if (ordinary) financial or retainedRefusalA4(state, c) else negativeDoneA4(state, previous)
  {caseId: c.descriptor.caseId, profile: c.descriptor.profile, sequence: c.steps.length() + 1,
    kind: if (ready) "case-end" else "diagnostic", observedGuard: ready, computations: List(),
    arguments: directA4(CaseEndA4({status: if (not(ordinary)) "negative-complete"
      else if (financial) "financial-terminal" else "refusal-terminal"}))}
}
```

`negativeDoneA4` is a local observed-terminal check. The accepted **full** selector sequence additionally proves that each negative ends at its required denial/rejection and does not omit preceding events. Neither check substitutes for independent replay.

- [ ] Implement literal case-table rendering by adding this code to the inventory module. Generation uses `apply_patch` at execution time: inspect `quint_cases()` output, then add those exact bytes as `candidate_a_integrated_cases.qnt`; do not silently regenerate frozen case tables during export.

```python
import json

IDS = {"first": "FirstFillAttempt", "second": "SecondFillAttempt", "cancel": "CancelAttempt",
       "fresh-cancel": "FreshCancelAttempt", "recovery": "RecoveryAttempt", "fund1": "FundingOneAttempt",
       "fund2": "FundingTwoAttempt", "disposition": "DispositionAttempt"}
MODES = dict(zip(I_MODES, ("Choice2I", "Timeout100I", "Timeout101I", "Refuse100I", "Refuse101I")))
EMODES = {"normal": "NormalEvidenceA4", "parent": "ParentEvidenceA4",
          "signer": "WrongSignerEvidenceA4", "nonce": "WrongNonceEvidenceA4"}

def quint_instruction(selector):
    parts = selector.split(":")
    head, verb = parts[:2]
    if head == "I":
        simple = {"prepare-parent": "PrepareParentI", "sign-parent": "SignParentI",
                  "prepare-recovery": "PrepareRecoveryI", "sign-recovery": "SignRecoveryI",
                  "advance": "AdvanceI", "reject-recovery": "RejectRecoveryI"}
        cmd = simple.get(verb)
        if cmd is None:
            tag = {"propose": "ProposeI", "verify": "VerifyI", "commit": "CommitI", "reject-stale": "RejectStaleI"}[verb]
            cmd = f"{tag}({IDS[parts[2]]})"
        return f"LifecycleIA4({cmd})"
    if head == "S":
        if verb in ("prepare", "sign"):
            cmd = f"{'PrepareS' if verb == 'prepare' else 'SignS'}({{id: {IDS[parts[2]]}, principal: {parts[3]}}})"
        elif verb == "advance":
            cmd = f"AdvanceS(Time{parts[2]})"
        else:
            tag = {"propose": "ProposeS", "verify": "VerifyS", "commit": "CommitS",
                   "reject-proposed": "RejectProposedS", "reject-verified": "RejectVerifiedS"}[verb]
            cmd = f"{tag}({IDS[parts[2]]})"
        return f"LifecycleSA4({cmd})"
    if head == "P":
        if verb == "cancel-parent": return "ParentCancellationA4"
        if verb == "slot": return f"ParentSlotA4({int(parts[2])})"
        return f"{'ReplayProposalA4' if verb == 'propose' else 'ReplayCommitA4'}({IDS[parts[2]]})"
    if head == "M":
        return f"MutateA4({'ProposedDerivationA4' if verb == 'proposed' else 'VerifiedDerivationA4'})"
    if head == "C" and verb == "propose":
        return f"DirectLifecycleIA4(ProposeI({IDS[parts[2]]}))"
    if head == "C" and verb in ("prepare", "sign"):
        tag = "PrepareS" if verb == "prepare" else "SignS"
        return f"DirectLifecycleSA4({tag}({{id: {IDS[parts[2]]}, principal: {parts[3]}}}))"
    simple = {"prepare-recovery": "LifecycleIA4(PrepareRecoveryI)", "prepare-parent": "DirectLifecycleIA4(PrepareParentI)",
              "financial-recovery": "RecoveryFinancialA4", "duplicate-cancel": "DuplicateCancellationA4",
              "sign-disposition": "StaleSignA4", "plan": "BadPlanA4", "bad-prepare": "BadPrepareA4",
              "bad-plan-propose": "BadProposeA4", "core": "WrongCoreA4"}
    if verb in simple: return simple[verb]
    if verb == "reject-verified": return f"RejectConstructedA4({IDS[parts[2]]})"
    if verb in ("verify", "reject"):
        tag = "VerifyEvidenceA4" if verb == "verify" else "RejectEvidenceA4"
        return f"{tag}({{id: {IDS[parts[2]]}, mode: {EMODES[parts[3]]}}})"
    raise ValueError(f"unknown selector {selector}")

def quint_scenario(desc):
    name = desc["scenario"]
    if desc["lifecycle"] == "installment":
        if name == "two-fills": return "InstallmentScenarioA4(TwoFillsI)"
        _, residual, mode = name.split("-", 2)
        return f"InstallmentScenarioA4(RecoverI({{residual: {'true' if residual == 'r1' else 'false'}, mode: {MODES[mode]}}}))"
    funded = int(name[6])
    mode = name.split("-", 1)[1]
    if mode == "settle": value = "SettleS"
    elif mode == "refund": value = "RefundS"
    elif mode.startswith("timeout"): value = f"TimeoutS(Time{mode[7:]})"
    else: value = f"RefuseS({{now: Time{mode[6:9]}, chosen: {mode[-1]}}})"
    return f"SwapScenarioA4({{funded: {funded}, mode: {value}}})"

def quint_case(desc, steps):
    descriptor = "{" + ", ".join(f"{key}: {json.dumps(desc[source])}" for key, source in
        (("caseId", "case_id"), ("lifecycle", "lifecycle"), ("profile", "profile"), ("scenario", "scenario"), ("control", "control"))) + "}"
    lowered = []
    for selector in steps[1:-1]:
        kind = "adversarial-derivation" if selector.startswith("M:") else "denied-probe" if selector.startswith(("D:", "P:")) else "transition"
        lowered.append(f'{{kind: "{kind}", instruction: {checked_quint_instruction(selector)}}}')
    return f"{{descriptor: {descriptor}, scenario: {quint_scenario(desc)}, profile: {desc['profile']}, steps: List(" + ",\n".join(lowered) + ")}"

def quint_cases():
    imports = "\n".join(f'  import {name}.* from "./{name}"' for name in
        ("effects", "consumption", "observations", "policies", "authorization", "execution",
         "candidate_a_types", "candidate_a_programs", "candidate_a_core", "candidate_a_projection",
         "candidate_a_authority_adapter", "candidate_a_authority_boundary",
         "candidate_a_authority_installment_fixtures", "candidate_a_authority_installment",
         "candidate_a_authority_swap_fixtures", "candidate_a_authority_swap", "candidate_a_integrated_observer"))
    body = []
    for lifecycle, name in (("installment", "INSTALLMENT_CASES_A4"), ("swap", "SWAP_CASES_A4")):
        values = [quint_case(d, steps) for d, steps in instructions() if d["lifecycle"] == lifecycle]
        body.append(f"pure val {name}: List[A4Case] = List(\n" + ",\n".join(values) + ")")
    return "module candidate_a_integrated_cases {\n" + imports + "\n" + "\n".join(body) + "\n}\n"
```

The renderer accepts only the reviewed selector language from `instructions()`. Add `assert set(selector for _, steps in instructions() for selector in steps[1:-1])` coverage by calling `quint_instruction` on every member; test altered selectors independently reject rather than being accidentally converted to commit/verified.

Before the first renderer branch, require membership in the fixed grammar. This prevents the convenient string branches from accepting unrecognized commands:

```python
def checked_quint_instruction(selector):
    allowed = {s for _, steps in instructions() for s in steps[1:-1]}
    if type(selector) is not str or selector not in allowed:
        raise ValueError("selector outside fixed inventory")
    return quint_instruction(selector)
```

In `quint_case`, call `checked_quint_instruction(selector)` instead of the unchecked internal renderer. Add:

```python
import pytest
from scripts.s02_candidate_a_integrated_inventory import checked_quint_instruction, quint_cases

@pytest.mark.parametrize("selector", ["P:invented:first", "M:invented", "S:advance:99", "D:verify:first:foreign"])
def test_unknown_selector_rejected(selector):
    with pytest.raises(ValueError):
        checked_quint_instruction(selector)

def test_every_selector_lowers_and_case_table_is_literal():
    for _, steps in instructions():
        for selector in steps[1:-1]:
            assert checked_quint_instruction(selector)
    text = quint_cases()
    assert text.count("descriptor: {") == 78
    assert "var " not in text
```

- [ ] Create `candidate_a_integrated_driver.qnt` with the common imports above plus `candidate_a_integrated_observer.*`. The complete machine body is:

```quint
const CASES_A4: List[A4Case]
var authorityState: AAuthorityExecution
var latestEvent: A4Event
var caseIndex: int
var cursor: int

action init = all {
  authorityState' = initialA4(CASES_A4.nth(0)), latestEvent' = startA4(CASES_A4.nth(0)),
  caseIndex' = 0, cursor' = 0
}
action step = {
  val c = CASES_A4.nth(caseIndex)
  if (latestEvent.kind == "diagnostic") all {
    false, authorityState' = authorityState, latestEvent' = latestEvent,
    caseIndex' = caseIndex, cursor' = cursor
  }
  else if (cursor < c.steps.length()) {
    val event = observeA4(authorityState, c, cursor + 1, c.steps.nth(cursor))
    all {
      latestEvent' = event,
      authorityState' = if (Set("transition", "adversarial-derivation").contains(event.kind))
        applyA4(authorityState, c, event.arguments) else authorityState,
      cursor' = cursor + 1, caseIndex' = caseIndex
    }
  } else if (cursor == c.steps.length()) all {
    latestEvent' = endA4(authorityState, c, latestEvent), authorityState' = authorityState,
    cursor' = cursor + 1, caseIndex' = caseIndex
  } else if (caseIndex + 1 >= CASES_A4.length()) all {
    false, authorityState' = authorityState, latestEvent' = latestEvent,
    caseIndex' = caseIndex, cursor' = cursor
  } else all {
    latestEvent.kind == "case-end",
    authorityState' = initialA4(CASES_A4.nth(caseIndex + 1)),
    latestEvent' = startA4(CASES_A4.nth(caseIndex + 1)), caseIndex' = caseIndex + 1, cursor' = 0
  }
}
val noDiagnosticA4 = latestEvent.kind != "diagnostic"
val sourceInvariantA4 = if (CASES_A4.nth(caseIndex).descriptor.lifecycle == "installment")
  safetyI(authorityState) else safetyS(authorityState)
val completeA4 = caseIndex == CASES_A4.length() - 1 and latestEvent.kind == "case-end"
```

There is no unconditional stutter. Normal terminal has no enabled next step; the exact `--max-steps` reaches its last case-end. Diagnostic terminal remains a failed invariant and incomplete inventory, never success. The export commands below must request the exact bounds, not one extra transition past normal terminal.

Create these complete instantiation modules:

```quint
module candidate_a_integrated_installment_export {
  import candidate_a_integrated_cases.* from "./candidate_a_integrated_cases"
  import candidate_a_integrated_driver(CASES_A4 = INSTALLMENT_CASES_A4).* from "./candidate_a_integrated_driver"
}
```

```quint
module candidate_a_integrated_swap_export {
  import candidate_a_integrated_cases.* from "./candidate_a_integrated_cases"
  import candidate_a_integrated_driver(CASES_A4 = SWAP_CASES_A4).* from "./candidate_a_integrated_driver"
}
```

- [ ] Add actual isolated driver tests. In the test module, add imports of observer/cases and the installment driver as `I` and swap driver as `S`. Pure helpers below do not initialize successful authority states; they execute the actual guarded command sequence.

```quint
type A4Prefix = {state: AAuthorityExecution, ok: bool, last: A4Event}
pure def executeStepsA4(c: A4Case, count: int): A4Prefix =
  c.steps.slice(0, count).foldl({state: initialA4(c), ok: true, last: startA4(c)}, (acc, step) => {
    val event = observeA4(acc.state, c, acc.last.sequence + 1, step)
    val after = if (Set("transition", "adversarial-derivation").contains(event.kind))
      applyA4(acc.state, c, event.arguments) else acc.state
    {state: after, ok: acc.ok and event.kind != "diagnostic", last: event}
  })
run inventoryBoundsTest = assert(INSTALLMENT_CASES_A4.length() == 32 and SWAP_CASES_A4.length() == 46
  and INSTALLMENT_CASES_A4.foldl(0, (n, c) => n + c.steps.length() + 2) == 638
  and SWAP_CASES_A4.foldl(0, (n, c) => n + c.steps.length() + 2) == 919)
run actualFirstCommitTest = assert({
  val c = SWAP_CASES_A4.nth(0)
  val prepared = executeStepsA4(c, 2)
  val committed = executeStepsA4(c, 5)
  prepared.ok and committed.ok and prepared.state.authority.context.candidate == beforeS(0)
    and committed.state.authority.context.candidate == beforeS(1)
    and committed.state.authority.context.ledger == ledgerS(1)
    and match committed.state.attempts.get(FundingOneAttempt) { | ExecutedOperation(_) => true | _ => false }
})
run sourceRoutesRetainedTest = assert(
  INSTALLMENT_CASES_A4.concat(SWAP_CASES_A4).foldl(true, (ok, c) => {
    val actual = executeStepsA4(c, c.steps.length())
    ok and actual.ok and endA4(actual.state, c, actual.last).kind == "case-end"
  }))
run dualMutationBoundariesTest = assert(
  INSTALLMENT_CASES_A4.concat(SWAP_CASES_A4).select(c =>
    Set("unused-successor", "reversed-effects", "reductions", "neutral-chooser").contains(c.descriptor.control))
  .foldl(true, (ok, c) => {
    val prefixCount = if (c.descriptor.lifecycle == "installment") 4 else 16
    val derived = executeStepsA4(c, prefixCount + 1)
    val denied = executeStepsA4(c, prefixCount + 2)
    val rejected = executeStepsA4(c, prefixCount + 3)
    val constructed = executeStepsA4(c, prefixCount + 4)
    val commitDenied = executeStepsA4(c, prefixCount + 5)
    val done = executeStepsA4(c, prefixCount + 6)
    val id = if (c.descriptor.lifecycle == "installment") FirstFillAttempt else DispositionAttempt
    ok and derived.ok and denied.ok and rejected.ok and constructed.ok and commitDenied.ok and done.ok
      and denied.state == derived.state and not(denied.last.observedGuard)
      and commitDenied.state == constructed.state and not(commitDenied.last.observedGuard)
      and rejected.state.authority == derived.state.authority and done.state.authority == constructed.state.authority
      and match rejected.state.attempts.get(id) {
        | RejectedOperation(r) => r.stage == VerificationBoundary and r.reason == UnauthorizedEffect
            and match done.state.attempts.get(id) {
              | RejectedOperation(final) => final.stage == CommitBoundary and final.reason == UnauthorizedEffect
                  and final.attempt == r.attempt and final.evidence == r.evidence
              | _ => false }
        | _ => false }
  }))
run deniedReplayPreservesAllFieldsTest = assert({
  val c = SWAP_CASES_A4.nth(0)
  val done = executeStepsA4(c, 18)
  val replayed = executeStepsA4(c, c.steps.length())
  done.ok and replayed.ok and replayed.state == done.state
})
run firstCaseEndAndResetTest = I::init.then(16.reps(_ => I::step))
  .expect(I::latestEvent.kind == "case-end" and I::caseIndex == 0 and I::cursor == 16)
  .then(I::step).expect(I::latestEvent.kind == "case-start" and I::caseIndex == 1
    and I::cursor == 0 and I::authorityState == unsignedI)
run initialEventsAreNotCompletionsTest = S::init.expect(S::latestEvent.kind == "case-start"
  and not(S::completeA4) and S::authorityState == unsignedS)
```

- [ ] Preserve a real, typed generic-Verify RED in the **new** `directGuardA4`: initial VerifyA4 branch calls `canVerify`, while `dualMutationBoundariesTest` requires A fidelity and exact retained rejection. Run it and record terminal assertion failure and full closure. Then replace only that branch with the displayed `canVerifyAuthorityA`; preserve the exact one-line correction and run all current tests GREEN. This is a controlled boundary-regression RED, not evidence that all reused lifecycle behavior was newly invented test-first. Do not alter accepted lifecycle Verify functions.

- [ ] Add the needed direct test imports, then run one recursive test typecheck and the full test file (11 named tests at this point) with Rust seed42. A parse error, unknown fixture or broken test action chain is not the required behavioral RED. If the test chain syntax needs repair, preserve that terminal outcome separately, rerun the actual assertion RED, and only then correct the guard.

- [ ] After Task3 source/unit tests and the independent checker source are frozen, run these actual deterministic export commands once each, through the recorder. This final capture is deferred until all source pins exist; do not generate an acceptance closure before the exporter/checker code exists and then substitute moving Python source. Require terminal exit0, no invariant violation, `completeA4` witnessed exactly once, and 638/919 actual ITF states. If a resource failure occurs, preserve it and propose a concrete semantics-preserving batch split before rerunning; do not shrink the 78-case inventory.

```bash
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/installment-export/python-cache scripts/record_s02_candidate_a_integrated.py --stage installment-export -- quint run specs/quint/s02/candidate_a_integrated_installment_export.qnt --backend=rust --seed=42 --max-samples=1 --n-traces=1 --max-steps=637 --invariants noDiagnosticA4 sourceInvariantA4 --witnesses completeA4 --out-itf .superpowers/sdd/a4-producer-receipts/installment-export/installment.itf.json
/home/charl/Moriarty/.venv/bin/python -B -X pycache_prefix=.superpowers/sdd/a4-producer-receipts/swap-export/python-cache scripts/record_s02_candidate_a_integrated.py --stage swap-export -- quint run specs/quint/s02/candidate_a_integrated_swap_export.qnt --backend=rust --seed=42 --max-samples=1 --n-traces=1 --max-steps=918 --invariants noDiagnosticA4 sourceInvariantA4 --witnesses completeA4 --out-itf .superpowers/sdd/a4-producer-receipts/swap-export/swap.itf.json
```

Record the actual CLI-reported seed hint separately if it differs from the requested `42`. Keep full raw ITFs byte-identical, including final newline, not just selected event JSON. These exact finite traces, not earlier 100-sample lifecycle summaries, are the A4 producer input.

## Task 3: Strict structural schema-2 exporter

**Files:** Create `scripts/export_s02_candidate_a_integrated.py`; extend `tests/test_s02_candidate_a_integrated_export.py`.

**Consumes:** Root-adopted inventory and separate admission; two actual raw ITFs; exact source and receipt trees. The admission is a trusted root input, not submitted-package self-attestation.

**Produces:** `export_document(input_root,source_root,inventory_path,admission_path,quint):dict`, and a CLI with no subset flag. This module does not import the independent checker or use producer safety booleans as semantic evidence.

Admission has exactly `schema_version,inventory_sha256,source_pins,input_pins,receipt_pins,entries`; entries is exactly:

```json
{"installment":"specs/quint/s02/candidate_a_integrated_installment_export.qnt","swap":"specs/quint/s02/candidate_a_integrated_swap_export.qnt"}
```

Inventory has exactly `schema_version:2,cases:[descriptor-five-fields plus event_count]`. Its digest is SHA256 of `json.dumps(value,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode('utf-8')`, without a newline. The producer's independent `instructions()` still specifies every selector and count; the submitted inventory cannot change it.

The structural validator reads Quint **type declarations only** from the pinned local parser output. It does not run an authority interpreter in Python or infer semantic correctness from types. An actual read-only planning probe confirmed Quint0.32.0 `parse --out /dev/stdout` JSON contains `modules[].declarations[kind=typedef]`, `params`, and type nodes `sum,rec,tup,fun,list,set,const,var,app,int,bool,str`. Pin the parser identity in the root receipt; unknown IR syntax is a failure, not a permissive fallback.

- [ ] Add a compiling/importable `validate_inventory` stub that returns `None`, then add/run this actual behavioral omission test before implementing validation:

```python
from scripts.export_s02_candidate_a_integrated import ExportError, validate_inventory

def test_dropped_whole_case_is_rejected():
    fixed = inventory()["cases"]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(fixed[:-1], fixed)

def test_reordered_profiles_are_rejected():
    fixed = inventory()["cases"]
    changed = list(fixed)
    changed[0], changed[1] = changed[1], changed[0]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(changed, fixed)
```

Record assertion RED, full source closure and terminal exit. Implement the following complete utility/type-validation portion; retain the old schema-1 scripts unchanged.

```python
from __future__ import annotations
import argparse
import hashlib
import json
import posixpath
import re
import subprocess
from pathlib import Path, PurePosixPath
from scripts.s02_candidate_a_integrated_inventory import inventory, instructions

class ExportError(ValueError):
    pass

def require(condition, message):
    if not condition:
        raise ExportError(message)

def fields(value, expected, label):
    require(type(value) is dict and set(value) == set(expected), f"{label}: exact fields")

def unique(pairs):
    result = {}
    for key, value in pairs:
        require(key not in result, "duplicate JSON key")
        result[key] = value
    return result

def load(data):
    try:
        return json.loads(data, object_pairs_hook=unique,
                          parse_constant=lambda text: (_ for _ in ()).throw(ExportError("nonfinite JSON")))
    except (UnicodeError, json.JSONDecodeError) as exc:
        raise ExportError("malformed JSON") from exc

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf-8")

def digest(data):
    return hashlib.sha256(data).hexdigest()

def safe(root, name):
    require(type(name) is str and name and "\\" not in name, "unsafe path")
    parts = name.split("/")
    require(not name.startswith("/") and all(p not in ("", ".", "..") for p in parts), "unsafe path")
    root = Path(root).absolute()
    require(all(not parent.is_symlink() for parent in (root, *root.parents)), "symlink root/ancestor")
    current = root
    for part in parts:
        current = current / part
        require(not current.is_symlink(), "symlink path")
    require(current.resolve().is_relative_to(root.resolve()), "path escape")
    return current

def verify_pins(root, pins):
    require(type(pins) is dict and all(type(k) is str and type(v) is str and re.fullmatch(r"[0-9a-f]{64}", v)
                                      for k, v in pins.items()), "malformed pins")
    for name in pins:
        safe(root, name)
    require(all(safe(root, name).is_file() and digest(safe(root, name).read_bytes()) == expected
                for name, expected in pins.items()), "admitted file/hash inventory")

def integer(value):
    fields(value, ("#bigint",), "ITF integer")
    text = value["#bigint"]
    require(type(text) is str and re.fullmatch(r"0|-?[1-9][0-9]*", text), "noncanonical ITF integer")
    return int(text)

def frozen(value):
    if type(value) is bool: return ("bool", value)
    if type(value) is str: return ("str", value)
    if type(value) is list: return ("list", tuple(frozen(x) for x in value))
    require(type(value) is dict, "raw scalar outside ITF domain")
    if set(value) == {"#bigint"}: return ("int", integer(value))
    if set(value) == {"#set"}:
        require(type(value["#set"]) is list, "ITF set members")
        members = [frozen(x) for x in value["#set"]]
        require(len(members) == len(set(members)), "duplicate ITF set member")
        return ("set", tuple(sorted(members, key=repr)))
    if set(value) == {"#map"}:
        require(type(value["#map"]) is list, "ITF map pairs")
        pairs = []
        for pair in value["#map"]:
            require(type(pair) is list and len(pair) == 2, "ITF map pair")
            pairs.append((frozen(pair[0]), frozen(pair[1])))
        require(len(pairs) == len({k for k, _ in pairs}), "duplicate ITF map key")
        return ("map", tuple(sorted(pairs, key=repr)))
    if set(value) == {"#tup"}:
        require(type(value["#tup"]) is list, "ITF tuple")
        return ("tuple", tuple(frozen(x) for x in value["#tup"]))
    require(not any(k.startswith("#") for k in value), "unknown ITF wrapper")
    return ("record", tuple(sorted((k, frozen(v)) for k, v in value.items())))

def raw_map(value):
    fields(value, ("#map",), "map")
    frozen(value)
    return value["#map"]

def tags(value):
    return {key["tag"] for key, _ in raw_map(value)}

class Types:
    def __init__(self, parsed):
        self.types = {}
        require(not parsed.get("errors"), "Quint parse diagnostics")
        for module in parsed["modules"]:
            for declaration in module["declarations"]:
                if declaration["kind"] == "typedef":
                    name = declaration["name"]
                    require(name not in self.types, "ambiguous type declaration")
                    self.types[name] = declaration

    def named(self, name, value, args=(), depth=0):
        require(depth < 128 and name in self.types, "unknown/recursive type")
        declaration = self.types[name]
        params = declaration.get("params", [])
        require(len(params) == len(args), "type arity")
        self.check(declaration["type"], value, dict(zip(params, args)), depth + 1)
        self.domain(name, value)
        full_choice_domain(name, value)

    def resolve(self, typ, env):
        if typ["kind"] == "var":
            require(typ["name"] in env, "unbound type variable")
            return self.resolve(env[typ["name"]], {})
        if typ["kind"] == "app":
            return {**typ, "args": [self.resolve(arg, env) for arg in typ["args"]]}
        return typ

    def check(self, typ, value, env, depth):
        require(depth < 128, "type depth bound")
        kind = typ["kind"]
        if kind == "var":
            self.check(self.resolve(typ, env), value, {}, depth + 1)
        elif kind == "const":
            self.named(typ["name"], value, (), depth + 1)
        elif kind == "app":
            self.named(typ["ctor"]["name"], value,
                       [self.resolve(arg, env) for arg in typ["args"]], depth + 1)
        elif kind in ("bool", "str"):
            require(type(value) is (bool if kind == "bool" else str), "scalar type")
        elif kind == "int":
            require(-1 <= integer(value) <= 340, "finite exported integer domain")
        elif kind in ("list", "set"):
            if kind == "set":
                fields(value, ("#set",), "set")
                frozen(value)
                items = value["#set"]
            else: items = value
            require(type(items) is list, "list/set representation")
            for item in items: self.check(typ["elem"], item, env, depth + 1)
        elif kind == "fun":
            for key, item in raw_map(value):
                self.check(typ["arg"], key, env, depth + 1)
                self.check(typ["res"], item, env, depth + 1)
        elif kind in ("rec", "sum", "tup"):
            row = typ["fields"]
            require(row["kind"] == "row" and row["other"]["kind"] == "empty", "open type row")
            entries = {f["fieldName"]: f["fieldType"] for f in row["fields"]}
            if kind == "sum":
                fields(value, ("tag", "value"), "variant")
                require(type(value["tag"]) is str and value["tag"] in entries, "unknown tag")
                self.check(entries[value["tag"]], value["value"], env, depth + 1)
            elif kind == "rec":
                fields(value, entries, "typed record")
                for key, item in value.items(): self.check(entries[key], item, env, depth + 1)
            else:
                fields(value, ("#tup",), "tuple")
                require(type(value["#tup"]) is list and len(value["#tup"]) == len(entries), "tuple arity")
                for index, item in enumerate(value["#tup"]): self.check(entries[str(index)], item, env, depth + 1)
        else:
            raise ExportError(f"unsupported type IR {kind}")

    def domain(self, name, value):
        if name == "AProgram":
            require(tags(value["nodes"]) == {f"N{i}" for i in range(16)}, "complete node table")
        elif name == "AState":
            require(len(raw_map(value["accounts"])) == 6, "complete six accounts")
            require(tags(value["choices"]) == {"SettleId", "FirstFillId", "SecondFillId", "RecoveryId", "OtherId"}, "complete optional choices")
            require(all(0 <= integer(v) <= 340 for _, v in raw_map(value["accounts"])), "account balance domain")
        elif name == "AuthorityKey":
            require(integer(value["nonce"]) in (0, 1), "nonce domain")
        elif name == "Environment":
            require(integer(value["physicalTime"]) in (1, 2, 100, 101), "environment time domain")
            require(integer(value["anchor"]) in (0, 1, 2), "anchor domain")
            require(all(integer(value[k]) in (0, 1) for k in ("implementationVersion", "enforcementMechanism")), "mechanism domain")
        elif name == "Ledger":
            require(len(raw_map(value)) == 18 and all(integer(v) >= 0 for _, v in raw_map(value)), "ledger domain")
        elif name in ("AuthorityContext", "SigningState", "ExecutionState"):
            required = {"AuthorityContext": ("registry", "parents"), "SigningState": ("signing",), "ExecutionState": ("attempts",)}[name]
            for key in required:
                require(len(raw_map(value[key])) == (8 if key == "attempts" else 12), "complete authority map")
        elif name == "AValue":
            require(integer(value["value"]) in (-1, 0, 1, 5, 10, 20), "constant domain")
        elif name == "ASuppliedInput":
            body = value["value"]
            require(integer(body["quantity" if value["tag"] == "DepositInputA" else "chosen"])
                    in ((-1, 0, 1, 5, 10, 20, 21) if value["tag"] == "DepositInputA" else (-1, 0, 1, 2)), "input domain")
        elif name == "OptionalInt" and value["tag"] == "IntValue":
            require(-1 <= integer(value["value"]) <= 340, "optional integer domain")
        elif name == "CoreError" and value["tag"] == "CoreErrorCode":
            require(value["value"] in ("time_before_state", "contract_closed", "input_required", "no_matching_input", "choice_out_of_bounds", "non_positive_deposit"), "Core error domain")

def validate_inventory(actual, expected):
    require(actual == expected, "case inventory")
```

The `OptionalInt` domain is deliberately wider than choice values because warnings use the same shared type. Add the **choice-specific** restriction in `AState` and `CoreStateObservation` below; zero and absence never normalize together. This is admission validation, not Core execution.

```python
def choice_values(mapping):
    for _, value in raw_map(mapping):
        require(value["tag"] == "NoInt" or integer(value["value"]) in (-1, 0, 1, 2), "choice value domain")

def full_choice_domain(name, value):
    if name == "AState":
        choice_values(value["choices"])
    elif name == "CoreStateObservation":
        require({key for key, _ in raw_map(value["choices"])} == {"settle", "fill1", "fill2", "recover", "other"}, "neutral choice IDs")
        require(len(raw_map(value["accounts"])) == 6, "neutral accounts")
        choice_values(value["choices"])
```

Call `full_choice_domain(name,value)` at the end of `Types.named` after `self.domain`. All known finite key sets follow from exact typed key domains, duplicate rejection and exact cardinality. Do not use map length alone without typed key validation.

- [ ] Add the source/admission/position exporter below. Its root inventory comparison is exact **ordered descriptor/count equality**; its raw scanner accounts for every state from both ITF files once. It cannot conceal an extra trailing state, an intermediate reset or an omitted event.

```python
import shutil
import sys

ENTRIES = {life: f"specs/quint/s02/candidate_a_integrated_{life}_export.qnt" for life in ("installment", "swap")}
QNT_ROOTS = (*ENTRIES.values(), "specs/quint/s02/candidate_a_integrated_export_test.qnt",
             "specs/quint/s02/candidate_a_authority_installment_test.qnt", "specs/quint/s02/candidate_a_authority_swap_test.qnt")
PYTHON_SOURCES = {
    "moriarty/__init__.py", "moriarty/core.py", "moriarty/swap.py", "scripts/check_s02_candidate_a_correspondence.py",
    "scripts/a4_carrier.py", "scripts/a4_agreement.py", "scripts/a4_authority.py", "scripts/a4_cases.py",
    "scripts/a4_inventory.py", "scripts/check_s02_candidate_a_integrated.py",
    "scripts/s02_candidate_a_integrated_inventory.py", "scripts/export_s02_candidate_a_integrated.py",
    "scripts/record_s02_candidate_a_integrated.py", "tests/test_s02_candidate_a_integrated.py",
    "tests/test_s02_candidate_a_integrated_export.py",
}
IMPORT = re.compile(r'^\s*import\s+[^\n]*?\s+from\s+"([^"]+)"', re.MULTILINE)
VARS = {"authorityState", "latestEvent", "caseIndex", "cursor"}

def required_sources(root):
    seen = set(PYTHON_SOURCES)
    pending = list(QNT_ROOTS)
    while pending:
        name = pending.pop()
        if name in seen: continue
        seen.add(name)
        source = safe(root, name).read_text(encoding="utf-8")
        for imported in IMPORT.findall(source):
            require(imported.startswith("./"), "foreign/nonlocal Quint import")
            target = posixpath.normpath(str(PurePosixPath(name).parent / (imported + ".qnt")))
            safe(root, target)
            pending.append(target)
    return seen

def parser_types(root, entry, quint):
    completed = subprocess.run([quint, "parse", entry, "--out", "/dev/stdout"], cwd=root,
                               capture_output=True, check=False)
    require(completed.returncode == 0, "pinned Quint parse failed")
    return Types(load(completed.stdout))

def expected_selector_shape(selector):
    head, verb, *rest = selector.split(":")
    if head == "B": return ("case-start", "NoGuardA4", "CaseStartA4") if verb == "start" else ("case-end", "NoGuardA4", "CaseEndA4")
    kind = "adversarial-derivation" if head == "M" else "denied-probe" if head in ("D", "P") else "transition"
    ordinary_tags = {"prepare": "PrepareA4", "sign": "SignA4", "propose": "ProposeA4", "verify": "VerifyA4", "commit": "CommitA4",
                     "advance": "AdvanceA4", "reject-stale": "RejectVerifiedA4", "reject-proposed": "RejectProposedA4",
                     "reject-verified": "RejectVerifiedA4", "prepare-parent": "PrepareA4", "sign-parent": "SignA4",
                     "prepare-recovery": "PrepareA4", "sign-recovery": "SignA4", "reject-recovery": "RejectProposedA4"}
    if head in ("I", "S"):
        return kind, "InstallmentCommandGuardA4" if head == "I" else "SwapCommandGuardA4", ordinary_tags[verb]
    if head == "M": return kind, "NoGuardA4", "DeriveA4"
    special = {"cancel-parent": ("CancelParentGuardA4", "CancelParentProbeA4"),
               "slot": ("ConsumeSlotGuardA4", "ConsumeSlotProbeA4"),
               "financial-recovery": ("FinancialGuardA4", "FinancialProbeA4"),
               "plan": ("PlanMatchesGuardA4", "PlanMatchesProbeA4"),
               "core": ("CoreAcceptedGuardA4", "CoreAcceptedProbeA4"),
               "prepare-recovery": ("InstallmentCommandGuardA4", "PrepareA4")}
    if verb in special: return (kind, *special[verb])
    command = {"prepare-parent": "PrepareA4", "sign-disposition": "SignA4", "bad-prepare": "PrepareA4",
               "bad-plan-propose": "ProposeA4", "duplicate-cancel": "ProposeA4", "reject": "RejectProposedA4"}.get(verb, ordinary_tags.get(verb))
    require(command is not None, "unknown fixed selector")
    return kind, command.replace("A4", "GuardA4"), command

def validate_receipt(root, life, input_hash, quint, source_pins):
    receipt = load(safe(root, f"{life}-export/receipt.json").read_bytes())
    require(type(receipt["exit_code"]) is int and receipt["exit_code"] == 0 and receipt["source_stable"] is True, "nonterminal/unstable producer receipt")
    require(type(receipt["recorder_exit_code"]) is int and receipt["recorder_exit_code"] == 0
            and receipt["runtime_stable"] is True, "unstable runtime receipt")
    require(receipt["runtime_before"] == receipt["runtime_after"]
            and receipt["shared_runtime_before"] == receipt["shared_runtime_after"], "runtime closure moved")
    require(receipt["sources_before"] == receipt["sources_after"] == source_pins, "receipt/source closure binding")
    expected = ["quint", "run", ENTRIES[life], "--backend=rust", "--seed=42", "--max-samples=1", "--n-traces=1",
                f"--max-steps={637 if life == 'installment' else 918}", "--invariants", "noDiagnosticA4", "sourceInvariantA4",
                "--witnesses", "completeA4", "--out-itf",
                f".superpowers/sdd/a4-producer-receipts/{life}-export/{life}.itf.json"]
    require(receipt["command"] == expected, "foreign producer command")
    require(receipt["executed_command"] == [receipt["tools"]["node"]["path"],
            receipt["tools"]["quint"]["path"], *expected[1:]], "actual Node/Quint command binding")
    require(receipt["artifacts"] == {f"{life}.itf.json": input_hash}, "receipt/raw-input binding")
    require(digest(Path(shutil.which(quint) or quint).resolve().read_bytes()) == receipt["tools"]["quint"]["sha256"], "parser tool pin")
    return receipt

def export_document(input_root, source_root, inventory_path, admission_path, quint="quint"):
    admitted = load(Path(admission_path).read_bytes())
    fields(admitted, ("schema_version", "inventory_sha256", "source_pins", "input_pins", "receipt_pins", "entries"), "admission")
    require(type(admitted["schema_version"]) is int and admitted["schema_version"] == 2, "admission version")
    require(admitted["entries"] == ENTRIES, "entry inventory")
    fixed = load(Path(inventory_path).read_bytes())
    require(fixed == inventory(), "root versus producer fixed inventory")
    require(digest(canonical(fixed)) == admitted["inventory_sha256"], "inventory digest")
    for root, key in ((input_root, "input_pins"), (source_root, "source_pins"), (input_root, "receipt_pins")):
        verify_pins(root, admitted[key])
    require(not (set(admitted["input_pins"]) & set(admitted["receipt_pins"])), "overlapping raw/receipt pins")
    declared = set(admitted["input_pins"]) | set(admitted["receipt_pins"])
    discovered = {p.relative_to(input_root).as_posix() for p in Path(input_root).rglob("*.itf.json")}
    require(discovered <= declared, "undeclared raw ITF")
    require(set(admitted["input_pins"]) == {"installment.itf.json", "swap.itf.json"}, "raw input inventory")
    require(set(admitted["source_pins"]) == required_sources(source_root), "source import closure")
    cases = []
    for life in ("installment", "swap"):
        name = f"{life}.itf.json"
        validate_receipt(input_root, life, admitted["input_pins"][name], quint, admitted["source_pins"])
        types = parser_types(source_root, ENTRIES[life], quint)
        document = load(safe(input_root, name).read_bytes())
        fields(document, ("#meta", "vars", "states"), "ITF document")
        fields(document["#meta"], ("format", "format-description", "source", "status", "description", "timestamp"), "ITF metadata")
        meta = document["#meta"]
        require(meta["format"] == "ITF" and meta["source"] == ENTRIES[life], "ITF source binding")
        require(meta["format-description"] == "https://apalache-mc.org/docs/adr/015adr-trace.html", "ITF format version")
        require(meta["status"] == "ok" and type(meta["description"]) is str and type(meta["timestamp"]) is int, "ITF terminal metadata")
        require(type(document["vars"]) is list and len(document["vars"]) == 4 and set(document["vars"]) == VARS, "raw variable inventory")
        states = document["states"]
        expected = [(d, steps) for d, steps in instructions() if d["lifecycle"] == life]
        require(type(states) is list and len(states) == sum(len(steps) for _, steps in expected), "raw state/event inventory")
        position = 0
        for case_index, (desc, selectors) in enumerate(expected):
            events = []
            for sequence, selector in enumerate(selectors):
                state = states[position]
                fields(state, (*VARS, "#meta"), "raw state")
                fields(state["#meta"], ("index",), "state index")
                require(type(state["#meta"]["index"]) is int and state["#meta"]["index"] == position, "raw position")
                require(integer(state["caseIndex"]) == case_index and integer(state["cursor"]) == sequence, "driver counters")
                raw = state["latestEvent"]
                types.named("A4Event", raw)
                types.named("AAuthorityExecution", state["authorityState"])
                kind, guard, command = expected_selector_shape(selector)
                require(raw["caseId"] == desc["case_id"] and raw["profile"] == desc["profile"] and integer(raw["sequence"]) == sequence, "event identity/order")
                require(raw["kind"] == kind and raw["arguments"]["guard"]["tag"] == guard
                        and raw["arguments"]["command"]["tag"] == command, "event selector shape")
                require(raw["observedGuard"] is (kind != "denied-probe"), "observed guard disposition")
                before_index = max(0, position - 1)
                before = states[before_index]["authorityState"]
                after = state["authorityState"]
                validate_computations(before, raw)
                if kind in ("denied-probe", "case-end"):
                    require(before == after, "denial/end changed authority")
                if kind == "case-start":
                    require(sequence == 0 and (position == 0 or states[position - 1]["latestEvent"]["kind"] == "case-end"), "illegal reset")
                    payload = raw["arguments"]["command"]["value"]
                    require(payload == {"caseId": desc["case_id"], "lifecycle": life, "profile": desc["profile"], "scenario": desc["scenario"], "control": desc["control"]}, "case-start descriptor")
                    require(after == states[0]["authorityState"], "noncanonical reset state")
                events.append({"case_id": raw["caseId"], "profile": raw["profile"], "sequence": integer(raw["sequence"]),
                    "kind": raw["kind"], "arguments": raw["arguments"], "observed_guard": raw["observedGuard"],
                    "computations": raw["computations"], "before": before, "after": after,
                    "provenance": {"input_path": name, "input_sha256": admitted["input_pins"][name],
                                   "before_index": before_index, "after_index": position}})
                position += 1
            cases.append({**desc, "events": events})
        require(position == len(states), "unaccounted raw states")
    validate_inventory([{**{k: c[k] for k in ("case_id", "lifecycle", "profile", "scenario", "control")}, "event_count": len(c["events"])} for c in cases], fixed["cases"])
    return {"schema_version": 2, "inventory_sha256": admitted["inventory_sha256"],
            "source_pins": admitted["source_pins"], "input_pins": admitted["input_pins"],
            "receipt_pins": admitted["receipt_pins"], "cases": cases}

def main(argv=None):
    parser = argparse.ArgumentParser()
    for option in ("input-root", "source-root", "inventory", "admission", "output"):
        parser.add_argument("--" + option, required=True)
    parser.add_argument("--quint", default="quint")
    args = parser.parse_args(argv)
    try:
        result = export_document(args.input_root, args.source_root, args.inventory, args.admission, args.quint)
        with Path(args.output).open("xb") as stream:
            stream.write(canonical(result) + b"\n")
        print(json.dumps({"ok": True, "cases": len(result["cases"]), "events": sum(len(c["events"]) for c in result["cases"])}))
        return 0
    except (ExportError, OSError, KeyError, TypeError, RecursionError) as exc:
        print(json.dumps({"ok": False, "error": str(exc)}), file=sys.stderr)
        return 1

if __name__ == "__main__":
    raise SystemExit(main())
```

At file start, before importing `scripts.s02_candidate_a_integrated_inventory`, insert the normal script-entry bootstrap:

```python
import sys
from pathlib import Path
if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
```

Only the **outer** event sequence converts canonical raw `#bigint` to an ordinary JSON integer. All nested structures stay raw value-identical. Raw integer forms with booleans, leading zeros or negative zero fail rather than being normalized. The original ITF file bytes, not the reserialized JSON envelope, are the byte-provenance anchor.

The accepted filesystem layout matches the checker plan: `--source-root .`; `--input-root evidence/s02-candidate-a-completion/a4`, containing the two raw files and receipt-relative paths such as `installment-export/receipt.json`. There is no separate receipt-root flag. Validate every admitted pin individually, require exact source-closure key equality and the exact two raw-input pin keys, and reject undeclared extra ITFs. Unrelated notes outside the admitted pin maps are not admitted evidence. Root reviews the exact receipt pin set; neither tool can silently select a smaller receipt set.

- [ ] Add strict computation-shape validation. After calculating `before`/`after` in the raw scanner, call `validate_computations(before,raw)`. This enforces positive call inventory without replaying Core in Python:

```python
def call_requests(call):
    if call["tag"] == "CancellationCallA": return []
    require(call["tag"] == "AgreementCallA", "unknown call discriminator")
    return [call["value"]]

def retained_attempt(state, raw_id):
    matches = [value for key, value in raw_map(state["attempts"]) if key == raw_id]
    require(len(matches) == 1, "retained attempt key")
    record = matches[0]
    require(record["tag"] != "NoAttempt", "missing retained computation attempt")
    return record["value"] if record["tag"] == "ProposedAttempt" else record["value"]["attempt"]

def computation_requests(before, raw):
    command = raw["arguments"]["command"]
    tag, body = command["tag"], command["value"]
    if tag in ("PrepareA4", "SignA4"):
        binding = body["policy"]["binding"]
        return [] if binding["tag"] == "BeforeResolution" else [request
            for op in binding["value"]["identity"]["operations"] for request in call_requests(op["artifactAndCall"])]
    if tag == "PlanMatchesProbeA4":
        return [request for op in body["plan"]["operations"] for request in call_requests(op["artifactAndCall"])]
    if tag == "CoreAcceptedProbeA4": return [body["request"]]
    if tag == "ProposeA4": observation = body["observation"]
    elif tag in ("VerifyA4", "CommitA4", "RejectProposedA4", "RejectVerifiedA4"):
        observation = retained_attempt(before, body["id"])["observation"]
    elif tag == "DeriveA4": observation = body["attempt"]["observation"]
    else: return []
    if observation["artifactAndCall"]["tag"] == "CancellationCallA":
        require(observation["coreProjection"]["tag"] == "NoCoreProjection" and observation["effects"] == []
                and observation["input"]["tag"] == "NoInput"
                and observation["predecessor"] == observation["proposedSuccessor"], "cancellation carrier")
    return call_requests(observation["artifactAndCall"])

def validate_computations(before, raw):
    expected = computation_requests(before, raw)
    records = raw["computations"]
    require(len(records) == len(expected), "computation inventory")
    for record, request in zip(records, expected):
        require(record["request"] == request, "computation request binding/order")
        require(record["evaluation"]["tag"] == "TransactionComputedA", "computation diagnostic")
        extraction = record["extraction"]
        require(extraction["tag"] == "ExtractionObservedA4" and extraction["value"]["tag"] == "EffectsExtractedA", "extraction diagnostic")
    if raw["kind"] == "case-end":
        require(raw["arguments"]["command"]["value"]["status"] in
                ("financial-terminal", "refusal-terminal", "negative-complete"), "unknown terminal label")
```

- [ ] Add parser/unit controls before the final actual-corpus gate. These are explicitly synthetic parser fixtures, not executed authority records. All mutated actual-event fields stay available to the independent checker's semantic controls; these tests demonstrate structural predicates only.

```python
from copy import deepcopy
from scripts.export_s02_candidate_a_integrated import (load, integer, frozen, safe, fields,
    Types, validate_computations, validate_inventory, ExportError)

@pytest.mark.parametrize("data", [b'{"x":1,"x":2}', b'{"x":NaN}', b'{"x":Infinity}'])
def test_duplicate_or_nonfinite_json(data):
    with pytest.raises(ExportError): load(data)

@pytest.mark.parametrize("value", [True, 0, {"#bigint": "00"}, {"#bigint": "-0"}, {"#bigint": True}])
def test_noncanonical_integer(value):
    with pytest.raises(ExportError): integer(value)

@pytest.mark.parametrize("value", [
    {"#set": [{"#bigint": "0"}, {"#bigint": "0"}]},
    {"#map": [[{"tag": "Alice", "value": {"#tup": []}}, True],
              [{"value": {"#tup": []}, "tag": "Alice"}, False]]},
    {"#map": [["x"]]}, {"#unknown": []},
])
def test_duplicate_or_foreign_itf(value):
    with pytest.raises(ExportError): frozen(value)

def test_absence_is_not_zero():
    absent = {"tag": "NoInt", "value": {"#tup": []}}
    zero = {"tag": "IntValue", "value": {"#bigint": "0"}}
    assert frozen(absent) != frozen(zero)

@pytest.mark.parametrize("name", ["../x", "/x", "a/../x", "a//x", "a\\x", "./x"])
def test_unsafe_path(tmp_path, name):
    with pytest.raises(ExportError): safe(tmp_path, name)

def test_symlink_member_rejected(tmp_path):
    (tmp_path / "real").mkdir()
    (tmp_path / "alias").symlink_to(tmp_path / "real", target_is_directory=True)
    with pytest.raises(ExportError, match="symlink"): safe(tmp_path, "alias/file")

def tiny_types():
    unit = {"kind": "tup", "fields": {"kind": "row", "fields": [], "other": {"kind": "empty"}}}
    sum_type = {"kind": "sum", "fields": {"kind": "row", "fields": [
        {"fieldName": "Only", "fieldType": unit}], "other": {"kind": "empty"}}}
    record = {"kind": "rec", "fields": {"kind": "row", "fields": [
        {"fieldName": "tagged", "fieldType": {"kind": "const", "name": "Tag"}}], "other": {"kind": "empty"}}}
    return Types({"modules": [{"declarations": [
        {"kind": "typedef", "name": "Tag", "type": sum_type},
        {"kind": "typedef", "name": "Record", "type": record}]}], "errors": []})

def test_unknown_tag_and_record_field_rejected():
    types = tiny_types()
    good = {"tagged": {"tag": "Only", "value": {"#tup": []}}}
    types.named("Record", good)
    for bad in ({**good, "foreign": True}, {"tagged": {"tag": "Foreign", "value": {"#tup": []}}}):
        with pytest.raises(ExportError): types.named("Record", bad)

def test_missing_computation_is_not_valid_denial():
    request = {"before": {}, "input": {}, "now": {}}
    raw = {"kind": "denied-probe", "arguments": {"command": {
        "tag": "CoreAcceptedProbeA4", "value": {"request": request}}}, "computations": []}
    with pytest.raises(ExportError, match="computation inventory"):
        validate_computations({}, raw)
```

Before final freeze, add actual-corpus regression tests using the root-admitted canonical archive. They must fail when the archive is missing; no scratch fallback or skip. Run parser/unit tests while the archive is pending, then run the **whole** test module once actual root intake exists:

```python
from pathlib import Path
from scripts.export_s02_candidate_a_integrated import export_document, load

A4_ARCHIVE = Path("evidence/s02-candidate-a-completion/a4")

def test_complete_actual_corpus_matches_export():
    actual = export_document(A4_ARCHIVE, Path("."), A4_ARCHIVE / "inventory.json", A4_ARCHIVE / "admission.json")
    stored = load((A4_ARCHIVE / "cases.json").read_bytes())
    assert actual == stored
    assert len(actual["cases"]) == 78
    assert sum(len(c["events"]) for c in actual["cases"]) == 1557

def test_actual_event_omission_cannot_pass_fixed_inventory():
    doc = load((A4_ARCHIVE / "cases.json").read_bytes())
    changed = deepcopy(doc["cases"])
    changed[0]["events"].pop(3)
    actual = [{**{k: c[k] for k in ("case_id", "lifecycle", "profile", "scenario", "control")},
               "event_count": len(c["events"])} for c in changed]
    with pytest.raises(ExportError, match="case inventory"):
        validate_inventory(actual, inventory()["cases"])

def test_actual_reordered_computation_requests_rejected():
    doc = load((A4_ARCHIVE / "cases.json").read_bytes())
    event = next(e for c in doc["cases"] for e in c["events"] if len(e["computations"]) == 2
                 and e["computations"][0]["request"] != e["computations"][1]["request"])
    raw = {"kind": event["kind"], "arguments": event["arguments"], "computations": list(reversed(event["computations"]))}
    with pytest.raises(ExportError, match="binding/order"):
        validate_computations(event["before"], raw)
```

An actual mutation of raw/request/source/admission fields is the checker's separate acceptance scope. Producer controls do not substitute for independent semantic mutation triples.

## Verification, freeze and handoff

Execute Task0 first, then Tasks1–3 source/unit cycles. Only after **all** producer/checker source files are frozen execute the two final export commands, root admission, complete actual-corpus checks and independent checker. No behavioral code is authorized by this planning document until root adopts it.

- [ ] Self-review all 78 descriptors/1,557 selectors against the independent checker enumeration. Record the canonical inventory hash, actual source commits and dependency receipts. Root owns the literal shared inventory/admission files.
- [ ] Preserve separate immutable stages `inventory-red/green`, `observer-red/green`, `observer-types`, `boundary-red/green`, `driver-types`, `exporter-red/green`, `installment-export`, `swap-export`, `producer-full-tests`, and `producer-cli`. A failed or corrected command gets a new suffix; never rewrite prior history.
- [ ] Exact recursive typecheck: `quint typecheck specs/quint/s02/candidate_a_integrated_export_test.qnt`. Exact model tests: `quint test specs/quint/s02/candidate_a_integrated_export_test.qnt --backend=rust --seed=42 --match '.*Test'`. Require all11 named tests and terminal exit0; add new tests only with explicit inventory update.
- [ ] Before root archive exists, run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -q -k 'not actual'` for parser/inventory controls. This excludes the three explicitly named actual-corpus tests and is not the final suite.
- [ ] Root copies the two actual ITFs and all admitted receipt members byte-identically into `evidence/s02-candidate-a-completion/a4`, verifies every snapshot/source pin and freezes `admission.json`. Root's external receipt audit establishes what was actually run; the exporter only verifies the admitted bytes and command bindings.
- [ ] Produce a fresh ignored output with `/home/charl/Moriarty/.venv/bin/python scripts/export_s02_candidate_a_integrated.py --input-root evidence/s02-candidate-a-completion/a4 --source-root . --inventory evidence/s02-candidate-a-completion/a4/inventory.json --admission evidence/s02-candidate-a-completion/a4/admission.json --output .superpowers/sdd/a4-producer-cases.json`. Require exit0 and exactly78cases/1,557events. Root then admits those exact bytes as the canonical `cases.json`; the author does not overwrite the root archive.
- [ ] Run `/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s02_candidate_a_integrated_export.py -q` with no skip/subset filter and preserve raw terminal outputs. Require the complete actual corpus, including all negative/replay events.
- [ ] Root coordinates unchanged schema1 and full Python regressions once after producer/checker closure freeze. Do not launch duplicate shared jobs from this subplan. The independent checker must separately accept the complete 78-case package and retain its security-mutant triples; structural export success alone is not A4 acceptance.
- [ ] Write `.superpowers/sdd/a4-producer-report.md` with exact source hashes, record counts, raw ITF paths/hashes, every stage command/terminal outcome, original RED closures and sole intended corrections, tool identity, source stability, independent review findings and scope limits. The author previously implemented model/lifecycle code; the separately authored checker remains the independent replay boundary. No cross-provider/Council assertion.
- [ ] Freeze all owned source/report bytes, request root nonauthor source/evidence review, and commit only after explicit root admission. This plan itself authorizes no commit.

Acceptance predicates for this bounded producer unit are: complete fixed inventory; actual guard/update histories; raw request/evaluation/extraction retention; cancellation no-Core separation; strict carrier/path/provenance validation; terminal commands with stable full source bytes; and nonauthor/root intake. A5 bounded model checking, A6 Council, A7 integration and the wider S02 program remain separate.
