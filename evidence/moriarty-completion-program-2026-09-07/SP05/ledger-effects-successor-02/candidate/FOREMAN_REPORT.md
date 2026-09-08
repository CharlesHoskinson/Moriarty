# FOREMAN_REPORT

Status: source authored in this worktree. This is not ledger acceptance, PCD, SP05 completion, or an operational admission.

## Outcome

`recheckFinalizedAgainstRecipe` now inspects transaction-level fields as well as intents. It still compares guaranteed and fallible unshielded inputs, outputs, signatures, TTL, and same-phase `signatureData`. It also compares:

- `Transaction.guaranteedOffer` and segmented `fallibleOffer` by input nullifier and contract address, output commitment and contract address, transient commitment/nullifier/contract address, and bigint token deltas
- network identity from the pinned `toString` form `StandardTransaction { network_id: "..." }`
- intent `dustActions` spends, registrations, and `ctime` when present
- intent `actions` address, entry point, communication commitment, and transcript presence

It does not compare full pre-proof bytes to proven or bound bytes. It does not treat `signatureData` as a commitment to Zswap or network fields. `bind` changes serialize bytes and binding phase. Effect identities stay. `signatureData` is compared only when both intents expose the same binding instance. `Transaction.merge` remains the accepted disjoint-segment combination, including native Zswap offer union.

`fundUnshielded` computes gross debit from the verified signed recipe before `finalizeRecipe`. It sums every guaranteed and fallible unshielded payer input across every recipe part and segment. Refund and change outputs do not reduce that sum. A caller `allowance.grossSpend` cannot replace the computed amount. A caller `allowance.submission` of 0 cannot replace the actual submission 1. Reservation uses those computed values and keeps them on finalize failure.

This source does not mark all of R6 repaired. Shielded Zswap input and transient values are not exposed as spend amounts, so those families are rejected as unsupported accounting rather than netted through deltas. Dust spends are compared when present and rejected for gross accounting. Contract-call transcript internals and claim-rewards transactions are unsupported. No live prove, bind-from-facade, wallet, or submission ran.

## Owned files

| File | Bytes | SHA-256 |
|---|---|---|
| experiments/moriarty-midnight-financial/ledger/providers.mjs | 57066 | 6cb23d27c5a5b1213f6e8fe1c6099fd508462a3434d606538dd37cc023921598 |
| experiments/moriarty-midnight-financial/ledger/receipt.test.mjs | 60393 | 6d18925b45668d6965464f4e878a41a37509821047a58245e89564501d6bae2d |
| FOREMAN_REPORT.json | fourth owned file | hash after freeze |
| FOREMAN_REPORT.md | this file | hash after freeze |

Four-file total stays under 262144 with the six unchanged parent files. Unowned parent files keep freeze hashes.

## Commands

Reproduction before edit, live providers:

```
node /tmp/r6-reproduce-probes.mjs
```

Network change, added Zswap, and Zswap remove/amount/recipient all accepted with unchanged `signatureData`. Unchanged copies accepted.

```
node --test --test-reporter=tap --test-timeout=30000 /tmp/r6-reproduce-gross.test.mjs
```

Exit 1. Tests 3. Pass 1. Fail 2. Control reached `LOCAL_FINALIZE_BOUNDARY` with reservedGross 100. Cap 50 plus declared 0 reached that boundary. Base 100 plus balancing 200 with cap 200 also reached it.

First failing owned tests after the new controls, before the helper and gross change:

```
node --test --test-reporter=tap --test-timeout=90000 --test-name-pattern='native unchanged Zswap|native fundUnshielded reserves|malformed input amounts' experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 1. Tests 3. Pass 0. Fail 3. Network mutation missed the expected throw. Declared 0 with cap 50 reached finalize. Number-valued input 100 reached finalize.

Final owned suite:

```
node --test --test-reporter=tap --test-timeout=90000 experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 0. Tests 36. Pass 36. Fail 0.

Retained diagnostics after repair:

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-effects.test.mjs
```

Exit 0. Tests 7. Pass 7. Fail 0.

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-sdk-effects.test.mjs
```

Exit 0. Tests 3. Pass 3. Fail 0.

Adapted recipe-shape diagnostic, import only pointed at the live module: Tests 3. Pass 3. Fail 0.

Adapted predecessor probes, import only pointed at the live module: 38 cases. 5 accepted (unchanged native, both disjoint merge controls, unchanged guaranteed Zswap, unchanged fallible Zswap). 33 rejected. The nine previously accepted material Zswap and network alterations now reject. Predecessor result files were not overwritten.

Adapted `root-gross-admission.test.mjs`: Exit 0. Tests 3. Pass 3. Fail 0.

No Git writes. No network. No compiler spawn. No proof. No wallet restore. No `deploy.ts` import. Original diagnostic files were not edited.

## Supported production path

`fundUnshielded` validates the signed recipe, inspects gross debit, reserves computed gross and one submission, calls `finalizeRecipe`, then calls this helper before submit.

Recipe shapes:

- `UNBOUND_TRANSACTION`: `baseTransaction` plus optional `balancingTransaction`. Facade binds the base and merges finalized balancing.
- `UNPROVEN_TRANSACTION`: `transaction`. Facade proves then binds that one transaction.
- `FINALIZED_TRANSACTION`: `originalTransaction` plus `balancingTransaction`. Facade merges original with finalized balancing.

The helper expects the finalized intent set and transaction-level Zswap offers to equal that union. Network identity must match the recipe. Bind and merge are modeled from the pinned Wallet facade and ledger-v8 contracts. Prove is not executed here.

## Remaining limitations

Offline tests use pre-proof and pre-binding objects, plus native `bind` and `merge`, with synthetic UTXO references. They do not establish valid finalized bytes, proofs, or ledger acceptance. `mockProve` changes `signatureData` and Zswap proof bytes while commitments stay. That is why the helper does not require `signatureData` across binding phases and does not compare proofs or whole serialize buffers.

Zswap inputs and transients have no exposed spend value on the pinned `ZswapInput` / `ZswapTransient` objects. Gross accounting rejects those families instead of using net deltas. Dust spends are the same. Mixed assets are rejected unless a single permitted `tokenType` is supplied. Empty-input synthetic objects still fail `validateSignedRecipe` before funding. Helper rejection is not a submit exploit.

Zswap inputs, transients, dust registration signatures, rewards, and nonempty contract actions were not given native mutation probes in this pass. Those fields are compared or rejected as unsupported when present. That is not a completeness proof for untested families.

## Unchanged source pins

The six unowned parent files match freeze. Binding `sources` were not modified. Package and WASM pins in `providers.mjs` were not modified.

## Excluded findings

R1 admitted driver execution, R2 successful byte decoder, R3 receipt flags, R4 oracle projection, R5 provider SDK construction, R7 deadlines, and R8 builder admission stay open. R6 still has unsupported shielded-input and dust-spend accounting, and untested contract-action families. This source does not claim on-chain settlement, PCD, SP05 completion, or twelve-sprint completion.
