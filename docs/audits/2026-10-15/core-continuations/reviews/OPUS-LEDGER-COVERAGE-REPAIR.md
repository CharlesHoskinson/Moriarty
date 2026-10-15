# Opus ledger coverage repair (F1): test-only source fix

- **Requested identity:** Opus 5.5, high effort, sole security author.
- **Returned identity:** Opus 5.5 (`claude-opus-5-5`), high effort.
- **Status:** terminal. The source write is complete. It has not been compiled or run.
- **Scope:** a bounded repair under the frozen design. This is not a new design. No design vote was cast and no code, security-policy or normative change was made.
- **Run:** WT=`/home/charl/Moriarty/.worktrees/core-continuations-20261015`, candidate `e9a2b80b`.

## Startup

- I read `AGENTS.md` and applied the checked-in `plugins/moriarty-dev/skills/develop/SKILL.md`.
- I ran `cli.py --repo . status --json` once. It reports the historical SP01.6 loan-swap campaign as blocked. That is operational history, unrelated to this task. Nothing was dispatched.

## Inputs I read

- `R/OPUS-COMPLETION-CONSUMERS.md` F1. It lives at lines 65-69.
- `docs/audits/2026-10-15/core-continuations/design/DESIGN-OPUS-FINAL.md` A2, lines 28-52. The coverage-test rule is at line 48: walk `mori-core/src`, `mori-lexer/src` and `mori-ledger/src` recursively. Every `.rs` file must be listed or appear in the reasoned exclusion list. `stage/*` files can never be excluded.
- `DESIGN-OPUS-CLARIFICATION.md` §3.4, lines 198-218: a committed exclusion list, with a one-line reason for each entry.
- `R/H28-PROPOSED-CHECKS.json`.
- The full owned test.
- `proving.rs::source_closure`, lines 150-225.
- The module declarations in `mori-lexer/src/lib.rs:14-17` and `mori-ledger/src/signed/mod.rs:340-341`.
- The head of `signed/tests.rs`.
- The U3 EARS discovery path `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md` UNI-006/007/008, lines 59-90. I read it for discovery only. No change of interpretation is needed.

## Source pins (sha256)

| File | Pre | Post |
|---|---|---|
| `crates/mori-ledger/tests/source_closure_coverage.rs` (owned) | `184f35a5d7af3dcc39983042859862e6b04ce86a57a2d4c0f8c2cb2118e4b0d5` (equals `CANDIDATE-CODE-CHECKSUMS.txt`) | **`85d144e1a57640fb180bc09b4af1e20f9e4b7c6aeca4d37e7b676aceaca298c5`** |
| `crates/mori-ledger/src/proving.rs` | `c636795d…5c38cb` | unchanged |
| `crates/mori-lexer/src/lib.rs` | `5cd5bd60…5e28d7` | unchanged |
| `crates/mori-ledger/src/signed/mod.rs` | `d9674af1…813257` | unchanged |

- The git blob of the owned test changed from `9ce375de` to `0a9e0ec7`.
- The diff is +125/−2 (127 changed lines in `git diff --stat`).
- **Worktree delta vs `e9a2b80b`:**
  - My owned test.
  - `examples/core/continuations/README.md`, a pre-existing doc fix.
  - `docs/CORE-CONTINUATIONS.md:104`, the F3 label wording. That change is not mine and was present when I first ran the diff.
- No other source changed. `source_closure` production code is unchanged.

## What changed (exact diff summary)

1. **`ROOTS`** = `["../../mori-core/src", "../../mori-lexer/src", "."]`. These are the three frozen A2 roots, resolved from the ledger `src`.
2. **`EXCLUDED`** has exactly 3 entries. Each entry gives `path`, `parent`, `module` and a one-line `reason`:

   | Excluded file | Declared by | Actual declaration | Reason |
   |---|---|---|---|
   | `mori-lexer/src/proptests.rs` | `mori-lexer/src/lib.rs:14-15` | `#[cfg(test)]\nmod proptests;` | lexer property tests, compiled only under cfg(test) |
   | `mori-lexer/src/tests.rs` | `mori-lexer/src/lib.rs:16-17` | `#[cfg(test)]\nmod tests;` | lexer snapshot unit tests, compiled only under cfg(test) |
   | `mori-ledger/src/signed/tests.rs` | `mori-ledger/src/signed/mod.rs:340-341` | `#[cfg(test)]\nmod tests;` | signed-call ordinary QA unit tests, compiled only under cfg(test) |

   No `mori-core` file is excluded.
