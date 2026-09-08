# SP05 fixed kernel custody

Local Compact wrappers for the pinned loan and swap kernels. This directory is not a wallet, ledger, proof, Preview or sprint-acceptance result.

## Commands

Run from `experiments/moriarty-midnight-financial`.

```
node custody/generate.mjs --output-dir DIR
node custody/build.mjs --output-dir DIR --skip-zk
MORIARTY_CUSTODY_ARTIFACTS=DIR node --test --test-reporter=tap --test-timeout=90000 custody/runtime.test.mjs
```

`DIR` is an explicit untracked build directory. Tests fail when `MORIARTY_CUSTODY_ARTIFACTS` is absent. Build may create a temporary `node_modules` symlink under `DIR` to the pinned compact-runtime 0.16.0. It does not install packages.

## QueryContext balances

Pinned compact-runtime `createInitialQueryContext` replaces the whole `QueryContext.block` object. The WASM `block` getter returns a copy. Tests must assign a replacement block, then read back time, uncertainty and balances. Piecemeal field writes do not persist. Synthetic balances are test-context only. They are not ledger UTXOs, signed offers or payer ownership.

## Identities

Constructor and test addresses are synthetic 32-byte values published in `bindings.json`. Role secrets are generated in memory for each test and are not written to bindings, fixtures or reports. They are not Preview or wallet custody keys.

## Deferred ledger work

`receiveUnshielded` does not name a payer. Signed offers, consumed UTXO ownership, third-party funding, fees, change, finality and real rollback stay for a later ledger task. The blocked comparison utility is not imported. The compiled-runtime suite does not establish transaction funding or finality.
