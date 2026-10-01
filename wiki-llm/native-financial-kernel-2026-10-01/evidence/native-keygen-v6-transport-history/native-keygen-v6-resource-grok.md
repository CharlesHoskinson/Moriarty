# Native keygen v6 resource review — Grok 4.6 high

Returned identity: Grok 4.6 high. Terminal status: complete independent source-and-resource review. This report is one new substantive resource vote. It is not a v5 actual-result review, not financial acceptance, and not allocation. Live freeze, candidate, result, manifest, runtime, ELF, and disk figures below were independently rehashed in this review; administrative freeze metadata remains identity only.

Scope: full current uncompiled keygen candidate, three Python helpers, proposal, inherited v5 result freeze and original receipts, official pinned APIs, cached Preview research, and a read-only resource refresh. No other auditor report, verdict, or disposition was read. Administrative freeze metadata is identity only. No script import or execution, no Cargo metadata/fetch/build, no Node/TS/native-candidate check, no binary copy, no SRS body, no keys, no keygen, no proof, no well_formed, no apply, no wallet, no transaction, no network acquisition.

Primary checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`. Develop skill was read from `plugins/moriarty-dev/skills/develop/SKILL.md`. Guarded CLI `status` was not executed because this audit forbids importing or executing scripts. Status below is reconstructed from `openspec/moriarty-completion-program.json`, `.moriarty-dev/actions.json`, and read-only SQLite `/home/charl/Moriarty/.git/moriarty-dev/state.sqlite3`.

## Status reconstructed from records

Active registered action family is SP01.6 loan-swap-subset (`sp01-loan-report` first). Program stages still list I2 as pending-review and native-path-freeze as specified-only. Outbox holds 32 confirmed transactions and zero rows with `delivered_at` NULL. Reservations table holds two finished rows. This reconstruction does not establish hook trust or product acceptance.

Maintainer resource controls apply to this allocation. They are not public developer authorization and are not compile/prove/deploy prerequisites.

## Independent freeze and candidate hashes

Computed SHA256 of the exact review package:

| Path | Bytes | SHA256 |
| --- | ---: | --- |
| `NATIVE-KEYGEN-V6-SOURCE-FREEZE.json` | 764 | `76dd7dfe69df23e0abee9b17db44a1fd08b0fda849a11dbead575a036d270b44` |
| `run-native-keygen-v6-bounded.py` | 17021 | `d1014784628ec2539035a99ffc0769b5e0d1eab15bfbdb38844bb97b33193c0a` |
| `native-keygen-v6-retain-binary.py` | 9009 | `89c372825ad99e0ad5a54fbacbf9a475e7f427269d464033178ac4ddb18340dd` |
| `native-keygen-v6-srs-fetch.py` | 2118 | `5769d9c23c03dfbcdf8cfe27de59191780a64492006cfcbc8a31fb3f505893e9` |
| `NATIVE-KEYGEN-V6-RESOURCE-PROPOSAL.md` | 10030 | `e24ecd6f416ce70a8242a7f633dde0df6927361d70466704a2269ea126ca5539` |
| `native-ledger-keygen-candidate/SOURCE-HASHES.json` | 889 | `73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df` |
| `NATIVE-LEDGER-V5-RESULT-FREEZE.json` | 4599 | `d9c2d56e396e5720a26d1a07183602056d2aef1a9c98ca101a5103968a599d0b` |

All six freeze-listed source hashes match live files. Result freeze contains 34 `sha256` entries. All 34 live files match. Compiled ELF `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer` rehashed without launch: 624,060,592 bytes, SHA256 `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`, mode `0100755`.

Candidate `SOURCE-HASHES.json` nine files all match:

| Candidate file | SHA256 |
| --- | --- |
| `INPUT-SOURCE-HASHES.json` | `9d34d65754d8282515fccd6941859bed273db729995baa9254a3055fb42a658b` |
| `Cargo.toml` | `78a76d91906032a331d6e4d91359312544289f6a0c69f606daea4f62645e649c` |
| `README.md` | `76b3a8623c73f0622038fb01937e5d4bba74576f5e5fadb4e2a0c934417c8cb4` |
| `Cargo.lock` | `b0cddb391929e964c82b1587675d9bd5079413e8461b2f2ac2b5e72b8de9b11e` |
| `src/verify.rs` | `8e9c59d0d80efbeb28a5d2f84d899be1dc5d732833178d4025656e69c8fe3c8b` |
| `src/artifacts.rs` | `4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7` |
| `src/keygen.rs` | `9f97af1657ad1e23b099e184ea8fb57d91a96976149d7163541f8218ee915f5b` |
| `src/provider.rs` | `1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f` |
| `src/main.rs` | `393efec218b0816e9c6bbc792da37e9a7f32b75719781c19d0c3e9d346945435` |

All 12 candidate `INPUT-SOURCE-HASHES.json` paths match. `Cargo.toml` and `Cargo.lock` are byte-identical to `native-ledger-consumer/`. `artifacts.rs`, `provider.rs`, and `verify.rs` are byte-identical. `src/main.rs` differs only by `mod keygen`, the `keygen` CLI arm, and the usage string. `ast.parse` accepted all three Python helpers.

Inherited `native-ledger-consumer/manifest.json` SHA256 `bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f`: 39 local + 260 external, 0 mismatches. Runtime V5 SHA256 `68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425`: 240 installed files + 240 kernel files, live rglob sets equal maps, every file hash matches, kernel `node_modules` is a symlink to `compiler-probe/runtime/node_modules`, `package-lock.json` SHA256 `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e`.

Python helpers parse as syntax-only. Wrapper and retention helper were not executed.

## Actual v5 prerequisites from original receipts

Inspected original attempt/receipt/log/artifact bytes. Administrative result/resource decision files were hashed for freeze identity and were not read as approval.

Handoff attempt `native-ledger-v5-handoff-attempt.json` reserved `unshare -Urn -- node native-beta-handoff/cli.ts` against production `packages/moriarty-beta/examples/signed-intent/transfer-ecdsa-wallet/{program.mori,scenario.json,signature.json}` and `moriarty-midnight-crypto`. Receipt: exit 0, `stop_reason` null, `source_identity_preserved` true, `binary_identity_preserved` true, binary SHA `034ed49b…`, 624,060,592-byte ELF, helper receipt SHA `f754359f…`. Log is one JSON object `NativeFixturePreparedUnqualified`. Production source digest `fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c` equals live `program.mori`. Core candidate SHA `8312b220fc9348d37ad972daefdcfc8ddb554dcc9cf7b3f84547aa46dcacac4b`. Premises and unverified bindings match the handoff contract. `authority_valid` is null. `proof_produced`, `well_formed_checked`, `ledger_applied`, and `ledger_accepted` are false.

Prepare attempt reserved the same frozen ELF `prepare` with config SHA `32441b1c38c6c71fd8dd0b673609859292535686340f10e053138c8e262a88e1`. Receipt: exit 0, `stop_reason` null, source and binary identity preserved, elapsed 47.476 s, peak sampled group RSS 202,268,672 bytes, peak sampled group CPU 46.45 s. Log line: `NATIVE_PREPARATION_SOURCE_CHECK_OK; no VK/PK/SRS/proof/ledger acceptance`.

Preparation artifacts:

- `constructed.preimage` and `frozen.preimage` are each 4,316 bytes with SHA256 `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402`.
- `construction-diagnostics.json` records `preimage_equal` true and seven field equalities true (`inputs`, `private_transcript`, `public_transcript_inputs`, `public_transcript_outputs`, `binding_input`, `communications_commitment`, `key_location`).
- Eight CallContext equalities are true (`own_address`, `caller`, `balance`, `tblock`, `tblock_err`, `parent_block_hash`, `com_indices`, `last_block_time`).
- `runtime_ops` 293, `runtime_reads` 47.
- `native-check-skips.json` has 293 entries, all JSON `null`. `native-check-result.txt` begins `Ok([None, …])`.
- `preliminary-fees.json`: estimate `1128519108356650`, allowance `100000000000000000000`, native parameter SHA `38d99ef89d303fc126754e68b35c79a1804abcf734f627f35748d164fe503c29`, method `Transaction::fees_with_margin(default LedgerParameters,2)`, `vk_present` false. `preparation.json` records generationless available `5000000000000000000000`, `native_check` `successful actual Zkir::check`, `proof_invocations` 0, `registered_vk_present` false.
- TTL: `ttl_seconds` 1000300, `block_seconds` 1000000, recorded default global TTL 3600. Delta 300 is within 3600.
- `partition-costs-effects.txt` records `guaranteed=None` and `fallible=Some` with unshielded output 1010 on A1, claimed spends 1000 to `02…` and 10 to `03…`.
- Keyless operation artifact `registered-pay-operation.tagged` is 35 bytes: tag `midnight:contract-operation[v6]:` then three zero bytes.

Handoff `cli.ts`/`handoff.ts` import production `auth.verifyAndPrepare`, `bridge.expand`, Source6 frontend `parseAndLowerSource6`, generated `kernel-prototype/output/contract/index.js`, and Compact runtime through the canonical symlink. Live hashes match `native-beta-handoff/INPUT-SOURCE-HASHES.json` for `auth.ts` `bbddda1d…`, `bridge.ts` `d4ca0837…`, `frontend.ts` `6c45599e…`, `json.ts` `e1631850…`, Source6 `1b3bd3b0…`, Core5 `855efd40…`, Source6 frontend `f3d2a05d…`, generated kernel `98649b5b…`, pay IR `c20c6e73…`, converter `runtime-proof-data.json` `2fb7f834…`, frozen preimage `96f932aa…`, and `moriarty-midnight-crypto` `42fa598d…`. Owner program is ECDSA wallet transfer. Native fixture payer is Schnorr `SigningKey::from_bytes(&[0x42;32])`. Signed A fee 10 is distinct from DUST protocol fee.

Consumed history remains on disk: v3 build log contains nine `error[` diagnostics, receipt exit 101, `produced_binary_sha256` null. v4 build log contains `Finished dev profile … in 19.03s` and no `error[`, receipt exit 0, binary `034ed49b…`. v4 handoff log is `TypeError: Cannot use 'in' operator to search for 'popeq' in member` at `handoff.ts:99`, exit 1, source and binary preserved.

These original artifacts establish typed keyless preparation of the production signed Beta profile. They do not establish proof, well_formed, apply, 8990 applied escrow, or Preview. Two terminal fresh actual-result reviews remain a separate root gate before any v6 allocation. This vote does not supply those reviews.

## Candidate and official API source facts

Pinned ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914` `zkir/src/ir.rs` implements `Zkir::keygen` at line 121: `setup_vk(params.get_params(self.k()).await?.as_ref(), self)` then `setup_pk(self, &vk)`, returning tagged-serializable `ProverKey` and `VerifierKey`. `keygen_vk` at line 110 uses `setup_vk`. `load_ir_from_tagged` and `load_prover_key_from_tagged` are tagged deserializers. `IrSource::load` parses JSON IR major 3 minor 0..=1. Frozen `pay.zkir` header is version major 3 minor 1 and has no JSON `k` field. Two `k()` methods exist: inherent `IrSource::k()` at line 1281 returns `self.model.k`; trait `Zkir::k()` at line 106 returns `midnight_zk_stdlib::optimal_k(self) as u8`. Candidate `keygen.rs` calls `ir.k()` after `IrSource::load`, which resolves to the inherent method. Official `Zkir::keygen` then calls trait `self.k()` for `get_params`. Provider `get_params` still refuses any requested `k != 17`. Original CLI row-model used trait `k()` plus `ir.model().rows()` and recorded `k=17, rows=114250`. A stored-model versus `optimal_k` divergence would fail closed at provider decode. It is not silent wrong-k keygen.

Official CLI `zkir/src/main.rs` Compile/CompileMany call the same `ir.keygen(&pp)` after `File::create` of output paths, with `MidnightDataProvider` `FetchMode::OnDemand`. Candidate keygen uses the existing offline `provider::Parameters`, closed `{ir,srs}` config with `deny_unknown_fields`, exact SRS SHA `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74`, exact 25,166,212 bytes before `ParamsProver::read`, and cursor EOF. There is no OnDemand path, no environment parameter lookup, and no extra config field for PK/VK. `std::env::args` is CLI dispatch only.

`ParamsProver::read` (`transient-crypto/src/proofs.rs:92`) uses `ParamsKZG::read_custom(..., SerdeFormat::RawBytesUnchecked)`. Candidate `get_params` additionally requires `k==17` and `c.position()==bytes.len()`. Digest identity of the SRS body is not a ceremony audit. Receipt field `srs_ceremony_independently_audited` is written false.

Tagged roundtrip: PK through `IrSource::load_prover_key_from_tagged` plus EOF and re-encode identity, VK through `decode` (tagged deserialize plus EOF) plus re-encode identity, IR through `load_ir_from_tagged` plus EOF, `k()==17`, and re-encode identity. Exclusive `pk.tagged`/`vk.tagged`/`ir.tagged` then `keygen-success.json`. Started/failed records cannot satisfy wrapper postconditions. Wrapper requires `keygen_complete` true, `tagged_roundtrip_eof_identity` true, `authority_valid` null, `proof_produced`/`well_formed_checked`/`ledger_applied`/`ledger_accepted` false, and `resolver_registration_key_agreement` equal to `NotChecked; required in future actual finalized proof`.

`ContractOperation::new(None, None)` sets `v2`/`v3`/`ir` all absent (`onchain-state/src/state.rs:903–908`). Prepare path already required that shape. Keygen does not register a VK.

Tokio is requested as `default-features=false, features=["rt"]`. Retained metadata receipt records unified features including `net`, `fs`, `macros`, `mio`, `socket2`. Candidate `Cargo.lock` lists those tokio dependencies. Direct lock text contains no `mock-verify` or `test-utilities`. Graph evidence remains the retained metadata receipt: 391 packages, 391 resolve nodes, 1,220 edges. Lock bytes are unchanged, so this is inherited graph identity, not a newly executed metadata run.

`reqwest` remains a transitive lock node. Build and keygen commands use `unshare -Urn`. SRS acquisition uses the public network.

Row-model original `kernel-prototype/row-model.stderr`: `Mock compiling circuit ".../pay.zkir" (k=17, rows=114250)`. Receipt scope: CLI row model only. That figure is not keygen CPU/RSS evidence.

Candidate binary source still contains `prepare`, `prove`, `verify`, and `accept`. This allocation does not run those modes.

## Wrapper and retention source facts

Retention helper is a separate process. It requires the same closed supervisor JSON keys `{wrapper_sha256, candidate_source_sha256, result_freeze_sha256, terminal_v5_result_reviews, resource_votes, old_binary_archive}`. It binds itself by parsing `retention_pin` from the frozen wrapper AST and by matching `NATIVE-KEYGEN-V6-SOURCE-FREEZE.json`. Copy uses 1 MiB chunks, stream hash, exclusive destination, file/root fsync, SIGALRM 60, RLIMIT_CPU 120, RLIMIT_AS 2 GiB, RLIMIT_FSIZE 1 GiB, directory ceiling 1 GiB, free floor 10 GiB. Partial copy and reservation remain on failure. No unlink, retry, subprocess, network, Cargo, or ELF execution.

Three-phase wrapper phases are `srs|build|keygen` only. It requires the retained archive file to already exist at `native-keygen-v6-retained-v4-binary/beta-native-ledger-consumer` with exact size/hash, plus a completed retention receipt. Sequential order is retain, then srs, then build, then keygen. Wall/CPU totals including retention: 60+60+120+300 = 540 s wall, 120+120+240+600 = 1080 s CPU. The proposal sentence that states 480/960 describes the three wrapper phases without retention.

Wrapper vote gate checks two Astra and two Grok records with `{provider,path,sha256,approved}` and `approved is True`. That boolean is administrative. Root must interpret the actual review bytes and exact scope before creating authorization. Child success flags grant no authority.

SRS helper: one URL `https://srs.midnight.network/bls_midnight_2p17`, no redirect handler, `Accept-Encoding: identity`, status 200, exact URL, Content-Length 25166212, Content-Encoding identity, stream rejects the first extra byte, hardlink `srs.part` to `bls_midnight_2p17`, apparent names 50,332,424 bytes, headers redacted, `ceremony_independently_audited` false. Wrapper applies wall 60 / CPU 120 / RSS-AS 2 GiB / socket 5 s to that child.

Build: `unshare -Urn -- env CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=/home/charl/research/moriarty-crypto-2026-09-30/target cargo build --locked --offline --manifest-path Cargo.toml --bin beta-native-ledger-consumer` in `native-ledger-keygen-candidate`. Wall 120 / group CPU 240 / RSS-AS 4 GiB. Requires live ELF still equal to the old pin before overwrite. Hashes the produced regular executable without running it.

Keygen: closed config `{ir:{path,sha256}, srs:{path,sha256}}` created exclusive and fsynced after durable attempt. Command `unshare -Urn -- env RAYON_NUM_THREADS=2 <binary> keygen <config> <configSHA> native-keygen-v6-keys`. Wall 300 / group CPU 600 / RSS-AS 4 GiB. `RAYON_NUM_THREADS=2` is a Rayon pool environment, not a hard cap on every native thread.

Sampling: RSS/CPU 0.1 s, disk 5 s. Overshoot is possible. Global non-loopback RX includes unrelated host activity. Catchable SIGINT/TERM run finally and SIGKILL the process group. Uncatchable SIGKILL/power loss leaves the durable reservation. No malicious process-group escape claim.

Each failed phase consumes its exclusive attempt. Dependent phases stop. There is no unchanged retry, delete, or fresh cache.

## Resource refresh

Same rglob `is_file`/`stat.st_size` algorithm as the wrapper:

| Quantity | Bytes |
| --- | ---: |
| target rglob st_size | 7,501,078,730 |
| target `du -sb` | 6,336,744,232 |
| cache rglob st_size | 2,933,611,498 |
| filesystem free on target | 22,457,090,048 |

The rglob versus `du` difference is hard-link accounting. It is not evidence of concurrent cleanup. Target remains under 8 GiB (headroom 1,088,855,862) and under 10 GiB (headroom 3,236,339,510). Incremental +2 GiB would exceed the old 8 GiB ceiling and would remain under 10 GiB (9,649,220,074). Free 22,457,090,048 is above 10 GiB. Live ELF still matches the old pin. No `native-keygen-v6-retained-v4-binary`, `native-keygen-v6-srs`, `native-keygen-v6-keys`, or v6 attempt/receipt files exist. Cache accounting uses the wrapper path `/home/charl/.cargo` (`st_size` 2,933,611,498).

v4 compile on this shared target: initial 5,549,887,375, final 7,501,078,730, increment 1,951,191,355, wall 20.271 s, peak sampled RSS 2,993,246,208, peak sampled CPU 30.16 s, exit 0. That is historical compile evidence for the current crate graph. It is not a measurement of the uncompiled keygen module.

## Preview research (cached, not live rejection)

`PREVIEW-COMPATIBILITY-20261001.md` records Preview `midnight_ledgerVersion=8.1.2` at finalized height 1103914. `preview-source-linkage-20261001/COMPATIBILITY-SOURCE-LINKAGE.md` maps official ledger-8.1.2 commit `36d7442172136758e33009f75ff4aa56616cff40` and node tag runtime-1.0.300 `c3895bd2bd930b03ad53a4c8de0d98a490b8bfcb` to ProofVersioned V2, ContractOperationVersion V3, and ZKIR major 2 minor 0. Candidate pins ledger-v9 `9a8777c4…`, IR 3.1, and V3 proof/V4 operation. That is conditional published-source incompatibility. It is not an observed network rejection of this candidate and not a live-node build attestation. Local trusted-genesis keygen does not close Preview.

## Uncertainty classification

Missing cold-from-empty build fit is a legitimate bounded uncertainty. The proposal uses the existing shared target, identical Cargo/lock, `--locked --offline`, and a two-file source delta. v4 already produced the current ELF on that target under a 2 GiB incremental cap. The 8→10 GiB total-target amendment is the arithmetic consequence of 7,501,078,730 + 2 GiB. A consumed compile failure remains a consumed attempt.

Missing keygen CPU/RSS/output-size is a legitimate bounded uncertainty. k17/114250 is a mock-compile row model. `setup_vk`/`setup_pk` at k=17 can exceed 4 GiB RSS, 300 s wall, 600 s CPU, or the 512 MiB SRS+key output cap. Those stops are in-contract. They do not require a diagnostic run before a bounded vote. They also do not license retry, cache reset, or a larger silent cap.

## Votes

Exact candidate under review: freeze `76dd7dfe…`, wrapper `d1014784…`, retention helper `89c37282…`, SRS helper `5769d9c2…`, proposal `e24ecd6f…`, candidate SOURCE-HASHES `73a68f85…`, result freeze `d9c2d56e…`, old ELF `034ed49b…` / 624,060,592 bytes.

| Item | Vote | Bound |
| --- | --- | --- |
| Retention of old ELF | **approve-bounded** | First sequential stage. Exclusive archive, 60/120/2 GiB AS/1 GiB file and directory, free ≥10 GiB, stream copy, fsync, consume on failure. Outside the 512 MiB SRS/key cap, separately 1 GiB. Same filesystem. |
| SRS acquisition | **approve-bounded** | One public endpoint, exact 25,166,212 bytes and SHA `4a9ef6c7…`, no redirect/compression/retry/header logging, extra-byte refuse, hardlink pair 50,332,424 apparent bytes, 60/120/2 GiB, socket 5 s. Digest integrity only. |
| Build | **approve-bounded** | After retention receipt and SRS success. Shared existing target only, `--locked --offline`, jobs 2, Rayon env 2, net-offline namespace, 120/240/4 GiB, incremental target ≤2 GiB, cache ≤1 GiB, total target ≤10 GiB. Hash produced ELF without launch. Preserve old archive as prior-result identity. |
| Keygen | **approve-bounded** | After successful same-candidate build hash and current SRS. Closed IR/SRS config, exclusive fsync, actual `IrSource::keygen`, 300/600/4 GiB, Rayon env 2, combined SRS+keys ≤512 MiB, per-file 2 GiB. Tagged PK/VK/IR EOF roundtrip and parent hash/size required. CPU/RSS/output size unmeasured. |
| 8→10 GiB total-target amendment | **approve-bounded** | This future allocation only. +2 GiB target / +1 GiB cache / free ≥10 GiB / 512 MiB SRS+keys / retained 624 MB under 1 GiB. Refresh rglob st_size and free before dispatch. |
| Four-stage sequential bundle | **approve-bounded** | Max 540 s wall / 1080 s CPU. Two current actual-result reviews plus two new resource votes, each with provider/path/hash/`approved`, independently interpreted by root, before any stage. |
| Proof / well_formed / apply / Preview / fifth Compact / R3 | **refuse** | No allocation. |
| Generic property / intent / transition / history / PCD / authentic genesis, funding, asset, account, time, head | **open** | Unclosed. |
| Financial publication / ledger acceptance | **refuse as acceptance** | Whole partition `guaranteed=None` / fallible `Some` is positive preparation only. No rollback or 8990 applied evidence. |

Allocation remains blocked until root records two terminal fresh actual-result reviews and a second agreeing new substantive resource vote on this exact candidate. This Grok 4.6 high vote is one of those two resource votes. Wrapper `approved: true` flags are not financial premises.

## Limitations

- Keygen module is uncompiled. Trait/lock compatibility is source-feasible from unchanged Cargo/lock and pinned APIs. Inherent `IrSource::k()` versus trait `Zkir::k()`/`optimal_k` is a closed fail if they diverge at k17.
- `ParamsProver::read` uses `RawBytesUnchecked`. Candidate EOF is a cursor-length check after that read.
- Official CLI OnDemand/fetch and pre-create of key files are not used. Candidate still embeds prove/verify source.
- Unified Tokio features include `net`. Containment for build/keygen is the user namespace.
- Monitor overshoot, global RX, Rayon-env-not-every-thread, catchable-signal cleanup, and SIGKILL reservation limits are as disclosed in the wrapper.
- Preview 8.1.2 versus ledger-v9 remains a cached source incompatibility. No live rejection was observed in this audit.
- Independent expectations for finalized financial call remain specified-only: real VK registration, nonzero ledger binding, `Transaction::prove`, signatures, default well_formed, apply, actual DUST fees, and Preview.

No proof, well_formed, apply, Preview, fifth Compact, or R3 work is authorized by this vote.
)
