# Native ledger v5 resource audit — Grok 4.6 high

2026-10-01. Independent complete current source, observed-result, and resource audit. Requested identity: Grok 4.6 at high effort. Returned identity: Grok 4.6 released by xAI. This session wrote only this file under `/home/charl/research/moriarty-signed-intent-2026-10-01`. No other auditor report, verdict, or disposition was read. Historical raw source, build, and handoff failures and receipts are evidence. They are not votes.

## 1. Scope, method, and startup

This is a resource vote on the frozen v5 helper-only candidate. It is not whole-handoff acceptance, not native preparation success, not proof, not well_formed, not apply, and not Preview or language closure.

Method: read-only source, API, hash, `ast.parse`, and `file(1)` inspection. Python hashed files with SHA-256. No wrapper import or execute. No Cargo metadata, fetch, or build. No Node or TypeScript candidate execution. No nativecheck or nativeprepare. No SRS, keys, parameters, proof, well_formed, apply, wallet, transaction, or network acquisition. No old approval reuse. No diff-only audit of the current tree.

Primary checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. AGENTS and `plugins/moriarty-dev/skills/develop/SKILL.md` were loaded. Guarded status JSON: capability `SP01.6 loan-swap-subset`; unresolved operational history; stale admission inputs; missing current accounting and live resource evidence; `pendingTransactions: []`. This audit created no campaign and no execution authority.

Contracts read in full: `NATIVE-LEDGER-INDEPENDENT-EXPECTATIONS.md`, `NATIVE-LEDGER-PREPARE-EXPECTATIONS.md`, `NATIVE-BETA-HANDOFF-INDEPENDENT-EXPECTATIONS.md`. Proposal read in full: `NATIVE-LEDGER-V5-RESOURCE-PROPOSAL.md`. Authoritative freeze: `NATIVE-LEDGER-V5-REVIEW-FREEZE.json`. Preview research evidence read from `PREVIEW-COMPATIBILITY-20261001.md` and `preview-source-linkage-20261001/COMPATIBILITY-SOURCE-LINKAGE.md`.

## 2. Independent freeze and pin hashes

Observed SHA-256 of freeze-named files, all matching the freeze object and the user pins:

| Path | SHA-256 | Bytes |
| --- | --- | ---: |
| `native-ledger-consumer/manifest.json` | `bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f` | 54842 |
| `run-native-ledger-v5-bounded.py` | `371df382f0758a86428d4a899c6b5f9ef7f695b78fe398c16e3d5903fb9af7be` | 23424 |
| `NATIVE-LEDGER-RUNTIME-INPUTS-V5.json` | `68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425` | 96718 |
| `NATIVE-LEDGER-V5-RESOURCE-PROPOSAL.md` | `d232c5eca15053983288ed2e22d99eba7d177e28e04080e5ad296590bf3bf07b` | 9013 |
| `NATIVE-LEDGER-V5-WRAPPER-CHANGES.md` | `f4cf7216f7e562052d27fc4c97148024db93238cf8995e347411ccf5a4d94906` | 2747 |
| `NATIVE-LEDGER-V4-COMPILED-MANIFEST.json` | `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb` | 54842 |
| `native-ledger-v4-build-receipt.json` | `df9cc75d34d9f24bd0ec94deab880a5db76d243fa76e77902829c613d095e03b` | 3602 |
| `native-beta-handoff/SOURCE-HASHES.json` | `20a8aaf775fed2a2ed1bfba87c811aceaa6924e9ecda030266b63143490ff3dc` | 435 |
| `NATIVE-BETA-HANDOFF-RUNTIME-REPAIR.patch` | `5dee39da4254343a08aac3b34429d5bda6501f59d1d2c13277f37944113f9036` | 1231 |

Additional independent hashes:

