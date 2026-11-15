# Key Management: findings fragment (Formal-Lean-B)

| ID | Requirement | Kind | Evidence | Proposed verdict |
|---|---|---|---|---|
| F-B-KM-1 | UKS-010 | stale cited number (status scenario) | The status of UKS-010 reads "GAP: 6 rows missing". `~/.cache/rekey-t11/LIST.md` (sha256 639220981074832c..., mtime 2026-11-15 10:31) had FP-3 and T12-SLOT15 rows at 10:31 (4 missing) and all six rows at 11:22 (sha256 8a315c07..., RSL-BIND, T22 era check with the ELD-007 fallback, prekey-fixes range checks and R_SL key leaf index). Lean `KeyManagement.uks010_rows_missing`, `t25_refused_rekey_int`. The requirement sentence is unaffected. | CONFIRMED (status text only; fix: "rows present at 11:22", owner Fable-B) |
| F-B-KM-2 | RKY-002, UKS-003, UKS-009 | state of the T25 start on recorded values (not a spec defect) | On rekey-int ca65f27 with LIST 11:22 the oracle refuses the keygen only with `row-uncommitted`: the wrap-levers C3.1 row is "staged, NOT committed" and the T11 blinders row has a Moriarty "commit pending". UKS-003 (RSL_KEY_IDX0 = 4, stmt.rs:275) and UKS-009 (every listed openvm commit is an ancestor of ca65f27) hold there. Lean `t25_refused_rekey_int`. | GUARD HOLDS (the T25 runner must refuse until the two commits land) |

## Resolution (2026-11-15)

- F-KM-1: FIXED in the text. UKS-010 status: all six rows in LIST.md of 11:38 (`:37`, `:44` to `:46`, `:51`, `:57`, `:63`, `:64`). F-KM-2: GUARD HOLDS.
