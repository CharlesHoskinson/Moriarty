# Change: WP06 high-risk composition

## Why

Conditional-token composition combines oracle, authorization, token, atomicity,
and refund risks. It is the strongest test of safe DeFi extension boundaries.

## What Changes

- Specify binary conditional-token split and merge.
- Compose conditional tokens with atomic exchange.
- Exercise invalid attestation, replay, timeout, refund, and partial payment.
- Preserve deterministic build and disclosure evidence.

## Capabilities

### New Capabilities

- `high-risk-composition`: Test whether conditional claims can compose with
  exchange without weakening Core guarantees.

### Modified Capabilities

None.

## Impact

This package can promote, revise, defer, or reject split and merge. It does not
authorize arbitrary minting, arbitrary Compact calls, or oracle trust.
