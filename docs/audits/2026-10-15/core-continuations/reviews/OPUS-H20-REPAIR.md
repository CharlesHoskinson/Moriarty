# OPUS-H20-REPAIR: T3/H20 reverse-direction decoder test (source author proposal)

- **Author and classifier:** Claude Opus 5.5 (`claude-opus-5-5`), high effort. I am the **source author** of this proposal, not an independent auditor. Nothing here is an independent approval.
- **Candidate:** WT `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`. It was clean at the start and is still clean (`git status --short` is empty).
- **Startup:** I read `AGENTS.md` and the develop skill (`plugins/moriarty-dev/skills/develop/SKILL.md`). Guarded `status --json` shows only the historical SP01 queue, with `pendingTransactions: []`. Native authority stays 0 and no counter was touched.
- **Nothing executed:** no build, test, fmt, Cargo, SDK, model, k, VM, prove, native verify, ledger or network use. No key, PK, SRS or proving-cache path was read or stat'ed. I used only public source reads, sha256sum, diff and file writes under R.
- **Writes:** only `R/opus-h20-repair/` and this file. No repository change.
- **Scope:** T3/H20 only. Scope A handles T4/H19 separately, and it is not duplicated here.

## Inputs read

- `R/OPUS-SUCCESSOR-B-COMPLETION.md`, all 172 lines, including T3 (`:128-132`) and the H20 row (`:123`).
- `crates/mori-backend/tests/stage_relation.rs`, all 453 lines. The pre-sha `e6b35eeb…6926` equals the pin recorded in that report.
- `crates/mori-backend/src/lib.rs:30-32` (the `/3` constants) and `:223-311`: `Artifact`, `ArtifactFields`, `decode_version` and the three `decode_*_version` wrappers.
- `crates/mori-backend/src/stage/artifact.rs:1-230`: the `/4` constants, `StagedArtifact`, its derived `Serialize` and its `validate`.
- `crates/mori-backend/src/stage/lower.rs:1-130` (`compile_staged`) and `lib.rs:1190-1228` (the tail of `lower_action_segment`).
- `Cargo.toml:50`, `crates/mori-backend/Cargo.toml` and the `serde_json` entry in `Cargo.lock` (`:3459-3470`).

## Decoder contract, as inspected

- `ARTIFACT_FORMAT = "moriarty-zkir-artifact/3"`, `PUBLIC_ABI_VERSION = "moriarty-public-abi/3"` and `RELATION_VERSION = "moriarty-zkir-relation/3"`.
- `Artifact` is `#[serde(try_from = "ArtifactFields")]`. `ArtifactFields` is a derived `deny_unknown_fields` struct whose `format`, `abi` and `relation` fields use `deserialize_with = decode_version(.., expected)`.
- `decode_version` returns `serde::de::Error::custom("Expected {expected}; recompile for this backend revision.")`. There is no `mori::backend::` prefix, because it is raised inside serde, before `TryFrom`.
- The derived visitor fails at the **first map key it visits** that refuses. That makes the reported diagnostic depend on key order.

## Finding: the prescribed `to_value`/`from_value` form cannot name `ARTIFACT_FORMAT`

- The T3 fix text says `from_value::<Artifact>(to_value(compile(MIXED)))` "must be `Err` and name `ARTIFACT_FORMAT`". **This would fail as written.**
- In `Cargo.lock`, `serde_json 1.0.151` depends only on `itoa`, `memchr`, `serde`, `serde_core` and `zmij`. There is no `indexmap`, so `preserve_order` is enabled nowhere in the workspace. `serde_json::Map` is therefore a `BTreeMap`, and `from_value` visits keys in sorted order.
- Sorted `/4` keys begin `abi`, `authentication`, `authorization_mode`, `circuits`, `consumption`, `format`, … So the `/3` decoder refuses at `abi` with `"Expected moriarty-public-abi/3; recompile for this backend revision."`. `format` is never reached.
- An assertion on the format message through `from_value` would fail. A loose `is_err()` would pass while hiding which predicate fired. Forging the value to reach `format` would also not work: every sorted path to it passes through unknown `/4` keys (`authentication`, `circuits`, `consumption`). Removing those keys would mean it is no longer a serialized `/4` artifact.
- Inference: **bytes** are the real legacy decode input, for example a published `artifact.json`. Bytes keep the derived `Serialize` declaration order, where `format` comes first (`artifact.rs:113-114`). So the precise `ARTIFACT_FORMAT` diagnostic is reachable through `from_slice`. That stream form is the targeted format test.

## Proposal: one appended test, the 9 existing tests unchanged

`legacy_decoder_refuses_staged_artifacts`, appended after line 453. It asserts the following, in order:

1. **Independent hard-coded `/3` strings.** `ARTIFACT_FORMAT == "moriarty-zkir-artifact/3"` and `PUBLIC_ABI_VERSION == "moriarty-public-abi/3"`.
2. **Positive control.** The `compile(MIXED)` value decodes as a `/4` `StagedArtifact`, and its `format` and `abi` are the literal `/4` strings. So the refusal is not caused by a broken input.
3. **Byte path (targeted format predicate).**
   - The `to_vec_pretty` bytes start with `{\n  "format": `, so `format` is the first key the decoder visits.
   - `from_slice::<mori_backend::Artifact>` errs, and `is_data()` holds.
   - After its own ` at line L column C` suffix is stripped, the `Display` equals **exactly** `"Expected moriarty-zkir-artifact/3; recompile for this backend revision."`.
   - Nothing can mask this: it is the first key, and the exact equality excludes unknown-field, Target or `TryFrom` messages.
