# Fresh non-author SOURCE completion audit — core continuations (successor)

Reviewer: Muse Wonder 1.0, high effort, fresh non-author **nonsecurity** auditor (not source author, finisher, or classifier). Candidate `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`, base `fe01bc8b7d889427192c304b0e75d0ec719f63db`, predecessor `e9a2b80b`. Worktree verified clean at candidate HEAD; only guarded `status --json` startup was run. No Cargo/build/test/fmt/Quint/SDK/model/degree/k/VM/transcript/keygen/prove/native-verify/deploy/ledger/network or private-material action was run or read. Authority 0; counters unchanged. Security judgments are Opus 5.5-owned and are not made here. This completes the exact unread SOURCE coverage of prior `MUSE-SUCCESSOR-REVIEW.md`; its ACTUAL-RESULTS/DOCS evidence accounting was not repeated except where source inspection contradicted it (none found).

## Dispositions (separate axes)

| Axis | Disposition |
|---|---|
| SOURCE | **APPROVE** (qualified, whole `/2` continuation child feature, local source-static scope only) |
| ACTUAL-RESULTS | **APPROVE (scoped, captured-local)** — retained from prior read-only candidate review; independently spot-consistent with source/test bodies read here; not a rerun |
| DOCS | **APPROVE (scoped)** — retained from prior review; no contradictory source fact found here |

This is not product V1 approval: all ten `/4` native/target gates remain `unperformed`, and U0–U7/M1–M17/MC/SP/G exits remain open. Labels are exactly `local-reference-stage`/`none`/`untracked` ([stage/mod.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/src/stage/mod.rs:34)); beta Source 6/Core 5 is separate and untouched.

## Coverage checklist (all read in full unless noted)

- [x] Normative precedence: `FROZEN-DESIGN.json` (A2/A1.1 > FINAL A0–A12 > CLARIFICATION > original > FINAL-NONSECURITY), `FINAL-NONSECURITY-DESIGN.md`, product contract permissionless boundary, spec UNI-006/007/008 (lines 59–90; `requirements.md` treated as index only). No redesign absent defect.
- [x] Core stage (8 modules, full): `mod` (load order profile→validate→plan→cost-invariants→digests, labels, zero-await `PendingFrame::idle`), `admit` (R0–R7 order, H25 emit-checked, H26 premise, false-guard unchanged continuation, rejection publishes nothing), `cost` (DAG-node counting, prefix `committed_before`, cap never raised by CLI, checked u128), `decode` (bounded read, R7 canonical boundary as-written, no re-sort), `validate` (legacy checks + Await bounds 16/16/256, prefix-monotone plan/cost invariants), `plan`, `profile` (`/1` vs `/2` isolation), `digest` (canonical SHA-256, exact lowercase match).
- [x] Core diffs + reached helpers: `runtime.rs` (`execute_statement` shared by legacy `simulate` and `run_segment`; exit-await charges entry 1, no guard/step; resume emits Await step at original ordinal with pre snapshot), `parser.rs` (`await … when` only under `/2`, `language()` routing), `ir.rs` (serialize-only shared `Statement::Await`; frozen `v1` wire with no Await, all decodes routed via `decode_via_v1`), `json.rs` (`parse_limited` changes only the byte ceiling), `logical_cost.rs` (adds profile `/2` constant only), `lib.rs`.
- [x] Backend stage (5 modules, full): `mod`, `lower` (signed Target refused at entry; `artifact.validate()` at [lower.rs:127](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-backend/src/stage/lower.rs:127) before sole successful return; shape-only, not provenance; `_out` unused, no backend filesystem effect), `artifact` (strict `/4` decoder: labels, ten gates, contiguous action runs, width-L capture prefixes, pending-position distinctness/disjointness/contiguity, fixed-point `artifact_bytes`, zero-await `pending_positions: None` per A1.1), `frame` (layouts re-derived from source awaits; only emitter of pending roles; guard is one `Assert`), `context` (29-element domain-separated preimage).
- [x] Backend helpers: `lower_action_segment` in `lib.rs` (`None` preserves `/3` layout exactly; inputs only at entry site 0; Await arm is typed `core1_refusal`), `signed_relation`/`proving` Await refusals and `require_core1` gates on `compile`/`compile_signed`/`compile_mode`.
- [x] CLI (full bodies/diffs): `check`, `compile` (staged branch publishes circuits then manifest last via `proof_io::publish_new`), `inspect` (source-only `/2`; `--target` on `/2` refuses with Core profile error before reading target), `simulate` (`--resume`/`--event` clap conflicts before any file read; labels on every staged object; rejected exit 1), `core_io`, `proof_io`, `cli.rs`.
- [x] Ledger diffs + closure test (full): `require_core1`/`core1_refusal` at every reached statement match and entry point; `source_closure` appends the 8 Core stage files after the frozen 61-member baseline; [source_closure_coverage.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-ledger/tests/source_closure_coverage.rs:1) walks Core/lexer/ledger sources, fails closed on unknown files, and permits only three reasoned `#[cfg(test)]` exclusions.
- [x] Successor delta (4 source/test files, complete): `lower.rs` validate-before-return, `stage_relation.rs` (+4 tests), `stage_hostile.rs` (appended H6/H7/H9–H12/H14/H24/F2, read in full), `source_closure_coverage.rs`; plus README/ROADMAP/guide doc fixes.
- [x] Normal test bodies (full): Core continuations 5, compat 4, static-cost 13, backend stages 5 (+1 ignored `/1` comparator), backend relation 9, frame 6 (in-module), CLI 7, ledger closure 6. Hostile 22 counted/spot-read as prior; security *judgments* on them remain Opus-owned — only source/API compatibility was inspected.
- [x] Oracle: frozen SHA-256 verified `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`. Exact exercised keys (`escrow`, `reserve`, `compatibility_v1`, labels) inspected; both trajectories recomputed independently (below).
- [x] Quint (full sources + ADAPTER/LIFECYCLE docs): prefix 2 tests/4 one-transition samples, lifecycle 9 tests/32 sampled traces, kept separate from the 78. Lifecycle first-test typecheck QNT404 retained; only 2 direct-constructor imports fixed, later typecheck + 9 tests pass, passed prefix not rerun. Model uses mathematical `int`, no UInt128/field correspondence, no Rust correspondence or native proof claimed — docs state this accurately.
- [x] Legacy baseline: leaf bytes hash-verified against `LEGACY-BYTE-FIXTURE-MANIFEST.json` (`000-deliver.zkir` 7,389 B `0afa370b…`, `artifact.json` `fc58d238…`, source/target match); `/1` comparator selected source unchanged; staged-only `lower.rs` repair is not called by it.
- [x] Hashes: all 84 current scope paths (56 files + 28 documentation/model inputs — not 84 entries in `files`) hash- and byte-match `SOURCE-SCOPE-CURRENT.json`. Capture-pin deltas (850 pins/20 stream hashes, 10 captures) are confined to test-only `stage_hostile.rs`, staged-only `lower.rs`/`stage_relation.rs`, and the examples README; none touches a carried-forward production call path.

