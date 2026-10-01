# Sorted-offer V9-R3 execution actual-result independent review

Requested model: `grok-4.6` high.
Returned identity: Grok 4.6.
Date: 2026-10-01.
Reviewer checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.
Evidence root B: `/home/charl/research/moriarty-signed-intent-2026-10-01`.

This document is the Grok actual-result report body. Root must hash this file and record the actual provider receipt. Source-review votes already frozen in this result packet remain historical authorization evidence. They do not approve this result.

No current Astra actual-result counterpart was read.

## Verdicts

**Source faithfulness: APPROVE.** The bytes that ran are the frozen R3 candidate `SOURCE-HASHES` `0179484a011c774bd5f6374200a28902d0873665ff098901574a679f9db189d7`, wrapper `9774a7370a5afd06cc192d1fb1902f90a9657f313b6a6787e09bbf5a0e8fac93`, helper `95291d490740b1d7bca1affdbaacf81bfa0ca57ae4e7600d25da3c4022004e0f`, execution source freeze `697199a9de5da23dd304f7274fa1e035a4820efe766426c4cecb930370ae8963`, and execution input freeze `6a133a8c8c85a40882396910547608b0f7967ab749d433ab38e2b3e2ff97fd3a`. Execution wrote no new Rust. Official called APIs still match the consumer surface.

**Result faithfulness: APPROVE** the narrow actual result named in `NATIVE-SORTED-V9-R3-EXECUTION-RESULT-FREEZE.json` SHA-256 `0251b31fa5bba4f6b1a0cae91f4e98bd6a319333a55765b896d0cb3260848df1`: exclusive R2 ELF retention, offline locked incremental build of this source, and one registered-VK native construction plus eager envelope plus `Zkir::check` preflight. Receipts, logs, eleven public outputs, twelve flushed stage lines, and the produced ELF match the freeze and the source predicates.

**Refuse** cryptographic proof, ownership or registration signatures, default transaction well-formedness, ledger apply, Preview settlement, independent verifier success, and any current next proof authority. This result grants none of those.

Approval flags here are administrative judgments of source identity and of this executed preflight slice. They are not financial acceptance, PCD correspondence, or product completion.

## Startup and method

Loaded `plugins/moriarty-dev/skills/develop/SKILL.md` and ran `python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`.

Status: capability `SP01.6 loan-swap-subset`. `blockedAction` implementation/repair of loan-swap-subset. `pendingTransactions` empty. Missing evidence includes stale SP01 binding/candidate inputs, missing `.moriarty-dev/runtime/current-accounting.json`, unavailable `sp01-loan-swap-grok-01` live resource, and unresolved operational history. This review does not dispatch that campaign.

Method: read-only hashing and parsing of the result freeze, result handoff, full current candidate Rust, wrapper, helper, phase authorizations, root phase decision, source and execution freezes, complete execution input map, receipts, attempts, logs, config, and all eleven preflight outputs. Official `IrSource::k` / `IrSource::check` and `UnshieldedOffer::well_formed` were read from the pinned checkouts. Typed tagged binaries were hashed and sized. No wrapper import or exec, no Cargo, no native preflight/prove/verify relaunch, no ELF copy, no network, no wallet, no git edits, and no source edits.

Historical source reviews frozen in this packet were read as authorization evidence:

- `NATIVE-SORTED-V9-R3-EXECUTION-GROK-REVIEW.md` SHA-256 `b2d09c1fb3207a140a2bbdd2d4eec7091a3e128d463ba4c446bbdec619087daf`
- `NATIVE-SORTED-V9-R3-EXECUTION-ASTRA-REVIEW.md` SHA-256 `f11ac7c7075eb2a66d79b276487e43d53617fda12ffa1c564743a48b7b5b9f02`

The Grok source-review provider receipt `native-sorted-v9-r3-execution-grok-receipt.json` records `requested_model` `grok-4.6`, `requested_effort` `high`, `modelUsage.grok-4.6-build`, `process_exit_code` 0. That receipt belongs to the source packet. It does not approve this result.

