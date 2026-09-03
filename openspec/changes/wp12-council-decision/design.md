# Design: WP12 terminal council decision

## Context

WP01 through WP11 create independent stop gates. The terminal decision must use
their frozen results without allowing a reviewer to redefine acceptance.

## Inputs

- Immutable outputs and gate results from WP01 through WP11.
- The current semantic scope ledger and contradiction register.
- Precommitted scorecard weights and decision thresholds.

## Outputs

- One blinded review bundle with digest and manifest.
- Grok, Sol, and exact Fable 5.1 per-package reviews.
- A dissent and abstention register.
- A signed decision record with one terminal outcome.

## Decisions

The decision authority considers Council advice, but owns the result. Provider
failure is an abstention. It is never approval. Prior stop-gate failures remain
binding.

## Failure Handling

If evidence is insufficient, the language option fails. The decision selects
audited libraries or stop. It does not select indefinite exploration.

## Verification

Verify provider and model terminal evidence. Verify review-bundle identity.
Check every score against preserved evidence and every unresolved dissent.
