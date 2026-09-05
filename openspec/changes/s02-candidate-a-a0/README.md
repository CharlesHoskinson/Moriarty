# A0: Recover and accept the completed boundary unit

Status: local A0 intake accepted on 2026-09-05.
Source: d14cfea98a1c9213e5ef5f12c1a088f4e966083d.
Evidence: 0190cb97270e4775273cb3737dc649d874861e91 and
95899b37fa03ccb61506e51c099dd6aea96bcc16 on s02-model-comparison.
This does not pass Council, S02, or a full XML release gate.

## Dependencies

Existing implementation anchor d14cfea98a1c9213e5ef5f12c1a088f4e966083d.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A0 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `evidence/s02-candidate-a-completion/a0/manifest.json`
- `evidence/s02-candidate-a-completion/a0/validation.json`
- `evidence/s02-candidate-a-completion/a0/review.md`

## Acceptance

Corrected whole-unit source/spec/runtime/evidence review is recorded; no missing or mismatched receipt is silently passed. This closes only local A0, not Council.

Requirements and failure scenarios are in `specs/s02-candidate-a-a0/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a0-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
