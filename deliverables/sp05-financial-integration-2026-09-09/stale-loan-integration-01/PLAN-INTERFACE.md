# Fixed stale-loan plan interface — source only

Two new uncommitted files in `/home/charl/Moriarty/.worktrees/sp05-deadline-review/experiments/moriarty-midnight-financial/ledger/`: `stale-loan-plan.mjs`, `stale-loan-plan.test.mjs`. No existing file changed. Apply/copy to the integration worktree only as this source slice; no operational plan or admission was created.

Exports:

- `STALE_LOAN`: recursively frozen fixed public descriptor fields (38 fields). To select the mode, construct `p.existingStaleLoan = {...STALE_LOAN, snapshotDirectory, inspectionDirectory}`. These are path metadata only; no private files are read.
- `validateStaleLoanPlan(d,p)`: closed descriptor and closed entire local-launch plan validation; returns a structured clone of `d`. Reads no filesystem. `schema: moriarty.local-financial-launch/1`, `kind: loan`, **existingStaleLoan** only. Descriptor schema `moriarty.existing-stale-local-loan/1`; old existingDeployment/existingInitializedLoan/existingInitializedSwap and staleSettledLoan names reject.
- `readStaleLoanInputs(d,ledger)`: bounded/symlink-rejecting reads of hard-pinned PUBLIC paths only. Caller should first validate entire plan. Returns `history`, `deployBinding`, `initializeBinding`, `initializedStateRaw`, `sources`, `settledSource`, `freshCurrentStateEstablished:false`, `executionAuthorized:false`, `scope`.

`history` has exactly four ordered entries `{stage, raw, transaction, txId}` for deploy, initialize, accrue, settle. `raw` is a Buffer; `transaction` is the actual native receipt decoder projection (including signatures checked, inputs, outputs, action/effects, native fee); `txId` is the submitted historical ID. `deployBinding` and `initializeBinding` are unchanged results from existing inspectExistingLoanBytes/inspectInitializedLoanBytes utilities.

`sources` is parsed, exactly SHA-pinned public JSON under keys deployResult, initializeResult, continuationResult, continuationIntegration, continuationReviewedResult, settledProbe. Full prior comparison/receipts are at `sources.continuationIntegration`. Native historical DUST nullifiers require deserializing each history.raw with caller's pinned ledger, as receipt projection deliberately carries fee debit but not all nullifiers. No historical/current wallet-spent predicates are asserted by this reader.

`settledSource` is the first retained snapshot from local-finalized-state-02, SHA `552d58ff2918332665b179b37a70907c4492a02b5c56d7db05dce5e427e573e0`, source block height20415. It retains serialized state, explicit zero USD balance, remaining0/revision2. It is **historical reference**, never current readiness. Fresh canonical state/history, actor/capability comparison, original-store preservation, spent-input/DUST wallet synchronization and caller deadline remain preflight/integration obligations. InitializedStateRaw has SHA1f0724d2… and is distinct.

Strict limits: submissions exactly1; grossByLogicalAsset exactly `{USD_TEST_ASSET:'0'}`; positive canonical string DUST <=1e15 SPECK; absolute deadline <=20minutes from validation. Allocation ID must use new family `sp05-stale-loan-` plus1–60 alphanumeric/underscore/hyphen chars, excluding every old loan/swap allocation family. Three destinations must be absolute canonical disjoint paths and outside original wallet/store/build/public-evidence roots and historical `sp05-local-loan-*`, `sp05-local-swap-*`, `sp05-local-finalized-state-*` directories. Actual uniqueness/symlink/exclusive-create checks for output/snapshot/inspection and reservation must occur in existing runtime ownership code; pure validation cannot establish filesystem freshness. Original fixed local endpoints are19944/18088api-v4/16300. No reservationStatePath override is accepted in plan limits; runtime derives it under exclusive output.

RED:10tests failed because required exports were missing. GREEN:10tests pass,0failed/skipped,276.924751ms. Tests cover all fixedfield mutation, all identityfields, unsupportedmode/extra fields, badcaps/oldallocations, pathoverlap, wrongnetwork, nestedgetter/symbol/array properties, private-redirection refusal, and allfour real publicnative histories+settledsource reading. No private files, wallet, service, proof or network operation executed. Outputs `/tmp/moriarty-stale-loan-plan-red.tap` and `...-green.tap`.

This is a fixed plan/public-input component, not the completed operational feature, source approval or live result. Both current source reviewers and real launcher integration remain required.

Source hashes:
- `stale-loan-plan.mjs`: `9cfc0d19a24869311c936050bbaf0838fd719bf388a2a955a214afe1f7418e08`
- `stale-loan-plan.test.mjs`: `3a60376a5c9533c8083f773ae90e0370471d5be7c4413a96003dcfb8447b44aa`