| Object | SHA-256 |
| --- | --- |
| `NATIVE-LEDGER-V5-REVIEW-FREEZE.json` | `e1dab9633b55082799c1c9770ec31ddeb9b56cc6cd11e5ed7cbc0eb5da8ed14b` |
| `native-beta-handoff/handoff.ts` | `add4ad0be8f2e6e71343bbec9aabaacc9764b3a5bf8d485e3cbdcd4797499a1c` |
| `native-beta-handoff/cli.ts` | `9c04b0cfface408224fdd88247dfd52b38c9ad9f50853dccd5f7c60360332591` |
| `native-beta-handoff/INPUT-SOURCE-HASHES.json` | `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476` |
| `native-beta-handoff/README.md` | `ad884d68e66d204597a549590be905701775c67e9c92e77c6861237645930a61` |
| actual executable `beta-native-ledger-consumer` | `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688` |
| signature binary `moriarty-midnight-crypto` | `42fa598d8b54334650f60bc13428bff3d79bb240a5bb0c203fc84684b5a30339` |

Freeze `scope`: `v5 helper-only source/resource amendment, actualcompiledv4inputs/binary unchanged; no v5execution/proof/application`.

## 3. Exact compiled correspondence

Current consumer manifest and compiled v4 manifest each contain 39 internal keys and 260 external keys. Internal maps are equal entry-for-entry. Every current internal hash matches the on-disk consumer file. Every current external hash matches the on-disk input. Zero missing paths.

External key sets are identical. Hash changes are exactly two keys:

- `/home/charl/research/moriarty-signed-intent-2026-10-01/native-beta-handoff/handoff.ts`
  - compiled: `d17f542d5afa2cadd91fa721ec48bcd36f4ec091ab730b33c1aac9c177e61b38`
  - current: `add4ad0be8f2e6e71343bbec9aabaacc9764b3a5bf8d485e3cbdcd4797499a1c`
- `/home/charl/research/moriarty-signed-intent-2026-10-01/native-beta-handoff/SOURCE-HASHES.json`
  - compiled: `bfebf8e7e117e8029dfb667aabe00df1836a776c955b5129e1512c7cb24bf05f`
  - current: `20a8aaf775fed2a2ed1bfba87c811aceaa6924e9ecda030266b63143490ff3dc`

All other 258 external hashes are unchanged, including Rust sources, Cargo.toml, Cargo.lock, official ledger/ZKIR APIs, production Beta/Core/Source files, kernel, IR, frozen preimage, projection, public CLI files other than `handoff.ts`, and INPUT authority `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476`.

This is precise binary-reuse correspondence: the compiled v4 internal map still describes the current Rust caller. The wrapper's declared helper-change set is exactly those two keys. An unrelated stale executable cannot inherit this map.

Current manifest `scope` string still says `v4 source candidate: actual v3 compile failed; compiler repairs and production Beta handoff unexecuted`. That prose is stale relative to the actual v4 compile and failed handoff receipts. The hash maps remain the authority.

## 4. Runtime 240+240 and canonical symlink

`NATIVE-LEDGER-RUNTIME-INPUTS-V5.json` pins `source_manifest_sha256` equal to the current consumer manifest. Installed `files_sha256` has 240 entries. `kernel_files_sha256` has 240 entries. On-disk installed `node_modules` file set equals the declared set. On-disk kernel `node_modules` file set equals the declared kernel set. All 240+240 content hashes match disk.

`kernel-prototype/node_modules` is a symlink. `resolve()` equals `compiler-probe/runtime/node_modules`. Hash-value sets of the two maps are equal. `symlink_resolution` has one entry: the kernel root to that canonical directory. `package-lock.json` SHA-256 `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e` matches the runtime record. Handoff and INPUT manifest pins inside the runtime record match the current handoff files.

## 5. Wrapper source (`ast.parse` only)

`ast.parse` of `run-native-ledger-v5-bounded.py` succeeded: 4368 AST nodes, 255 lines. Imports are `os,sys,time,json,hashlib,subprocess,resource,signal,shutil,pathlib`. The module was not imported or executed.

Observed recipe:

