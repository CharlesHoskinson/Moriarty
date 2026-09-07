# ContractConstructorResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / ContractConstructorResult

# Type Alias: ContractConstructorResult\<C>

> **ContractConstructorResult**<`C`> = `object`

The updated states resulting from executing a contract constructor.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

## Properties[​](#properties "Direct link to Properties")

### nextContractState[​](#nextcontractstate "Direct link to nextContractState")

> `readonly` **nextContractState**: `ContractState`

The public state resulting from executing the contract constructor.

***

### nextPrivateState[​](#nextprivatestate "Direct link to nextPrivateState")

> `readonly` **nextPrivateState**: `Contract.PrivateState`<`C`>

The private state resulting from executing the contract constructor.

***

### nextZswapLocalState[​](#nextzswaplocalstate "Direct link to nextZswapLocalState")

> `readonly` **nextZswapLocalState**: `ZswapLocalState`

The Zswap local state resulting from executing the contract constructor.
