# Opus public security expectations: H2 kind, H3 payer, H4 rehashed map tampering

Frozen **2026-10-15T02:25:00Z**, before any regression code was written and before any execution. Author: Opus 5.5 (high), sole security owner. Base `eb2577d6c67684b950288d769ea2fcb91e731bf1`. **Specified-only.** Nothing has been compiled, formatted or run. This is not an approval and not a command classification. Native authority stays at zero.

Machine-readable twin: [OPUS-PUBLIC-EXPECTATIONS.json](OPUS-PUBLIC-EXPECTATIONS.json). It records the SHA-256 of every input I derived these expectations from.

## How the expectations were derived

I derived them by reading source, not from product output. Main facts:

- Transfer and Fee lower through one arm. The kind flag and `from` reach only `record()` and `payments`, and both are used only on the signed path (`compiler.rs:45-56`, `:553-577`, `:762-771`). `claim` keys on token and recipient (`:316-388`).
- The IR prologue is `Dup/Size/Popeq(3)` (`ops.rs:25-34`). It commits to no source, program or ABI hash.
- `debit_sources` is the sorted, deduplicated set of `from` roles. It carries no statement index (`relation/mod.rs:375-394`).
- Reopen checks the marker identity, leaf digests and leaf bytes before it runs the fresh compile. After those checks, byte equality with a fresh compile is the only remaining guard, and it refuses with `static-compile: fresh-source-mismatch` (`static_compile.rs:513-619`).
- The marker identity is `codec::object_hash` (sha256 of `mori_core::intent::canonical`) over the marker with `identity` removed. The marker file is serde_json pretty output of a `Value`.

Variants come from the frozen five-step `PROVENANCE_SOURCE` by exact edits. Each edit pattern must occur exactly once. Expression ids 0–6 do not change. Every variant is lawful: its bindings are recomputed by the existing `bindings()` helper, and the roles alice, bob and treasury are all bound.

## H2 — `provenance_security_h2_kind_swap_changes_abi_not_ir`

Variant K swaps the kinds of steps 0 and 1: `fee(PAY, "alice", "treasury", 2);` then `transfer(PAY, "alice", "treasury", amount);`.

Expected:
- Both exports are accepted.
- K's source, program and ABI hashes differ from B's.
- **The IR leaf bytes are equal.**
- With `source_hash`, `program_hash` and `abi` removed, the two manifests are equal. That makes `ir_hash`, `groups`, `debit_sources` and `statement_emissions` equal.
- K's ABI equals B's ABI with only `program_hash` and the two kinds changed.
- K's `debit_sources` is still [alice, bob].
- The marker identities differ.
- Each bundle reopens under its own inputs.
- B's bundle under K's inputs is refused with `static-compile: marker-identity`.

Reading: the kind is not bound by the IR or by the map. Only the source commitment binds it.

## H3 — `provenance_security_h3_payer_change_changes_debit_metadata_not_ir`

- **P1** changes step 4's payer from bob to alice. Expected `debit_sources` is [alice], so the payer set changes. The manifest otherwise differs only in the hashes and ABI, and the ABI differs only at `statements[4].from`.
- **P2** changes step 0's payer from alice to bob. Expected `debit_sources` is [alice, bob], **byte-identical to B**. The ABI differs only at `statements[0].from`.

In both variants, the IR bytes and `statement_emissions` equal B's. The three marker identities are pairwise distinct, and P1 and P2 reopen.

Reading: P2 shows that `debit_sources` cannot attribute payers to statements. Per-statement payer attribution exists only in the committed ABI. Nothing authenticates or funds a debit.

## H4 — `provenance_security_h4_rehashed_map_tampering_refuses_fresh_source_mismatch`

Starting from one baseline export, apply each mutation alone. After each one, recompute the manifest leaf sha256 and byte count, the action's `manifest_hash` and the marker identity, using the public `codec::{hash, object_hash}`:

| Id | Mutation |
|---|---|
| M1 | Entries 0 and 1 exchange instruction and group ranges (to 41/75/16/29 and 7/41/3/16) |
| M2 | Entry 2's field {group 31, offset 3} becomes offset 2 (still in width 4) |
| M3 | Entry 2's `dynamic_fields` becomes `[]` |
| M4 | `statement_emissions` member removed |
| M5 | `format` relabelled `/2` → `/1` |
| M6 | M4 + M5 (downgrade to a /1-shaped manifest) |

Harness preconditions:
- Resealing the unmodified manifest reproduces the original identity and the original marker bytes exactly.
- Every mutated manifest equals the original with only the stated JSON change applied.

Expected:
- Every mutation is refused with `static-compile: fresh-source-mismatch`.
- Reopen writes nothing.
- Restoring the original bytes reopens successfully.

Reading: the map is never deserialized, so fresh-recompile equality is the only defence against a rehashed map. H4 pins that single dependency.

## Non-claims and scope

- These tests cover no runtime values, guard activity, funding, custody, authorization, ledger, proof or native correspondence.
- IR equality is a static byte observation. It does not show that Transfer and Fee settle the same way.
- Sol's normal test, including the optional `MORI_PROVENANCE_BASELINE_BINARY` branch, is unchanged.
- H5 stays review-only, and no ordinary unsigned call is added.
- No compiler injection seam is added.
- `relation::emission_tests` is unchanged.
- No U0–U7, M1–M17, MC, SP or G exit closes.

If an observed result disagrees, stop and report. Do not tune these expectations.
