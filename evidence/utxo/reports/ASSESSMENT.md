# Scoped eUTXO public-compatibility assessment

Author: Grok 4.6 (author freeze). Independent Grok review is parent-owned after this freeze. This file does not self-approve. Historical Luna and GPT-6 audits remain in place.

**FULL MORIARTY PRIVATE/FINANCIAL COMPATIBILITY IS NOT CONFIRMED.**

## Scope

Task root: `/home/charl/moriarty-eutxo-validation-2026-10-12`.  
Worktree: `/home/charl/Moriarty-wt-eutxo-validation-20261012` branch `feat/eutxo-pattern-validation-20261012` base `84f64b3`.  
Immutable research: `/home/charl/moriarty-utxo-research-2026-10-12`.

This assessment covers:

1. Five independent designer suites (20 pattern families) on actual Moriarty source APIs and atomic Simulation.
2. Source-derived public Core predicate `public-single-head-wallet-transfer/1` compiled to Aiken V3 / UPLC.
3. Isolated final Haskell Cardano-node 11.0.1 Conway protocol version 11 native run of that repaired public subset, including rejecting controls.

It does not cover Midnight Preview, public chain, current user wallets, frozen Complete, private/PCD, or production deployment.

Moriarty develop CLI status at continuation: capability `SP01.6 loan-swap-subset`, blocked dependent dispatch, nextAction `sp01-loan-report`. That campaign is unrelated. This work used authorized file edits and isolated test-network execution without a new campaign.

## Source

All five designer `verify.mjs` programs pass. Combined they exercise 83 actual source API / Simulation controls plus unsupported-selector probes:

| Designer | Controls | Exit |
| --- | --- | --- |
| 01-state | 13 evaluate calls, four rejection predicates, three trust-boundary observations, four unsupported selectors, one Split source probe | 0 |
| 02-escrow-time | 26 frozen source cases (22 exact ordinary matches, 4 named profile limits) | 0 |
| 03-swaps-batch | 4 sources, 6 positive steps, 10 negative controls | 0 |
| 04-finance | 13 cases with complete ordered effects and mutation controls | 0 |
| 05-authority-private | 15 controls; certificate `OPERATION_BINDING`; private `UNKNOWN_DECLARATION` | 0 |

Receipt: `SOURCE-SUITE-RESULTS.json`. Command: `python3 run-source-suite.py`.

`npm --prefix experiments/moriarty-eutxo run validate` writes 20 patterns, 24 rows, 5 fresh verifiers, 0 discrepancies, 0 missing. This continuation reran it (`final-validate.stdout.json` ok true). Node tests 19/19. Aiken named tests 20/20 on both the example compile (`programHash` `ced6488b…`) and the native fixture compile (`programHash` `044d573f…`).

Source expressivity already covers the economic families. No new source grammar was required. Backend context bindings for the public subset are implemented features of `experiments/moriarty-eutxo`.

Profile `moriarty-financial-agreement-source/5` and `/6` remain the designer source physics. Their required claims stay unverified. Public experiment profile is `moriarty-bounded-atomic/1` only.

Admit guard SHA-256: `e1a156d05ac52b6e6e0d5a79e1619377e907495d909f8a07a90f66a04e0fbf65` (`src/admit.mjs`). Quantum other than `1` is `SETTLEMENT_QUANTUM_UNSUPPORTED`. Ledger-amount conversion is unimplemented in this public subset.

## UPLC

The admitted public Core counter/Transfer compiles through Aiken 1.1.21 + stdlib 3.1.0 to Plutus V3. Isolated final compile (`tooling/final-runtime/native-compile.json`):

- `ok` true, Aiken check 0, build 0, 20 named tests success
- `programHash` `044d573f236b22b3d322a9e573cf3b0f0972d03bb7e32fdc2a0f3bf22ba22e70`
- `sourceHash` `f187d73311da69d56de6f8cde4ace42dffd04b1ec45ec4ed8aed909485232ad1`
- `boundsHash` `ad0e1d45c9cfb5b1843d73f4d497d7d07f0450caddfcd49f3ef81f07f63d567c`
- `validatorHash` `d8c06b3ab11540a2875c255cbb31344b3b0db171fa48d74439dcb11b866ce7c8` (SHA-256 of generated Aiken `atomic.ak`, not the Cardano payment credential)
- CIP-57 / Lucid payment credential `56d0170da0c16b0c8beb3031c3c8e8c1be003516be58295be123e26b` bound to `addr_test1wptdq9cd5rqkkrytavcrrs7garqmuqp4z6l9s22muy37y6cgzmrzj`
- native predicate `public-single-head-wallet-transfer/1`
- `privateVerification` `explicitly-not-performed`
- `frozenComplete` false

Designer financial/5 and /6 sources stay `PROFILE_UNSUPPORTED` at the UPLC tier. Compilation of the prototype is compiler acceptance of generated Aiken, not quantified compiler correctness and not generic lowering of all 20 families.

