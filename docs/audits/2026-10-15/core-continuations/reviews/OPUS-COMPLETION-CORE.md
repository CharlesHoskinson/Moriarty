# Opus 5.5 completion review: Core security subscope A

- Reviewer: Opus 5.5 (`claude-opus-5-5`), high effort. Fresh, independent, non-author security reviewer.
- Candidate: `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`. Base: `fe01bc8b7d889427192c304b0e75d0ec719f63db`.
- Worktree delta: only `examples/core/continuations/README.md` (see section 6).
- Scope: subscope A only. This review covers Core. It makes no claim about the backend, CLI, ledger, Quint or native security, and it does not approve the whole feature. The final integrated Opus review is still required after A/B/C.
- Actions taken: read-only source, git, gzip and sha256sum. No Cargo, build, test, fmt, Quint, SDK, native or network actions. Native authority is 0. No counters were reset. This file is the only write.

## Verdict: **NEEDS_CHANGES (test-only)**

I found no source defect in the Core security path. The verdict comes from required hostile controls in the frozen design that have no executed evidence: H6, H7, H9, H10, H11, H12, H14 and H24. The R7 canonical-boundary checks and the stage decoder limits are therefore implemented but never exercised by a hostile test. I cannot approve that coverage gap, even though my reading found the code correct.

## 1. Coverage

**Source, read in full at the candidate.** `sha256sum --check CANDIDATE-CODE-CHECKSUMS.txt` returned OK, and each file below matches the pin in every Core start.json.
- `stage/mod.rs`, `plan.rs`, `profile.rs`, `validate.rs`, `cost.rs` (including its inline unit tests), `admit.rs`, `decode.rs`, `digest.rs`: all eight modules.
- Full `git diff fe01bc8b..e9a2b80b` of `ir.rs`, `parser.rs`, `json.rs`, `lib.rs`, `logical_cost.rs` and `runtime.rs`.
- In `runtime.rs` I also read the current `evaluate`/`eval_cached`, `charge_logical`, `amount`, `liability_identities`, `scenario_snapshot`, `move_funds`, `execute_statement`, `run_segment` and the `simulate_with_meter` head.
- `intent::canonical`, the head only (its sort-keys JSON encoding).

**Tests, read in full:** `tests/continuations.rs`, `compat_v1.rs`, `stage_hostile.rs`, `stage_static_cost.rs`. I also grepped `crates/mori/tests/core_continuations.rs` for hostile controls, without making any CLI security claim.

**Results:** I decompressed stdout, stderr, result.json and start.json for `core-normal-01`, `core-compat-01`, `core-hostile-01` and `core-static-cost-01`.
- Pass counts: 5 + 4 + 10 + 13 = 32 passed, 0 failed, 0 ignored. All four exited 0 with `native_actions_requested: 0`.
- The `gzip -cd | sha256sum` hash of each stdout equals its `stdout_sha256`: `f7ffdb85…`, `af01aab1…`, `38cab21b…`, `42e03299…`.
- Every start.json pins the current Core sources, the four tests, the oracle fixture `76075b63…` and the examples. The README pin `ece59582…` equals `git show e9a2b80b:examples/core/continuations/README.md`.

**Oracle:**
- `D/oracle/ORACLE-CONTINUATIONS.json` = fixture = `examples/.../normal-oracle.json` = `76075b63…`.
- `oracle-escrow-v2.mori` = `escrow.mori`, and `oracle-reserve-v2.mori` = `reserve.mori`, byte for byte.
- I recounted the escrow plan independently: segment 0 is transfer 2 + liability 2 + await entry 1 = 5. Segment 1 is dispatch+guard 2 + transfer 2 + discharge 2 = 6. A false guard attempts 2. All three match the tests.
- I did not read the 4,488-line fixture line by line. I relied on the hash identity with the pre-implementation oracle.

**Not read:** nothing inside subscope A. Out of scope by instruction: the backend `stage/*`, CLI `commands/*`, ledger files and their tests.

## 2. Initial open items: resolved, no defect

**O1, wrong-typed start inputs.**
- `runtime.rs:572-585` (`scenario_snapshot`) requires exactly the selected action's input count and each declared type.
- `admit.rs:236` calls it before the cap preflight and evaluation. A refusal returns `input` with attempted 0 and committed 0.
- The later `admit.rs:240` lookup is therefore defensive and unreachable.

**O2, let/state shadowing before the captures overlay.** Name collisions are refused statically at every layer:
- `validate.rs:154` covers inputs, `:189` await binders and `:206` lets, each against an env that already holds every state and earlier name. Asset names are refused too.
- `validate.rs:397` (plan invariant) refuses any capture slot that names a state or repeats.
- R4 (`admit.rs:329`) binds capture names and types exactly to those slots, and R5 does the same for event binders.
- So the `run_segment` overlay (`runtime.rs:944-952`: state map, then captures and event) can never shadow a state, and binders never collide with captures.
- `Next` updates both `current` and `env` exactly as in `/1`.

## 3. Security checks performed (all hold in source)

