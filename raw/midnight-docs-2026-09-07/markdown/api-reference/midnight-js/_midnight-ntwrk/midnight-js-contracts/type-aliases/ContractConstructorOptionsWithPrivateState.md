# ContractConstructorOptionsWithPrivateState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / ContractConstructorOptionsWithPrivateState

# Type Alias: ContractConstructorOptionsWithPrivateState\<C>

> **ContractConstructorOptionsWithPrivateState**<`C`> = [`ContractConstructorOptionsWithProviderDataDependencies`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithProviderDataDependencies.md)<`C`> & `object`

Conditional type that optionally adds the inferred circuit argument types to the target of a circuit invocation.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

> `readonly` **initialPrivateState**: `Contract.PrivateState`<`C`>

The private state to run the circuit against.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
