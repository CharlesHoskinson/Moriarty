# Task 1 report

## Status

Task 1 is complete. The S01 package remains specified-only and in-progress.
Tasks 2 through 7 remain open.

## RED

Command:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_openspec.py -q
```

Result: `3 failed in 0.03s`.

The structure test found no package files. The normative specification file did
not exist. The audit-scope test found no status or checker boundary.

## GREEN

The first GREEN run returned `2 passed, 1 failed`. The remaining failure found a
wrapped required phrase. The second repository run returned `79 passed, 1
failed` for the same phrase. I corrected the prose without changing the scope.

Final focused command:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_openspec.py -q
```

Final focused result: `3 passed in 0.00s`.

## Self-review

- The package contains exactly the six required OpenSpec files.
- The normative specification contains exactly ten scenarios in the required order.
- The package labels all ten predicates as requirements, not passed results.
- The package status is specified-only and in-progress.
- The checker scope is `SignAfterResolve` and exact transfer comparison only.
- The checker cannot establish authenticated effect completeness.
- The checker cannot authorize signing.
- Complete effect projection remains open.
- The `SignBeforeResolve` boundaries remain open.
- Only Task 1 is selected in the seven-task checklist.
- No Task 2 through Task 7 file was created or changed.
- No prompt, approved design, Core, scope, or old evidence file was changed.

## Reviewer correction

Added an explicit `VALID` result for the Scenario 8 local baseline comparison.
The signing and authenticated-effect-completeness limits remain unchanged.

RED command and output:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_openspec.py -q
.F.                                                                      [100%]
=================================== FAILURES ===================================
__________________ test_s01_normative_spec_has_ten_scenarios ___________________

    def test_s01_normative_spec_has_ten_scenarios() -> None:
        text = (CHANGE / "specs/intent-safety/spec.md").read_text(encoding="utf-8")
        assert text.count("#### Scenario:") == 10
        assert "UNAUTHORIZED_EXTRA_EFFECT" in text
        assert "optimization preference" in text
>       assert "local comparison result SHALL be `VALID`" in text
E       AssertionError: assert 'local comparison result SHALL be `VALID`' in '# Intent-safety specification\\n\\n## ADDED Requirements\\n\\nThe following ten acceptance predicates are requirements, n...` boundaries remain\\nopen. An architecture-specific Core operation produces\\n`move-architecture-operation-to-s02`.\\n\\n'

tests/test_s01_openspec.py:48: AssertionError
=========================== short test summary info ============================
FAILED tests/test_s01_openspec.py::test_s01_normative_spec_has_ten_scenarios
1 failed, 2 passed in 0.01s
```

GREEN command and output:

```text
/home/charl/Moriarty/.venv/bin/python -m pytest tests/test_s01_openspec.py -q
...                                                                      [100%]
3 passed in 0.01s
```
