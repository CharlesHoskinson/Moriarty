# ContractMaintenanceTxInterface

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / ContractMaintenanceTxInterface

# Interface: ContractMaintenanceTxInterface

Interface for creating maintenance transactions for a contract that was deployed.

## Methods[​](#methods "Direct link to Methods")

### replaceAuthority()[​](#replaceauthority "Direct link to replaceAuthority()")

> **replaceAuthority**(`newAuthority`): `Promise`<`FinalizedTxData`>

Constructs and submits a transaction that replaces the maintenance authority stored on the blockchain for this contract.

#### Parameters[​](#parameters "Direct link to Parameters")

##### newAuthority[​](#newauthority "Direct link to newAuthority")

`string`

The new contract maintenance authority for this contract.

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`FinalizedTxData`>
