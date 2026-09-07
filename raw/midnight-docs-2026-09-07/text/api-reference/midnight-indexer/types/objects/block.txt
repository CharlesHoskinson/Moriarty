# Block

> For the complete documentation index, see [llms.txt](/llms.txt)

A block with its relevant data.

```
type Block {

  hash: HexEncoded!

  height: Int!

  protocolVersion: Int!

  timestamp: Int!

  author: HexEncoded

  ledgerParameters: HexEncoded!

  parent: Block

  transactions: [Transaction!]!

  systemParameters: SystemParameters!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`Block.hash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#blockhashhexencoded-- "Direct link to blockhashhexencoded--")

The block hash.

#### [`Block.height`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#blockheightint-- "Direct link to blockheightint--")

The block height.

#### [`Block.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#blockprotocolversionint-- "Direct link to blockprotocolversionint--")

The protocol version.

#### [`Block.timestamp`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#blocktimestampint-- "Direct link to blocktimestampint--")

The UNIX timestamp.

#### [`Block.author`](#) ● [`HexEncoded`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) scalar[​](#blockauthorhexencoded- "Direct link to blockauthorhexencoded-")

The hex-encoded block author.

#### [`Block.ledgerParameters`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#blockledgerparametershexencoded-- "Direct link to blockledgerparametershexencoded--")

The hex-encoded ledger parameters for this block.

#### [`Block.parent`](#) ● [`Block`](/api-reference/midnight-indexer/types/objects/block.md) object[​](#blockparentblock- "Direct link to blockparentblock-")

The parent of this block.

#### [`Block.transactions`](#) ● [`[Transaction!]!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#blocktransactionstransaction-- "Direct link to blocktransactionstransaction--")

The transactions within this block.

#### [`Block.systemParameters`](#) ● [`SystemParameters!`](/api-reference/midnight-indexer/types/objects/system-parameters.md) non-null object[​](#blocksystemparameterssystemparameters-- "Direct link to blocksystemparameterssystemparameters--")

The system parameters (governance) at this block height.

### Returned By[​](#returned-by "Direct link to Returned By")

[`block`](/api-reference/midnight-indexer/operations/queries/block.md) query ● [`blocks`](/api-reference/midnight-indexer/operations/subscriptions/blocks.md) subscription

### Member Of[​](#member-of "Direct link to Member Of")

[`Block`](/api-reference/midnight-indexer/types/objects/block.md) object ● [`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object ● [`SystemTransaction`](/api-reference/midnight-indexer/types/objects/system-transaction.md) object ● [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface
