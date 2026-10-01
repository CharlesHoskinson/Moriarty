# Native ledger v4: independent current source and resource audit

2026-10-01. Fresh full-current source and resource inspection of the exact v4 caller plus production Beta handoff candidate. Specified-only. No other auditor report, verdict or disposition was read, including v3 resource reports. Historical raw compile diagnostics, receipts, the precompile snapshot and the compiler-repair patch were inspected as evidence, not as votes.

This report is the only file written. No wrapper import or execution, cargo metadata/fetch/build, Node/TS candidate execution, native check/prepare, parameter/key/SRS/proof, well_formed, apply, wallet or network transaction was performed. `ast.parse` and local byte-hash inspection were used.

## Returned identity receipt

Requested: Grok 4.6 high.

Actual returned host identity from this session record:

| Field | Value |
| --- | --- |
| `current_model_id` | `grok-4.6` |
| `reasoning_effort` | `high` |
| session id | `01a0f67f-1dc1-7f71-bdca-63bd01822d66` |
| `GROK_SESSION_ID` | `01a0f67f-1dc1-7f71-bdca-63bd01822d66` |
| agent id | `ag1.d716b06116814ee49b476a37306dc838` |
| attempt id | `at1.6b83679c113148bb849bec6d829db05d` |
| request id | `4cb370ed-0a97-4269-a7b9-d6dc3426e627` |
| agent name | `grok-build-plan` |
| session kind | `headless` |
| created_at | `2026-10-01T08:05:20.967432103Z` |
| sandbox_profile | `off` |
| announced product string | Grok 4.6 released by xAI |

No silent substitution occurred. This identity does not approve product acceptance.

## Scope and method

Contracts read: `NATIVE-LEDGER-INDEPENDENT-EXPECTATIONS.md`, `NATIVE-LEDGER-PREPARE-EXPECTATIONS.md`, `NATIVE-BETA-HANDOFF-INDEPENDENT-EXPECTATIONS.md`. Allocation read: `NATIVE-LEDGER-V4-RESOURCE-PROPOSAL.md`. Authoritative freeze read: `NATIVE-LEDGER-V4-REVIEW-FREEZE.json`. Wrapper-change note in the freeze set was hashed and read as candidate documentation.

Inspected current Rust `main.rs` / `artifacts.rs` / `provider.rs` / `verify.rs` / `export-ledger-fixture.mjs` / `Cargo.toml` / lock / metadata receipts; current `native-beta-handoff/{cli.ts,handoff.ts}` and production `auth.ts` / `bridge.ts` / `json.ts` / Source6 frontend / Core5; generated kernel `output/contract/index.js`, retained projection/trace/preimage, official converter/WASM runtime files as pinned; official ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` and ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914` APIs that control construction, source check, binding, fees, verification and application.

Startup: Moriarty AGENTS plus `plugins/moriarty-dev/skills/develop/SKILL.md`. Guarded status in `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` reports capability `SP01.6 loan-swap-subset`, unresolved operational history, stale admission, missing current accounting/live resource evidence, `nextAction` `sp01-loan-report`, and no pending transactions. This audit creates no campaign and no execution authority.

## Independent hash verification

Freeze file SHA256 `cf92e78702fb3df29f34e6dd905b1a1c12ece349dc5da4321e068e3782fe6c51`.

| Path | Expected | Actual | Result |
| --- | --- | --- | --- |
| `native-ledger-consumer/manifest.json` | `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb` | same | MATCH |
| `run-native-ledger-v4-bounded.py` | `cb427c75cf6f53d54c4cc240a5e09b2cf23e511fac5e58d90da771eb5ce701a2` | same | MATCH |
| `NATIVE-LEDGER-RUNTIME-INPUTS-V4.json` | `db7fde2b7d988543021786874da6d9a14a604f15be53598ca2ba446cf7a7d9e5` | same | MATCH |
| `NATIVE-LEDGER-V4-RESOURCE-PROPOSAL.md` | `faa03976aab1174273b0fef9ba960eea047ec71728edf8817e00e6b7a22d5bd4` | same | MATCH |
| `NATIVE-LEDGER-V4-WRAPPER-CHANGES.md` | `7281ba3d1542d354db3b79e522620fea245b704a0e11110fe5c0cd161f1d129a` | same | MATCH |
| `native-beta-handoff/SOURCE-HASHES.json` | `bfebf8e7e117e8029dfb667aabe00df1836a776c955b5129e1512c7cb24bf05f` | same | MATCH |
| `native-beta-handoff/INPUT-SOURCE-HASHES.json` | `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476` | same | MATCH |
| `native-beta-handoff/handoff.ts` | `d17f542d5afa2cadd91fa721ec48bcd36f4ec091ab730b33c1aac9c177e61b38` | same | MATCH |

