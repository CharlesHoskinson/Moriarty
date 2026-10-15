# Opus 5.5 scope A completion: fresh non-author review

- **Candidate:** `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`, base `fe01bc8b`. The worktree was clean.
- **Startup:** the guarded `status --json` ran. It reports only historical campaign state; I changed no counters and Authority is 0.
- **Role:** I am not an author, finisher or classifier of this candidate.
- **What I did:** read-only source, git and hash work, plus Read of the root-decompressed public captures. I ran no build, test, Quint, SDK or native step.
- **Scope:** this closes the items OPUS-SUCCESSOR-A.md left unread. The Core stage modules are cited from that report (premise P1) and were not reread.
- **Parents remain open:** whole-feature integrated Opus review, scope B, and every U0–U7, M1–M17, MC, SP and G exit.

## Verdicts (scope A only)

| Axis | Verdict |
|---|---|
| SOURCE | **APPROVE_SUBSCOPE with 2 required test-coverage findings (A-C1, A-C2).** I found no product-code defect in the CLI, ledger or closure source I read. |
| ACTUAL-RESULTS | **APPROVE_SUBSCOPE.** The 57 selected scope-A functions are confirmed from actual stdout, and their carry-forward is sound. One limit is listed below. |
| DOCS | **APPROVE_SUBSCOPE with 1 required finding (A-D1).** |

These verdicts cover scope A only. They are not approval of the whole feature, of native work or of security qualification.

## Direct coverage (read in full with Read unless noted)

### Item 1: CLI, ledger and closure source

- **CLI commands:** `simulate.rs` (234 lines), `inspect.rs` (378), `check.rs` (136), `core_io.rs` (379), `proof_io.rs` (184), and `compile.rs` (its diff plus the new staged publication).
- **`cli.rs`:** routing lines only. It is unchanged from base.
- **inspect.rs:** I read `public_field`, `authorization_rule` and `unknown_premises` and confirmed their bodies are unchanged.
- **Trace boundary:** I read `replay::generate` (`replay.rs:141-170`). It reaches `signed_relation::reconstruct`/`accept`, which call `mori_core::parse` and then `require_core1` at `signed_relation.rs:676-677`. So `/2 --trace` is refused structurally, with a defence-in-depth check behind it.
- **Ledger diff base..candidate:** I read all of it: `layout.rs`, `ops.rs`, `proving.rs` (the closure list in full, 69 members), `relation/{compiler,mod}.rs`, `signed/mod.rs` and `static_compile.rs`.
- **Tests:** `source_closure_coverage.rs` in full (6 tests) and `crates/mori/tests/core_continuations.rs` in full (7 tests).

### Item 2: Core tests

- I read in full `continuations.rs` (5 tests), `compat_v1.rs` (4) and `stage_hostile.rs` (all 883 lines, 22 tests). The last includes the H11, H12, H24 caps and `read_bounded` tests, the three F2 tests and `TempInput`.
- I read `stage_static_cost.rs` lines 100-554, which contain all 13 tests. Lines 1-99 (helpers and constants) I saw only through their uses.

### Item 3: normative documents

- `docs/MORIARTY-PRODUCT-CONTRACT.md` in full.
- `spec.md` lines 59-90 (UNI-006/007/008).
- `wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md` in full.
- `FROZEN-DESIGN.json` and `DESIGN-OPUS-A2.md` (A1.1) in full.
- Targeted parts only:
  - `DESIGN-OPUS-FINAL.md`: A9/A10/A11, lines 318-347, plus a grep for H-items.
  - `DESIGN-OPUS.md`: the H1–H22 table, lines 278-301, and P1.
  - `DESIGN-OPUS-CLARIFICATION.md`: the H-to-file map.
  - `DESIGN-MUSE-FINAL.md`: MF6, MF7 and MF9.
  - The dossier `README.md` (labels section 205-239, plus a grep) and the examples `README.md` in full.
  - `docs/CORE-CONTINUATIONS.md` lines 105-134, plus a grep.

### Item 4: actual results

