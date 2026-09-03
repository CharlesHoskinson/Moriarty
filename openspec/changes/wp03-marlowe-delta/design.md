# Design: WP03 Marlowe semantic delta

## Context

Marlowe V1 has distinct Isabelle, Haskell, Plutus, Agda, JSON, and TypeScript
artifacts. Their coverage differs. The delta must preserve disagreement.

## Inputs

- `evidence/repository-locks-2026-09-02.tsv`.
- `evidence/marlowe-docs-live-acquisition-2026-09-02.json`.
- The V1 syntax and transition reconstruction.
- Existing formal claims and contradiction records.
- The WP02 protocol roster for final coverage alignment.

## Outputs

- `evidence/wp03/v1-construct-inventory.json`.
- `evidence/wp03/marlowe-moriarty-delta.csv`.
- `evidence/wp03/toolchain-lock.json` and `trace-vectors.json`.
- `evidence/wp03/correspondence-results.json` and `evidence-manifest.json`.

## Decisions

No implementation is normative by default. Abstract non-locking remains
separate from ledger feasibility, continuation availability, and participant
cooperation.

## Failure Handling

Conflicting sources remain explicit. An unsupported inheritance claim becomes
new proof work. Missing implementation correspondence blocks compatibility.
A row without a reproducible definition cannot enter the WP04 Core freeze.

## Verification

Trace representative contracts through interval fixing, reduction, input
application, continuation selection, validation, and payout. Check every row
against pinned source code. Join the delta to every identifier in
`v1-construct-inventory.json`. Run `uv run python
scripts/validate_sprint_evidence.py --package WP03 --manifest
openspec/work-packages.json`.
