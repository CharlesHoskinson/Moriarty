# Opus 5.5 effect and identity review: direct `rustc --test` of `source_closure_coverage.rs` for CUR03

**Status:** Approved with conditions. This covers effect and identity only. It is not a cryptographic approval.

**Basis:** I used only the source text supplied in this request: the test file, `proving.rs`, the ledger `Cargo.toml`, the root `Cargo.toml` and `rust-toolchain.toml`. I ran no tools, executed nothing and made no edits. I have not checked that the supplied text matches the files on disk. The hash preconditions below bind this review to the on-disk files.

## 1. Verdict

- **Ordinary under the native-zero boundary: yes.** The proposed direct path compiles a std-only test binary. It does not resolve, fetch or compile any Cargo dependency, the SDK, `mori-ledger`, `mori-core` or `mori-lexer`.
- **What a pass covers:** declared-closure membership selection, as this unchanged test defines it.
- **Earlier refusal:** Review d5d19cfc refused because a Cargo test path compiles the SDK. That refusal still holds for Cargo. It does not apply to this exact direct path.

## 2. Source findings (`tests/source_closure_coverage.rs`)

**It is standalone.** There is no `use` of `mori_ledger` or any external crate, only `std::{collections, fs, path}`. No `extern crate`, no `#[path]` and no `mod` declarations. Under Cargo the target would get `--extern mori_ledger` plus dev-deps, which would force SDK compilation. Plain `rustc` supplies no externs, and the file needs none.

**`proving.rs` is only text.** `include_str!("../src/proving.rs")` embeds it as a `&'static str`. It resolves relative to the test file, so the result is the same regardless of the working directory. None of its `use midnight_*` items or functions are compiled, linked or called. `source_closure()`, `codec::hash`, key generation, proving and verification are not reachable from this binary.

**`env!("CARGO_MANIFEST_DIR")` is read at compile time only.** The proposed value matches the test file's crate directory.

**Runtime operations are read-only:**
- `fs::canonicalize`, which uses stat and readlink.
- `fs::read_dir` recursion over `mori-core/src`, `mori-lexer/src` and `mori-ledger/src`.
- `DirEntry::file_type`, which does not follow symlinks, so the symlink assertion is effective.
- `fs::read_to_string` of walked `.rs` files and the two exclusion parents.
- Existence checks on all 69 members, which also touch CLI sources, the third-party notices and both `Cargo.toml`/`Cargo.lock` paths.

**The rest is in-memory.** The gap-control test builds abstract `PathBuf` sets such as `/r/a.rs` and never touches the filesystem.

**Nothing writes, spawns or connects.** There are no file writes, no process spawns, no environment mutation and no network access. libtest writes to stdout and stderr only, since no `--logfile` is used.

**No unexpected side effects found.** I have no grounds for rejection.

**Parse check against the supplied `proving.rs`:**
- There is exactly one `fn source_closure(`. The other occurrences are calls written as `source_closure()`.
- The body ends at the first `\n}\n`. The body is indented, so no earlier match exists.
- The `codec::hash(concat!(…).as_bytes())` shape matches.
- 69 `include_str!` paths parse.
- The first 61 equal `BASELINE` and the last 8 equal `CORE_STAGE`.
- On the supplied text, test 1 should pass. CRLF line endings or any extra token inside the `concat!` would fail closed.

## 3. Effects of the proposal

**rustc:**
- Writes `closure-tests` and transient `rustc*/` codegen temporaries in the output directory.
- Invokes the system linker (`cc`/`ld`).
- Does not use incremental, dep-info or metadata output.
- Writes nothing in the repository.
- Does not read `RUSTFLAGS`; that variable is Cargo-only.

**rustup proxy:**
- The `+1.99.0` form goes through the rustup proxy.
- rustup auto-install could reach the network if 1.99.0 is missing. Condition C3 below closes this.

**Test binary:**
- Only the read-only effects listed in §2.

**Not touched:**
- No Cargo, `Cargo.lock` writes, registry or git cache, `target/` or build scripts.
- No SDK, parameters, SRS, keys, proofs, native VM or ledger calls.

## 4. Approved commands (exact; conditions are binding)

```bash
W=/home/charl/Moriarty/.worktrees/utxo-pr54-currentness-20261015
S=/tmp/mori-cardano-pr54-ledger-source-closure

# A. Preconditions (read-only): see C1–C2. Then:
mkdir -m 0700 "$S"                       # no -p; must fail if $S already exists
cd "$S"

# B. Compiler identity record (read-only)
env -i HOME="$HOME" PATH="$PATH" RUSTUP_AUTO_INSTALL=0 \
  rustc +1.99.0 -vV >"$S/rustc-version.log" 2>&1; echo $? >"$S/rustc-version.exit"

# C. Compile
timeout -k 5 120 prlimit --as=2147483648 --cpu=60 -- \
  env -i HOME="$HOME" PATH="$PATH" RUSTUP_AUTO_INSTALL=0 \
  CARGO_MANIFEST_DIR="$W/crates/mori-ledger" \
  rustc +1.99.0 --edition 2024 --test \
  "$W/crates/mori-ledger/tests/source_closure_coverage.rs" \
  -o "$S/closure-tests" >"$S/compile.log" 2>&1; echo $? >"$S/compile.exit"

# D. Run
timeout -k 5 120 prlimit --as=2147483648 --cpu=60 -- \
  env -i "$S/closure-tests" --nocapture >"$S/run.log" 2>&1; echo $? >"$S/run.exit"

# E. Record
sha256sum "$S/closure-tests" >"$S/binary.sha256"
```