Current consumer manifest `sha256` map: **39/39 internal MATCH**, 0 missing. `external_inputs_sha256`: **260/260 MATCH**, 0 missing. Current `SOURCE-HASHES.json` rust/docs/patch entries MATCH the files they name, including `src/main.rs` `2d9ddbc8e7b4919e150c5ce7543c62d7225278fcd19dfd8e6bcf934d8add297b`, `src/artifacts.rs` `4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7`, `src/verify.rs` `8e9c59d0d80efbeb28a5d2f84d899be1dc5d732833178d4025656e69c8fe3c8b`, provider unchanged `1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f`. Consumer `INPUT-SOURCE-HASHES.json` 25/25 MATCH.

Handoff `SOURCE-HASHES.json` 4/4 MATCH. Handoff `INPUT-SOURCE-HASHES.json` **226/226 MATCH**.

Runtime V4: physical `files_sha256` **240/240 MATCH**, walk file-set exact. Kernel-path `kernel_files_sha256` **240/240 MATCH**. Relative path sets equal. Corresponding hashes equal. `kernel-prototype/node_modules` is a symlink whose real path is `compiler-probe/runtime/node_modules`. `kernel_root_is_symlink` true, `kernel_root_resolved` and claimed `symlink_resolution` match live `realpath`. Package-lock SHA256 `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e` MATCH. Runtime record also pins the three freeze manifests above.

Public caller fixtures at `packages/moriarty-beta/examples/signed-intent/transfer-ecdsa-wallet`:

| File | SHA256 | Consumer manifest | Equals projection |
| --- | --- | --- | --- |
| `program.mori` | `fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c` | MATCH | MATCH `projection.source` / `source_sha256` |
| `scenario.json` | `12928a795bba2435140fc22c0d33f039da3b04c8de6c244d0c4da338900f0430` | MATCH | MATCH `projection.scenario` |
| `signature.json` | `116c238a8b18638dc282bc8efb323926ee440fe506aa2eaff0a37d50bf02969f` | MATCH | MATCH `artifact.signatureHex` and `statement` |

Those three paths are parent-manifest pins. Handoff authority pins `projection.json` `6afe4f734ab85f767eb2578a5b5a8699bfda81de360ce50b8fb9cb9cd82ee381` and compares caller bytes to that retained relation. Wrapper later requires receipt hashes to equal the public-fixture pins.

Pinned kernel/trace/preimage:

| Artifact | SHA256 |
| --- | --- |
| `kernel-prototype/output/zkir/pay.zkir` | `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae` JSON `version.major=3` |
| `native-adapter-successor-v3/good.preimage` | `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402` |
| `native-adapter-successor-v3/runtime-proof-data.json` | `2fb7f8343a5e773820abe9fe7d13f0ebe816a9ad043abb4d794f9450a80f000a` 293 ops, 47 `popeq` |
| `kernel-prototype/source-attempt-4.compact` | `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db` identical to `fixed-transfer.compact` |
| installed `moriarty-midnight-crypto` | `42fa598d8b54334650f60bc13428bff3d79bb240a5bb0c203fc84684b5a30339` executable mode 0755 |

Official checkouts: ledger HEAD `9a8777c4d035fc7f38ae286bcf5f8656668efd9f`, ZKIR HEAD `e82d81d25aabcc5f5092e2bd559083487f577914`.

Wrapper `ast.parse` succeeded (254 lines). No v4 attempt, receipt, log, config, handoff output or preparation output exists. Produced `beta-native-ledger-consumer` binary is absent.

## Live resource measurement

