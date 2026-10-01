# Combined audit: actual v7 result, v8-03 source/resources, Beta CLI display

Auditor: Grok 4.6 high. Date: 2026-10-01.  
Primary checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.  
B: `/home/charl/research/moriarty-signed-intent-2026-10-01`.  
Assigned output: this file only. No source changes. No wrapper import or execution. No Cargo, Node, native, copy, network, SRS request, keygen, proof, well-formed, apply, wallet, or transaction. Other auditor report bodies, verdicts, and root dispositions were not read. Administrative identities were hashed.

## 1. Scope of this report

Two distinct substantive families, plus one separate CLI publication vote.

**(A) Actual v7 result publication.** Exact `NATIVE-KEYGEN-V7-RESULT-FREEZE.json` (44 entries, freeze SHA `44c63a77861faca25737320db910fa404ae1081653918fceab1593e03b3e3389`) and produced ELF SHA `9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559` (696706768 bytes). Vote is approve or refuse narrow publication of the actual build + SRS + keygen result. This vote is not financial acceptance.

**(B) New v8-03 source and resources.** Exact `NATIVE-FINANCIAL-V8-03-SOURCE-FREEZE.json` SHA `22df762da25eb59dbd4e239145fbbdc48976b9cb18326ce766902eeb63ca6916`, wrapper SHA `f5af2a11614365a702eb805c42c5252b59c9fe7b77e2398ed129e63c7e572687`, INPUT SHA `4254e9547d3211c4fd695ebf3627a91add2a565925221e325ece2e35ec723a0b` (667 absolute identities plus closed binary / prove_config / v7 freeze), proposal SHA `eb57330ac372a6790045d6c12898cbb57174a53b05a68fcfc0bfa2e19581821b`. Separate approve-bounded or refuse votes for each of: one proof, one independent verification, and the bundle. Old resource or source votes do not constitute these approvals.

**(C) Scoped Beta CLI source publication.** Exact `BETA-CLI-FOLLOWUP-CANDIDATE-FREEZE.json` (6 current files). One-line display repair. Independent of (A) and (B).

This report supplies both administrative families because it contains independent substantive judgments on both precise frozen scopes. Root interprets two `terminal_v7_result_reviews` plus two `resource_votes` (providers Astra and Grok), hash-bound, `approved: true`. This file is one Grok member of those families. It is not a financial premise and not a Preview or developer prerequisite.

## 2. Startup and method

Loaded `plugins/moriarty-dev/skills/develop/SKILL.md` and `AGENTS.md`. Executed READ-ONLY guarded `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`.

Guarded status: capability `SP01.6 loan-swap-subset`; blocked on unresolved operational history, stale SP01 admission, missing current accounting, unavailable live resource `sp01-loan-swap-grok-01`; `nextAction` `sp01-loan-report`; `pendingTransactions` `[]`. This audit is authorized read-only review. It does not dispatch the blocked loan-swap implementation.

Method: rehash every freeze map against live files; read raw v7 reservations, commands, logs, receipts, config, acquisition, keys, and ELF strings; read full candidate 9, API 12, wrapper, proposal, expectations, native prove/verify path, production Beta/Core/successor, generated handoff, inherited 39+260, runtime 240+240, CLI 6 plus git original; inspect AST/source of `IrSource::load`, `Zkir::keygen`/`setup_vk`/`setup_pk`, ledger `prove.rs` binding override, and consumer `construct`/`prove`/`accept`/`verify`. Family hashes were checked and then the compiled/source consumers were read. Hashes alone are not the vendor audit.

## 3. Current pins actually rehashed this session

