# EnvGet hit/exit counterpart reference

`egTwoEventWindow(EGSnapshot)` consumes only a genuine actual BEFORE and returns
`EGWindow(before, after_cell, after_exit, first_event, second_event)`. The model's
declaration fixes the argument order; the observer exposes native JSON with exact
root keys `before`, `after_cell`, `after_exit`, `events` (ordered list of two).
Each snapshot retains admission, run_id and complete owned state. That state holds
the full 17-argument root, DATA tables and helper/result/task tables. Every event
has all 28 declared fields. Maps, arrays, raw slices, Some(0), None, frame order
and source payloads are retained, not reduced to selected projections.

`egNoBoundary` denotes the unit enum `BoundaryKind::None`; native JSON is the
string `"None"`. Actual option-none constructors remain JSON null. Treating both as
null was the archived port defect. The corrected six-regression suite explicitly
distinguishes them and preserves nested Some(0).

The archive preserves full source and typed trees, raw KAST/Kore/print, actual
commands, F03 records and freezes. See [the source and evidence index](../../results/sherlock-k-envget-cell-hit-exit-20261020/README.md)
and [current local disposition](../../results/sherlock-k-envget-cell-hit-exit-20261020/corrected-tail/whole-window-comparison/ROOT-DISPOSITION.json).
The independent whole Rust prefix is still MISMATCH. This API is a conditional
normal hit/Exit counterpart, not complete interpreter/refinement semantics.
