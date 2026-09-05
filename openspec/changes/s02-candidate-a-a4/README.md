# A4: Export actual integrated records and independently compare

Status: specified-only completion contract.
Existing source status is recorded in the roadmap.
This package does not create a passed gate.

## Dependencies

A2 A3.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A4 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `evidence/s02-candidate-a-completion/a4/manifest.json`
- `evidence/s02-candidate-a-completion/a4/validation.json`
- `evidence/s02-candidate-a-completion/a4/review.md`
- `evidence/s02-candidate-a-completion/a4/manifest.json`
- `evidence/s02-candidate-a-completion/a4/validation.json`

## Acceptance

Full mandatory integrated inventory agrees with the independent checks; every security-critical mutant is rejected or explicitly blocks acceptance. A precomputed expected-result fixture is not an executed export.

Requirements and failure scenarios are in `specs/s02-candidate-a-a4/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a4-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