| Identity | Claimed SHA-256 | Live match | Bytes / notes |
|---|---|---|---|
| `NATIVE-KEYGEN-V7-RESULT-FREEZE.json` | `44c63a77861faca25737320db910fa404ae1081653918fceab1593e03b3e3389` | yes, 44/44 entries | 5411; produced ELF closed separately |
| `NATIVE-KEYGEN-V7-SOURCE-FREEZE.json` | `8e40a50f8bf03ea9461fa7a6cbe053bc008bf5d5a5868e9a8bc65d46d9b5087a` | yes, 10/10 | 1231 |
| wrapper `run-native-keygen-v7-bounded.py` | `36264262437ef33bbbbecfda6835b5eea58052eaab994ce9f13872568be0c556` | yes | 20260 |
| fetch `native-keygen-v7-srs-fetch.py` | `99bcbdc85a7f5ca09f9ef603f1fa88449537be295f6c9c9a8934d44eab250f84` | yes | 2717 |
| candidate `SOURCE-HASHES.json` | `73a68f8577c271a236dd737d213742f2a7dcc8d3e5c5e1cab0688ef839adf8df` | yes, 9/9 | 889 |
| `NATIVE-LEDGER-V5-RESULT-FREEZE.json` | `d9c2d56e396e5720a26d1a07183602056d2aef1a9c98ca101a5103968a599d0b` | yes, 34/34 | 4599 |
| `NATIVE-FINANCIAL-V8-03-SOURCE-FREEZE.json` | `22df762da25eb59dbd4e239145fbbdc48976b9cb18326ce766902eeb63ca6916` | yes, 3/3 | 500 |
| wrapper `run-native-financial-proof-v8-03-bounded.py` | `f5af2a11614365a702eb805c42c5252b59c9fe7b77e2398ed129e63c7e572687` | yes | 22015 |
| `NATIVE-FINANCIAL-V8-03-INPUT-FREEZE.json` | `4254e9547d3211c4fd695ebf3627a91add2a565925221e325ece2e35ec723a0b` | yes, 667/667 | 130501 |
| proposal `NATIVE-FINANCIAL-V8-03-RESOURCE-PROPOSAL.md` | `eb57330ac372a6790045d6c12898cbb57174a53b05a68fcfc0bfa2e19581821b` | yes | 7851 |
| produced ELF | `9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559` | yes | 696706768 |
| retained v4 ELF | `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688` | yes | 624060592 |
| pay.zkir JSON | `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae` | yes | 76046 |
| pk.tagged | `28fbadfa9db2cc227f49347140f9f7f062ced44f4f77aa2b1579e3810c6ea6da` | yes | 234911946 |
| vk.tagged | `6152e754000515f07b63b8296d6072bc4b8f98504653075ae15b7ab5f1e4326c` | yes | 2745 |
| ir.tagged | `a5a6f501847521e9744b66965b4a9917ae2a72ae2f1963271e3a60a8f1f19214` | yes | 25252 |
| SRS `bls_midnight_2p17` | `4a9ef6c7c0619aab74eede44b13e753e3ba54508a02dd3b7106a949aabb73b74` | yes | 25166212 |
| v6 `srs.part` | `242070c2be0e98541837ece08f9c3179bb4a532ea921918e071c19cbc225b2c5` | yes | 327680 |
| runtime inputs V5 | `68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425` | yes, 240+240 | package-lock `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e` |
| consumer `manifest.json` | `bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f` | yes, 39+260 | |
| candidate API 12 `INPUT-SOURCE-HASHES.json` | `9d34d65754d8282515fccd6941859bed273db729995baa9254a3055fb42a658b` | yes, 12/12 | |
| CLI freeze file | `dcd331a51009ce67150caa61934f2e65deb526871e0fac6a36578bfef3a3b581` | 6/6 files match inner map | 1639 |
| v7 config | `faa226f94d538137f3e1073ebfbb96e21886bab7e51205b813c7d1a9a539bf39` | yes | 413 |
| metadata stdout | `37cb2e536ad104b56597c1390b468eb6e5d30ebe9d5c7f6ae6a32784205f20fe` | receipt present | 391 packages / 391 nodes / 1220 edges |

Administrative identities hashed (bodies unread):  
`native-keygen-v7-resource-astra.md` `98630683648e02649f8c8bebb3ff0dba20f4809081879642ee91a47d7da186a2`;  
`native-keygen-v7-resource-grok.md` `49c48978b62105e81b83e67aac73e43493ac6f828f9fbced413f4060c23392ae`;  
`NATIVE-KEYGEN-V7-RESOURCE-DECISION.json` `9d071c88f3446ba6232abe65238fc276e7481962b08af6c46dc1d73f5ae5c898`;  
`NATIVE-KEYGEN-V7-SUPERVISOR-AUTHORIZATION.json` `3a72066034ebb284fe4a6a8b342573c78505df750672f80718fa9d408545fd3f`.

No v8-03 attempt, config, log, receipt, proof directory, verification directory, authorization, or `NATIVE-FINANCIAL-V8-03-PROOF-RESULT-FREEZE.json` exists.

Fresh disk recheck (rglob `st_size` on target and `~/.cargo`, `shutil.disk_usage` free): target **8503734623**, cache **2933611498**, free **20610478080**. Target and cache equal the v7 keygen final sample. Free is 12804096 bytes below that receipt's `20623282176` (host drift). Floor 10 GiB remains satisfied. Apparent rglob size includes hardlinks; `du` unique-hardlink values differ. No cleanup is inferred.

