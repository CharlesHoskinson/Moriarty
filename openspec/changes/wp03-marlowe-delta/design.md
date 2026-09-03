# Design: WP03 Marlowe semantic delta

## Context

Marlowe V1 has distinct Isabelle, Haskell, Plutus, Agda, JSON, and TypeScript
artifacts. Their coverage differs. The delta must preserve disagreement.

## Inputs

- Pinned Marlowe repositories and source receipts.
- The V1 syntax and transition reconstruction.
- Existing formal claims and contradiction records.
- The WP02 protocol roster for final coverage alignment.

## Outputs

- A row for each V1 construct, warning, error, and realization boundary.
- Exact source and proof locators.
- A disposition of preserve, change, surface-only, runtime-only, or reject.
- New theorem, conformance, serialization, and migration obligations.

## Decisions

No implementation is normative by default. Abstract non-locking remains
separate from ledger feasibility, continuation availability, and participant
cooperation.

## Failure Handling

Conflicting sources remain explicit. An unsupported inheritance claim becomes
new proof work. Missing implementation correspondence blocks compatibility.

## Verification

Trace representative contracts through interval fixing, reduction, input
application, continuation selection, validation, and payout. Check every row
against pinned source code.
