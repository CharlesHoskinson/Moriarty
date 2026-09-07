# createUnprovenDeployTx

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createUnprovenDeployTx

# Function: createUnprovenDeployTx()

Calls a contract constructor and creates an unbalanced, unproven, unsubmitted, deploy transaction from the constructor results.

## Param[​](#param "Direct link to Param")

The providers to use to create the deploy transaction.

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **createUnprovenDeployTx**<`C`>(`providers`, `options`): `Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

### Parameters[​](#parameters "Direct link to Parameters")

#### providers[​](#providers "Direct link to providers")

[`UnprovenDeployTxProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenDeployTxProviders.md)<`C`>

#### options[​](#options "Direct link to options")

[`DeployTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsBase.md)<`C`>

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **createUnprovenDeployTx**<`C`>(`providers`, `options`): `Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### providers[​](#providers-1 "Direct link to providers")

[`UnprovenDeployTxProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenDeployTxProviders.md)<`C`>

#### options[​](#options-1 "Direct link to options")

[`DeployTxOptionsWithPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateState.md)<`C`>

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>
