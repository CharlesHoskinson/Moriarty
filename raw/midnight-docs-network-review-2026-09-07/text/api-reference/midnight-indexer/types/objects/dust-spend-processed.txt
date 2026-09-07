# DustSpendProcessed

> For the complete documentation index, see [llms.txt](/llms.txt)

No description

```
type DustSpendProcessed implements DustLedgerEvent {

  id: Int!

  raw: HexEncoded!

  maxId: Int!

  protocolVersion: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`DustSpendProcessed.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustspendprocessedidint-- "Direct link to dustspendprocessedidint--")

The ID of this dust ledger event.

#### [`DustSpendProcessed.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#dustspendprocessedrawhexencoded-- "Direct link to dustspendprocessedrawhexencoded--")

The hex-encoded serialized event.

#### [`DustSpendProcessed.maxId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustspendprocessedmaxidint-- "Direct link to dustspendprocessedmaxidint--")

The maximum ID of all dust ledger events.

#### [`DustSpendProcessed.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustspendprocessedprotocolversionint-- "Direct link to dustspendprocessedprotocolversionint--")

The protocol version.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`DustLedgerEvent`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) interface[​](#dustledgerevent- "Direct link to dustledgerevent-")

A dust related ledger event.
