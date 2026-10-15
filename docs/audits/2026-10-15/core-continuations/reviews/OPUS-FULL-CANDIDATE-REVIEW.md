# Opus 5.5 full-candidate security review: core continuations

- Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort, non-author, owns all security work for this scope.
- Candidate: `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`; base `fe01bc8b7d889427192c304b0e75d0ec719f63db`; worktree clean at start (`git status --short` empty, HEAD verified).
- Startup: read `AGENTS.md`, `plugins/moriarty-dev/skills/develop/SKILL.md` and `references/execution-focus.md`. Ran only `cli.py --repo . status --json`. It reported historical SP01 operational state (`blockedAction: dependent dispatch for loan-swap-subset`). Nothing was dispatched.
- Actions: git, source and summary reads only. No Cargo, build, test, fmt, Quint, SDK, native, network or key actions. No source, docs, example or test mutations. This file is the only write.

## Verdict (bounded)

**Source security: NOT APPROVED. Review incomplete. No defect found in the files read.**
**Actual results: NOT APPROVED. Not verified.**
**Full candidate: no APPROVE issued.** The review ran out of time and some tool calls were refused before coverage was complete. I am not issuing NEEDS_CHANGES, because I found no concrete source defect. I am not issuing APPROVE, because approval must not rest on a lack of evidence. Treat this seat as **not approving** until a completed Opus pass covers the remaining files listed below.

Native authority stays at 0. The scope stays local, unsigned and static: labels `qualification=local-reference-stage`, `authentication=none`, `consumption=untracked` (`crates/mori-core/src/stage/mod.rs:37-41`). This review does not accept V1 or any U/M/MC/SP/G item. Full native `mori-ledger` remains unsupported, and all ten `/4` native and target gates remain unperformed.

## Read coverage

Read completely:
- `crates/mori-core/src/stage/decode.rs`, `digest.rs`, `mod.rs`, `admit.rs`
- `crates/mori-core/src/stage/cost.rs`: `start_preflight`, `resume_preflight`, `require_static`, and the unit tests at lines 302-356
- `crates/mori-core/src/runtime.rs:934-1021` (`run_segment`)
- `crates/mori-core/src/stage/validate.rs`: binder, label and shadowing rules (grep at lines 5-194)
- `crates/mori-backend/src/stage/context.rs`: refusal lines only (grep)
- Runtime `CORE-ACTUAL-CHECK-SUMMARY.json`: first segment only

**Not read (remaining limits):**
- Backend: `stage/frame.rs` (632 lines), `stage/artifact.rs` (502), `stage/lower.rs`, `stage/mod.rs`, the full `context.rs`, `lib.rs`, `signed_relation.rs`, `proving.rs`
- Core: `ir.rs`, `parser.rs`, `json.rs`, `stage/plan.rs`, `stage/profile.rs`, the rest of `validate.rs` and `cost.rs`, and the legacy `runtime.rs` diff (≈443 lines, including the `scenario_snapshot` and `execute_statement` changes)
- Ledger: `layout.rs`, `ops.rs`, `proving.rs`, `relation/*`, `signed/mod.rs`, `static_compile.rs`
- CLI: `check.rs`, `compile.rs`, `inspect.rs`, `simulate.rs`
- All test sources: `continuations.rs`, `compat_v1.rs`, `stage_hostile.rs`, `stage_static_cost.rs`, `stages.rs`, `stage_relation.rs`, `source_closure_coverage.rs`, `mori/tests/core_continuations.rs`
- Docs and design: `docs/CORE-CONTINUATIONS.md`, ROADMAP delta, D `README`/`SCOPE`/`OBSERVATIONS`, `FROZEN-DESIGN.json` and the precedence documents, security addenda, product contract, U3 EARS
- Results and oracles: all original stdout/stderr captures, frozen oracle `76075b63…9640`, `normal-oracle.json`, the legacy byte manifest and postcheck, `FRAME-POSTCHECK`, the CLI, backend, relation, ledger-closure and frame summaries
- Pin binding: comparing the runtime `*SOURCE-PINS.json` against the candidate blobs could not be done because the tool refused the inline hash comparison. So I have **not** verified that any of the 58 captured tests ran against candidate bytes or against the current 56 product hashes. The Core summary itself says `"scope": "four exact approved Core targets only; candidate and actual result audits remain owed"`, with baseline `fe01bc8b`. Its link to `e9a2b80b` is unverified.

## Security observations on the code read

None of these is a blocking defect. They are confirmations and open items.

1. **Strict, bounded decoding.** Confirmed in source; tests not inspected.
   - `decode.rs:19-33` reads at most `max+1` bytes and refuses anything over 4 MiB (continuation) or 64 KiB (event) before parsing. Non-UTF-8 input is refused.
   - `strict` goes through `json::parse_limited` and then `serde_json::from_value`. All wire structs use `deny_unknown_fields` (`mod.rs`).
   - `StageLogical.cap` is a mandatory key that may be `null` (`mod.rs`, `deserialize_with = Option::deserialize`).
   - Not verified: that `parse_limited` keeps the duplicate-key, depth and node limits as the comment claims (`json.rs` diff not read).
