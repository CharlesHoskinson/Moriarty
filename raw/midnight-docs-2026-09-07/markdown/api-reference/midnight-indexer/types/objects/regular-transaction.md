# RegularTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

A regular Midnight transaction.

```
type RegularTransaction implements Transaction {

  id: Int!

  hash: HexEncoded!

  protocolVersion: Int!

  raw: HexEncoded!

  transactionResult: TransactionResult!

  identifiers: [HexEncoded!]!

  merkleTreeRoot: HexEncoded!

  startIndex: Int!

  endIndex: Int!

  fees: TransactionFees!

  block: Block!

  contractActions: [ContractAction!]!

  unshieldedCreatedOutputs: [UnshieldedUtxo!]!

  unshieldedSpentOutputs: [UnshieldedUtxo!]!

  zswapLedgerEvents: [ZswapLedgerEvent!]!

  dustLedgerEvents: [DustLedgerEvent!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`RegularTransaction.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#regulartransactionidint-- "Direct link to regulartransactionidint--")

The transaction ID.

#### [`RegularTransaction.hash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#regulartransactionhashhexencoded-- "Direct link to regulartransactionhashhexencoded--")

The hex-encoded transaction hash.

#### [`RegularTransaction.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#regulartransactionprotocolversionint-- "Direct link to regulartransactionprotocolversionint--")

The protocol version.

#### [`RegularTransaction.raw`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#regulartransactionrawhexencoded-- "Direct link to regulartransactionrawhexencoded--")

The hex-encoded serialized transaction content.

#### [`RegularTransaction.transactionResult`](#) ● [`TransactionResult!`](/api-reference/midnight-indexer/types/objects/transaction-result.md) non-null object[​](#regulartransactiontransactionresulttransactionresult-- "Direct link to regulartransactiontransactionresulttransactionresult--")

The result of applying this transaction to the ledger state.

#### [`RegularTransaction.identifiers`](#) ● [`[HexEncoded!]!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#regulartransactionidentifiershexencoded-- "Direct link to regulartransactionidentifiershexencoded--")

The hex-encoded serialized transaction identifiers.

#### [`RegularTransaction.merkleTreeRoot`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#regulartransactionmerkletreeroothexencoded-- "Direct link to regulartransactionmerkletreeroothexencoded--")

The hex-encoded serialized merkle-tree root.

#### [`RegularTransaction.startIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#regulartransactionstartindexint-- "Direct link to regulartransactionstartindexint--")

The zswap state start index.

#### [`RegularTransaction.endIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#regulartransactionendindexint-- "Direct link to regulartransactionendindexint--")

The zswap state end index.

#### [`RegularTransaction.fees`](#) ● [`TransactionFees!`](/api-reference/midnight-indexer/types/objects/transaction-fees.md) non-null object[​](#regulartransactionfeestransactionfees-- "Direct link to regulartransactionfeestransactionfees--")

Fee information for this transaction.

#### [`RegularTransaction.block`](#) ● [`Block!`](/api-reference/midnight-indexer/types/objects/block.md) non-null object[​](#regulartransactionblockblock-- "Direct link to regulartransactionblockblock--")

The block for this transaction.

#### [`RegularTransaction.contractActions`](#) ● [`[ContractAction!]!`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) non-null interface[​](#regulartransactioncontractactionscontractaction-- "Direct link to regulartransactioncontractactionscontractaction--")

The contract actions for this transaction.

#### [`RegularTransaction.unshieldedCreatedOutputs`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#regulartransactionunshieldedcreatedoutputsunshieldedutxo-- "Direct link to regulartransactionunshieldedcreatedoutputsunshieldedutxo--")

Unshielded UTXOs created by this transaction.

#### [`RegularTransaction.unshieldedSpentOutputs`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#regulartransactionunshieldedspentoutputsunshieldedutxo-- "Direct link to regulartransactionunshieldedspentoutputsunshieldedutxo--")

Unshielded UTXOs spent (consumed) by this transaction.

#### [`RegularTransaction.zswapLedgerEvents`](#) ● [`[ZswapLedgerEvent!]!`](/api-reference/midnight-indexer/types/objects/zswap-ledger-event.md) non-null object[​](#regulartransactionzswapledgereventszswapledgerevent-- "Direct link to regulartransactionzswapledgereventszswapledgerevent--")

Zswap ledger events of this transaction.

#### [`RegularTransaction.dustLedgerEvents`](#) ● [`[DustLedgerEvent!]!`](/api-reference/midnight-indexer/types/interfaces/dust-ledger-event.md) non-null interface[​](#regulartransactiondustledgereventsdustledgerevent-- "Direct link to regulartransactiondustledgereventsdustledgerevent--")

Dust ledger events of this transaction.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface[​](#transaction- "Direct link to transaction-")

A Midnight transaction.

### Member Of[​](#member-of "Direct link to Member Of")

[`RelevantTransaction`](/api-reference/midnight-indexer/types/objects/relevant-transaction.md) object
