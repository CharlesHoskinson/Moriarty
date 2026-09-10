# Fixed financial ledger integration

These modules implement the local loan and swap integration path for SP05. The [reviewed local loan trace](../../../deliverables/sp05-financial-integration-2026-09-09/local-continuation-02/attempt-result.json) completed deploy, initialize, accrue and a partial payment. Principal remains outstanding. The [first swap attempt](../../../deliverables/sp05-financial-integration-2026-09-09/local-swap-01/REVIEWED-RESULT.md) deployed and initialized, then stopped at the initialize comparison; it did not swap or close. Preview financial settlement, actual failure rollback and mandatory PCD evidence remain required.

`integrate-local.mjs` composes the pinned SDK, actual proof-asset loader, deployment preparation, durable provider reservations, native observation and full financial comparator. It takes an existing wallet handle, explicit local endpoints, public participant addresses, private capability/signing handles, exact build receipt bindings, explicit protocol version, spending limits and durable stage/event callbacks. It creates no wallet and offers no public-network CLI.

Preparation runs the constructor once, fixing the native contract address before logical asset limits are converted to exact native colors. The same prepared transaction is submitted once. A new preparation or provider instance cannot erase the existing allocation's charges or unresolved operations.

`receipt.mjs` decodes actual native bytes and checks indexed outputs, contract action, canonical finalized block, block-pinned state and balances. Native unshielded reserves come from the SDK provider’s exact `ContractState` object used for state decoding. Its runtime class differs from the native transaction decoder’s class. The separate indexed balance projection was empty for the retained initialized swap, despite native reserves of 1,000,000 A and 2,000,000 B; the [reviewed correction](../../../deliverables/sp05-financial-integration-2026-09-09/native-contract-balances-01/REVIEWED-13.md) preserves that discrepancy and its tests. It does not verify proofs. Native DUST debit remains separate from indexer fee strings; the retained historical fixture explains the observed difference.

`financial-comparison.mjs` checks the complete generated public state, both cumulative result slots, native effects, participant net changes, gross inputs, contract reserves and conservation. Its snapshots preserve actual public values. It requires explicit protocol version; ledger package version 8.1.0 does not mean protocol version 8. Original expectations and the observed mint-to-self mapping correction are retained in the SP05 deliverable.

The local driver requires each comparison to pass and each stage to be retained before proceeding. Current pinned SDK cleanup does not expose full indexer disposal. The real driver therefore reports incomplete containment until a separately reviewed outer process launch proves termination. A successful source test or stage callback is not a network acceptance receipt.

## Offline checks

From the repository root:

```sh
npm --prefix experiments/moriarty-midnight-financial test
npm --prefix experiments/moriarty-midnight-financial run test:ledger
MORIARTY_CUSTODY_ARTIFACTS=/absolute/path/to/retained-custody-build \
  npm --prefix experiments/moriarty-midnight-financial run test:compiled
```

`test:ledger` includes the native balance, actual SDK provider boundary, recovery and continuation regressions. It does not build contracts or start a ledger.

The compiled comparison requires retained generated artifacts and verifies their pins. It runs real generated contract code with explicitly simulated ledger context and transport; it does not compile or submit a transaction. Source tests use inert adapters whose results cannot establish proof or network acceptance.

## Full build and execution gates

`npm --prefix experiments/moriarty-midnight-financial run build -- --request /absolute/admitted-request.json` invokes the real builder. It requires current source/review/resource bindings, a fresh output directory and an exclusive persistent attempt file. A complete compiler run must produce all prover/verifier and ZK IR artifacts. Synthetic adapters and skip-zk artifacts cannot qualify.

Do not run the compiler or financial integration from an invented request. Actual dispatch requires the existing Moriarty admission workflow, both required independent reviews, enforced resource limits and preserved earlier charges. Docker loan and swap settlement comes first; Preview is a separately admitted campaign, with every real transaction ID reported publicly.
