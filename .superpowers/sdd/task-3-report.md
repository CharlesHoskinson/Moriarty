# Task 3 report: candidate intent-safety judgment

## Scope

Added the architecture-neutral, candidate-unmechanized `INTENT-SAFETY`
judgment artifact and its exact-shape registry test. The artifact remains a
statement of premises and obligations; it is not a theorem proof and does not
grant signing permission.

## RED evidence

The theorem-shape test was added before the judgment artifact. Command:

`/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_registries.py::test_intent_safety_judgment_has_the_frozen_shape -q`

Observed result: `1 failed`. The failure was the expected
`FileNotFoundError` for the missing
`evidence/s01-intent-theorem-freeze/intent-safety-judgment.json` artifact.

## GREEN evidence

Focused theorem command:

`/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_registries.py::test_intent_safety_judgment_has_the_frozen_shape -q`

Observed output:

```text
.                                                                        [100%]
1 passed in 0.04s
```

Full registry command:

`/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_registries.py -q`

Observed output:

```text
.............................                                            [100%]
29 passed in 0.06s
```

`git diff --check` reported no whitespace errors.

## Self-review

- The judgment uses schema id `moriarty.dev/intent-safety-judgment/v1`,
  artifact kind `intent-safety-judgment`, scope `0.0.0-e00.2`, version `1`,
  neutral architecture binding, and candidate-unmechanized status.
- Tests assert all 11 exact premise id/expression pairs, all 11 quantified
  variables, the exact conclusion object, the original binding set plus
  `complete-effect-projection` and `signing-order-profile`, all 13 subsidiary
  claims, and exactly the seven approved exclusions.
- The test checks closure on the judgment, premise records, and conclusion
  record, and rejects the optimization-preference term from the serialized
  judgment.
- The theorem remains architecture neutral. It contains no architecture-
  specific operation, proof, backend correspondence, ledger correspondence,
  or signing permission.
- No schema, registry, Core, prompt, scope, or implementation file was
  changed. The report is the required task record.

## Changed files

- `tests/test_s01_registries.py`
- `evidence/s01-intent-theorem-freeze/intent-safety-judgment.json`
- `.superpowers/sdd/task-3-report.md`

## Concerns and residual boundary

The artifact freezes a candidate interface only. Mechanization remains an S04
obligation, architecture selection remains an S02 obligation, and complete
effect-projection evidence is still a required premise rather than a local
proof. The registry suite currently contains 29 tests; the earlier plan's
`5 passed` estimate is stale.
