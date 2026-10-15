# OPUS-LAST-TWO-TESTS: F-1 / F-2 test source and classification

**Role.** Claude Opus 5.5 (`claude-opus-5-5`, high). I am the security-test **source author** and the ordinary-command classifier. I am **not** an independent reviewer, and nothing here is approval.

**Candidate.** `dd4e4298322666c140c74343a7827000f578c848`. The worktree was clean before and after, and no repository file changed.

**Method.** Authority 0, and the counters are unchanged. Only static git and source reads were used: no build, test, format, Cargo, SDK, product execution, or key/PK/SRS/cache read or stat. Every new test is **unexecuted** and is not added to the 86.

## Payload (`R/opus-last-two-tests/`)

| Proposed repo path | Pre sha256 (dd4e4298) | Post sha256 |
|---|---|---|
| `crates/mori/tests/core_continuations.rs` | `816a4320…a0f994670ae` (16301 B) | `7a93fd084b5df5412a80531cf78d9b1c9ea7f1c9031f46e2e7df1328f277c6d9` |
| `crates/mori-core/tests/compat_v1.rs` | `c1f75cf8…c7665ad` (4838 B) | `1a8d5de45830c8bc5b7ed229bb269a76b20b404009a224e7da4b18122b119559` |
| `crates/mori-core/tests/fixtures/compat_v1_identifiers/program.json` | absent | `27f7719725a0f2c0cf03efdb62586b11f328cc606309db6e245bd88c3f594699` (881 B) |
| `crates/mori-core/tests/fixtures/compat_v1_identifiers/execution.json` | absent | `88315015f9b0eb7a6d10378d9eac4f48d77db9c56dd750f471daf76006367d7d` (2092 B) |

- The exact pre hash of `compat_v1.rs` is `c1f75cf8266abe803d6bf48e5ae2d4e1cda236a5cac4044a90d51a45bc7665ad` (4838 B).
- **Additions only.** `cmp -n` shows the first 16301 B and 4838 B of the two test files are byte-identical to the originals. The old 11 CLI tests and 4 compat tests are byte-exact, and no import was added.
- **Patches:**
  - `core_continuations.additions.patch` (`92d2dd5b…`): +18.
  - `compat_v1.additions.patch` (`fa398d3a…`): +22.
  - Neither patch has a `-` body line.
- **Oracle freeze:** see `opus-last-two-tests/ORACLE-FREEZE-NOTE.md`. It was frozen at 06:43:12Z, before the assertions were written, with `test_source_exists_at_freeze: false`. A Python format self-check was declined by the harness and **not performed**.
- **Unchanged pins:** oracle `normal-oracle.json` `76075b63…`, `Cargo.lock` `dfffb853…`, `compile.rs` `8ec1daaa…`, `lower.rs` `0e3d7aac…`, `json.rs` `faef7db8…`.
- **Test counts in the files:** 12 `#[test]` in the CLI file and 5 in compat.

## F-1: `h18_core_two_compile_signed_target_is_unsupported_before_output_directory`

**Inputs.**
- Source: `delivery.mori` (`/2`), the `DELIVERY` constant that already exists.
- Target: `SIGNED_TARGET`, a literal byte-identical to the decoded `target_canonical_utf8` in `examples/core/native-owner-intent/golden-vector.json`. A Grep with the full escaped regex matched once. This is the same data as the backend `signed_target()` fixture.
- It is embedded as a literal so the CLI test binary does not `include_str!` that file, which also holds `seed`/`private_witness` fields. No key material is used.

**Assertions.**
- The target parses publicly with `signed_policy` set.
- Exit code 1, with empty stdout.
- `diagnostic()` equals `mori::command: mori::backend::unsupported: Staged moriarty/2 relations are unsigned; a signed Target is refused.`
- `!out.exists()`.
- The temp directory holds exactly 2 entries (`source.mori`, `target.json`).

**Guard order traced** (`crates/mori/src/commands/compile.rs:18-35`):
1. `read_source`.
2. `mori_core::language == "moriarty/2"`.
3. `core_io::read_json::<Target>` (`core_io.rs:235-239`). This is the same `json::parse` + `serde_json::from_value` as `Target::from_json`, `lib.rs:74-77`.
4. `compile_staged`, whose first statement refuses a signed Target (`stage/lower.rs:22-28`).
5. That happens before `stage::load`, `target.validate`, any lowering and the `create_dir` at `:25`.

The error then goes through `core_io::operation` (`:158-164`). The code is `mori::command`, and the message is the backend `Error` Display `"{code}: {message}"` (`lib.rs:344-354`) through the ASCII-only `display::text`. There is no cryptographic verification, no signature, key or SDK call, and no source compilation past the guard.

## F-2: `h15_await_and_when_identifiers_keep_program_and_execution_bytes`

**Inputs.** The source constant is `/1` with `let await = 1; let when = await; next count = when;`, and the scenario is the default `{}`.

**Assertions.**
- `format!("{}\n", mori_core::json::export(&parse(src)))` equals `program.json`, byte for byte.
- `format!("{}\n", export(&simulate(.., "run", ..)))` equals `execution.json`, byte for byte.
- Both fixtures are also decoded and compared for equality with the actual values. This is in addition to the byte checks, not a substitute for them.

**Notes.**
- `export` is the baseline public wire formatter (pretty, ASCII-escaped) plus the CLI `println!` newline. It is unchanged since `fe01bc8b`.
- No `StageSegment` deserialisation is involved, and there are no negative cases.
- **Prediction (not derivation):** the current parser dispatches `await` only for `/2` (`parser.rs:319`). The `await`/`when` reservation exists only in `stage/validate.rs:30`. So I expect a pass, but it is unexecuted.

## Classification (author/classifier only; conditional ordinary)

Both commands share these conditions:
- The existing `flock` 120 s / `timeout` 300 s / env-scrub wrapper and the shared ordinary public Cargo object cache.
- Source, fixture and config/env pins recorded before and after.
- `--offline --locked -j2 --test-threads=1`.

### 1. `cargo +1.99.0 --offline --locked test -p mori --test core_continuations -j2 -- --test-threads=1`

**Verdict: conditional ORDINARY, anticipated 12.**

- **Old 11:** their bytes are unchanged, so the prior classifications are reused verbatim.
  - The `--trace` and `compile-call` cases stay the source-profile-first ordinary exceptions from OPUS-FINAL-A-REPAIR. This is not broadened to native commands in general.
- **New path:** the `compile` refusal described above. It reads only `source.mori` and the public `target.json` in the temp directory, and it never writes the output directory.
- **Residual:** the build links `mori` with the retained SDK compile-time public-verifier parameter embedding. Nothing private, SRS or cache related is inspected.

### 2. `cargo +1.99.0 --offline --locked test -p mori-core --test compat_v1 -j2 -- --test-threads=1`

**Verdict: conditional ORDINARY, anticipated 5.**

- Everything is pure legacy serde, parser and interpreter work, plus `include_str!` of two public JSON fixtures. There is no I/O at test runtime and no SDK runtime.
- **Residual:** the `mori-core` dev-dependencies (`midnight-base-crypto`/`midnight-serialize`, `Cargo.toml:17-19`) compile into the test target as before. `compat_v1` does not reference them, and this matches the already-captured compat command.

**If both pass later:** 12 CLI + 5 compat replace the old 11 + 4, giving **88 distinct tests**. Reruns are not summed.

**Still required:** execution captures and fresh non-author review. No product, docs or count changes were made.