---

# Scope A — actual v7 build + SRS + keygen result

## 4. Actual build

Receipt `native-keygen-v7-build-receipt.json`: phase `build`, exit **0**, `stop_reason` **null**, `source_identity_preserved` **true**, `binary_identity_preserved` **true**, `child_launched` **true**, `supervisor_error` **null**. Command: `unshare -Urn -- env CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=.../moriarty-crypto-2026-09-30/target cargo build --locked --offline --manifest-path Cargo.toml --bin beta-native-ledger-consumer`. Source manifest `73a68f...`, v5 freeze `d9c2d56e...`, supervisor auth `3a720660...`. Elapsed **24.6288 s**, peak RSS **3265310720**, peak CPU **36.71**. Target **7501078730 → 8503734623** (incremental ~1.00 GiB under the 2 GiB incremental cap). Cache unchanged **2933611498**. Log: `Finished dev profile ... in 22.61s`. Produced ELF path `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer`, SHA `9e1b153a...`, 696706768 bytes, mode 0755, ELF64 LSB PIE x86-64, BuildID `8c269ecee2f8ba025d02c5e984c123d4ef8029c7`, debug_info, not stripped.

## 5. Actual SRS

Receipt `native-keygen-v7-srs-receipt.json`: phase `srs`, exit **0**, stop **null**, source and binary identities preserved, SRS SHA `4a9ef6c7...`. Helper `native-keygen-v7-srs-fetch.py` (one new full-body GET, no redirect, no range resume, prior v6 transport attempt consumed). Elapsed **7.5129 s**, RSS **25776128**, CPU **0.18**. Acquisition JSON: `verified` **true**, `stage` `final_name_published`, `bytes`/`bytes_written` **25166212**, `attempts` **1**, `attempts_this_allocation` **1**, `prior_consumed_transport_attempts` **1**, `range_resume_fallback` **false**, `ceremony_independently_audited` **false**. Terminal log: `EXACT_PUBLIC_SRS_BODY_HASH_VERIFIED; previous v6 request remains consumed; not ceremony/keygen evidence`. Live `bls_midnight_2p17` and `srs.part` share inode 16770252, nlink 2 (hardlink). Body identity is a digest of the published k17 file. It is not a ceremony audit.

## 6. Actual keygen

Config `native-keygen-v7-config.json` SHA `faa226f9...` equals `{ir: pay.zkir c20c6e73..., srs: bls_midnight_2p17 4a9ef6c7...}`. Receipt: phase `keygen`, command `unshare -Urn -- env RAYON_NUM_THREADS=2 <ELF> keygen <config> faa226f9... <keys dir>`, exit **0**, stop **null**, sourcebinary **true**, elapsed **183.931 s**, RSS **2415271936**, CPU **206.45**. Log: `NATIVE_KEYGEN_COMPLETE; no financial proof/WF/application/ledger acceptance`.

`keygen-success.json`: `keygen_complete` true; `tagged_roundtrip_eof_identity` true; `k` 17; `input_ir_sha256` `c20c6e73...`; `srs_bytes` 25166212; `authority_valid` **null**; `resolver_registration_key_agreement` `NotChecked; required in future actual finalized proof`; `srs_ceremony_independently_audited` **false**; `proof_produced` / `well_formed_checked` / `ledger_applied` / `ledger_accepted` all **false**. Artifacts: PK 234911946 `28fbadfa...`, VK 2745 `6152e754...`, IR tagged 25252 `a5a6f501...`. Directory set is exactly those three tagged files plus `keygen-started.json` and `keygen-success.json`.

Source path: `keygen.rs` loads JSON IR via `IrSource::load`, requires k==17 and SRS length 25166212, decodes parameters with EOF, then `ir.keygen(params)` which is public `Zkir::keygen`. Pinned `zkir/src/ir.rs` (SHA `61118e73...`) implements that trait with `setup_vk` then `setup_pk`. Tagged roundtrip uses `load_prover_key_from_tagged` / `decode` / `load_ir_from_tagged` with full EOF and re-encode identity. Official zkir CLI (`zkir/src/main.rs` SHA `a6e8f3fb...`) uses the same `keygen` API with `FetchMode::OnDemand`. This consumer uses the fixed offline `Parameters` reader. The OnDemand path is the official CLI, not this caller.

