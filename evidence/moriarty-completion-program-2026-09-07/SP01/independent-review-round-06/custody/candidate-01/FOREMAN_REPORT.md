# FOREMAN_REPORT

Worker: Grok 4.6. Task: SP05 kernel custody completion. Scope: generated Compact wrappers and compiled-runtime tests only.

## Result

The completion reused the existing skip-zk loan and swap artifacts. It did not restart the wrappers. The compiled-runtime suite now reports 17 passed tests and 0 failed tests.

SP05, ledger, Preview, proof, wallet, full BNF, K, and sprint acceptance remain open. Runtime transcripts do not establish transaction funding, payer ownership, or finality. Independent GPT-6 Astra result review is required.

## Defect

Pinned compact-runtime `createInitialQueryContext` replaces the whole `QueryContext.block` object. The WASM `block` getter returns a copy. The prior test helper wrote `balance`, `lastBlockTime`, and `secondsSinceEpochErr` on that copy.

Readback after piecemeal writes showed empty balance, uncertainty 0, and `lastBlockTime` 0. `secondsSinceEpoch` 1700000000 persisted because `createCircuitContext` already assigned time through a whole-block write.

A replacement block object persisted configured balances, time, and uncertainty. `rawTokenType(domain, dummyContractAddress())` matched ledger `colorA` and `colorB`.

## Fix

`runtime.test.mjs` now assigns a replacement block and reads the values back. Circuit `INSUFFICIENT_ENTRY_RESERVE_A` and `INSUFFICIENT_ENTRY_RESERVE_B` guards were not removed, weakened, or mocked. `generate.mjs`, `loan.compact`, `swap.compact`, kernel, and arithmetic were not changed.

Synthetic balances remain an explicit test-context boundary. Init mints do not fund the next call.

## Tests

The preserved old red capture failed because `MORIARTY_CUSTODY_ARTIFACTS` was absent. That capture lives in `state/raw/red.stdout` and `state/raw/red.stderr`. argv was not recorded there.

The root diagnostic against the same artifacts reported 15 tests, 10 passed, and 5 failed. Failures 6, 7, 8, 9, and 14 raised `INSUFFICIENT_ENTRY_RESERVE_A` or `INSUFFICIENT_ENTRY_RESERVE_B`. That is a different failure from the missing-artifact red.

After the context fix, funded swap and close paths passed. A 19742 B reserve failed at `INSUFFICIENT_ENTRY_RESERVE_B`. Close with A=0 failed at `INSUFFICIENT_ENTRY_RESERVE_A`. Close with B=0 failed at `INSUFFICIENT_ENTRY_RESERVE_B`. `minOut` 19700 and 1 passed. Isolated `minOut` 19744 failed at `minimum output not met`. Pinned source still permits close before swap. Swap after close failed at `epoch is closed`. Remaining did not refresh to 8.

Tests inspect unshielded mints, inputs, outputs, and claimed spends for color, recipient tag, recipient address, and amount. Loan init claims USD to the synthetic borrower. Settle claims USD to the synthetic lender. Swap output claims asset B to the synthetic trader. Close claims remaining A and B to the synthetic provider.

Exact stdout, stderr, sha256, argv, and exits are in `experiments/moriarty-midnight-financial/custody/test-evidence.json`.

## Reproducibility

Two generate runs into `state/gen-a` and `state/gen-b` produced wrappers equal to owned `loan.compact` and `swap.compact`.

- loan wrapper sha256 `3a4cbce4a31521dd56f0e7bb368a86ef417e84dd9f302cc2e45023881dbf291f`
- swap wrapper sha256 `77eef85b796fabb1d1b78dd51f573053121d72d172e2bd03e9f8f55c11f0967d`
- arithmetic sha256 `b2969d0bc346fa93d88546578c9f9d29f1b74d7bc3f88f2e04709fa0932d9415`
- loan kernel sha256 `77e44b28b191b6c7ca904f92356f0da006a13839ff241393e0cf6088959cf363`
- swap kernel sha256 `090b9e2ecf5a0850bee0d46570ca9edb3ac374743e2ed29235913c6e1bb58741`

Pinned kernel and arithmetic bytes match the existing build stage copies. This completion issued 0 compile invocations because owned wrappers were unchanged.

## Secrets

Role secrets are generated in memory for each test. They are not in bindings, evidence, or this report. Constructor addresses are synthetic. They are not wallet custody keys.

## Open

Independent GPT-6 Astra must review this result. Merge status is not product acceptance.
