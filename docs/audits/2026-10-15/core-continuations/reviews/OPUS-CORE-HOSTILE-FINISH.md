# Opus Core hostile-test finish and classification

Role: author-finisher and classifier, `claude-opus-5-5` (Opus 5.5). This is not an independent review or an approval of the whole candidate. Nothing about the timed-out author (exit 124, no identity or report) was inferred. Any generic review tag from the receipt adapter does not count as independence.

## Source pins
- Base `e9a2b80b:crates/mori-core/tests/stage_hostile.rs`: 323 lines, sha256 `56f04e4f880931246517df74a99caf96b0542d949472f6e8a76150ba7fa003da`.
- Written payload (pre): 874 lines, sha256 `b52300688fd14654c2665942d3611b5eb1bcbb7a3c910a06219250b72191203d`, which is +551 lines and 0 lines removed.
- Adopted (post): 883 lines, sha256 `e8d3af1257d447aad9f14b566b6eb9b3005a929f26887cbe3863a546f93054a7`, which is +560 lines and 0 lines removed against e9a2b80b.
- Core and lexer production, the Core Cargo.toml, the workspace Cargo.toml and Cargo.lock, the toolchain file and `.cargo` are all unchanged from e9a2b80b (`git diff --quiet`). The only Core path that differs is this test file. The concurrent backend and ledger edits are outside the selected Core target.

- The independent oracle `examples/core/continuations/normal-oracle.json` still has sha256 `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`, so its bytes are unchanged. `escrow.mori` is unchanged from e9a2b80b.

## Minimal diff (only edit)
`TempInput::new` used to do `std::fs::write(path)`. It now uses `OpenOptions::new().write(true).create_new(true)` (O_CREAT|O_EXCL), and `Self` is built before `write_all`. Why: the path in `/tmp` is predictable (static label plus pid). The old code would follow a planted symlink or overwrite an existing file, and `Drop` could then delete a file the test did not create. With the change, a file or symlink already at the path makes the test panic before it holds the path, so nothing gets written through or removed. A partial write is still cleaned up on unwind. No other change.

## Test inventory (from source)
Original prefix, unchanged byte for byte: 9 `#[test]` plus 1 proptest = 10 functions. These are `r0_r1_selected_action_format_and_profile_are_exact`, `h1_h2_...`, `h3_...`, `h4_...`, `h5_...`, `h8_...`, `c10_...`, `h13_p3_...` and `h25_...`, plus the proptest at line 278, which uses `ProptestConfig::with_cases(32)`. Nothing was weakened.

New: 12 `#[test]` functions:
1. `h6_duplicate_json_keys_are_refused_by_the_strict_stage_decoders`
2. `h7_non_canonical_uint_strings_are_refused_on_decode`
3. `h9_boundary_balances_must_be_strictly_ascending_and_unique`
4. `h10_boundary_liability_identity_must_match_the_source_registry`
5. `h11_drained_escrow_fails_the_true_resume_at_step_3_then_a_corrected_retry_completes`
6. `h12_removed_release_fails_at_step_4_and_publishes_no_transfer_or_segment`
7. `h14_tampered_capture_is_accepted_as_a_characterized_unauthenticated_limitation`
8. `h24_stage_size_caps_apply_before_parse_and_the_legacy_ceiling_is_unchanged`
9. `h24_read_bounded_refuses_oversized_stage_files_before_decoding`
10. `f2_start_requires_exactly_the_typed_action_inputs`
11. `f2_r7_requires_exactly_the_declared_typed_states`
12. `f2_r7_requires_strictly_ascending_unique_liabilities`

Total: 22 test functions, with 32 proptest cases. All required items are covered: H6, H7, H9, H10, H11, H12, H14 and H24, plus F2 for input, state and liability ordering. H14 asserts that a tampered capture is **ACCEPTED** (`Complete`, level 100 to 106). That is a characterized limitation (`authentication:none`, `consumption:untracked`), not a protection.

