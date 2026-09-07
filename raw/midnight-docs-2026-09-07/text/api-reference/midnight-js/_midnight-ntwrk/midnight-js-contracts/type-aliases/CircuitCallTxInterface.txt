# CircuitCallTxInterface

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CircuitCallTxInterface

# Type Alias: CircuitCallTxInterface\<C>

> **CircuitCallTxInterface**<`C`> = `{ [PCK in Contract.ProvableCircuitId<C>]: { (args: CircuitParameters<C, PCK>): Promise<FinalizedCallTxData<C, PCK>>; (txCtx: TransactionContext<C, PCK>, args: CircuitParameters<C, PCK>): Promise<CallResult<C, PCK>> } }`

A type that lifts each circuit defined in a contract to a function that builds and submits a call transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
