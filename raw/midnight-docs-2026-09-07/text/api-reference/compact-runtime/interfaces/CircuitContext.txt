# CircuitContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CircuitContext

# Interface: CircuitContext\<PS>

The external information accessible from within a Compact circuit call

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS` = `any`

## Properties[​](#properties "Direct link to Properties")

### activeContracts?[​](#activecontracts "Direct link to activeContracts?")

```
optional activeContracts: Set<string>;
```

The set of contract addresses currently executing on the cross-contract call stack: the entry contract plus every callee whose call has not yet returned. Maintained by [crossContractCall](/api-reference/compact-runtime/functions/crossContractCall.md) and shared by reference across the call tree (via [copyCircuitContext](/api-reference/compact-runtime/functions/copyCircuitContext.md)). Only consulted when [reentrancyGuard](#reentrancyguard) is set.

***

### callContext[​](#callcontext "Direct link to callContext")

```
callContext: CallContext<PS>;
```

The context for the current call.

***

### callProofDataTrace[​](#callproofdatatrace "Direct link to callProofDataTrace")

```
callProofDataTrace: CallProofDataTrace;
```

Sequence of calls made during the execution of the circuit (including the call for the root circuit).

***

### contractStates?[​](#contractstates "Direct link to contractStates?")

```
optional contractStates: Record<string, ContractState>;
```

The deployed [ocrt.ContractState](/api-reference/compact-runtime/classes/ContractState.md) of every cross-contract callee resolved during the execution, keyed by address. Populated by [crossContractCall](/api-reference/compact-runtime/functions/crossContractCall.md) (via the state provider) the first time a callee is reached. Retained — unlike the cached query context, which keeps only ledger data — so the implementation-binding guard can read a callee's deployed verifier key for *any* of its circuits on *every* call, including later calls to a different circuit of an already-resolved callee. The entry contract is not recorded here; only fetched callees are.

***

### costModel[​](#costmodel "Direct link to costModel")

```
costModel: CostModel;
```

The cost model to use for the execution.

***

### events[​](#events "Direct link to events")

```
events: LogEvent[];
```

Events emitted by the on-chain VM during circuit execution from `log` operations, each tagged with the address of the emitting contract. A single global list shared across the whole call tree (threaded like [callProofDataTrace](#callproofdatatrace)); a per-contract view is a filter over the `address` tag. Surfaced via `CircuitResults.context.events`.

***

### gasCosts[​](#gascosts "Direct link to gasCosts")

```
gasCosts: Record<ContractAddress, RunningCost>;
```

The current gas costs for every contract in the call tree.

***

### gasLimit?[​](#gaslimit "Direct link to gasLimit?")

```
optional gasLimit: RunningCost;
```

The gas limit for this circuit.

***

### queryContexts[​](#querycontexts "Direct link to queryContexts")

```
queryContexts: Record<ContractAddress, QueryContext>;
```

The current query context of every contract in the call tree.

***

### reentrancyGuard?[​](#reentrancyguard "Direct link to reentrancyGuard?")

```
optional reentrancyGuard: boolean;
```

When `true`, [crossContractCall](/api-reference/compact-runtime/functions/crossContractCall.md) refuses to enter a contract that is already executing on the current call stack — i.e. a re-entrant cross-contract call (`A -> A`, or `A -> B -> A`) — and throws instead. On by default (the upstream ledger can mis-apply transcripts on re-entry). Pass `false` to [createCircuitContext](/api-reference/compact-runtime/functions/createCircuitContext.md) to opt out, e.g. for tests that deliberately exercise recursion.

***

### stateProvider?[​](#stateprovider "Direct link to stateProvider?")

```
optional stateProvider: ContractStateProvider;
```

Can fetch the current state of a contract from the blockchain.

***

### zswapLocalStates[​](#zswaplocalstates "Direct link to zswapLocalStates")

```
zswapLocalStates: Record<ContractAddress, EncodedZswapLocalState>;
```

The current Zswap local state of every contract in the call tree — the shielded-coin counterpart of [queryContexts](#querycontexts) and [gasCosts](#gascosts), and keyed the same way.

Each contract keeps its own state, with its own `currentIndex`, `inputs` and `outputs`; only the transaction submitter's `coinPublicKey` is shared, since one wallet pays for the whole transaction. Threaded across cross-contract calls (see `restoreCircuitContext`) so a callee's coin operations survive its return, and mirrored onto each [CallProofData](/api-reference/compact-runtime/interfaces/CallProofData.md) so transaction assembly can attribute every input and output to the contract that made it.
