# CallOptionsWithArguments

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallOptionsWithArguments

# Type Alias: CallOptionsWithArguments\<C, PCK>

> **CallOptionsWithArguments**<`C`, `PCK`> = `Contract.CircuitParameters`<`C`, `PCK`> *extends* \[] ? [`CallOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsBase.md)<`C`, `PCK`> : [`CallOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsBase.md)<`C`, `PCK`> & `object`

Conditional type that optionally adds the inferred circuit argument types to the options for a circuit call.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>
