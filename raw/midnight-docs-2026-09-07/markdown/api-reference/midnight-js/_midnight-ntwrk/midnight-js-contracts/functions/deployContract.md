# deployContract

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / deployContract

# Function: deployContract()

Creates and submits a contract deployment transaction. This function is the entry point for the transaction construction workflow and is used to create a [DeployedContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployedContract.md) instance.

## Param[​](#param "Direct link to Param")

The providers used to manage the transaction lifecycle.

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Throws[​](#throws "Direct link to Throws")

DeployTxFailedError If the transaction is submitted successfully but produces an error when executed by the node.

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **deployContract**<`C`>(`providers`, `options`): `Promise`<[`DeployedContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployedContract.md)<`C`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

### Parameters[​](#parameters "Direct link to Parameters")

#### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`, `ProvableCircuitId`<`C`>, `unknown`>

#### options[​](#options "Direct link to options")

[`DeployContractOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployContractOptionsBase.md)<`C`>

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`DeployedContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployedContract.md)<`C`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **deployContract**<`C`>(`providers`, `options`): `Promise`<[`DeployedContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployedContract.md)<`C`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### providers[​](#providers-1 "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

#### options[​](#options-1 "Direct link to options")

[`DeployContractOptionsWithPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployContractOptionsWithPrivateState.md)<`C`>

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`DeployedContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployedContract.md)<`C`>>
