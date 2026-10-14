# SP05 fixed kernel custody

Local Compact wrappers for the pinned loan and swap kernels. This directory is not a wallet, ledger, proof, Preview or sprint-acceptance result.

## Commands

Give an explicit output directory. Run generate and build as their own Node entry files.

```
node experiments/moriarty-midnight-financial/custody/generate.mjs --output-dir DIR
node experiments/moriarty-midnight-financial/custody/build.mjs --output-dir DIR --skip-zk
MORIARTY_CUSTODY_ARTIFACTS=DIR node --test --test-reporter=tap --test-timeout=90000 experiments/moriarty-midnight-financial/custody/runtime.test.mjs
```

`DIR` is an untracked build directory. Tests fail when `MORIARTY_CUSTODY_ARTIFACTS` is absent. Build may create a temporary `node_modules` symlink under `DIR` to the pinned compact-runtime 0.16.0. It does not install packages.

`isDirectRun` requires the caller module URL. Direct `node build.mjs` uses `build.mjs` identity and enters compilation. Import of `build.mjs` does not run the compiler. A zero exit with empty stdout is not compile evidence. Fake compiler counters are not compile evidence.

This completion compiled skip-zk into `state/build/binding-completion/20260908T084719Z`. Compiler outputs exist under that path. Two generate runs wrote byte-identical wrappers.

## QueryContext balances

Pinned compact-runtime `createInitialQueryContext` replaces the whole `QueryContext.block` object. The WASM `block` getter returns a copy. Tests must assign a replacement block, then read back time, uncertainty and balances. Piecemeal field writes do not persist. Synthetic balances are test-context only. They are not ledger UTXOs, signed offers or payer ownership.

## Constructor and reserves

Constructors store the pinned program digest from metadata. They reject the 0x7f-repeat-32 value. Duplicate `bindings.loan.programDigest` and `bindings.swap.programDigest` must equal the metadata hashes. Swap and close require both entry token balances to equal the kernel reserve fields. Close rejects a one-unit surplus. After close, a later swap uses the post-effect 0/0 balances and fails with `epoch is closed`. Tests carry balances by actual mint, input and output deltas.

## Identities

Constructor and test addresses are synthetic 32-byte values published in `bindings.json`. Role secrets are generated in memory for each test and are not written to bindings, fixtures or reports. They are not Preview or wallet custody keys.

## Deferred ledger work

`receiveUnshielded` does not name a payer. Signed offers, consumed UTXO ownership, third-party funding, fees, change, finality and real rollback stay for a later ledger task. The blocked comparison utility is not imported. The compiled-runtime suite does not establish transaction funding or finality.
