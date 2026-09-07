# submitCallTxAsync

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / submitCallTxAsync

# Function: submitCallTxAsync()

> **submitCallTxAsync**<`C`, `PCK`>(`providers`, `options`): `Promise`<[`SubmittedCallTx`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmittedCallTx.md)<`C`, `PCK`>>

Creates and submits a transaction for the invocation of a circuit on a given contract, returning immediately after submission without waiting for finalization.

Unlike [submitCallTx](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitCallTx.md), this function does not wait for transaction finalization, check transaction status, or update private state. The caller must handle these steps manually.

## Transaction Execution Phases[​](#transaction-execution-phases "Direct link to Transaction Execution Phases")

Midnight transactions execute in two phases:

1. **Guaranteed phase**: If failure occurs, the transaction is NOT included in the blockchain
2. **Fallible phase**: If failure occurs, the transaction IS recorded on-chain as a partial success

## Manual Post-Submission Steps[​](#manual-post-submission-steps "Direct link to Manual Post-Submission Steps")

After calling this function, you must manually:

1. Watch for transaction finalization using `providers.publicDataProvider.watchForTxData(txId)`
2. Check transaction status (compare against `SucceedEntirely`)
3. Handle failures appropriately (throw errors, log, etc.)
4. Update private state if transaction succeeded and `privateStateId` was provided

## Failure Behavior (Manual Handling Required)[​](#failure-behavior-manual-handling-required "Direct link to Failure Behavior (Manual Handling Required)")

**Guaranteed Phase Failure:**

* Transaction is rejected and not included in the blockchain
* `watchForTxData` may reject or return error status
* You must NOT store private state updates

**Fallible Phase Failure:**

* Transaction is recorded on-chain with non-`SucceedEntirely` status
* `watchForTxData` returns transaction data with failed status
* You must NOT store private state updates
* Transaction appears in blockchain history as partial success

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

`SubmitCallTxProviders`<`C`, `PCK`>

The providers used to manage the invocation lifecycle.

### options[​](#options "Direct link to options")

[`CallTxOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptions.md)<`C`, `PCK`>

Configuration.

## Returns[​](#returns "Direct link to Returns")

`Promise`<[`SubmittedCallTx`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmittedCallTx.md)<`C`, `PCK`>>

A `Promise` that resolves with the transaction ID and call transaction data immediately after submission; or rejects with an error if the submission fails.

## Example[​](#example "Direct link to Example")

```
// 1. Submit

const { txId, callTxData } = await submitCallTxAsync(providers, options);



// 2. Watch (when ready)

const finalizedData = await providers.publicDataProvider.watchForTxData(txId);



// 3. Check status

if (finalizedData.status !== SucceedEntirely) {

  throw new CallTxFailedError(finalizedData, options.circuitId);

}



// 4. Update private state manually if needed

if (options.privateStateId) {

  await providers.privateStateProvider.set(

    privateStateId,

    callTxData.private.nextPrivateState

  );

}
```
