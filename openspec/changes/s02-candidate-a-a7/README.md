# A7: Commit, integrate and hand off without scope inflation

Status: specified-only completion contract.
Existing source status is recorded in the roadmap.
This package does not create a passed gate.

## Dependencies

A6.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A7 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `evidence/s02-candidate-a-completion/a7/manifest.json`
- `evidence/s02-candidate-a-completion/a7/validation.json`
- `evidence/s02-candidate-a-completion/a7/review.md`
- `evidence/s02-candidate-a-completion/a7/manifest.json`
- `evidence/s02-candidate-a-completion/a7/validation.json`

## Acceptance

A self-contained Candidate A dossier names exact commits, commands, results, review evidence, limitations, integration state and the next S02 obligation.

Requirements and failure scenarios are in `specs/s02-candidate-a-a7/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a7-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
