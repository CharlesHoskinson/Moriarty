I've fixed the coverage test (F1): it now walks the Core, lexer and ledger source folders, not just Core. I haven't compiled or run it, because builds and tests were outside what I was allowed to do this round. So whether it compiles, passes or survives `fmt` is still unknown. I expect the 3 new tests to pass on today's files, based on reading the source.

I changed only `crates/mori-ledger/tests/source_closure_coverage.rs`, and wrote the report to `R/OPUS-LEDGER-COVERAGE-REPAIR.md`.

**Hashes**
- Test file before: `184f35a5d7af3dcc39983042859862e6b04ce86a57a2d4c0f8c2cb2118e4b0d5`. This matches `CANDIDATE-CODE-CHECKSUMS.txt`.
- Test file after: **`85d144e1a57640fb180bc09b4af1e20f9e4b7c6aeca4d37e7b676aceaca298c5`** (+125/−2).
- `proving.rs` (`c636795d…`), lexer `lib.rs` and `signed/mod.rs` are unchanged. The closure production code is untouched.
- The other uncommitted changes are both docs-only and not mine: the earlier `examples/core/continuations/README.md` fix, and the F3 wording fix at `docs/CORE-CONTINUATIONS.md:104`.

**Exclusion list.** It has exactly 3 entries, each with one reason. I checked each against its real `#[cfg(test)]` declaration:
- `mori-lexer/src/proptests.rs` (`lib.rs:14-15`): lexer property tests, compiled only under `cfg(test)`.
- `mori-lexer/src/tests.rs` (`lib.rs:16-17`): lexer snapshot unit tests, compiled only under `cfg(test)`.
- `mori-ledger/src/signed/tests.rs` (`signed/mod.rs:340-341`): signed-call unit tests, compiled only under `cfg(test)`.

No Core file is excluded.

**Tests.** There are now 6: the 3 existing tests plus 3 new ones. The 61 ordered members, the 8 appended Core stage files (69 exactly) and the distinct-existing-files test are unchanged.
- `closure_covers_every_core_lexer_and_ledger_source_file`: every Rust file in the three folders must be in the closure or on the exclusion list. A new unknown file fails, and so does an exclusion that doesn't match a real unlisted file. That stops the list from acting as a broad allowlist.
- `exclusions_are_reasoned_cfg_test_modules_outside_core_stage`: each exclusion must have a one-line reason and sit outside the Core stage folder. It must also be declared exactly once under `#[cfg(test)]` by a parent file that is itself in the closure. No source file may pull it in another way, such as a `#[path]` attribute.
- `coverage_gap_controls_reject_unknown_files_and_unused_exclusions`: checks the gap logic with made-up paths held in memory, both cases that should pass and cases that should fail. No files are created or deleted.

The file walk now also refuses symbolic links, which only makes the original Core test stricter. The three folders hold 51 Rust files: 48 in the closure and 3 excluded, with no symbolic links. No test hashes anything, calls `source_closure`, or does any proving or native work.

**Check classification.** The command in `H28-PROPOSED-CHECKS.json` stays as it is (`cargo … test -p mori-ledger --test source_closure_coverage`). I classify it as an approved ordinary check under the existing ledger pure-helper class, pinned to the new test hash. It should report exactly 6 tests; any other count or any failure blocks it.

**Acceptance.** Writing this source is not security acceptance. F1 closes only after a fresh run of that check and a fresh, full integrated Opus audit of the changed candidate. The earlier 3-test capture stays as a historical record for the old hash. F2, F4 and oracle `76075b63…` are unchanged. The U3 spec lines (UNI-006/007/008) were used only to locate them and don't change anything here.
