# Design: WP06 high-risk composition

## Context

Prediction markets need conditional claim creation and redemption. A safe
kernel must conserve the backing asset and bind resolution to one event domain.

## Inputs

- WP01 backend constraints.
- WP04 decisions for Attest, authority, payment, and split or merge.
- WP05 theorem and conformance strategy.

## Outputs

- `experiments/moriarty-conditional-exchange/` source and generated artifacts.
- `artifact-manifest.json` and `translation-certificate.json`.
- `adversarial-results.json` with minimized counterexamples.
- `evidence/wp06/evidence-manifest.json`.

## Decisions

The Core never treats an oracle assertion as truth without a named capability.
Split and merge conserve backing under a declared token policy and event.
Any promotion, revision, or rejection returns to WP04 as a new motion. It creates
a new semantic version, immutable snapshot, index entry, and journal digest
transition before WP07 starts.

## Failure Handling

A conservation, authorization, replay, or substitution failure rejects Core
inclusion. A backend-only success cannot override a semantic failure.

## Verification

Run model, property, differential, compiler, and artifact-verification tests.
Preserve invalid attestation, replay, deadline-boundary, and refund vectors.
Extend the WP01 certificate validator with the new Core and disclosure digests.
Run `uv run python scripts/validate_sprint_evidence.py --package WP06 --manifest
openspec/work-packages.json`.
