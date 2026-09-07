# createUnprovenCallTxFromInitialStates

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createUnprovenCallTxFromInitialStates

# Function: createUnprovenCallTxFromInitialStates()

Calls a circuit using the provided initial `states` and creates an unbalanced, unproven, unsubmitted, call transaction.

## Param[​](#param "Direct link to Param")

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Param[​](#param-2 "Direct link to Param")

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **createUnprovenCallTxFromInitialStates**<`C`, `PCK`>(`zkConfigProvider`, `options`, `walletEncryptionPublicKey`): `Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

#### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters "Direct link to Parameters")

#### zkConfigProvider[​](#zkconfigprovider "Direct link to zkConfigProvider")

[`ZKConfigProvider`](#)<`string`>

#### options[​](#options "Direct link to options")

[`CallOptionsWithProviderDataDependencies`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsWithProviderDataDependencies.md)<`C`, `PCK`>

#### walletEncryptionPublicKey[​](#walletencryptionpublickey "Direct link to walletEncryptionPublicKey")

`string`

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **createUnprovenCallTxFromInitialStates**<`C`, `PCK`>(`zkConfigProvider`, `options`, `walletEncryptionPublicKey`): `Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

#### PCK[​](#pck-1 "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### zkConfigProvider[​](#zkconfigprovider-1 "Direct link to zkConfigProvider")

[`ZKConfigProvider`](#)<`string`>

#### options[​](#options-1 "Direct link to options")

[`CallOptionsWithPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsWithPrivateState.md)<`C`, `PCK`>

#### walletEncryptionPublicKey[​](#walletencryptionpublickey-1 "Direct link to walletEncryptionPublicKey")

`string`

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>
