# Transaction

> For the complete documentation index, see [llms.txt](/llms.txt)

A Midnight transaction.

```
interface Transaction {

  id: Int!

  hash: HexEncoded!

  protocolVersion: Int!

  raw: HexEncoded!

  block: Block!

  contractActions: [ContractAction!]!

  unshieldedCreatedOutputs: [UnshieldedUtxo!]!

  unshieldedSpentOutputs: [UnshieldedUtxo!]!

  zswapLedgerEvents: [ZswapLedgerEvent!]!

  dustLedgerEvents: [DustLedgerEvent!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`Transaction.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#transactionidint-- "Direct link to transactionidint--")

#### [`Transaction.hash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#transactionhashhexencoded-- "Direct link to transactionhashhexencoded--")

#### [`Transaction.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#transactionprotocolversionint-- "Direct link to transactionprotocolversionint--")

#### [`Transaction.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#transactionrawhexencoded-- "Direct link to transactionrawhexencoded--")

#### [`Transaction.block`](#) ● [`Block!`](/api-reference/midnight-indexer/types/objects/block.md) non-null object[​](#transactionblockblock-- "Direct link to transactionblockblock--")

#### [`Transaction.contractActions`](#) ● [`[ContractAction!]!`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) non-null interface[​](#transactioncontractactionscontractaction-- "Direct link to transactioncontractactionscontractaction--")

#### [`Transaction.unshieldedCreatedOutputs`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#transactionunshieldedcreatedoutputsunshieldedutxo-- "Direct link to transactionunshieldedcreatedoutputsunshieldedutxo--")

#### [`Transaction.unshieldedSpentOutputs`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#transactionunshieldedspentoutputsunshieldedutxo-- "Direct link to transactionunshieldedspentoutputsunshieldedutxo--")

#### [`Transaction.zswapLedgerEvents`](#) ● [`[ZswapLedgerEvent!]!`](/api-reference/midnight-indexer/types/objects/zswap-ledger-event.md) non-null object[​](#transactionzswapledgereventszswapledgerevent-- "Direct link to transactionzswapledgereventszswapledgerevent--")

#### [`Transaction.dustLedgerEvents`](#) ● [`[DustLedgerEvent!]!`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) non-null interface[​](#transactiondustledgereventsdustledgerevent-- "Direct link to transactiondustledgereventsdustledgerevent--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`transactions`](/api-reference/midnight-indexer/operations/queries/transactions.md) query

### Member Of[​](#member-of "Direct link to Member Of")

[`Block`](/api-reference/midnight-indexer/types/objects/block.md) object ● [`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface ● [`ContractCall`](/api-reference/midnight-indexer/types/objects/contract-call.md) object ● [`ContractDeploy`](/api-reference/midnight-indexer/types/objects/contract-deploy.md) object ● [`ContractUpdate`](/api-reference/midnight-indexer/types/objects/contract-update.md) object ● [`UnshieldedTransaction`](/api-reference/midnight-indexer/types/objects/unshielded-transaction.md) object ● [`UnshieldedUtxo`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) object

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object ● [`SystemTransaction`](/api-reference/midnight-indexer/types/objects/system-transaction.md) object
