# DeployTxFailedError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployTxFailedError

# Class: DeployTxFailedError

An error indicating that a deploy transaction was not successfully applied by the consensus node.

## Extends[​](#extends "Direct link to Extends")

* [`TxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new DeployTxFailedError**(`finalizedTxData`): `DeployTxFailedError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### finalizedTxData[​](#finalizedtxdata "Direct link to finalizedTxData")

`FinalizedTxData`

The finalization data of the deployment transaction that failed.

#### Returns[​](#returns "Direct link to Returns")

`DeployTxFailedError`

#### Overrides[​](#overrides "Direct link to Overrides")

[`TxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md).[`constructor`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md#constructor)

## Properties[​](#properties "Direct link to Properties")

### circuitId?[​](#circuitid "Direct link to circuitId?")

> `readonly` `optional` **circuitId?**: `string` | `string`\[]

The name of the circuit that was called to create the call transaction that failed. Only defined if a call transaction failed.

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`TxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md).[`circuitId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md#circuitid)

***

### finalizedTxData[​](#finalizedtxdata-1 "Direct link to finalizedTxData")

> `readonly` **finalizedTxData**: `FinalizedTxData`

The finalization data of the transaction that failed.

#### Inherited from[​](#inherited-from-1 "Direct link to Inherited from")

[`TxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md).[`finalizedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md#finalizedtxdata)
