# A3: Complete actual swap authority lifecycle

Status: locally executed and reviewed at experimental aggregate `28d35d8`;
required cross-provider Council and integration remain open.
Existing source status is recorded in the roadmap.
This package does not create a passed gate.

## Dependencies

A1.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A3 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `specs/quint/s02/candidate_a_authority_swap_fixtures.qnt`
- `specs/quint/s02/candidate_a_authority_swap.qnt`
- `specs/quint/s02/candidate_a_authority_swap_harness.qnt`
- `specs/quint/s02/candidate_a_authority_swap_test.qnt`
- `evidence/s02-candidate-a-completion/a3/manifest.json`
- `evidence/s02-candidate-a-completion/a3/validation.json`

## Acceptance

Both profiles execute each required positive financial path and retained rejection path from actual initial funds. No generic string fixture supplies the result. Finite attempt-ID limits remain explicit.

Requirements and failure scenarios are in `specs/s02-candidate-a-a3/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a3-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
