I'm writing the classification record to the plan file. It is the only file I may write in plan mode. I ran no other tools, read nothing in the checkout, and computed no hashes.


<invoke name="Write">
<parameter name="file_path">/home/charl/.claude/plans/you-are-opus5-5-high-abstract-wand.md</parameter>
<parameter name="content"># Opus security classification — exact runtime/command scope only (core-source-provenance)

## Context

This review classifies three things:
- the compile/link of the locked graph;
- the fixed baseline mapped export;
- the single exact normal test `source_step_field_correspondence_survives_public_export_and_reopen`.

It supersedes my OPUS-DESIGN §5 NEED-SOURCE for this command only. It is **not** candidate approval, test approval or execution.

- Native authority stays ZERO, with gross4334/remaining0 unchanged.
- All U/M/MC/SP/G exits remain open.
- The candidate artifact is unproven.
- I read only the supplied excerpts. I ran no tools against the checkout and made no hash or checkout verification claim.
- Root supplies the source hashes separately.

## Decisions

| Item | Decision |
|---|---|
| **(A)** compile/link of the same locked graph (`cargo +1.99.0 --offline --locked test -p mori --test mapped_role_static_io -j2`) | **APPROVE ORDINARY CONDITIONAL ON FINAL HELPER REVIEW**, with residuals R1–R2 and R7 |
| **(B)** fixed baseline export (`R/baseline-mori --color never compile-call … --authorization unsigned-mapped-roles … --out baseline-out`) | **APPROVE ORDINARY CONDITIONAL ON FINAL HELPER REVIEW, plus condition C2** (baseline three-file diff). Without C2, (B) is **NEED-SOURCE**. |
| **(C)** exact normal test, including candidate export, the in-test baseline invocation and candidate `--inspect-static` | **APPROVE ORDINARY CONDITIONAL ON FINAL HELPER REVIEW** (C1–C8) |

No REJECT. I found no reachable forbidden effect in the supplied closure.

## Actual reachability (traced from supplied source)

### 1. Test process (in-process, public data only)

- `Fixture::new` creates `std::env::temp_dir()/mori-mapped-cli-<pid>-<n>`.
- It writes the `PROVENANCE_SOURCE` fixture and copies `examples/core/static-compile/work-limits.json`.
- `bindings()` runs:
  - `serde_json` decode of the public `financial-duty.bindings.json`;
  - clearing of duties;
  - `mori_core::parse`, `codec::hash`/`object_hash`, `layout::templates_hash`, `name_key`, `native_hex` for the `base` StateField.
- No secret key, signing, preimage or relation check is involved.
- Drop is `remove_dir_all` of its own fixture directory only.

### 2. Candidate product, first run (export)

The binary is `env!("CARGO_BIN_EXE_mori")`, a compile-time absolute path into this `CARGO_TARGET_DIR`. There is no PATH search or discovery.

1. `main` → `Cli::parse` → `commands::run` → `compile_call::run`.
2. `validate_shape`, then `run_static`.
3. `read_static_input` (O_NOFOLLOW, capped) reads the limits, source and bindings.
4. Parsing and budget: `parse_work_limits`, `parse_static_bindings` (strict JSON), `check_input_budget`, `mori_core::parse`.
5. `static_compile::compile`:
   - `validate`, then `compile_mapped_roles`, then `layout::validate_source_static_roles` → `admit_bindings_inner(MappedRoles)`.
   - The binding check decodes three public `SignatureVerifyingKey`s (k256 point decode) and calls `UserAddress::from` (SHA-256). It also decodes the public token type.
   - It then calls `relation::compile_source_static_roles`, which reaches `compiler::compile_inner` with `signed=None`.
   - `compile_inner` uses `Builder` and `ops::{abi_guard, claim_update, read_cell}` as **Op constructors only**.
   - `Op::field_vec`/`field_size` are pure Fr encodings, as shown in the supplied `ops.rs` 456–558, `state.rs` 169–225 and `fab.rs`.
   - `validate_statement_emissions` is pure.
   - `proving::source_closure()` is `codec::hash(concat!(include_str!…))`: compile-time constants, with no runtime file, cache, parameter or key I/O.
   - `codec::encode(IrSource)` is a tagged serialize into a `Vec`, and `json_bytes` writes into a bounded writer.
   - `proof_io::publish_new` stages the files and hard-links them inside the fixture's `out/` only.
6. `Sp::new` **is** reached. `ops::push`/`cell` in every claim block calls `Sp::new`, which calls `default_storage::<InMemoryDB>().arena.alloc`.
   - D is concretely `InMemoryDB`, because `NativeOp = Op<ResultModeVerify, InMemoryDB>`. So any backend or DB call resolves to `InMemoryDB`'s `Arc<Mutex<BTreeMap/HashMap>>`.
   - No other storage type is set in a fresh process.
   - The lockfile contains no rusqlite or parity-db.
