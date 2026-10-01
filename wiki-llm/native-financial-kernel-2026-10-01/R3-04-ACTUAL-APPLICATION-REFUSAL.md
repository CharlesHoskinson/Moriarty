# Sorted native financial proof: application refused excess NIGHT

2026-10-01. **One consumed native attempt, followed by two independent full actual-result reviews. No financial Success or Preview settlement.**

## Actual result

Experiment observation: the sorted R3 caller produced a finalized 6,336-byte native proof, a 2,275-byte statement and a 10,499-byte signed sealed transaction. Its producing process performed real proof verification against both supplied and embedded verifier parameters. The source ordering and retained application outputs establish that default Real native well-formedness returned successfully before application. These are checks in the producing process; the separate independent verifier was not run.

Native application returned:

```text
Failure(InvariantViolation(NightBalance(24001000000000000)))
```

The resulting state is byte-identical to the 5,227-byte genesis, SHA256 `63f008f253b133570827334446c3a173f309e68fc699e1ff63954d48498a146f`. This is rollback of this guaranteed-section failure. It does not establish rollback for every possible fallible-section failure. No success receipt, replay-refusal artifact, accepted payout or consumed-fee result was produced. The child exited 1 without a resource stop.

The retained measurement is 203.823581967 seconds wall, 269.11 seconds sampled group CPU and 2,910,072,832 bytes sampled peak RSS, inside the reviewed 600-second/1,200-CPU/4-GiB envelope. Sampling and per-process address-space limits retain their stated limitations.

## Immutable evidence and reviews

[Portable archive](evidence/native-financial-r3-failure/archive-manifest.json) preserves the exact 45-file failed-result closure, the original result freeze, source wrapper and both full actual-result reports. Binary/key/SRS bodies remain externally retained and hash-bound. Historical absolute paths identify the original experiment; they are not portable install instructions.

| Object | SHA256 |
|---|---|
| Failed-result freeze | `6ed8f63baad2d9c0933e41bf081894b158cbadc95392ebe4f82fbd2d047f91e5` |
| Proof | `21321b44f4bd5a0acf2230dcf88886d5b4d44c09a76c771bfe76a48d15dcadac` |
| Statement | `50712a1652cb42de2f562734d61a2e952fd3c214790cfee81f5b77561dfe3ac6` |
| Signed transaction | `8195468bff10112ac50e20c6bc201cfe0c11468357fedc778a81b42fbbd35493` |
| Current financial ELF | `7430db3b105a0da85bc64f92885f3cc3e9968371df79f49d28094939d9a96fb6` |

Both [Astra](evidence/native-financial-r3-failure/NATIVE-FINANCIAL-R3-FAILED-RESULT-ASTRA-REVIEW.md) and [Grok](evidence/native-financial-r3-failure/NATIVE-FINANCIAL-R3-FAILED-RESULT-GROK-REVIEW.md) approve source identity and faithful classification of the consumed Failure. Both refuse financial acceptance and future execution authority. Grok was requested as `grok-4.6` high; the actual receipt returned `grok-4.6-build`, `end_turn`, process exit 0. Astra was requested through host dispatch as `gpt-6-astra` medium; no separate provider-returned attestation is exposed. Root read both full reports and actual Grok terminal metadata.

## Cause and next predicate

Source fact: official ledger `9a8777c4d035fc7f38ae286bcf5f8656668efd9f` initializes all `24,000,000,000,000,000` atomic NIGHT in reserve. The local fixture then inserts a NIGHT UTXO worth `1,000,000,000,000` without debiting reserve. The apply-time invariant sums NIGHT across reserve, UTXOs and the other native pools. The observed total is exactly their sum. A1 escrow is a separate asset.

Inference: the invalid test genesis explains the application refusal. A cheap retained-genesis diagnostic must call the actual invariant, check a valid default state, remove the sole excess UTXO in an isolated clone, and verify the original remains invalid and unchanged. That removal supplies no funded replacement genesis. Diagnostic source and build outcomes require their own evidence; this note records no completed diagnostic.

The inspected public reserve → reward distribution → signed reward claim route is a possible local funding repair. Distribution and claim nonces must differ because distribution records its intent hash. Any successor must derive the actual produced funding UTXO and timestamp from native state, check supply after each transition, use real sealed signature validation, retain replay and wrong-signature controls, and reconstruct the same funding history independently. Privileged transitions on a trusted local genesis do not authenticate public-chain funding. The [source-only funding note](evidence/native-financial-r3-failure/NATIVE-NIGHT-FUNDING-ROUTE-SOURCE-NOTE.md) retains its original wording; its step 3 same-nonce wording is superseded by this explicit distinct-nonce requirement.

The failed proof and reservation remain immutable and consumed. A changed funding context requires a distinct proof and fresh phase authority. No fifth Compact compile, key regeneration, invariant bypass or wallet transaction follows from this report.

## Remaining acceptance

Native full Success, accepted escrow 8990, complete payout/UTXO accounting, protocol DUST fees, replay rejection, independent verification and mutation controls remain open. Authenticated source-owner/native-payer mapping, live funding/deployment, state/time/history, general Moriarty lowering, property/intent/transition/history correspondence and PCD remain open. The candidate is a manually specialized public development fixture. Published-source Preview format compatibility remains conditional on the deployed implementation; there is no new Preview transaction or settlement evidence.
