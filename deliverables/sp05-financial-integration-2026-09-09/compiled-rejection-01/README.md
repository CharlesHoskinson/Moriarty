# Compiled swap rejection — source-only result

The retained fully compiled swap contract rejects three `close` calls through the actual installed `createUnprovenCallTxFromInitialStates`: wrong revision, program and network. The SDK returns `failed assert: REVISION_MISMATCH`, `PROGRAM_MISMATCH` or `NETWORK_MISMATCH` with `ContractRuntimeError` → `CompactError` causes. No unproven transaction result returns. All8150serialized public-state bytes and the serialized synthetic `{}` private input remain unchanged. The connected ZK configuration provider receives no lookup. The real build loader is closed and rejects later `assertFresh()` calls.

This is a call-preparation boundary result using a historical public initialized-state snapshot and synthetic role/coin-key bytes. Those guards precede capability validation. No original private state, wallet identity, credentials, proof generation, compiler, runtime service or network operation is used. The result does not establish current-state validity, ledger rollback, failed-transaction fees, adversarial payer behavior or on-chain nonmutation. SP05.2 and Preview acceptance remain open for those predicates.

The helper accepts only a receipt path and one of the three fixed mutations. Actual build/state/SDK entry hashes are fixed; the existing full loader checks all build artifacts and source provenance. Module mocks exist only inside tests and intercept the helper's actual imported call/loader interfaces. There are no disconnected proof/balance/submit/private-write counters. The dependency entry hashes are direct pins, not a new transitive supply-chain attestation.

## Reproduce

Use the existing retained artifacts; this command fails when the required environment input is absent and never installs or rebuilds them:

```sh
MORIARTY_SWAP_BUILD_RECEIPT=/home/charl/.local/state/moriarty/sp05-full-build-20260909-01/swap-output/build/build-receipt.json npm --prefix experiments/moriarty-midnight-financial run test:compiled-rejection
node --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/proven-assets.test.mjs
```

## Checks and history

- `red.tap`: test-first failure because the new checker export was absent. This establishes missing checker behavior, not a defect in the already-rejecting generated circuit.
- `green-01.tap`: the three real SDK/generated tests passed; 10/11 total passed. The public mutation test used the native `data` setter with a StateValue instead of its required ChargedState and therefore threw an unrelated error. This failed test remains retained.
- `green-02.tap`: corrected that test fixture to mutate actual native `balance` through its setter;11/11passed, zero skipped. Added actual loader post-close verification. Controls also reject unexpected success, unrelated errors, matching text without runtime cause, synthetic private mutation, connected ZK lookup, cleanup failure and caller SDK override fields.
- `loader-regression.tap`:18/18existing loader tests passed, zero skipped. `git diff --check` exited0.

Node24.18.1 module mocking requires the explicit experimental flag in the separate package script; the retained log includes its warning and the Node deprecation warning for `namedExports`. This source-only check changes no consumed allocation and admits no new campaign. Independent review is pending; no commit was made.

`inputs-and-source.json` binds the public inputs and implementation files. The original failed/successful financial attempts, `swap-close-state-01` and `swap-lessons-01` remain untouched.
