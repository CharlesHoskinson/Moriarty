# createUnprovenDeployTxFromVerifierKeys

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createUnprovenDeployTxFromVerifierKeys

# Function: createUnprovenDeployTxFromVerifierKeys()

Calls a contract constructor and creates an unbalanced, unproven, unsubmitted, deploy transaction from the constructor results.

## Param[​](#param "Direct link to Param")

The verifier keys for the contract being deployed.

## Param[​](#param-1 "Direct link to Param")

The Zswap coin public key of the current user.

## Param[​](#param-2 "Direct link to Param")

Configuration.

## Param[​](#param-3 "Direct link to Param")

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **createUnprovenDeployTxFromVerifierKeys**<`C`>(`zkConfigProvider`, `coinPublicKey`, `options`, `encryptionPublicKey`): `Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

### Parameters[​](#parameters "Direct link to Parameters")

#### zkConfigProvider[​](#zkconfigprovider "Direct link to zkConfigProvider")

[`ZKConfigProvider`](#)<`string`>

#### coinPublicKey[​](#coinpublickey "Direct link to coinPublicKey")

`string`

#### options[​](#options "Direct link to options")

[`DeployTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsBase.md)<`C`>

#### encryptionPublicKey[​](#encryptionpublickey "Direct link to encryptionPublicKey")

`string`

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **createUnprovenDeployTxFromVerifierKeys**<`C`>(`zkConfigProvider`, `coinPublicKey`, `options`, `encryptionPublicKey`): `Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### zkConfigProvider[​](#zkconfigprovider-1 "Direct link to zkConfigProvider")

[`ZKConfigProvider`](#)<`string`>

#### coinPublicKey[​](#coinpublickey-1 "Direct link to coinPublicKey")

`string`

#### options[​](#options-1 "Direct link to options")

[`DeployTxOptionsWithPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateState.md)<`C`>

#### encryptionPublicKey[​](#encryptionpublickey-1 "Direct link to encryptionPublicKey")

`string`

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`UnsubmittedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxData.md)<`C`>>
