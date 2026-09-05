# S02 common Quint foundations

This directory contains incremental common models for the S02 architecture
comparison. These are not candidate A–D interpreters or a passed S02 gate.

## Implemented common units

| Module | Responsibility | Evidence directory |
| --- | --- | --- |
| `effects.qnt` | Ordered transfer arithmetic and conservation | `foundation/` |
| `consumption.qnt` | Exact parent claims, slots, cancellation, and residuals | `consumption/` |
| `observations.qnt` | Neutral observations and structural Core result checks | `observations/` |
| `policies.qnt` | Canonical branch policies and complete plan bindings | `policies/` |
| `authorization.qnt` | Persistent symbolic signatures and fresh pre-sign checks | `authorization/` |
| `execution.qnt` | Per-operation verification, atomic commit, and retained rejection | `execution/`, `rejection/` |

Evidence directories are under `evidence/s02-model-comparison/`. Read their
source pins and scope before reusing results; historical receipts are not
automatically current after an imported module changes.

`execution_harness.qnt` executes both signing profiles from prefunded unsigned
swap escrow through check, sign, propose, verify, and atomic settlement.
`rejection_harness.qnt` isolates missing-proof and stale-context rejection from
a constructed signed context; run it with `--init=rejectionInit
--step=rejectionStep`. Rejection preserves money and is not settlement.

External evidence dispositions are trusted abstractions. They do not derive
candidate semantics, cryptographic validity, or Core correspondence.
`installment_harness.qnt` separately executes both-profile initial fill/cancel
races, explicit stale-loser rejection, and then either both fills or a separately
checked/signed recovery of ten or five. Its scope and root receipts are in
`installment/`; it does not trace funding, time evolution, or a second concurrent
slot-two/cancellation race. Candidate
A–D execution, independent correspondence, mutation controls, Quint/Apalache
checking, architecture selection, and requested Council gates remain required.

## Effect arithmetic boundary

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

## Parent consumption foundation

`consumption.qnt` checks one exact-parent nonce entry for the fixed two-slot
installment workload. Its generic policy payload is compared structurally.
Slot ordering, exact residuals, cancellation, and stale prepared snapshots
are checked without wrapping revisions. The separate harness has no signing
event and transfers no money. Its policy record is an equality-test fixture,
not a complete signed envelope. These guards are bookkeeping prerequisites,
not authorization. Full caller-side signature, registry ownership, conditions,
candidate-semantic checks, and separately authorized refund recovery remain
required. A cancellation witness here does not satisfy S02 recovery coverage.
