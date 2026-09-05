# A6: Independent acceptance, Council and dossier

Status: specified-only completion contract.
Existing source status is recorded in the roadmap.
This package does not create a passed gate.

## Dependencies

A0 A1 A2 A3 A4 A5.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A6 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `evidence/s02-candidate-a-completion/a6/manifest.json`
- `evidence/s02-candidate-a-completion/a6/validation.json`
- `evidence/s02-candidate-a-completion/a6/review.md`
- `evidence/s02-candidate-a-completion/a6/manifest.json`
- `evidence/s02-candidate-a-completion/a6/validation.json`

## Acceptance

Each A obligation has explicit pass/fail/open evidence. No material unresolved finding, missing formal result or missing required Council verdict is hidden by an aggregate pass.

Requirements and failure scenarios are in `specs/s02-candidate-a-a6/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a6-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
