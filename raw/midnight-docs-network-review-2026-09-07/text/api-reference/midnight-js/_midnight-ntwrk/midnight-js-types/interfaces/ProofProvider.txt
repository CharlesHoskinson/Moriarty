# ProofProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ProofProvider

# Interface: ProofProvider

Interface for a proof server running in a trusted environment.

## Type Param[​](#type-param "Direct link to Type Param")

The type of the circuit ID used by the provider.

## Methods[​](#methods "Direct link to Methods")

### proveTx()[​](#provetx "Direct link to proveTx()")

> **proveTx**(`unprovenTx`, `proveTxConfig?`): `Promise`<[`UnboundTransaction`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnboundTransaction.md)>

Creates call proofs for an unproven transaction. The resulting transaction is unbalanced and must be balanced using the [WalletProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/WalletProvider.md) interface. contain a single contract call.

#### Parameters[​](#parameters "Direct link to Parameters")

##### unprovenTx[​](#unproventx "Direct link to unprovenTx")

`UnprovenTransaction`

##### proveTxConfig?[​](#provetxconfig "Direct link to proveTxConfig?")

[`ProveTxConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProveTxConfig.md)

The configuration for the proof request to the proof provider. Empty in case a deploy transaction is being proved with no user-defined timeout.

#### Returns[​](#returns "Direct link to Returns")

`Promise`<[`UnboundTransaction`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnboundTransaction.md)>
