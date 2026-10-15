# OPUS-CORE-FINISH — SOURCE_WRITTEN

Author: Opus 5.5 (`claude-opus-5-5`), sole security author for this bounded case.
Date: 2026-10-14. Worktree `.worktrees/core-continuations-20261015`, branch
`feat/core-continuations-20261015`.

**Status: SOURCE_WRITTEN.** Implementation only. No evidence, no acceptance.

## Native actions

All zero. No Cargo, build, fmt, test, model, evaluator, compiler, SDK, VM,
transcript, keygen, prove, verify, deploy, ledger, key, SRS or proving-cache
action ran. No commit or push. Reads were source and read-only git only.

## Files

| Path | Change | sha256 |
|---|---|---|
| `crates/mori-core/tests/stage_hostile.rs` | new, 10 tests (9 unit, 1 proptest) | `56f04e4f880931246517df74a99caf96b0542d949472f6e8a76150ba7fa003da` |
| `crates/mori-core/tests/stage_static_cost.rs` | unchanged | `de1d385fd497718f649e026d6327e6d88c8197ca0498b6711d8cfa8a478fd344` |
| `R/OPUS-CORE-FINISH.patch` | `git diff --no-index /dev/null` of the new file | — |

No production, shared or module source was edited. No API correction hunk was
needed: the tests use only public items (`stage::{load,start,resume}`, the
`Continuation`/`StageEvent`/`StageOutcome`/`NamedValue` DTOs,
`stage::decode::continuation`, and the `STAGE_*`/`CONTINUATION_FORMAT`/
`LOGICAL_COST_PROFILE_2`/`MAX_CONTINUATION_BYTES` constants). The
`cost::{fill,statement_cost,dispatch_cost,start_preflight,resume_preflight,
segment_meter,require_static,committed_after}` helpers are `pub(crate)`, so an
integration test can't call them. They are covered indirectly through the
public path and directly by the existing in-module `cost::tests`.

Compile compatibility comes only from reading the source. Nothing was compiled.

## Tests written (expectations frozen from source plus DESIGN-OPUS-A2; no oracle or evaluator blessing)

Each hostile case changes one field of a genuine escrow Waiting continuation
(site 1, captures `[amount=10]`, committed 5, plan `[5,6]`). Every rejection
is checked for `attempted 0` and for `committed` equal to the input's value.

1. `r0_r1_selected_action_format_and_profile_are_exact`: a mismatched CLI
   action gives `stage_site`. A wrong continuation format or logical profile
   gives `stage_format`.
2. `h1_h2_program_and_plan_digests_bind_the_continuation`: one flipped hex
   character in `program_hash` or in `plan_hash` gives `stage_program`. An
   uppercase encoding is refused when it differs. Continuations from an
   edited program (agreement renamed) are refused in both directions.
3. `h3_only_a_planned_pending_site_is_admitted`: sites 0, 2 and `u16::MAX`,
   and an unknown action, give `stage_site`.
4. `h4_captures_must_match_the_site_layout_exactly`: a renamed, retyped,
   dropped or extra capture gives `stage_capture`.
5. `h5_event_must_match_the_pending_await_exactly`: a wrong format, action,
   label, binder name or binder type, a missing binder, or a duplicated binder
   gives `stage_event`.
6. `h8_committed_work_and_cap_cannot_be_forged`: committed values
   {0,4,6,11,MAX} give `stage_logical`. A stored cap lowered to 10 gives
   `stage_logical`, and so does a CLI cap of 12 over a stored cap of 11.
   **Characterized limitation:** removing or raising the stored `cap` is
   accepted. `cap` is outside both digests and nothing is authenticated.
7. `c10_rejection_leaves_the_input_valid_for_a_corrected_retry`: after a
   rejection the input is unchanged, and the corrected retry completes with
   committed 11 and attempted 6.
8. `h13_p3_duplicate_resume_is_an_untracked_characterized_limitation`: the
   same continuation resumes twice with identical outcomes, and starting
   twice gives equal continuations. This documents P3 "consumption: untracked".
   It is not a replay-protection claim.
9. `h25_emitted_continuations_round_trip_through_the_resume_decoder`: for
   cap ∈ {None, 11, MAX}, the serialized form is within 4 MiB, decodes equal
   and resumes. A missing `cap` key or an unknown field gives `json`.
10. `h17_h26_static_cost_is_literal_independent` (proptest, 32 cases,
    literal `n` and input `k` each in `0..1000`): the plan is `[9,6]` with
    committed `[0,9]` and total 15 for every literal. The value was counted
    by hand: `let m = k*N` = 1+3, `next n = n+m` = 1+3, await exit = 1;
    then resume dispatch+guard = 1+1, `next n = n+N` = 1+3. Start attempts 9,
    a false guard attempts 2 and leaves the continuation unchanged, and a true
    guard attempts 6 and commits 15 with `n = k*n + n`. Literals `n` and `n+1`
    give different program and plan hashes (H1).

The existing independent oracle JSON and the oracle counts in
`stage_static_cost.rs` were not changed.

## Source clarification (not a ledger claim)

`decode::canonical_boundary` (R7) keeps the legacy baseline balance predicate.
Every transfer/fee account of the action must be present. Other accounts of a
declared asset, such as the extra `observer` 42 balance in the escrow and
reserve scenarios, are kept and not refused. That is what the source does. It
says nothing about any ledger binding or authorization of those balances.

## Still owed (not closed here)

- Execution of these tests and of `stage_static_cost.rs`. Expected values
  above are unverified until run. Any failure needs Opus classification, not
  edits to the expectations.
- The rest of the H1-H28 security families: R7 boundary hostility (state,
  balance order, liability identity, trace bound), H6/H7, H9-H12, H14-H16,
  H18-H24 beyond the existing H22 core, and H27/H28. Also oversized
  continuation and event byte limits, and the CLI/backend stage surfaces.
- Full U/M/MC/SP/G gates stay open. No approval is implied by this report.
  The three earlier timed-out Opus source jobs keep their original timeout
  status.
