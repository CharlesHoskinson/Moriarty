# OPUS legacy refusals — SOURCE_WRITTEN (not approval)

Source-only. Nothing applied, compiled, formatted or tested. No commits.

## Written
- `OPUS-LEGACY-REFUSALS.patch`: A9 Await arms + `require_core1` gates.
  - ledger `layout.rs`: new `core1_refusal()` / `require_core1()` helpers (identity `mori::ledger::profile: core1-required`); gate at top of `admit_bindings_inner` (covers `admit_bindings` and the `MappedRoles` caller); explicit Await arm before the inner `_ => {}`.
  - ledger `ops.rs` expression match, `relation/compiler.rs` both `statement()` matches, `signed/mod.rs` label scan: explicit Await refusal arms. All enclosing fns already return `Result`; no helper signature change needed.
  - ledger entry gates: `relation::compile_source_static_roles`, `static_compile::compile`, `SignedStaticBindings::from_source`.
  - backend `proving.rs::financial_values`: gate plus explicit Await arm before `_ => None`, reusing `crate::stage::core1_refusal` from Sol's backend stage module (no duplication of `OPUS-BACKEND-SHARED.patch`). The second match (~L982) already errors on non-duty statements; left unchanged.
- `OPUS-SOURCE-CLOSURE.patch`: A2 hunk, exact 8 lex-ordered stage files appended after `diagnostics.rs`.

## Pending / not done (honest)
- Hunk line numbers are from the current uncommitted worktree; `@@` offsets approximate, context lines exact.
- Not verified that `relation/compiler.rs` has `layout` in scope as `layout::` (it imports `crate::{.., layout, ..}` at L6 — yes) and that `ops.rs` match is in a `Result`-returning fn (uses `?` with ledger `Error` — yes).
- NOT covered: `local_deployment` / signed preparation / CLI `CompileCall`/`run_static` explicit gates (they reach `admit_bindings_inner` / `static_compile::compile`, which are now gated, but no separate entry hunk was written).
- NOT covered: CLI `inspect.rs` `statement_rule` / `expression_constraints` arms and unsupported `--target` refusal.
- NOT written: `crates/mori-ledger/tests/source_closure_coverage.rs` (H28).
- Test families pending: ledger Await-refusal identity pins, backend `financial_values` refusal, H28 closure coverage, CLI inspect/target refusals.
