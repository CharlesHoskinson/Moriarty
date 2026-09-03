# Design: WP04 semantic motions

## Context

The candidate Core preserves finite Marlowe-style contracts. Proposed DeFi
extensions can weaken determinism, conservation, authorization, or boundedness.

## Inputs

- WP01 backend restrictions and disclosure evidence.
- WP02 DeFiFormal demand and residue evidence.
- WP03 semantic and proof-premise matrix.

## Outputs

- One decision record under `evidence/wp04/motions/` per semantic motion.
- Operational rules and typed failure results.
- Resource-bound equations and privacy disclosures.
- A disposition of accept, revise, defer, outside-core, or reject.
- An immutable snapshot under `evidence/semantic-scope/`.
- `evidence/wp04/model-check-results.json` and `evidence-manifest.json`.

## Decisions

Prefer elaboration and libraries over Core growth. Accept a Core construct only
when it has demand evidence, finite semantics, and a credible proof path.
Every accepted motion changes the prerelease semantic version. The motion record
names the previous and new snapshot digests. The index and wiki journal record
the same transition.

## Failure Handling

An unresolved motion blocks the Core freeze. If two semantics remain viable,
defer the feature or keep it outside the Core.

## Verification

Use transition tables, counterexamples, and finite model tests. Record race,
front-running, timeout, authorization, and resource consequences.
Run `uv run python scripts/validate_sprint_evidence.py --package WP04 --manifest
openspec/work-packages.json`.