## Freeze and handoff identity

`NATIVE-SORTED-V9-R3-EXECUTION-RESULT-FREEZE.json` SHA-256 `0251b31fa5bba4f6b1a0cae91f4e98bd6a319333a55765b896d0cb3260848df1` (6438 bytes). Scope string: `actual R3 retention/offline build/registered-VK native construction+eager-envelope+IRcheck only; no proof/signatures/defaultWF/apply/Preview`.

`NATIVE-SORTED-V9-R3-EXECUTION-RESULT-HANDOFF.md` SHA-256 `6bbc70ba8e08ade6deca72c71d5bfbf9f4fbb3a35abf1730ec84e988d1db16ab` (6431 bytes).

The freeze `sha256` map has 47 entries. Every path exists. Every digest matches. Missing 0. Mismatch 0.

`produced_binary` is a closed object outside that map:

| Field | Recorded | Live observation |
| --- | --- | --- |
| path | `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer` | exists, ELF magic `\x7fELF`, mode `0755` |
| sha256 | `7430db3b105a0da85bc64f92885f3cc3e9968371df79f49d28094939d9a96fb6` | match |
| bytes | 696902872 | match |

The wrapper `run-native-sorted-v9-r3-execution-bounded.py` is absent from the 47-entry result map. It is pinned by `NATIVE-SORTED-V9-R3-EXECUTION-SOURCE-FREEZE.json`, which is in the result map, at SHA-256 `9774a7370a5afd06cc192d1fb1902f90a9657f313b6a6787e09bbf5a0e8fac93` / 24137 bytes. The helper is both in that source freeze and listed directly in the result map. Handoff coverage of wrapper/helper holds through that split.

## Source-execution closure

Execution source freeze SHA-256 `697199a9de5da23dd304f7274fa1e035a4820efe766426c4cecb930370ae8963`. All 6 owned files match.

Execution input freeze SHA-256 `6a133a8c8c85a40882396910547608b0f7967ab749d433ab38e2b3e2ff97fd3a`. Top-level keys remain `{sha256, binary, prove_config, v7_result_freeze}`. The `sha256` map has 751 paths. Every path exists and matches. Versus original R3 input freeze `c2d354026679a621f08fd6ad54d213d5802ef53cfbc6c51413de8f82103d178f` (748 paths): three added paths, zero removed, zero hash changes on the common set. Added paths are the original R3 source freeze, source handoff, and original R3 input freeze.

The input-freeze `binary` field still declares the pre-build R2 live ELF `6619da6da9e291d47a78b405cd0cdeb07572f3c91c547e7f7f1529c34a0b34a4` / 696885656. That immutable snapshot is correct. The result freeze records the later produced ELF separately.

Candidate `SOURCE-HASHES.json` SHA-256 `0179484a011c774bd5f6374200a28902d0873665ff098901574a679f9db189d7`. All 10 files match. `INPUT-SOURCE-HASHES.json` SHA-256 `9d34d65754d8282515fccd6941859bed273db729995baa9254a3055fb42a658b`. All 12 official/current pins match, including:

| Pin | SHA-256 |
| --- | --- |
| `zkir/src/ir.rs` | `61118e731ae492f61bbcae73633c7ba4ab627cbe7cd36b5ac455fc721028052c` |
| `onchain-state/src/state.rs` | `d187c4ce1d3a953ba31bcdcd2d014b3378c6e899e3bda0e4faa82487b4283c84` |
| `pay.zkir` | `c20c6e733500823c478eea885bebab5d62ba9958cfdd1ec8cf2ae89b32f9d7ae` |

`ledger/src/verify.rs` remains `35b93fc008ae052e9b7948d1f87959b82e459321a2d40fbc09f099d8dcad25db` in the execution input map.