7. **Not reached by this fixture:**
   - `serialize_to_node_list_bounded`, `transient_commit` (Compress atom) and `hash_to_field`. These occur only on the `emit` path, and the oracle fixture has no `emit`.
   - `local_deployment::compile_all_policy`. It sits only in the Unsigned/SignedV2 match arm; the mapped arm is statically separate.
   - `relation::{compile, check, admit}`, `process`, `prepare`, `proving` (other than `source_closure`), `parameters`, `signed::*`, `lower_ops`.
   - The VM, keygen, prove, verify, degree, SRS, ledger apply, network and private key paths.
   - `onchain_vm` symbols used here are the `Op` enum and constants only; no VM is executed.

### 3. Baseline product (B)

- The binary comes from `MORI_PROVENANCE_BASELINE_BINARY`, read only inside the `if let Some`. It is spawned with `Command::new(binary)` and receives identical `get_args()` with `--out baseline-out`.
- Its path is the same as item 2 at 95df. Production differences from the candidate are confined to `relation/{mod,compiler,arithmetic}.rs`, according to the supplied git status. Those baseline-side bytes are not supplied; see C2.
- The test compares only the `action-000000.ir.bin` bytes. No digest is invented.

### 4. Candidate product, second run (`--inspect-static out`)

1. `static_compile::reopen`: input budget, `validate_source`, no-follow marker read, strict marker parse.
2. Identity, leaf-set and directory-entry checks; capped exact-length leaf reads.
3. A fresh `compile` (same path as item 2) and an exact byte-equality check.
4. Only after that equality, `codec::decode_artifact::<IrSource>` runs, using the SDK tagged deserialize (`Operand` hex → `Fr::from_le_bytes`) plus a canonical re-encode check.
5. Output goes to stderr status only. There are no writes.

## Conditions (all must hold before any execution; root's later source check)

- **C1 – Final helper review.** The final `relation/mod.rs` must be reviewed separately. Final `compiler.rs`, `arithmetic.rs` and `mapped_role_static_io.rs` must be byte-identical to the versions supplied here, or be re-reviewed. The review must confirm all of the following:
  - The non-test additions are pure data. There is no new `use` or call of `std::{fs,env,process,net}`, io beyond `Vec`/bounded writer, `relation::check`/`ProofPreimage`, the VM, proving, keygen, parameters, ledger or signed modules.
  - The validator is called only from `compile_source_static_roles`.
  - `compile`/`compile_bindings`/`check`/`admit`, `Manifest`, `Group` and the shared `Builder::op` refusal are unchanged.
  - Any refusal uses `source-static-emission`.
  - Added `#[cfg(test)]` unit tests are **neither compiled nor run** by this command, because mori-ledger is built as a dependency. They therefore need their own classification before any `-p mori-ledger` run.
  - No `Cargo.toml` or `Cargo.lock` change.
- **C2 – Baseline diff (B only).** Supply `git diff eb2577d6 -- crates/mori-ledger/src/relation/{mod,compiler,arithmetic}.rs`. It must show that the baseline-only (removed) lines contain no I/O, env, process or SDK-execution calls. Equivalently, show that 95df's three files equal the candidate minus the emission/format/validator hunks. Without this, (B) is NEED-SOURCE.
- **C3 – Minimum CLI excerpt.** Supply `crates/mori/src/cli.rs` (`Cli`, global args, `Command::CompileCall`, verbosity/`ColorChoice` conversions), `display::text` and `error::CliError`. They must confirm no `#[arg(env = …)]`, I/O or process use. If any appears, (B) and (C) revert to NEED-SOURCE.
- **C4 – Exact command.**
  - Use the command exactly as stated: `env -u MORI_FEE_POOL_CAPTURE_DIR`, the same `CARGO_TARGET_DIR`, `-j2`, `--exact --test-threads=1`, the 300 s walltime and the existing build lock. No broader suite.
  - `MORI_PROVENANCE_BASELINE_BINARY` must be the literal absolute path `/home/charl/Moriarty/.moriarty-dev/runtime/core-source-provenance-20261015/baseline-mori`. No unexpanded variable and no bare name, because a bare name would trigger a PATH search.
- **C5 – Baseline pinning.** Root checks SHA-256 `89ae66c4…eff28e` immediately before and after the run. There is no baseline rebuild. Any mismatch voids the evidence.
- **C6 – Toolchain and environment.**
  - `+1.99.0` must already be installed, so there is no rustup auto-install or network.
  - No RUSTFLAGS or feature or profile change relative to PR52.
  - No other `MORI_*` variables set.
