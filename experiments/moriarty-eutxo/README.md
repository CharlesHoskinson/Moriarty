# Moriarty public atomic eUTXO experiment

Public experiment only. It does not verify private/PCD claims and is not a production backend. It does not claim frozen Complete.

Pinned toolchain: `/home/charl/.aiken/bin/aiken` **v1.1.21+42babe5** with **stdlib v3.1.0**.

## Public command

Compile admitted `moriarty-bounded-atomic/1` source to an Aiken V3 spend validator, run Transaction-context tests, and emit a CIP-57 blueprint:

```sh
node experiments/moriarty-eutxo/src/cli.mjs compile \
  --source experiments/moriarty-eutxo/examples/revision-step.mori \
  --manifest experiments/moriarty-eutxo/examples/revision-step.manifest.json \
  --out experiments/moriarty-eutxo/artifacts
```

Run all five designer `verify.mjs` subprocesses, compile the prototype, and write the 20-pattern source/UPLC/native/private matrix (`--designers` is required). `validate` exits nonzero on failed Aiken build/test, failed verifier, missing pattern, or unexpected source result:

```sh
node experiments/moriarty-eutxo/src/cli.mjs validate \
  --designers /home/charl/moriarty-eutxo-validation-2026-10-12/designers \
  --manifest experiments/moriarty-eutxo/examples/revision-step.manifest.json \
  --out experiments/moriarty-eutxo/artifacts
```

```sh
node --test experiments/moriarty-eutxo/tests/*.test.mjs
```

## Admitted native subset

Public single-head wallet-transfer subset:

- Exactly one consumed **script credential** input (the thread head) plus VerificationKey inputs. A second script input is rejected regardless of thread policy.
- Native unit limit: every settlement binding `quantum.value` must be exactly `1`. The generated validator uses the source nominal amount as the ledger quantity. Any other quantum, including `2`, is a closed admission failure `SETTLEMENT_QUANTUM_UNSUPPORTED`. Source evaluation still converts `ledgerAmount = nominal / quantum` and rejects nondivisible amounts; this public subset does not implement that conversion.
- Net debit/credit of each mapped VerificationKey party and each admitted native asset equals the sum of source `Transfer`/`Fee` effects (inputs minus outputs). Net delta is not gross input debit. Wallet policy / fee collateral / whole-input spending remain unsupported.
- Head value, including ADA, is preserved. CASH native tokens are distinct from ADA. ADA settlement is unsupported unless the manifest sets an explicit `feePolicy`.
- Required signers include every debiting mapped party. Actor VKHs and asset (policy, name) pairs are injective.
- Script-held custody and Amount/Quantity state inventory (pool/loan projection) are closed admission failures.
- Injective per-action payout outputs are retained for multiple effects.

This is not multi-head composition and not full native coverage.

## Genesis

Thread NFT issuance is an **external** native precondition. Root uses the official Aiken consumed-seed one-shot mint policy. This spend validator constrains continuation of an already-issued head. It does not authenticate first issuance and does not perform PCD verification.

`artifacts/sample-datum.json` is the source-level genesis constructor: revision `0`, `values == manifest.initialState`, `remaining == lifetime`.

## Artifacts (`experiments/moriarty-eutxo/artifacts/`)

| file | role |
| --- | --- |
| `plutus.json` | CIP-57 blueprint, Plutus V3, validator title `atomic.atomic.spend`; published only after a successful isolated Aiken build |
| `atomic.atomic.spend.uplc` | textual UPLC dump |
| `atomic.ak` | generated validator (source-derived Core lowering) |
| `sample-datum.json` | genesis `HeadDatum` for the parent Yaci/Lucid driver |
| `sample-redeemer.json` | `Step` constructor 0, actor `payer`, payout `5` |
| `data-schemas.json` | Plutus Data field order and genesis constructor note |
| `admission.json` | capability, TIME_PREDICATE, native subset, hashes, unverified private claims |
| `matrix.json` | 20 representative patterns with linked cases; native overlay records the public prototype and keeps designer families unsupported |

Parent native driver (not this experiment) should submit the compiled `atomic.atomic.spend` script with:

- one genuine-head script input carrying thread NFT `policy=aaaaaaaa…aa` / `name=48454144` (`HEAD`) and inline `HeadDatum`
- VerificationKey wallet inputs that realize the source net CASH deltas
- exactly one continuation at the same full script address, same value including ADA, thread qty 1, source-computed post datum
- unique exact-quantity `Transfer`/`Fee` outputs (no shared double-satisfaction)
- required debiting-party VKH in `extra_signatories`
- finite validity interval; `obs.now` is last inclusive POSIX millisecond (`TIME_PREDICATE_v1`)
- mapped-asset mint quantity 0

Dummy script hash in unit tests is `ee…ee`. The parent ledger driver must use the real compiled script address. The isolated final native driver lives in the task root, not this package. See **Isolated final native** below.

## Actual model