4. **Value path (the form named in T3).**
   - `from_value::<mori_backend::Artifact>` errs, and `is_data()` holds.
   - The `Display` equals **exactly** `"Expected moriarty-public-abi/3; recompile for this backend revision."`. A Value error has no position, so there is no suffix.
   - This pins the predicate that actually fired under the locked sorted map order, so a different error cannot be accepted silently.

Imports: the test uses `ARTIFACT_FORMAT` and `PUBLIC_ABI_VERSION`, which are already imported at `:194-197`, and the full path `mori_backend::Artifact`. No new `use` lines. No production change. No SDK or native call: the only compile call is the existing `compile(MIXED)` helper.

### Payload and identity

| Item | sha256 |
|---|---|
| Pre: WT `crates/mori-backend/tests/stage_relation.rs` (453 lines, 9 `#[test]`) | `e6b35eebba3d6729230abbdf2c5c3f60f17340ea6f985ac33f390bd9b1cf6926` |
| Post: `R/opus-h20-repair/stage_relation.rs` (483 lines, 10 `#[test]`), full proposed file | `3c5d011149d157feb5b7b99eb81661522cc9ab2b2bb1b40079802d96d3119a8b` |
| First 453 lines of the post file (equal to the pre hash, so the existing 9 tests are byte-identical) | `e6b35eebba3d6729230abbdf2c5c3f60f17340ea6f985ac33f390bd9b1cf6926` |
| `R/opus-h20-repair/stage_relation.patch` (unified, 36 lines, one hunk `@@ -451,3 +451,33 @@`, additions only) | `20f9e3f77fc0ee7dd913838dee45b0ecf5aa1ca12997c07bb4f8532272d5649f` |
| `R/opus-h20-repair/append.rs.txt` (the exact appended 30 lines) | `b03a650f0537f8a22d06374afbcbddb860b4727640244e334ba579b366d5354b` |

Apply the patch with `git apply` against `e6b35eeb…`, or copy the post file. I applied neither.

### Unexecuted predictions (authorship claims, not results)

- Compiles: **unverified**. The serde_json APIs relied on are `Value: PartialEq<&str>`, `Error::{is_data, line, column}` and `from_slice`/`from_value`. `[u8]::starts_with` takes the byte-string literal through unsized coercion.
- Byte-path position: serde_json's `deserialize_struct` calls `fix_position`, so the error has `line > 0` and the suffix form holds. I deliberately do not predict the exact line and column, because whether `end_map` runs before position fixing depends on the serde_json version.
- rustfmt was not run, and the appended lines may not be in rustfmt form. Every new line is under 100 columns.
- If `preserve_order` is ever enabled, the Value-path assertion would fail loudly, because the decoder would refuse at `format` instead. That is intended: it flags the change rather than hiding it.

## Classification of the proposed check (no execution authorized by this report)

Command: `cargo +1.99.0 --offline --locked test -p mori-backend --test stage_relation -j2 -- --test-threads=1`, under the existing `flock` 120 s, `timeout` 300 s, scrubbed environment and shared ordinary object cache.

- **Anticipated:** 10 tests, the 9 existing plus `legacy_decoder_refuses_staged_artifacts`, all expected to pass. No ignored tests are expected in this binary.
- **Code reached by the new test (inspected):**
  - `compile_staged`, which also covers the signed-Target guard, `mori_core::stage::load`, `Target::validate`, `liabilities::registry`, `frame::layouts` and `lower_action_segment`. That function builds the IR in memory and round-trips it through upstream `IrSource` serde (`IrSource::load` on in-memory bytes, `lib.rs:1208-1214`).
  - Hex and sha context digests, then `StagedArtifact::validate`.
  - serde_json serialization, the `/4` decoder, and `ArtifactFields` decoding up to the first version key.
  - No filesystem effect: `_out` is unused (`lower.rs:22`). There is no `parameters::OfflineParameters::load`, which is reached only from `proving.rs`, and no prove, verify, SDK check/model, VM, network or env reads.
- **Ordinary pure static path**, the same reach as the already-classified existing tests in this binary, plus serde decoding. It needs no SDK model, k, proving, key, PK or SRS.
- **Residual retained:** linking compiles the `midnight-zkir-v3` and `midnight-transient-crypto` dependency graph from the shared ordinary object cache. Any compile-time embedding of public verifier parameters in those crates is not inspected here and stays a residual. I did not open the registry, proving cache or any private or SRS material to resolve it.
- **Status:** classification proposal only. Root must inspect this payload before any execution. This report authorizes nothing.

## Not done or not claimed

- Not applied, built, formatted or tested. No result exists.
- No independent review. A fresh non-author Opus 5.5 audit of these exact bytes is still required before acceptance.
- The T3 fix text needs a root decision. As written ("from_value … name `ARTIFACT_FORMAT`") it is not satisfiable under the locked dependency graph. This proposal meets its intent with the byte path, and pins the Value path to its actual ABI diagnostic.
- H19/T4, docs (D-O1), CLI and ledger are outside this scope and untouched.
