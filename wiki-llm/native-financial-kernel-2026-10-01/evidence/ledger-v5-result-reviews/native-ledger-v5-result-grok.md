# Native ledger v5 result audit

Requested identity: Grok 4.6 high.
Returned identity: Grok 4.6.
Primary checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.
Evidence root B: `/home/charl/research/moriarty-signed-intent-2026-10-01`.
Authority: `NATIVE-LEDGER-V5-RESULT-FREEZE.json` recorded_at `2026-10-01T09:06:40.000276+00:00`.
Inspection: read-only hashing and parsing. No wrapper import/exec, Cargo, Node, native binary launch, key/SRS/proof/WF/apply, wallet, or network.

Moriarty develop skill loaded from `plugins/moriarty-dev/skills/develop/SKILL.md`. Guarded `status --json` reports capability `SP01.6 loan-swap-subset`, unresolved operational history/admission/accounting, and empty `pendingTransactions`.

## Verdict

**Approve** the narrow source-to-native keyless preparation result and scoped publication of that result.

The executed path is production signed Beta transfer → Source6/Core5 → generated Compact constructor/pay → official WASM preimage converter → frozen v4 native `prepare`. Handoff and prepare both exited 0. Source and binary identities stayed intact. Independent hashes match the result freeze.

**Refuse** allocation or acceptance of keygen, cryptographic proof, well_formed, apply, Preview settlement, generic compiler/financial correspondence, authenticated custody/time/history, and native PCD.

This is one of two required fresh current audits. Publication of this preparatory result still needs the matching independent audit. Resource gates for later stages stay separate.

## Freeze hashes

Every `sha256` entry in `NATIVE-LEDGER-V5-RESULT-FREEZE.json` matches current bytes. Count 34. Missing 0. Mismatch 0.

The compiled binary at `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer` exists, mode `100755`, size 624060592, SHA256 `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`. That equals the freeze `compiled_binary` pin and the v4 build receipt pin.

Review-freeze overlap (9 source/resource pins) equals the matching result-freeze values.

## Source maps and runtime trees

Current `native-ledger-consumer/manifest.json` SHA256 `bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f`.

- Internal map: 39 files, 39 hash matches.
- External map: 260 paths, 260 hash matches.

`NATIVE-LEDGER-V4-COMPILED-MANIFEST.json` SHA256 `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb`.

- Internal map equals the current 39-file map exactly.
- External key sets are identical (260).
- Hash values differ on exactly two helper keys:
  - `native-beta-handoff/handoff.ts`: v4 `d17f542d5afa2cadd91fa721ec48bcd36f4ec091ab730b33c1aac9c177e61b38` → current `add4ad0be8f2e6e71343bbec9aabaacc9764b3a5bf8d485e3cbdcd4797499a1c`
  - `native-beta-handoff/SOURCE-HASHES.json`: v4 `bfebf8e7e117e8029dfb667aabe00df1836a776c955b5129e1512c7cb24bf05f` → current `20a8aaf775fed2a2ed1bfba87c811aceaa6924e9ecda030266b63143490ff3dc`

Those two deltas are the Popeq unit-string guard recorded in `NATIVE-BETA-HANDOFF-RUNTIME-REPAIR.patch`. Rust sources, Cargo.toml, Cargo.lock, official ledger/ZKIR pins, production Beta/auth/bridge/frontend/json, Source6/Core5, generated kernel/IR/preimage, and public transfer fixture hashes are unchanged from the compiled v4 map.

`NATIVE-LEDGER-RUNTIME-INPUTS-V5.json` SHA256 `68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425`.

- `files_sha256`: 240 files, 240 matches, under `compiler-probe/runtime/node_modules`.
- `kernel_files_sha256`: 240 files, 240 matches, under `kernel-prototype/node_modules`.
- Relative path sets and hashes are equal.
- `kernel-prototype/node_modules` is a symlink to `compiler-probe/runtime/node_modules`.
- `package-lock.json` SHA256 `9e0b397d1d5a391aa337fc3ce31b815e9d1192ae1621ecb75dd2764bdef6b02e`.