Compiled ELF strings: FOUND `NATIVE_KEYGEN_COMPLETE`, `setup_vk`, `setup_pk`, `IrSource`, pay SHA `c20c6e73...`, `proof-verifying`, `load_ir_from_tagged`, `load_prover_key_from_tagged`, `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, `CONDITIONAL_TRUSTED_GENESIS_NATIVE_LEDGER_SUCCESS`, `MockProver`, `OnDemand`. ABSENT `test-utilities`, `mock-verify`. `undeployed` is absent from the binary (network id is config JSON). `Cargo.toml` enables `ledger` with `default-features = false, features = ["proof-verifying"]`. `Cargo.lock` contains 391 `name =` entries (360 unique names); metadata receipt records 391 packages, 391 resolve nodes, 1220 edges, stdout SHA `37cb2e53...`. Lock has no `mock-verify` or `test-utilities` feature strings. Unified tokio features in the metadata receipt include `net`; actual build/keygen deny is `unshare -Urn`. MockProver/OnDemand bytes are linked-crate strings. Consumer source does not enable mock-verify or test-utilities, and does not construct OnDemand.

README of the candidate still labels the tree "Uncompiled/unexecuted isolated next-capability candidate." That label is as-of source preparation. Current receipts are the execution authority. v3's missing produced binary caused the historical source-identity label; independent hashes remain qualified by that history.

## 7. Historical failures preserved

| Stage | Actual evidence | Disposition |
|---|---|---|
| v3 build | exit 101, `stop_reason` `source identity changed during phase`, `produced_binary_sha256` null, source_identity_preserved false | consumed; no produced ELF |
| v4 build | exit 0, ELF `034ed49b...` 624060592 bytes, source/binary preserved | compile passed; old ELF retained |
| v4 handoff | exit 1, TypeError `Cannot use 'in' operator to search for 'popeq' in member` at `handoff.ts:99` filter `'popeq' in v` on string opcode `member` | consumed |
| v5 object-guard repair | `v!==null&&typeof v==='object'&&'popeq'in v`; handoff exit 0; prepare exit 0, 47.48 s, RSS 202268672, CPU 46.45; still old ELF `034ed49b...` | actual keyless construction |
| v6 SRS | exit 1, TimeoutError, 327680-byte `srs.part` SHA `242070c2...`, 13.528 s, no final `bls_midnight_2p17`, no v6 build/keygen attempts | consumed transport; no retry as fifth Compact/R3 |
| v6 retention | fsynced 624060592 bytes SHA `034ed49b...` to `native-keygen-v6-retained-v4-binary/` | old archive preserved |

No reset of those consumed reservations. Exact k17 / 114250 model remains. No fifth Compact compile.

## 8. v5 construction retained as producer projection

Handoff and prepare both exit 0. Eight call-context fields all true (`own_address`, `caller`, `balance`, `tblock`, `tblock_err`, `parent_block_hash`, `com_indices`, `last_block_time`). Seven preimage fields all true (`inputs`, `private_transcript`, `public_transcript_inputs`, `public_transcript_outputs`, `binding_input`, `communications_commitment`, `key_location`). Constructed and frozen preimage SHA both `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402`, 4316 bytes, `preimage_equal` true. Runtime 293 ops, 47 popeq-object reads. Skips JSON length 293, every element null. Preliminary fee `1128519108356650` ≤ allowance `1e20` ≤ generationless available `5e21`. TTL 1000300 − 1000000 = 300; default global TTL 3600. Native parameter SHA `38d99ef89d303fc126754e68b35c79a1804abcf734f627f35748d164fe503c29`. Partition: `guaranteed=None`, `fallible=Some(...)`. Transcript unit strings: `member` 6, `pop` 2, `lt` 1, `neg` 4, `add` 4 (17 total); both `jmp` objects `{skip:1}`. Whole consumer financial source checks exist; they are not yet an actual proof.

Production/successor files closed in INPUT include worktree `packages/moriarty-beta/src/{auth,bridge,frontend,json}.ts`, signed-intent example program/scenario/signature, and successor `mil4-s0-source-v6.ts`, `mil4-s0-core-v5.ts`, `financial-agreement-source-v6-frontend.ts`. Kernel `node_modules` is a symlink onto `compiler-probe/runtime/node_modules`; 240+240 file sets equal; same physical tree.

## 9. Wiki archive

`wiki-llm/native-financial-kernel-2026-10-01/evidence/native-keygen-v7-build-srs-keygen-history`: 43 files. Overlap with RESULTFREEZE 41 small files, hashes equal. PK, full SRS body, and `srs.part` are absent from the archive (externally preserved). Archive extras: `NATIVE-KEYGEN-V7-RESULT-FREEZE.json` and `archive-manifest.json`. New 696 MB ELF and old 624 MB ELF are external, not copied into git.

## 10. Vote A — actual v7 result publication

**APPROVE** exact actual v7 build + SRS + keygen **narrow result publication**.

Grounds: all three phases exit 0, no stop, source and binary identities preserved; produced ELF SHA/size match freeze and live file; official `Zkir::keygen`/`setup_vk`/`setup_pk` is in both source and compiled strings; tagged EOF identity is in the success receipt and in `keygen.rs`; SRS body verified at 25166212 / `4a9ef6c7...` with no resume fallback; config IR is original JSON pay.zkir `c20c6e73...`; keygen flags correctly record no proof, no well-formed, no apply, `authority_valid` null, resolver NotChecked, ceremony unaudited.

This approval publishes the actual keys/SRS/ELF/receipts as the v7 result freeze. It does not accept a financial proof, well-formed transaction, ledger apply, Preview settlement, or ceremony.

---

# Scope B — v8-03 source and resources

## 11. Critical format: JSON constructor vs tagged resolver

`construct` (`main.rs`) calls `IrSource::load(irbytes.as_slice())`. Pinned API `IrSource::load` parses JSON, versions 3.0..=1. ProveConfig IR **must** remain original raw JSON `kernel-prototype/output/zkir/pay.zkir` SHA `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae`. INPUT `prove_config.ir` is that pair. Wrapper asserts the same path/SHA and equality with the retained v5 producer projection (`native-ledger-v5-keyless-prepare-config.json`) plus new PK/VK/SRS.

Provider internally `tagged_serialize`s the parsed `IrSource` into `ProvingKeyMaterial.ir_source`, then `IrSource::load_ir_from_tagged` at resolve/check/prove. That tagged encoding is the resolver artifact. It is distinct from constructor JSON.

Unexecuted `run-native-financial-proof-v8-draft.py` SHA `6a5cdad9...` wrongly mapped ProveConfig `ir` to `native-keygen-v7-keys/ir.tagged`. Corrected `...-v8-draft-02.py` SHA `a424f7dc...` and current `...-v8-03-bounded.py` SHA `f5af2a11...` preserve original JSON pay.zkir. No current Rust change was required. Actual keygen `ir.tagged` SHA `a5a6f501...` (25252 bytes) is an identity artifact of keygen. It is never JSON constructor input.

## 12. Native prove / verify / accept consumers

Read in full: candidate `src/{main,keygen,provider,artifacts,verify}.rs`, `Cargo.toml`, `Cargo.lock`; consumer copies of provider/verify/artifacts/Cargo.toml/lock (byte-identical); consumer `main.rs` differs only by absence of the keygen module/CLI arm.

**Prove path (source, unexecuted).** `Transaction::prove` on the constructed unproven transaction with one shared `Provider` (cloned budget). Official `ledger/src/prove.rs` 253–391: check skips, insert/coalesce Noops, add noop gas, build intermediate call, then `prover.prove(preimage, Some(intermediate_call.binding_input(binding_commitment)))`. Tag `verifier-key[v7]` selects `ProofVersioned::V3`. Consumer provider refuses `None` binding and zero `Fr`. Resolver admits only `LOCATION` `local-public-fixture/pay/c20c6e73...`. Budget: `attempts==0` then increment; exactly one record. `ir.prove` returns `(proof, pis, skips)`; skips must equal prior check; VK verifies against SRS-derived `ParamsVerifier` **and** `PARAMS_VERIFIER`. `main::prove` requires `pis == rec.pis` and `pis.first() == Some(&rec.binding)`, then sign/seal, then asserts public inputs unchanged. `accept`: `WellFormedStrictness::default()`, `LedgerState::apply` whitelist `None`, require `TransactionResult::Success`, escrow 8990, expected storage, A1 outputs 02=1000 and 03=10, no other family, NIGHT principal conserved, funding consumed, positive DUST debit `<= allow <= available` with remainder, identical replay `Failure` with unchanged state hash. Partial/guaranteed evidence is written before refusing a non-Success claim.

Fixture: `trust` `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, `network` `undeployed`, Schnorr signing key `from_bytes(&[0x42;32])` (public development fixture, never a wallet import). ECDSA intent owner (handoff signed-intent example) is a distinct role from the Schnorr NIGHT payer. Signed A fee 10 is a distinct asset from native DUST. Constructor time/`trustedRound` is unauthenticated stored data.

