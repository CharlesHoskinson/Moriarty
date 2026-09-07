# CircuitMaintenanceTxInterface

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CircuitMaintenanceTxInterface

# Type Alias: CircuitMaintenanceTxInterface

> **CircuitMaintenanceTxInterface** = `object`

An interface for creating maintenance transactions for a specific circuit defined in a given contract.

## Methods[​](#methods "Direct link to Methods")

### insertVerifierKey()[​](#insertverifierkey "Direct link to insertVerifierKey()")

> **insertVerifierKey**(`newVk`): `Promise`<`FinalizedTxData`>

Constructs and submits a transaction that adds a new verifier key to the blockchain for this circuit at this contract's address.

#### Parameters[​](#parameters "Direct link to Parameters")

##### newVk[​](#newvk "Direct link to newVk")

`VerifierKey`

The new verifier key to add for this circuit.

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`FinalizedTxData`>

***

### removeVerifierKey()[​](#removeverifierkey "Direct link to removeVerifierKey()")

> **removeVerifierKey**(): `Promise`<`FinalizedTxData`>

Constructs and submits a transaction that removes the current verifier key stored on the blockchain for this circuit at this contract's address.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`FinalizedTxData`>
