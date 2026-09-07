# UnsubmittedCallTxData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / UnsubmittedCallTxData

# Type Alias: UnsubmittedCallTxData\<C, PCK>

> **UnsubmittedCallTxData**<`C`, `PCK`> = [`CallResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResult.md)<`C`, `PCK`> & `object`

Data for an unsubmitted call transaction.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### private[​](#private "Direct link to private")

> `readonly` **private**: [`UnsubmittedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedTxData.md)

Private data relevant to this call transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>
