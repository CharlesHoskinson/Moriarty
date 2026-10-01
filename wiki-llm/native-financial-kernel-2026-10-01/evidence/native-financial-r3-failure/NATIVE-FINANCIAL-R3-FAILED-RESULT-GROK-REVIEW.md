# Frozen R3 failed actual-result independent review

Requested model: `grok-4.6` high.
Returned identity: Grok 4.6.
Date: 2026-10-01.
Reviewer checkout: `/home/charl/Moriarty/.worktrees/moriarty-beta-20260930`.
Evidence root B: `/home/charl/research/moriarty-signed-intent-2026-10-01`.

This document is the Grok failed-result report body. Root must hash this file and attach the actual provider receipt. Prior source/resource votes already frozen in this packet remain historical authorization evidence. They do not approve this result as financial Success, independent verification, or product acceptance.

No current failed-result counterpart was read.

## Verdicts

**Source faithfulness: APPROVE.** The child that produced this failure executed the frozen sorted R3 caller `SOURCE-HASHES` `0179484a011c774bd5f6374200a28902d0873665ff098901574a679f9db189d7`, wrapper `eb0f65aa5623f9c2e5cfa24cda5e57ba00006f13876442eb039bb4e5cdf6e3bf`, proof input freeze `3f396e121e5ce1ff81ed210dd884c467f3e9a590aa9bd2651daa5f1328bb0149` (786/786), and live ELF `7430db3b105a0da85bc64f92885f3cc3e9968371df79f49d28094939d9a96fb6` / 696902872 bytes. All five current Rust modules match. Official ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` and ZKIR `e82d81d25aabcc5f5092e2bd559083487f577914` pins match the input freeze. `main::accept` writes `application-result.txt` and `poststate.tagged` after `well_formed(WellFormedStrictness::default())` and `LedgerState::apply`. That control flow, together with the retained Failure bytes, is the source meaning of this run.

**Actual-result faithfulness: APPROVE as a consumed native apply Failure.** Freeze `NATIVE-FINANCIAL-R3-FAILED-RESULT-FREEZE.json` SHA-256 `6ed8f63baad2d9c0933e41bf081894b158cbadc95392ebe4f82fbd2d047f91e5` is 45/45 plus the closed ELF object. Child exit 1, `stop_reason` null, `supervisor_error` null, `source_identity_preserved` true, `binary_identity_preserved` true, `native_postconditions_passed` false. Wall `203.82358196700807` s, peak sampled group CPU `269.11` s, peak sampled group RSS `2910072832` bytes, all inside the admitted 600/1200/4 GiB envelope. Exact application result is `Failure(InvariantViolation(NightBalance(24001000000000000)))`. `poststate.tagged` is byte-identical to `genesis.tagged` (`63f008f253b133570827334446c3a173f309e68fc699e1ff63954d48498a146f`, 5227 bytes). Seventeen artifacts, no more. Success receipt, replay-refusal, independent verifier, and native Success are absent.

**Refuse** native Success, independent `verify::run` acceptance, Preview settlement, generic compiler/property/intent/transition/history correspondence, authenticated genesis/funding, PCD, retry, fifth Compact, old-proof rebinding, chain/wallet action, diagnostic execution, and any new resource allocation. Source-review approval of this caller is historical. It does not convert this Failure into financial acceptance.

## Startup and method

Loaded `plugins/moriarty-dev/skills/develop/SKILL.md` and `references/execution-focus.md`. Applied `AGENTS.md`. Ran:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Status: capability `SP01.6 loan-swap-subset`. `blockedAction` implementation/repair of loan-swap-subset. `pendingTransactions` empty. Missing evidence: stale SP01 binding/candidate inputs, missing `.moriarty-dev/runtime/current-accounting.json`, unavailable `sp01-loan-swap-grok-01` live resource, unresolved operational history. This review does not dispatch that campaign.

Method: read-only hashing and full-file inspection of the failed-result freeze, prove receipt/attempt/config/log/authorization/live admission/root decision, all 17 proof artifacts, five current Rust modules, wrapper, resource handoff, source freeze, complete 786-entry input freeze, pinned production ledger/ZKIR/transient-crypto APIs, Beta/Core handoff expectations, source-only night-supply diagnosis and unexecuted diagnostic, and historical source/result reports listed below. No wrapper import or exec, no Cargo, no native prove/verify/apply relaunch, no diagnostic compile or run, no ELF copy, no network, no wallet, no git edits, and no source edits.

Historical reports read as authorization or prerequisite evidence:

- `NATIVE-FINANCIAL-R3-PROOF-GROK-REVIEW.md` SHA-256 `0365cdc74e5c86679149fe82328b7b2c9610548940a96f6b51abd8c135d45bb6`
- `NATIVE-FINANCIAL-R3-PROOF-ASTRA-REVIEW.md` SHA-256 `76472b692cf740eb91a31cceee3ba29719bcdf212d6921cef0624e2204c8dd4e`
- `NATIVE-SORTED-V9-R3-EXECUTION-ACTUAL-RESULT-GROK-REVIEW.md` SHA-256 `5b4bbb281af2b29f8e72871d1dd9eea3bac412be6b00e2e416df3f1160fc8715`
- `NATIVE-SORTED-V9-R3-EXECUTION-ACTUAL-RESULT-ASTRA-REVIEW.md` SHA-256 `a0860d844fc80f1c1cc00d7b9287a1bfa55a0944344b4db16b7040e5390fa54d`

Those reports approved source/resources and the earlier retain/build/preflight slice. They allocated one prove. They did not pre-approve this Failure as Success.

## Freeze identity

Independently rehashed `NATIVE-FINANCIAL-R3-FAILED-RESULT-FREEZE.json`:

| Field | Value |
| --- | --- |
| SHA-256 | `6ed8f63baad2d9c0933e41bf081894b158cbadc95392ebe4f82fbd2d047f91e5` |
| Bytes | 5898 |
| Prompt pin match | true |
| `sha256` entries | 45 |
| Matches | 45 |
| Missing | 0 |
| Mismatch | 0 |
| Scope | `consumed failed real proof/defaultWF/native apply attempt; full ledger Failure NightBalance; no Success/replay/independentverification` |

Closed `produced_binary` object:

| Field | Freeze | Live |
| --- | --- | --- |
| path | `/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer` | exists, ELF magic `\x7fELF`, mode `0755` |
| sha256 | `7430db3b105a0da85bc64f92885f3cc3e9968371df79f49d28094939d9a96fb6` | match |
| bytes | 696902872 | match |

Input freeze `3f396e12…` 786/786 match, including production APIs:

| API | SHA-256 |
| --- | --- |
| `ledger/src/structure.rs` | `eebb60349d7c4b1e9954d4e07a1ed389b81612d55d946e2aa43e0525b8bbde9b` |
| `ledger/src/semantics.rs` | `023f16f66436830246f89c58cf1e051f61b7955842aadd8d968656a727f3effd` |
| `ledger/src/verify.rs` | `35b93fc008ae052e9b7948d1f87959b82e459321a2d40fbc09f099d8dcad25db` |
| `ledger/src/prove.rs` | `c9d024eebf712266b3c008a064570c91cdae46b8aa5a1c5cdb94491b0c428477` |
| `ledger/src/construct.rs` | `ffec97687251135ab1ce00bd355e9e3a97d390ab272f7d84de28eec1c770c418` |
| `zkir/src/ir.rs` | `61118e731ae492f61bbcae73633c7ba4ab627cbe7cd36b5ac455fc721028052c` |
| `transient-crypto/src/proofs.rs` | `8cefef1922d64e6ca0bdf24ad52ec15f5f0f52740cc5fdbee9ebbad1dbcd1266` |
| `onchain-state/src/state.rs` | `d187c4ce1d3a953ba31bcdcd2d014b3378c6e899e3bda0e4faa82487b4283c84` |

Wrapper `run-native-financial-r3-proof-bounded.py` SHA-256 `eb0f65aa5623f9c2e5cfa24cda5e57ba00006f13876442eb039bb4e5cdf6e3bf` / 24655 bytes. Candidate five modules:

| File | SHA-256 | Bytes |
| --- | --- | --- |
| `src/main.rs` | `2a25e005931c4efb940aae25260ebf23f0d70846a7cf9a43172d9a4641a00d80` | 30741 |
| `src/provider.rs` | `1bf45f54594e1741214635251dd98ed3cac13651c57d1df30c491f808aaf2c6f` | 4324 |
| `src/verify.rs` | `8e9c59d0d80efbeb28a5d2f84d899be1dc5d732833178d4025656e69c8fe3c8b` | 6433 |
| `src/artifacts.rs` | `4e7be84ede3fb11dc90a58fd166fb749bc53ec07247f86266cacb6bf6ee622d7` | 1771 |
| `src/keygen.rs` | `9f97af1657ad1e23b099e184ea8fb57d91a96976149d7163541f8218ee915f5b` | 4526 |

`Cargo.lock` contains no `mock-verify` string. Ledger feature remains `proof-verifying`.

## Authority that admitted this prove

Root `NATIVE-FINANCIAL-R3-ROOT-RESOURCE-DECISION.json` SHA-256 `dd40ccaa55bfdecdb74cfdd60d7a5a964ee5030025dc331bec2d5078ff1a4d2c`: one offline prove 600 wall / 1200 CPU / 4 GiB after two agreeing source/resource votes and two agreeing actual-preflight votes. Distinct verify remained unallocated until successful proof plus artifact freeze plus fresh admission. Native full Success, exact storage/UTXOs, actual fees, and replay were required. Any failure consumes the attempt.

Prove authorization SHA-256 `c7f5d8d7968b4e18a94816ec6df3b4cf63afd928bdf982ca500aa66deebd8bd1`: `phase` `prove`, `proof_result_freeze` null, wrapper/source/input/V5/V7 pins match. Live admission SHA-256 `a8758ea94a31fde02ba53f9cf584718c4c48dcf72cfeca42e7a40b592169a61e`: UTC `2026-10-01T14:19:46.438621+00:00`, `python_optimize` 0, mem available 30942703616, free 19974848512, no competing native proving/Cargo process. Attempt reservation SHA-256 `861e50064522de13d1e1a9db99acb7667d5e787c88ff1d591c0b30fa1632247b`, unix_time `1790864405.2370272`, state `reserved before child launch; never delete to retry`.

This prove allocation is consumed. The durable reservation remains. Silent retry by deleting it is forbidden.

## Actual execution

Command in attempt and receipt:

```
unshare -Urn -- env RAYON_NUM_THREADS=2
/home/charl/research/moriarty-crypto-2026-09-30/target/debug/beta-native-ledger-consumer
prove
/home/charl/research/moriarty-signed-intent-2026-10-01/native-financial-r3-prove-config.json
96684666c6d855c4992caef4fa65de553a385b5ab1842bd6ef4736173f03ea4b
/home/charl/research/moriarty-signed-intent-2026-10-01/native-financial-r3-proof
```

Receipt SHA-256 `1f49d47f03ff34d3f8cb2249936b04fc76072bf759e469d204aea8277a968152`:

| Field | Value |
| --- | --- |
| `exit_code` | 1 |
| `stop_reason` | null |
| `supervisor_error` | null |
| `child_launched` | true |
| `source_identity_preserved` | true |
| `binary_identity_preserved` | true |
| `produced_binary_sha256` | `7430db3b…` |
| `native_postconditions_passed` | false |
| `elapsed_seconds` | 203.82358196700807 |
| `peak_sampled_group_cpu_seconds` | 269.11 |
| `peak_sampled_group_rss_bytes` | 2910072832 |
| limits | 600 wall / 1200 CPU / 4294967296 RSS-AS / 10 GiB target and free floor / Rayon 2 |

The review prompt names child PID 64697. Frozen attempt, receipt, log, and live admission do not store a PID. The recorded terminal, wall, CPU, RSS, stop, and identity fields above are the freeze-owned execution record.

Log SHA-256 `950188d15fe06bb439e58acaad695dade0b0994146e04a7744cd5c6e9cf9c179` is twelve flushed `NATIVE_PUBLIC_STAGE` lines in order (`ir_parse`, `k_model`, `replay`, `add_calls`, `envelope`, `ir_check` begin/end), then:

```
Error: "not full native Success; inspect retained guaranteed/fallible fee effects"
```

That string is `main.rs` `accept()` after a non-`Success` `TransactionResult`. Target and cache byte counts are unchanged (8653635786 / 2933611498). Free dropped from 19974639616 to 19973181440. Combined proof output is the 17-file tree, far under 512 MiB.

Config SHA-256 `96684666c6d855c4992caef4fa65de553a385b5ab1842bd6ef4736173f03ea4b` equals the wrapper-written ProveConfig and the historical preflight config pin. Fixture remains `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY`, network `undeployed`, `night_value` 1000000000000, `fee_allowance` 100000000000000000000, block 1000000, TTL 1000300.

## Seventeen artifacts

Receipt `output_artifacts` equals the on-disk file set. Count 17. Extra files 0.

| File | Bytes | SHA-256 |
| --- | --- | --- |
| `application-result.txt` | 105 | `5673c3fbc83fca0450827bc90b789ba8ca734235e3173a14934f1b566fae62fb` |
| `constructed.preimage` | 4316 | `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402` |
| `construction-diagnostics.json` | 1935 | `7adad761e43c1bb311dc89ea1161005cf8dc5ba7dd4eeaf2c7b43b4e879c1cf1` |
| `envelope-preflight.json` | 611 | `c870e6ac171cf3b92d2f32add126722cb06b2e031a8d082a9d28a7c2aca7c14c` |
| `frozen.preimage` | 4316 | `96f932aab236782897987da873b3cd6fab3cb8c8c35eb4e4777ca41864f41402` |
| `genesis.tagged` | 5227 | `63f008f253b133570827334446c3a173f309e68fc699e1ff63954d48498a146f` |
| `native-check-result.txt` | 4120 | `36c0ff3241f677cf6e6ee9a9140bd1c4c7aa1ad555ea6a6301724cd43149c1b2` |
| `native-check-skips.json` | 2346 | `b03a54a74a4b95862404a07b1d0429ae6660305e316c01dae0a5b16cd92c3d13` |
| `partition-costs-effects.txt` | 11774 | `73a50f06d519977da05919c1ecc14134d59b83a6ec17c2c287dbd9e6463f777b` |
| `poststate.tagged` | 5227 | `63f008f253b133570827334446c3a173f309e68fc699e1ff63954d48498a146f` |
| `preliminary-fees.json` | 662 | `5b6ff0d1555335db5f7ac5ebce4789f82418a20d43cc2e825fe64a447f827ad9` |
| `preparation.json` | 401 | `083a4b42bc1c5409df0a0d5b109247a5e2336bc97c171d54d5641efd2729f8b8` |
| `prepared-unproven.tagged` | 8246 | `52b51c8a4b0e8e1db3620f73467f6a40417f2384a2bff15713e816096e641be0` |
| `proof.raw` | 6336 | `21321b44f4bd5a0acf2230dcf88886d5b4d44c09a76c771bfe76a48d15dcadac` |
| `skips.json` | 2346 | `b03a54a74a4b95862404a07b1d0429ae6660305e316c01dae0a5b16cd92c3d13` |
| `statement.tagged` | 2275 | `50712a1652cb42de2f562734d61a2e952fd3c214790cfee81f5b77561dfe3ac6` |
| `transaction.tagged` | 10499 | `8195468bff10112ac50e20c6bc201cfe0c11468357fedc778a81b42fbbd35493` |

`genesis.tagged` and `poststate.tagged` are byte-identical. `constructed.preimage` equals `frozen.preimage` and retained `good.preimage`. `skips.json` equals `native-check-skips.json`: length 293, every entry null. Construction diagnostics: `preimage_equal` true, all seven preimage fields equal, all eight CallContext fields equal, `runtime_ops` 293, `runtime_reads` 47. Envelope: guaranteed 1 input / 1 output, fallible 0 inputs / 2 outputs, `well_formed_checked` false, `ledger_applied` false, `ledger_accepted` false. Partition effects: guaranteed VM transcript `None`; fallible A1 output 1010 with claimed spends recipient 1000 and fee 10. Preliminary fee estimate `1128519108356650` with margin 2; this remains an unproven estimate.

Absent from the proof directory and from B:

- `receipt.json`
- `replay-refusal.txt`
- `independent-ledger-receipt.json`
- `NATIVE-FINANCIAL-R3-PROOF-RESULT-FREEZE.json`
- `native-financial-r3-verification/`
- `native-financial-r3-verify-attempt.json` / `-receipt.json` / `-config.json` / `.log`

Wrapper native postconditions require exit 0, `receipt.json` with `result` `Success`, proof/statement/transaction/genesis/poststate/application/replay-refusal, positive consumed fee, and distinct pre/post hashes. Those predicates stayed false.

## Source control flow that produced this Failure

`main::prove` reconstructs registered-VK genesis through `construct()`, builds `provider::Provider`, calls official `Transaction::prove`, requires exactly one contract-proof record, checks native PIs against `ContractCall::public_inputs`, signs and seals, checks PI equality again, then `accept()`.

`accept()` order:

1. `tx.well_formed(state, WellFormedStrictness::default(), block.tblock)?`
2. `state.apply(&verified, &TransactionContext { ref_state: state.clone(), block_context: block, whitelist: None })`
3. write `application-result.txt` and `poststate.tagged`
4. refuse unless `TransactionResult::Success`
5. only then write `receipt.json` and print Success

`application-result.txt` is Debug of `TransactionResult::Failure(TransactionInvalid::InvariantViolation(InvariantViolation::NightBalance(24001000000000000)))`. Official `error.rs` maps `InvariantViolation` into `TransactionInvalid`. `well_formed` errors are `MalformedTransaction` and would have aborted before those writes. The Failure file therefore means default-Real `well_formed` returned `Ok` in this child, and `apply` returned this Failure.

Official `WellFormedStrictness::default()` sets `enforce_balancing`, `verify_native_proofs`, `verify_contract_proofs`, `verify_signatures`, `enforce_limits`, and `ProofVerificationMode::Real`. `verify.rs` `well_formed` with `verify_contract_proofs` calls `P::proof_verify` on the registered operation. `verify.rs` contains no `NightBalance` check. Supply conservation is an apply-time invariant.

Official `LedgerState::apply` on guaranteed error returns `(self.clone(), TransactionResult::Failure(e))`. `apply_section` ends with `state.check_night_balance_invariant()?`. Byte-identical poststate/genesis is the official guaranteed-error rollback.

## Production NightBalance arithmetic

`structure.rs:3361` `MAX_SUPPLY = 24_000_000_000 * STARS_PER_NIGHT` with `STARS_PER_NIGHT = 1_000_000` equals `24000000000000000` (24e15). `LedgerState::new` calls `with_genesis_settings(network, INITIAL_PARAMETERS, locked 0, reserve MAX_SUPPLY, treasury 0)` and checks the invariant there.

Caller `genesis()` then inserts A1 escrow 10000 and one NIGHT UTXO of `fixture.night_value` `1000000000000`. It does not re-check the invariant. `NightAnn` for UTXOs counts only `type_ == NIGHT`. `NightAnn` for contracts counts only contract NIGHT balance. A1 10000 is excluded.

Official `semantics.rs:529` `check_night_balance_invariant` sums UTXO NIGHT annotation + `locked_pool` + `reserve_pool` + `block_reward_pool` + treasury NIGHT + unclaimed rewards + bridge receiving + contract NIGHT, and requires exact `MAX_SUPPLY`.

Constructed genesis total is `24000000000000000 + 1000000000000 = 24001000000000000`. That is the native Failure value. A later balanced spend and return of the same 1e12 principal leaves reserve at MAX and UTXO NIGHT at 1e12. Apply cannot repair this provenance.

`with_genesis_settings` tests at `structure.rs:3475` already return `NightBalance` when locked+reserve+treasury differ from MAX. Default `LedgerState::new` is internally consistent. The extra manual UTXO is the excess.

## Exact warranted crypto, signature, and WF claims

These claims are source-plus-artifact implications from this same child. They are not an independent verifier result.

**Producer-path proving.** Official `ContractCall::prove` calls `ProvingProvider::check`, inserts noops from the skip vector, then `prover.prove(preimage, Some(intermediate_call.binding_input(binding_commitment)))`. Local `provider.rs` refuses missing or zero binding, requires `budget.attempts==0` and skip agreement, calls actual `IrSource::prove` (`zkir/src/ir.rs` `midnight_zk_stdlib::prove`), then `vk.verify` against SRS-derived `ParamsVerifier` and `transient_crypto::proofs::PARAMS_VERIFIER`. `proof.raw` is written only after `records.len()==1` and PI equality. Those producer `vk.verify` calls therefore returned `Ok` in this process. Skip vectors from check and prove are identical (293 nulls). `IrSource::k` remains `midnight_zk_stdlib::optimal_k`. Resolver admits only location `local-public-fixture/pay/c20c6e73…`. Dust spends in the fixture are empty, so the one-call budget was available for the contract proof.

**Same-process default Real well-formedness.** Artifact order plus `accept()` implies `WellFormedStrictness::default()` returned `Ok` on the signed sealed transaction, including Real contract-proof verification and signature verification. Envelope preflight still left the deferred signature closure uncalled and recorded `well_formed_checked` false; that file is construction-stage. Default transaction WF is the later `accept()` call.

**Fixture signatures.** After prove, the caller signs segment 1 with `SigningKey::Schnorr` from public bytes `[0x42;32]`, then `seal`. `transaction.tagged` exists and PI equality was checked again. That is the explicit public-development fixture key. It is a separate role from production Beta ECDSA `verifyAndPrepare` / source-owner authorization.

**Independent verifier remains unexecuted.** `verify::run` reconstructs genesis, dual-verifies, runs mutation/EOF/absent-signature negatives, then pristine `accept` and good-again. Wrapper verify phase requires a root-created successful proof-result freeze, parent `native_postconditions_passed` true, and a distinct `phase: verify` authorization. Those objects are absent. `proof.raw` existence, producer `vk.verify`, and same-process WF are not `INDEPENDENT_CONDITIONAL_NATIVE_LEDGER_GOOD_AGAIN_OK`.

Historical V8 proof `6d5b5d2c464a5a96e3a224e14f6c454b58798d4776c5a3a8bc8d03799ee2b6d2` is 6336 bytes and a different digest from this `21321b44…`. This proof cannot be rebound onto another genesis, statement, or envelope.

## Source-only night-supply diagnosis

`NATIVE-FINANCIAL-R3-NIGHT-SUPPLY-DIAGNOSIS.md` SHA-256 `7181e4dbc8c5a4ac61292b13ddd0e00d2100ee8f4957850dd73f6cadd4eba721` / 6938 bytes. `night-supply-diagnostic.rs` SHA-256 `378ff7273c3e342ac0b26d3a6f4b125939ac9d27a1ecb45c765986be4600861d` / 3083 bytes. Neither path is in the 45-entry failed-result freeze. This review did not compile or run the diagnostic. No resource authority exists for it.

The diagnosis’s official-API chain (MAX in reserve, extra 1e12 UTXO, `semantics.rs:529` sum, apply rollback) matches the production source and the actual Failure value. That agreement is source reading. It is not a newly executed native invariant check on retained genesis.

The diagnostic source is a proposed isolated checker only. It produces no funded replacement genesis, proof, or transaction. This review allocates nothing for building or running it. Caller, keys, ELF, and failed artifacts stay immutable.

## Open gaps that this Failure leaves untouched

| Gap | Status after this run |
| --- | --- |
| Native full Success, escrow 8990, UTXO conservation, consumed DUST fee, replay Failure | Open; apply returned NightBalance |
| Independent `verify::run` / proof-result freeze | Open; blocked by missing Success |
| Authenticated public genesis, mint, deploy, funding, owner mapping | Open; fixture is `TRUSTED_GENESIS_PUBLIC_DEVELOPMENT_ONLY` plus illegal extra NIGHT |
| Source-owner ECDSA vs native Schnorr NIGHT payer | Open; separate roles |
| Generic Moriarty compiler lowering | Open; Compact kernel is a manual pay specialization |
| Property, intent refinement, transition validity, history compliance | Open |
| Native PCD / recursive accumulator / F0–F3 | Open; one k17 contract proof in this child, no independent verify |
| Preview settlement / wallet / chain | Open; network `undeployed`, `unshare -Urn` |
| Published SRS ceremony audit | Open |
| Sorted-but-wrong semantic/VK/funding/dust/time/stale-state matrix | Open |
| I2/MC02/MC05, SP05, mandatory proof-carrying product acceptance | Open |
| Guarded SP01 operational history / accounting | Open; status unchanged |

Production Beta/Core handoff remains the exact supported fixture producer (`pay.zkir` `c20c6e73…`, runtime `329cbc73…`, expected storage `716eb7dd…`). Passing this public fixture would still be conditional trusted-genesis evidence. This run did not pass.

## All blockers together

1. Actual native result is `Failure(InvariantViolation(NightBalance(24001000000000000)))`. `native_postconditions_passed` is false. Child exit 1.
2. Poststate equals genesis. No ledger mutation was retained.
3. Success `receipt.json`, replay-refusal, independent verifier, and proof-result freeze are absent.
4. Official genesis already holds MAX in reserve; the fixture adds 1e12 NIGHT. Apply-time invariant reports exactly that sum. Balanced spend of the same UTXO cannot remove the excess.
5. Same-process producer dual `vk.verify` and default Real WF are source implications from this child. Independent verifier acceptance is unexecuted.
6. The prove reservation is consumed. Deleting it to retry is forbidden.
7. Current caller, keys, ELF, V7 PK/VK/SRS, Compact kernel, and these failed artifacts stay immutable.
8. No fifth Compact compile, old-proof rebinding, chain dispatch, or wallet import is authorized.
9. Night-supply diagnostic is source-only, unexecuted, and without resource authority. This review allocates no build or run.
10. Wrapper verify remains a second closed phase after a successful freeze. This Failure cannot fill that freeze.
11. Sampling 0.1 s / disk 5 s, parent hashing outside the child monitor, Rayon 2 as a non-cgroup cap, and SIGKILL/power-loss cleanup skip remain documented residuals of the consumed run.
12. Preliminary fee estimate is not consumed protocol DUST.
13. `Zkir::check` 293-null skip vector is construction/prove skip agreement. It is not independent `vk.verify`.
14. Expected-contract bytes remain storage-only. Native post-application escrow 8990 was never reached.
15. Broader financial negatives, generic correspondence, authentic genesis/funding, PCD, and Preview settlement remain Open.
16. Guarded develop status still reports unresolved SP01 history, stale binding/candidate inputs, missing current accounting, and unavailable loan-swap live resource.
17. Source-review approval of this caller is not actual financial acceptance.

Items 1–10 block any Success, verify, retry, or successor execution claim. Items 12–17 stay Open even after a later supply-consistent fixture, which would be a new source packet with new proofs.

## Boundaries of these verdicts

Source faithfulness covers identity of the frozen wrapper, five Rust modules, official APIs, producer-projection pins, and the accept/apply control flow that wrote these artifacts. Result faithfulness covers freeze `6ed8f63b…` as an accurate record of one consumed failed prove: real proof bytes, signed transaction bytes, default Real WF returning in-process, and native apply Failure NightBalance with unchanged genesis.

This review grants no Success, no independent verification, no Preview, no product completion, and no future allocation.

## Model receipt to preserve

Requested: `grok-4.6` at high effort.
Returned identity: Grok 4.6.
Host system line: Grok 4.6 released by xAI.
This document is the report body. Root must hash it and attach the actual provider receipt JSON when the session ends.
