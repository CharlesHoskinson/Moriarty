# submitTxAsync

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / submitTxAsync

# Function: submitTxAsync()

> **submitTxAsync**<`C`, `PCK`>(`providers`, `options`): `Promise`<`string`>

Proves, balances, and submits an unproven deployment or call transaction using the given providers, according to the given options. Unlike [submitTx](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitTx.md), this function returns immediately after submission without waiting for finalization.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

## Parameters[​](#parameters "Direct link to Parameters")

### providers[​](#providers "Direct link to providers")

[`SubmitTxProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmitTxProviders.md)<`C`, `PCK`>

The providers used to manage the transaction lifecycle.

### options[​](#options "Direct link to options")

[`SubmitTxOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/SubmitTxOptions.md)<`PCK`>

Configuration.

## Returns[​](#returns "Direct link to Returns")

`Promise`<`string`>

A promise that resolves with the transaction ID immediately after submission, or rejects if an error occurs during preparation or submission. To watch for finalization, use providers.publicDataProvider.watchForTxData(txId).
