# OPUS-V1-WIRE: `/1` frozen decode wire (clarification §1.4)

Status: SOURCE_WRITTEN, not executed. No build, test, fmt or `git apply` was run.

- Patch: `OPUS-V1-WIRE.patch`, sha256 `80c19e9f83b9b99c91da23d7942921d7750e773ca4b30e1166adb29e308bcb47`
- Size: 2 files; 206 lines added, 5 removed; 6 hunks (5 in `ir.rs`, 1 in `runtime.rs`).
- Owned test written in the worktree: `crates/mori-core/tests/compat_v1.rs`. It needs the patch applied first. Its fixtures are inline literals; no fixture files were added.

## Shared hunks for the integrator to apply verbatim

1. **`crates/mori-core/src/ir.rs`**
   - `Statement`, `Action`, `Program`, `Step` and `Execution` drop `derive(Deserialize)`. Their `Serialize` derive and `#[serde]` attributes are unchanged, so non-Await output bytes and `program_hash` should not change.
   - The shared `Statement::Await` serializes only. It exists for in-memory and source `/2` use; there is no `/2` Program JSON import.
   - New inline `pub mod v1` holds the five baseline types, copied verbatim from `git show fe01bc8b:crates/mori-core/src/ir.rs`, and has no Await.
     - Type names are kept on purpose. serde's `expecting` text ("struct Program", "internally tagged enum Statement") and the unknown-variant list then stay identical to baseline.
   - Infallible `From<v1::T>` conversions are added for each of the five types.
   - The `decode_via_v1!` macro gives each shared type a manual `Deserialize` impl: `v1::T::deserialize(..).map(From::from)`.
     - This routes every direct, nested and generic decode through the wire automatically. That covers CLI `from_value::<Program>`, `replay`'s `Execution`/`Step`, and backend bundles with nested `Program` or `Execution`, so decode sites don't have to be listed and edited one by one (U10).
     - `StageSegment` already serializes only (`stage/mod.rs`), so nothing decodes Await-bearing history.
2. **`crates/mori-core/src/runtime.rs` `validate`**
   - Adds an `Await` arm to the existing statement match: `failure("profile", "Await requires moriarty-core/2.")`. This is the same identity as `stage::require_core1`.
   - It is needed because the match is exhaustive, and it enforces C7 for in-memory `/1`-labelled programs.
   - No other validate code or message changes.

## Applicability
- The patch is a `diff -u` taken against byte copies of the current worktree `ir.rs` and `runtime.rs`, with the foundation and helper hunks already in. It should apply if those files haven't changed since.
- `git apply --check` was attempted but needed permission approval and did **not** run, so applicability is unconfirmed.
- No ledger, CLI or backend hunks.

## compat_v1.rs (unexecuted)
- **H23:** checks three cases against the literal baseline message text, both directly and through `json::parse` and nested `Statement`/`Step`/`Execution` decoding:
  - a well-formed Await;
  - a malformed Await (bad guard plus an extra field);
  - an unknown kind `Foo`.
- **Round trip:** decoding then re-encoding a non-Await Program and Execution gives the same JSON value.
- **C7:** an in-memory `/1` Await is refused by `validate` with `mori::core::profile`.
- The tests use `from_value`, so error text carries no line or column suffix.

## Open (not claimed)
- The full U/M/MC/SP/G set and H1–H28 are not covered here.
- No test results exist. Whether the patch compiles has not been checked.