`Cargo.lock` still has 391 `[[package]]` nodes and 1220 dependency-list edges. The string `mock-verify` is absent. The string `test-utilities` is absent. `Cargo.toml` still enables `proof-verifying` on ledger rev `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` and pins ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914`.

Preflight config SHA-256 `96684666c6d855c4992caef4fa65de553a385b5ab1842bd6ef4736173f03ea4b` equals the wrapper-written ProveConfig and the historical V8 prove-config pin. Every config path exists and matches:

| Section | Bytes | Digest match |
| --- | --- | --- |
| `pay.zkir` | 76046 | true |
| `initial-contract.tagged` | 1432 | true |
| `expected-contract.tagged` | 1483 | true |
| `runtime-native.json` | 38679 | true |
| `good.preimage` | 4316 | true |
| `pk.tagged` | 234911946 | true |
| `vk.tagged` | 2745 | true |
| `bls_midnight_2p17` | 25166212 | true |

Fixture remains `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, network `undeployed`, block 1000000, TTL 1000300, night_value 1000000000000, fee_allowance 100000000000000000000.

The wrapper hashes PK/SRS bodies as identity pins before launch. Consumer `preflight()` deserializes `ProveConfig`, decodes VK only, then `construct()`. It does not load PK or SRS bodies.

## Source faithfulness detail

R3 `src/main.rs` SHA-256 `2a25e005931c4efb940aae25260ebf23f0d70846a7cf9a43172d9a4641a00d80` / 30741 bytes. `SORTED-OFFER.patch` is the R2-to-R3 `main.rs` delta: `public_stage` writes `NATIVE_PUBLIC_STAGE {name}` to locked stderr and flushes, then twelve calls around existing IR parse, `ir.k()!=17`, replay, `add_calls`, envelope, and `ir.check`. Financial constructors, `offer()` sort, envelope negatives, preimage equality, fee estimate, `accept()`, and `ir.k()!=17` remain the R2 bodies.

Official `IrSource` still implements:

```
fn check(&self, preimage: &ProofPreimage) -> Result<Vec<Option<usize>>, ProvingError> {
    Ok(self.preprocess(preimage)?.pi_skips)
}
fn k(&self) -> u8 { midnight_zk_stdlib::optimal_k(self) as u8 }
```

`k_model` markers wrap that `optimal_k` search. They do not replace it with a host flag. `ir_check_end` is emitted after `ir.check(p)` returns and before `checked?` propagates an error.

Official `UnshieldedOffer::well_formed` still returns a deferred signature-verification closure after eager `is_sorted`, nonzero-output, and duplicate-input checks, and still uses `MalformedTransaction::OutputsNotSorted` for unsorted outputs. Consumer `envelope_preflight` still calls that API and still leaves the closure uncalled.

`prove()`, `verify::run`, and `keygen::run` remain in the binary CLI. Wrapper AST and runtime accept only `{build|preflight}`. There is no `proof_result_freeze` field.

Candidate `README.md` still contains leftover source-packet sentences: a V9-R3 wrapper proposing prove/verify, and “No build/preflight/proof/verify has been executed for this source.” Those sentences are frozen source prose inside this result map. Result receipts and the result handoff describe the executed retain/build/preflight. The README sentences do not expand command scope.

Wrapper constant name `V8_INPUT_PIN` still holds execution input freeze `6a133a8c…`. The bound value is correct.

## Phase authority and actual operations

Root `NATIVE-SORTED-V9-R3-EXECUTION-ROOT-RESOURCE-DECISION.md` SHA-256 `4ceea7af386f07ccb3d02e212da7251c32f1ecb24a85ef54199dcffec2caad3b` records sequential retain then build then preflight authority after both source reports. Attempt commands equal receipt commands. Prior-phase receipt pins in the authorizations match the receipts on disk.

### Retain

Authorization SHA-256 `ed85d84fc2e6ecd6e13a820fa98a4894c2c12edca98846a59659bf491121ac96`, phase `retain`.

Receipt SHA-256 `0ad521bbbead40460439a0892cc4da67d845c6cbe06101561c3e6c7b3238202b`:

