SOURCE_WRITTEN. I've written the patch files but not applied them to the worktree. Nothing was compiled, formatted or tested, and nothing was committed or pushed. This is not execution approval.

**Files written (in R):**
- **`OPUS-SOURCE-CLOSURE.patch`**: the A2 change to `crates/mori-ledger/src/proving.rs`. It adds a comma after the `diagnostics.rs` entry, then appends the 8 core stage files in alphabetical order: admit, cost, decode, digest, mod, plan, profile, validate. The existing order is kept, and no backend or CLI files are added to the closure.
- **`OPUS-LEGACY-REFUSALS.patch`**: the A9 Await arms and `/1` entry checks.
  - **Ledger `layout.rs`:** two new `pub(crate)` helpers, `core1_refusal()` and `require_core1()`. The refusal is `mori::ledger::profile: core1-required`. `require_core1` now runs at the top of `admit_bindings_inner`, before `validate`, so both callers are covered. There is also an explicit Await arm before the inner `_ => {}`.
  - **Ledger Await arms:** `ops.rs`, `relation/compiler.rs` (both matches in `statement()`) and the `signed/mod.rs` label scan each return the refusal. All the surrounding functions already return `Result`, so no helper's signature changed.
  - **Ledger entry checks:** `relation::compile_source_static_roles`, `static_compile::compile` and `SignedStaticBindings::from_source`.
  - **Backend `proving.rs::financial_values`:** a `require_core1` check plus an Await arm before `_ => None`. Both reuse Sol's `crate::stage::core1_refusal`, so nothing in `OPUS-BACKEND-SHARED.patch` is duplicated. The second statement match there (around line 982) already returns an error for any non-duty statement, so I left it alone.
- **`OPUS-LEGACY-REFUSALS-DETAIL.md`**: summary plus the pending items below.

**Not done:**
- **Exact line numbers:** the context lines in the patches match the current uncommitted files, but the `@@` line numbers are approximate. Apply by context.
- **Other ledger entry points:** `local_deployment`, signed preparation and the CLI `CompileCall`/`run_static` path have no check of their own. They do reach `admit_bindings_inner` or `static_compile::compile`, which are now checked.
- **CLI `inspect.rs`:** the `statement_rule` and `expression_constraints` arms are still missing, as is the refusal for an unsupported `--target`. The CLI won't compile until those arms exist.
- **Closure coverage test:** `crates/mori-ledger/tests/source_closure_coverage.rs` (H28) was not written.
- **Tests still needed:** checks that pin the ledger and backend Await refusal errors, H28 closure coverage, and the CLI inspect and target refusals.