**Verify path (source, unexecuted; only after actual proof).** Separate process. Wrapper binds verify authorization to root-exclusive `NATIVE-FINANCIAL-V8-03-PROOF-RESULT-FREEZE.json` closed `{sha256: absolute map}` that must include parent prove receipt and every proof output file. Child manifest has no authority. VerifyConfig uses actual VK/SRS/raw proof/canonical statement/final tx/genesis/expected/fixed fixture. EOF on VK, proof, statement, transaction, genesis; registered v3 VK equals supplied VK; proof bytes and public inputs equal; binding nonzero; every-PI +1, missing field, extra field, address, entry, communication, gas, effects, proof bit/truncation/suffix, decoder suffix, missing ownership signature; then pristine `accept` again; wrapper requires independent receipt `==` original prove receipt and complete proof output set unchanged; config hash preserved.

Implemented negatives and replay/partial-state/fee retention are honest boundaries of this candidate. Broader independent financial matrix remains specified-only: wrong registered/resolver keys, registration signature, altered funding/dust/allowance/time, stale state/work/head, competing candidates. This report fabricates no coverage for those.

New wrapper postconditions (on future actual output, not yet present): `result==Success`, `proof_count==1`, `runtime_ops==293`, `runtime_reads==47`, statement/genesis/poststate hashes, `0 < fee <= allow <= available` and `fee+remainder==available`, required artifact set, native unknown-failure logs preserved via exclusive receipts/stop.

