# submitCallTx

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / submitCallTx

# Function: submitCallTx()

Creates and submits a transaction for the invocation of a circuit on a given contract.

## Transaction Execution Phases[​](#transaction-execution-phases "Direct link to Transaction Execution Phases")

Midnight transactions execute in two phases:

1. **Guaranteed phase**: If failure occurs, the transaction is NOT included in the blockchain
2. **Fallible phase**: If failure occurs, the transaction IS recorded on-chain as a partial success

## Failure Behavior[​](#failure-behavior "Direct link to Failure Behavior")

**Guaranteed Phase Failure:**

* Transaction is rejected and not included in the blockchain
* `CallTxFailedError` is thrown with transaction data and circuit ID
* Private state updates are NOT stored (state remains unchanged)
* No on-chain record of the failed transaction

**Fallible Phase Failure:**

* Transaction is recorded on-chain with non-`SucceedEntirely` status
* `CallTxFailedError` is thrown with transaction data and circuit ID
* Private state updates are NOT stored (state remains unchanged)
* Transaction appears in blockchain history as partial success

## Param[​](#param "Direct link to Param")

The providers used to manage the invocation lifecycle.

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Param[​](#param-2 "Direct link to Param")

Optional scoped transaction context to participate in an existing transaction scope.

## Throws[​](#throws "Direct link to Throws")

When transaction fails in either guaranteed or fallible phase. The error contains the finalized transaction data and circuit ID for debugging.

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **submitCallTx**<`C`, `PCK`>(`providers`, `options`): `Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

#### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters "Direct link to Parameters")

#### providers[​](#providers "Direct link to providers")

[`SubmitTxProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmitTxProviders.md)<`C`, `PCK`>

#### options[​](#options "Direct link to options")

[`CallTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsBase.md)<`C`, `PCK`>

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **submitCallTx**<`C`, `PCK`>(`providers`, `options`): `Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

#### PCK[​](#pck-1 "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### providers[​](#providers-1 "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

#### options[​](#options-1 "Direct link to options")

[`CallTxOptionsWithPrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsWithPrivateStateId.md)<`C`, `PCK`>

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

## Call Signature[​](#call-signature-2 "Direct link to Call Signature")

> **submitCallTx**<`C`, `PCK`>(`providers`, `options`, `transactionContext`): `Promise`<[`CallResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResult.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters-2 "Direct link to Type Parameters")

#### C[​](#c-2 "Direct link to C")

`C` *extends* `Any`

#### PCK[​](#pck-2 "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters-2 "Direct link to Parameters")

#### providers[​](#providers-2 "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

#### options[​](#options-2 "Direct link to options")

[`CallTxOptionsWithPrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsWithPrivateStateId.md)<`C`, `PCK`>

#### transactionContext[​](#transactioncontext "Direct link to transactionContext")

[`TransactionContext`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/TransactionContext.md)<`C`, `PCK`>

### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`CallResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResult.md)<`C`, `PCK`>>

## Call Signature[​](#call-signature-3 "Direct link to Call Signature")

> **submitCallTx**<`C`, `PCK`>(`providers`, `options`, `transactionContext`): `Promise`<[`CallResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResult.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters-3 "Direct link to Type Parameters")

#### C[​](#c-3 "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

#### PCK[​](#pck-3 "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters-3 "Direct link to Parameters")

#### providers[​](#providers-3 "Direct link to providers")

[`SubmitTxProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmitTxProviders.md)<`C`, `PCK`>

#### options[​](#options-3 "Direct link to options")

[`CallTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsBase.md)<`C`, `PCK`>

#### transactionContext[​](#transactioncontext-1 "Direct link to transactionContext")

[`TransactionContext`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/TransactionContext.md)<`C`, `PCK`>

### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<[`CallResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallResult.md)<`C`, `PCK`>>
