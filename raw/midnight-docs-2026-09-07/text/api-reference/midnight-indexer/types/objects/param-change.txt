# ParamChange

> For the complete documentation index, see [llms.txt](/llms.txt)

No description

```
type ParamChange implements DustLedgerEvent {

  id: Int!

  raw: HexEncoded!

  maxId: Int!

  protocolVersion: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ParamChange.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#paramchangeidint-- "Direct link to paramchangeidint--")

The ID of this dust ledger event.

#### [`ParamChange.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#paramchangerawhexencoded-- "Direct link to paramchangerawhexencoded--")

The hex-encoded serialized event.

#### [`ParamChange.maxId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#paramchangemaxidint-- "Direct link to paramchangemaxidint--")

The maximum ID of all dust ledger events.

#### [`ParamChange.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#paramchangeprotocolversionint-- "Direct link to paramchangeprotocolversionint--")

The protocol version.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`DustLedgerEvent`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) interface[​](#dustledgerevent- "Direct link to dustledgerevent-")

A dust related ledger event.
