# Design: WP07 seven canonical demonstrations

## Context

The family taxonomy suggests seven reference slices. Each slice must isolate
the conservative financial state machine from external protocol machinery.

## Inputs

- WP02 family and facet definitions.
- WP03 Marlowe delta.
- WP04 frozen semantic scope.
- WP05 conformance strategy.

## Outputs

- F1 atomic exchange.
- F2 over-collateralized loan.
- F3 fully escrowed option.
- F4 bounded staking-claim lifecycle.
- F5 reserve-claim state machine.
- F6 bounded allocation mandate.
- Prediction conditional-token market.

Each output includes source, Core, Compact, ZKIR, bounds, manifest, certificate,
client checks, tests, and outside assumptions.

## Decisions

Canonical applications model the bounded value-control kernel. They do not
simulate custodians, legal enforceability, oracle honesty, validator quality,
or discretionary strategy performance.

## Failure Handling

A slice that needs unbounded or undeclared behavior receives an outside-kernel
result. It cannot silently expand Core or claim complete protocol coverage.

## Verification

Run positive, timeout, authorization, invariant, backend, and substitution
tests for every slice. Compare duplicated generator code to shared abstractions.
