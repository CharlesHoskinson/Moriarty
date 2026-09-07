# UnshieldedTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

A transaction that created and/or spent UTXOs alongside these and other information.

```
type UnshieldedTransaction {

  transaction: Transaction!

  createdUtxos: [UnshieldedUtxo!]!

  spentUtxos: [UnshieldedUtxo!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`UnshieldedTransaction.transaction`](#) ● [`Transaction!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#unshieldedtransactiontransactiontransaction-- "Direct link to unshieldedtransactiontransactiontransaction--")

The transaction that created and/or spent UTXOs.

#### [`UnshieldedTransaction.createdUtxos`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#unshieldedtransactioncreatedutxosunshieldedutxo-- "Direct link to unshieldedtransactioncreatedutxosunshieldedutxo--")

UTXOs created in the above transaction, possibly empty.

#### [`UnshieldedTransaction.spentUtxos`](#) ● [`[UnshieldedUtxo!]!`](/api-reference/midnight-indexer/types/objects/unshielded-utxo.md) non-null object[​](#unshieldedtransactionspentutxosunshieldedutxo-- "Direct link to unshieldedtransactionspentutxosunshieldedutxo--")

UTXOs spent in the above transaction, possibly empty.

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`UnshieldedTransactionsEvent`](/api-reference/midnight-indexer/types/unions/unshielded-transactions-event.md) union