- Frontend: `compile(source, bounds)` → Core (`UInt128`/`Amount`/`Text`/`Bool`; `Guard`/`Let`/`Set`/`Emit`).
- Admission: profile `moriarty-bounded-atomic/1` only; effects `Transfer`/`Fee`; exactly observation `now: UInt128`; manifest-bound injective actors/assets/thread/domain.
- Lowering: generic Core → checked UInt128 arithmetic, And/Or right-hand evaluation only inside the chosen branch, revision-0 genesis values, domain `0..MAX128` on every numeric prestate field and argument, net wallet deltas, one script head.
- Time: Plutus V3 `[lower, upper)` → last valid `upper-1` when exclusive. Unbounded intervals reject.
- Compilation means compiler acceptance of generated Aiken, not quantified compiler correctness.
- Private claims on admitted source are recorded and **explicitly not verified**.

## Gaps / explicit rejections

- `moriarty-financial-agreement-source/5` and `/6` (designers 01, 02, 04, 05 loan/escrow/authority): **UPLC unsupported** (`PROFILE_UNSUPPORTED`). Source suite still evaluates them via designer `verify.mjs`.
- Atomic extras: `DueCreated`/`DueSettled`, consent/oracle observations, unbound or non-injective actor/asset maps, Amount state inventory, ADA settlement without `feePolicy`, settlement `quantum` other than `1` (`SETTLEMENT_QUANTUM_UNSUPPORTED`): rejected, never omitted.
- Split/join/migrate/terminate: selector `SOURCE_ACTION_UNKNOWN` is distinct from unsupported source check `OPERATION_BINDING` on `unsupported-split.mori`.
- Escrow E06/E08 self-refund and E09/E10 unlock-only: frozen `cases.json` expected `FundedExpressionPrepared`; designer `verify.mjs` names `SELF_TRANSFER` / `EMPTY_BATCH`. Expected values are not copied into outputs.
- Native isolated Yaci/Lucid driver is parent-owned. `npm run validate` still writes `pending-unsupported` because it does not submit to a node. `overlay_native_matrix.py` then links the measured public Core prototype and marks all 20 designer families `unsupported-measured-gap`. FULL MORIARTY PRIVATE/FINANCIAL COMPATIBILITY IS NOT CONFIRMED.
- No production, no private/PCD verifier, no campaign/orchestration registration.

## Isolated final native

Task root `/home/charl/moriarty-eutxo-validation-2026-10-12`. Dossier: [ASSESSMENT.md](/home/charl/moriarty-eutxo-validation-2026-10-12/ASSESSMENT.md), [PATTERN-RESULTS.md](/home/charl/moriarty-eutxo-validation-2026-10-12/PATTERN-RESULTS.md), [INDEX.md](/home/charl/moriarty-eutxo-validation-2026-10-12/INDEX.md).

The live cluster and retained CBOR are a **single-use chain fixture**. Compile/validate in this package is rerunnable. Driver submit against the current network is not rerunnable.

Do not invoke `npm start.mjs` for Yaci (it uses `~/.yaci-cli.pid` and kills an existing process). Do not submit to the frozen old cluster. Do not print seed files or `devnet-start.log`. Do not restart the current live network. Do not resubmit retained CBOR. Validity windows were about 25 seconds. Receipts skip labels that already exist. A new native run needs a new namespace/home. Re-run of the driver against this chain is not turnkey.

```sh
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/start-final-devnet.py
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/wait-final-devnet.py
# faucet, collateral split, Cash mint, official gift-card HEAD NFT, then only on a fresh namespace:
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/compile-native.mjs
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/native-final.mjs
node /home/charl/moriarty-eutxo-validation-2026-10-12/tooling/final-runtime/native-final-t3b-race.mjs
```

Home symlink `/tmp/moriarty-eutxo-final-20261012`. Direct `yaci-cli`. Cluster name `default`. Epoch 3600. Haskell node 11.0.1 PV11. Horizon fixture `2000000000000` POSIX ms in task-root `examples/native-revision-step.mori` only. This package example keeps `2000000000`. `compile.validatorHash` is SHA-256 of generated Aiken. Deployed Cardano payment credential is `56d0170da0c16b0c8beb3031c3c8e8c1be003516be58295be123e26b`.

Rejected hashes (cardano-cli TxId of retained signed CBOR): T3 `5377a951…e97c` `InvalidWitnessesUTXOW`; T3b `1697c0c3…a72b` phase-2 wrong-state; loser `619cb0f7…d394` `BadInputsUTxO`. Race is sequential conflicting-spend. Unused extra `Text` arguments and rounding-none `FloorDiv` remain backend limits. Fee effect is not Cardano `txfee`. Full private/PCD is unconfirmed. This is not a 20-family native pass.

Measured hashes and reject classes: task-root `ASSESSMENT.md`. `npm run validate` overwrites `artifacts/matrix.json` native status. Restore the overlay after validate:

```sh
python3 /home/charl/moriarty-eutxo-validation-2026-10-12/overlay_native_matrix.py
```
