# Verdict: **APPROVE** (source-only, exact four-hunk scope, UNRUN)

Reviewer: claude-opus-5-5, HIGH effort, fresh nonauthor. No tools, files, commands or execution were used. The Gemini/fallback labels in the packet are historical metadata only.

## Repair 1: E0603, `literal_witness_native_tests.rs`

**Diagnostic.** BODY 07 reports `error[E0603]: struct import LiteralLane is private` at `:9:43`. I counted the preimage line 9 by hand: 5 leading spaces, then `ImageValues, ` (13 bytes), then `LiteralConstraintCheck, ` (24 bytes), which places `LiteralLane` at column 43. The anchor matches.

**Root cause.** `literal_bridge.rs:9` is `use super::*;`, a private glob. Its explicit `pub use super::literal_host::{BINDING_VERSION, CapacityRecord, ImageValues, LiteralLayout, …}` does not list `LiteralLane`. So `literal_bridge::LiteralLane` cannot be named from the sibling test module.

**Replacement path.** The new import resolves to the same type:
- `lib.rs`: `pub mod sherlock_machine;`
- `sherlock_machine/mod.rs`: `pub mod literal_host;`
- `literal_host.rs:82`: `pub struct LiteralLane { pub name, pub bits, pub boolean, pub maximum }`

This is the same definition that `literal_step.rs` re-exports via `pub use super::literal_host::{…LiteralLane…}` and that `mod.rs` re-exports via `pub use literal_step::{…LiteralLane…}`. Both are named in the compiler's note chain. The only uses are `column_index(&[LiteralLane], …)` and `image_lane_total(&[LiteralLane], …)`, which read only the `.name` field and `.len()`. Type identity is unchanged.

**Other imports.** Every remaining `literal_bridge::{…}` import is a `pub fn`, `pub enum`, `pub struct` or `pub use` there:
- `check_literal_statement`, `check_literal_transition`, `local_max_statement_lanes`, `CircuitMode`, `LiteralConstraintCheck`
- the `literal_host` re-export list

`literal_step` is a private module, but the test is a descendant of `sherlock_machine`, so that unchanged import stays legal. The diagnostic did not flag it either.

**Fix placement.** Fixing this on the test side is the correct choice. The alternative would be adding `LiteralLane` to `literal_bridge`'s `pub use`. That would mutate `literal_bridge.rs`, which is one of the nine `include_str!` sources hashed into `relation_hash` in `LiteralLayout::new`. That change would also shift `context` and the emitted ZKIR constants. The patch touches no production or relation-bound byte.

## Repair 2: E0308, `literal_complete_bridge_normal_tests.rs`

**Diagnostic.** BODY 07 reports `expected u128, found UInt` at `:44:59`, with a note pointing to `reference_types.rs:22  UInt(u128)`. Hand count of preimage line 44: 8 spaces, then `entry: `, then `Arc::new(`, then `Expr::Literal(`, which puts `SherlockValue::UInt` at column 39 (19 bytes, matching the underline) and the argument at column 59. The anchor matches.

**Types involved.**
- `SherlockValue` is `mori_core::sherlock::Value` (an explicit alias, kept), whose variant is `UInt(u128)`.
- The removed `mori_core::UInt` is the distinct newtype `ir.rs: pub struct UInt(pub u128)`.

`SherlockValue::UInt(val)` with `val: u128` is the exact constructor. The specimen value (42 at both call sites) is unchanged.

**Removed import.** `use mori_core::UInt;` is removed. In the afterimage, `UInt` survives only in the variant path, the fn name and a doc string. That agrees with BODY 14's "use count 2". Removing it avoids an `unused_imports` warning and causes no shadowing. `mod.rs` imports only `mori_core::{Type, Value}`, and the `Value as SherlockValue` alias still prevents a clash through `super::*`.

## Byte and hunk verification (by hand)

| Check | Result |
|---|---|
| File 1 delta: −13 (`LiteralLane, `) +56 (new `use` line + LF) | +43 = 20479→20522 ✓ |
| File 2 delta: −21 (`use mori_core::UInt;\n`) −6 (`UInt(`,`)`) | −27 = 15126→15099 ✓ |
| Offset 496 (preimage, start of `ImageValues`) | ✓ |
| Offset 691 (sequential, post-hunk-1 line 13 start) | ✓ |
| Offset 1316 (line 21 start; required for the empty-new inverse) | ✓ |
| Offset 1980 (post-hunk-3, line 43, col 39) | ✓ |
| Unified `@@ -6,10 +6,11 @@`, `-18,7 +18,6`, `-41,7 +40,7` | consistent ✓ |
| Afterimages ≡ preimage + four literal old→new pairs; nothing else differs | ✓ |

BODY 04 has three `@@` hunks because file 1's two literal pairs are adjacent and merged. That is consistent with the four literal pairs in BODY 05.

I did not and cannot recompute SHA-256 values without tools. The hashes are accepted only as packet pins.

## Unchanged-scope confirmation

The following are untouched:
- All test bodies, fixtures, capacities, fuel and stack inputs, expected constants (`EXPECTED_*`, `cc6887…7f81`), assertions, corruption cases and env-gated output writes.
- Every production module, guard, `include_str!` relation binding, check/prove API and visibility.

No test is shortened or mocked. Neither test file is in `relation_hash`'s source list. Test 2 asserts only `assert_ne!` relative to A102, so it pins no absolute hash.

## Non-blocking note

`literal_witness_native_tests.rs:9` keeps a pre-existing 5-space indent (` ImageValues…`). This is cosmetic and correctly left out of scope.

## Retained limitations

1. **UNRUN.** Source review cannot prove the crate now compiles. rustc did complete name resolution and type checking for the crate, since E0603 and E0308 were reported together and no unresolved names appeared. However, MIR borrow-check and later lint phases may have been skipped because of these errors. New diagnostics in these or other files remain possible on rebuild.
2. `literal_complete_bridge.rs`, `literal_witness.rs` and `ProducedLiteralTransition` were not supplied. The unchanged uses of them are trusted only to the extent that no error was reported against them.
3. This verdict grants no native, Cargo, SDK, compiler, test, SRS, keygen, prove or build permission or resource. SDK outputs stay unaccepted. Frozen native Wf935 and nominal-parser WW80ba0fc8 are unaffected. Applying the patch, preserving and cleaning up P7, and freezing the source cut remain Root's separate actions.