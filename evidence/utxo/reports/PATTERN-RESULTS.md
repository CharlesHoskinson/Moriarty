# Pattern results

Source oracle: five independent `designers/*/verify.mjs` programs. Inventory: `PATTERN-INVENTORY.json`. Matrix: `experiments/moriarty-eutxo/artifacts/matrix.json` after `overlay_native_matrix.py`.

UPLC compiled-tested: public Core prototype only. Native measured: that prototype plus rejecting controls. Private: unsupported on every row.

## 20 families

| ID | Designer | Source | UPLC | Native | Private |
| --- | --- | --- | --- | --- | --- |
| S01 Authenticated genesis and counterfeit state | 01-state | verifier pass; financial/6 | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| S02 Consume-and-recreate, conflict, reference-read | 01-state | verifier pass; financial/6 | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| S03 Split/join conserved authority | 01-state | SOURCE_ACTION_UNKNOWN / OPERATION_BINDING | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| S04 Migration and termination | 01-state | SOURCE_ACTION_UNKNOWN | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| escrow-time | 02-escrow-time | 26 cases; E06/E08 SELF_TRANSFER, E09/E10 EMPTY_BATCH named limits | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| interval-boundary | 02-escrow-time | verifier pass | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| joint-role | 02-escrow-time | verifier pass | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| oracle-observation | 02-escrow-time | verifier pass | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| bilateral atomic escrow swap | 03-swaps-batch | 6 positive / 10 negative on 4 sources | prototype UPLC is a different admitted source | unsupported-measured-gap | unsupported |
| constant-product rounding/slippage | 03-swaps-batch | source pass | unsupported for this family | unsupported-measured-gap | unsupported |
| order execution versus timeout | 03-swaps-batch | source pass | unsupported for this family | unsupported-measured-gap | unsupported |
| two-order batch / double satisfaction | 03-swaps-batch | source pass; UPLC unit tests cover single-script and shared-payout reject on the prototype | unsupported for this family as native | unsupported-measured-gap | unsupported |
| F01-origination | 04-finance | FundedExpressionPrepared + INSUFFICIENT_BALANCE | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| F02-interest-period | 04-finance | accrue + PERIOD_NOT_ELIGIBLE / DUPLICATE / LIABILITY_CAP | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| F03-partial-repayment-residual-liability | 04-finance | partial + replay DUPLICATE + final settle | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| F04-fee-gross-net-policy | 04-finance | fee + INTENT_NET_GOAL + INTENT_DEBIT_CAP | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| AP01 Fee-inclusive bounded outcome | 05-authority-private | source pass | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| AP02 Per-instance durable nonce | 05-authority-private | source pass | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| AP03 Native multisig/role authority | 05-authority-private | source pass | PROFILE_UNSUPPORTED | unsupported-measured-gap | unsupported |
| AP04 Failclosed unsupported certificate/private | 05-authority-private | OPERATION_BINDING / UNKNOWN_DECLARATION | closed | unsupported-measured-gap | unsupported (probe executed) |

S02 consume-and-recreate is adjacent in intent to the public Core prototype. It is a financial/6 `head-step.mori` family. The native script is `moriarty-bounded-atomic/1` `native-revision-step.mori`. Do not treat S02 as natively validated.

Escrow E06/E08 self-refund and E09/E10 unlock-only: frozen `cases.json` still records `FundedExpressionPrepared`. Designer `verify.mjs` names `SELF_TRANSFER` / `EMPTY_BATCH`. Expected values are not copied into outputs.

## 83 source controls

13 (01-state) + 26 (02-escrow-time) + 16 (03-swaps-batch: 6 positive + 10 negative) + 13 (04-finance) + 15 (05-authority-private) = 83.

Logs: `suite-01-state.log` … `suite-05-authority-private.log`. Combined receipt: `SOURCE-SUITE-RESULTS.json` and `source-suite.stdout.json`.

## Public Core prototype (native + UPLC)

Source: `tooling/final-runtime/examples/native-revision-step.mori` (horizon fixture `2000000000000`). Compiler: worktree `compilePublic`. Manifest: live cash/thread policy IDs.

UPLC named tests (all success): positive_step, missing_continuation, multiple_continuation, wrong_token, wrong_address, wrong_datum, changed_result, signer_missing, stale_program_hash, conflicting_predecessor, shared_payout, unbounded_time, wallet_debit_required, cross_script_head, forged_genesis_values, prestate_out_of_range, argument_out_of_range, arithmetic_boundary_count, shortcircuit_or, shortcircuit_and.

Native sequence on isolated Haskell node:

1. Lock rev0 remaining8 count0 (`93e7a370…`).
2. Step-1 rev1 remaining7 count1 payout CASH 5 (`59c11a0e…`).
3. Step-2 rev2 remaining6 count2 (`25a8dbbb…`).
4. T3 stale-vkey mutate hash `5377a951f5654965e560a6b188988adb4859bd276c3702341ec368d29fa7e97c`: `InvalidWitnessesUTXOW` (signature reject, not script wrong-state).
5. T2 Ogmios evaluate of mutated body: 3010/3012.
6. T3b stripped-vkey mutate count 99 hash `1697c0c373a86eea95b67763ac2e3a1c08150edfeb7030c6fae042954a8aa72b`: `ValidationTagMismatch` IsValid True; head unchanged.
7. Sequential conflicting-spend: winner rev3 remaining5 count3 (`29a1751b…`) included, then loser `619cb0f72f6a24a55e46044cfd7142983a020873f5937521879478bedd0da394` `BadInputsUTxO`. Distinct CASH funding inputs. Shared head. Not concurrent mempool racing.
8. Unsubmitted original preimage `ebeb43a45a7e6519386c74324f59570eeeebb6fa52b80a291d685369e4bbe97e` retained; never included.

Receipts: `tooling/final-runtime/native-final-transactions.json`, `native-final-readback.json`, `native-final-readback-step-1.json`, `native-final-readback-step-2.json`, `native-final-wrong-datum.json`, `native-final-wrong-datum-t3b.json`, `native-final-t2-ogmios.json`, `native-compile.json`, `native-script-binding.json`. Signed CBOR retained next to those files. `compile.validatorHash` is SHA-256 of generated Aiken. Cardano payment credential is `56d0170d…e26b`. Native horizon `2000000000000` POSIX ms is distinct from source example `2000000000`.

## Gaps retained as results

- Authenticated general genesis (S01 native predicate). External trusted genesis is not proved.
- Split/join/migrate/terminate
- Financial custody and gross debit
- Custom quantum
- Private proof / certificate
- Multi-head
- Full 20-family native suite
- Unused extra `Text` arguments admit as unconstrained `ByteArray` (no UTF-8/length enforcement)
- rounding-none `FloorDiv` policy is source Simulation only, not admission/Aiken
- Fee effect is not Cardano `txfee`; net delta is not gross whole-input

Honest negatives are the measured result for those families. This is not a 20-native-family claim. Private/PCD remain unconfirmed.

Navigation: [INDEX.md](INDEX.md) · [ASSESSMENT.md](ASSESSMENT.md) · [NATIVE-MATRIX-OVERLAY.json](NATIVE-MATRIX-OVERLAY.json) · [GROK-AUTHOR-FREEZE.json](GROK-AUTHOR-FREEZE.json)
