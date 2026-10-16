# Independent execution-eligibility review: B-core pure compile candidate

This is a non-author review of the Opus-authored adapter, CONFIG and replacements as materialized in the supplied candidate. I did not execute anything, call any tool, or recompute any hash. Every hash and byte count below is a recorded input that I compared textually, not one I verified. Native authority stays 0, C4 is unchanged, and no resource or wall-clock class changes.

## 1. Manifest and config consistency (textual cross-check)

- **CANDIDATE.json** has 10 rows: the 7 `candidate/` rows, plus `SOURCE-CANDIDATE.json`, `CONFIG.json` and `run_arithmetic.py`.
  - This matches the runner's requirement that the manifest equal the source rows plus exactly those three files.
  - The guard path `candidate/models/sherlock/qa/run_host.py` is present.
- **The 7 candidate rows agree in all three places.** Path, bytes and sha256 are identical across CANDIDATE.json, SOURCE-CANDIDATE.json and `CONFIG.source_candidate`.
- **CONFIG passes `checked_config` as written:**
  - the format string and key set are exact;
  - `snapshot_members` equals the 7 `candidate/` paths with the prefix stripped;
  - `compile_closure` has exactly 3 entries, includes `compile_harness.rs`, and is a subset of the members;
  - the guard and `rust-toolchain.toml` are members;
  - `expected_tests` is `[]`.
- **Recorded pins match the stated baselines.**
  - `types.proposed.rs` is `ef9c70…`, 18821 bytes.
  - `functions.proposed.rs` is `c97063…`, 1407 bytes.
  - The guard is `ae5a04da…`, 26360 bytes, the same value as the prior reviewed guard. Equality with the earlier reviewed bytes is Root's condition to confirm; I cannot do it.

## 2. Compile closure: source composition and syntax reading

My reading of the source is that it should compile. The outcome stays UNRUN until the attempt.

- **`compile_harness.rs`**
  - `#[path = "types.proposed.rs"] mod raw_types;` sits at the crate root, so the path resolves to `<snapshot>/types.proposed.rs`.
  - `include!("functions.proposed.rs")` sits inside the private inline module `row_projections` and resolves relative to the invoking file, to `<snapshot>/functions.proposed.rs`. `include!` in item position accepts multiple items.
  - The `use super::raw_types::{ConstRow, DataRow, LogicalKind}` import is legal: a child module may name a private sibling at the crate root. All three imports are used.
- **`types.proposed.rs`**
  - A leading `//!` inner doc comment is legal at the top of a non-inline module file.
  - I counted 104 declarations (8 + 24 + 35 + 2 + 21 + 14) with no duplicate type-namespace name.
  - Field types named `Bool` resolve to the alias, because variants are not in type scope.
  - `r#ref` is a valid raw identifier.
  - There is no edition-2024 reserved word (`gen`), no recursive or infinitely sized type, and no impl, derive, const, static, macro, unsafe, extern or crate attribute.
  - Variants named `None`, `Ok`, `Copy` and `Error` are legal enum variants and do not conflict.
- **`functions.proposed.rs`**
  - Both matches are exhaustive over the frozen variants: 7 for `ConstRow`, 6 for `DataRow`.
  - `ConstRow::Nil { .. }` is a struct pattern and `DataRow::Nil` is a unit pattern, each correct for its declaration.
  - Default binding mode on `&T` with only `{ .. }` patterns involves no edition-2024 match-ergonomics change.
  - There is no wildcard arm, and `ConstructorExtensionReserved` is never returned.
- **Expected warnings** are many `dead_code` warnings: unused pub items in a private module, two unused private fns, and unconstructed variants. I see no deny-by-default lint and no `-D warnings` flag. Warnings are retained and not judged here.

**Compile-time attack surface.** No proc macro, build script, `include_str!`/`include_bytes!`/`env!`, or extern crate is present. Compile-time execution is therefore limited to rustc itself plus the unhashed sysroot. The rlib is written by rustc's archive writer and is only file-recorded; no step executes, loads or links it.

## 3. Adapter flow, traced against the unchanged guard

**Pre-attempt refusals.** All of these refuse before any attempt directory exists:

- fixed `ROOT` and runner path;
- strict JSON;
- manifest-versus-bytes check;
- exact manifest composition;
- CONFIG consistency;
- `candidate/` holds exactly the 7 regular files, with links and non-regular files refused;
- review gate;
- guard constants (`EDITION == '2024'`, `TIMEOUT_SECONDS == 30`, `SAFE_PATH`);
- `attempt-01` lies outside `candidate/`.

**Guard loading.** The guard runs from its verified bytes with `__name__ != '__main__'`, so `run_host.main()` never runs.

- `REPO` is overwritten to `CANDIDATE`; its computed `parents[3]` already equals `CANDIDATE`.
- The guard's `read_sources`/`postflight` resolve `REPO` as a module global at call time, so they read `candidate/`.

**Realization.** The tuple has the correct 11 fields.

- `producer = compile_harness.rs`, `tests = ()`, `sources` = the 7 members.
- `Attempt.__init__` derives `not_selected = NOT_SELECTED` from the empty suite.

**Preflight subclass.** It is a faithful adaptation:

