# Task 2 report: S01 registries and schema

## Scope

Created the combined Draft 2020-12 schema, seven S01 registries, and focused
registry tests. This task did not create the theorem artifact, validation
receipt, evidence manifest, or validator.

## RED evidence

The tests were written before the schema or registries. Command:

`/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_registries.py -q`

Observed result: 17 failures. The failures were the expected
`FileNotFoundError` results for the missing schema and registry artifacts.

## GREEN evidence

After the artifacts were created, the focused command reported 23 passed.
`Draft202012Validator.check_schema` also accepted the combined schema, and an
independent inventory check found 69 unique canonical nouns and 20 unique XML
aliases. `git diff --check` reported no whitespace errors.

## Interface and schema decisions

- Each of the ten `$defs` entries is self-contained and uses no `$ref`.
- Registry top-level objects and every fixed-shape nested record reject unknown
  properties. Tests mutate every registry at the top level and record level,
  and also exercise wrong types and missing required metadata.
- Registry metadata uses `scope_version`; the existing vector keeps its
  established `semantic_scope_version`, `artifact_kind`, `scope`, and
  `resolution` fields.
- `judgment` supports the frozen Task 3 shape. Its binding array can carry the
  audit supplement's `complete-effect-projection` and
  `signing-order-profile` bindings in addition to the original bindings.
- `planVector`, `validationReport`, and `evidenceManifest` are defined now so
  later tasks use the same schema. No passing report is published by this task.
- Terminology includes every XML alias, all 27 W4 objects, W5 and theorem nouns,
  the six distinct partial-fill/atomicity kinds, `DisplayProjection`, and
  `AuthorizationProjection`.
- `CrossDomainAllOrRefund` has global semantics: every required domain fulfills
  or every affected domain satisfies the jointly authorized refund predicate.
  Mixed fulfillment/refund is invalid; compensation is not refund; named
  finality and liveness assumptions remain premises.
- `AuthorizationProjection` requires complete inclusion and exclusion evidence
  for all effects. `DisplayProjection` is never authorization evidence, and
  declassification changes visibility only.
- `SignAfterResolve` and `SignBeforeResolve` have separate authority boundaries.
  Both are explicitly recorded as not implemented, and S01 makes no proof claim.
- G17 remains two separate empirical obligations. Recording its resolved
  predicate does not pass either human pilots or ACTUS.

## Produced interfaces

The seven registries validate independently through `$defs` named
`terminology`, `lifecycleObjects`, `observationModel`, `predicateRegistry`,
`assumptionRegistry`, and `ambiguityResolutions`. The schema also exposes
`judgment`, `planVector`, `validationReport`, and `evidenceManifest` for Tasks 3
and 5.