- I read stdout and `result.json` for all 10 relevant Core, CLI and closure captures, and the root `CAPTURE-PIN-COMPARISON-CURRENT.json` and `OBSERVATIONS-CURRENT.json` in full.
- I hashed 11 current files with `sha256sum`. Each matches its fresh capture `start.json` pin and its `SOURCE-SCOPE-CURRENT.json` entry: `stage_hostile.rs` e8d3af…, `core_continuations.rs` 81cdd7…, `simulate.rs` 724886…, `inspect.rs` 5cef6e…, `compile.rs` 8ec1da…, `check.rs` 2199d7…, and `source_closure_coverage.rs` 85d144….
- The oracle is `76075b63…`. That hash matches the dossier copy, `examples/core/continuations/normal-oracle.json` (the CLI test's `include_str!`) and `crates/mori-core/tests/fixtures/continuations/normal-oracle.json` (the Core test's).

## Source predicates verified

1. **Resume validation order (simulate).** `--resume` conflicts with scenario, trace, target, profile, authority, at and signature in clap (`simulate.rs:19-25`), so a bad invocation fails before any file read. The resume path then reads in a fixed order: source, then `stage::load`, then the continuation (`read_bounded` 4 MiB), then the event (64 KiB). Each failure before evaluation becomes a labelled rejection with attempted 0 and committed 0 (`simulate.rs:181-234`). The `/2` start keeps the 1 MiB strict scenario read (`:211`).
2. **Labels.** Every staged report is built by `print_stage_report`, which inserts all three `STAGE_LABELS` (`simulate.rs:141-147`). `check --json` on `/2` and source-only `inspect` both use it (`check.rs:45-46`, `inspect.rs:26,31`).
3. **Staged compile publication.**
   - `create_dir` refuses an existing `--out` (`compile.rs:25`).
   - Each file is published through `publish_new` (`create_new`, `sync`, then `hard_link`), and the manifest is written last as the completion marker (`compile.rs:81-93`, `proof_io.rs:111-142`).
   - Premise not verified: that action identifiers cannot contain path separators. That depends on the lexer's identifier charset, which I did not read.
4. **inspect on `/2`.** `inspect --target` on a `/2` source returns the typed profile refusal before reading the target (`inspect.rs:39-49`). The Await arms in `statement_rule` and `expression_constraints` return typed errors, not `unreachable!` (`:289,319`), which satisfies A9.
5. **Ledger (A9).**
   - `require_core1` is called at `layout.rs:158`, `relation/mod.rs:361`, `signed/mod.rs:210` and `static_compile.rs:370`.
   - The Await arms are at `layout.rs:261`, `ops.rs:332`, `relation/compiler.rs:512,521` and `signed/mod.rs:307`. `statement_root` now returns `Result`.
   - None of these uses a panic or a `_` fallthrough.
6. **H28 closure.**
   - The closure is the 61 frozen baseline members, then exactly the 8 Core stage files (`source_closure_coverage.rs:200-207`).
   - The walk is recursive over the Core, lexer and ledger `src` directories, refuses symlinks, and accepts only 3 exclusions. Each exclusion is checked as a `#[cfg(test)]` module of a closure-member parent, and the test refuses any stage-file exclusion.
   - Exclusions that are unknown or unused fail the test, and a synthetic control covers both cases.
   - The 6 tests match the frozen A10 H28 requirement.
7. **`TempInput` (`stage_hostile.rs:737-759`).** Files are created with `create_new`, so an existing file or symlink is never written. The path is pid- and tag-unique and is removed on drop. The guard is constructed before the write, so a panic still cleans up. The largest file is 4 MiB + 1.

## Hostile-family map (H1–H28)

| H | Test location | Scope |
|---|---|---|
| H1, H2 | `stage_hostile` h1_h2, plus `static_cost` digests | A |
| H3, H4, H5, H8, H13, H14 | `stage_hostile` | A |
| H6 | `stage_hostile` h6 | A, but see A-1 |
| H7 | `stage_hostile` h7 | A |
| H9, H10, H11, H12 | `stage_hostile` h9–h12 (h11/h12 messages, steps and attempted counts match the frozen values in `DESIGN-OPUS.md:286-289`) | A |
| H15 | `static_cost:506-507` (parse OK) | Execution-byte identity is the /1 comparator's job, scope B |
| H16 | Core `stage::load(/1)` gives profile (`static_cost:502`) | **CLI part untested: A-C2** |
| H17, H26 | `static_cost` proptests and deterministic tests, plus `stage_hostile` h17_h26 | A |
| H22 | `static_cost` h22 ×2 | A |
| H23 | `compat_v1` ×3, plus the C7 test | A |
| H24, H25 | `stage_hostile` | A |
| H28 | `source_closure_coverage` | A |
| H18, H20, H21, H27 | `stage_relation.rs` / `frame` | Scope B (boundary only) |
| H19 | No test found | **A-C1** (ledger refusal); emission is scope B |