- `completed` true, `fsync_done` true, `error` null
- `bytes_copied` 696885656
- `sha256` `6619da6da9e291d47a78b405cd0cdeb07572f3c91c547e7f7f1529c34a0b34a4`
- `source_identity_preserved` true
- elapsed 4.672528647992294 s
- CPU 3.9601206180000004 s
- limits 60 wall / 120 CPU / 2 GiB AS / 1 GiB file and directory / 10 GiB free floor
- qualification: retained reviewed ELF only; no process/network/Cargo/ELF execution

Archive `native-sorted-v9-r3-execution-retained-r2-binary/beta-native-ledger-consumer` is mode `0444`, size 696885656, digest `6619…`. Retention copied the prior R2 executable. It did not execute it.

### Build

Authorization SHA-256 `d3732ccf6eff83067108f19247f26d18eb98837ddb83414c4024ec2a65220a68`, phase `build`. Prior receipt is the retention receipt above.

Receipt SHA-256 `f5e0441a63a9628612149b440f94a53096e7f1b29398145916d1b5722781bc60`:

- command: `unshare -Urn -- env CARGO_BUILD_JOBS=2 RAYON_NUM_THREADS=2 CARGO_NET_OFFLINE=true CARGO_TARGET_DIR=… cargo build --locked --offline --manifest-path Cargo.toml --bin beta-native-ledger-consumer`
- `exit_code` 0, `stop_reason` null, `supervisor_error` null, `child_launched` true
- `source_identity_preserved` true, `binary_identity_preserved` true, `native_postconditions_passed` true
- `produced_binary_sha256` `7430db3b105a0da85bc64f92885f3cc3e9968371df79f49d28094939d9a96fb6`
- elapsed 43.37700816500001 s
- peak sampled group CPU 57.75 s
- peak sampled group RSS 3228184576 bytes
- limits 120 wall / 240 CPU / 4 GiB RSS and AS
- output artifacts `{}`

Log SHA-256 `dde9befe616b44a983137669f114f300eea14860cf81e43fb46655c3a5ecc7c0` reports `Finished dev profile [unoptimized + debuginfo] target(s) in 40.11s`. Successful compilation evidence. Not financial acceptance.

New ELF is 17216 bytes larger than the retained R2 ELF. That matches a marker-only debug rebuild on the same target.

### Preflight

Authorization SHA-256 `129aebd75924d1105e1d3ded24468c53f5b44e4c186fcbc521a2584dde54595e`, phase `preflight`. Prior receipts are retention plus build.

Receipt SHA-256 `c7ac02f6a12e118b85bd78646d9229cdede4b531ae307695f354c48da5ada6da`:

- command: `unshare -Urn -- env RAYON_NUM_THREADS=2 NEW_ELF preflight CONFIG CONFIG_SHA OUTPUT`
- `exit_code` 0, `stop_reason` null, `supervisor_error` null, `child_launched` true
- `source_identity_preserved` true, `binary_identity_preserved` true, `native_postconditions_passed` true
- `produced_binary_sha256` `7430db3b…`
- elapsed 79.04924262498389 s
- peak sampled group CPU 73.16 s
- peak sampled group RSS 206262272 bytes
- limits 120 wall / 240 CPU / 2 GiB RSS and AS / 32 MiB output
- eleven `output_artifacts`, each digest and size matching the freeze and the files

Each of retain, build, and preflight terminated with wrapper/helper exit 0.

Log SHA-256 `1a970cd6ffd4a0ea781b4aa6f1db13ec7a8e12b13a318ca08871d3f9b32b5788` contains exactly these twelve flushed markers in order, then the terminal line:

```
NATIVE_PUBLIC_STAGE ir_parse_begin
NATIVE_PUBLIC_STAGE ir_parse_end
NATIVE_PUBLIC_STAGE k_model_begin
NATIVE_PUBLIC_STAGE k_model_end
NATIVE_PUBLIC_STAGE replay_begin
NATIVE_PUBLIC_STAGE replay_end
NATIVE_PUBLIC_STAGE add_calls_begin
NATIVE_PUBLIC_STAGE add_calls_end
NATIVE_PUBLIC_STAGE envelope_begin
NATIVE_PUBLIC_STAGE envelope_end
NATIVE_PUBLIC_STAGE ir_check_begin
NATIVE_PUBLIC_STAGE ir_check_end
REGISTERED_VK_NATIVE_ENVELOPE_PREFLIGHT_ONLY; no proof/signature/defaultWF/apply
```