Wrapper-equivalent file-size walk, same method the supervisor uses:

| Quantity | Live bytes | GiB | Gate |
| --- | --- | --- | --- |
| shared target `/home/charl/research/moriarty-crypto-2026-09-30/target` | 5,549,887,375 | 5.169 | total ≤ 8 GiB |
| cache `/home/charl/.cargo` | 2,933,611,498 | 2.732 | incremental ≤ 1 GiB |
| free on target filesystem | 23,865,229,312 | 22.226 | ≥ 10 GiB |
| remaining to 8 GiB total | 3,040,047,217 | 2.831 | incremental +2 GiB fits |

Target and cache byte counts are identical to the immutable v3 build receipt finals. Free is slightly lower than that receipt (`23,878,524,928`). Headroom still satisfies total ≤ 8 GiB and free ≥ 10 GiB. Incremental +2 GiB from the current target lands at 7.169 GiB.

Host `MemAvailable` 28,697,132 kB (~27.4 GiB). Build RSS/AS 4 GiB and helper RSS/AS 2 GiB fit. Node `v24.21.0`, rustc `1.98.1`, cargo `1.98.1`, Python `3.14.4`. Sampling limitations recorded in the wrapper remain: 0.1 s RSS/CPU/RX, 5 s disk, global non-loopback RX, SIGKILL/power-loss cannot run `finally`.

v3 first build used 38.36 s wall, 73.41 s sampled group CPU, 1,536,049,152 peak RSS against 600/1200/4 GiB. Those observations bound the *prior* compile of this shared target. They do not freeze a v4 result.

## Immutable v3 compile evidence

`native-ledger-v3-build-attempt.json` SHA256 `a7568059729c5923d3cd9aec10b2091a72171c9bb9114c47028ee6b5d194abab`, wrapper then `2ce2ae7b80b4930de7a32b9093b9745b0ab8933fdad557703f94ce59c89a340b`, manifest then `8e064acbb067080792e8bc27c1f179aa138e33c1a8d9ebb7e1ae47f7b66d1b38`.

`native-ledger-v3-build-receipt.json` SHA256 `c7d40c0f9311c5f2ce48e6cbdca1f3a1d52e6526197e586b89248c76d0e09758`: exit 101, `produced_binary_sha256` null, `source_identity_preserved` false, `stop_reason` `source identity changed during phase`, runtime pin then `b774ce2388d2625d501b038276f1688d5acb6e9f0fc27498682a5715a1699ac3`. No export/prepare ran.

`native-ledger-v3-build.log` SHA256 `510780b677203a9ece99569b55cfb671d2e07a971516db375a0f0bb73d6926fb` ends `could not compile ... due to 9 previous errors`:

1. `src/main.rs:192` `Transaction::seal(&mut rng)` — `&mut StdRng: SplittableRng` / `SeedableRng` not satisfied (two E0277 notes, rand_core 0.6 vs 0.9).
2. `src/artifacts.rs:2` `format!("{:x}", Sha256::digest(bytes))` — digest array lacks `LowerHex`.
3. Five `E0382` moved `parent.binding_commitment` (`PureGeneratorPedersen` not `Copy`) in `src/verify.rs`.
4. `src/main.rs:143` refutable `ProofPreimageVersioned::V2` pattern (`E0005`, official type `non_exhaustive`).

The compiled tree is preserved at `native-ledger-consumer-source-v4-precompile-repair`. That snapshot’s `manifest.json` is exactly the v3 attempt pin `8e064acb…`. Current versus snapshot: 32 files identical; only current adds `COMPILE-REPAIR-v1.md` / `.patch`; content changes are `src/artifacts.rs`, `src/main.rs`, `src/verify.rs`, plus hash inventories (`manifest.json`, `SOURCE-HASHES.json`, `INPUT-SOURCE-HASHES.json`). Provider, `Cargo.toml`, `Cargo.lock`, baseline lock, exporter and metadata graph files are unchanged.

