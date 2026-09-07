# SubmitTxOptions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / SubmitTxOptions

# Type Alias: SubmitTxOptions\<PCK>

> **SubmitTxOptions**<`PCK`> = `object`

Configuration for [submitTx](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitTx.md).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `AnyProvableCircuitId`

## Properties[​](#properties "Direct link to Properties")

### circuitId?[​](#circuitid "Direct link to circuitId?")

> `readonly` `optional` **circuitId?**: `PCK` | `PCK`\[]

A circuit identifier to use to fetch the ZK artifacts needed to prove the transaction. Only defined if a call transaction is being submitted.

#### Remarks[​](#remarks "Direct link to Remarks")

Where a transaction involves multiple circuits (e.g., when circuit calls are scoped to a transaction context), this may be an array of circuit IDs.

***

### unprovenTx[​](#unproventx "Direct link to unprovenTx")

> `readonly` **unprovenTx**: `UnprovenTransaction`

The transaction to prove, balance, and submit.
