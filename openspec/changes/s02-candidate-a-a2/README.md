# A2: Complete actual installment authority lifecycle

Status: specified-only completion contract.
Existing source status is recorded in the roadmap.
This package does not create a passed gate.

## Dependencies

A1.
Read `docs/MORIARTY_ROADMAP.md` from the repository root.
Read XML phase A2 in `deliverables/moriarty-candidate-a-completion-prompt-2026-09-05.xml`.
Read `openspec/WORK-PACKAGES-EARS.md` and its global contract.

## Immutable inputs

Use the handoff snapshot manifest for exact prior-source hashes.
Preserve the controlling XML and semantic scope.
Preserve common authorization and frozen Python semantics.

## Exact outputs

- `specs/quint/s02/candidate_a_authority_installment_fixtures.qnt`
- `specs/quint/s02/candidate_a_authority_installment.qnt`
- `specs/quint/s02/candidate_a_authority_installment_harness.qnt`
- `specs/quint/s02/candidate_a_authority_installment_test.qnt`
- `evidence/s02-candidate-a-completion/a2/manifest.json`
- `evidence/s02-candidate-a-completion/a2/validation.json`

## Acceptance

Each profile and race order has an explicit deterministic positive path; all major actions have nonzero witnesses, never true merely at init. Token conservation, valid escrow coupling, parent accounting, nonce preservation and atomicity hold in the executed traces. Financial terminal means actual N0/zero escrow and resolved attempts, not cancellation alone.

Requirements and failure scenarios are in `specs/s02-candidate-a-a2/spec.md`.
Specification validation does not run those acceptance scenarios.

## Failure outcome

Return `open-a2-acceptance` for missing or inconclusive required evidence.
Preserve failed source and receipts.
Do not change semantic scope to obtain acceptance.