Independent post-failure fact: the v3 receipt’s `source identity changed` predicate is false for source/runtime bytes of the compiled snapshot. `run-native-ledger-v3-bounded.py:182-186` hashes the produced binary inside the same `try` as source/runtime `verify()`, so a missing binary after exit 101 is recorded as source mutation. v4 wrapper splits `source_identity_preserved` from `binary_artifact_status` / `binary_identity_preserved`, and a failed build leaves `produced_binary_sha256` null without adopting a stale target executable.

`SOURCE-HASHES-v3.json` still names an older `src/main.rs` (`d34c958a…`) that matches neither the compiled snapshot (`a6f9c341…`) nor current repair (`2d9ddbc8…`). That inventory is historical documentation drift from before the compiled snapshot. The v3 *attempt* bound the snapshot manifest, not that stale main hash.

Repair patch SHA256 `8ad3ae37ba30da0732ef10278558c40ee097b23f9ed18d05e8539963233debea` matches current files: owned `seal(rng)`; explicit unknown-preimage refusal; per-use `binding_commitment.clone()`; exact digest-byte lowercase hex without a new hex crate. Official `base-crypto/src/rng.rs` implements `SplittableMarker` for owned `StdRng` and a blanket `SplittableRng` for that owned `SeedableRng`. Official `structure.rs` marks `ProofPreimageVersioned` non-exhaustive. These are source-feasible replies to the nine diagnostics. They are uncompiled.

## Metadata, lock, features

Current `metadata-receipt.json`: exit 0, packages 391, resolve nodes 391, resolve edges 1220, `before_lock_sha256` `2c07881f96e2a19fde1e21b0421f8173dee4c44885482a666081ffdc4866c6fd` (baseline file), `after_lock_sha256` `b0cddb391929e964c82b1587675d9bd5079413e8461b2f2ac2b5e72b8de9b11e` (current `Cargo.lock`). `unified_tokio_features` include `net`, `default`, `fs`, `rt`, `sync`, `time`, `macros`.

`metadata-tuple-diff.json`: removed 1 (old proof-consumer root), added 39, **unchanged 352**. Baseline lock has 352 external tuples; those 352 remain.

Fail history, lock unchanged: `metadata-v1/metadata-receipt.json` exit 101, empty stdout SHA256 `e3b0c442…`; `metadata-v2/metadata-receipt.json` exit 101, empty stdout, different stderr. Current success does not rewrite those receipts. Metadata is not compilation.

Consumer `Cargo.toml`: ledger `default-features = false`, features `["proof-verifying"]`; tokio `{ version = "=1.53.1", default-features = false, features = ["rt"] }`; **no** `mock-verify`, **no** `test-utilities`. Current `Cargo.lock` contains **zero** `mock-verify` and **zero** `test-utilities` strings. Official ledger `Cargo.toml` defines those features; they are not enabled here. Unified Tokio in the 391-node graph includes network crates because other packages enable them. Direct consumer features remain `rt` only. Execution namespace is `unshare -Urn`.

## Current Rust caller

`PrepareConfig` is `deny_unknown_fields` and contains only IR, initial/expected contract, runtime, retained preimage and fixture. PK/VK/SRS fields are absent. `require_prepare_operation` demands exactly one `pay` operation with `v2`/`v3`/`ir` all `None` on incoming initial, incoming expected, and constructed genesis when no VK is supplied. `prepare` calls `construct(..., None, ...)` and prints `NATIVE_PREPARATION_SOURCE_CHECK_OK; no VK/PK/SRS/proof/ledger acceptance`.

Construction: decode tagged artifacts with EOF; `IrSource::load` on JSON IR (`pay.zkir` is JSON major 3); reject `k()!=17`; genesis trust label `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`; contract address `HashOutput([4;32])`, A1 `HashOutput([0xa1;32])` escrow 10000, recipients `[2;32]` / `[3;32]`; Schnorr development `SigningKey::from_bytes(&[0x42;32])` for NIGHT, distinct from the ECDSA intent owner; generationless NIGHT UTXO at creation 0, block 1000000, value 1_000_000_000_000, allowance 100_000_000_000_000_000_000, TTL 1000300; `QueryContext::query` of the supplied 293 ops / 47 Popeq; `financial_outputs` requires unshielded output 1010 and spends 1000+10 with no shielded/mint/call families; `PrePartitionContractCall` / `add_calls::<ProofPreimage>` / partition; registration-only dust (`spends: []`, `dust_address: Some`); full native `ProofPreimage` equality against the frozen artifact; eight CallContext fields (`own_address`, `caller`, `balance`, `tblock`, `tblock_err`, `parent_block_hash`, `com_indices`, `last_block_time`); `fees_with_margin(..., 2)` preflight; `ir.check(p)` recording skips; `proof_invocations: 0`. Exclusive create-new writes and `fsync`.

