# FinalizedCallTxData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / FinalizedCallTxData

# Type Alias: FinalizedCallTxData\<C, PCK>

> **FinalizedCallTxData**<`C`, `PCK`> = [`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`> & `object`

Data for a submitted, finalized call transaction.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### public[​](#public "Direct link to public")

> `readonly` **public**: `FinalizedTxData`

Public data relevant to this call transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>