- Allowed phases are exactly `handoff` and `prepare`. The argv assertion text is `exact production beta handoff then native preparation; no build allocation`.
- No Cargo or build command is constructed. The substring `cargo` appears as the cache path, a `cargo_jobs` receipt field, and monitor prose. Child commands are `unshare -Urn -- node .../cli.ts ...` then, only after frozen handoff success, `unshare -Urn -- .../beta-native-ledger-consumer prepare CONFIG PIN OUT`.
- One exclusive durable attempt per phase (`O_EXCL` on `native-ledger-v5-{phase}-attempt.json`). Fresh output paths. Existing v4 attempts are not deleted.
- Prepare requires `native-ledger-v5-handoff-receipt.json` with exit 0, null stop, current manifest/runtime pins, and the compiled v4 binary pin. Handoff failure blocks preparation.
- Binary reuse checks: successful v4 build receipt pin, compiled-manifest pin, path `target/debug/beta-native-ledger-consumer`, SHA-256 `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`, rehashed before launch and after each helper.
- Limits encoded: wall 60 s, group CPU 120 s, group RSS and per-process address space `2 * 1024**3`, incremental cache 1 GiB, incremental target 2 GiB, total target 8 GiB, free floor 10 GiB, combined outputs 32 MiB, per-process file 2 GiB, sample 0.1 s memory/CPU/RX and 5 s disk, `unshare -Urn`, owned `killpg` cleanup, signal mask around launch, durable reservation remaining if SIGKILL/power loss.
- Handoff postconditions freeze `NativeFixturePreparedUnqualified`, four premises, four unverified bindings, `authority_valid: null`, false proof/WF/apply/ledger flags, exact head mapping with `authenticates_predecessor: false`, and four artifact names/paths/hashes/lengths plus helper receipt and config digests.
- Prepare parent independently builds the closed five-artifact plus literal fixture config, compares helper config as parsed data, then exclusive `xb` + fsync of root config and exact-digest `prepare`.

Monitor limitation text records overshoot and global non-loopback RX as a conservative host-wide upper bound.

## 6. Current Rust caller (complete source)

Inspected `src/main.rs`, `artifacts.rs`, `provider.rs`, `verify.rs`, `export-ledger-fixture.mjs`, `Cargo.toml`, `Cargo.lock`, frozen metadata stdout/receipt.

`Cargo.toml` selects `midnight-ledger-v9` at `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` with `default-features = false` and `features = ["proof-verifying"]`. ZKIR is `e82d81d25aabcc5f5092e2bd559083487f577914`. Direct Tokio is `default-features = false, features = ["rt"]`. Checkout HEADs independently match those commits, including old transient `a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1`.

Official `ledger/Cargo.toml` defines optional `mock-verify` and `test-utilities`. Those strings are absent from `Cargo.lock`. The locked `midnight-ledger-v9` dependency list has no `reqwest`, `zkir_v2`, or `zkir_v3`. `ProofVerificationMode::CalibratedMock` is `#[cfg(feature = "mock-verify")]`. Default `WellFormedStrictness` uses `ProofVerificationMode::Real` with balancing, native proofs, contract proofs, signatures, and limits all true.

Unified Tokio disclosure: frozen `metadata-receipt.json` records 391 packages, 391 resolve nodes, 1220 resolve edges, and unified Tokio features including `net`, `fs`, `default`, `mio`, `socket2`. Independent parse of frozen `metadata.stdout` (SHA-256 `37cb2e536ad104b56597c1390b468eb6e5d30ebe9d5c7f6ae6a32784205f20fe`, 1,996,198 bytes) counts packages 391, resolve nodes 391, summed deps 1220. Locked `tokio 1.53.1` lists `mio` and `socket2`. Direct consumer Cargo.toml requests `rt` only. The resolved graph still carries networking features. The wrapper still launches helpers inside `unshare -Urn`. That namespace is the network deny. It is not a claim that Tokio net code is absent from the linked debug binary.

