**Verdict: NEEDS_CHANGES for Scope C.** One required finding, and it's a small test-only fix. I found no production-code defect in the CLI or ledger changes I read. Even with that fix, Scope C can't be approved until the unread files listed below are covered. The report is at `R/OPUS-COMPLETION-CONSUMERS.md`. The guarded status command ran fine from the inherited cwd; it shows only the old SP01.6 campaign, and I dispatched nothing.

**Required (Low):** `crates/mori-ledger/tests/source_closure_coverage.rs:157-164` only walks `mori-core/src`. The frozen design (FINAL A2, which takes precedence) requires walking `mori-core`, `mori-lexer` and `mori-ledger`, with a committed exclusion list that gives a reason for each entry. The author narrowed this on their own premise (P3) without a design decision. Today the closure does cover every non-test file. The only files it leaves out are `mori-lexer/src/{proptests,tests}.rs` and `mori-ledger/src/signed/tests.rs`. So the current value is complete, but a future ledger or lexer file left out wouldn't be caught. Fix: add those two walks plus an exclusion list of the three test files, then capture a fresh run.

**Optional (Low):**
- **F2:** running `simulate --trace --target` on a `/2` source reads all the side files before the old parser refuses it, and the error carries no stage labels. From what I read, no native step happens first.
- **F3:** the guide's line "every staged result carries the labels" is too broad. Human-mode `check` output and `/2` error messages on stderr have no labels.
- **F4:** the CLI conflict refusals, the size limits, and the `/2` refusals in the native and ledger paths have been read in source but never exercised by a captured test.

**Hash checks I did myself:**
- `sha256sum --check` on the checksum file: every entry OK.
- `git diff e9a2b80b`: only `examples/core/continuations/README.md` differs, so every other file in the worktree equals the candidate.
- The candidate's README hashes to `ece59582…`, matching its pin in the binding file.
- 10 files hashed with `git show e9:<path> | sha256sum` all match the binding.
- The `cli-normal-02` start record's pins match for the 8 CLI/ledger files I checked.
- I recomputed the stdout hashes for `cli-normal-02` and `ledger-source-closure-01`; both match their receipts.

**Not recomputed (I relied on the root's binding file):** the other ~155 file hashes, the start-record pins of the other 11 captures, and the remaining stdout/stderr hashes.

**Captures:** I decompressed all 10 successful runs and they total 58 passing tests, with zero native actions. The two kept failures (the E0004 compile error and the 4 pass / 1 fail / 1 ignored backend run) aren't double-counted, and both fixes are in the candidate. The README doc correction is accurate and needs no change.

**Not read:**
- the product contract body (I only searched it);
- the U3 EARS text (no U3 or continuation match in `requirements.md`, and I didn't find where it lives);
- `wiki-llm/SECURITY-CURRENTNESS-2026-10-14.md`;
- the contents of `SOURCE-SCOPE.json` (I only counted 56 entries);
- `DESIGN-OPUS.md`, the rest of `DESIGN-OPUS-FINAL.md` and `DESIGN-OPUS-CLARIFICATION.md`, `FINAL-NONSECURITY-DESIGN.md`, `ADOPTION-SOL.md`;
- `CORE-ACTUAL-CHECK-SUMMARY.json`;
- `cli.rs`;
- the unchanged helpers in `inspect.rs`;
- the bodies of `signed_relation::reconstruct` and `accept`.

The final integrated Opus review is still required, and nothing here closes any U, M, MC, SP or G exit.
