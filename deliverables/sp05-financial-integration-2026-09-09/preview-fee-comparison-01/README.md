# Admission-bound native fee comparison

MC02's partial-comparison scenario requires rejection of excess fees. The old complete comparator checked only equality between the receipt native debit and native transaction fee. `red.tap` records the concrete failure: four internally consistent 123 SPECK debits passed a supplied 491 SPECK ceiling. Original production bytes are retained beside the log. The RED test snapshot is reconstructed from the unchanged base test plus the original appended failing case; its intended original location is recorded in the candidate metadata.

The comparator now requires `dustFeeCap`, a finite uint128 bigint in native DUST/SPECK, and charges each accepted stage against its allocation. It rejects a single excessive debit or the stage crossing the cumulative ceiling with `NATIVE_FEE_CAP_EXCEEDED`. A failed comparison remains stopped. No source-fixture unlimited default remains. Exact equality passes. Outputs and durable result schemas are unchanged; driver and integration diagnostics retain the closed rejection code.

New local and Preview traces use their existing admitted `limits.dustFee`. Recovery and continuation additionally supply fixed, contiguous historical stage groups. Local integration checks the exact retained admission and proposal hashes before obtaining each original ceiling: local-execution-04 for loan deploy, local-recovery-03 for initialize, local-continuation-02 for historical accrue/settle, and local-swap-01 for swap deploy/initialize. Each original ceiling is 2e15 SPECK. Existing public history gates bind those stages to their exact transactions and allocations. Counters stay separate: unused old allowance does not increase the current allowance. This comparison covers the selected financial trace; it does not erase any previous reservation or count unrelated allocations as free.

`green-final-03.tap`: 172 targeted tests pass, zero failures/skips. `compiled.tap`: three retained generated-contract tests pass; no compilation was performed. `green-01.tap` is the first 126-test pass. `green-02.tap` retains one test-harness failure caused by expecting complete cleanup from an intentionally incomplete fixture; the assertion was corrected. `green-final.tap` retains two missing-module-mock-flag failures. `green-final-02.tap` passes 172 tests; final03 additionally verifies the closed driver rejection code.

`replay-01.tap` and the final suite verify actual retained Preview loan native bytes decode exactly to all four published receipts. Native fees total 1200000000000004 SPECK against its pinned plan ceiling of 2000000000000000. Public financial state/effects are retained unchanged; only capability commitments in reconstructed public summaries are replaced with synthetic test-secret commitments. No original secrets are read. This is a public financial replay, not renewed secret-custody, canonical-finality or proof acceptance. Indexer paid/estimated units remain unresolved and are never converted or used as the native ceiling.

Only source checks ran. No wallet, private store, prover, service, network call, transaction or allocation was invoked. Independent source review remains required; this change alone does not complete SP05.

Commands from the worktree root:

```sh
node --experimental-test-module-mocks --test --test-reporter=tap experiments/moriarty-midnight-financial/ledger/financial-comparison.test.mjs experiments/moriarty-midnight-financial/ledger/integration-negative.test.mjs experiments/moriarty-midnight-financial/ledger/preview-integration.test.mjs experiments/moriarty-midnight-financial/ledger/integrate-local.test.mjs experiments/moriarty-midnight-financial/ledger/initialized-swap-integration.test.mjs experiments/moriarty-midnight-financial/ledger/stale-loan-integration.test.mjs experiments/moriarty-midnight-financial/ledger/preview-swap-composition.test.mjs experiments/moriarty-midnight-financial/ledger/preview-fee-replay.test.mjs
MORIARTY_CUSTODY_ARTIFACTS=/home/charl/.local/state/moriarty/sp05-kernel-custody-20260908/build/main-integration npm --prefix experiments/moriarty-midnight-financial run test:compiled
git diff --check
```
