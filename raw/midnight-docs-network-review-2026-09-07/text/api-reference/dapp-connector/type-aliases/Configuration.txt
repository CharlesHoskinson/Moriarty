# Configuration

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / Configuration

# Type Alias: Configuration

> **Configuration** = `object`

## Properties[​](#properties "Direct link to Properties")

### indexerUri[​](#indexeruri "Direct link to indexerUri")

> **indexerUri**: `string`

Indexer URI

***

### indexerWsUri[​](#indexerwsuri "Direct link to indexerWsUri")

> **indexerWsUri**: `string`

Indexer WebSocket URI

***

### networkId[​](#networkid "Direct link to networkId")

> **networkId**: `string`

Network id connected to - present here mostly for completness and to allow dapp validate it is connected to the network it wishes to

***

### ~~proverServerUri?~~[​](#proverserveruri "Direct link to proverserveruri")

> `optional` **proverServerUri**: `string`

Prover Server URI, likely to not be present, as different proving modalities emerge

#### Deprecated[​](#deprecated "Direct link to Deprecated")

Use `getProvingProvider` instead

***

### substrateNodeUri[​](#substratenodeuri "Direct link to substrateNodeUri")

> **substrateNodeUri**: `string`

Substrate URI
