# expectSuccessfulCallTx

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

> **expectSuccessfulCallTx**<`C`, `PCK`>(`providers`, `callTxData`, `callTxOptions?`, `nextPrivateState?`): `Promise`<`void`>

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

`MidnightProviders`<`ProvableCircuitId`<`C`>, `string`, `unknown`>

### callTxData[​](#calltxdata "Direct link to callTxData")

`FinalizedCallTxData`<`C`, `PCK`>

### callTxOptions?[​](#calltxoptions "Direct link to callTxOptions?")

`CallTxOptions`<`C`, `PCK`>

### nextPrivateState?[​](#nextprivatestate "Direct link to nextPrivateState?")

`PrivateState`<`C`>

## Returns[​](#returns "Direct link to Returns")

`Promise`<`void`>
