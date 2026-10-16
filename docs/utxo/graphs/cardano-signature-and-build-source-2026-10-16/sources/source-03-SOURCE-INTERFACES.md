# Frozen Core workspace — build source snapshot

This packet exports the exact tracked public four-crate workspace for a future separately reviewed signer-helper build. It contains **70 workspace files (253,884 bytes)**, exactly matching the 70 unique `CORE_FILES` entries in the current [roster-reference source](source/roster-reference/build_identity.rs). The estimated 69-file count is not used. All 73 copied source/provenance/license files match the named checkout HEAD bytes; the [manifest](SOURCE-MANIFEST.json) records each original path, destination, size and SHA-256.

The captured checkout context is `6bd2af3de77b30180343f03c86834e0c87ded507`. That commit identifies repository context, not a complete compiler build, helper binary, dependency resolution, cryptographic implementation or target result. Existing source comments and historical proposed/unreviewed labels are preserved unchanged. No Rust/Cargo/native code ran.

## Future local dependency mapping

The workspace layout is retained under [source/moriarty](source/moriarty/Cargo.toml). An eventual helper's Cargo path dependency can identify the existing crate by this **source location**:

```toml
[dependencies]
mori-core = { path = "/home/charl/moriarty-eutxo-validation-2026-10-12/current-rust-core-signer-build-source-snapshot-sol-20261016/source/moriarty/crates/mori-core" }
```

This is a location mapping, not an authored helper manifest, runnable instruction or build authorization. If the packet moves, the path must identify the corresponding `source/moriarty/crates/mori-core` directory. The helper must retain its enclosing captured workspace for inherited package/dependency declarations. No source file or Cargo manifest was rewritten.

| Captured package | Direct local edges in its manifests | Direct normal external declarations |
| --- | --- | --- |
| [mori-core](source/moriarty/crates/mori-core/Cargo.toml) | mori-lexer | serde, serde_json, thiserror, sha2, k256 |
| [mori-lexer](source/moriarty/crates/mori-lexer/Cargo.toml) | mori-diagnostics, mori-span | soa-rs |
| [mori-diagnostics](source/moriarty/crates/mori-diagnostics/Cargo.toml) | mori-span | miette |
| [mori-span](source/moriarty/crates/mori-span/Cargo.toml) | none | miette |

The workspace uses resolver 3, edition 2024 and declared rust-version 1.99. [rust-toolchain.toml](source/moriarty/rust-toolchain.toml) requests 1.99.0, minimal profile, rustfmt and clippy. This does not establish that any such toolchain is installed or usable. The workspace has `members = ["crates/*"]`; the actual tracked selected subtree has precisely these four package manifests.

The [static inventory](STATIC-BUILD-INPUTS.json) retains inherited declarations, feature maps, normal/dev/build dependency sections, workspace patches and local edges. Diagnostics has a `fancy` feature; test/dev declarations include proptest, insta, indoc and Midnight dependencies. Workspace declarations also name six absent local packages (mori-ast/checker/backend/ledger/fmt/parser), none referenced by the selected four crates' direct dependency sections. Those unused declarations are preserved; their effect on a future Cargo resolution was not experimentally checked. Git-based Midnight declarations/patches are retained as source text, without following or acquiring them.

## Captured source boundaries

All tracked workspace files, including snapshots and test source, are included. A bounded text scan found **27 `mod name;` declarations with matching captured default-path candidates**, no `include!`, `include_str!`, `include_bytes!`, `env!`, `option_env!`, `#[path = ...]` or explicit build assignment, and no tracked `build.rs` in this workspace. No direct include dependency was therefore followed. These are text-scan observations, not macro-expanded Rust or Cargo closure proofs. Snapshot source was copied, not tested.

The separately copied [build_identity.rs](source/roster-reference/build_identity.rs) is provenance for roster membership. It is **outside the dependency workspace** and is not a new build member. Its own compiler-source include macros and historical identity claims are not evaluated, materialized as another compiler closure or adopted here. This packet does not supply the K2/R1CS driver/compiler source or establish how a previously executed binary was built.

The exact existing [K2 Cargo.lock](source/k2/Cargo.lock) is retained as a **resolution candidate**, SHA-256 `39b7db9be3e68b2a29cf281d53e54546a65c6862921771b9462cacb8f08de556`. It remains in `source/k2`; no `Cargo.lock` was invented or installed inside `source/moriarty`. A new helper's resolved dependency graph, feature union and lock are unobserved. This lock is not an exact cryptographic or compiler-binary guarantee. The [repository license](source/REPOSITORY-LICENSE) is copied separately from its original root location; it is not represented as a pre-existing workspace file.

## Remaining external inputs

The packet is complete for the **tracked selected workspace**, not for a build. Registry/git dependency bodies, proc macros, their build scripts, compiler/sysroot/linker/target libraries, Cargo configuration/source replacements, platform inputs, helper source/manifest and its eventual lock/flags/features remain outside this snapshot. No Cargo cache, tool binary or external body was inspected. Earlier k256/ecdsa source acquisition is a separate packet and is not silently incorporated here.

[Read coverage](READ-COVERAGE.json) distinguishes the fully inspected manifests/roster from source bodies that received only static scans and byte checks. The selected export digest in the manifest names sorted path/size/SHA-256 rows; it is not the compiler's own Core-closure digest. No semantic equivalence, security judgment, key operation, signing, proof, target acceptance or new execution authority follows from this export. Public Moriarty compile/prove/deploy remains permissionless. A future isolated helper build and its review remain separate work.
