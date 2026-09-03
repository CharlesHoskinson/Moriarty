# Design: WP07 seven canonical demonstrations

## Context

The family taxonomy suggests seven reference slices. Each slice must isolate
the conservative financial state machine from external protocol machinery.

## Inputs

- WP01 backend feasibility and its post-WP04 reproduction result.
- WP02 family and facet definitions.
- WP03 Marlowe delta.
- WP04 frozen semantic scope.
- WP05 conformance strategy.
- WP06 conditional-token disposition and artifacts.

## Outputs

- F1 atomic exchange.
- F2 over-collateralized loan.
- F3 fully escrowed option.
- F4 bounded staking-claim lifecycle.
- F5 reserve-claim state machine.
- F6 bounded allocation mandate.
- Prediction conditional-token market.

Each slice resides under `experiments/moriarty-family-slices/<slice-id>/`.
Each output includes source, Core, Compact, ZKIR, bounds, manifest, certificate,
client checks, tests, and outside assumptions.

## Decisions

Canonical applications model the bounded value-control kernel. They do not
simulate custodians, legal enforceability, oracle honesty, validator quality,
or discretionary strategy performance.

## Failure Handling

A slice that needs unbounded or undeclared behavior receives an outside-kernel
result. It cannot silently expand Core or claim complete protocol coverage.
The primary gate requires all seven slices to compile. If WP06 rejects the
Prediction slice, the package records `six-family-evidence` and does not pass the
seven-family language gate.

## Verification

Run positive, timeout, authorization, invariant, backend, and substitution
tests for every slice. Compare duplicated generator code to shared abstractions.
Run `uv run python scripts/validate_sprint_evidence.py --package WP07 --manifest
openspec/work-packages.json`.