Worktree example `examples/revision-step.mori` keeps horizon `2000000000`. The isolated native fixture replaces both horizon and time-guard with `2000000000000` POSIX ms. Do not treat example compile outputs (`programHash` `ced6488b…`, CIP-57 `98a365ef…`) as the deployed native script. Binding receipt: `tooling/final-runtime/native-script-binding.json`.

## Native (isolated final network)

Old Yaci cluster at `/tmp/moriarty-eutxo-20261012` froze at slot 4879 epoch 48. It was left unchanged. Old candidate head-lock `1c95081a1ff16570e4b6bab99775d194e1c5aab10a1c1a6e00ba580ef7056208` and step `d5a214bfbc5cf667dd15436506e8f77673da968966f1244a9e9ac05c41dc06e9` are historical pre-repair snapshots. They are not repaired-script approval. Unconfirmed old NFT v2 split `e13febb723f882a5a6869f9150ea2f6c62882ad395d768abbbb5ea2640a7978a` remains pending.

Isolated final home: symlink `/tmp/moriarty-eutxo-final-20261012` → `tooling/final-runtime/devnet`. Direct `yaci-cli` (not `npm start.mjs`). Cluster name `default`. Epoch length 3600. Ports: admin 20010, node 23001, submit-api 28090, store 28080, ogmios 21337. Haskell node Conway PV11, tip advancing.

Actual Haskell payment credential observed in phase-2 failure: `56d0170da0c16b0c8beb3031c3c8e8c1be003516be58295be123e26b`. Script address `addr_test1wptdq9cd5rqkkrytavcrrs7garqmuqp4z6l9s22muy37y6cgzmrzj`. Payee Enterprise `addr_test1vp8gu96lfqguvj9mx560qr8vvlsksdl5dt57lwy23msfumqcrd63y`.

### Confirmed transactions

| Label | Hash | Status |
| --- | --- | --- |
| wallet-collateral-split | `0ec8c92ff779b93dd21223faf55a0909e6d9b31833760082fce603df649c4188` | confirmed-by-utxo |
| cash-mint | `c7b5a51cf7be982ca236cda277d19bd988c227cd05702a61575ba803258c195e` | confirmed-by-utxo |
| one-shot-thread-mint | `28d839181e1edf7e7d6499df94fd6bd67c078c13b831f68f481e437a677fddeb` | confirmed-by-utxo |
| moriarty-final-head-lock | `93e7a3704b5e6b83dea6800af5b90eeb10f585d8aab14bd8e34014c2f7f27a9d` | confirmed-by-utxo; rev0 remaining8 count0; 20 ADA + HEAD NFT1 |
| moriarty-final-step-1 | `59c11a0edd665cb2c5aa9f77106323a7d6b695752f82344b4614e91ab0695c4e` | confirmed-by-utxo; rev1 remaining7 count1; CASH 5 to Enterprise payee |
| moriarty-final-step-2 | `25a8dbbb37a22272bfb9b8eaab203c056b3e0acb4d61933fed10f9b1b5dee0e6` | confirmed-by-utxo; rev2 remaining6 count2 |
| cash-split-for-race | `9039f6e009033d800dd7e142305b434ec49927ff196ed100e3a103a06c5b7338` | confirmed-by-utxo |
| moriarty-final-race-winner | `29a1751b08ce0bcb7b7a38fe3132e9f84734b3493f3c5d23b666482e364ce7b6` | confirmed-by-utxo; rev3 remaining5 count3 |

CASH policy `418948f1d75dce34dcc57987de4f48c7beb3047663950acc27e1abdd` name `43415348`. HEAD policy `3fc09b2589de6e124eb4f19fbb68bcabc67219bf3fcabe6e9499a1c5` name `48454144`. Seed for one-shot mint is split `0ec8c92f…#3`. Gift-card policy is official Aiken reference at `tooling/gift-card-reference/plutus.json`.

Latest readback: head `29a1751b…#0` preserves 20 ADA + NFT quantity 1. Three Enterprise CASH 5 payouts exist from step-1, step-2, and race-winner.

### Rejecting controls

Hashes are `cardano-cli 11.0.0.0 latest transaction txid` of retained signed CBOR. No extra submit.

| Label | Hash | Class | Status |
| --- | --- | --- | --- |
| T2 Ogmios evaluateTransaction of mutated continuation datum | (eval only) | jsonrpc 3010 / 3012 scripts failed to evaluate | provider eval reject |
| T3 `moriarty-final-wrong-datum` stale vkeys | `5377a951f5654965e560a6b188988adb4859bd276c3702341ec368d29fa7e97c` | `InvalidWitnessesUTXOW` | node submit reject; signature reject; not script wrong-state |
| T3b stripped-vkey re-sign, count 99 | `1697c0c373a86eea95b67763ac2e3a1c08150edfeb7030c6fae042954a8aa72b` | `ValidationTagMismatch (IsValid True) (FailedUnexpectedly (PlutusFailure))` | phase-2 script fail; headUnchanged true; collateral unconsumed because isValid true |
| race-loser sequential conflicting spend | `619cb0f72f6a24a55e46044cfd7142983a020873f5937521879478bedd0da394` | `BadInputsUTxO` of `25a8dbbb…` TxIx 0 | node submit reject after winner inclusion |
| original valid preimage | `ebeb43a45a7e6519386c74324f59570eeeebb6fa52b80a291d685369e4bbe97e` | unsubmitted | never submitted; never included |