## 13. Completeness of closed INPUT / wrapper gates

Wrapper `verify()` rehashes INPUT 667, RESULTFREEZE 44 relative to B, candidate 9, inherited 39+260, runtime 240+240 exact sets and symlink resolution, v5 34, ELF size/SHA, and asserts all three v7 receipts: exit 0, stop null, sourcebinary true, matching source/config/SRS/keys flags, acquisition verified/no resume, keygen success schema, no v6 final SRS / v6 build / v6 keygen, retained old ELF. ProveConfig closed from frozen v5 producer projection plus actual PK/VK/SRS. Authorization schema: `wrapper_sha256`, `candidate_source_sha256`, `v5_result_freeze_sha256`, `v7_result_freeze_sha256`, `input_freeze_sha256`, two `terminal_v7_result_reviews`, two `resource_votes`, `proof_result_freeze` (null for prove). Commands: `unshare -Urn -- env RAYON_NUM_THREADS=2 <same ELF> prove|verify CONFIG CONFIG_SHA NEW_OUTPUT`. No build, new keys, fetch, helper export, or second proof.

This auditor independently confirmed those maps (667/667, 44/44, 9/9, 12/12, 39+260, 240+240, 34/34, ELF, keys, SRS, pay.zkir). Semantic review of the consumers is in §§11–12. History and archive are in §§7–9.

## 14. Resource request and fit

| Phase | Wall | Group CPU | Group RSS/AS | Notes |
|---|---|---|---|---|
| Proof | 600 s | 1200 s | 4 GiB | Rayon 2; exclusive `native-financial-v8-03-proof` |
| Independent verify | 120 s | 240 s | 2 GiB | only after proof parent 0 / closed PROOF-RESULTFREEZE |
| Bundle | 720 s | 1440 s | parent hash outside child | combined output 512 MiB, per-file 2 GiB |

Disk: existing target 10 GiB total / +2 GiB incremental / +1 GiB cache / free floor 10 GiB. Existing keys, SRS, and old ELF are prior baseline, outside the new proof+verify output cap. Current actual target 8503734623 / cache 2933611498 / free 20610478080 (fresh). Group CPU/RSS sampled 0.1 s, disk 5 s, cooperative overshoot possible. Per-process AS/CPU/FSIZE supplemental. Rayon environment is not a global thread cap. Ownership-safe catchable SIGINT/TERM cleanup; SIGKILL/power loss leaves durable reservation. Exclusive attempt/config/log/receipts; dependent stop; no retry.

**Proof fit and every-PI verification cost are unmeasured.** Keygen 183.93 s / 206.45 CPU / 2.4 GiB RSS and build 24.63 s / 36.71 CPU / 3.26 GiB RSS do not establish proof or verify duration or RSS. pay.zkir contains 61 `public_input` ops; `verify.rs` loops every PI plus extra/missing/address/entry/communication/gas/effects/proof mutations. 120 s verify wall is a **bounded diagnostic qualifier**. It is not a specific blocking defect on this source. Caps are proposed diagnostic ceilings.

## 15. Votes B — proof, independent verification, bundle