`prepare` is synchronous. It calls `construct(..., None, out)`, requires pay operation `v2/v3/ir` all `None` on incoming initial, expected, and constructed genesis, decodes tagged artifacts with EOF, loads IR via `IrSource::load` (JSON), replays 293 ops / 47 `Popeq` in `QueryContext::query`, sets all eight `CallContext` fields (`own_address`, `tblock`, `tblock_err`, `parent_block_hash`, `caller`, `balance`, `com_indices`, `last_block_time`), partitions through `add_calls`, checks complete financial effects and both transcript parts, compares entire constructed `ProofPreimage` to the frozen preimage, estimates `fees_with_margin(..., 2)`, and records actual `ir.check(p)` with `proof_invocations: 0` and `registered_vk_present: false`. Binding zero is the pre-finalization preimage. Prepare returns before PK/VK/SRS read, `Zkir::prove`, `Transaction::prove`, well_formed, or apply.

`prove` and `verify::run` remain in the same binary. They are future proof/application source. This allocation does not invoke them. `provider.rs` requires a non-zero finalized binding override, one-call budget, real `IrSource::prove`, and dual VK verify. `accept` requires full `TransactionResult::Success`, exact 8990 escrow, UTXO conservation, and replay refusal. Those predicates stay unallocated.

`artifacts.rs` tagged decode refuses trailing bytes. SHA-256 is a manual lowercase encoder. Exclusive create-new writes are fsync'd.

Old unused exporter `export-ledger-fixture.mjs` still counts Popeq with `'popeq'in o` and remains frozen. v5 does not execute it. The same unit-string trap remains in that unused file.

## 7. Handoff and production path

`native-beta-handoff-source-v2-pre-runtime-repair/handoff.ts` versus current `handoff.ts`: 115 lines each, exactly one changed line (99). The patch adds `v!==null&&typeof v==='object'&&` before `'popeq'in v`. No operation, effect, field, head, or preimage assertion was removed or relaxed. `cli.ts`, README, and INPUT bytes are identical to the pre-repair tree. SOURCE-HASHES.json changed only because it pins `handoff.ts`.

`cli.ts` takes six arguments: source path, action, scenario path, signature path, absolute crypto binary, absolute new output. It does not accept a passed candidate.

`handoff.ts` embeds INPUT pin `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476`, imports production `auth.ts` / `json.ts` / `bridge.ts` and Source/6 frontend, calls `verifyAndPrepare`, requires `SignedPreparedUnqualified` with `sourceMatched true`, native `signature_valid true`, four premises, four unbound bindings, `authority_valid null`, `nativeProof NotChecked`, `ledger NotSubmitted`, `ledger_accepted false`. It does not import `LocalSettlementStore` (`atomic.ts`). `simulate` in `bridge.ts` calls `prepareSource6S0Unqualified`, which calls `prepareMil4S0` in `mil4-s0-core-v5.ts`.

Production hashes match the current external map:

- `auth.ts` `bbddda1d5fe9e2148d29fb1983f0a842dfe52d2677402265f17acd72019153e0`
- `bridge.ts` `d4ca083798688a4b1d8e77da1beb28cbc2e04731ea550ba717ff908ef7c695fb`
- `frontend.ts` `6c45599ee9efcc479bbe5a1dcb17c3c42f72630f04f8a55a78ecef6ee5e92db3`
- `json.ts` `e16318504647b6fbd1be4286d1553a6f116da25eb898a518ddb70a822d6e0624`
- Source/6 frontend `f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7`
- Core/5 `855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda`
- Source/6 S0 `1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8`

Public fixture `program.mori` SHA-256 `fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c` matches the frozen projection source digest. Projection `6afe4f734ab85f767eb2578a5b5a8699bfda81de360ce50b8fb9cb9cd82ee381`. `json.parseBoundedJson` is duplicate-safe bounded text. Handoff compares whole Core candidate, Source/6 prestate 10000/0/0, six ordered effects, post 8990/1000/10, all 21 generated ledger fields, effect families, independently assembled 16-field little-endian native head, ordered trace, then official `proofDataIntoSerializedPreimage` versus frozen `good.preimage` (`96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402`, 4316 bytes). Publication uses exclusive create. WASM `ContractOperation` constructor is `new(None, None)`; getter returns undefined when both keys are absent.