3. **`rust_files`** now refuses symlinks. It uses `DirEntry::file_type`, which does not follow links, and fails closed. This only strengthens the original Core test.
4. **`walked()`** collects every `.rs` file under the three roots.
5. **`gaps(walked, listed, excluded)`** is a pure function. It returns two lists:
   - `missing`: files that are walked but neither listed nor excluded.
   - `unused`: exclusions that do not name an unlisted walked file. This catches entries that are stale, broad (such as a directory), already listed, or outside the roots.

## Tests

There are now 6 tests: 3 unchanged and 3 new. No literal expectation changed and no test was weakened.

**Unchanged:**
- `closure_is_frozen_baseline_then_core_stage_files`: the 61 ordered baseline members plus the 8 Core stage files, 69 exactly.
- `closure_members_are_distinct_existing_files`.
- `closure_covers_every_core_source_file`. It keeps its Core-only body and gains the symlink refusal.

**New:**
1. `closure_covers_every_core_lexer_and_ledger_source_file`
   - Walks all three roots.
   - Rejects duplicate exclusions.
   - Probes that the walk reaches lexer `lib.rs`, ledger `lib.rs` and `stage/validate.rs` in a subdirectory, so the check is not vacuous.
   - Requires `missing` and `unused` to be empty. An unknown future file therefore fails closed, and the list cannot act as an unused or broad allowlist.
2. `exclusions_are_reasoned_cfg_test_modules_outside_core_stage`. For each exclusion it checks that:
   - the reason is one line and non-empty;
   - the file is not under the canonical `mori-core/src/stage`. A control first asserts that all 8 `CORE_STAGE` files do resolve under that directory, so this check is not vacuous;
   - the `parent` is a closure member, so the hashed parent pins the file's test-only status;
   - the parent is `lib.rs` or `mod.rs`, and the excluded path equals the parent's sibling `<module>.rs`;
   - the parent contains `#[cfg(test)]\nmod <module>;\n` exactly once and `mod <module>` exactly once, so there is no second non-test declaration;
   - no walked file contains `<module>.rs"`. That would mean a `#[path]` or `include!` reference that pulls the file into non-test code.
3. `coverage_gap_controls_reject_unknown_files_and_unused_exclusions`. These controls use only in-memory `PathBuf` sets. No file is created, deleted or hidden, and the real tree is not touched.
   - Positive: no gaps.
   - Negative: an unknown future file is reported as missing.
   - Negative: a removed exclusion is reported as missing.
   - Negative: a broad directory, a listed member and a stale path are each reported as unused.

**Current-tree inspection.** I ran `find`. The three roots hold 51 `.rs` files: 18 Core, 8 lexer and 25 ledger.
- 48 are closure members: 18 Core, 6 lexer and 24 ledger. The other 3 are exactly the 3 exclusions.
- There are no symlinks.
- The lexer `snapshots/*.snap` files are not `.rs` files and are not walked.
- No source under the roots contains `tests.rs`, and none uses `include!(`.
- The existing `#[path = "../tests/support/mod.rs"]` attributes target `crates/mori-ledger/tests/`, which is outside the frozen roots. Their file name `mod.rs` does not collide with any exclusion.

I expect the 3 new tests to pass on today's tree. That expectation comes from source inspection; nothing was observed running.

**Bounds.**
- The tests do no hashing, call no `source_closure`, and do no proving, key, parameter or native work.
- The walk is read-only. It reads source path metadata and the public source text in the current repo only, with no external symlinks and no private material.

## Check classification (my scope)

- I classify the `R/H28-PROPOSED-CHECKS.json` argv, unchanged, as an **approved ordinary check** under the existing ledger pure-helper class, with the new owned-test pin `85d144e1…298c5`. The argv is `flock … cargo +1.99.0 --offline --locked test -p mori-ledger --test source_closure_coverage -j2 -- --test-threads=1`, with `RUSTFLAGS` unset and the same `CARGO_TARGET_DIR`.
- Conditions:
  - all other production bytes equal `e9a2b80b` (the only other delta is docs);
  - the build compiles `mori-ledger` as an ordinary dependency;
  - the test bodies only list and read public source paths;
  - native authority stays at 0;
  - stdout and stderr are kept;
  - the run expects 6 tests. Any other count or any failure blocks.
- Every selected closure was inspected, so the handoff is **approved ordinary**.
- I ran no Cargo, build, test, fmt, Quint, SDK, model, VM, proof, keygen, native, deploy, ledger or network action. Compile success and rustfmt stability are therefore **unverified**.

## Acceptance (separate)

- This is source writing, not security acceptance.
- F1 closes only after two things: a fresh capture of the argv above, and a **fresh, full, integrated Opus audit** of the changed candidate.
- The prior `ledger-source-closure-01` capture of 3 tests stays historical for the old pin `184f35a5…`.
- F2 and F4 are unchanged by this work. The independent oracle `76075b63…` is unchanged.
