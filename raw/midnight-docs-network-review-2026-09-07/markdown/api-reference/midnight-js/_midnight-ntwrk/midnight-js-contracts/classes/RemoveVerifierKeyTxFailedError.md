# RemoveVerifierKeyTxFailedError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / RemoveVerifierKeyTxFailedError

# Class: RemoveVerifierKeyTxFailedError

An error indicating that a verifier key removal transaction failed.

## Extends[​](#extends "Direct link to Extends")

* [`TxFailedError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/classes/TxFailedError.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new RemoveVerifierKeyTxFailedError**(`finalizedTxData`): `RemoveVerifierKeyTxFailedError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### finalizedTxData[​](#finalizedtxdata "Direct link to finalizedTxData")

`FinalizedTxData`

#### Returns[​](#returns "Direct link to Returns")

`RemoveVerifierKeyTxFailedError`

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
