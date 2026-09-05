# A5: Obtain honest bounded model-checking evidence

Status: specified-only completion contract.
Existing source status is recorded in the roadmap.
This package does not create a passed gate.

## Dependencies

A2 A3.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A5 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `evidence/s02-candidate-a-completion/a5/manifest.json`
- `evidence/s02-candidate-a-completion/a5/validation.json`
- `evidence/s02-candidate-a-completion/a5/review.md`
- `evidence/s02-candidate-a-completion/a5/manifest.json`
- `evidence/s02-candidate-a-completion/a5/validation.json`

## Acceptance

Required bounded properties have actual terminal verification evidence. If genuinely externally blocked, record the exact gap and complete other authorized work; do not mark A fully complete.

Requirements and failure scenarios are in `specs/s02-candidate-a-a5/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a5-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
