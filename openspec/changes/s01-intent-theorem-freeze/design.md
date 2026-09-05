# Design: S01 intent-theorem freeze

## Boundary

Freeze one architecture-neutral interface. S02 remains responsible for the
architecture decision. S04 remains responsible for mechanization.

## Evidence manifest

The evidence manifest binds immutable inputs and exact outputs with SHA-256
digests. The validator recomputed each local acceptance predicate from
repository files and recorded `recomputed-package-gate-passed`. This bounded S3
package result is not a mechanized theorem and does not pass any of the 24
prompt release gates. Task 7 separately completed whole-package verification
and independent review, including the settlement-process correction.

## Negative controls

The atomic-swap baseline and extra-effect mutant have a local executable test.
The baseline must return `VALID`. The mutant must fail with
`UNAUTHORIZED_EXTRA_EFFECT` without a wider policy. Removing the appended
effect must restore the baseline result.

The local checker is limited to the `SignAfterResolve` flow. It performs
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
