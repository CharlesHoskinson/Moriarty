# DustLedgerEvent

> For the complete documentation index, see [llms.txt](/llms.txt)

A dust related ledger event.

```
interface DustLedgerEvent {

  id: Int!

  raw: HexEncoded!

  maxId: Int!

  protocolVersion: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`DustLedgerEvent.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustledgereventidint-- "Direct link to dustledgereventidint--")

#### [`DustLedgerEvent.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#dustledgereventrawhexencoded-- "Direct link to dustledgereventrawhexencoded--")

#### [`DustLedgerEvent.maxId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustledgereventmaxidint-- "Direct link to dustledgereventmaxidint--")

#### [`DustLedgerEvent.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#dustledgereventprotocolversionint-- "Direct link to dustledgereventprotocolversionint--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`dustLedgerEvents`](/api-reference/midnight-indexer/operations/subscriptions/dust-ledger-events.md) subscription

### Member Of[​](#member-of "Direct link to Member Of")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object ● [`SystemTransaction`](/api-reference/midnight-indexer/types/objects/system-transaction.md) object ● [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`DustGenerationDtimeUpdate`](/api-reference/midnight-indexer/types/objects/dust-generation-dtime-update.md) object ● [`DustInitialUtxo`](/api-reference/midnight-indexer/types/objects/dust-initial-utxo.md) object ● [`DustSpendProcessed`](/api-reference/midnight-indexer/types/objects/dust-spend-processed.md) object ● [`ParamChange`](/api-reference/midnight-indexer/types/objects/param-change.md) object
