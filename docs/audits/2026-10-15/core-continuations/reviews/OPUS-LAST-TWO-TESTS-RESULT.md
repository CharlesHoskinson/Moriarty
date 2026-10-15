I've written the two proposed tests and the two JSON fixtures. Nothing has been run: no build, no tests, no tools that generate output. I wrote only under `R/opus-last-two-tests/` and `R/OPUS-LAST-TWO-TESTS.md`. The repo is still clean at `dd4e4298`, and this is author/classifier work, not approval. I finished in about 315 s.

## Payload

| Path | Pre sha256 | Post sha256 |
|---|---|---|
| `crates/mori/tests/core_continuations.rs` | `816a4320…` | `7a93fd08…f277c6d9` (+18 lines) |
| `crates/mori-core/tests/compat_v1.rs` | `c1f75cf8…` | `1a8d5de4…2b119559` (+22 lines) |
| `…/fixtures/compat_v1_identifiers/program.json` | absent | `27f77197…3f594699` (881 B) |
| `…/fixtures/compat_v1_identifiers/execution.json` | absent | `88315015…06367d7d` (2092 B) |

- **Additions only.** The original 11 CLI tests and 4 compat tests are unchanged byte for byte (checked with `cmp -n`), and no imports were added. Both `.additions.patch` files contain only added lines.
- **Pins unchanged:** the oracle `76075b63…`, `Cargo.lock`, `compile.rs`, `lower.rs` and `json.rs`.

## F-1: H18, `mori compile` with a signed Target

- **Setup:** `delivery.mori` (`/2`) with `SIGNED_TARGET`, a literal copy of the public `target_canonical_utf8`. I embedded it as a literal rather than reading `golden-vector.json`, because that file also contains `seed` and `private_witness` fields. A grep confirmed the literal matches the golden target exactly.
- **Asserts:**
  - exit 1 and empty stdout;
  - the exact `mori::command: mori::backend::unsupported: Staged moriarty/2 relations are unsigned; a signed Target is refused.` diagnostic;
  - `!out.exists()`;
  - exactly 2 files left in the temp directory.
- **Guard order:** `compile.rs:19-22` reads the source, checks the language, and reads the Target the same way `Target::from_json` does. `lower.rs:23` then refuses the signed Target before load, validation, lowering, or the `create_dir` at `:25`.

## F-2: H15, `await` and `when` as `/1` identifiers

- **Asserts:** the `json::export` output of `parse(src)` and of `simulate(…, "run", default scenario)`, each plus one `\n` (matching the CLI's `println!`), must equal the two fixtures byte for byte. Decoded equality is checked as well.
- **Freeze:** I typed both fixtures by hand from the `fe01bc8b` `ir.rs`, `parser.rs`, `runtime.rs` and `json.rs` definitions. I froze them at 06:43:12Z, before writing any assertions (`test_source_exists_at_freeze: false`); details are in `ORACLE-FREEZE-NOTE.md`.
- **Not done:** a Python check of the fixture formatting was declined by the permission prompt and is recorded as not done. So the only check of the hand-typed formatting will be the test run itself.

## Classification (conditional ordinary, assuming the existing flock/timeout/envscrub wrapper and before/after pins)

1. **`cargo +1.99.0 --offline --locked test -p mori --test core_continuations -j2 -- --test-threads=1`** → ORDINARY, 12 tests expected.
   - The old 11 keep their previous classifications. The `--trace` and `compile-call` cases remain the source-profile-first exceptions from OPUS-FINAL-A-REPAIR, not broadened to native commands in general.
   - The new test only reads the source and a public Target, then refuses before anything is written.
   - The SDK compile-time public-verifier embedding is still a residual.
2. **`cargo +1.99.0 --offline --locked test -p mori-core --test compat_v1 -j2 -- --test-threads=1`** → ORDINARY, 5 tests expected.
   - These are pure legacy serde, parser and interpreter tests with no I/O and no SDK at runtime.
   - The `midnight-*` dev-dependencies still compile into the test target, as in the existing compat capture, but are unused.

If both commands pass later, the new 12 + 5 replace the old 11 + 4, for 88 distinct tests (reruns not counted twice). Captures and a fresh non-author review are still required.
