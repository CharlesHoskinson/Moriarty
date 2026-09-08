# FOREMAN_REPORT

Status: source authored in this worktree. This is not ledger acceptance, PCD, SP05 completion, or an operational admission.

## Outcome

The owned ledger source implements the approved packet for offline use. Tests were written first. The first substantive red was 26 failed tests because the owned modules were absent. One test passed against accepted `generate.mjs`. The final green run is 27 passed, 0 failed, exit 0.

## Owned files

| File | Bytes | SHA-256 |
|---|---|---|
| experiments/moriarty-midnight-financial/ledger/README.md | 3163 | 8dad40c67e53b04c49c0c317bd6da3de4d9b430f473e71051d388aa3a8fc5a2e |
| experiments/moriarty-midnight-financial/ledger/build-proven.mjs | 4796 | d0120812b0f46f35c8fce31333288c2876720bdb3bb0e26877104349477010ee |
| experiments/moriarty-midnight-financial/ledger/providers.mjs | 37593 | 747109368a969e21c9d46308a5c5e95e0b0bcffbc30dab9f795827f12ca11e2a |
| experiments/moriarty-midnight-financial/ledger/decode-receipt.mjs | 21154 | fe594c9fab7457fdc3abe228927ee92b38af5873ed2075b9b4048c52d4b5a104 |
| experiments/moriarty-midnight-financial/ledger/run-local.mjs | 5253 | 0c5d5347923d169ffb7c9090060f7846ae0d6449378aade4ca7ec779c64bf433 |
| experiments/moriarty-midnight-financial/ledger/receipt.test.mjs | 37579 | 94d3955f2956d5d81521bd25e8c6fbc74c09a4faab45d4cb118caa1c4b7624c6 |
| FOREMAN_REPORT.json | eighth owned file | hash after freeze |
| FOREMAN_REPORT.md | 4413 | ace351ca3e9ad0a2a47ca27cc89ef4b68ccda78877fa6c5e9ca796a93a8c4d43 |

Eight-file total at last measurement is 119610 bytes, under 262144. Source six-file total is 109538 bytes.

Accepted custody and comparator bytes were not modified.

## Commands

First substantive red:

```
node --test --test-reporter=tap --test-timeout=30000 experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 1. Tests 27. Pass 1. Fail 26. The passing test checked frozen `validatePinnedInputs` and `generateWrappers` names. Failures were missing owned modules.

Final green:

```
node --test --test-reporter=tap --test-timeout=90000 experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 0. Tests 27. Pass 27. Fail 0.

No Git writes. No network. No compiler spawn. No proof. No wallet restore. No `deploy.ts` import.

## API references used

- `WalletFacade.balanceUnboundTransaction(tx,{shieldedSecretKeys,dustSecretKey},{ttl})`
- `WalletFacade.signRecipe(recipe,payload=>unshieldedKeystore.signData(payload))`
- `WalletFacade.finalizeRecipe(signedRecipe)` then `submitTransaction(finalized)`
- `PublicDataProvider.queryContractState(address,{type:'blockHash',blockHash})`
- `PublicDataProvider.queryUnshieldedBalances(address,{type:'blockHash',blockHash})`
- `Transaction.deserialize('signature','proof','binding',raw)`
- `sampleSigningKey`, `signData`, `verifySignature`, `addressFromKey`, `rawTokenType`
- `CompiledContract.make`, `withVacantWitnesses`, `withCompiledFileAssets`
- `NodeZkConfigProvider`, `httpClientProofProvider`, `indexerPublicDataProvider`, `levelPrivateStateProvider`
- `deployContract`, `submitCallTx`
- Accepted `generate.mjs` exports `validatePinnedInputs` and `generateWrappers`
- Accepted `compareFinancialEffects` always returns `networkAcceptance: false`

Runtime entry lists are treated as sets. 23 advertised CJS entries remain absent and are not admitted.

## Test fixture provenance

All fixtures are synthetic-test. Keys come from in-memory `sampleSigningKey`. Transport is mocked. Comparator expected records are the unchanged accepted `loan.json` and `swap.json`. Source constants come from pinned `loan.mori` and `swap.mori`. Metadata is used only for field layout and program identity.

## Unresolved gaps

- No positive proven finalized-byte fixture was constructed offline. The production decoder rejects mocked finalize objects. Signature and offer checks use the pinned ledger API. Full decoder coverage of proven bound bytes is not claimed.
- Full compiler and proof generation remain disabled. An injected adapter can only emit `unproven-test-manifest`.
- `runLocalFinancialCase` is implemented and stays blocked without a later reviewed operational admission. That admission is not filled with invented constants.
- Oracle projection arithmetic does not establish ledger acceptance.

This source does not claim on-chain settlement, PCD, SP05 completion, or twelve-sprint completion.