Official APIs used by this path:

- `ContractOperation::new(vk, ir)` sets `v2: None`, `v3: vk`, `ir` (`onchain-state/src/state.rs:903-908`). `new(None, None)` is all-None.
- WASM `ContractOperation::new()` calls that constructor; `verifierKey` getter returns `undefined` when both keys absent (`onchain-runtime-wasm/src/state.rs:497-509`).
- `PreTranscript::split_at` executes guaranteed ops, clones context, resets effects, executes fallible (`construct.rs:765-814`).
- `IrSource::check` is `preprocess(...).pi_skips` (`zkir/src/ir.rs:76-80`). Binding is copied from the preimage into public inputs (`ir_vm.rs:258,896,934-944,1032`); it is not forced to zero.
- `fees_with_margin` is synthetic `cost(params,false)` plus price-adjustment margin (`structure.rs:1929-1941`), not consumed protocol fees.
- `CallContext` is eight fields with `Default` (`context.rs:308-318`). `QueryContext::new` starts from that default then the caller overwrites listed fields; `com_indices` stays default unless official `context()` fills it.
- Default `WellFormedStrictness` is Real proof-verifying (`verify.rs:444-471`). v4 must not invoke it.

`prove` / `verify` / `accept` / provider `Zkir::prove` remain in the same binary source. v4 command is `prepare` only. Ledger `ContractCall::prove` later inserts Noops and passes `Some(intermediate_call.binding_input(binding_commitment))` (`prove.rs:356-371`). Preparation equality is pre-finalization binding-zero. That is the specified prepare boundary, not a proven statement.

## Actual Beta handoff caller

`cli.ts` SHA256 `9c04b0cfface408224fdd88247dfd52b38c9ad9f50853dccd5f7c60360332591` is unchanged from `native-beta-handoff-source-v1`. Six arguments: caller source path, action, scenario path, signature path, absolute crypto binary, absolute new output. Fatal UTF-8, 65536-byte input bound. Dynamic import of `handoff.ts`.

`handoff.ts` versus v1 differs by one line: embedded authority pin refreshed from `2b63cf24…` to `80860593…` after `src/main.rs` compiler repair. Handoff INPUT map is otherwise identical; the only changed entry is current `native-ledger-consumer/src/main.rs`.

Actual consumer path: production `verifyAndPrepare` with installed Rust verifier; refuse unless `SignedPreparedUnqualified`, `sourceMatched true`, `signature_valid true`, premises `canonical-intent-signature` / `snapshot-to-head` / `head-extension` / `atomic-ledger-compare-and-consume`, unverified bindings `agreement-id` / `selected-program` / `asset-scale` / `authenticated-predecessor`, `nativeProof NotChecked`, `ledger NotSubmitted`, `ledger_accepted false`, `authority_valid null`. No `LocalSettlementStore` import or `settle` call. Production `atomic.ts` still contains that store; this entry point does not invoke it.

Whole Source6 prestate and Core5 candidate are compared to the retained projection, including ordered effects Debit 1010 / Credit 1000 / Credit 10 / UseAllowance 1010 / UseReplay / AdvanceHead and post 8990/1000/10. Official generated `Contract` constructor/pay uses checked message and signature. WASM `ContractOperation` is constructed empty; `verifierKey` asserted `undefined`. All 21 generated `ledger()` getters are present in `kernel-prototype/output/contract/index.js` and checked: `ownerKey`, `sourceDigest`, `ownerProgramDigest`, `usedNonce`, `head`, `revision`, `trustedRound`, `ownerBalance`, `recipientBalance`, `feeBalance`, `workRemaining`, `workSpent`, `allowanceRemaining`, `allowanceSpent`, `assetColor`, `recipientAddress`, `feeAddress`, `lastDebit`, `lastRecipientCredit`, `lastFeeCredit`, `lastAllowanceUse`. Effect families, output 1010, spends 02=1000 and 03=10. Ordered 293/47 trace equals retained JSON; serialized preimage equals `good.preimage`.

