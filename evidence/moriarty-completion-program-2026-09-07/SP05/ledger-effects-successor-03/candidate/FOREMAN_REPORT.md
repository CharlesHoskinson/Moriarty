# FOREMAN_REPORT

Status: source authored in this worktree for successor-effects-03. This is not ledger acceptance, PCD, SP05 completion, or an operational admission.

Predecessor `c6249e14c06907946ff47f0815b1540abf4bb65409a234ce3a94ddb611a87f52` remains frozen. Its stopped ledger, failed receipt, and GPT-6 REJECT stay intact. This pass is a new charged author run. It is not a retry or refund.

## Outcome

`recheckFinalizedAgainstRecipe` now compares a complete native representation. When both the signed recipe and the finalized object are pinned `Transaction` values, the helper merges recipe parts with `Transaction.merge` and compares `eraseProofs().serialize()` bytes. Binding and proofs are the only permitted differences. Transcript mint or gas, maintenance authority or counter, Zswap, network, signatures, and dust registrations stay in that byte form.

Unchanged native maintenance, contract-call, Zswap, and disjoint merge controls still accept after bind. The four predecessor bind mutations now reject in the same phase and across bind. Diagnostic Maps keep the older field helper. They do not authorize the native funding path.

`inspectSignedRecipeGross` sums guaranteed and fallible unshielded inputs across every recipe part and segment. Refund outputs do not reduce gross. `DustSpend.vFee` is a separate DUST duty. It is never added to USD_micro or asset A/B. `DustRegistration.allowFeePayment` is a fee authorization. The helper constrains it against an explicit DUST cap and does not treat it as a paid debit. The pinned `DustRegistration.signature` getter returns undefined, so native registration signatures are unreadable. Structural fixtures with a readable signature are verified against `intent.signatureData`. Shielded Zswap inputs and transients stay unsupported because spend values are not exposed.

`reserveBudget` checks every counter, cap, and delta as a nonnegative bigint before any update. Missing `grossSpend` and negative `reservedGross` now fail closed. DUST duties require explicit `dustFee` and `reservedDustFee` fields. Zero DUST duty does not invent unbounded fee capacity.

## Owned files

| File | Bytes | SHA-256 |
|---|---|---|
| experiments/moriarty-midnight-financial/ledger/providers.mjs | 62857 | 200b7ecebc56eeeba98c9d3f1d2b10f9dd2db0741a9aa3f649531aa250105e6a |
| experiments/moriarty-midnight-financial/ledger/receipt.test.mjs | 71636 | 92d89ce4621abd57d63f97660c1952a348d55206ff57748c2be6a1a12a540073 |
| FOREMAN_REPORT.json | fourth owned file | hash after freeze |
| FOREMAN_REPORT.md | this file | hash after freeze |

Four-file total stays under 262144. Unowned parent files keep freeze hashes. Package and WASM pins were not modified.

## Commands

Reproduction before edit, live providers:

```
node /tmp/r6-reproduce-predecessor-14.mjs
```

Fourteen predecessor cases reproduced. Four altered native actions rejected in the same phase and accepted across bind. Negative `reservedGross=-1000` with cap 50 reached inert finalize. Missing `grossSpend` reached inert finalize. Native dust registration with `allowFeePayment=900` counted only the 100-unit unshielded input.

First failing owned tests after the new controls, before the helper change:

```
node --test --test-reporter=tap --test-timeout=90000 --test-name-pattern='native contract and maintenance|malformed counters|structural dust fee|native dust registration' experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 1. Tests 4. Pass 0. Fail 4. Bind mutations were accepted. Negative reservation reached finalize. Dust spends threw `unsupported accounting: dust spends`. Registration inspect omitted `dustFee`.

Final owned suite:

```
node --test --test-reporter=tap --test-timeout=90000 experiments/moriarty-midnight-financial/ledger/receipt.test.mjs
```

Exit 0. Tests 40. Pass 40. Fail 0.

Retained diagnostics after repair, import only pointed at the live module:

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-effects.test.mjs
```

Exit 0. Tests 7. Pass 7. Fail 0.

```
node --test --test-reporter=tap --test-timeout=30000 /home/charl/.local/state/moriarty/sp05-ledger-integration-20260908/root-sdk-effects.test.mjs
```

Exit 0. Tests 3. Pass 3. Fail 0.

Adapted `root-sdk-recipe-shape.test.mjs`: Exit 0. Tests 3. Pass 3. Fail 0.

Adapted `root-gross-admission.test.mjs`: Exit 0. Tests 3. Pass 3. Fail 0.

Adapted 38-case predecessor probes: 38 cases. 5 accepted. 33 rejected. 0 construction errors. Predecessor result files were not overwritten.

New independent 14-case probes, live module, output under `/tmp/r6-adapted/`:

- Unchanged maintenance and contract-call bind accepted. `eraseProofs` bytes matched.
- Counter, authority, mint, and gas mutations rejected in the same phase and across bind.
- Positive funding reserved gross 100 and one submission at the inert finalize boundary.
- Zero submission rejected with no reservation.
- Negative `reservedGross` and missing `grossSpend` rejected as malformed budget. No finalize call.
- Boolean allowance rejected as malformed budget.
- Dust registration inspect returned gross 100, dustFee 0, feeAuthorization 900, then rejected `dust fee allowance missing`.
- Two-part unshielded gross stayed 300.
- Unchanged disjoint merge bind kept equal `eraseProofs` bytes.

No Git writes. No network. No compiler spawn. No proof. No wallet restore. No `deploy.ts` import. Original diagnostic files were not edited.

## Supported production path

`fundUnshielded` validates the signed recipe, inspects asset gross and DUST duties, constrains fee authorization, reserves computed gross plus actual DUST fee plus one submission, calls `finalizeRecipe`, then calls this helper before submit.

Recipe shapes:

- `UNBOUND_TRANSACTION`: `baseTransaction` plus optional `balancingTransaction`. Facade binds the base and merges finalized balancing.
- `UNPROVEN_TRANSACTION`: `transaction`. Facade proves then binds that one transaction.
- `FINALIZED_TRANSACTION`: `originalTransaction` plus `balancingTransaction`. Facade merges original with finalized balancing.

Native comparison models bind and merge from those contracts by erasing proofs after merge. Prove is not executed here.

## Remaining limitations

Offline tests use pre-proof and pre-binding objects, plus native `bind`, `merge`, and `eraseProofs`, with synthetic UTXO references. They do not establish valid finalized bytes, proofs, or ledger acceptance.

Zswap inputs and transients have no exposed spend value on the pinned objects. Gross accounting rejects those families instead of using net deltas. Mixed non-DUST assets still reject unless a single permitted `tokenType` is supplied.

Native `DustSpend` construction needs a backing Night UTXO in `DustLocalState`. That fixture was not built. Fee-spend tests use a labeled structural `vFee` object. Native `DustActions` registrations are real SDK objects. The pinned registration signature getter is undefined, so this source does not claim a verified native registration signature.

Genuine proof-generation compatibility is untested. Only proof erasure and bind were exercised.

## Unchanged source pins

The six unowned parent files match freeze. Binding `sources` were not modified. Package and WASM pins in `providers.mjs` were not modified.

## Excluded findings

R1 admitted driver execution, R2 successful byte decoder, R3 receipt flags, R4 oracle projection, R5 provider SDK construction, R7 deadlines, and R8 builder admission stay open. R6 still has unsupported shielded-input and transient accounting, untested proof generation, and no Preview settlement. This source does not claim on-chain settlement, PCD, SP05 completion, or twelve-sprint completion.
