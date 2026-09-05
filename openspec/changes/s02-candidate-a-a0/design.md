# Design: A0

## Context and decisions

Use the adopted Candidate A authority contract.
Use the shared-state model and unchanged common transitions.
The XML phase A0 provides the detailed execution sequence.

## Dependencies and inputs

Dependencies: existing boundary source and retained evidence.
Read the exact source inventory in the handoff snapshot manifest.
Read the controlling S02 acceptance specification.

## Outputs and evidence boundary

- `evidence/s02-candidate-a-completion/a0/manifest.json`
- `evidence/s02-candidate-a-completion/a0/validation.json`
- `evidence/s02-candidate-a-completion/a0/review.md`

Place the acceptance manifest at `evidence/s02-candidate-a-completion/a0/manifest.json`.
Place the recomputed result at `evidence/s02-candidate-a-completion/a0/validation.json`.
These paths are planned outputs, not existing evidence.

## Verification design

Map each requirement to a named deterministic check.
Map each lifecycle case to both signing profiles.
Preserve complete expected observations before implementation.
Keep actual source closures with every runtime receipt.
Keep sampled, bounded, correspondence, and review evidence separate.

## Failure handling and rollback

Preserve the last reviewed source commit before each change.
Retain failed source and terminal command receipts.
Refuse package acceptance when a required check is incomplete.
Use a reviewed revert commit for necessary rollback.
Do not reset the worktree or rewrite retained evidence.

## Semantic scope

This package changes no normative Core scope.
Record a demonstrated conflict before requesting a scope change.
Independent review and Council remain separate acceptance obligations.
