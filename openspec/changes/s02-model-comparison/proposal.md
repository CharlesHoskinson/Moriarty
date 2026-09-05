# Change: S02 bounded model-comparison contract

## Why

S02 needs one closed evidence contract before four architecture representations
are implemented. The contract must permit evidence-backed rejection, selection
of a safe alternative, or a justified stop without treating unperformed work as
infeasibility.

## What changes

- Specify ten package gates for four distinct Quint representations, two
  workloads, and both signing profiles.
- Specify the common safety and nonvacuity floor, positive witnesses, negative
  controls, independent E00 comparison, and fail-closed evidence manifest.
- Separate completion of required checks from candidate eligibility.
- Preserve the frozen Core, swap, S01 judgment, and semantic scope.

## Acceptance predicates

The normative specification defines S02-01 through S02-10 as obligations only.
They have no passed status until the future validator independently recomputes
the complete evidence gate. A candidate with a preserved safety counterexample
is ineligible, but that rejection does not invalidate decisive evidence for a
safe alternative or an evidence-backed stop decision.

## Failure outcomes

- `block-s02-on-incomplete-evidence`
- `reject-architecture-on-counterexample`
- `require-discriminating-experiment`
- `stop-language-path-with-evidence`

Missing work is not infeasibility and cannot justify a stop decision.

## Impact

This change adds only the specification package and its boundary test. It adds
no `.qnt` model, generated receipt, validation result, selected candidate, Core
motion, or semantic-scope change.