## Independent oracle recomputation

Escrow (amount 10): alice 100→90, escrow 0→10, `release` 0→10; costs [5,6], total 11; false resume attempted 2/committed 5; true resume escrow→bob 10, discharge 10, residual [] — matches oracle and [continuations.rs:64](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/tests/continuations.rs:64).
Reserve (extra 2, basis 40): held = 42 captured before `basis`→35; `allocation` 5→47; quote 43 → allocated 42, reserve 100→58, target 0→42, discharge 42 → residual 5, basis 35+43=78; final fee 2 (reserve 58→56, treasury 0→2), emit 43; costs [11,15,10], total 36 — matches oracle and [continuations.rs:79](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/tests/continuations.rs:79). Refunds/fees do not erase the residual 5 duty, per UNI-006/008.

## Findings

No required (blocking) nonsecurity finding. Optional notes only:

1. Optional — [OBSERVATIONS-CURRENT.json](/home/charl/Moriarty/.worktrees/core-continuations-20261015/docs/audits/2026-10-15/core-continuations/OBSERVATIONS-CURRENT.json:4): `successor_commit` remains `PENDING_ROOT_FREEZE`. Predicate: a committed snapshot cannot contain its own hash; consumers must bind externally to `7d6c3965…`. Severity: optional. Trace: evidence-binding hygiene, FINAL-NONSECURITY scope note.
2. Optional — [parser.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/src/parser.rs:1) `language()`: an unrecognized profile error names only `moriarty/1` as expected, though `/2` is also accepted. Predicate: diagnostic text only; routing behavior is correct and tested. Severity: optional. Trace: CLI contract, FINAL-NONSECURITY CLI table.

## Unread / not performed (explicit)

Nothing in the required SOURCE coverage above is unread. Not performed by design and not approval blockers for this scoped source verdict: all ten `/4` native/target gates, any execution/rerun (78 = 49 fresh + 29 carried forward, not summed or rerun here), security interpretation of hostile/relation tests (Opus-owned), and the full historical review corpus beyond orientation from the prior e9a2 full-source approval and successor reports (used as directions, not evidence).

**SOURCE APPROVE** — qualified to the local `/2` continuation child feature in source-static scope, retaining all native/unperformed/full-parent limits above.
