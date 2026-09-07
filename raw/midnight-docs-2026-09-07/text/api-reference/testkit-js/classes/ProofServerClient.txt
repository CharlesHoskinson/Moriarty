# ProofServerClient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new ProofServerClient**(`proofServer`, `logger`): `ProofServerClient`

Creates an instance of ProofServerClient.

#### Parameters[​](#parameters "Direct link to Parameters")

##### proofServer[​](#proofserver "Direct link to proofServer")

`string`

The URL of the proof server service.

##### logger[​](#logger "Direct link to logger")

`Logger`

The logger instance for logging information.

#### Returns[​](#returns "Direct link to Returns")

`ProofServerClient`

## Properties[​](#properties "Direct link to Properties")

### proofServer[​](#proofserver-1 "Direct link to proofServer")

> `readonly` **proofServer**: `string`

## Methods[​](#methods "Direct link to Methods")

### health()[​](#health "Direct link to health()")

> **health**(): `Promise`<`AxiosResponse`<`any`, `any`, { }>>

Checks the health status of the indexer service. Makes a GET request to the status endpoint of the indexer service.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`AxiosResponse`<`any`, `any`, { }>>

A promise that resolves to the response of the health check or logs an error if the request fails.

***

### proveTx()[​](#provetx "Direct link to proveTx()")

> **proveTx**(`data?`, `config?`): `Promise`<`AxiosResponse`<`any`, `any`, { }>>

Proves a transaction by sending a POST request to the proof server.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### data?[​](#data "Direct link to data?")

`ArrayBuffer`

serialized transaction data

##### config?[​](#config "Direct link to config?")

`AxiosRequestConfig` = `...`

Axios request configuration

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`AxiosResponse`<`any`, `any`, { }>>
