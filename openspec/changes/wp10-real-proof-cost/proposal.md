# Change: WP10 real proof and cost

## Why

Mock ZKIR compilation cannot establish proof feasibility or economic viability.
Moriarty needs measured proving, verification, state, and transaction evidence.

## What Changes

- Run real key generation, proving, verification, and ledger paths.
- Measure distributions for every canonical application and failure control.
- Separate compiler, circuit, proof, ledger, wallet, and provider costs.
- Compare specialized circuits with any viable bounded interpreter candidate.

## Capabilities

### New Capabilities

- `proof-cost-measurement`: Produce reproducible real-proof and operational-cost
  evidence for the candidate language and library fallback.

### Modified Capabilities

None.

## Impact

This package can pass, reduce scope, select library-only, or stop. It does not
accept guessed budgets or mock-proof measurements.
