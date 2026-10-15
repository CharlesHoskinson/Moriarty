I've written the `/1` wire patch, the compat test and the detail report. Nothing has been built, tested or applied, and the applicability check didn't run.

**SOURCE_WRITTEN** — `R/OPUS-V1-WIRE.patch`, sha256 `80c19e9f83b9b99c91da23d7942921d7750e773ca4b30e1166adb29e308bcb47`.

**Patch size:** 2 files, 206 lines added and 5 removed, in 6 hunks (5 in `ir.rs`, 1 in `runtime.rs`).

**What the patch does:**
- **`ir.rs`:**
  - `Statement`, `Action`, `Program`, `Step` and `Execution` no longer derive `Deserialize`. Their `Serialize` derive and serde attributes are unchanged, so output for everything except `Await` stays the same and `program_hash` shouldn't move. `Await` is serialize-only, for in-memory and source `/2` use.
  - A new inline `pub mod v1` holds the five baseline types, copied from `git show fe01bc8b`. They keep the same names, so serde's error text and the unknown-variant list stay identical to baseline.
  - Each of the five gets a `From` conversion and a hand-written `Deserialize` (via the `decode_via_v1!` macro) that goes through `v1`. Every direct, nested or generic decode uses the wire, so decode sites don't need editing one by one.
- **`runtime.rs` `validate`:** adds an `Await` arm returning `profile` / "Await requires moriarty-core/2.", the same error as `require_core1`. No other validate code or messages change.
- No ledger, CLI or backend hunks.

**Applicability:** I built the patch with `diff -u` against copies of the current worktree `ir.rs` and `runtime.rs`, with the foundation and helper hunks already in. It should apply cleanly if those files haven't changed since. `git apply --check` was blocked pending your approval, so this is unconfirmed. No shared file was changed.

**Test:** `crates/mori-core/tests/compat_v1.rs`, with inline fixtures and no new fixture files. It only works once the patch is applied.
- **H23:** a well-formed Await, a malformed Await and an unknown `Foo` each give the exact baseline error text. This is checked on a direct Program decode, through `json::parse`, and on nested Statement, Step and Execution decodes.
- **Round trip:** decoding and re-encoding a Program and an Execution without Await gives the same JSON.
- **C7:** an in-memory `/1` program containing Await is refused by `validate` with `mori::core::profile`.

**Detail report:** `R/OPUS-V1-WIRE-DETAIL.md`.

The full U/M/MC/SP/G set and H1–H28 are still open.
