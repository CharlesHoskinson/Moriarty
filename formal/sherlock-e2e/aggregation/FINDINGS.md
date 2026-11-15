# Aggregation findings (Formal-Lean-B)

| ID | Kind | Evidence | Proposed verdict |
|---|---|---|---|
| F-AGG-1 (SHP-003 vs NAG-011) | contradiction between requirements | SHP-003 (modular-leaf): "MERGE-WORK SHALL take two enabled prior SEG proofs plus `k >= 1` real owner steps". NAG-011 (e2e): "a MERGE-WORK node has one linked child, the node AIRs SHALL accept the witness"; the built test `afk_f9_work_one_child` (openvm `t10q` 0082832 `crates/sherlock-leaf/tests/quint_regressions.rs:190`, manifest row `expect = accept`, Moriarty `tests/quint-regressions/manifest.json:235`) accepts one child. The oracle follows NAG-011 (`work_guard`, `pin_work`: one linked child accepted, no prior child refused). Spec text fix: SHP-003 "one or two enabled prior SEG proofs" (owner text of modular-leaf; send to the spec owner). | CONFIRMED (the built code accepts one child; the SHP-003 sentence is the stale side) |

Observations (not findings):

- MERGE-WORK width, three recorded values for different layouts: 5,128 MEASURED "as built" in T8 (openvm `t10mm` 5b7ada8 `frozen.rs:165`), 5,284 MEASURED in T8b and on the T10m keys (Moriarty `research/sherlock-t10m-20261114/RESULTS.md:51`, `:96`), 4,988 MEASURED for `Kind::Work` without the GP block after T10q (rekey-chips dcf1b4c `frozen.rs:167`). Every MEASURED value is at most 5,400. The 5,121 of coverage-B.md and NAG-005 is DERIVED (4,988 + 71 ESTIMATE + 62) and is labelled DERIVED in `lpc002_derived_full_width`. coverage-B.md cites `frozen.rs:166` for 4,988; on dcf1b4c the comment is at `:166` to `:168` and the constant at `:169` (on 5b7ada8 the constant is at `:166`). No conflict with the bound.
- NAG-005 cites "5,284 MEASURED in T8b (RESULTS.md:51)": confirmed at that line of `research/sherlock-t10m-20261114/RESULTS.md` (the T10m key table, which repeats the T8b width).
- FVR-004/FVR-006 allow 3,300 columns for FV units while NODE-007 bounds nodes at 3,000 with keygen refusal; FVR-006 is the specific owner exception (O1, O2), so this reads as consistent, but NODE-007's sentence names no exception.

## Resolution (2026-11-15)

- F-AGG-1: FIXED in the text. SHP-003 takes one or two enabled prior SEG proofs (LEAD PICK for T15). Odin replay on openvm `rekey-int` 3826e38: `afk_f9_work_one_child` accepted, `afk_f9_work_one_child_unlinked_refused` refused (`cx/fbagg1.log`). Lean `pin_work`.
- MERGE-WORK width: the texts give 4,988 MEASURED (current layout, `frozen.rs:166` to `:167`), 5,121 DERIVED (`:168`) and the bound at `:169` (rekey-int 3826e38). 5,128 (T8) and 5,284 (T8b, T10m keys) are earlier layouts.
