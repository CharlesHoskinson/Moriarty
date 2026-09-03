# Change: WP04 Core semantic scope

## Why

The Core cannot freeze while its numeric, action, authority, oracle, and state
semantics remain optional. Each motion needs one explicit disposition.

## What Changes

- Decide numbers, partial payments, and atomic action collection.
- Decide Attest, authority, bounded mandate, and conditional split or merge.
- Define state, lifetime, and transaction bounds.
- Maintain a versioned semantic scope ledger after every accepted motion.
- Move rejected or deferred behavior outside the Core.

## Capabilities

### New Capabilities

- `semantic-motions`: Produce complete decision records for the smallest Core
  semantics needed by Moriarty demonstrations.

### Modified Capabilities

None.

## Impact

This package controls the Core freeze and the input to proof, composition, SDK,
and application packages. It does not implement accepted motions.