- it re-reads members from `candidate/`;
- it requires hashes equal to the reviewed pins before setting `self.sources`;
- it writes the 0444 snapshot;
- it pins channel `1.99.0` and the exact `RUSTC` path;
- it uses the unchanged `compiler_record`, which rejects symlinks, rustup proxies and a missing sysroot.

The removal of the `check.py`/helper scan is correct, because no helper exists in the closure.

**Steps, in order:** `preflight` → `rustc-version` → `compile-library`.

- `rustc-version` is the unchanged `version()`, logging to `rustc-version.stdout` and requiring `release == channel`.
- `compile-library` is the unchanged `Attempt.compile`, producing exactly this argv: `[RUSTC, --crate-type=lib, --edition=2024, --crate-name=sherlock_b_core_pure_compile, --emit=dep-info=<build>/….d,link=<build>/…, <snapshot>/compile_harness.rs]`.
- The first failure marks all later plan steps not-run.
- `compile-tests` and `run-tests` are then appended as not-selected, using the guard's own names.
- `candidate-postflight` re-hashes all 10 manifest members, CANDIDATE.json, both reports and the `candidate/` tree.
- `finish()` runs the unchanged postflight: checkout and snapshot bytes, then the Python and rustc binary hashes before and after.
- `terminal_state` permits only the two declared not-selected steps, and `test_disposition` reads `not-selected`. An empty suite is never reported as a pass.

**Dep-info gate.** It is unchanged and correct for an rlib.

- Target lines list the three sources.
- Per-source empty-dependency lines have no `': '` separator and add nothing.
- `#` lines (env-dep or checksum) are skipped.
- The read set must equal the closure exactly, inside the snapshot.

**Unchanged controls:**

- child environment `{PATH=/usr/bin:/bin, LANG, TMPDIR}`, replacing the parent environment;
- no shell, Cargo, or rustup resolution;
- `stdin=DEVNULL`;
- a new session with process-group SIGKILL on leader timeout;
- 30 s per child;
- exclusive `attempt-01` creation (once-only);
- `-I -B` enforcement.

**Cargo.toml and git dependencies.** These are snapshot members only. No Cargo runs and no network fetch occurs.

## 4. Findings

None block the compile.

- **N-1. Empty directories go undetected.** `candidate_entries` ignores directories, so an extra empty directory under `candidate/` would pass. The snapshot is built only from pinned bytes and rustc reads only the snapshot, so this has no effect on the compile.
- **N-2. The review gate is line-exact and fails closed.** CRLF line endings, trailing whitespace, or a second line beginning with either gated token would refuse. Root must store this report byte-faithfully.
- **N-3. Self-attestation remains.** The runner's executing code is what Python loaded, and the on-disk bytes are checked against the manifest. The gate checks report text, not reviewer identity.
- **N-4. Toolchain dependency.** No toolchain probe has run. If rustc is missing, is a proxy, or reports a release other than `1.99.0`, the attempt stops at preflight or `rustc-version` and records that failure. That is the correct first-failure behaviour, not a reason to block.

## 5. Scope approved

Approval covers exactly one invocation of `python3 -I -B run_arithmetic.py`, with no arguments, from the fixed root, producing `attempt-01`. That invocation performs:

- preflight;
- `rustc -vV`;
- one direct `rustc --crate-type=lib` compile of the 3-file closure;
- not-selected test steps;
- both postflights.

Approval is conditional on these preconditions:

- Root confirms the guard bytes equal the previously reviewed guard.
- Root independently inspects the final adapter.
- Root allocates a fresh slot.
- A fresh Muse report bound to the same manifest SHA exists.

The approval excludes:

- tests, or execution, loading or linking of the artifact;
- Cargo, the SDK, a backend, models, proofs or native code;
- retries, patches without fresh review, or any resource change.

## 6. Limitations

This is a reading of supplied text only. There was no hash recomputation, no filesystem inspection and no compilation. The source-level reading is not a prediction of the result. Residuals are retained unchanged:

- the 0444 snapshot is no barrier to a same-UID child;
- memory, disk and process counts are unbounded;
- the sysroot, libc, Python stdlib and kernel are unhashed;
- dep-info is split on whitespace, constrained only by `SAFE_PATH`;
- authenticity rests on local SHA-256 bindings only;
- linker non-invocation is not verified.

## 7. Remaining adoption decisions (not decided here)

A compile success would be syntax and type evidence only. It would not mean schema freeze, B-core adoption, a two-vote decision, StepInv, admission, or a public API. R2 votes do not cover these Rust bytes. Still open:

- module placement, and replacing the `raw_types`/`row_projections` runtime-only names;
- the `profile_id`/`profile` spelling;
- host representation choices (`String`, `Vec`, `usize`/`u128` newtypes, `OccurrenceOwner`);
- derives, `repr`, discriminants, and D-23 ordinal alignment (declaration order is not ABI);
- visibility and private constructors under C6;
- reserved-tag C-INV at decode/admission, and the `Marker`/reserved handling of `arity`/`sortOfKind`;
- full `kind(ValueRef)`, `shallowDiffer` and `checkedArith`;
- the actual audit of the compile result.

VERDICT: APPROVE_WITH_SCOPE
CANDIDATE_MANIFEST_SHA256: 426355f97f13123f5e06e7080140446fe59449ce9d6c9dd91b443005c30db3f1