Heads: `sha256("local-fixture:h0")` = `9e4601c9af94102208bd6bdea6693cbf5a9638f6d6b9fa716eae540443cc30e4` = projection `beforeHead` = wrapper `nativePre`. Retained trace `output.value[0].bytes` = `4e87ddfda49bafabddab1a05cbdd0538735744a318dda80a211075ebb343b09e` = wrapper `nativePost`. Sixteen little-endian persistentHash inputs are assembled in source. Mapping `authenticates_predecessor: false`. Opaque Beta `h0`/`h1` are not claimed equal to native bytes.

Outputs after comparisons, exclusive directory: `initial-contract.tagged`, `expected-contract.tagged` (storage-only; escrow remains 10000), `runtime-native.json`, `registered-pay-operation.tagged`, `prepare-config.json`, `receipt.json` status `NativeFixturePreparedUnqualified`, no proof / well_formed / apply / ledger acceptance. Helper config is data. Root independently rebuilds the five-artifact closed config plus literal fixture and compares parsed helper JSON to that recipe. Root writes its config with `xb` and `fsync` only after the prepare reservation.

Node 24 type stripping of this production TS graph, WASM load, and 60 s wall are unexecuted. `enum` / `namespace` / parameter properties were not found in the imported production `.ts` files; imports use `.ts` extensions. That is source compatibility, not a runtime result.

Four Compact attempts `source-attempt-1.compact` … `source-attempt-4.compact` remain in `kernel-prototype`. Wrapper contains no Compact invocation.

## v4 wrapper phases and caps

`run-native-ledger-v4-bounded.py` phases `build` → `handoff` → `prepare`. Exclusive durable `native-ledger-v4-{phase}-attempt.json`. No deletion/reset of the v3 attempt. No automatic retry. Offline `unshare -Urn`. Existing shared target only. Jobs 2.

| Phase | Wall | Group CPU | RSS/AS | Child |
| --- | --- | --- | --- | --- |
| build | 600 s | 1200 s | 4 GiB | `cargo build --locked --offline --bin beta-native-ledger-consumer` |
| handoff | 60 s | 120 s | 2 GiB | `node native-beta-handoff/cli.ts` + public fixtures + frozen crypto binary + fresh `native-ledger-v4-keyless-handoff` |
| prepare | 60 s | 120 s | 2 GiB | frozen binary `prepare CONFIG PIN_SHA native-ledger-v4-keyless-preparation` |

Shared: incremental target 2 GiB, incremental cache 1 GiB, total target 8 GiB, free floor 10 GiB, combined handoff+prepare directories 32 MiB, per-process file 2 GiB. Build success required for handoff; frozen handoff success required for prepare. Binary SHA256 frozen after a successful build without running it; helpers re-hash before and after. Failed build cannot adopt a stale executable.

Prepare config is computed from fixed IR/preimage pins plus observed handoff artifact digests, compared to helper `prepare-config.json` as data, then created exclusively after reservation. Child is launched with that exact digest. Postcondition reads `preparation.json` for `proof_invocations==0`, `preimage_equal true`, `registered_vk_present false`, `native_check=="successful actual Zkir::check"`. Decode/effects/eight-field context/full preimage/fee preflight/all-None operation are enforced in the Rust `construct` path before that receipt.

## Findings

### Blocking even as a bounded diagnostic

None on the exact current candidate. Identity pins hold. Live disks satisfy the stated ceilings. The wrapper does not reset v3, does not invoke Compact, does not enable mock-verify/test-utilities, and does not treat a missing produced binary as source mutation. The handoff source is an actual production `verifyAndPrepare` consumer. Prepare config has no PK/VK/SRS. Unqualified status and null authority are demanded by helper source and parent.

### Legitimate bounded diagnostic uncertainty

These may consume the allocated attempt honestly. They are not grounds to demand future-proof work before the diagnostic compile.

