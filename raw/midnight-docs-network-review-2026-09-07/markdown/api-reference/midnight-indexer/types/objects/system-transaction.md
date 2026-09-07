# SystemTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

A system Midnight transaction.

```
type SystemTransaction implements Transaction {

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

#### [`SystemTransaction.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#systemtransactionidint-- "Direct link to systemtransactionidint--")

The transaction ID.

#### [`SystemTransaction.hash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#systemtransactionhashhexencoded-- "Direct link to systemtransactionhashhexencoded--")

The hex-encoded transaction hash.

#### [`SystemTransaction.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#systemtransactionprotocolversionint-- "Direct link to systemtransactionprotocolversionint--")

The protocol version.

#### [`SystemTransaction.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#systemtransactionrawhexencoded-- "Direct link to systemtransactionrawhexencoded--")

The hex-encoded serialized transaction content.

#### [`SystemTransaction.block`](#) ● [`Block!`](/api-reference/midnight-indexer/types/objects/block.md) non-null object[​](#systemtransactionblockblock-- "Direct link to systemtransactionblockblock--")

The block for this transaction.

#### [`SystemTransaction.contractActions`](#) ● [`[ContractAction!]!`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) non-null interface[​](#systemtransactioncontractactionscontractaction-- "Direct link to systemtransactioncontractactionscontractaction--")

The contract actions for this transaction.

#### [`SystemTransaction.unshieldedCreatedOutputs`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#systemtransactionunshieldedcreatedoutputsunshieldedutxo-- "Direct link to systemtransactionunshieldedcreatedoutputsunshieldedutxo--")

Unshielded UTXOs created by this transaction.

#### [`SystemTransaction.unshieldedSpentOutputs`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#systemtransactionunshieldedspentoutputsunshieldedutxo-- "Direct link to systemtransactionunshieldedspentoutputsunshieldedutxo--")

Unshielded UTXOs spent (consumed) by this transaction.

#### [`SystemTransaction.zswapLedgerEvents`](#) ● [`[ZswapLedgerEvent!]!`](/api-reference/midnight-indexer/types/objects/zswap-ledger-event.md) non-null object[​](#systemtransactionzswapledgereventszswapledgerevent-- "Direct link to systemtransactionzswapledgereventszswapledgerevent--")

Zswap ledger events of this transaction.

#### [`SystemTransaction.dustLedgerEvents`](#) ● [`[DustLedgerEvent!]!`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) non-null interface[​](#systemtransactiondustledgereventsdustledgerevent-- "Direct link to systemtransactiondustledgereventsdustledgerevent--")

Dust ledger events of this transaction.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface[​](#transaction- "Direct link to transaction-")

A Midnight transaction.
