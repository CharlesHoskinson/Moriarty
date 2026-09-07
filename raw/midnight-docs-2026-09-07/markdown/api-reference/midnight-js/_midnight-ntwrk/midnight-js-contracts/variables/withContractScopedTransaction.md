# withContractScopedTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / withContractScopedTransaction

# Variable: withContractScopedTransaction

> `const` **withContractScopedTransaction**: <`C`, `PCK`>(`providers`, `fn`, `options?`) => `Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

Executes a function within the context of a contract-scoped transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`> = `Contract.ProvableCircuitId`<`C`>

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`, `PCK`>

The contract providers to use within the transaction.

### fn[​](#fn "Direct link to fn")

(`txCtx`) => `Promise`<`void`>

The function to execute within the transaction context.

### options?[​](#options "Direct link to options?")

[`ScopedTransactionOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ScopedTransactionOptions.md)

Optional transaction scope options.

## Returns[​](#returns "Direct link to Returns")

`Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

A `Promise` that resolves with the finalized transaction data of the single transaction created for all circuit calls made within `fn`.

## Remarks[​](#remarks "Direct link to Remarks")

Where `fn` make circuit calls, these are batched together and submitted as a single transaction when the function completes successfully. If `fn` throws an error, any unsubmitted circuit calls are discarded.
