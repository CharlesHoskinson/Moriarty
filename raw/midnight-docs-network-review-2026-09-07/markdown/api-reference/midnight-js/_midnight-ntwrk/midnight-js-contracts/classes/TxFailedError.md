# TxFailedError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / TxFailedError

# Class: TxFailedError

An error indicating that a transaction submitted to a consensus node failed.

## Extends[​](#extends "Direct link to Extends")

* `Error`

## Extended by[​](#extended-by "Direct link to Extended by")

* [`CallTxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/CallTxFailedError.md)
* [`DeployTxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/DeployTxFailedError.md)
* [`InsertVerifierKeyTxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/InsertVerifierKeyTxFailedError.md)
* [`RemoveVerifierKeyTxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/RemoveVerifierKeyTxFailedError.md)
* [`ReplaceMaintenanceAuthorityTxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/ReplaceMaintenanceAuthorityTxFailedError.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new TxFailedError**(`finalizedTxData`, `circuitId?`): `TxFailedError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### finalizedTxData[​](#finalizedtxdata "Direct link to finalizedTxData")

`FinalizedTxData`

The finalization data of the transaction that failed.

##### circuitId?[​](#circuitid "Direct link to circuitId?")

`string` | `string`\[]

The name of the circuit that was called to create the call transaction that failed. Only defined if a call transaction failed.

#### Returns[​](#returns "Direct link to Returns")

`TxFailedError`

#### Overrides[​](#overrides "Direct link to Overrides")

`Error.constructor`

## Properties[​](#properties "Direct link to Properties")

### circuitId?[​](#circuitid-1 "Direct link to circuitId?")

> `readonly` `optional` **circuitId?**: `string` | `string`\[]

The name of the circuit that was called to create the call transaction that failed. Only defined if a call transaction failed.

***

### finalizedTxData[​](#finalizedtxdata-1 "Direct link to finalizedTxData")

> `readonly` **finalizedTxData**: `FinalizedTxData`

The finalization data of the transaction that failed.
