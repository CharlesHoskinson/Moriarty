# Recursion pipeline Quint model: results (2026-10-26)

The model is `pipeline.qnt` (module `pipeline_model`, parameterized by LEAVES and ARITY) and `pipeline_test.qnt`, written from `BRIEF.md` and `wiki-llm/recursion-perf/stages.json`. A Codex gpt-6.1-sol run wrote the model and tests and then stopped at a model-capacity error. The lead (Claude) added the M1 instance modules and ran every check below with Quint 0.33.0 on this host, under the capped agents slice.

## Commands and outcomes

| Command | Outcome |
|---|---|
| `quint typecheck pipeline.qnt` | ok |
| `quint typecheck pipeline_test.qnt` | ok |
| `quint test pipeline_test.qnt --main pipeline_test` | **36 passing** (4.6 s). Includes the four seeded mutants, each caught: `mutantOverAdmissionCaughtTest`, `mutantChildReuseCaughtTest`, `mutantVkRebindingCaughtTest`, `mutantReorderedBatchCaughtTest`. Also the A4 failure class: `changedVkRefusedAndFailedTest`, `explicitReinstallAllowsNewVkTest`, `reinstallationWhileDecidingRefusedTest`, `consumedParentCannotChangeVkTest` |
| `quint run pipeline.qnt --main pipeline --invariant <inv> --max-steps 40 --max-samples 2000 --seed 0x2a`, for each of memoryCaps, cpuGpuCaps, verifiedChildren, singleUse, vkBinding, orderedBatches, resourceAccounting | no violation for any of them |
| `quint run … --main <m> --invariant <inv>` for m in `pipeline` (1 leaf, arity 2), `pipeline_l2a2`, `pipeline_l4a2`, `pipeline_l4a4` (2,000 samples × 40 steps) and `pipeline_l16a4` (300 samples × 60 steps); inv in allInvariants, vkBinding, orderedBatches | no violation in any of the 15 runs |
| `quint run … --invariant wAdmitted` | a "violation" is found immediately. This is expected: `wAdmitted` is a reachability witness, so the hit shows admission is reachable. It is not a safety property |

## What this establishes, and what it does not

- **Establishes (simulation evidence only):** within the sampled traces, the honest model never exceeds the 32 GB per-run or 90 GB host RAM caps or 20 cores, never lets more than one job use the GPU, never consumes an unverified or already-consumed child, never finalizes a batch whose decider VK differs from the installed contract VK, and finalizes batches in order. The seeded mutants that break these properties are caught by the tests.
- **Does not establish:** exhaustive model checking (no Apalache or TLC run), liveness or fairness, the 16-leaf configuration beyond 300 samples × 60 steps, the timing model's agreement with measured stage times (that is predicate M3, which needs the L0 phase profile), or any correspondence to the Rust prover.
- **Predicates:** M1 (typecheck; invariants for N ∈ {1, 2, 4, 16} and arity 2–4) is met as simulation evidence. M2 (seeded mutants caught) is met by the tests above. M3 and M4 remain open.
