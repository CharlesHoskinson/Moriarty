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
requirements, not passed results. The package status remains specified-only and
in-progress until later tasks produce and validate the evidence.

## Failure outcomes

- `block-s01-on-ambiguity`
- `reject-incomplete-effect-projection`
- `move-architecture-operation-to-s02`
- `reject-mutant-without-widening-intent`

## Impact

This package adds only S01 contract files and their structure test. It does not
change Core, semantic scope, prompt inputs, approved design, or old evidence.

