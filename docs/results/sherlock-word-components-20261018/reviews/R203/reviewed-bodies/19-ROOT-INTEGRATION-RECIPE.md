# Root integration recipe

Status: SOURCE_ONLY / UNRUN. Only Root may select, review/audit, copy and register
these afterimages. No W or Git mutation was performed by the author.

1. Review the exact final manifest and declaration contract, obtain required fresh
   non-author audit and independently owned expectations under current routing.
2. Copy the two Rust afterimages to their matching W paths and the documentation
   afterimage to `docs/sherlock/typed-word-arithmetic.md`.
3. Add exactly this module registration to W `crates/mori-core/src/sherlock/mod.rs`:

```rust
// SDK-free typed host components; no circuit/native correspondence claim.
pub mod word_arithmetic;
```

The component includes `word_arithmetic_tests.rs` as its private cfg(test) child;
no additional test module registration is needed. Existing reference/byte auxiliary
registration remains unchanged. A module-doc navigation link is optional.

For a selected SDK-free ordinary source check, Root can compile the component
file as a standalone test crate with edition2021 and run that binary, then run the
separately owned independent oracle harness against the identical selected source.
Those are planned commands, not observed results. Cargo/native/provider operations
were not performed in this lane. Root determines the exact selected rustc command
and scoped evidence receipt after required review.
