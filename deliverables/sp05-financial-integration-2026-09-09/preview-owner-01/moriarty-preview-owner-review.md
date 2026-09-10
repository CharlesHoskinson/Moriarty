# Preview indexed-owner source slice

Isolated branch `feat/sp05-preview-owner` at `/home/charl/Moriarty/.worktrees/sp05-preview-owner`, base `68c6d9a79f8f520a2bfe54115d407b233cc1c184`. Only four ledger files changed, pinned in `/tmp/moriarty-preview-owner-source.json`. Uncommitted and not published; ready for independent GPT6/Grok audit.

`decodeLocalIndexedOwner` preserves its exact undeployed SDK decode/re-encode behavior. New `decodePreviewIndexedOwner` uses the same pinned SDK and exact roundtrip for Preview. `indexedOwnerDecoderForNetwork` accepts only undeployed (default) or preview. `observeFinalizedStage` gains optional `network`, validates before any observation, then applies the chosen closed decoder to both spent and created owners. No caller codec override, transport/launcher relaxation or receipt output-schema change.

This selects address encoding only. Chain/genesis, wallet, protocol and deployment identity remain caller integration obligations. Tests using local native fixtures with Preview indexer encodings are controlled boundary tests, not actual Preview transactions or acceptance.

Checks, from `experiments/moriarty-midnight-financial`:

```
node --experimental-test-module-mocks --test ledger/indexed-owner.test.mjs ledger/receipt.test.mjs ledger/receipt-native-balances.test.mjs ledger/integration-negative.test.mjs ledger/integrate-local.test.mjs ledger/launch-local.test.mjs
```

109 pass, zero failures/skips; `/tmp/moriarty-preview-owner-regression.tap`. `git diff --check` passed. Baseline 15/15; first RED 15 pass/4 expected failures. Initial GREEN attempt 18/19 exposed an invalid test assumption: the Bech32 encoder lowercases a supplied mixed-case HRP. Corrected the fixture to mutate the encoded string instead; no production workaround. Preserved all raw logs; focused GREEN 19/19.

Controls cover actual pinned SDK encode/decode, same bytes across two networks, cross-network/prefix/type/checksum/length/case/extra-HRP rejection, both receipt input/output owner paths, unequal native owner, unchanged default local rejection, and unsupported target failing before provider access. Existing native balances and callable financial negatives remain green. No wallet/private/service/network/prover/compiler/submission operations were performed; existing proof-free synthetic transaction fixtures do not invoke a prover body.
