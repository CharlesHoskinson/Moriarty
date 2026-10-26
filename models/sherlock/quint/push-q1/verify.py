#!/usr/bin/env python3
"""Reproduce green checks and the expected source/contract counterexample."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import shlex
import subprocess
import time

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]
MODEL = "models/sherlock/quint/push-q1/"
WITNESSES = [
    "entered", "accepted", "rejected", "refused", "cutReached", "bodyReturned",
    "calleeReturned", "added", "callerReturned", "published", "exhausted",
    "cutBeforeAddition",
]
COMMON_RUN = ["--max-steps", "30", "--max-samples", "10000", "--n-threads", "2",
              "--seed", "1401516", "--verbosity", "1"]


def main() -> int:
    results = []
    log = []
    # Invalidate older green evidence before invoking any external command.
    (HERE / "verification-results.json").write_text(json.dumps({"status": "running", "checks": []}) + "\n")
    (HERE / "verification.log").write_text("")
    cases = [
        (["quint", "--version"], 0),
        *[(["quint", "typecheck", MODEL + name], 0) for name in (
            "suspensionResume.qnt", "suspensionResume_test.qnt",
            "resumeAdmission.qnt", "resumeAdmission_test.qnt", "resumeEpoch.qnt", "resumeEpoch_test.qnt",
            "awaitLowering.qnt", "awaitLowering_test.qnt")],
        *[(["quint", "test", MODEL + name, "--max-samples", "1", "--verbosity", "1"], 0)
          for name in ("suspensionResume_test.qnt", "resumeAdmission_test.qnt", "awaitLowering_test.qnt")],
        (["quint", "test", MODEL + "resumeEpoch_test.qnt", "--backend", "typescript",
          "--max-samples", "1", "--verbosity", "1"], 0),
        (["quint", "run", MODEL + "resumeEpoch.qnt", "--invariant", "sourceBehavior",
          "--backend", "typescript", *COMMON_RUN, "--witnesses", "wrapped", "panicked",
          "acceptedWithoutOverflow", "notReady"], 0),
        (["quint", "run", MODEL + "resumeEpoch.qnt", "--step", "checkedAccept", "--invariant", "safeHeader",
          "--backend", "typescript", *COMMON_RUN, "--witnesses", "refusedOverflow",
          "acceptedWithoutOverflow", "notReady"], 0),
        (["quint", "run", MODEL + "resumeEpoch.qnt", "--invariant", "safeHeader",
          "--backend", "typescript", "--max-steps", "30", "--max-samples", "100",
          "--seed", "1", "--verbosity", "2", "--out-itf",
          MODEL + "resumeEpoch-counterexample.itf.json"], 1),
        (["quint", "run", MODEL + "suspensionResume.qnt", "--invariant", "safety",
          *COMMON_RUN, "--witnesses", *WITNESSES], 0),
        (["quint", "run", MODEL + "resumeAdmission.qnt", "--invariant", "acceptedIsAffordable",
          *COMMON_RUN, "--witnesses", "notReadyOnUnresumable", "lateResourceFailure"], 0),
        (["quint", "run", MODEL + "resumeAdmission.qnt", "--step", "contractCheck",
          "--invariant", "matchesContract", *COMMON_RUN], 0),
        (["quint", "run", MODEL + "resumeAdmission.qnt", "--invariant", "matchesContract",
          "--max-steps", "30", "--max-samples", "100", "--n-threads", "1",
          "--seed", "1", "--verbosity", "2", "--out-itf",
          MODEL + "resumeAdmission-counterexample.itf.json"], 1),
        (["quint", "run", MODEL + "awaitLowering.qnt", "--invariant", "sourceBehavior",
          *COMMON_RUN, "--witnesses", "si20ViolatedAtInnerReturn", "technicalCutRefusedAtInnerReturn",
          "binderLeakedAfterReturn", "reversedSequentialSites", "surfaceRejectedYieldOrLetLocal",
          "returnFMasksEnvLeakAfterPop"], 0),
        (["quint", "run", MODEL + "awaitLowering.qnt", "--step", "scopedStep",
          "--invariant", "contractSafety", *COMMON_RUN, "--witnesses",
          "scopedCutAcceptedAtInnerReturn", "scopedCleanAfterRightF"], 0),
        (["quint", "run", MODEL + "awaitLowering.qnt", "--invariant", "si20AndCutSafety",
          "--max-steps", "30", "--max-samples", "100", "--n-threads", "1",
          "--seed", "1", "--verbosity", "2", "--out-itf",
          MODEL + "awaitLowering-counterexample.itf.json"], 1),
    ]
    for command, expected in cases:
        started = time.monotonic()
        try:
            process = subprocess.run(command, cwd=ROOT, text=True,
                                     stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                                     timeout=60, check=False)
        except (subprocess.TimeoutExpired, OSError) as error:
            process = subprocess.CompletedProcess(command, -1, stdout=f"{type(error).__name__}: {error}\n")
        elapsed = round(time.monotonic() - started, 3)
        valid = process.returncode == expected
        if expected == 1:
            valid = valid and "Invariant violated" in process.stdout
        elif "test" in command:
            expected_count = {"suspensionResume_test.qnt": 30, "resumeAdmission_test.qnt": 4,
                              "resumeEpoch_test.qnt": 7, "awaitLowering_test.qnt": 7}[Path(command[2]).name]
            valid = valid and f"{expected_count} passing" in process.stdout
        elif "--witnesses" in command:
            names = command[command.index("--witnesses") + 1:]
            valid = valid and all(
                f"{name} was witnessed in " in process.stdout
                and f"{name} was witnessed in 0 trace(s)" not in process.stdout
                for name in names
            )
        row = {"command": command, "expectedExit": expected,
               "exit": process.returncode, "wallSeconds": elapsed, "passed": valid}
        results.append(row)
        log.append(f"$ {shlex.join(command)}\n{process.stdout}\nexit={process.returncode} wall={elapsed}s\n")
        (HERE / "verification.log").write_text("\n".join(log))
        (HERE / "verification-results.json").write_text(json.dumps(
            {"status": "running" if valid else "failed", "checks": results}, indent=2) + "\n")
        print(f"{'PASS' if valid else 'FAIL'} {shlex.join(command)} ({elapsed}s)", flush=True)
        if not valid:
            break
    sources = [
        ROOT / "docs/sherlock/design/suspension-and-finance.md",
        ROOT / "openspec/changes/sherlock-universal-interpreter-design/specs/universal-interpreter/spec.md",
        ROOT / "crates/mori-core/src/sherlock/helper_machine/shared/owner_resume.rs",
        ROOT / "crates/mori-core/tests/sherlock_technical_chunks_positive.rs",
    ]
    owner = Path.home() / "repos/Moriarty-wt-factory-u"
    owner_head = subprocess.run(
        ["git", "-C", str(owner), "rev-parse", "--short", "HEAD"],
        text=True, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, check=False,
    ).stdout.strip()
    sources += [owner / "docs/sherlock/design/suspension-and-finance.md"]
    sources += [owner / "crates/mori-core/src/sherlock" / name for name in (
        "compile_artifact/suspension2.rs",
        "compile_artifact/suspension2_fin.rs",
        "draft_source.rs",
        "helper_machine/shared/owner_suspend.rs",
        "helper_machine/shared/owner_resume.rs",
        "helper_machine/shared/owner_runtime.rs",
    )]
    sources += sorted(HERE.glob("*.qnt"))
    hashes = {str(p): hashlib.sha256(p.read_bytes()).hexdigest() for p in sources if p.exists()}
    (HERE / "verification.log").write_text("\n".join(log))
    (HERE / "verification-results.json").write_text(json.dumps(
        {"status": "passed" if len(results) == len(cases) and all(r["passed"] for r in results) else "failed",
         "factoryUHead": owner_head,
         "checks": results, "sourceSha256": hashes,
         "scope": "Sampled abstract models; source projection is not a Rust execution or refinement proof."},
        indent=2) + "\n")
    return 0 if len(results) == len(cases) and all(r["passed"] for r in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
