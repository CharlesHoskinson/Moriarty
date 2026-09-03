# Design: WP06 high-risk composition

## Context

Prediction markets need conditional claim creation and redemption. A safe
kernel must conserve the backing asset and bind resolution to one event domain.

## Inputs

- WP01 backend constraints.
- WP04 decisions for Attest, authority, payment, and split or merge.
- WP05 theorem and conformance strategy.

## Outputs

- A binary conditional-token state machine.
- An atomic conditional-token exchange application.
- Adversarial traces and minimized counterexamples.
- Deterministic Compact, ZKIR, manifest, and certificate artifacts.

## Decisions

The Core never treats an oracle assertion as truth without a named capability.
Split and merge conserve backing under a declared token policy and event.

## Failure Handling

A conservation, authorization, replay, or substitution failure rejects Core
inclusion. A backend-only success cannot override a semantic failure.

## Verification

Run model, property, differential, compiler, and artifact-verification tests.
Preserve invalid attestation, replay, deadline-boundary, and refund vectors.