**PROOF: APPROVE-BOUNDED** the exact v8-03 wrapper/input/proposal/candidate/v7-result/v5-producer projection for one finalized `Transaction.prove` at 600 wall / 1200 group CPU / 4 GiB group RSS-AS / Rayon 2, same compiled ELF, target offline `unshare -Urn`, no build/new keys/fetch/helper export/second proof.

**INDEPENDENT VERIFICATION: APPROVE-BOUNDED** the same frozen sources for one separate verify at 120 wall / 240 CPU / 2 GiB, only after actual proof parent exit 0, no stop, sourcebinary true, whole output hashes, **and** root-exclusive `NATIVE-FINANCIAL-V8-03-PROOF-RESULT-FREEZE.json` closed absolute SHA map of parent receipt plus every file. Separate verify authorization binds that exact root freeze.

**BUNDLE: APPROVE-BOUNDED** 720 wall / 1440 CPU ceilings plus parent hash outside child, with the stated disk floors and the prior keys/SRS/old-ELF baseline excluded from the new 512 MiB output cap.

Approve-bounded means: source and wrapper gates are complete for this exact candidate; resource ceilings are authorized as diagnostics; unmeasured proof/PI-verify fit remains a qualifier; resulting actual proof and verify receipts still require independent result audits before financial publication. Old v7 resource votes do not allocate v8.

Root may run only after two terminal reviews agree on the exact current scopes (this Grok report plus the independent Astra counterpart, hash-bound). Successful execution would establish conditional local ledger9 acceptance from the explicit trusted genesis fixture and production Beta/Core handoff. It would not close generic compiler/property/intent/transition/history/PCD, authenticated mint/deploy/funding/custody/accounts/assets/time, or Preview.

---

# Scope C — Beta CLI one-line display publication

## 16. Exact current files and git original

`BETA-CLI-FOLLOWUP-CANDIDATE-FREEZE.json` inner map, all 6 live matches:

| File | SHA-256 |
|---|---|
| `packages/moriarty-beta/src/cli.ts` | `5148fdcafb1515b2e7badaf78aa986c08d43f876fe62417e74395760f043d952` |
| `packages/moriarty-beta/src/json.ts` | `e16318504647b6fbd1be4286d1553a6f116da25eb898a518ddb70a822d6e0624` |
| `packages/moriarty-beta/src/intent-display.ts` | `8e935906b9a020edec7f684936db76b2d2b59f1e2bfea832ff8fc87dc9448b42` |
| `tests/intent-display-error.test.mjs` | `cef119a9f598855087e4f3cec8be34618ce172e9750d3dda8a7b3121cd3c2cde` |
| `tests/intent-display.test.mjs` | `02ffd4ec99f3903192ff017f4cc05f07163cae877461c6d7d0c3befa60d1c883` |
| `tests/json.test.mjs` | `f4782092ae9dc8ae6a3922c6331dc1227552e7d161e78777bf8f7fe8f74a0311` |

Git porcelain: only `M packages/moriarty-beta/src/cli.ts`. `json.ts`, `intent-display.ts`, and the three tests equal HEAD. HEAD `cli.ts` SHA `35c2c5bd145d201eeac918c8de3ffe196e449cc4cb5f950946621cb41e547b08`. Diff is one line in `main().catch`:

```
-  const extra=... ${escapeAscii(asciiJson(error.claimed??null,0))} ... ${escapeAscii(asciiJson(error.computed??null,0))} ...
+  const extra=... ${asciiJson(error.claimed??null,0)} ... ${asciiJson(error.computed??null,0)} ...
```

`asciiJson` already emits printable ASCII plus newline (`JSON.stringify` then non-ASCII `\uXXXX`). `renderErrorReview` still `escapeAscii`s `error.code`, `error.message`, and each `extra` line. The pending delta removes a redundant wrap around an already-ASCII JSON value at extra construction. `--review` still applies one `escapeAscii` pass. Unsigned JSON path writes `asciiJson(result)` unchanged. `process.exitCode` remains 2 for `BETA_CRYPTO_*`, else 1; `verify-intent` still sets 1 when status is not `SignedPreparedUnqualified`. JSON limit wording in `json.ts` (`parseBoundedJson` 65536/1024, depth 32, nodes 4096, fields 64) is already published PR14 and is unchanged.

Freeze records a historical root test command `env MORIARTY_REQUIRE_NATIVE=1 MORIARTY_CRYPTO_BINARY=.../moriarty-midnight-crypto node --test packages/moriarty-beta/tests/json.test.mjs packages/moriarty-beta/tests/intent-display-error.test.mjs` with 5 pass / 0 fail / 0 skip. This auditor did not execute tests. That count is not a broad build or acceptance claim. Existing red/green archives remain historical. Direct regression coverage of this extra-line construction is not invented from the 5-test count. Broader native financial predicates remain open.

