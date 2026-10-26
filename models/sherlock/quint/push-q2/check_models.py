#!/usr/bin/env python3
"""Reproduce the bounded checks; all generated files stay beside this script."""
import json
from pathlib import Path
import subprocess
import time


HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[3]


def checks():
    for model, invariant, backend, witnesses in (
        ("financialSegments", "allInvariants", "rust", ["paidPrefix", "retainedFee", "rejected", "exhausted", "completed", "cutObserved", "notReadyObserved"]),
        ("financialAccounting", "allInvariants", "typescript", ["selfTransfer", "overflowed", "feeObserved"]),
        ("feePoolHistory", "historyInvariants", "typescript", ["wFinish", "wClaimOrdinary", "wClaimDust"]),
        ("financialImport", "currentCodeInv", "typescript", ["imported", "malformedAccepted", "overflowReached", "epochMaxRefused", "r0LateAfterR3Observed"]),
        ("financialSegmentImport", "currentCodeInv", "rust", ["imported", "malformedSegmentAccepted", "boundsPanicReached", "notReadyWithMalformedSegment", "malformedCapturesAccepted", "si26DiagnosticsLeakedToHalted"]),
    ):
        source = str((HERE / (model + ".qnt")).relative_to(ROOT))
        tests = str((HERE / (model + "Test.qnt")).relative_to(ROOT))
        yield model + "-typecheck", ["quint", "typecheck", source], 0
        yield model + "-tests", ["quint", "test", tests, "--backend", backend, "--seed", "17"], 0
        yield model + "-simulation", ["quint", "run", source, "--backend", backend,
            "--invariant", invariant, "--max-steps", "30", "--max-samples", "10000",
            "--n-threads", "4", "--seed", "17", "--witnesses", *witnesses, "--verbosity", "1"], 0
    yield "existing-fee-pool-tests", ["quint", "test", "specs/quint/fee_pool_component_test.qnt",
        "--backend", "typescript", "--seed", "17"], 0
    for model, invariant, trace in (
        ("financialSegments", "awaitStarvedCommitsPrefixInv", "waiting-zero-fuel"),
        ("financialAccounting", "unboundedHistoryFitsU128Inv", "accounting-overflow"),
        ("financialImport", "acceptedWorkInv", "financial-import-counters"),
        ("financialSegmentImport", "acceptedSegmentInv", "financial-import-segment"),
    ):
        backend = "rust" if model in ("financialSegments", "financialSegmentImport") else "typescript"
        yield trace, ["quint", "run", str((HERE / (model + ".qnt")).relative_to(ROOT)),
            "--backend", backend, "--invariant", invariant, "--max-steps", "30",
            "--max-samples", "10000", "--n-threads", "4", "--seed", "17",
            "--out-itf", str((HERE / (trace + ".itf.json")).relative_to(ROOT)), "--verbosity", "1"], 1


def main():
    results = []
    for name, command, expected in checks():
        started = time.monotonic()
        run = subprocess.run(command, cwd=ROOT, text=True, capture_output=True, check=False)
        result = {"name": name, "command": command, "exit": run.returncode,
                  "expectedExit": expected, "wallSeconds": round(time.monotonic() - started, 3),
                  "output": run.stdout + run.stderr}
        # An exit 1 must be an actual property violation, not a tool failure.
        valid = run.returncode == expected and (expected == 0 or "Invariant violated" in result["output"])
        result["matchedExpectation"] = valid
        results.append(result)
        (HERE / "verification.json").write_text(json.dumps(results, indent=2) + "\n")
        print(f"{name}: exit {run.returncode}, expected {expected}, {result['wallSeconds']}s", flush=True)
        if not valid:
            print(result["output"], flush=True)
            return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
