# FOREMAN_REPORT

Status: source authored in this worktree for original finding R5. This is not ledger acceptance, PCD, SP05 completion, or operational admission.

The incomplete R6 review remains unaccepted. This pass did not resume that audit or its transaction probes.

## Outcome

`createFinancialProviders` now constructs pinned private-state, ZK, proof, and public-data providers from explicit caller configuration. The returned bundle includes `walletProvider` and `midnightProvider` with the pinned midnight-js-contracts method shapes.

Root diagnosis before the edit, live module:

- Direct `levelPrivateStateProvider({privateStateStoreName, privateStateStorePath})` threw `privateStoragePasswordProvider is required.`
- The returned bundle lacked `walletProvider` and `midnightProvider`.

After the edit, missing password, account, or compiled-assets input fails at the `createFinancialProviders` boundary. The private-state constructor receives `midnightDbName`, `accountId`, and the caller password callback. It does not receive `privateStateStorePath`. `NodeZkConfigProvider` receives a compiled-assets root that is not the private-state directory.

## Owned files

| File | Bytes | SHA-256 |
|---|---|---|
| experiments/moriarty-midnight-financial/ledger/providers.mjs | 65732 | 0efb5f394a895afb7ee7cb946a14a17542dcee42c9f1ce7beff01516c9b1efa9 |
| experiments/moriarty-midnight-financial/ledger/receipt.test.mjs | 83019 | 654e288dd71aeeb5e0fe1dc305c032f8a3cd1dd065c76ee1ec6329254bd5b15b |
| FOREMAN_REPORT.json | fourth owned file | hash after freeze |
| FOREMAN_REPORT.md | this file | hash after freeze |

Four-file total stays under 262144. Unowned parent files keep freeze hashes. Package and WASM pins were not modified.

## Caller requirements

The caller must supply:

- `privateStoragePasswordProvider`, a function. This pass does not read environment files, invent a password, or log callback output.
- `accountId`, a non-empty string. This pass does not generate or persist a wallet identity.
- `privateStateLocation`, used as `midnightDbName`.
- `compiledAssetsLocation`, or the same path declared on `provenAssetManifest`. The path must differ from `privateStateLocation`.
- `walletContext.shieldedSecretKeys.coinPublicKey` and `encryptionPublicKey`.

`proven: true` without `inspectedProofAssets` still fails as a false proven manifest. Unproven assets stay labeled unproven. R8 still owns proven-asset admission.

## Changed provider interfaces

`walletProvider` exposes `getCoinPublicKey`, `getEncryptionPublicKey`, and `balanceTx`. `balanceTx` maps the existing facade `balanceUnboundTransaction` then `finalizeRecipe`.

`midnightProvider.submitTx` composes the existing `wrapMidnightSubmit`. It does not duplicate submission. The `fundUnshielded` body is unchanged.

## Commands

Pre-repair diagnosis, live providers:

```
node --input-type=module
```

Result: `constructorError` `privateStoragePasswordProvider is required.` `returnedWalletProvider` false. `returnedMidnightProvider` false.

First failing owned R5 tests, before the constructor change:

```
node --test --test-reporter=tap --test-timeout=90000 --test-name-pattern='^R5 ' experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 1. Tests 4. Pass 0. Fail 4.

Final focused R5 tests:

```
node --test --test-reporter=tap --test-timeout=90000 --test-name-pattern='^R5 ' experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 0. Tests 4. Pass 4. Fail 0.

Owned file after fixture updates:

```
node --test --test-reporter=tap --test-timeout=90000 experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 0. Tests 44. Pass 44. Fail 0. This rerun checked constructor-caller fixtures in the owned test file. It is not an R6 financial audit. Predecessor `gpt6-independent-probes` scripts were not run.

## Source and pin checks

Protected `providers.mjs` bytes before `createFinancialProviders` still hash to `ecb3fa6f90feb75daa44bbd6f0d721b3cd1d5887fda25f8b2f959429b35d9c0c`. The `fundUnshielded` function still hashes to `4bb791ca1bd9929e279a6c4b71d15e392c772ca07a684676e87544ef6eed0539`. Package and WASM pins were not modified.

Pinned SDK files used as references still match the binding hashes for:

- `midnight-js-level-private-state-provider/dist/index.mjs`
- `midnight-js-contracts/dist/index.mjs`
- `midnight-js-node-zk-config-provider/dist/index.mjs`
- `midnight-js-types/dist/index.d.mts`
- hello-world `src/deploy.ts`

Unowned parent files keep freeze hashes. No Git writes.

## Remaining limitations

`run-local.mjs` is unowned. It still does not pass password, account, or compiled-assets fields.

Constructor tests instantiate providers. They do not restore a wallet, submit or finalize a transaction, generate a proof, or contact a network.

R1 through R4, R6 through R8, the incomplete R6 review, plugin milestones, full BNF/K, PCD, and financial Preview acceptance remain open. This pass authorizes no deployment or on-chain action.
