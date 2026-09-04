# Change: S01 intent-theorem freeze

## Why

S01 needs one architecture-neutral authorization judgment before S02 compares
Core architectures. The package must expose incomplete evidence and ambiguity.

## What changes

- Specify the ten intent-safety acceptance predicates.
- Specify the negative controls and stable failure results.
- Preserve the current semantic scope and all immutable inputs.

## Acceptance predicates

The normative specification defines all ten predicates. These predicates are
requirements, and the repository validator has now recomputed each one as
passing for the bounded local S01 package. This S3 result is not a mechanized
theorem, Task 7 completion, or passage of any prompt release gate.

## Failure outcomes

- `block-s01-on-ambiguity`
- `reject-incomplete-effect-projection`
- `move-architecture-operation-to-s02`
- `reject-mutant-without-widening-intent`

## Impact

This package adds the S01 contract, registries, local exact-transfer checker,
tests, validation report, and evidence manifest. It does not change Core,
semantic scope, prompt inputs, approved design, or old evidence.
