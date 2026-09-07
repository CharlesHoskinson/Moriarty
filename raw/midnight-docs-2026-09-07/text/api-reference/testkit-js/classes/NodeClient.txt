# NodeClient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Client for interacting with a Midnight node's JSON-RPC API

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new NodeClient**(`nodeURL`, `logger`): `NodeClient`

Creates a new NodeClient instance

#### Parameters[​](#parameters "Direct link to Parameters")

##### nodeURL[​](#nodeurl "Direct link to nodeURL")

`string`

URL of the Midnight node

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

#### Returns[​](#returns "Direct link to Returns")

`NodeClient`

## Properties[​](#properties "Direct link to Properties")

### nodeURL[​](#nodeurl-1 "Direct link to nodeURL")

> `readonly` **nodeURL**: `string`

## Methods[​](#methods "Direct link to Methods")

### contractState()[​](#contractstate "Direct link to contractState()")

> **contractState**(`contractAddress`): `Promise`<`ContractState` | `null`>

Fetches the state of a contract

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

Address of the contract

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`ContractState` | `null`>

Contract state or null if not found

***

### health()[​](#health "Direct link to health()")

> **health**(): `Promise`<`AxiosResponse`<`any`, `any`, { }>>

Checks the health status of the node. Makes a GET request to the health endpoint of the node.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`AxiosResponse`<`any`, `any`, { }>>

A promise that resolves to the response of the health check or logs an error if the request fails.

***

### ledgerState()[​](#ledgerstate "Direct link to ledgerState()")

> **ledgerState**(`blockHash`): `Promise`<`LedgerState`>

Fetches the ledger state at a given block

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### blockHash[​](#blockhash "Direct link to blockHash")

`string`

Hash of the block

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`LedgerState`>

Ledger state

***

### ledgerStateBlob()[​](#ledgerstateblob "Direct link to ledgerStateBlob()")

> **ledgerStateBlob**(`blockHash`): `Promise`<`Uint8Array`<`ArrayBufferLike`>>

Fetches the raw ledger state blob at a given block

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### blockHash[​](#blockhash-1 "Direct link to blockHash")

`string`

Hash of the block

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`Uint8Array`<`ArrayBufferLike`>>

Raw ledger state data

#### Throws[​](#throws "Direct link to Throws")

If no ledger state is found

***

### ledgerVersion()[​](#ledgerversion "Direct link to ledgerVersion()")

> **ledgerVersion**(`blockHash`): `Promise`<`string`>

Fetches the ledger version at a given block

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### blockHash[​](#blockhash-2 "Direct link to blockHash")

`string`

Hash of the block

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<`string`>

Ledger version

#### Throws[​](#throws-1 "Direct link to Throws")

If no ledger version is found
