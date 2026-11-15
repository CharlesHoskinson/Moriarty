# Key Management: findings fragment (Formal-Lean-B)

| ID | Requirement | Kind | Evidence | Proposed verdict |
|---|---|---|---|---|
| F-B-KM-1 | UKS-010 | stale cited number (status scenario) | The status of UKS-010 reads "GAP: 6 rows missing". `~/.cache/rekey-t11/LIST.md` (sha256 639220981074832c..., mtime 2026-11-15 10:31) now holds the FP-3 lane table and the T12-SLOT15 row; range checks, R_SL statement bindings, R_SL key leaf index and era-check words are still missing (4 rows). Lean `KeyManagement.uks010_rows_missing`, `t25_refused_20261115` on the recorded list. The requirement sentence is unaffected; the refusal still fires. | CONFIRMED (status text only; fix: "4 rows missing" in the status scenario, owner Fable-B) |