## Symbol visibility (static read, no compile)
Every symbol the test uses already exists as `pub`, so no production visibility edits were made:
- `stage::{decode, STAGE_EVENT_FORMAT, MAX_CONTINUATION_BYTES, MAX_EVENT_BYTES, STAGE_LABELS, StageSegment, StageRejection{error,attempted,committed}}`
- `StagedProgram::plan`, `StagePlan.actions[].segment_cost`
- `stage::decode::{continuation, event, read_bounded, CONTINUATION_TOO_LARGE, EVENT_TOO_LARGE}`
- `mori_core::{json::parse, Effect, Error{step}, Balance}` (via `pub use ir::*`)

The test adds no SDK imports, no new dependencies and no Cargo changes. It uses `serde_json` and `proptest`, which the prefix already uses.

The expected messages match production strings in `stage/decode.rs`, `runtime.rs`, `json.rs` and `ir.rs`: duplicate decoded key, canonical unsigned decimal, "Balances are not canonical", "Liabilities are not canonical", identity differs, Missing/Wrong type state, every declared state, exactly the selected action's inputs, Missing or wrong type for input, Insufficient balance, Unknown residual liability, the 4 MiB, 64 KiB and 1 MiB caps, and "Stage input is not UTF-8".

Cosmetic issues, not fixed because they don't change any outcome:
- In H6, a comment says "JSON escape", but that case actually duplicates `site` with a different value.
- In H6, the `flag` event mutation appears twice in the list.

## Unverified (needs execution)
I did not run any compiler or test, so none of these is confirmed:
- that the file compiles, including possible unused-import warnings, borrow and closure types, and whether the `PAIR` source parses;
- step ordinals 3 and 4, and the attempted/committed meters (4/5, 6/5, 2, 11/6, 7, 4, 15/8);
- the E2 `segment_cost` `[7,8]`;
- the exact error-code order, i.e. which R-stage fires first, for the F2 state and liability cases and for the H10 `ghost` case;
- the H24 padding behavior at exactly the caps;
- the R5 `stage_event` code in H7.

Expected values come from the source read and the design notes, not from evaluator output. They are not acceptance evidence.

## Execution classification
Command: `CORE-PROPOSED-CHECKS.json` index 3, `--test stage_hostile`, with the same offline Rust 1.99, `-j2`, serial, 300 s, under the same flock target. I did not open or execute that file in this task. The premises from the prior Core classification are kept as they are: environment cleanup, Cargo config, toolchain, public target cache, no proptest seed files.

New effect class: **filesystem**. This was not in the prior Core four-class classification. `h24_read_bounded...` creates and removes 4 files under `std::env::temp_dir()`:
- `mori-core-stage-hostile-<pid>-event-cap` (65,536 B)
- `mori-core-stage-hostile-<pid>-event-over` (65,537 B)
- `mori-core-stage-hostile-<pid>-continuation-over` (4 MiB + 1 B)
- `mori-core-stage-hostile-<pid>-not-utf8` (1 B)

Only static labels and the pid are used. No private, key, SRS, cache or network paths are touched, and no reads happen outside the test's own fixtures. Peak temp usage is about 4.2 MiB. Creation is exclusive, so the test cannot overwrite a file or follow a symlink. Removal is limited to files the test created.

Additional check pins required before running:
1. Pin `TMPDIR` (unset, which means `/tmp`, or an explicit public scratch directory) and record it.
2. Pre-run: no `mori-core-stage-hostile-*` entries in that directory.
3. Post-run: still no leftover entries. A leftover can only come from a hard abort or a timeout kill, and should be recorded rather than treated as a failure of the code.
4. Pin the adopted sha256 `e8d3af12...` as the source.

Everything else in memory is pure: decode, start and resume against inline sources and `examples/core/continuations/escrow.mori` (via `include_str!`).

## Verdicts
- Source adoption, adopted as Opus-authored security test source at `e8d3af12...`: **APPROVE_ORDINARY**, with correctness still subject to the unverified outcomes above.
- Execution of index 3 with the FS class and the pins above: **APPROVE_ORDINARY**, provided the TMPDIR and leftover pins are added. If they are not, this becomes NEEDS_SOURCE for those pins.
- The final independent whole-candidate reviews (Opus and Muse) are still owed. This report is neither of them.
