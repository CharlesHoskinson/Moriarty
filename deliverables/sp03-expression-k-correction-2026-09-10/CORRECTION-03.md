# Expression K correction 03

`source-candidate-03.json` binds the preserved full40 candidate and the three repairs below. The prior GPT-6 and Grok reviews are retained unchanged under `deliverables/sp03-expression-k-2026-09-10/`; their `REQUEST_CHANGES` verdicts do not approve this changed candidate.

1. `exInput` now applies the 65536-byte schema limit before J-domain validation, matching `createExpressionContractV1`. The fixture generator adds `invalid-schema-domain-65536` (invalid J domain, expected `INPUT_SCHEMA`) and `invalid-schema-domain-65537` (same domain, expected `INPUT_BOUND`).
2. `command()` enforces the declared post-capture limit on `stdout` plus `stderr`, including an exact-boundary control and a one-byte-over control. It retains both captured files.
3. Semantic expression artifacts remain J-canonical. Only `*.command.json` telemetry uses ordinary JSON, so actual float `elapsedSeconds` and `aggregateSeconds` are retained and an already-counted attempt is never reset.

The candidate has 115 fixtures. `resource-proposal-03.json` requests, but does not consume, a reviewed delta of two krun calls and forty seconds: 1 compile, 115 traces, 2552 seconds total. It binds only `/home/charl/Moriarty/.worktrees/roadmap-loop-prompt`; the preserved original worktree remains untouched.

Offline validation before the repairs was red: the runner failed with `EXPRESSION_TRANSPORT`, the aggregate-limit test had no limit table, and the K-order assertion failed. After the repairs, Python runner/source tests pass (9 tests) and the published TypeScript fixture comparison passes all 115 cases. Parent separately observed real harmless subprocess telemetry at the 64 MiB aggregate boundary, one byte above it, and stderr-only overflow; no K/native command ran.

Remaining obligations: fresh Astra medium and Grok 4.6 high exact source/resource reviews, root preflight/admission, then at most the separately admitted bounded K command and fresh result reviews. This document makes no K correspondence, compile, proof, financial, ledger, network, or publication claim.