- **C7 – Out of scope.** No H2–H5, ordinary `unsigned` or signed mode, other tests, `fee_pool_component.rs`, `mapped_role_static.rs` or `mori-ledger` tests are approved by this decision.
- **C8 – Stop on mismatch.** On any mismatch against the frozen oracle or the baseline IR, stop and report. Do not edit expectations.

## Residuals (recorded, NOT waived)

- **R1 – Build scripts and features.**
  - The transitive build scripts (cc/cmake: blst, ring, aws-lc-sys, etc.) and the unified feature graph were not observed. They are cached from PR52's identical locked graph and only rerun if fingerprints change.
  - `onchain-runtime/build.rs`: the vendored branch is assumed because mori-ledger's workspace dependency keeps default features (`vendored`). The non-vendored branch would run `scheme` and write into the immutable git checkout. That would not touch the network and would fail closed.
  - Repository crates are not confirmed to lack a `build.rs`. This only matters for a mori-ledger build script, which would rerun on the source change.
- **R2 – Proc macros.** Proc-macro expansion text (serialize/storage/serde/thiserror derives) was not expanded. I reviewed the generator source. The Storable non-base `Serializable`-via-`Sp` route applies to `StateValue`, which is not serialized on this path.
- **R3 – Crypto internals.** Uninspected: k256 x-only and SEC1 point decoding for the three public role keys, sha2, the midnight-curves `Fr::from(u128)` and `from_le_bytes` (on reopen).
- **R4 – Storage-core gaps.**
  - `StorageBackend::cache/uncache`, `Cache`, `Sp`/`Arena` drop, `DEFAULT_CACHE_SIZE` and `dubious_batch_update` were not supplied.
  - The type-level `InMemoryDB` confines DB calls to memory.
  - Storage-core depends on `sysinfo` and `tempfile`, but their call sites are unseen. Neither is a forbidden effect class: at most a `/proc` read or a temp file.
- **R5 – Standard library and runtime.** Uninspected: std, allocator and mutex internals; serde_json; clap; anstream; terminal_size; mori-lexer; mori-diagnostics rendering.
- **R6 – Fixture location and timeout.**
  - The fixture location follows `TMPDIR`. This affects fixture placement only, not the product target.
  - A SIGKILL at the 300 s walltime may leave a public fixture directory behind.
  - With the capture directory unset, a panic leaves no persisted outputs beyond cargo stdout.
- **R7 – Linked but unreached.** Linked, with no call site on the traced path: reqwest/rustls/aws-lc/quinn/tokio, midnight-proofs/circuits/zkir prove/check, `local_deployment`, `proving`, `parameters`, `prepare`, `signed`.

## Independent oracle expectations noted before execution (ORACLE-DESIGN 33e37dc7…)

**Asserted by this test:**
- one action, `input_count` 1, format `/2`;
- `debit_sources` [alice, bob];
- source statements with roots 0, 1, 4, 5, 6;
- the five emission rows: instruction ranges [7,41), [41,75), [75,86), [86,120), [120,154) and group ranges [3,16), [16,29), [29,32), [32,45), [45,58);
- dynamic `(group, offset)` pairs (8,3), (11,3), (13,4), (21,3), (24,3), (26,4), (31,3), (37,3), (40,3), (42,4), (50,3), (53,3), (55,4);
- all 58 group widths and origins (claim block `[1,4,1,19,1,4,1,18,4,19,5,1,1]`; state-read 1, 8 and state-opening 4);
- baseline IR byte equality;
- reopen accepted.

**Checked only implicitly or not at all:**
- The 155-instruction total and the final `Output` are implied only by the product validator's `body_end`.
- Not checked:
  - Impact instruction indices;
  - `%native.N` operand identities, including Let `%native.27` and `%native.28`;
  - old/positive guards;
  - operation-field positions (35, 58, 82, …; 176/178);
  - all financial pre/post values.

These remain open. They are arithmetic expectations, not observations.

**H1 note.** The supplied validator enforces "every Variable Impact input is listed" and explicitly permits listed Immediates. It does not enforce my original "Immediate only for literal root" clause. The correction belongs to the security author and final review. This test does not implement H1.

## Verification (later, only after C1–C3 and root's GO)

- Run the single exact command once.
- Record the exit code, the cargo stdout, and the baseline SHA-256 before and after.
- Any failure is a stop-and-report, not a fix-forward.
</parameter>
</invoke>

<invoke name="ExitPlanMode">
</invoke>