## Findings

| ID | Location | Predicate | Severity | Required? | Trace |
|---|---|---|---|---|---|
| A-C1 | `crates/mori-ledger/src/layout.rs:62-68`; entries at `static_compile.rs:370`, `relation/mod.rs:361`, `signed/mod.rs:210`; arms at `ops.rs:332`, `compiler.rs:512,521`, `signed/mod.rs:307`, `layout.rs:261` | A9 says "Tests pin the exact error identity" for the ledger `require_core1` entries. A grep of `crates/mori-ledger/**` finds no test that references `core1-required` or a `/2` program. H19 (`CompileCall` with `/2` refused before emission) has no test. The source refusals look correct on inspection; the required test pins are missing. | MEDIUM (coverage) | Required | DESIGN-OPUS-FINAL A9 (lines 319-329); DESIGN-OPUS H19 (line 296) |
| A-C2 | `crates/mori/tests/core_continuations.rs` (whole file) | No CLI test covers these surviving H16 cases:<br>• a `/1` program given to `simulate --resume`, which should be a labelled profile rejection;<br>• `/2` with `--trace --target`, which should print `rejected`;<br>• `inspect --target` on `/2`, which should be a typed profile refusal (`inspect.rs:39-49`).<br>The Core API is covered. I found no superseding A2 or FINAL text. | LOW–MEDIUM | Required (frozen H16) | DESIGN-OPUS.md:293; A9 (inspect arms) |
| A-D1 | `docs/CORE-CONTINUATIONS.md:112-124`; `examples/core/continuations/README.md:41-52` | Neither document says that the stored `continuation.logical.cap` is outside both digests and so is advisory. A caller can remove or raise it, and only the CLI cap ≤ stored cap and the committed-work checks are enforced (`stage_hostile.rs:195-206`). UNI-008's hostile witness ("rollover resets a spent budget") makes this disclosure material. The `authentication:none` label alone is not specific enough. | LOW | Required for DOCS accuracy (not a security blocker) | UNI-008 (`spec.md:80-89`); P1 and P3 |
| A-1 | `crates/mori-core/tests/stage_hostile.rs:413-414` | The comment claims "a JSON escape decodes to the same key", but the payload `"site":1,"site":2` contains no escape. Frozen H6 (DESIGN-OPUS.md:283) requires only plain duplicate keys, in the event and in the continuation, and both are covered. Escaped keys are optional. | LOW | Optional | H6 |
| A-2 | `stage_hostile.rs:425-426` | The `(flag, …)` case appears twice, which adds no coverage. | INFO | Optional | H6 |
| A-3 | `stage_hostile.rs:2-4`, `:327-329`; `stage_static_cost.rs` header | The comments still say "Written before execution; not yet run", which is now stale. | INFO | Optional | Docs hygiene |
| A-4 | `OBSERVATIONS-CURRENT.json:4` | `successor_commit` reads `PENDING_ROOT_FREEZE`. A file committed in the tree cannot name its own commit, so the root must bind it to `7d6c3965` externally. | INFO | Optional | Evidence identity |

## Actual results

**The 57 distinct scope-A functions.** These come from the plain stdout `test … ok` lines and the `test result:` line of each selected capture:

| Capture | Passed |
|---|---|
| `core-normal-01` | 5 |
| `core-compat-01` | 4 |
| `core-static-cost-01` | 13 |
| `core-hostile-02` | 22 |
| `cli-normal-03` | 7 |
| `ledger-source-closure-02` | 6 |

- Every selected capture reports `exit_code` 0 and `native_actions_requested` 0.
- The test names match the test-function lists I read in source, one to one.
- `core-hostile-02`, `cli-normal-03` and `ledger-source-closure-02` report empty leftover lists, and `core-hostile-02` also reports no new property-seed files.

**Arithmetic.**
- 49 fresh = 22 + 5 + 9 + 7 + 6.
- 29 carried forward = 5 + 4 + 13 + 6 + 1.
- 49 + 29 = 78, as `OBSERVATIONS-CURRENT.json` states.
- No reruns are summed. These historical runs are excluded: `core-hostile-01` (10 tests), `cli-normal-02` (7) and `ledger-source-closure-01` (3).

**Original failure retained, not counted.** `cli-normal-01` has `exit_code` 101, and its stderr is `error[E0004]: non-exhaustive patterns: Statement::Await` in `mori-ledger` (stderr lines 11 and 32). Its stdout is empty, with the empty-input hash e3b0c4….

**Carry-forward deltas, checked against call paths.**
- `core-normal-01`, `core-compat-01` and `core-static-cost-01` differ from current bytes in only two files:
  - `crates/mori-core/tests/stage_hostile.rs` is a separate integration-test crate, so it is not compiled into the `continuations`, `compat_v1` or `stage_static_cost` binaries.
  - `examples/core/continuations/README.md` is not `include_str!`'d anywhere: a grep of `crates/` finds only unrelated README references.
- So the carry-forward is sound.
- The `lower.rs` and `stage_relation.rs` deltas on `backend-frame-01` and `legacy-byte-comparator-01` belong to scope B.

**Evidence form.** CLI JSON assertions (labels, financial snapshots, the oracle comparisons at `core_continuations.rs:52-93`) are shown by tests passing, not by captured JSON bodies, and the dossier does not claim otherwise.

**Quint.** Not in scope A. It is separate from the Rust totals (`OBSERVATIONS-CURRENT.json` `separate_models`, `included_in_rust_total:false`).

## Docs

- The three labels are exact in the README (`:231-232`), the examples README (`:44`) and `CORE-CONTINUATIONS.md` (`:123`).
- "All ten `/4` gates unperformed" is stated in README `:234-235`.
- Native authority is 0, and the U, M, MC, SP and G gates are stated as open.
- No claim of conditional settlement, consumption, native acceptance or ledger acceptance is made, which is consistent with UNI-007 staying unmet. H13 and H14 pin the gap honestly.
- The product contract (`:76,80-82`) calls persistent continuations unfinished. Nothing in scope A contradicts that.
- The only gap is A-D1.

## Previous-review premises (not reverified here)

- **P1.** The 8 Core stage modules were read in full at `7d6c3965`, together with runtime, parser and json, in OPUS-SUCCESSOR-A.md, and no required defect was found.
- **P2.** That report ran `sha256sum -c SHA256SUMS`: 417 entries OK. My own run failed only because of a working-directory path issue: the entries are dossier-relative, and a `cd` was not permitted.
- **P3.** The `MANIFEST.json` lossless-decompression hashes are taken as root-supplied.

## Unread or partial (explicit)

- **Full stderr of the selected passing captures.** I read only `result.json` hashes and stdout. I read the `cli-normal-01` stderr failure lines.
- **`start.json` of the selected captures.** I grepped them for the pins and command above; I did not read them in full.
- **The 84-path manifest.** I did not enumerate all 56 + 28 entries of `SOURCE-SCOPE-CURRENT.json`; I spot-checked 7 of them against current bytes, all matching.
- **`stage_static_cost.rs` lines 1-99.**
- **Normative text:**
  - `security-reconciliation-2026-10-14.md` (OSR-1014): not read.
  - `DESIGN-OPUS-FINAL.md` outside A9–A11: not read in full.
  - `DESIGN-OPUS-CLARIFICATION.md` and `FINAL-NONSECURITY-DESIGN.md`: grep only.
  - `DESIGN-OPUS.md` outside the H table and P1, and `ADOPTION-SOL.md`: not read.
- **The lexer identifier charset**, which source predicate 3 depends on.

None of these blocks the verdicts above, which are scoped to what I read. They remain open for the integrated Opus review.
