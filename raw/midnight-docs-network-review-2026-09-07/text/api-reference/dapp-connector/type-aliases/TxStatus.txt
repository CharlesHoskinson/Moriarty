# TxStatus

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/dapp-connector-api v4.0.1**](/api-reference/dapp-connector.md)

***

[@midnight-ntwrk/dapp-connector-api](/api-reference/dapp-connector/globals.md) / TxStatus

# Type Alias: TxStatus

> **TxStatus** = { `executionStatus`: [`ExecutionStatus`](/api-reference/dapp-connector/type-aliases/ExecutionStatus.md); `status`: `"finalized"`; } | { `executionStatus`: [`ExecutionStatus`](/api-reference/dapp-connector/type-aliases/ExecutionStatus.md); `status`: `"confirmed"`; } | { `status`: `"pending"`; } | { `status`: `"discarded"`; }

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

{ `executionStatus`: [`ExecutionStatus`](/api-reference/dapp-connector/type-aliases/ExecutionStatus.md); `status`: `"finalized"`; }

### executionStatus[​](#executionstatus "Direct link to executionStatus")

> **executionStatus**: [`ExecutionStatus`](/api-reference/dapp-connector/type-aliases/ExecutionStatus.md)

### status[​](#status "Direct link to status")

> **status**: `"finalized"`

Transaction included in chain and finalized

{ `executionStatus`: [`ExecutionStatus`](/api-reference/dapp-connector/type-aliases/ExecutionStatus.md); `status`: `"confirmed"`; }

### executionStatus[​](#executionstatus-1 "Direct link to executionStatus")

> **executionStatus**: [`ExecutionStatus`](/api-reference/dapp-connector/type-aliases/ExecutionStatus.md)

### status[​](#status-1 "Direct link to status")

> **status**: `"confirmed"`

Transaction included in chain and not finalized yet

{ `status`: `"pending"`; }

### status[​](#status-2 "Direct link to status")

> **status**: `"pending"`

Transaction sent to network but is not known to be either confirmed or discarded yet

{ `status`: `"discarded"`; }

### status[​](#status-3 "Direct link to status")

> **status**: `"discarded"`

Transaction failed to be included in chain, e.g. because of TTL or some validity checks
