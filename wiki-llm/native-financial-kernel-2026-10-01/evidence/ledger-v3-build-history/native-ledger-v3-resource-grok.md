# Native ledger v3 resource audit (Grok 4.6 high)

2026-10-01. Fresh independent full-source plus bounded-resource audit of the current v3 candidate. Specified-only. No fetch, metadata rerun, cargo build, Node, export, prepare, native check, SRS, keygen, proof, well_formed, apply, wallet or network transaction was performed. Other auditor reports, verdicts and dispositions were not read. Source existence and metadata receipts are not compilation, proof or ledger acceptance.

## Identity and startup

- Requested auditor: Grok 4.6 high (`grok-4.6`). Host `GROK_MODEL` unset in this session; this report is the Grok 4.6 high audit named by the request.
- Checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930` HEAD `033e9a90465d4c06b787dedb5010e4e18715e030`.
- Startup: repository AGENTS plus `plugins/moriarty-dev/skills/develop/SKILL.md`; guarded `status --json` reports capability `SP01.6 loan-swap-subset`, unresolved operational history / stale admission / missing current accounting and live resource evidence, `pendingTransactions: []`.
- Pipeline: the same AFK one signed financial pipeline. This allocation covers bounded compile plus one keyless export and one keyless prepare. It does not close Preview settlement, authentic genesis, asset provenance, key custody, time correspondence, Core5 lowering, generic property/intent/transition/history, PCD or fee-finality claims.
- Base: `/home/charl/research/moriarty-signed-intent-2026-10-01`.
- Contracts read as acceptance text, not prior approval: `NATIVE-LEDGER-INDEPENDENT-EXPECTATIONS.md` SHA256 `9335bad25cc348ab3617aa16589dbe465ad5e88c6a993ee732adfa58543dfecc`; `NATIVE-LEDGER-PREPARE-EXPECTATIONS.md` SHA256 `e45170b48ab8569ab50aa694780a002e0dc8e8082dd7c6cc1189b2dc779f54fa`.

## Candidate hashes (independently recomputed)

| Artifact | SHA256 | Size |
| --- | --- | --- |
| `NATIVE-LEDGER-V3-RESOURCE-PROPOSAL.md` | `d8e6822380ee1d58e24101179fea8910e26e2f9ffabe879eedcb5d9605f11da9` | 11241 |
| `native-ledger-consumer/manifest.json` | `8e064acbb067080792e8bc27c1f179aa138e33c1a8d9ebb7e1ae47f7b66d1b38` | 9680 |
| `run-native-ledger-v3-bounded.py` | `2ce2ae7b80b4930de7a32b9093b9745b0ab8933fdad557703f94ce59c89a340b` | 16827 |
| `NATIVE-LEDGER-RUNTIME-INPUTS-V3.json` | `b774ce2388d2625d501b038276f1688d5acb6e9f0fc27498682a5715a1699ac3` | 49173 |

All 37 internal manifest SHA256 entries and all 34 external-input SHA256 entries match current bytes. Wrapper, proposal and runtime-input pins match the requested values. `NATIVE-LEDGER-RUNTIME-INPUTS-V3.json.source_manifest_sha256` equals the current caller manifest.

Phase artifacts `native-ledger-v3-{build,export,prepare}-{attempt,receipt}.json`, logs, `native-ledger-v3-keyless-export`, `native-ledger-v3-keyless-preparation` and `native-ledger-v3-keyless-prepare-config.json` are absent. The ledger consumer binary `.../target/debug/beta-native-ledger-consumer` is absent.

## Scope inspected (full current source)

Inspected in full, not as a diff-only reuse of an old source vote:

- Caller: `native-ledger-consumer/src/{main,artifacts,provider,verify}.rs`, `export-ledger-fixture.mjs`, `Cargo.toml`, `Cargo.lock`, baseline lock, pre-repair Cargo.toml snapshots, `INPUT-SOURCE-HASHES.json`, `SOURCE-HASHES.json`, `SOURCE-HASHES-v3.json`, `manifest-v1.json`, README, `NATIVE-BETA-HANDOFF-GAP.md`.
- Snapshots: `native-ledger-consumer-source-v1`, `source-v2`, `source-v3` (hash comparison of every regular file).
- Metadata history: current and `metadata-v1`/`metadata-v2` attempt/receipt/tuple-diff/stdout/stderr; current `metadata.stdout` resolve graph.
- Wrapper AST-parsed without import or execution.
- Installed exporter runtime: 240 files under `compiler-probe/runtime/node_modules`, symlink resolution, `package-lock.json`, compact-runtime JS and onchain-runtime WASM.
- Kernel: `source-attempt-4.compact` / `fixed-transfer.compact`, compile-1..4 receipts, `projection.json`, generated `output/contract/index.js` (`export class Contract`, `export function ledger`), `output/zkir/pay.zkir`, `test.mjs`, kernel manifest.
- Adapter v3: `native-adapter-successor-v3/src/main.rs`, `Cargo.toml`, `manifest.json`, `runtime-proof-data.json`, `good.preimage` pin, plus actual fetch/build/check receipts and `adapter-v3-check.log` / `ADAPTER-V3-RESULT.json`.
- Pinned official APIs at ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` (`/home/charl/.cargo/git/checkouts/midnight-ledger-b2f9c59d942dfdca/9a8777c`) and standalone ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914`; legacy transient `a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1` present.

Current caller versus preserved `native-ledger-consumer-source-v3` differs in `src/main.rs`, `SOURCE-HASHES.json` and `manifest.json`, plus current-only `SOURCE-HASHES-v3.json` and `manifest-v1.json`. The Rust change is one diagnostic `scope` string in `construct`, keyed on `vk_present`: registered-VK preflight versus keyless preparation; both texts exclude final proof, strict well_formed and ledger application. Current `src/main.rs` SHA256 `a6f9c34174bcb0d5fafc829c9b5ec1c5d19568ecbbc13d8d8247774a3e45743e`. Remaining caller/helper/Cargo bytes match source-v3.

Old declined bundle identity remains immutable: `manifest-v1.json` SHA256 `285d3f7b59148e874b9055c0e98484599c46c2ecc07d946986e299ec04e65b1e`. This audit assigns no inherited authority to that bundle.

## Graph, features, Tokio, Anyhow

Current `metadata-receipt.json`: command `unshare -Urn -- cargo metadata --offline --format-version 1 --manifest-path Cargo.toml`, `exit_code` 0, `packages` 391, `resolve_nodes` 391, `resolve_edges` 1220. Independent parse of `metadata.stdout` reproduces 391 packages, 391 resolve nodes and 1220 dep edges.

`Cargo.lock` SHA256 `b0cddb391929e964c82b1587675d9bd5079413e8461b2f2ac2b5e72b8de9b11e`. Independent lock-tuple comparison against `baseline-native-proof-consumer.lock`:

- baseline `[[package]]` 353 = 1 root `beta-native-proof-consumer` + 352 external;
- new `[[package]]` 391 = 1 root `beta-native-ledger-consumer` + 390 external;
- external identity tuples unchanged: 352;
- external removed: 0;
- added: 39 = new root + 38 dependency packages;
- `metadata-tuple-diff.json` removed/added sets match this recount; its `unchanged` field 352 matches external preservation.

Anyhow lock is exactly `1.0.104` (`checksum 330a5ed07fa54e4702c9d6c4174f74427fc0ef6e214bbd677ae50a5099946470`). Direct `Cargo.toml` pin `anyhow = "=1.0.104"`. Preserved `metadata-v2` exit 101 is the overstrict `=1.0.102` conflict (`^1.0.102` wants 1.0.104/1.0.103). Preserved `metadata-v1` exit 101 is missing legacy `midnight-transient-crypto ^2.2.0`. Current patch reproduces upstream alias `midnight-transient-crypto-old` at `a01a1ea0270d2e8a1f9a58f0e3cf5ceb029283e1`. Those v1/v2 receipts remain history.

Direct Tokio: `tokio = { version = "=1.53.1", default-features = false, features = ["rt"] }`. Unified resolve node `tokio@1.53.1` enabled features: `bytes, default, fs, io-util, libc, macros, mio, net, rt, socket2, sync, time, tokio-macros, windows-sys`. Matches the proposal’s direct-versus-unified disclosure. `reqwest` is pulled by `midnight-base-crypto`, not by ledger `test-utilities`.

Ledger-v9 resolve features: `proof-verifying` only. Zswap resolve features: `default`, `proof-verifying`. Official ledger `Cargo.toml` default is `proof-verifying`; `mock-verify` and `test-utilities` exist as declared features and are enabled on zero resolve nodes. Lock text contains neither feature name.

Two ZKIR packages sit in the graph: standalone `midnight-zkir 3.1.0-rc.1` at e82d81d (direct caller) and ledger-bundled `midnight-zkir 2.2.0` at 9a8777c (zswap `zkir_v2`). Metadata establishes that graph. It does not compile type identity between standalone `IrSource` and ledger `ProofPreimage` in this binary.

Host status (not this crate’s compile): rustc/cargo 1.98.1, edition-2024 capable; `unshare` from util-linux 2.41.3; Node v24.21.0. Existing shared target `/home/charl/research/moriarty-crypto-2026-09-30/target` is 4515479003 bytes (4.515 GB decimal, 4.205 GiB), 12056 files, `<= 8 GiB`; free 24903106560 bytes (23.19 GiB) `>= 10 GiB`; `target + 2 GiB` remains under 8 GiB.

## Wrapper and runtime-input recipe

`run-native-ledger-v3-bounded.py` AST-parses. Phase argument is exactly `build|export|prepare`. Child commands are the locked offline cargo build, `node export-ledger-fixture.mjs ... PREPARE_NO_VK NONE <export_dir>`, and `unshare -Urn -- <binary> prepare <config> <config_sha> <prepare_dir>`. No prove/verify child is constructed.

Identity `verify()` pins the caller manifest, every listed source file, every external input, the v3 runtime-input document, the exact 240-file set, each file SHA256, the one symlink resolution, and `compiler-probe/runtime/package-lock.json` before and after each phase. Independent recount: 240/240 files, 0 hash mismatches, sum 4243553 bytes, symlink match, package-lock SHA256 `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e`. Installed tree includes `compact-runtime/dist/index.js` and `midnight_onchain_runtime_wasm_bg.wasm` (1424427 bytes).

Build prerequisite reads `adapter-v3-check-receipt.json` (`exit_code` 0, `stop_reason` null, `source_identity_preserved` true) and `metadata-receipt.json` (`exit_code` 0). Export/prepare re-read the frozen build receipt, rehash the produced binary, and require a fresh output directory. Prepare additionally re-validates the helper export receipt (`prepare_no_vk is True`, `registered_vk_sha256 is None`), the four artifact SHA256s, independent frozen IR/preimage pins from the caller manifest, and the literal fixture object.

Config construction uses exact path/hash objects for ir, initial_contract, expected_contract, runtime, retained_preimage plus the fixed fixture. IR/preimage hashes are checked against the independent frozen manifest. Durable `O_EXCL` attempt reservation and directory fsync run first. Config is then created with `xb`, `fsync` of the file and parent directory, then the child launches. A pre-existing config is refused. The digest of the exact created bytes is the CLI pin.

Export postcondition on child exit 0: helper receipt `prepare_no_vk true` / `registered_vk_sha256` JSON null, projection pin `6afe4f734ab85f767eb2578a5b5a8699bfda81de360ce50b8fb9cb9cd82ee381`, trace pin `2fb7f8343a5e773820abe9fe7d13f0ebe816a9ad043abb4d794f9450a80f000a`, exactly the four names `initial-contract.tagged`, `expected-contract.tagged`, `runtime-native.json`, `registered-pay-operation.tagged`, each actual SHA256 and length matching the helper receipt. Parent receipt freezes `helper_receipt_sha256` and `export_artifacts_sha256`. Prepare rehashes those plus the compiled binary after the helper. Errors keep the consumed attempt; the wrapper has no reset or autoretry.

Ownership-safe SIGINT/TERM mask around launch, process-group SIGKILL in `finally`, and namespace/offline flags are in source. Host SIGKILL/power loss cannot run that `finally`; the exclusive reservation remains. RSS/CPU/traffic sample at 0.1 s and disk at 5 s; stop/receipt record overshoot. Traffic is global non-loopback RX. `RLIMIT_AS` equals the group RSS ceiling (4 GiB build / 2 GiB helper). These are stopping ceilings.

Failed compile with no produced executable can be labeled `source identity changed during phase` while `exit_code` and the log still carry cargo’s result. That is a receipt-label limitation on the failure path.

## Kernel, projection, generated runtime, adapter prerequisite

Four Compact compile attempts exist (`compile-1` exit 255, `compile-2` exit 255, `compile-3` exit 0 on source `781e5050…`, `compile-4` exit 0 on source `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db`). `compile-5` / `source-attempt-5` are absent. Pinned `source-attempt-4.compact` equals `fixed-transfer.compact` and the compile-4 source hash. Constructor assertions freeze A1/02/03 mappings; `pay` sends 1000 + 10 unshielded and updates storage to owner 8990 / recipient 1000 / fee 10 / work 9 / allowance 8990. `trustedRound` is constructor-initialized in 0..10.

`projection.json` carries framed signing message, artifact signature, public key and trust metadata. Retained `runtime-proof-data.json` has 293 public ops and 47 `popeq` entries (private transcript outputs length 0). `pay.zkir` is JSON IR `version {major:3, minor:1}`, `do_communications_commitment: true`, 44 inputs, 788 instructions. Official `IrSource::load` accepts major 3 minor 0..=1. `k` is computed by `optimal_k`; caller later requires `ir.k()==17`. That numeric k is unexecuted here.

Generated `index.js` exports `Contract` and `ledger`, and checks compact-runtime `0.20.0`. Exporter imports that generated file plus the pinned compact-runtime, restores the projection signature, constructs official constructor state, optionally leaves verifier key unset on `PREPARE_NO_VK`, runs `pay`, compares `nativeJson` of the generated trace to the frozen trace, copies constructor storage then replaces data for the expected-contract export, and asserts compact-runtime `ledger()` 8990/1000/10. Expected tagged contract remains storage-only; escrow in that envelope stays 10000.

Adapter v3 receipts match `ADAPTER-V3-RESULT.json`: fetch exit 0; build exit 0 in 80.9 s under `unshare -Urn`, jobs 2, Rayon 2, offline, shared target, binary SHA256 `aa6569176191d2f0c91da57449d4640438ac46128f443a0c9d7f3ec8bbcb842e`; check exit 0. Log: good `SOURCE_CHECK_OK` with 293 `None` skips; five forged preimages refused (`Communications commitment mismatch`, three `Failed direct assertion`, `Public transcript input mismatch for input 1016`). Adapter source is `IrSource::load` plus tagged `ProofPreimage` decode with EOF, host fixture guards on location/binding-zero, then `preimage.check(&ir)`. Qualification on those receipts is IR source preprocessing. It is not cryptographic proof or ledger acceptance. Adapter `Cargo.toml` uses the same standalone zkir e82 and transient-crypto 3.0.0 patches.

## Caller API source facts and uncompiled uncertainty

Source-feasible against pinned APIs (uncompiled in this binary):

- `ContractOperation::new(vk, ir)` sets `v2: None`, `v3: vk`, `ir`; WASM constructor is `new(None, None)`; `verifierKey` undefined when both keys absent.
- `QueryContext::new(ChargedState, address)` and `query(&[Op], Option<RunningCost>, &CostModel)`; `ContractState.data` is `ChargedState` with public `get`/`get_ref`.
- `CallContext` has the eight fields the caller compares: `own_address`, `tblock`, `tblock_err`, `parent_block_hash`, `caller`, `balance`, `com_indices`, `last_block_time`.
- `PrePartitionContractCall` field set matches the caller struct literal; `add_calls` / `partition_transcripts` / `PreTranscript::split_at` reset effects between guaranteed and fallible query.
- `fees_with_margin(params, 2)` uses synthetic `cost(params, false)` plus price-adjustment margin.
- `IrSource::check` / `prove` / `load` / `load_ir_from_tagged` exist; check is preprocessing (`preprocess` → `pi_skips`), not proof.
- `ir_vm` prepends `binding_input` into public instance; binding-zero is legal for this pre-finalization preimage.
- `ProofPreimageVersioned::V2(Arc<ProofPreimage>)`; `**p == frozen` and `ir.check(p)` depend on Deref coercion.
- PrepareConfig `deny_unknown_fields` with only ir/contracts/runtime/preimage/fixture; PK/VK/SRS fields are structurally refused.
- `prepare` calls `construct(..., None)` and returns before Provider, `Transaction::prove`, SRS/PK/VK read, well_formed or apply.
- Tagged decode in `artifacts.rs` enforces EOF; exclusive create-new plus `sync_all`.
- Genesis overwrites the pay operation after incoming keyless checks; constructed genesis is rechecked for v2/v3/ir absence.
- Financial-effect checker requires exact 1000/10 destinations and 1010 unshielded output total; shielded/mint families are refused.
- Fixture NIGHT availability uses `min(value * night_dust_ratio, age * value * generation_decay_rate)` against actual genesis UTXO metadata.

Legitimate bounded-diagnostic failure modes (not observed compiler errors):

- Combined crate type identity: standalone zkir 3.1.0-rc.1 `IrSource` versus ledger 9a8777c `ProofPreimage` / `ContractCall` in one binary.
- JS `nativeJson` BigInt emission as unquoted decimal JSON numbers; `AlignedValue` alignment/normal-form serde; lowercase externally tagged `Op` including `Popeq`/`pushPath` versus compact-runtime trace objects.
- `ir.k()==17` on this IR; full native preimage equality against frozen converter bytes; 8-field CallContext equality after `add_calls`.
- Address-space 4 GiB for rustc of ledger-v9 + circuits, and +2 GiB target increment while compiling newly added packages (adapter checker already grew this target by about 1.89 GiB).

Production `prove`/`verify`/`accept` remain in the binary and are compiled if build runs. Wrapper does not invoke them. Provider still requires a nonzero ledger binding override, one-call budget, skip agreement and dual-parameter verification. `split` clones resolver/params/budget and uses `OsRng` at prove time; RNG split-state is a future prove-phase restriction. `NATIVE-BETA-HANDOFF-GAP.md` is specification/gap text inside the source tree. It is not executable authority. Beta/Core5 producer integration remains open.

Declared fixture (trusted public development only): `network=undeployed`, `block_seconds=1000000`, `night_creation_seconds=0`, `night_value=1000000000000`, `fee_allowance=100000000000000000000`, `ttl_seconds=1000300`. Kernel ECDSA owner and Schnorr NIGHT payer are different roles. Address 04 / A1 / 02 / 03 are explicit genesis bytes. Preliminary `fees_with_margin` is a resource preflight. Actual protocol fees, signatures, final binding, registered VK, well_formed and apply stay out of this allocation.

## Per-phase resource votes

### build — approve-bounded

One exclusive durable attempt. Command is `unshare -Urn -- env CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=<existing shared target> cargo build --locked --offline --manifest-path Cargo.toml --bin beta-native-ledger-consumer` in the exact source directory. Ceilings: wall 600 s, sampled group CPU 1200 s, RSS and per-process AS 4 GiB, incremental target 2 GiB, incremental cache 1 GiB, total target 8 GiB, free floor 10 GiB. Current disk status fits those floors. Graph/lock/anyhow/Tokio/feature claims independently match. Runtime 240-file identity is enforced. Success hashes the regular executable without running it. Compile, type or linker failure, address-space kill, or increment-disk stop consumes the attempt and is a scoped diagnostic. No SRS/keygen/proof execution is in this phase.

### export — approve-bounded

Runs only after a hash-frozen successful build receipt and an unchanged rehashed binary. Node helper is pinned `export-ledger-fixture.mjs` with hardcoded `PREPARE_NO_VK`/`NONE`, pinned projection/trace, installed compact-runtime plus WASM, and generated contract. Ceilings: wall 60 s, sampled CPU 120 s, RSS/AS 2 GiB, no-network namespace, combined export+prepare outputs 32 MiB. Success requires the exact four artifacts and helper-receipt fields named above; parent freezes those digests. JS/WASM serde or equality assertion failure is a legitimate consumed diagnostic. No VK/PK/SRS path exists in this invocation.

### prepare — approve-bounded

Runs only after successful export plus unchanged compiled binary and independently frozen IR/preimage. Supervisor computes the config, refuses a pre-existing config, reserves the attempt, then `xb`+fsync+parent fsync writes the config before launch. CLI is `prepare` only, pin is the digest of those bytes, fixture is the literal trusted-genesis recipe. Success requires `proof_invocations==0`, `preimage_equal is True`, `registered_vk_present is False`, `native_check=="successful actual Zkir::check"`, and unchanged config bytes. The binary itself is the decode/replay/partition/effect/fee-preflight/check gate. One good attempt; the adversarial matrix is not allocated. Remaining prove/verify/well_formed/apply/fee-finality/Core5 findings are future-phase restrictions.

## Bundled resource vote — approve-bounded

The three phases are sequential prerequisites under one wrapper, one existing target, one exclusive durable attempt per phase, no autoretry, no prove/verify, no SRS/PK/VK read, no well_formed/apply. Old declined exact bundle remains immutable and unused. Four Compact attempts remain exhausted. Adapter v3 source-check and offline metadata are observed prerequisites with their recorded qualifications. Two fresh agreeing bounded votes are still required before the first actual build; this is one of those votes.

## Limits (exact)

| Limit | Value |
| --- | --- |
| Build wall | 600 s |
| Build sampled group CPU | 1200 s |
| Build RSS and per-process AS | 4 GiB |
| Export/prepare wall | 60 s each |
| Export/prepare sampled group CPU | 120 s each |
| Export/prepare RSS and per-process AS | 2 GiB each |
| Combined export+prepare outputs | 32 MiB |
| Incremental target | 2 GiB |
| Incremental `~/.cargo` cache | 1 GiB |
| Total target | 8 GiB |
| Free floor | 10 GiB |
| Jobs | `CARGO_BUILD_JOBS=2`, `RAYON_NUM_THREADS=2` |
| Network | `unshare -Urn`, `CARGO_NET_OFFLINE=true`, `--locked --offline` |
| Target | existing `/home/charl/research/moriarty-crypto-2026-09-30/target` only |
| Attempts | one exclusive durable reservation per phase; owned cleanup on catchable signals; source pins before/after |
| Sampling | memory/CPU/traffic 0.1 s; disk 5 s; overshoot recorded |
| Uncatchable | host SIGKILL / power loss skip `finally`; reservation remains |
| Success gate | one export then one prepare, each after frozen prior receipt |

Observed current target 4515479003 bytes and free 24903106560 bytes satisfy the total-target and free-floor predicates at audit time. Those numbers are perishable host status.

## Future-phase restrictions (explicit, no proof authority)

- Official SRS body acquisition, keyed compile/keygen, `Transaction::prove`, cryptographic verification, well_formed, apply, wallet import and network transactions.
- Final binding scalar, registered v3 VK identity, PI/statement equality after Noop finalization, signatures/seal, actual dust fee consumption, NIGHT conservation under strict validation.
- Authentic deployment/deposit/mint provenance, recipient spend keys, consensus time, Preview ledger8 versus this ledger9, Beta/Core5 producer handoff, generic property/intent/transition/history, recursive PCD.
- Broad negative/fault-matrix execution; provider RNG-split semantics; any claim that metadata, source hashes or adapter IR-check equal financial acceptance.

## Qualification

approve-bounded for build, export, prepare and the exact three-phase bundle. This is resource authority for one locked offline compile and, on that success, one keyless official export and one keyless native preparation diagnostic. It is not proof, not ledger acceptance, and not closure of the signed financial pipeline.
