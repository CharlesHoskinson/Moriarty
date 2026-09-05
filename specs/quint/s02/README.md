# S02 effect arithmetic foundation

This directory contains the executable S02 effect foundation. It models pure
transfer arithmetic and a small non-candidate deposit and refund harness.

## Model boundary

- `effects.qnt` defines `Transfer`, `Ledger`, `canApply`, `applyTransfers`,
  `policyAllows`, and `totalAsset`.
- Wallet and escrow are separate locations.
- The ledger contains every modeled location and asset key.
- Transfers preserve order and multiplicity.
- `canApply` checks each prefix before an update can authorize execution.
- `applyTransfers` computes a candidate ledger only.
- `effects_harness.qnt` instantiates both initial balances and runs deposit,
  then refund, in a terminal two-action path.
- `effects_test.qnt` contains ten discovered behavior tests.

The harness does not implement full authorization, candidate execution, signed
policy verification, Core correspondence, signing, proof, or a ledger claim.
It does not pass the S02 gate. It does not select an architecture.

## Evidence

Evidence is in `evidence/s02-model-comparison/foundation/`. The deterministic
ITF receipt is `effects-receipt_0.itf.json`. Its SHA-256 is
`04c3b3d8febde81855662e440b7471cfa6f281248d6643d0c5edac878aed2378`.

The 10,000-sample run checks `safety` and reports `deposited` and `refunded`.
The run reaches both witnesses in 10,000 of 10,000 traces.

## Quint command note

Quint 0.32.0 includes imported definitions when `--match '.*'` selects tests.
That command therefore attempts 34 non-test definitions and fails with
evaluator errors. Use `--match 'Test$'` to discover the ten `run` tests.
The import correction instantiates `effects_machine` in the test module and
imports pure effects separately. This correction does not change a predicate.