Markers have no timestamps. They establish completed boundary sequence in this successful run. They do not measure per-stage cost. They do not attribute the prior R2 timeout.

Wrapper postcondition requires that exact twelve-line `NATIVE_PUBLIC_STAGE` sequence plus `preparation.json` / `envelope-preflight.json` flags. `native_postconditions_passed` true is consistent with those bytes.

## Public outputs and exact predicates

Eleven files under `native-sorted-v9-r3-execution-preflight/`. No `proof.raw`, `transaction.tagged`, `statement.tagged`, `application-result.txt`, `poststate.tagged`, or `replay-refusal.txt`.

| Artifact | Bytes | SHA-256 |
| --- | --- | --- |
| `constructed.preimage` | 4316 | `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402` |
| `frozen.preimage` | 4316 | `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402` |
| `genesis.tagged` | 5227 | `63f008f253b133570827334446c3a173f309e68fc699e1ff63954d48498a146f` |
| `prepared-unproven.tagged` | 8246 | `52b51c8a4b0e8e1db3620f73467f6a40417f2384a2bff15713e816096e641be0` |
| `construction-diagnostics.json` | 1935 | `7adad761e43c1bb311dc89ea1161005cf8dc5ba7dd4eeaf2c7b43b4e879c1cf1` |
| `envelope-preflight.json` | 611 | `c870e6ac171cf3b92d2f32add126722cb06b2e031a8d082a9d28a7c2aca7c14c` |
| `preparation.json` | 401 | `083a4b42bc1c5409df0a0d5b109247a5e2336bc97c171d54d5641efd2729f8b8` |
| `preliminary-fees.json` | 662 | `5b6ff0d1555335db5f7ac5ebce4789f82418a20d43cc2e825fe64a447f827ad9` |
| `partition-costs-effects.txt` | 11774 | `73a50f06d519977da05919c1ecc14134d59b83a6ec17c2c287dbd9e6463f777b` |
| `native-check-result.txt` | 4120 | `36c0ff3241f677cf6e6ee9a9140bd1c4c7aa1ad555ea6a6301724cd43149c1b2` |
| `native-check-skips.json` | 2346 | `b03a54a74a4b95862404a07b1d0429ae6660305e316c01dae0a5b16cd92c3d13` |

`constructed.preimage` equals `frozen.preimage` byte-for-byte and equals retained `native-adapter-successor-v3/good.preimage`.

`preparation.json`: `native_check` is `successful actual Zkir::check`; `preimage_equal` true; `registered_vk_present` true; `proof_invocations` 0; `preliminary_fee_with_margin2` `1128519108356650`; `generationless_available` `5000000000000000000000`; scope registered-VK preflight only.

`construction-diagnostics.json`: `runtime_ops` 293, `runtime_reads` 47; `preimage_equal` true; all seven preimage field equalities true (`inputs`, `private_transcript`, `public_transcript_inputs`, `public_transcript_outputs`, `binding_input`, `communications_commitment`, `key_location`); `actual_context_matches` true; all eight `call_context_fields_equal` true (`own_address`, `caller`, `balance`, `tblock`, `tblock_err`, `parent_block_hash`, `com_indices`, `last_block_time`); `block_seconds` 1000000; `ttl_seconds` 1000300; `default_global_ttl_seconds` `3600`; `expected_contract_scope` storage-only generated poststate reference. Exported escrow is not applied ledger balance.

`envelope-preflight.json`: guaranteed 1 input / 1 output; fallible 0 inputs / 2 outputs; `proof_invocations` 0; `well_formed_checked` false; `ledger_applied` false; `ledger_accepted` false. Frozen source requires exact native `OutputsNotSorted` / `ZeroValueUtxo` / `DuplicateInputs` branches on eligible isolated copies, then original-good eager validation again. With these counts, guaranteed exercises duplicate funding and zero output; fallible exercises unsorted financial outputs and zero output. DuplicateInputs is skipped on the empty fallible input vector. OutputsNotSorted is skipped on the single guaranteed output. Deferred signature-verification closures remain uncalled. There is no independently complete negative suite per partition.

