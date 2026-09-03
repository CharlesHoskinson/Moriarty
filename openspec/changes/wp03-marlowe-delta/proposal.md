# Change: WP03 Marlowe semantic delta

## Why

Moriarty must not inherit Marlowe guarantees by analogy. Each retained or
changed construct needs an exact semantics, implementation locator, and proof
premise.

## What Changes

- Build a construct-by-construct V1-to-Moriarty delta matrix.
- Correct claims about tokens, parties, inputs, time, partial pay, and continuations.
- Separate abstract semantics from Cardano realization.
- Identify every new proof obligation.

## Capabilities

### New Capabilities

- `marlowe-delta`: Reconcile Marlowe specifications and implementations against
  the proposed Moriarty Core.

### Modified Capabilities

None.

## Impact

This package updates the Marlowe baseline, formal-assurance matrix, and semantic
motion inputs. It does not authorize a compatibility claim or translator.
