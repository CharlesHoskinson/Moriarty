# Response matrix: review findings and minimum repairs → repaired `operator-proposal/run_r3.py`

All entries are source readings. Nothing was executed or parsed. Line numbers refer to the repaired file. Root's diff against `d2ddbc3c…` is authoritative.

## Edits

| Edit | Required text (OPERATOR-MINIMUM-REPAIRS.md) | Applied | Where | Notes |
|---|---|---|---|---|
| E-1 | `put()` source read in `try/except OSError` → `source unreadable` STOP | Yes, verbatim | 302–307 | The loop continues, so every unreadable source is listed. Destination-side errors still propagate to the E-2 handler. |
| E-2a | Remove trailing `return finish(result)` after the loop | Yes, verbatim, before E-2c | after 652 | `attempt()` ends at `break`. |
| E-2b | Refusal before the marker; marker; `try: journal + attempt / except Exception`; single `finish`; new `def attempt` | Yes, verbatim | 550–570 | `RUN.mkdir()` and the `result` dict sit outside the `try`. |
| E-2c | Replace all ten eight-space `return finish(result)` with `return` | Yes, ten replacements, confirmed by read-back | 573, 578, 583, 588, 593, 609, 614, 625, 634, 637 | The four-space line 565 in `main()` did not match. |
| E-2d | Docstring Effects lines | Yes, verbatim | 13–15 | Record only. |
| E-3 (optional) | `hw_rule` quotes Q-1's general rule | Yes, verbatim | 737–739 | String only. |

## E-2 "Expected result" checklist

| Expected | My reading of the bytes |
|---|---|
| `main()` writes nothing and returns 2 when not isolated, wrong task name or wrong path | Holds (553–556). One stdout line; no filesystem write. |
| After `RUN.mkdir()`, `attempt(result)` runs inside `try/except Exception`; an exception records `operator_exception` and one STOP | Holds (559–564). |
| `finish(result)` runs exactly once and `main()` returns its value | Holds. The only call site is 565. |
| `attempt()` is the former body with early returns bare and the loop last | Holds (568–652). |
| `finish()`, `post_run()` (except E-3), check functions, constants and argv byte-unchanged; `proposal_mismatches()` unaffected | Holds by my reading. Root diff confirms. |
| An exception after `Popen` can leave the child alive while `finish()` runs | Confirmed. Recorded as residual 1 in REPORT §4. |

## Findings

| Finding | Disposition in these bytes |
|---|---|
| OR-01 Medium (failure propagation, R-C.7) | **Addressed** by E-1 and E-2, within `except Exception`. Residuals 1–3 in REPORT §4. |
| OR-02 Low (placement after marker) | **Addressed** by E-2b. Its limits are residual 4. |
| OR-03 Low (`hw_rule` wording) | **Addressed** by E-3. A result review must still apply Q-1 and not quote the analysis hw block alone. |
| OR-04 Info (collision/escape STOPs uncontrolled) | Unchanged; source-argued. |
| OR-05 Info (HW-24 / Q-2) | Unchanged. Not in this source scope. No vote from me. |
| OR-06 Info (whole-run resources) | Unchanged. The repair adds no spawn (residual 6). |
| OR-07 Info (listing metadata) | Unchanged. |
| OR-08 Info (residual TOCTOU) | Unchanged. |
| OR-09 Info (untested bytes) | Still open, and stronger now that E-2 adds new syntax. P-AST recommended (VOTES). |
| R-05 build-script exposure | Unchanged (residual 5). |

## Not changed (preserved)

- The other six seeded files.
- The frozen request (`2d803e50…`), the amendment (`a7778b74…`) and the qualification (`ae03b92b…`).
- Both helpers (`466aeb8a…`, `b37bbc1d…`) and `OPTION-A-PLAN.json` (`881a7795…`).
- The compiler, candidate and harness.
- Every argv, budget, limit, control and retry policy (there is no retry).
- The FREEZE schema.

The prepared FREEZE `babba71b…` is stale for its `tools/run_r3.py` record. Root regenerates it.
