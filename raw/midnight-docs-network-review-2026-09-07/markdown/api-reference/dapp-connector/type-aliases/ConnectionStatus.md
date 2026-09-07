# ConnectionStatus

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / ConnectionStatus

# Type Alias: ConnectionStatus

> **ConnectionStatus** = { `networkId`: `string`; `status`: `"connected"`; } | { `status`: `"disconnected"`; }

Status of an existing connection to wallet It either indicates that the connection is established to a specific network id, or that the connection is lost

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

{ `networkId`: `string`; `status`: `"connected"`; }

### networkId[​](#networkid "Direct link to networkId")

> **networkId**: `string`

### status[​](#status "Direct link to status")

> **status**: `"connected"`

Connection is established to following network id

{ `status`: `"disconnected"`; }

### status[​](#status-1 "Direct link to status")

> **status**: `"disconnected"`

Connection is lost
