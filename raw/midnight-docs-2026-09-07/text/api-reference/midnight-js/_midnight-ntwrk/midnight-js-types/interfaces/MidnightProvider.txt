# MidnightProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / MidnightProvider

# Interface: MidnightProvider

Interface for Midnight transaction submission logic. It could be implemented, e.g., by a wallet, a third-party service, or a node itself.

## Methods[​](#methods "Direct link to Methods")

### submitTx()[​](#submittx "Direct link to submitTx()")

> **submitTx**(`tx`): `Promise`<`string`>

Submit a transaction to the network to be consensed upon.

#### Parameters[​](#parameters "Direct link to Parameters")

##### tx[​](#tx "Direct link to tx")

`FinalizedTransaction`

The finalized transaction to submit.

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`string`>

The transaction identifier of the submitted transaction.
