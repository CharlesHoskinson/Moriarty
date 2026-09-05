# A1: Correct and adopt concrete lifecycle plans

Status: local A1 planning accepted at experimental-branch commit `82d2c0b`.
Both plans have nonauthor semantic approval and passing final static typechecks.
This is not A2/A3 behavioral acceptance or a Council/release gate.

## Dependencies

A0.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A1 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `evidence/s02-candidate-a-completion/a1/manifest.json`
- `evidence/s02-candidate-a-completion/a1/validation.json`
- `evidence/s02-candidate-a-completion/a1/review.md`
- Exact assembled source modules and two raw typecheck receipts in the same directory.

These outputs are committed on `s02-model-comparison`, not integrated into main.

## Acceptance

Both plans trace every case below to a deterministic test and observable action witness, and are source-compatible with existing common and A contracts.

Requirements and failure scenarios are in `specs/s02-candidate-a-a1/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a1-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