`partition-costs-effects.txt` starts `guaranteed=None` and retains actual fallible transcript gas/effects (`unshielded_outputs` A1 1010, claimed spends 1000 and 10). Digest `73a50f06…` equals the same file under `native-ledger-v5-keyless-preparation/` and `native-financial-v8-03-proof/`. Runtime/Core ordered effects stayed unchanged. Unsigned native-envelope order was the V9 correction.

`native-check-result.txt` is Debug `Ok([None, …])` with 293 `None` entries. `native-check-skips.json` is the same 293-length JSON array, all null. These are native relation diagnostics from `Zkir::check` → `preprocess(preimage)?.pi_skips`. They are not a PLONK proof and not a verified final public statement.

Tagged `genesis.tagged` (5227) and `prepared-unproven.tagged` (8246) are retained and hashed. This review did not invoke a native decoder on them. No prepared transaction is relabeled signed, sealed, or accepted.

## Fees artifact wording

`preliminary-fees.json` actual flags:

- `keyless` false
- `vk_present` true
- `unproven` true
- `estimate_with_margin2` `1128519108356650`
- `allowance` `100000000000000000000`
- `estimate_within_allowance` true
- method `Transaction::fees_with_margin(default LedgerParameters,2)`

The inherited `limitation` string still says “No actual final proof/signature/noop serialized size or real VK read cost established in keyless preparation…”. That sentence is hardcoded in `construct()` regardless of `vk_present`. The handoff qualifies it as stale wording. Treat the boolean flags as the actual record. Treat the limitation sentence as leftover keyless-preparation prose. This estimate is not consumed DUST, not a serialized-size fee, and not transaction Success.

## Inherited R2 wall failure and V8 OutputsNotSorted

Both remain pinned in the 751-entry execution input map at their historical identities. This successful R3 preflight does not erase them and does not rebind them as acceptance.

R2 failed-result freeze SHA-256 `183b5416025d9b107546bd04c93f6d9ef9bed7ad4c79b3c8c70dc170820d8e05`. R2 preflight receipt SHA-256 `08f2e79a8398b1e37a063387a66c606287e8306e1570356cef9807e28e1b5c1a`:

- `exit_code` -9
- `stop_reason` `wall limit`
- elapsed 62.91793476999737 s
- peak sampled CPU 59.85 s
- peak sampled RSS 188895232 bytes
- `native_postconditions_passed` false
- `output_artifacts` `{}`
- `source_identity_preserved` true, `binary_identity_preserved` true
- produced ELF `6619da6d…`
- log 0 bytes, SHA-256 `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`
- output directory file set empty

V8 failed-result freeze SHA-256 `ef283f6d93f0123f48acc8295621bc6198312f8ec875f7fa43fd4da14be69079`. Prove receipt exit 1, `stop_reason` null, `native_postconditions_passed` false. `proof.raw` 6336 bytes SHA-256 `6d5b5d2c464a5a96e3a224e14f6c454b58798d4776c5a3a8bc8d03799ee2b6d2`. `transaction.tagged` 10499 bytes SHA-256 `7c4de0ca2def7e8b9abae4831f7d026a97e5e975905bd6f9b1d92895e49ef416`. Log line: `Error: OutputsNotSorted([UtxoOutput { value: 1000, owner: UserAddress(0202…02), … }, UtxoOutput { value: 10, owner: UserAddress(0303…03), … }])`. No apply. No independent verifier success.

V7 archive remains distinct: `native-sorted-v9-r2-retained-v7-binary/beta-native-ledger-consumer` SHA-256 `9e1b153a17cd55969dd800fd47bc728923963093267cc8d23765f0a6ea484559` / 696706768 bytes / mode `0444`.