- **One evaluator, no second semantics.** `execute_statement` (`runtime.rs:702`) is a verbatim extraction of the old `simulate_with_meter` closure, and legacy simulation and `run_segment` both call it. `run_segment` adds only the await entry charge and capture extraction. Legacy `validate` refuses `Await` (`runtime.rs:235`), and `require_core1`/`require_core2` isolate the profiles. `/2` parse skips legacy validate, and only `stage::load` admits it.
- **Frozen `/1` serde.** The shared types are Serialize-only, and `decode_via_v1!` routes every Deserialize, including nested Step and Execution, through `ir::v1`. Its variant list and expected text are unchanged, and compat_v1 checks the error strings byte for byte (4/4 pass). `intent::canonical` is sort-keys JSON, so adding the variant leaves `/1` hashes unchanged.
- **Strict JSON.**
  - `parse_limited` shares `parse_inner`, so the duplicate-key, depth-32 and node-65,536 limits are unchanged and only the byte ceiling differs.
  - `read_bounded` reads at most max+1 bytes.
  - All stage DTOs use `deny_unknown_fields`, and `cap` is a mandatory nullable key.
  - Digest comparison accepts only exact lowercase hex.
- **Admission order R0→R7** matches DESIGN-OPUS-FINAL:115. Every pre-evaluation refusal returns attempted 0 with the input's committed value. The evaluator only sees a clone (C10).
- **Work.**
  - The meter limit is exactly `segment_cost`. `require_static` demands equality, and a meter overrun maps to the premise error.
  - Without short-circuiting, and with a per-statement cache, cost is exactly 1 + reachable nodes, so the plan is value-independent.
  - `resume_preflight` requires committed = `committed_before[site]`, uses checked addition, rejects a CLI cap above the stored cap, and refuses any site that is 0 or out of range.
  - A completed action must reach `plan_total`, and an exit site must equal entry+1.
- **Boundary (R7).**
  - States must be exact, with declared types.
  - Balances must use declared assets and be strictly ascending, which also rejects duplicates. Every account the action transfers from or to must be present.
  - Liabilities must be strictly ascending by name, each with its exact registry identity.
  - The trace bound is enforced.
  - Strict liability order is load-bearing: `eval_cached` uses `binary_search`, so this check is what makes liability reads correct on a fabricated boundary.
- **Emission.**
  - `emit_checked` re-encodes each continuation, applies the size cap, decodes it, checks equality and reruns R7.
  - Start with sites runs R7 up front, so a legacy external liability cannot strand a continuation.
  - `move_funds` creates no accounts, and new liabilities are re-sorted.
- **Labels.** `STAGE_LABELS` carry qualification `local-reference-stage`, authentication `none` and consumption `untracked`. Captures, the boundary and the stored cap are unauthenticated and not consumed (H8 tail, H13). This is by design, and I did not reopen it.

## 4. Findings

| # | File:line | Severity | Kind | Required? |
|---|---|---|---|---|
| F1 | `crates/mori-core/tests/stage_hostile.rs` (absent cases); design `DESIGN-OPUS.md:283-291`, `DESIGN-OPUS-FINAL.md:333` | Medium | Requirement/test gap, not a bug | **Required** |
| F2 | `crates/mori-core/tests/*` (absent) | Low | Test gap | Optional |
| F3 | `crates/mori-core/src/stage/cost.rs:245-372` | Low | Evidence gap | Optional |
| F4 | `crates/mori-core/src/stage/decode.rs:94-226` vs `runtime.rs:528` | Info | Error-order divergence | Optional |

**F1.** None of these frozen hostile controls has a test in any captured Core target. My grep found none in the CLI test either.
- **H6:** duplicate key in an event (`ready` true then false) and in a continuation.
- **H7:** non-canonical UInt `"010"` in a continuation or event.
- **H9:** unsorted or duplicated boundary balances, expecting `balance`.
- **H10:** `release` debtor changed, expecting `liability` with "Existing liability identity differs".
- **H11:** escrow 0, then a true resume, expecting `balance`, step 3, attempted 4, then a successful retry.
- **H12:** `release` removed, expecting `liability`, step 4, attempted 6, with no Transfer effect.
- **H14:** capture `y` tampered, accepted (characterization).
- **H24:** 4 MiB+1 and 65,537 bytes rejected before parse, and `/1` still reports "JSON exceeds 1 MiB".

My reading predicts each would pass: decode.rs:152/202/206, runtime.rs:685-689, admit.rs:329 and json.rs:97. R7 still carries the liability binary-search correctness, so add these as Opus-authored tests and capture them through a fresh classified run. No source change is needed.

**F2.** Not covered: a `stage::start` with a wrong-typed, missing or extra input (O1 holds in source); an R7 check for a missing or extra state, or a state with the wrong type; and a liability-order inversion.

**F3.** `cost.rs` inline unit tests (`resume_preflight` overflow, the H26 meter ceiling) are not in the four captured `--test` targets. No `--lib` Core capture was in my scope.

**F4.** R7 checks bounds, then states, balances and liabilities, then trace cells. `scenario_snapshot` checks trace cells first, so a boundary with several defects can report a different first code. This has no security impact.

## 5. Severity summary

There are no critical, high or source findings. F1 (Medium) is required before this subscope can be approved.

## 6. Documentation delta

The README now states that the examples ran within the 58 passing focused checks, says full security review remains pending, and keeps U0–U7/M/MC/SP/G open with no native or ledger claim. The Core part of that statement agrees with the 32 Core passes I verified. This review does not verify the "58" total or the CLI runs. The text is acceptable, but the claim that the frozen assertions were "exercised" does not cover F1's hostile controls.

## 7. Bounded conclusion

**NEEDS_CHANGES** for subscope A, on test coverage only. The Core source is clean under my reading. Approval of this subscope needs the F1 tests plus their executed, classified captures, followed by a fresh Opus pass over that delta. This is not a whole-feature approval.
