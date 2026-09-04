# Design: S01 intent-theorem freeze

## Boundary

Freeze one architecture-neutral interface. S02 remains responsible for the
architecture decision. S04 remains responsible for mechanization.

## Evidence manifest

The later evidence manifest must bind immutable inputs and exact outputs with
SHA-256 digests. A later validator must recompute each acceptance predicate from
repository files. This Task 1 package supplies no validation result.

## Negative controls

The atomic-swap baseline and extra-effect mutant are specified test vectors.
Their checks have not run in Task 1. The mutant must fail with
`UNAUTHORIZED_EXTRA_EFFECT` without a wider signed intent.

The eventual checker is limited to the `SignAfterResolve` flow. It performs
exact transfer comparison only. It cannot establish authenticated effect completeness.
It cannot authorize signing.

The complete effect projection remains open. The boundaries for
`SignBeforeResolve` remain open. Later tasks must not report these gaps as
resolved without new evidence and an approved scope change.

## Rollback

Rollback removes only S01 outputs. Rollback preserves all prior immutable
evidence and semantic-scope files.

## Semantic-scope transition

S01 specifies an evidence-only transition. Keep semantic scope at
`0.0.0-e00.2`. Keep its digest unchanged. S02 still owns the architecture
decision.