Fixed trusted development mappings remain 04/A1/02/03, escrow 10000, stored round 1. Intent ECDSA owner is distinct from NIGHT Schnorr payer (`SigningKey::Schnorr` from `[0x42;32]`). Signed A fee 10 is separate from protocol DUST. Opaque h0/h1 mapping does not authenticate predecessor or history. Fixture recipe: `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, undeployed, block 1000000, NIGHT creation 0 / value 1000000000000, allowance 100000000000000000000, TTL 1000300.

Handoff README freeze prose still says the Rust caller is uncompiled. Actual v4 receipts supersede that sentence. README is not execution evidence.

## 8. Generated kernel, trace, converter

Frozen retained trace `runtime-proof-data.json` SHA-256 `2fb7f8343a5e773820abe9fe7d13f0ebe816a9ad043abb4d794f9450a80f000a`. Independent count of `publicTranscript`: 293 entries = 276 objects + 17 unit strings + 0 nulls. Popeq objects: 47. Unit strings: `member` 6, `neg` 4, `add` 4, `pop` 2, `lt` 1. Official `onchain-vm/src/ops.rs` uses lowercase externally tagged ops; unit variants serialize as strings; `Popeq { cached, result }` serializes as an object. The v4 TypeError `Cannot use 'in' operator to search for 'popeq' in member` is the official mixed encoding hitting the unguarded `'popeq'in v` filter.

Generated contract `index.js` calls `checkRuntimeVersion('0.20.0')`. `pay.zkir` header is `"version": { "major": 3, "minor": 1 }`. Official `IrSource::load` accepts JSON major 3 minor 0..=1. Native prepare uses `IrSource::load` on the JSON IR. Provider tagged-IR load is the prove-mode path.

`proofDataIntoSerializedPreimage` remains unexecuted in v5. v4 died before that call.

## 9. Official API facts used by the candidate

Pinned ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` and ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914`:

- `QueryContext::query` executes ops and charges execution/state costs (`context.rs:923–978`).
- `PreTranscript::split_at` executes guaranteed, clones context, resets effects, executes fallible (`construct.rs:765–814`).
- `ContractCall::prove` supplies `Some(intermediate_call.binding_input(...))` (`prove.rs:367–371`). Tag `verifier-key[v7]` selects `ProofVersioned::V3` (`prove.rs:374–377`).
- `ProofVersioned` has V2 discriminator 1 and V3 discriminator 2 (`structure.rs:294–311`). Candidate verify path requires V3.
- `ContractOperation::new(vk, ir)` sets `v2: None`, `v3: vk`, `ir` (`onchain-state/src/state.rs:902–908`). Prepare `vk=None` yields all None.
- WASM `ContractOperation::new` is `new(None, None)`; `verifierKey` undefined when both absent (`onchain-runtime-wasm/src/state.rs:495–509`).
- `AlignedValue` serde validates alignment fit and normal form (`encoding.rs:450–494`).
- `IrSource::check` is preprocess/`pi_skips` (`ir.rs:76–80`).
- `fees_with_margin` uses synthetic `cost(params, false)` and price-adjustment margin (`structure.rs:1929–1941`).
- `LedgerState::apply` returns unchanged self on guaranteed failure; fallible can retain guaranteed changes (`semantics.rs:1343–1354`).
- `CallContext` has exactly the eight fields compared by prepare (`context.rs:309–318`).

These are source facts. They are not observed native prepare or proof results.

## 10. Actual history (mandatory)

### v3 compile failed

Receipt `native-ledger-v3-build-receipt.json`: phase `build`, exit 101, `produced_binary_sha256: null`, `source_identity_preserved: false`, stop `source identity changed during phase`, elapsed 38.3608 s, peak RSS 1,536,049,152, CPU 73.41 s, target grew 4,515,479,003 → 5,549,887,375. Log SHA is the frozen v3 log. Exactly nine `error[E…]` diagnostics, then `could not compile ... due to 9 previous errors`:

1. E0277 `&mut StdRng: SplittableRng` at `main.rs` `seal(&mut rng)`
2. E0277 same bound, second obligation
3. E0277 `LowerHex` for `Sha256::digest` in `artifacts.rs`
4–8. E0382 moved `parent.binding_commitment` in `verify.rs`
9. E0005 refutable `ProofPreimageVersioned::V2` pattern with `todo!()`

`COMPILE-REPAIR-v1.md` / `.patch` record the source repair: owned `seal(rng)`, manual hex encoder, `clone()` before each `into()`, explicit unsupported-preimage else. Full pre-edit snapshot is `native-ledger-consumer-source-v4-precompile-repair`. Current `main.rs:192` is `seal(rng)`. Current `artifacts.rs` encodes hex by hand. Current `verify.rs` clones the commitment. Current preimage match has an explicit refusal else. Those repairs are now inside the compiled v4 identity.

### v4 compile succeeded

Receipt pin `df9cc75d34d9f24bd0ec94deab880a5db76d243fa76e77902829c613d095e03b`. Phase `build`, exit 0, `stop_reason: null`, `source_identity_preserved: true`, `binary_identity_preserved: true`, `adapter_manifest_sha256` equal to compiled-manifest pin `a8a8825…`, `produced_binary_path` `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer`, `produced_binary_sha256` `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`. Elapsed 20.2709 s. Peak sampled RSS 2,993,246,208. CPU 30.16 s. Final target 7,501,078,730. Free 22,531,923,968. Log: `Finished dev profile ... in 19.03s`. Attempt reservation `native-ledger-v4-build-attempt.json` still exists (SHA-256 `6fd2760be58d4f15c0e51363aa3354b4c5b3353dc9161150098ef619e6bff715`). This proves compilation of that caller. It does not prove native preparation, proof, or ledger acceptance.

### Actual binary, hashed, not executed

Path exists, regular file, mode 0755, size 624,060,592 bytes. SHA-256 `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`. `file(1)`: ELF 64-bit LSB pie executable, x86-64, dynamically linked, with debug_info, not stripped, BuildID `24c3b135d3724c6b7badea6363b55f99d2a1f00e`. The binary was not executed.

### v4 handoff failed

Receipt `native-ledger-v4-handoff-receipt.json`. Phase `handoff`, exit 1, `stop_reason: null`, source and binary identity preserved, helper receipt/artifacts null. Elapsed 1.4879 s. Peak RSS 13,737,984. Sampled CPU 0 (one sample at 0.002 s). Target unchanged at 7,501,078,730. Attempt `native-ledger-v4-handoff-attempt.json` still exists (SHA-256 `47e06e603d4953badeba7fe607cf7e553c9e8d58561f6962566613cadfe5f9dc`). Output directory `native-ledger-v4-keyless-handoff` is absent. Prepare did not run.

Log, Node.js v24.21.0, `handoff.ts:99`:

```
TypeError: Cannot use 'in' operator to search for 'popeq' in member
```

Stack is the Popeq `filter` inside `prepareExactNativeTransfer`, then `cli.ts:7`. The thrown name is `TypeError`, not `AssertionError`. Assertions before line 99 therefore did not throw. That is exception control-flow inference, not independent per-assertion instrumentation. Preimage conversion, exclusive output, and native prepare were not reached.

## 11. v5 execution state and resource headroom

No v5 phase has executed. Absent: `native-ledger-v5-handoff-attempt.json`, `native-ledger-v5-handoff-receipt.json`, `native-ledger-v5-handoff.log`, `native-ledger-v5-prepare-attempt.json`, `native-ledger-v5-prepare-receipt.json`, `native-ledger-v5-prepare.log`, `native-ledger-v5-keyless-handoff`, `native-ledger-v5-keyless-preparation`, `native-ledger-v5-keyless-prepare-config.json`.

Read-only refresh of current disk, no rebuild. Target bytes are the per-name `st_size` sum used by the v4 receipt (hard links counted per directory entry). `du -sb` unique size of the same tree is 6,336,744,232.

| Quantity | Bytes |
| --- | ---: |
| target | 7,501,078,730 (12,799 files) |
| cache `/home/charl/.cargo` | 2,659,034,456 (83,112 files) |
| free | 22,519,459,840 |
| target versus v4 final | 0 |
| cache versus v4 final 2,933,611,498 | −274,577,042 |
| remaining under 8 GiB cap | 1,088,855,862 |
| free minus 10 GiB floor | 11,782,041,600 |

Target still equals the recorded after-build 7,501,078,730. Cache file count is unchanged at 83,112; content sum is smaller than the v4 receipt. Helpers are not expected to grow the Rust target. The 8 GiB total cap remains. Remaining total-cap headroom is about 1.01 GiB, which is inside the +2 GiB incremental stop so the total cap would fire first if target grew.

v5 helper RSS/AS is 2 GiB. Observed v4 handoff RSS was 13.7 MiB (Node). Prepare would exec the 624,060,592-byte debug ELF. Prepare RSS is unmeasured. The wrapper stops on sampled RSS/AS. One attempt is consumed on stop or failure.

## 12. Compact attempts and R3 debt

Four Compact compile receipts remain:

| Attempt | Exit | Source SHA-256 |
| ---: | ---: | --- |
| 1 | 255 | `bcba78890e69f624291f4f4f36a6d100983e6a67c87bccfdda6857b3371c2faf` |
| 2 | 255 | `fc2d022b22022e04734186520a8d41e3035d1b4cf3423339bb5da44f2e10189a` |
| 3 | 0 | `781e5050ce95434607fb1bc6798275bffec4c31b2133e1e031c90c395597ad14` |
| 4 | 0 | `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db` |

Attempt 4 source equals frozen `source-attempt-4.compact` and the current external pin. No fifth Compact invocation is in this allocation.

R3 k17 exhaustion remains open repository debt (`AGENTS.md` current constraints; native-path-freeze / F0 still listed in guarded status remaining stages). This allocation does not retry R3 and does not raise k.

## 13. Preview research facts (not this allocation)

Read-only Preview observation in `PREVIEW-COMPATIBILITY-20261001.md`: ledger 8.1.2, protocolVersion 1000300, transactionVersion 3, same-block RPC/indexer hash match. Matching published runtime/ledger source in `COMPATIBILITY-SOURCE-LINKAGE.md` supports V2 proof / V3 operation / IR 2.0. This candidate's pinned ledger-v9 plus generated IR 3.1 / Compact runtime 0.20.0 requires V3 proof / V4 operation / IR 3.1. That is conditional source incompatibility given live node/build correspondence. No WASM attestation of the live node and no candidate network rejection were observed. This audit acquired no network. No silent format downgrade and no fifth compile are allocated.

## 14. Finding classification

### Blocking even as diagnostics

None that refuse the described two-phase helper allocation on source or correspondence grounds. The v4 Popeq TypeError is the defect this helper repair targets. Unused exporter still contains the unguarded count; v5 does not execute it.

### Legitimate bounded uncertainty

- v4 whole Source6/Core/21-field/effects/16-head/trace assertions are control-flow inferences from a TypeError at line 99. They are not independently instrumented successes. v5 must re-execute them.
- Native decode, query, partition, eight-field context, full preimage equality, fee estimate, and `Zkir::check` are unperformed.
- WASM keyless operation serialize versus native `ContractOperation` decode is source-aligned (`new(None,None)`) and unexecuted.
- Unified Tokio graph includes `net` while consumer Cargo.toml requests `rt`. Network deny is the user namespace, not feature absence in the linked debug binary.
- Global RX deltas are host-wide. Historical v4 handoff RX 64,551 occurred under `unshare -Urn`.
- Prepare RSS for a 624 MiB unstripped debug ELF under 2 GiB AS is unmeasured. Wrapper stop exists.
- Target remaining to 8 GiB is 1,088,855,862 bytes. Helpers should not grow target. The cap is still enforced. Current cargo cache is 274,577,042 bytes smaller than the v4 receipt at the same 83,112 file count.
- Parent hashing of 39+260+240+240 files and a 7.5 GiB target walk sits outside child wall. v4 handoff total elapsed was 1.49 s including supervisor.
- Consumer manifest and handoff README contain stale compile/unexecuted sentences. Hash maps and receipts are the authority.
- Disk samples at 5 s and CPU/RSS at 0.1 s record overshoot. v4 handoff CPU sample was 0.

### Future proof / acceptance (unallocated)

`prove`, `provider.prove`, `Transaction::prove`, SRS body, PK/VK, keygen, independent `verify::run`, `well_formed`, `apply`, fault matrix, public transactions, Preview settlement, generic compiler correspondence, property/intent/transition/history, PCD, authenticated genesis/custody/time, and fifth Compact/R3 retry. Inspected as source. Not accepted. Not allocated.

## 15. Votes

These are resource votes on the frozen v5 helper candidate and the observed v4 compile/handoff results. They are not publication of a v5 result.

**Handoff — APPROVE-BOUNDED.** Allocate one new exclusive durable v5 handoff attempt of the current `handoff.ts` (`add4ad0b…`) under the v5 wrapper (`371df382…`), 60/120/2 GiB, `unshare -Urn`, fresh `native-ledger-v5-keyless-handoff`, reusing the actual v4 binary `034ed49b…` after the correspondence checks in sections 3–5. The repair is the object guard required by the official mixed 276-object + 17-string transcript. This is not whole-handoff acceptance in advance.

**Prepare — APPROVE-BOUNDED as a gated second phase.** Allocate one new exclusive durable v5 prepare attempt only after a frozen successful current-manifest/runtime v5 handoff receipt, using the independently closed five-artifact + literal fixture config, exclusive root config fsync, and the same compiled binary. Handoff failure consumes the handoff attempt and blocks preparation. This is not native-prepare success in advance.

**Bundle / exact candidate — APPROVE-BOUNDED helper-only.** The exact candidate is current manifest `bf9db118…`, wrapper `371df382…`, runtime `68afbf29…`, proposal `d232c5ec…`, compiled map `a8a8825c…`, build receipt `df9cc75d…`, binary `034ed49b…`, handoff `add4ad0b…`, SOURCE-HASHES `20a8aaf7…`, INPUT `80860593…`. No v5 build/Cargo allocation. Old v4 attempts remain consumed. Two fresh actual-result audits remain mandatory after execution before any publication of a narrow v5 result.

**Actual compiled scope — accept as compile identity only.** The v4 receipt and rehashed ELF establish that this caller compiled. Refuse treating that compile as prepare, proof, well_formed, apply, or ledger acceptance.

**Actual result scope — refuse whole-handoff success.** The v4 handoff failed at Popeq-count TypeError. Prior assertions passed only by exception control-flow inference. Preimage conversion and export were not reached. No output directory. No v5 result exists. Positive partial observations do not accept whole handoff, proof, or ledger.

**Refuse.** SRS/PK/VK/keygen/`Transaction::prove`/WF/apply/Preview/generic compiler/PCD/authenticated genesis/time/fifth Compact/R3 retry/old-approval reuse/silent format downgrade.

## 16. Remaining open predicates

Trust fixture 04/A1/02/03 escrow 10000, NIGHT/round/time/owner role are not authenticated. ECDSA owner remains distinct from Schnorr payer. A fee 10 remains separate from DUST. Opaque heads/native hash mapping is not history. Generic compiler, property, intent, transition, history, PCD, authenticated genesis, and time remain open. Maintainer resource controls do not become public developer compile/prove/deploy prerequisites.

Both current resource votes are required before allocation. Fresh actual-result audits are required before publication. This report is one vote.

## 17. Work this session did not perform

No v5 phase. No wrapper execute. No Cargo metadata/fetch/build. No Node/TS candidate run. No binary execute. No nativecheck/prepare. No SRS/keys/parameters/proof/WF/apply/wallet/transaction/network. No other auditor files. No file written except this report.