Race type is sequential conflicting-spend: winner `29a1751b…` included, then loser submitted. Distinct wallet CASH funding inputs from `cash-split-for-race`. Shared head `25a8dbbb…#0`. This is not concurrent mempool racing.

T2 is Ogmios evaluate. T3/T3b/race-loser are Haskell submit-api / node. Failed CBOR is retained beside the receipts. Do not retry stale heads.

## Measured gaps (unsupported)

These remain closed or unproven. Test counts do not complete them.

- Private/PCD correspondence and deployment-owned authenticate / verifyAndCommit / requiredClaimRoot
- Frozen Complete
- Financial profiles `/5` and `/6` native/UPLC lowering
- Native script-held custody, Amount/Quantity inventory, gross-input debit, fee/collateral as source effects
- Multi-head, split, join, migrate, terminate
- Settlement quantum other than 1
- Constrained general genesis: gift-card binds seed/name/quantity only; program/address/revision of first head are a trusted fixture. External trusted genesis and general genesis are not proved.
- Unused extra `Text` action arguments compile and admit. They lower to unconstrained `ByteArray` with no UTF-8 or length enforcement.
- Policy provenance (`rounding-none` versus `FloorDiv`) is enforced in source Simulation. Admission and Aiken do not enforce it. A `floor_div` emit with rounding none admits and lowers to `checked_div`.
- `Fee` as an extra exact-quantity wallet output is in the admitted effect set. That effect is not Cardano `txfee`. Net delta is not gross whole-input.
- CIP-31 reference-input mutation as a source feature
- ADA settlement without explicit `feePolicy`
- Production publication of this adapter
- 20-family native matrix pass; private/PCD; frozen Complete

Fail-closed designer probes for unavailable full-financial and private-proof bodies remain useful.

## Graph

Luna targeted 20-family graph (`luna-experiments.json`, 82 nodes / 212 edges) is merged into the full inventory graph. Luna authorship and old candidate nodes stay. Grok native evidence is a later layer (`graphify-out/grok-native-evidence.json`). Coverage of 20+ docs is TARGETED, not a 29-million-word body review. Host cost/identity is unknown; unknown is not recorded as zero. Independent backend review `audit-grok-final/REVIEW-BACKEND.md` is scoped public-backend only. Independent native review `audit-grok-native-graph/REVIEW.md` is provisional live-ledger facts. Neither is this author freeze and neither is final native/graph approval.

Navigation: [INDEX.md](INDEX.md) · [PATTERN-RESULTS.md](PATTERN-RESULTS.md) · [GROK-AUTHOR-FREEZE.json](GROK-AUTHOR-FREEZE.json) · [NATIVE-MATRIX-OVERLAY.json](NATIVE-MATRIX-OVERLAY.json) · [graphify-out/EXPORT-VERIFICATION.json](graphify-out/EXPORT-VERIFICATION.json) · [audit-grok-final/REVIEW-BACKEND.md](audit-grok-final/REVIEW-BACKEND.md) · [audit-grok-native-graph/REVIEW.md](audit-grok-native-graph/REVIEW.md)

## Commands

Source matrix and prototype compile:

```sh
npm --prefix /home/charl/Moriarty-wt-eutxo-validation-20261012/experiments/moriarty-eutxo run validate
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/overlay_native_matrix.py
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/run-source-suite.py
```

Isolated final native is a **single-use chain fixture**. Do not use `npm start.mjs`; it kills the global yaci-cli pid. Do not dump `devnet-start.log` or seed files. Do not restart the live cluster. Do not resubmit retained CBOR. Validity windows were about 25 seconds. Receipts skip labels that already exist. A new native run needs a new namespace/home.

```sh
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/start-final-devnet.py
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/wait-final-devnet.py
# after faucet/split/cash/thread policies exist, and only on a fresh namespace:
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/compile-native.mjs
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/native-final.mjs
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/native-final-t3b-race.mjs
```

Re-run of those driver scripts against the current live chain is not turnkey. Overlay restore after `npm run validate`:

```sh
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/overlay_native_matrix.py
```

Node tests: `npm --prefix experiments/moriarty-eutxo test`. This author freeze does not re-run the 19 Node / 20 Aiken suites. Independent Grok review is parent-owned after this freeze. This file does not self-approve native or graph.
