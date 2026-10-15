# Fresh independent non-author successor review — core continuations

Reviewer: Muse Wonder 1.0, high effort, nonsecurity auditor (this session; not source author, finisher, or classifier of any reviewed artifact). Candidate `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`, base `fe01bc8b7d889427192c304b0e75d0ec719f63db`, predecessor `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`. Worktree verified clean/frozen at candidate HEAD; only guarded `status --json` startup run. No Cargo/build/test/fmt/Quint/SDK/model/native/prove/deploy/network action was run by this reviewer; all result evidence is retained public captures inspected by decompression. Authority 0; counters unchanged. Security interpretation is reserved to Opus 5.5 — this report makes no security judgment and treats labels/gates only as nonsecurity source predicates.

## Dispositions (separate axes)

| Axis | Disposition |
|---|---|
| SOURCE | **INCOMPLETE** — no blocking nonsecurity defect found in inspected paths, but full line-by-line coverage listed below was not completed in this pass, so approval is not given by lack of evidence |
| ACTUAL-RESULTS | **APPROVE (scoped, captured-local)** — retained captures support exactly 78 distinct selected Rust functions (49 fresh + 29 carried forward), failed captures retained and uncounted, Quint kept separate; this is not a rerun and not native/target acceptance |
| DOCS | **APPROVE (scoped)** — prior Muse DOCS F1 (stale examples README) is fixed; guide/ROADMAP/dossier counts and limits are accurate as inspected |

## Verified evidence (independent inspection)

- Manifest: `sha256sum -c SHA256SUMS` in `docs/audits/2026-10-15/core-continuations/` passes all 417 entries, exit 0. (The task brief's "84 paths" figure does not match any manifest I could identify — `SOURCE-SCOPE-CURRENT.json` lists 56 files, all 56 current bytes hash-match their recorded SHA-256; I do not rely on the 84 figure.)
- Frozen oracle: `docs/audits/2026-10-15/core-continuations/oracle/ORACLE-CONTINUATIONS.json` SHA-256 = `76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`, matching the frozen design pin; unchanged from `e9a2`.
- Successor repair: [lower.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-backend/src/stage/lower.rs:127) now calls `artifact.validate()` before the sole successful return in `compile_staged`, and its doc comment (lines 15–21) correctly states the predicate is shape-only, not provenance. `compile_staged` takes `_out` unused — no filesystem effect in the backend; publication remains the CLI's. Its only consumers are `crates/mori/src/commands/compile.rs:22`, `crates/mori-backend/tests/stages.rs`, and `stage_relation.rs`; the `/1` byte comparator path does not call it (staged-only repair).
- Labels/gates: [stage/mod.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-core/src/stage/mod.rs:35) emits exactly `local-reference-stage`/`none`/`untracked`; [artifact.rs](/home/charl/Moriarty/.worktrees/core-continuations-20261015/crates/mori-backend/src/stage/artifact.rs:42) fixes all ten `/4` gates to `unperformed` and refuses label/gate divergence at decode (lines 196–225). Zero-await pending representation follows A1.1 (`pending_positions: Option`, N=0 ⇒ `None`, no pending roles), per `DESIGN-OPUS-A2.md` C1.
- Captures: every `checks/*/result.json.gz` + stdout/stderr was decompressed and read. Fresh post-repair: `core-hostile-02` 22 passed, `backend-normal-03` 5 (1 comparator ignored, separately passed in `legacy-byte-comparator-01`), `backend-relation-02` 9, `cli-normal-03` 7, `ledger-source-closure-02` 6 = 49. Carried forward: `core-normal-01` 5, `core-compat-01` 4, `core-static-cost-01` 13, `backend-frame-01` 6 (17 filtered), `legacy-byte-comparator-01` 1 = 29. Total 78 distinct; no rerun sums. Source test-function counts independently match (hostile 22, relation 9, closure 6, CLI 7, continuations 5, compat 4, static-cost 13).
- Failures retained, not counted: `backend-normal-01` exit 101 (4 passed/1 failed/1 ignored), `cli-normal-01` exit 101 (0 tests, unhandled `Statement::Await`), `quint-lifecycle-03` exit 1 (QNT404; fixed by two direct-constructor imports in `-03b`, then 9 tests pass in `-04`; passed prefix runs not repeated).
- Capture-pin deltas (`CAPTURE-PIN-COMPARISON-CURRENT.json`, 850 pins/20 stream hashes over 10 captures) assessed against call paths: carried-forward deltas are confined to `stage_hostile.rs` (test-only, expanded after those captures), `lower.rs`/`stage_relation.rs` (not called by the frame unit tests or the `/1` comparator), and the examples README (documentation only). No delta touches a production call path of the carried-forward selections. Stream hashes match their result receipts in all 10.
- Quint: prefix 2 tests/4 one-transition samples and lifecycle 9 tests/32 sampled traces are reported separately from the 78, with no Rust correspondence or native proof claimed — accurate in README, guide, and ROADMAP.
- DOCS F1 closure: `examples/core/continuations/README.md` no longer claims "not executed"/"specified-only"; it states the 49+29 split, non-rerun, pending reviews, and open U0–U7/M1–M17/MC/SP/G exits. `docs/CORE-CONTINUATIONS.md` and the ROADMAP successor section match the same counts and limits. No publication, merge, or native qualification is asserted anywhere inspected; beta Source 6/Core 5 remains separate.

## Findings

No required (blocking) nonsecurity finding was identified in the inspected source, captures, or docs. Two optional notes:

1. Optional — `SOURCE-SCOPE-CURRENT.json` / `OBSERVATIONS-CURRENT.json`: `successor_commit` remains `PENDING_ROOT_FREEZE`. Predicate: a committed snapshot cannot contain its own commit hash, so this is an expected placeholder, not a defect; consumers must bind evidence to `7d6c3965…` externally (as this review does). Trace: evidence-binding hygiene, FINAL-NONSECURITY scope note.
2. Optional — adapter generic `report_kind` values (e.g. `opus-independent-security-review`) on author/finisher/classifier receipts do not establish independence; the underlying reports themselves state the non-independent role. This review relies only on decompressed captures and source bytes, not on any prior report's disposition. Trace: AGENTS.md reviewer-identity rule.

## Unread / incomplete required coverage (why SOURCE is INCOMPLETE)

- Full line-by-line reading of every new production module (`stage/{admit,cost,decode,validate,plan,frame,context,artifact}.rs`, `runtime.rs`/`parser.rs`/`ir.rs` diffs, CLI command bodies) was sampled at predicate level (labels, validate-before-return, signed-target refusal, resume conflicts, zero-await branches), not exhaustively audited line by line.
- Test bodies were counted and spot-read, not each assertion re-derived against the oracle JSON (4,488 lines) line by line; financial totals were checked against README/oracle summaries, not recomputed independently.
- Quint model sources/adapters, legacy-baseline leaf bytes beyond the manifest/postcheck records, and the full historical review corpus were read as directions/summaries, not re-audited.
- All ten `/4` native/target gates remain unperformed by design; UNI-006/007/008 and all U0–U7/M1–M17/MC/SP/G exits remain open and are not closed by this review.

Terminal identity: muse-wonder-1.0, high effort, fresh non-author nonsecurity reviewer. Final: SOURCE INCOMPLETE, ACTUAL-RESULTS APPROVE (scoped captured-local), DOCS APPROVE (scoped).
