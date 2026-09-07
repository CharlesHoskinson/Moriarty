# expectFoundAndDeployedStatesEqual

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

> **expectFoundAndDeployedStatesEqual**<`C`>(`providers`, `deployTxData`, `foundDeployTxData`, `privateStateId?`, `initialPrivateState?`): `Promise`<`void`>

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

`MidnightProviders`<`ProvableCircuitId`<`C`>, `string`, `unknown`>

### deployTxData[​](#deploytxdata "Direct link to deployTxData")

`FinalizedDeployTxData`<`C`>

### foundDeployTxData[​](#founddeploytxdata "Direct link to foundDeployTxData")

`FinalizedDeployTxDataBase`<`C`>

### privateStateId?[​](#privatestateid "Direct link to privateStateId?")

`string`

### initialPrivateState?[​](#initialprivatestate "Direct link to initialPrivateState?")

`PrivateState`<`C`>

## Returns[​](#returns "Direct link to Returns")

`Promise`<`void`>
