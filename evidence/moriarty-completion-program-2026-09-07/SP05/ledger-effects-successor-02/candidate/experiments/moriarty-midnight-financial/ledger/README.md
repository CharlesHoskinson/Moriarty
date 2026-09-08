# Fixed financial ledger integration source

This directory holds offline source for local Midnight loan/swap custody integration. It is not an operational admission and it does not claim ledger settlement, PCD, SP05 completion, or on-chain acceptance.

## Owned files

- `build-proven.mjs` — wrapper pin check and proven-compile command construction. Full `compact compile` without `--skip-zk` stays disabled until a later reviewed admission inspects the compiler and proof assets. An injected adapter may only emit an `unproven-test-manifest`.
- `providers.mjs` — hash-validating loader for parent-pinned SDK entries, `createFinancialProviders`, and unshielded fund/submit control. Import does not start a wallet, network, or compiler.
- `decode-receipt.mjs` — public receipt decode, closed validator, and financial oracle projection.
- `run-local.mjs` — `runLocalFinancialCase(options)`. Import is inert. The CLI explains missing operational admission and does not invent identities.
- `receipt.test.mjs` — offline tests. Synthetic in-memory keys and mocked transport only.

## Pinned APIs

Loader validates `package.json`, present runtime entry, and package-owned WASM hashes before import. Absent compact-js/platform-js CJS entries are not admitted. Node uses `@midnight-ntwrk/ledger-v8/midnight_ledger_wasm_fs.js`.

Public data reads use `BlockHashConfig` `{type:'blockHash', blockHash}` with `queryContractState` and `queryUnshieldedBalances`.

Token funding order: `balanceUnboundTransaction` → `signRecipe` → validate signed inputs → reserve submission/gross-spend → `finalizeRecipe` → structural recheck → `submitTransaction`. Finalization registers pending wallet state, so validation and reserve happen first. Local failures record known identifiers as unsubmitted-but-pending and stop. `transactionHash()` is not an identifier.

`CompiledContract.make(...).pipe(withVacantWitnesses, withCompiledFileAssets(...))`, `NodeZkConfigProvider`, `httpClientProofProvider`, `indexerPublicDataProvider`, `levelPrivateStateProvider`, `deployContract`, and `submitCallTx` are wired to pinned signatures. They cannot run without a later reviewed operational admission.

## Oracle projection labels

`compareFinancialEffects` accepts only synthetic-local records and always returns `networkAcceptance: false`. A real receipt stays separate.

The oracle projection maps a public receipt into that synthetic schema:

- **observed** — exact receipt or readback path (paid amount, reserve, due, residual, work, revision).
- **source-derived** — exact pinned `.mori` constant bound to the deployed `programDigest`. Generated metadata may check field layout and compilation identity only. It does not supply numeric source parameters.
- **arithmetic-derived** — explicit integer expression over observed and source-derived values. Example: from pre-accrual notional `n`, remainder `n*8*31 % (100*365)` reduced by gcd. At `n=5000000000` that is `54/73`.

Projection success is arithmetic comparison only. It does not establish ledger acceptance. Color/address binding, fees, canonical block, and finality checks remain mandatory and separate.
