# ZswapLedgerEvent

> For the complete documentation index, see [llms.txt](/llms.txt)

A zswap related ledger event.

```
type ZswapLedgerEvent {

  id: Int!

  raw: HexEncoded!

  maxId: Int!

  protocolVersion: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ZswapLedgerEvent.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#zswapledgereventidint-- "Direct link to zswapledgereventidint--")

The ID of this zswap ledger event.

#### [`ZswapLedgerEvent.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#zswapledgereventrawhexencoded-- "Direct link to zswapledgereventrawhexencoded--")

The hex-encoded serialized event.

#### [`ZswapLedgerEvent.maxId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#zswapledgereventmaxidint-- "Direct link to zswapledgereventmaxidint--")

The maximum ID of all zswap ledger events.

#### [`ZswapLedgerEvent.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#zswapledgereventprotocolversionint-- "Direct link to zswapledgereventprotocolversionint--")

The protocol version.

### Returned By[​](#returned-by "Direct link to Returned By")

[`zswapLedgerEvents`](/api-reference/midnight-indexer/operations/subscriptions/zswap-ledger-events.md) subscription

### Member Of[​](#member-of "Direct link to Member Of")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object ● [`SystemTransaction`](/api-reference/midnight-indexer/types/objects/system-transaction.md) object ● [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface
