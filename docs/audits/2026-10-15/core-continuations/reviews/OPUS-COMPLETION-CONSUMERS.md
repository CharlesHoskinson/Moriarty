# Opus 5.5 completion review: Scope C (consumers, security and evidence binding)

Reviewer: Opus 5.5 (claude-opus-5-5), high effort, fresh and non-author. Read-only.
Candidate `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`, base `fe01bc8b7d889427192c304b0e75d0ec719f63db`.
This covers Scope C only. It is not approval of the whole feature: the final integrated Opus review after A/B/C is still required. Native authority is 0, and no counters were reset.

## Verdict: NEEDS_CHANGES

There is one required finding (F1, Low severity, test-only fix). The coverage gaps listed below also prevent `APPROVE_SUBSCOPE` even once F1 is fixed. No production-code defect was found in the CLI or ledger hunks I read.

## Startup and commands

- I read `AGENTS.md`, `plugins/moriarty-dev/skills/develop/SKILL.md` and `references/execution-focus.md`.
- I ran `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json` once from the inherited cwd, with no compound `cd`. It reported the historical SP01.6 loan-swap campaign as blocked (stale inputs, missing accounting). That is operational history, unrelated to this review, and nothing was dispatched.
- Commands used: `git status/diff/show`, `sha256sum` and `gzip -cd`, plus file reads and greps. I ran no Cargo, build, test, Quint, SDK, native, ledger or network action.

## Hash checks I did independently

1. `sha256sum --check CANDIDATE-CODE-CHECKSUMS.txt`: every listed file is OK.
2. `git diff e9a2b80b --stat` touches only `examples/core/continuations/README.md` (+11/−5). So every other tracked file in the worktree equals the candidate. This ties the checksum file to the candidate independently of the root's work.
3. `git show e9a2b80b:examples/core/continuations/README.md | sha256sum` gives `ece59582…11aa8`. This equals the binding file's README pin, which is the historical candidate README that the captures used.
4. I hashed 10 files with `git show e9a2b80b:<path> | sha256sum`. All 10 equal the binding values: `simulate.rs`, `inspect.rs`, `check.rs`, `compile.rs`, `core_io.rs`, `mori/tests/core_continuations.rs`, ledger `relation/mod.rs`, ledger `proving.rs`, `layout.rs` and `tests/source_closure_coverage.rs`.
5. I opened `cli-normal-02/start.json.gz`. Its pins for `check`, `compile`, `core_io`, `inspect`, `proof_io`, `simulate`, `core_continuations.rs` and ledger `relation/mod.rs` equal the binding values.
6. I recomputed the stdout SHA-256 for `cli-normal-02` (`e81cf378…`) and `ledger-source-closure-01` (`e5148c68…`). Both equal their result receipts.

**Root premises I did not recompute:** the other ~155 entries among the 169 unique blob hashes; the start.json pins of the other 9 successful captures and the 2 failed ones; the stdout/stderr hashes of the other captures; `EVIDENCE-INVENTORY.json`; `SHA256SUMS`. For these I relied on the root observation in `CANDIDATE-CAPTURE-BINDING.json`, not on my own check.

## Captures I inspected (result, stdout and stderr, decompressed)

All 10 successful runs: core-normal 5, compat 4, static-cost 13, hostile 10, cli-normal-02 7, backend-normal-02 5 (comparator ignored), backend-relation 4, ledger-source-closure 3, legacy-byte 1, backend-frame 6 (17 filtered out). The sum is **58**. Each exited 0 with `native_actions_requested: 0` and unchanged pins. Every test name matches the source tests I read. `OBSERVATIONS.json`, the dossier README, `docs/CORE-CONTINUATIONS.md` and the ROADMAP delta (Core 32 = 5+4+13+10) agree with these totals.

The two initial failures are kept and not double-counted:
- `cli-normal-01` exited 101 with **E0004**, a non-exhaustive `Statement::Await` match at ledger `relation/mod.rs:271`, and ran zero tests. The fix at candidate `relation/mod.rs` makes `statement_root` return `Result` with a typed `core1_refusal()` and propagates `?` at the emission check. No expression is faked.
- `backend-normal-01` exited 101 with 4 passed, 1 failed and 1 ignored. The failure was a `&` lex error in the `stages.rs:45` specimen at source load. `backend-normal-02` passed after the specimen correction.

**Frozen oracle:** `cli-normal-02` passed the CLI test `accepted()`, which asserts `normal_output_labels_from_opus` on every accepted report. It also checks pre/post state, effects, steps, captures, residuals and attempted/committed work against `examples/core/continuations/normal-oracle.json`. That file's pin `76075b63…` equals `FROZEN-DESIGN.json` `ORACLE-CONTINUATIONS.json`. The retained stdout is only the harness summary, so the CLI JSON bodies are evidenced through the passing assertions, not a direct dump.

## Source I read and what it shows

**`simulate.rs` (full file, plus the candidate diff)**
- Clap: `--resume` requires `--event` and `--event` requires `--resume`. `--resume` conflicts with `scenario`, `trace`, `target`, `profile`, `authority`, `at` and `signature`, which are the real field IDs of the flattened `AuthorizationArgs` (`replay.rs:15-28`). `--action` and `--scenario` are `required_unless_present = "resume"`. `--logical-cap` conflicts with `--trace`.
- Resume reads in this order: source (`read_source`, 1 MiB), then `stage::load`, then the continuation (`read_bounded`, 4 MiB+1 take), then the event (64 KiB+1). Every failure after the source read goes through `no_work` and `print_stage_result`, so it is a labelled rejection with exit 1.
- A `/2` start keeps the 1 MiB scenario bound with strict JSON parsing, and its errors are labelled. `print_stage_report` inserts all three `STAGE_LABELS` (`stage/mod.rs:34-38`).

**`check.rs` and `inspect.rs`**
- `/2` JSON check and source-only inspect use `stage::load` and labelled reports.
- `inspect --target` on a `/2` source refuses with Core's typed profile error before it reads the target.
- The legacy helpers (`statement_rule`, `expression_constraints`, `action_report`, `enforcement_report`) now return `Result`. An Await gets a typed `mori::core::profile` refusal, with no panic and no omitted statement.

**`compile.rs`, `core_io.rs` and `proof_io.rs` (full files)**
- On a `/2` source, compile reads the Target and calls `compile_staged`, which refuses a signed Target before loading (`lower.rs:19-25`).
- Compile serializes fully, then `create_dir`, then publishes each circuit with `publish_new` (create_new staging plus a `hard_link` that cannot overwrite). The manifest is published last. The manifest carries the labels (`lower.rs:95-97`).

**Ledger hunks (all of them)**
- `require_core1` runs first in `admit_bindings_inner`, `compile_source_static_roles`, `SignedStaticBindings::from_source` and `static_compile::compile`.
- Each `/1` statement match now has a typed `Statement::Await` refusal arm: `layout.rs:261`, `ops.rs:332`, `compiler.rs:512,521`, `relation/mod.rs:272` and `signed/mod.rs:307`.
- The other ledger entry points (`prepare.rs:347`, `layout.rs:659/709`, `local_deployment.rs:37`) re-run `mori_core::parse`, which is `/1`-only (`parser.rs:48-53`), so a `/2` source is refused there.
- Backend `compile`, `compile_signed` and `compile_mode` (`lib.rs:366-388`) and `signed_relation.rs:50,676-677` call `require_core1` or the legacy parse.
- **Source-closure append:** `proving.rs` appends exactly the 8 stage files from FINAL A2, in alphabetical order, for 69 entries total. H28 tests 1 and 2 enforce this.

**Doc delta:** `examples/core/continuations/README.md` replaces the stale "not executed" text. It keeps "Full security review remains pending" and every U/M/MC/SP/G exit open, and says the observations establish no native proof or ledger acceptance. It is accurate against the captures and needs no change.

## Findings

**F1: Required, Low severity.** `crates/mori-ledger/tests/source_closure_coverage.rs:157-164` departs from the frozen design.
- `DESIGN-OPUS-FINAL.md` A2 (line 48) has precedence over the Clarification. It requires the coverage test to walk `mori-core/src`, `mori-lexer/src` **and** `mori-ledger/src` recursively, with each `.rs` file either listed or in a reasoned exclusion list. Clarification §3.4 (lines 207-213) asks for a committed exclusion list with a one-line reason per entry.
- The test walks only `mori-core/src`, and there is no exclusion list. The author's premise P3 (`reviews/OPUS-SOURCE-CLOSURE-COVERAGE.md:34`) narrowed the scope without any design disposition.
- Today the closure does cover every non-test ledger and lexer file (I checked by glob). The only unlisted files are `mori-lexer/src/{proptests,tests}.rs` and `mori-ledger/src/signed/tests.rs`. So the current closure value is complete, but a future ledger or lexer source file left out of the closure would not be detected.
- Fix (test-only, within the approved scope, no design vote): add lexer and ledger walks with a committed exclusion list of those three test files and their reasons. Then capture the run afresh.

**F2: Optional, Low.** `simulate.rs:63-77` with a `/2` source plus `--trace --target` behaves like the legacy path.
- It reads the Target, authorization files, source and scenario before `replay::generate` reaches `signed_relation` legacy parse or `require_core1`, which then refuses with an unlabelled legacy error.
- From the source I read, no native operation is reached first, but I did not read the bodies of `reconstruct` and `accept` line by line.
- Optional: detect `/2` before the trace branch and give a labelled refusal.

**F3: Optional, Low (doc).** `docs/CORE-CONTINUATIONS.md:104` says "Every staged result carries the … labels". Human-mode `check`, `/2` compile errors, the `inspect --target` refusal and an unreadable resume source all print unlabelled stderr diagnostics. Suggested wording: "every staged JSON report and `/4` manifest".

**F4: Optional, evidence gap.** No captured test exercises these through the public CLI:
- clap conflict refusals;
- oversized continuation or event reads;
- `/2` with `--trace`;
- `/2` with `inspect --target`;
- a signed Target for `/2` compile;
- the ledger `require_core1` refusals.

Those behaviours are source-inspected only. The backend refusal is covered by `legacy_compilers_refuse_staged_programs`, and the decode bounds by the Core hostile tests at API level.

## Reading coverage

**Read fully:** `check.rs`, `compile.rs`, `core_io.rs`, `proof_io.rs`, `simulate.rs`, `mori/tests/core_continuations.rs`, `mori-ledger/tests/source_closure_coverage.rs`, all ledger hunks, `CANDIDATE-CAPTURE-BINDING.json`, `OBSERVATIONS.json`, `FROZEN-DESIGN.json`, `DESIGN-OPUS-A2.md`, the dossier README, `docs/CORE-CONTINUATIONS.md`, the ROADMAP delta, the README doc delta, `OPUS-SOURCE-CLOSURE-COVERAGE.md`, and the header of `OPUS-CLI-CONFLICT.patch`.

**Read in part:** `inspect.rs` (lines 1-95 plus every candidate hunk); `parser.rs:1-60`; `stage/mod.rs:20-99`; `decode.rs:1-50`; `lower.rs:1-60`; backend `lib.rs:355-394`; backend `stage/mod.rs`; `DESIGN-OPUS-FINAL.md:28-52`; `DESIGN-OPUS-CLARIFICATION.md:196-218`; `security-reconciliation-2026-10-14.md` (lines 165-226 plus grep); ROADMAP U3 row 112.

**Not read (exact list):**
- `docs/MORIARTY-PRODUCT-CONTRACT.md` body (grep only);
- the U3 EARS text: `openspec/changes/consolidated-language-kernel/requirements.md` has no U3 or continuation match, and I did not find the right EARS location;
- `wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md`;
- `SOURCE-SCOPE.json` contents (I only counted 56 path entries);
- `DESIGN-OPUS.md`, the remainder of `DESIGN-OPUS-FINAL.md` and `DESIGN-OPUS-CLARIFICATION.md`, `FINAL-NONSECURITY-DESIGN.md`, `ADOPTION-SOL.md`;
- `execution-inputs/CORE-ACTUAL-CHECK-SUMMARY.json` (stale history; the README marks it superseded);
- `crates/mori/src/cli.rs`;
- the unchanged helpers in `inspect.rs` (`public_field`, `authorization_rule`, `unknown_premises`);
- the bodies of `signed_relation::reconstruct` and `signed_relation::accept`;
- the mori-core `stage/*` remainder (Scope A) and backend `artifact.rs`/`frame.rs` (Scope B).

Quint is outside this scope. I make no claim about the first prefix (2 tests and 4 traces, separately counted), and the later lifecycle increment is not part of this candidate.

## What this review does not establish

Native proof, ledger acceptance, event authentication, consumption, any of the ten `/4` gates, or any U0–U7, M1–M17 or MC/SP/G exit.
