# Recursion phase profile (C0)

Status: PROPOSED, 2026-10-26. Nothing in this change is implemented. All requirements are proposed acceptance contracts, not execution receipts.

## Why

All nine wave-1 research clusters found the same gap. The receipts record command totals and proving totals. They do not record exclusive costs for synthesis, commitments, lookups, quotient, opening or key work ([s9 wave 2](../../../wiki-llm/recursion-perf/clusters/s9-orchestration/wave-2/FINDINGS.md)). The cross-review round 1 agreed on this measurement as the first step ([CONSENSUS.md](../../../wiki-llm/recursion-perf/CONSENSUS.md), row C0; [Muse Wonder Full](../../../wiki-llm/recursion-perf/reviews/consensus-1/MUSE-WONDER-FULL.md); [Gemini Argon](../../../wiki-llm/recursion-perf/reviews/consensus-1/AGY-ARGON.md)). The reviewers found that the stage shares assumed by the speedup rows C3, C4, C6, C7 and C9 add up to 216–241% of one proof. A measured profile fixes those shares. It can confirm or reject each row C1–C10 before any accelerator work.

This change can alter an evidence-gated decision: no speedup row becomes "agreed" until the profile gives its deciding metric.

## What changes

- A default-off observation feature on a pinned local copy of published midnight-proofs 0.8.3. This copy lives in a separate profiling workspace, so production builds keep the registry crate and their lockfile.
- Caller timers around stdlib key load, `setup_pk`, prove and verify in the Sherlock proving callers.
- A bounded event collector, an external RSS sampler, process CPU clocks, typed byte counters and a JSON sidecar that conforms to `profile.schema.json`.
- A factory research runner `scripts/qa/profile-recursion.py` with the commands `prepare`, `calibrate`, `one`, `reduce` and `decide`. All builds and proofs go through `scripts/doctorwatson heavy`.
- A reducer that reconciles phases with command receipts and writes one deciding metric for each row C1–C10.

## Impact

Affected artifacts: `tools/profile/` (new workspace and patched crate), `crates/mori-recursion` and `crates/mori-backend` (feature-gated caller timers only), `scripts/qa/profile-recursion.py`, `scripts/qa/tests/test_profile_*.py`, `qa/sherlock/profiles/` (sidecars). `qa/holdout/` stays evaluator-owned. The factory receipt schema does not change.

## Non-goals

This change claims a 1.00× speedup. It adds no GPU kernel, proof system, SRS, scheduler concurrency, key cache, proof market, circuit or ledger deployment. It does not change the PLONK/KZG protocol, BLS12-381, the official SRS, any transcript or any verifying key. The public build and proving path does not need the profiling feature.