Three ELF identities stay separate: V7 `9e1b153a…` / 696706768, R2 `6619da6d…` / 696885656, R3 produced `7430db3b…` / 696902872.

## All blockers together

These remain open after this faithful preflight. They are listed together. None is closed by the 47-file freeze, the produced ELF, or the eleven outputs.

1. Cryptographic proof. `proof_invocations` 0. No `proof.raw`. Provider `prove` was not dispatched.
2. Ownership and registration signatures. Envelope deferred signature closures uncalled. Dust registration `signature: None`.
3. Default transaction well-formedness. `well_formed_checked` false. Eager offer `well_formed` is a cheaper native envelope check.
4. Ledger apply and `TransactionResult::Success`. `ledger_applied` false, `ledger_accepted` false. No poststate, dust event, or conservation evidence from this run.
5. Preview settlement and public transaction finality. Network namespace remained unshared. `pendingTransactions` empty.
6. Current next proof or verifier authority. Wrapper has no prove/verify branch. Root decision supplies none. Successful preflight grants none.
7. Consumed R2 wall-limit failure. Child -9, empty log and artifacts, identities preserved. R3 success does not identify the R2 interrupting stage.
8. Consumed V8 `OutputsNotSorted` failed proof. 6336-byte raw proof and signed transaction retained. No independent verifier success and no apply.
9. Financial correspondence from Moriarty source to this native fixture. Manual Compact specialization remains the producer of `pay.zkir` / runtime / contracts.
10. Trusted genesis provenance. Fixture trust string is `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY` on network `undeployed`. Authentic mint, deploy, funding, registry, time, and history remain unverified.
11. Owner/custody correspondence. Deterministic development signing key `[0x42;32]`. No wallet import and no authenticated owner.
12. Generic compiler / lowering correspondence. Fixed pay specialization is not a generic Moriarty-to-ZKIR proof.
13. Complete native negative matrix, including sorted-but-wrong financial data, stale state, competing candidates, VK mismatch, funding/dust/time faults.
14. Independent complete negative suite per envelope partition. Current branches are count-gated.
15. Actual consumed protocol fees. Preliminary unproven `fees_with_margin` estimate only. Limitation string still talks about keyless preparation while `keyless` is false and `vk_present` is true.
16. `Zkir::check` skip vector as a substitute for `vk.verify` / default Real proof-verifying well-formedness.
17. Stage markers as cost accounting or as R2 timeout attribution.
18. Expected-contract storage bytes as applied native escrow.
19. Mandatory proof-carrying transactions, native PCD, and full SP05/I2/MC05 financial acceptance.
20. Website publication and SP01 operational-history/accounting stops. Those are separate. This result does not clear them.

## Boundaries of these verdicts

This result is actual sorted native construction, eager offer validation, and registered-VK `Zkir::check` on the exact production Beta/Core public fixture, under the recorded retain/build/preflight limits, with source and binary identities preserved.

The result freeze, handoff timings (retain 4.6725 s, build 43.377 s, preflight 79.0492 s / 73.16 CPU / 206262272 RSS), eleven outputs, and twelve public stages match the receipts and files.

This review allocates no future proof, verify, keygen, Compact, wallet, or network resource. A later proof decision needs a distinct source/resource packet, fresh audits, and new root authority.

Website publication remains an independent root task. Full product goal remains open.

## Model receipt to preserve

Requested: `grok-4.6` at high effort.
Returned identity: Grok 4.6.
Host system line: Grok 4.6 released by xAI.
This document is the report body. Root must hash it and attach the actual provider receipt JSON when the session ends.

Historical source-review receipt, not this verdict: `native-sorted-v9-r3-execution-grok-receipt.json` SHA-256 `14a42c33e7e313d00895d6ca119191154ae0b8ea2d9df9eedc0905edd66796c8`, `requested_model` `grok-4.6`, `requested_effort` `high`, `modelUsage.grok-4.6-build`, `process_exit_code` 0, source-review file SHA-256 `b2d09c1fb3207a140a2bbdd2d4eec7091a3e128d463ba4c446bbdec619087daf`.
