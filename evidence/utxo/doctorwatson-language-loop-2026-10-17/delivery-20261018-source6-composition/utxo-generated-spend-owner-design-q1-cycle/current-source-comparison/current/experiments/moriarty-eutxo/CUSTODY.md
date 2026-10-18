# T1 script custody

Callable path for `public-single-head-script-custody-transfer/1`. This is source and UPLC evidence. It is not a native spend, not a devnet result, and not an implementation approval.

## Command

```
node src/cli.mjs compile-custody --source <escrow-custody-v5.mori> --seed <financial-seed-custody-v5.json> --manifest examples/custody-unit.manifest.json --out <dir>
```

`compileCustody` in `src/compile-custody.mjs` is the same path the tests call.

## What the call does

1. `admitCustody` calls `createFinancialAgreementSourceV5().elaborate` on the source text.
2. It refuses profile `/6`, contract other than `moriarty-financial-expression-contract/4`, nonempty obligations, Text/Bool/Amount/Quantity ordinary fields, FloorDiv, Let, quantum other than the string `1`, shape-sample ids, all-zero ids, and `nativePins: true` (`NATIVE_PINS_ABSENT`).
3. It hashes the live Core with the revision03 domains. The measured pins are source `facb73a9…`, program `93ed8296…`, schema `2254f3cc…`, seed `fc93bf4a…`, bounds `b3d02ace…`.
4. `compileCustody` runs `evaluate` for release, refund, and cancel-empty and deep-compares release and refund with `E13-release-real-custody.expected.json` and `E11-refund-real-custody.expected.json`. Cancel-empty must match the E11 balances with used id `Cancel1`, empty obligations, and work `975/49/16`. E12 residual state stays `NONEMPTY_OBLIGATIONS`.
5. `lowerCustody` walks each action's Core before it renders. Require, Ensure, NextWrite, and the Transfer emit become Aiken. The recounted constructor count is 48 and the kernel debit is 49. A forged admission debit or a mutated core refuses before render. The debit literal is that recounted count plus one.
6. The ledger interval rule is separate from the source guards: inclusive lower equals `args.lower` and `last_valid_posix` equals `args.upper`. Original guards 1000, 1100, and 1200 stay in the source.
7. The UTXO frame checks one script input, one continuation at the same script address, and the admitted asset ids: thread policy plus name `Head` quantity 1, Cash policy plus name `Cash` equal to the Escrow kernel balance, and lovelace. Any other policy or asset name on the head input, continuation, or payout fails. Mint must be empty. An absent receiver balance row is appended in source order for any party and asset. That append is refused when the balance list length is already the lifecycle collection capacity 128. Every transfer is refused when `usedTransferIds` is already 128. An in-place credit at length 128 stays allowed. The bound is the lifecycle constant, not a new bounds-hash field. Duplicate balance or allowance keys fail the spend. The unit payout lovelace quantity is the fixture `2000000`. That quantity is not protocol min-ADA and not kernel Cash. Fee and collateral are a transaction-body expectation. The Plutus V3 `Transaction` value has no collateral field, so the spend validator does not treat extra inputs as collateral.
8. `compileCustody` removes `plutus.json` before admission, lowering, check, or build returns. An isolated copy of `eutxo-custody` runs `aiken check` and, only after check and build both return 0, copies `plutus.json`. `admission.json` `ok` is the same boolean as the returned `ok`. The public atomic project is not written.

## Unit fixture

`examples/custody-unit.manifest.json` uses repeating ids `a1`, `b2`, `c3`, `d4`, and `e5`. Those ids are not the shape sample and not all zero. `instance_id` is `hashDomain("MORIARTY-INSTANCE-financial-agreement/5", …)`. The compiled script hash in `plutus.json` is a different 28-byte value. A later native manifest has to recompute thread, cash, and script ids from real scripts and then recompute `instance_id`. This phase refuses `nativePins: true`.

## Time and receipts

`planWindows` is offline integer math. Slot length other than 1000 returns `SLOT_LENGTH_REFUSED`. Uint128 overflow returns `OVERFLOW`. The recipe text also says `IMPOSSIBLE_BOUNDARY` for that overflow; both refuse, and the executable code remains `OVERFLOW`. Negative POSIX is outside the admitted domain. `calibrateGuards` does not emit source in this phase.

The approved receipt helper is `tooling/transaction-receipts/revision02/submit-with-receipt.mjs` SHA `f3689ef82e078b9331907c4d13bde9ebfc3e2f501d88edb41d394c4f7479bf18`. This phase does not import or execute it. The rejected root helper is not imported.

## Gap this closes, and what stays open

The atomic backend stops script custody with `SCRIPT_CUSTODY_UNSUPPORTED` and compiles only `public-single-head-wallet-transfer/1`. This command compiles the financial-source/5 escrow into a spend validator and runs that validator on synthetic transactions. Devnet inclusion, POSIX parity, calibrated guard substitution, real policy ids, and the receipt helper are still later work. The 20-pattern inventory stays open. Private, PCD, and Complete are unverified. The lock that creates the head is a trusted fixture.
