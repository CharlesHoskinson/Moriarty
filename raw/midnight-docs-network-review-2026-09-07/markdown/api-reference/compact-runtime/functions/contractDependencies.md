# contractDependencies

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / contractDependencies

# Function: contractDependencies()

```
function contractDependencies(contractReferenceLocations, state): string[];
```

// TODO: Remove compiler support for contract dependencies once CCCs land.

Given a [StateValue](/api-reference/compact-runtime/classes/StateValue.md) representing the current ledger state of a contract, uses the [ContractReferenceLocations](/api-reference/compact-runtime/type-aliases/ContractReferenceLocations.md) object produced by the Compact compiler to extract the current contract addresses present in the given ledger state. The produced contract addresses represent the contracts on which the root contract depends. The dependencies are used in a multi-contract setting to fetch the ledger states of all contracts on which the root contract depends prior to execution.

NOTE: The given [ContractReferenceLocations](/api-reference/compact-runtime/type-aliases/ContractReferenceLocations.md) must be from the contract executable containing the ledger state constructor that produced the given [StateValue](/api-reference/compact-runtime/classes/StateValue.md).

## Parameters[​](#parameters "Direct link to Parameters")

### contractReferenceLocations[​](#contractreferencelocations "Direct link to contractReferenceLocations")

[`ContractReferenceLocations`](/api-reference/compact-runtime/type-aliases/ContractReferenceLocations.md)

A data structure pointing to contract references in the ledger state of the root contract.

### state[​](#state "Direct link to state")

[`StateValue`](/api-reference/compact-runtime/classes/StateValue.md)

The current ledger state of the root contract.

## Returns[​](#returns "Direct link to Returns")

`string`\[]

A list of all contract addresses (references) present in the given ledger state.

## Remarks[​](#remarks "Direct link to Remarks")

The algorithm has three main stages:

1. It unwraps the [PublicLedgerSegments](/api-reference/compact-runtime/type-aliases/PublicLedgerSegments.md) in the given [ContractReferenceLocations](/api-reference/compact-runtime/type-aliases/ContractReferenceLocations.md) until a [SparseCompactADT](/api-reference/compact-runtime/type-aliases/SparseCompactADT.md) is reached. Each time a [PublicLedgerSegments](/api-reference/compact-runtime/type-aliases/PublicLedgerSegments.md) is unwrapped, it casts the current state value to a state value array and proceeds recursively with each of the state values and unwrapped ledger segments.
2. It unwraps each [SparseCompactADT](/api-reference/compact-runtime/type-aliases/SparseCompactADT.md) in the current [PublicLedgerSegments](/api-reference/compact-runtime/type-aliases/PublicLedgerSegments.md) until a [SparseCompactType](/api-reference/compact-runtime/type-aliases/SparseCompactType.md) is reached. Each time a [SparseCompactADT](/api-reference/compact-runtime/type-aliases/SparseCompactADT.md) is unwrapped, it casts the current state value to a state representation indicated by the [SparseCompactADT](/api-reference/compact-runtime/type-aliases/SparseCompactADT.md).
3. Once the current state can no longer be reduced, it must represent a Compact contract address somewhere inside the state, and that contract address is added to the dependency set.