Current consumer Rust identities:

| file | sha256 |
| --- | --- |
| src/main.rs | 2d9ddbc8e7b4919e150c5ce7543c62d7225278fcd19dfd8e6bcf934d8add297b |
| src/artifacts.rs | 4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7 |
| src/provider.rs | 1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f |
| src/verify.rs | 8e9c59d0d80efbeb28a5d2f84d899be1dc5d732833178d4025656e69c8fe3c8b |
| Cargo.toml | 78a76d91906032a331d6e4d91359312544289f6a0c69f606daea4f62645e649c |
| Cargo.lock | b0cddb391929e964c82b1587675d9bd5079413e8461b2f2ac2b5e72b8de9b11e |

Cargo.toml enables `proof-verifying` on `midnight-ledger-v9` rev `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` and pins ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914`. The lock graph has no `mock-verify` or `test-utilities` feature. `Cargo.lock` equals `Cargo.lock.after-metadata`. Metadata receipt records 391 packages after lock repair from baseline `2c07881f96e2a19fde1e21b0421f8173dee4c44885482a666081ffdc4866c6fd`.

The current manifest `scope` string still says "compiler repairs and production Beta handoff unexecuted". That prose is leftover v4 wording inside an otherwise current hash map. Map bytes remain the authority.

The keygen tree `native-ledger-keygen-candidate/` is a separate uncompiled source. Its `src/main.rs` SHA256 is `393efec218b0816e9c6bbc792da37e9a7f32b75719781c19d0c3e9d346945435`. That differs from this consumer. Its README and `NATIVE-KEYGEN-INDEPENDENT-EXPECTATIONS.md` keep keygen out of this candidate and this authority.

Prove, verify, and accept source in this consumer remains unexecuted. Provider `prove` and `Transaction::prove` were not invoked. No PK/VK/SRS files appear in v5 outputs.

## Historical compile and handoff

Original `native-ledger-v3-build.log` contains nine compiler diagnostics:

- two `E0277` `SplittableRng` on `&mut StdRng` for `Transaction::seal`
- one `E0277` `LowerHex` on the SHA256 digest array
- five `E0382` moved `parent.binding_commitment`
- one `E0005` refutable `ProofPreimageVersioned` pattern

v3 build receipt: exit 101, elapsed 38.3608s, `produced_binary_sha256` null. `COMPILE-REPAIR-v1.md` and patch record the source repair now present in current `main.rs`/`artifacts.rs` (owned `StdRng` to `seal`, manual hex encoder, cloned commitments, explicit unknown-preimage refusal).

v4 build receipt SHA256 `df9cc75d34d9f24bd0ec94deab880a5db76d243fa76e77902829c613d095e03b`: exit 0, stop null, elapsed 20.2709s, binary `034ed49b…6158688`. That is the reused executable.

v4 handoff log retains `TypeError: Cannot use 'in' operator to search for 'popeq' in member` at `handoff.ts:99` under Node v24.21.0. v4 handoff receipt: exit 1, elapsed 1.4879s, no output directory. The mixed transcript contains 276 objects and 17 unit strings. The v5 helper repair adds `v!==null&&typeof v==='object'` before `'popeq' in v`.

Four Compact sources remain: `source-attempt-1.compact` through `source-attempt-4.compact`. Attempt 4 equals `fixed-transfer.compact` SHA256 `a3205e5bed3aabcbf06e7293be3384724498b7fbf1ae15e5c62a4c36486199db`. R3 k17 row exhaustion remains an AGENTS constraint.

## Actual v5 handoff

Reservation `native-ledger-v5-handoff-attempt.json` records exclusive pre-launch state at unix 1790845417.451924. Command is `unshare -Urn -- node …/cli.ts` with public `program.mori` / `pay` / `scenario.json` / `signature.json`, installed `moriarty-midnight-crypto` SHA256 `42fa598d8b54334650f60bc13428bff3d79bb240a5bb0c203fc84684b5a30339`, and exclusive output `native-ledger-v5-keyless-handoff`.

Receipt `native-ledger-v5-handoff-receipt.json` SHA256 `0457a9c61f3daf6f382c39ab6490db26f7d908e57e89ac46966a869ac9ce2ad0`:

- exit_code 0, stop_reason null
- elapsed_seconds 1.5070123699842952
- source_identity_preserved true, binary_identity_preserved true
- binary_artifact_status `prerequisite_v4_binary_frozen`
- produced_binary_sha256 `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`
- peak_sampled_group_rss_bytes 15683584, peak_sampled_group_cpu_seconds 0
- child_launched true
- target bytes unchanged at 7501078730

Helper receipt `native-ledger-v5-keyless-handoff/receipt.json` SHA256 `f754359fac2c39824b8ee1734d20ff0ec8d8fdac2b0193056e7f60b7d2384fd8` equals stdout JSON in `native-ledger-v5-handoff.log`. Status `NativeFixturePreparedUnqualified`. Flags: `authority_valid` null, `ledger_accepted` false, `proof_produced` false, `well_formed_checked` false, `ledger_applied` false. Premises: canonical-intent-signature, snapshot-to-head, head-extension, atomic-ledger-compare-and-consume. Unverified bindings: agreement-id, selected-program, asset-scale, authenticated-predecessor.

Four artifacts:

| name | bytes | sha256 |
| --- | --- | --- |
| initial-contract.tagged | 1432 | 06475852f5b0ee69e3627409bdbcc16005cff2136073443ff1ec58f8c32c7d57 |
| expected-contract.tagged | 1483 | 716eb7ddb665924809efa4a91cd9fb72336c5cd40178d86338e0e75154a8e448 |
| runtime-native.json | 38679 | 329cbc73735d282718b25540a9ccb3ed611a9fef8e2bd5325431cdba2751a049 |
| registered-pay-operation.tagged | 35 | c704fef673e8b562779ff23c5387ef3b5e589038fbdda88c6214e315fee4f7c6 |

Operation tag is `midnight:contract-operation[v6]:` plus three zero bytes. WASM `ContractOperation::new()` calls `state::ContractOperation::new(None, None)` and `verifierKey` is undefined when v2/v3/ir are absent. Handoff asserts that getter, then serializes the empty operation.

Head mapping in the receipt: betaPre `h0` / nativePre `9e4601c9af94102208bd6bdea6693cbf5a9638f6d6b9fa716eae540443cc30e4`, betaPost `h1` / nativePost `4e87ddfda49bafabddab1a05cbdd0538735744a318dda80a211075ebb343b09e`, `authenticates_predecessor` false. Initial tagged bytes contain nativePre. Expected tagged bytes contain nativePost.

Public pins match the frozen transfer fixture: source `fb507bfb848a8859a55c020faf410e52f9e40c00fc3938a90ea3c763e1cf1d6c`, scenario `12928a795bba2435140fc22c0d33f039da3b04c8de6c244d0c4da338900f0430`, signature `116c238a8b18638dc282bc8efb323926ee440fe506aa2eaff0a37d50bf02969f`, projection `6afe4f734ab85f767eb2578a5b5a8699bfda81de360ce50b8fb9cb9cd82ee381`.

Handoff prepare-config SHA256 `452244931d582364128f75c809c7a977e72b14d2f2a63d8667acffbdbec9ee78`. Root prepare-config SHA256 `32441b1c38c6c71fd8dd0b673609859292535686340f10e053138c8e262a88e1`. Parsed JSON values are equal. Serialization differs (compact nativeJson versus pretty JSON). Wrapper compared them as data before launching prepare.

## Actual v5 prepare

Reservation `native-ledger-v5-prepare-attempt.json` records exclusive pre-launch state at unix 1790845433.2206595. Command is `unshare -Urn -- beta-native-ledger-consumer prepare` with the root config pin and exclusive `native-ledger-v5-keyless-preparation`.

Receipt `native-ledger-v5-prepare-receipt.json` SHA256 `7a6d43f63938c6282f916ec5b1f762595e4061d4fabba1c9c5280de82ccf385d`:

- exit_code 0, stop_reason null
- elapsed_seconds 47.47654036601307
- source_identity_preserved true, binary_identity_preserved true
- binary still `034ed49b…6158688`
- peak_sampled_group_rss_bytes 202268672 (about 202 MB)
- peak_sampled_group_cpu_seconds 46.45
- last disk sample at 45.567s, CPU 45.55s, RSS 188145664
- target bytes unchanged 7501078730, cache unchanged 2933611498
- child_launched true
- log text: `NATIVE_PREPARATION_SOURCE_CHECK_OK; no VK/PK/SRS/proof/ledger acceptance`

Combined handoff+prepare output bytes 86498, under the 32 MiB cap. No `application-result.txt`, proof, PK, VK, or SRS artifact exists in the preparation directory.

`PrepareConfig` uses `deny_unknown_fields` and contains only ir, initial_contract, expected_contract, runtime, retained_preimage, fixture. `construct(..., None, ...)` is the prepare path. Prove mode is a later unexecuted function that would pass `Some(vk)`.

## Preimage, context, transcript, partition, check, fees

Constructed preimage bytes 4316, SHA256 `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402`. Frozen copy in the output directory has identical bytes. Both equal `native-adapter-successor-v3/good.preimage`. That equals the freeze entries for `constructed.preimage` and `frozen.preimage`.

`construction-diagnostics.json` records all eight CallContext field equalities true: own_address, caller, balance, tblock, tblock_err, parent_block_hash, com_indices, last_block_time. Debug dump shows address `04`×32, tblock 1000000, tblock_err 0, parent hash 0, last_block_time 1000000, A1 balance 10000, caller User `53d2280d23b40240f82bcf5482961129093c1ada7bf940a8b318cca4dc315bea` (NIGHT Schnorr verifying key from public fixture `[0x42;32]`).

All seven full-preimage field equalities are true: inputs, private_transcript, public_transcript_inputs, public_transcript_outputs, binding_input, communications_commitment, key_location. `preimage_equal` true. `runtime_ops` 293, `runtime_reads` 47.

`runtime-native.json` publicTranscript length 293: 276 objects and 17 unit strings. Unit strings: `member` 6, `neg` 4, `add` 4, `pop` 2, `lt` 1. Object ops: `idx` 71, `dup` 65, `popeq` 47, `push` 41, `ins` 32, `swap` 12, `branch` 6, `jmp` 2. `jmp` is two objects, not unit strings. privateTranscriptOutputs length 0. Native `construct` counts `Op::Popeq` as 47 and refuses mismatch. Handoff compared the generated trace to frozen `runtime-proof-data.json` SHA256 `2fb7f8343a5e773820abe9fe7d13f0ebe816a9ad043abb4d794f9450a80f000a` through `nativeJson` after restore, then called official `proofDataIntoSerializedPreimage` from `@midnightntwrk/onchain-runtime-v4` WASM. Converter source is `midnight_onchain_runtime_wasm_bg.js` wrapping `wasm.proofDataIntoSerializedPreimage`. Location string is `local-public-fixture/pay/c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae`. pay.zkir header is IR major 3 minor 1. Native load requires `ir.k()==17`.

Partition file: `guaranteed=None`, `fallible=Some(Transcript)`. Fallible effects: unshielded_outputs A1=1010, claimed_unshielded_spends 02=1000 and 03=10, empty nullifiers/shielded/mints/inputs/calls. `financial_outputs` in `main.rs` requires that exact shape. Official `PreTranscript::split_at` (`construct.rs` 768–812) executes guaranteed ops, clones context, resets effects, then executes fallible ops. Empty guaranteed program yields `None`. Transcript version major 2 minor 3.

`require_prepare_operation` refuses any incoming initial, incoming expected, or constructed genesis pay operation with v2/v3/ir Some. Prepare calls it on all three because `vk` is None.

`ir.check(p)` is the public `Zkir::check` on `IrSource` (`zkir/src/ir.rs` 76–80), which returns `preprocess(preimage)?.pi_skips`. `native-check-result.txt` is `Ok([None × 293])`. `native-check-skips.json` is a JSON array of 293 `null` values. `None` skips do not encode active block widths. This is transcript preprocessing, not proof generation.

Preliminary fee artifact:

- method `Transaction::fees_with_margin(default LedgerParameters,2)`
- margin 2
- estimate `1128519108356650`
- allowance `100000000000000000000`
- generationless_available `5000000000000000000000`
- estimate_within_allowance true
- native_parameter_sha256 `38d99ef89d303fc126754e68b35c79a1804abcf734f627f35748d164fe503c29`
- unproven true, vk_present false, keyless true
- limitation text records missing final proof/signature/Noop/VK size

Official `fees_with_margin` uses synthetic `cost(params,false)` and `max_price_adjustment().powi(margin)` (`structure.rs` 1929–1941). Genesis refuses `fee_allowance > availability`. Prepare refuses `preliminary_fee > fee_allowance`. Recorded relation: estimate ≤ allowance ≤ generationless availability.

Independent dust arithmetic from pinned `INITIAL_DUST_PARAMETERS`: `night_dust_ratio = 5 * (SPECKS_PER_DUST / STARS_PER_NIGHT) = 5e9`, `generation_decay_rate = 8267`, NIGHT value `1e12`, age `1e6` seconds. `vfull = 5e21`. `age * value * 8267 = 8.267e21`. Clamp is `5e21`, matching `generationless_available`. TTL 1000300 minus block 1000000 is 300 seconds, under recorded default global TTL 3600.

Expected-contract scope in both handoff receipt and construction diagnostics is storage-only. Native genesis requires A1 escrow 10000. Replay compares `replay.context.state` to `expected.data`. No `LedgerState::apply` output exists. Partition effects would debit 1010 A1 if applied. There is no applied-8990 evidence.

## Executed Source6 / Core5 / production caller

Handoff `cli.ts` takes six arguments and calls `prepareExactNativeTransfer`. That function pins `INPUT-SOURCE-HASHES.json` SHA256 `808605933655ebaafb78ac386404cc2ddbf801a15072d5a2a181802f28f6f476`, then imports production:

- `packages/moriarty-beta/src/auth.ts` `bbddda1d5fe9e2148d29fb1983f0a842dfe52d2677402265f17acd72019153e0`
- `json.ts` `e16318504647b6fbd1be4286d1553a6f116da25eb898a518ddb70a822d6e0624`
- `bridge.ts` `d4ca083798688a4b1d8e77da1beb28cbc2e04731ea550ba717ff908ef7c695fb`
- `experiments/.../financial-agreement-source-v6-frontend.ts` `f3d2a05d07b051d2097fd7fb00ed1cd013eba91e9da7a2591b6f69707d4a02e7`

`auth.verifyAndPrepare` is the production function. It uses bounded JSON, native intent-build/intent-verify, `expand`, `parseAndLowerSource6`, and Core `simulate`. Success status is `SignedPreparedUnqualified` with `sourceMatched` true, `signature_valid` true, `authority_valid` null, `nativeProof` NotChecked, `ledger` NotSubmitted, `ledger_accepted` false, `state` LocalStipulationOnly, and the four premises / four unverified bindings. Handoff asserts that entire receipt, then compares `verified.local.result.candidate` to the pinned projection expected value.

Core5 `mil4-s0-core-v5.ts` SHA256 `855efd40083618e36e4d12a9c16202a1cb8761d3438952551efd72b32e67beda` emits the six Transfer effects for this fixture: Debit Owner A 1010, Credit Recipient A 1000, Credit Fee A 10, UseAllowance 1010, UseReplay `["Midnight","Owner","n1"]`, AdvanceHead h0→h1. Handoff asserts that list and poststate 8990/1000/10, allowance 8990/1010, work 9/1, used replay, head h1.

Source6 frontend parse/lower supplies prestate domain Midnight, asset A, head h0, round 1, balances 10000/0/0, allowance 10000/0, work 10/0, empty obligations, unused replay. `mil4-s0-source-v6.ts` SHA256 `1b3bd3b01ed2694ea967f62fea6808843a198045d870270b1d44c121081f66d8` is the Source6-to-Core5 module. Handoff uses frontend parse plus Core candidate equality, which is the executed consumer path.

Generated `kernel-prototype/output/contract/index.js` SHA256 `98649b5b07c7bc8a8cc9dd40fc86db56c695e94a04ca134082d35a1f7629f61b` requires Compact runtime 0.20.0. Ledger getters exist for all 21 native fields asserted by handoff: ownerKey, sourceDigest, ownerProgramDigest, trustedRound, assetColor, recipientAddress, feeAddress, usedNonce, head, revision, ownerBalance, recipientBalance, feeBalance, workRemaining, workSpent, allowanceRemaining, allowanceSpent, lastDebit, lastRecipientCredit, lastFeeCredit, lastAllowanceUse. Head uses `CompactTypeVector(16, CompactTypeBytes(32))` over little-endian 32-byte scalar casts of 8990, 1000, 10, 8990, 1010, 9, 1, 1, 1 plus the seven digest/address inputs.

Constructor sets trusted 04/A1/02/03, escrow 10000, work 10, allowance 10000, round 1. Handoff then writes `balance` 10000 onto the contract state. That is a trusted genesis mapping. Owner ECDSA public key comes from the signed statement. NIGHT payer is a separate Schnorr development key. Signed A fee 10 is the 03 output. Protocol DUST is the registration allowance/estimate path.

JSON flags in the helper receipt are written only after those assertions and exclusive `wx` publication. Wrapper re-validates status, four premises, four bindings, null authority, false proof/WF/apply/ledger, exact head mapping, four artifact names/paths/hashes/sizes, and closed config equality.

Early-exit success is not fault-matrix coverage. The independent refusal tables in the expectation documents were not executed as mutated inputs in v5.

## Wrapper prerequisites, caps, sampling

Wrapper SHA256 `371df382f0758a86428d4a899c6b5f9ef7f695b78fe398c16e3d5903fb9af7be`. Phases are exactly `handoff` and `prepare`. There is no build branch. Prerequisites parsed from source:

- current manifest pin `bf9db118…`
- runtime v5 pin `68afbf29…` plus 240+240 file-set and symlink closure
- compiled v4 manifest pin `a8a8825c…` with equal internal map and exactly the two helper external deltas
- v4 build receipt pin `df9cc75d…` with exit 0, stop null, matching binary
- actual binary rehashed before launch and after the phase
- exclusive `O_EXCL` attempt, log, receipt, output directory, and prepare config
- prepare requires frozen successful current-manifest handoff plus independent closed five-artifact config

Recorded limits: wall 60s, group CPU 120s, RSS/AS 2 GiB, FSIZE 2 GiB, total target 8 GiB, free floor 10 GiB, incremental target 2 GiB, cache 1 GiB, combined outputs 32 MiB. Child runs under `unshare -Urn` and `RLIMIT_AS`/`CPU`/`FSIZE`.

Sampling disclosure from wrapper source, compared with receipt metadata:

- RSS and CPU are sampled in the child-wait loop from `/proc/[pid]/stat` about every 0.1s.
- Disk is sampled every 5s.
- Wall/CPU/RSS/disk stops are evaluated in that loop only.
- `elapsed_seconds` starts after reservation fsync and includes child runtime plus parent post-child postcondition and identity rehash. Pre-launch `verify()` hashing is before that timer.
- `traffic()` reads `/proc/net/dev` once before launch and once after cleanup. Receipt field `global_nonloopback_received_byte_delta` is that host-wide non-loopback RX difference (handoff 55889, prepare 412246). It includes other host activity.
- Receipt `monitor_limitations` claims traffic sampled at 0.1s. Wrapper source samples RX only at start and end. The 0.1s RX claim is inaccurate.
- Receipt `limits.cargo_jobs` and `rayon_threads` are 2. Helper commands do not set `CARGO_BUILD_JOBS` or `RAYON_NUM_THREADS`. Those fields are inert for these helpers. v4 build did set them.
- Namespace isolation and global RX observation can both be true. This audit does not infer zero host traffic or a hard two-thread execution cap for the helpers.

Parent hashing and postchecks sit outside child CPU/RSS samples. Recorded elapsed for prepare (47.477s) is larger than the last 5s disk sample (45.567s). Peak CPU 46.45s is a sampled child-group value, not a claim of isolated host-wide CPU.

## Fixture and Preview remaining obligations

Trusted public development fixture remains: address 04, color A1, destinations 02/03, escrow 10000, round 1, network undeployed, block 1000000, NIGHT creation 0 / value 1e12, allowance 1e20, TTL 1000300. These are constructor/genesis assumptions. They are unauthenticated as deployment, funding, custody, or chain time.

Owner ECDSA and NIGHT Schnorr payer are distinct. Signed A fee 10 is distinct from protocol DUST. Opaque h0/h1 maps to native hashes through `sha256("local-fixture:h0")` and Compact `persistentHash`. That mapping does not authenticate predecessor or history.

Generic compiler, contract properties, intent refinement, transition validity, history compliance, and PCD remain open. Four Compact attempts and R3 k17 debt remain.

`PREVIEW-COMPATIBILITY-20261001.md` and `preview-source-linkage-20261001/COMPATIBILITY-SOURCE-LINKAGE.md` were read as research evidence. Live Preview reported ledger 8.1.2 / protocol 1000300. Published matching-tuple source for that version supports V2 proof / V3 operation / IR 2.0. This candidate requires V3 proof / V4 operation / IR 3.1. That is a conditional published-source format incompatibility. This audit observed no network rejection and no live-build attestation.

## Publication scope

Approved for publication as: actual production signed Beta transfer to generated native keyless preparation on the frozen v4 compiled caller, with unqualified flags, null authority, and no proof/WF/apply.

Refused as publication of: keygen, SRS body, PK/VK, cryptographic proof, well_formed, apply, Preview readiness, generic financial-language acceptance, authenticated genesis/custody/time, or native recursive PCD.

Independent expectation documents used as specified-only contracts: `NATIVE-LEDGER-INDEPENDENT-EXPECTATIONS.md`, `NATIVE-LEDGER-PREPARE-EXPECTATIONS.md`, `NATIVE-BETA-HANDOFF-INDEPENDENT-EXPECTATIONS.md`, `NATIVE-PROVER-INDEPENDENT-EXPECTATIONS.md`, `NATIVE-KEYGEN-INDEPENDENT-EXPECTATIONS.md`, `ADAPTER-INDEPENDENT-EXPECTATIONS.md`. Positive prepare predicates that this result establishes: typed local genesis, native QueryContext replay, partitioned transcripts, unproven transaction, full preimage equality with independently frozen bytes, `IrSource::check` skips, eight CallContext fields, fee estimate/allowance/availability relation, keyless operation None checks, zero proof invocations. Unexecuted: mutation refusal matrix, finalized binding, PK/VK/SRS, `Zkir::prove`, `Transaction::prove`, signatures/seal, default well_formed, apply, applied 8990 escrow, Preview.

## Independent hash recap

Result freeze file hashes, compiled binary, 39/260 maps, 240+240 runtime trees, and all listed v5 artifacts match. Preimage 4316 / `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402`. Binary `034ed49b124c5b9118ee518c789ce276092567d4f991892ab396fdd6e6158688`. Wrapper `371df382f0758a86428d4a899c6b5f9ef7f695b78fe398c16e3d5903fb9af7be`. Current source map `bf9db118385ecc328e1e20f9e10e9da08b845dc416477c41e8fe0eff62f5545f`. Compiled v4 map `a8a8825ca2b1cba66db6b6a83ce8ccdbcac6ff191bd184cc045388e542f8d8cb`. Runtime v5 `68afbf297ef0ca6cb7954d1e4ddab3065b7c89e4d8e5797e927cdf1d2cf4d425`.