2. **Admission order R0–R7.** Matches the design in source (`admit.rs:279-373`).
   - R0: CLI action must equal the continuation action.
   - R1: format and profile.
   - R2: `/2` profile plus exact lowercase program and plan digests.
   - R3: `site ∈ 1..=sites.len()`.
   - R4: capture layout must match exactly in length, order, name and type.
   - R5: event format, action, label and binder layout.
   - R6: committed work must equal `plan.committed_before[site]`. A CLI cap can never exceed the stored cap. Overflow is checked.
   - R7: canonical boundary.
   - Every refusal reports attempted 0 and echoes the input's committed value.
3. **Capture values are untrusted.** This is by design, not a defect.
   - `canonical_boundary` discards `captures` (`decode.rs:100`, `let _ = captures`). R4 checks only shape.
   - The stored `cap` and `committed` are caller-editable. `committed` is pinned to the plan, but `cap` can be removed or raised by editing the JSON.
   - There is no replay or consumption protection (`admit.rs:3-7`).
   - This matches "cap untrusted locals" and P3. It must stay labelled and must never be reported as authorization.
4. **False guard.** In `admit.rs:103-121`, `NotAdvanced` returns the input continuation unchanged with `segment: None`. Committed work is unchanged. Only the metered dispatch is reported as `attempted`. `emit_checked` re-validates. A false guard is refused on start (`admit.rs:105-110`).
5. **True-entry `AwaitStep`.** In `runtime.rs:955-975`, resume records the await statement's own `Step` at its original ordinal, with the pre-boundary snapshot. Execution continues at `ordinal+1`. At the next `Await`, only the entry is charged and the captures are taken from `env` by slot name (`runtime.rs:980-994`).
6. **Static-cost premise (H26).** `require_static` demands that the meter equals exactly the planned cost (`cost.rs:227`). `premise()` maps a meter overflow under the planned limit to `stage_logical` (`admit.rs:39-47`). The exit site and committed values are re-checked against the plan (`admit.rs:133-182`).
7. **Open item: start input typing.** `start` checks only that each input is present (`admit.rs:239-249`), although its message says "Missing or wrong type". Type rejection relies on `runtime::scenario_snapshot`, which I did not read. A hostile test with a wrong-typed start input is needed to confirm the refusal. **Required to verify; this is not a finding yet.**
8. **Open item: name collisions in the resume environment.** `env` is built from state, then captures, then event binders (`runtime.rs:956`). `validate.rs:157,187-194` refuses inputs that shadow declarations and binders that shadow any earlier name. I did not confirm that `let` names, and therefore capture names, cannot shadow state names. If they could, a capture could overwrite a state value in `env` while the boundary state stays unchanged. **Required to verify** from the `validate.rs` let rule and `stage_hostile.rs`.
9. **Boundary R7 checks.** `decode.rs:91-213` enforces:
   - an exact state set with correct types
   - balances strictly ascending by `(asset, account)`, with no re-sorting
   - every transfer and fee account of the action present
   - every liability matching its source registry identity `(asset, debtor, creditor)`
   - trace-cell bounds

   The registry refuses conflicting identities for the same liability across actions (`decode.rs:65-71`).
10. **Backend signed refusal.** `context.rs:35` refuses a signed Target for staged `/2` relations. I did not inspect the rest: the context-29 layout, the pending null/zero positions, the frame wire and the other target refusals.

## Requirement trace (inspected subset only)

| Requirement | Status |
|---|---|
| Typed explicit await, N+1 circuits, no false-wait circuit | Core plan and runtime are consistent; backend not read |
| Original statement ordinals | Confirmed (`runtime.rs:957,979`) |
| Captures = inputs, evaluated lets and earlier events | Slot-driven capture confirmed; plan construction (`plan.rs`) not read |
| Current boundary state | Confirmed (`admit.rs:195`) |
| False guard: unchanged continuation, segment None, no committed cost | Confirmed |
| Rejected suffix publishes no new segment | Confirmed (all `fail` paths return `Err`) |
| Positive residual duties stated explicitly | Confirmed (`mod.rs` `residual`) |
| `/1` serde wire, errors, no language unification, beta unchanged | Not verified |
| `/3` byte-exact | Not verified |
| `/4` context-29 and pending roles | Not verified |
| Logical schedule `2` is not DUST or circuit cost | Labelled in source; docs not read |
| Ledger closure: eight files appended to the old 61 | Not verified |

## Required before any approval from this seat

1. Finish reading the files listed under "Not read", especially backend `frame.rs`, `artifact.rs` and `context.rs`, the CLI publication paths, and all four hostile and static test files.
2. Compare every runtime `*SOURCE-PINS.json` and `*ADDITIONAL-INPUT-PINS.json` entry with `git show e9a2b80b:<path>`, and record any stale captures.
3. Inspect the original stdout/stderr for all 58 tests against the frozen oracle `76075b63…9640`. That includes the escrow and contrasting two-await reserve pre/post/effects/steps/captures/work. It also includes the preserved earlier failures: CLI E0004 with 0 tests executed, and backend 4 pass / 1 fail / 1 ignored, without double-counting prefix passes.
4. Resolve open items 7 and 8 by checking the source or tests.

## Limitations

- Nothing was executed except the startup status call.
- The Quint prefix, its witness traces and the R lifecycle draft were not reviewed. They are abstract QA, not Rust correspondence.
- Muse owns the independent nonsecurity verdict.
- No provider timeout, identity, classification or receipt ingestion counts as approval.
