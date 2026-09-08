# FOREMAN_REPORT

Status: source authored in this worktree. This is not ledger acceptance, PCD, SP05 completion, or an operational admission.

## Outcome

`recheckFinalizedAgainstRecipe` now compares complete signed-recipe effects with the finalized transaction. The helper walks every recipe transaction that the pinned Wallet SDK recipe types expose. It compares guaranteed and fallible offers, input and output identity, signature slots, `signatureData` when both sides expose it, and TTL. It rejects missing, extra, and colliding intent segments. It does not sort offer arrays and it does not convert identities with `String`.

The pinned SDK merge of disjoint-segment unproven transactions is the accepted combination. `Transaction.merge` unions intents by segment. Same-segment merge fails with a key collision, and the helper rejects that mapping. `bind` changes serialized bytes. Effect identities and `signatureData` stay the same, so the helper does not require byte equality after bind.

This repairs the complete-effects part of GPT-6 finding R6. Gross allowance computation and reservation stay open. R1 through R5, R7, and R8 stay open.

## Owned files

| File | Bytes | SHA-256 |
|---|---|---|
| experiments/moriarty-midnight-financial/ledger/providers.mjs | 44726 | ace3d039c8b3eb7d4f6e503a9b44899fd34c61b40c4dda0e922db35e93a122bc |
| experiments/moriarty-midnight-financial/ledger/receipt.test.mjs | 49534 | 4cd124c9ebfdfb1fd58fdea9497696dfb1a32be5e9cf4a072b2d6f10f25f3b3b |
| FOREMAN_REPORT.json | fourth owned file | hash after freeze |
| FOREMAN_REPORT.md | this file | hash after freeze |

Four-file total stays under 262144 with the six unchanged parent files. Unowned parent files keep freeze hashes.

## Commands

Reproduction before edit:

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-sdk-effects.test.mjs
```

Exit 1. Tests 3. Pass 1. Fail 2. Both failures were Missing expected exception at `root-sdk-effects.test.mjs:48`. The unchanged SDK control passed. Native signature verification already rejected the mutated copy.

First failing owned comparison tests after the new controls were added, and before the helper change:

```
node --test --test-reporter=tap --test-timeout=90000 --test-name-pattern='native signed|native comparison|native merge|colliding recipe|synthetic complete-effects' experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 1. Tests 6. Pass 2. Fail 4. The unchanged native copy and the disjoint merge already passed. Mutation cases failed only because the helper did not throw.

Final owned suite:

```
node --test --test-reporter=tap --test-timeout=90000 experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 0. Tests 33. Pass 33. Fail 0.

Retained diagnostics after repair:

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-effects.test.mjs
```

Exit 0. Tests 7. Pass 7. Fail 0.

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-sdk-effects.test.mjs
```

Exit 0. Tests 3. Pass 3. Fail 0.

No Git writes. No network. No compiler spawn. No proof. No wallet restore. No `deploy.ts` import. Original diagnostic files were not edited.

## Supported production path

`fundUnshielded` still validates the signed recipe, reserves, calls `finalizeRecipe`, then calls this helper before submit. Recipe shapes:

- `UNBOUND_TRANSACTION`: `baseTransaction` plus optional `balancingTransaction`
- `UNPROVEN_TRANSACTION`: `transaction`, with `baseTransaction` as a diagnostic fallback
- `FINALIZED_TRANSACTION`: `originalTransaction` plus `balancingTransaction`

The helper expects the finalized intent set to equal that union. That matches `finalizeRecipe` for unbound and finalized recipes, which bind and then merge.

## Remaining limitations

Offline tests use pre-proof and pre-binding objects and synthetic UTXO references. They do not establish valid finalized bytes, proofs, binding, or ledger acceptance. Native `UnshieldedOffer.new` canonicalizes input and output order. The helper compares the stored SDK order. It still rejects swapped synthetic arrays because those arrays keep insertion order. Empty-input synthetic objects still fail `validateSignedRecipe` before the funding path. Helper rejection is not a submit exploit. Gross spend still scans only base segment 1 and still trusts `allowance.grossSpend`.

## Unchanged source pins

The six unowned parent files match freeze. Binding `sources` were not modified. Package and WASM pins in `providers.mjs` were not modified.

## Excluded findings

R1 admitted driver execution, R2 successful byte decoder, R3 receipt flags, R4 oracle projection, R5 provider SDK construction, R6 gross allowance and reservation, R7 deadlines, and R8 builder admission stay open. This source does not claim on-chain settlement, PCD, SP05 completion, or twelve-sprint completion.
