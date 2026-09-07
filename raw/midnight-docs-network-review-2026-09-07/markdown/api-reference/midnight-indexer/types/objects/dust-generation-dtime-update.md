# DustGenerationDtimeUpdate

> For the complete documentation index, see [llms.txt](/llms.txt)

No description

```
type DustGenerationDtimeUpdate implements DustLedgerEvent {

  id: Int!

  raw: HexEncoded!

  maxId: Int!

  protocolVersion: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`DustGenerationDtimeUpdate.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustgenerationdtimeupdateidint-- "Direct link to dustgenerationdtimeupdateidint--")

The ID of this dust ledger event.

#### [`DustGenerationDtimeUpdate.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#dustgenerationdtimeupdaterawhexencoded-- "Direct link to dustgenerationdtimeupdaterawhexencoded--")

The hex-encoded serialized event.

#### [`DustGenerationDtimeUpdate.maxId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustgenerationdtimeupdatemaxidint-- "Direct link to dustgenerationdtimeupdatemaxidint--")

The maximum ID of all dust ledger events.

#### [`DustGenerationDtimeUpdate.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustgenerationdtimeupdateprotocolversionint-- "Direct link to dustgenerationdtimeupdateprotocolversionint--")

The protocol version.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`DustLedgerEvent`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) interface[​](#dustledgerevent- "Direct link to dustledgerevent-")

A dust related ledger event.
