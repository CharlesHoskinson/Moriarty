# SubmittedCallTx

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / SubmittedCallTx

# Type Alias: SubmittedCallTx\<C, PCK>

> **SubmittedCallTx**<`C`, `PCK`> = `object`

Data returned from an asynchronous call transaction submission. Contains the transaction ID and call transaction data without waiting for finalization.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>

## Properties[​](#properties "Direct link to Properties")

### callTxData[​](#calltxdata "Direct link to callTxData")

> `readonly` **callTxData**: [`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>

The unproven call transaction data including private state.

***

### txId[​](#txid "Direct link to txId")

> `readonly` **txId**: `string`

The transaction ID returned from submission.
