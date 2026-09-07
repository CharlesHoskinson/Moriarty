# IndexerClient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new IndexerClient**(`indexerUrl`, `logger`): `IndexerClient`

Creates an instance of IndexerClient.

#### Parameters[​](#parameters "Direct link to Parameters")

##### indexerUrl[​](#indexerurl "Direct link to indexerUrl")

`string`

The URL of the indexer service.

##### logger[​](#logger "Direct link to logger")

`Logger`

The logger instance for logging information.

#### Returns[​](#returns "Direct link to Returns")

`IndexerClient`

## Properties[​](#properties "Direct link to Properties")

### indexerUrl[​](#indexerurl-1 "Direct link to indexerUrl")

> `readonly` **indexerUrl**: `string`

## Methods[​](#methods "Direct link to Methods")

### health()[​](#health "Direct link to health()")

> **health**(): `Promise`<`AxiosResponse`<`any`, `any`, { }>>

Checks the health status of the indexer service. Makes a GET request to the status endpoint of the indexer service.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`AxiosResponse`<`any`, `any`, { }>>

A promise that resolves to the response of the health check or logs an error if the request fails.
