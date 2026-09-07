# submitDeployTx

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / submitDeployTx

# Function: submitDeployTx()

Creates and submits a deploy transaction for the given contract.

## Transaction Execution Phases[​](#transaction-execution-phases "Direct link to Transaction Execution Phases")

Midnight transactions execute in two phases:

1. **Guaranteed phase**: If failure occurs, the transaction is NOT included in the blockchain
2. **Fallible phase**: If failure occurs, the transaction IS recorded on-chain as a partial success

## Failure Behavior[​](#failure-behavior "Direct link to Failure Behavior")

**Guaranteed Phase Failure:**

* Transaction is rejected and not included in the blockchain
* `DeployTxFailedError` is thrown with transaction data
* Private state (if `privateStateId` provided) is NOT stored
* Contract signing key is NOT stored in private state provider
* Contract is NOT deployed

**Fallible Phase Failure:**

* Transaction is recorded on-chain with non-`SucceedEntirely` status
* `DeployTxFailedError` is thrown with transaction data
* Private state (if `privateStateId` provided) is NOT stored
* Contract signing key is NOT stored in private state provider
* Transaction appears in blockchain history as partial success
* Contract may be partially deployed but not functional

## Param[​](#param "Direct link to Param")

The providers used to manage the deploy lifecycle.

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Throws[​](#throws "Direct link to Throws")

When transaction fails in either guaranteed or fallible phase. The error contains the finalized transaction data for debugging.

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **submitDeployTx**<`C`>(`providers`, `options`): `Promise`<[`FinalizedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxData.md)<`C`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

### Parameters[​](#parameters "Direct link to Parameters")

#### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`, `ProvableCircuitId`<`C`>, `unknown`>

#### options[​](#options "Direct link to options")

[`DeployTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsBase.md)<`C`>

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`FinalizedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxData.md)<`C`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **submitDeployTx**<`C`>(`providers`, `options`): `Promise`<[`FinalizedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxData.md)<`C`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### providers[​](#providers-1 "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

#### options[​](#options-1 "Direct link to options")

[`DeployTxOptionsWithPrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateStateId.md)<`C`>

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`FinalizedDeployTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedDeployTxData.md)<`C`>>