## 17. Vote C — CLI source publication

**APPROVE** publication of this exact one-line Beta CLI display fix (current `cli.ts` SHA `5148fdca...` versus HEAD `35c2c5bd...`), with the other five freeze files unchanged. Display safety of review extras remains `renderErrorReview`'s `escapeAscii`. Exit status is unchanged. This vote is independent of votes A and B and of native financial proof.

---

# 18. Blockers, repairs, and remaining failures

Current failures and qualifiers audited together:

1. **No actual v8 proof/verify yet.** No reservations/outputs. Financial publication is blocked until actual proof + independent verify + result audits.
2. **Unmeasured proof and every-PI verify cost.** Diagnostic qualifier on the 600/1200/4GiB and 120/240/2GiB caps. Not a specific source defect.
3. **Ceremony unaudited.** SRS digest verified; `ceremony_independently_audited` false.
4. **Resolver registration NotChecked; `authority_valid` null.** Required on the future finalized proof.
5. **Broader financial matrix specified-only** (registration/funding/dust/time/stale-head/competing candidates). Implemented negatives: PI mutations, proof corruption, decoder suffix, missing ownership signature, replay, partial-state evidence on non-Success.
6. **Trusted genesis / fixture key 42 / constructor time unauthenticated.** Conditional local development only.
7. **Generic compiler / property / intent / transition / history / PCD / authenticated mint-deploy-funding / custody / accounts / assets / time** remain open. Keys and a conditional genesis proof do not close them.
8. **Cached Preview** research/source-linkage is older conditional proof/operation/IR versus current 9-target. No live build attestation, network rejection, or wallet public submission. No Preview authority or readiness.
9. **Guarded SP01 admission** remains blocked (stale binding/candidate, missing accounting, unavailable live resource). Unrelated to this native-keygen/v8 review scope.
10. **v3–v6 historical failures** stay on disk. No retry-as-reset. No fifth Compact/R3.
11. **Draft-01 tagged-IR constructor bug** is unexecuted and is not in 03. Repair already present in 02/03 wrapper. No further Rust repair required for that defect.

No source repair is assigned to this auditor. Root closed authorization, if both terminals agree, is the next administrative step for v8 execution. CLI publication may proceed on vote C independently.

## 19. Limitations of this audit

- Read-only. No execution of wrappers, Cargo, Node tests, native prove/verify, or network.
- ELF string presence is compiled-closure evidence; it is not a full disassembly of every linked crate.
- Availability `5e21` and fee estimate are v5 keyless-preparation receipts; final consumed DUST after a real VK/proof size is unmeasured.
- Maintainer resource votes are not public developer compile/prove/deploy prerequisites.
- Requested identity and actual returned attestation: this report is Grok 4.6 high on the assigned frozen scopes. Host/model routing outside this file is not re-attested here.
- Other auditor outputs were not used.

## 20. Final terminal path and verdict

**Terminal path.** Root may treat this file as the Grok member of both `terminal_v7_result_reviews` and `resource_votes` because both precise frozen scopes received independent substantive votes above. Closed v8 authorization requires the Astra counterpart with the same two families, exact report hashes, and `approved: true`, interpreted by root. After that: `python3 run-native-financial-proof-v8-03-bounded.py prove AUTHORIZATION AUTH_SHA`. On actual proof parent 0 / no stop / sourcebinary true / native postconditions, root writes exclusive `NATIVE-FINANCIAL-V8-03-PROOF-RESULT-FREEZE.json` as a closed absolute SHA map of the parent receipt and every proof file. Then a **separate** verify authorization pinned to that freeze: `python3 run-native-financial-proof-v8-03-bounded.py verify VERIFY_AUTHORIZATION VERIFY_AUTH_SHA`. Actual proof and verify result audits remain mandatory before any financial publication. CLI source publication is a separate, already-voted scoped action.

**Verdict.**

| Scope | Vote |
|---|---|
| (A) exact actual v7 build+SRS+keygen result publication | **APPROVE** (narrow; not financial) |
| (B) v8-03 proof 600/1200/4GiB | **APPROVE-BOUNDED** |
| (B) v8-03 independent verification 120/240/2GiB | **APPROVE-BOUNDED** |
| (B) v8-03 bundle 720/1440 + disk floors | **APPROVE-BOUNDED** |
| (C) exact one-line Beta CLI display source publication | **APPROVE** |

End of report.
