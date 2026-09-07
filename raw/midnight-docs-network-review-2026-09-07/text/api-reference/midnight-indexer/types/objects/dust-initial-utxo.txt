# DustInitialUtxo

> For the complete documentation index, see [llms.txt](/llms.txt)

No description

```
type DustInitialUtxo implements DustLedgerEvent {

  id: Int!

  raw: HexEncoded!

  maxId: Int!

  protocolVersion: Int!

  output: DustOutput!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`DustInitialUtxo.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustinitialutxoidint-- "Direct link to dustinitialutxoidint--")

The ID of this dust ledger event.

#### [`DustInitialUtxo.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#dustinitialutxorawhexencoded-- "Direct link to dustinitialutxorawhexencoded--")

The hex-encoded serialized event.

#### [`DustInitialUtxo.maxId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustinitialutxomaxidint-- "Direct link to dustinitialutxomaxidint--")

The maximum ID of all dust ledger events.

#### [`DustInitialUtxo.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustinitialutxoprotocolversionint-- "Direct link to dustinitialutxoprotocolversionint--")

The protocol version.

#### [`DustInitialUtxo.output`](#) ● [`DustOutput!`](/api-reference/midnight-indexer/types/objects/dust-output.md) non-null object[​](#dustinitialutxooutputdustoutput-- "Direct link to dustinitialutxooutputdustoutput--")

The dust output.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`DustLedgerEvent`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) interface[​](#dustledgerevent- "Direct link to dustledgerevent-")

A dust related ledger event.