If `RUSTUP_HOME` or `CARGO_HOME` are set to non-default locations, pass them through `env -i` explicitly. No other variables may be added.

## 5. Conditions

**C1. Pre-freeze.**
- Recompute the root's 123-file freeze. It must equal the full recorded `384e…` digest; a prefix match is not enough.
- The freeze set must include:
  - `tests/source_closure_coverage.rs`
  - `src/proving.rs`
  - every walked `.rs` file under the three roots
  - the 69 members
- If the set does not include all of these, separately hash the missing files before step A.
- The hashes of the test file and `proving.rs` must equal the versions the root read for this review.
- Leave the Muse reference PASS `2defc8…` untouched; do not re-run it.

**C2. Scratch directory.** `$S` must not exist before step A. It must not be a symlink.

**C3. Toolchain.**
- `RUSTUP_AUTO_INSTALL=0` is mandatory.
- `rustc-version.log` must show `release: 1.99.0` and exit 0.
- Any install, download or "not installed" outcome means stop.

**C4. Compile.** `compile.exit` must be `0`.
- Record warnings verbatim; they do not fail the step.
- Any error, timeout or rlimit kill is a failure.

**C5. Run.** `run.exit` must be `0`. `run.log` must contain `running 6 tests` and exactly these six tests marked `... ok`, in any order:
- `closure_is_frozen_baseline_then_core_stage_files`
- `closure_members_are_distinct_existing_files`
- `closure_covers_every_core_source_file`
- `closure_covers_every_core_lexer_and_ledger_source_file`
- `exclusions_are_reasoned_cfg_test_modules_outside_core_stage`
- `coverage_gap_controls_reject_unknown_files_and_unused_exclusions`

It must also contain `test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`.

**C6. Post-freeze.**
- Recompute the freeze; it must be identical to the C1 value.
- The test file and `proving.rs` hashes must be unchanged.
- `$S` must contain only `closure-tests`, the `*.log`, `*.exit` and `binary.sha256` files, with no leftover `rustc*/` directory.
- `Cargo.lock` files must be unchanged; this request has no path that writes them.
- If git status is consulted, use `git --no-optional-locks status` only.

**C7. Stop rule.** Any failure in C1–C6 ends the request. Record the exact logs and exit codes. Then:
- Do not retry, raise caps, change the toolchain or edition, or set a different `CARGO_MANIFEST_DIR`.
- Do not edit the test.
- Do not fall back to Cargo.

Any variation needs a new review. Do not delete `$S` as part of this approval.

## 6. What a pass establishes, and what it does not

If C1–C7 all hold, CUR03 is satisfied for membership selection by this unchanged test at the frozen tree. Specifically, the declared `source_closure` `include_str!` list is all of the following:
- the frozen 61-file baseline followed by the 8 Core stage files, 69 in total, distinct and existing;
- covering every `.rs` file under `mori-core/src`;
- covering every `.rs` file under `mori-lexer/src` and `mori-ledger/src`, except three exclusions that are reasoned, `#[cfg(test)]`-declared and named nowhere else.

It also confirms the C1 cleanup at the file-membership level: any leftover Cardano or export `.rs` file under `mori-core/src` would fail tests 3 and 4.

It does **not** establish any of the following:
- The closure hash value, or that it matches any `PublicDerivedCallKey` snapshot. The disclosed root `Cargo.lock` addition changes the hashed closure. Any previously derived snapshot would therefore refuse with `derived-source-identity`, and build identity stays unqualified.
- That the actual Rust build used these inputs. The closure is declared identity, not build authentication.
- Coverage outside the three walked roots. This includes:
  - new files under `crates/mori/src` (only listed CLI files are pinned)
  - `build.rs` at crate roots
  - non-`.rs` files pulled in by `include_*`
  - non-test `#[path]` modules outside `src`
  - feature or `cfg` selection
  - git and SDK dependency sources
- Any semantic or native qualification: SRS provenance or setup, SDK keygen correspondence, the source–constraint theorem, proofs, verification, Cardano export semantics, settlement.

The author's source report counted an SDK effect based on the assumed Cargo path. For this direct path only, that effect finding is superseded. Independent source-comparison stages do not waive any other required native qualification.