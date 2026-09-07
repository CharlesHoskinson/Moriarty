# indexerPublicDataProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-indexer-public-data-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-indexer-public-data-provider.md) / indexerPublicDataProvider

# Function: indexerPublicDataProvider()

> **indexerPublicDataProvider**(`queryURL`, `subscriptionURL`, `webSocketImpl?`): [`PublicDataProvider`](#)

Constructs a [PublicDataProvider](#) based on an [ApolloClient](#).

## Parameters[​](#parameters "Direct link to Parameters")

### queryURL[​](#queryurl "Direct link to queryURL")

`string`

The URL of a GraphQL server query endpoint.

### subscriptionURL[​](#subscriptionurl "Direct link to subscriptionURL")

`string`

The URL of a GraphQL server subscription (websocket) endpoint.

### webSocketImpl?[​](#websocketimpl "Direct link to webSocketImpl?")

*typeof* `WebSocket` = `ws.WebSocket`

An optional websocket implementation for the Apollo client to use.

TODO: Re-examine caching when 'ContractCall' and 'ContractDeploy' have transaction identifiers included.

## Returns[​](#returns "Direct link to Returns")

[`PublicDataProvider`](#)