1. Remaining rustc/API/serde errors after the nine repaired diagnostics, including JS runtime JSON versus native `Op`/`AlignedValue`/`ProofPreimage` decode.
2. Node 24 type stripping, dynamic production `.ts` imports, Compact WASM class identity, and first-run handoff wall/CPU/RSS.
3. Native `CallContext` equality if official `context()` fills `com_indices` or caller differently from the replay assignment.
4. Constructed versus frozen preimage equality, partition effects, fee estimate versus allowance/availability, and actual `IrSource::check` skips.
5. Incremental target growth if cargo rebuilds more of the already-populated 5.169 GiB tree than the +2 GiB cap. v3 growth was ~1.03 GiB while compiling many crates; a similar rebuild could approach the cap.
6. 0.1 s / 5 s sampling overshoot and global RX.

A compiler, API, serde, trace, preimage, context or fee refusal is a useful scoped result. It must not be replaced by a host flag, relaxed financial field, changed frozen preimage, mock proof or unsafe verification.

### Future proof / acceptance restrictions

These remain closed after a successful bounded run.

- No SRS body, PK, VK, keygen, native financial proof, `Transaction::prove`, default strict `well_formed`, `LedgerState::apply`, wallet import or public transaction.
- Trusted public development genesis only: undeployed address `04`, A1 color, destinations `02`/`03`, escrow 10000, NIGHT recipe, constructor `trustedRound` 1. Not authenticated deploy, mint, custody or consensus time.
- ECDSA intent owner and Schnorr NIGHT payer are distinct roles.
- Opaque Beta heads and native persistentHash bytes are a declared mapping, not predecessor/history authentication.
- Signed A application fee 10 is separate from protocol DUST fees. `fees_with_margin` is not consumed-fee evidence. `feeCap` 10 does not bound DUST.
- Preview observed ledger 8 versus pinned ledger 9 compatibility is unknown here. Compact 0.35 targets ledger 9. No public Preview transaction is allocated.
- Generic compiler correspondence, contract properties, intent refinement, transition validity, history compliance and native PCD remain open.
- Source feasibility of binding override and generationless registration is not execution authority.
- `export-ledger-fixture.mjs` remains in the consumer tree and is not the v4 handoff path.
- Prove/verify modes remain in Rust source without v4 permission.

## Votes on the exact current candidate

Candidate identity: consumer manifest `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb`, wrapper `cb427c75cf6f53d54c4cc240a5e09b2cf23e511fac5e58d90da771eb5ce701a2`, runtime V4 `db7fde2b7d988543021786874da6d9a14a604f15be53598ca2ba446cf7a7d9e5`, proposal `faa03976aab1174273b0fef9ba960eea047ec71728edf8817e00e6b7a22d5bd4`.

| Phase | Vote | Meaning |
| --- | --- | --- |
| build | **approve-bounded** | Allocate one exclusive v4 offline locked compile at 600/1200/4 GiB, 2 jobs, existing target. Compiler failure consumes the attempt and blocks dependents. |
| handoff | **approve-bounded** | Only after frozen successful build and binary hash. 60/120/2 GiB. Node/TS/WASM/API failure is a legitimate diagnostic. |
| prepare | **approve-bounded** | Only after unchanged build plus hash-frozen handoff artifacts. Root-closed config, `xb`+fsync after reservation, actual native decode/check path, zero proofs. |
| bundled | **approve-bounded** | Run the three-phase wrapper once per phase in order. Do not interpret this as product acceptance. |

Refuse would be required if identity, caps, actual-consumer, unqualified-claim or stale-binary-adoption predicates failed. They hold on this candidate.

A second current full-source vote from the selected Astra auditor is still required. Both current votes plus fresh actual-result reviews are required before any published narrow scope. Source and documentation approval never become actual financial acceptance.

## Limitations

Inspection is source and byte-hash only. No v4 phase has run. Adapter-v3-check receipt exit 0 is a wrapper prerequisite observation (`elapsed_seconds` 1.04, identity preserved); it is not native ledger prepare evidence. Parent inspections and full hash scans lie outside child time budgets. This maintainer resource control is not a public developer permission prerequisite.
