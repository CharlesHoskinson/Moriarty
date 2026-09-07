# RelevantTransaction

> For the complete documentation index, see [llms.txt](/llms.txt)

A transaction relevant for the subscribing wallet and an optional collapsed merkle tree.

```
type RelevantTransaction {

  transaction: RegularTransaction!

  collapsedMerkleTree: CollapsedMerkleTree

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`RelevantTransaction.transaction`](#) ● [`RegularTransaction!`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) non-null object[​](#relevanttransactiontransactionregulartransaction-- "Direct link to relevanttransactiontransactionregulartransaction--")

A transaction relevant for the subscribing wallet.

#### [`RelevantTransaction.collapsedMerkleTree`](#) ● [`CollapsedMerkleTree`](/api-reference/midnight-indexer/types/objects/collapsed-merkle-tree.md) object[​](#relevanttransactioncollapsedmerkletreecollapsedmerkletree- "Direct link to relevanttransactioncollapsedmerkletreecollapsedmerkletree-")

An optional collapsed merkle tree.

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`ShieldedTransactionsEvent`](/api-reference/midnight-indexer/types/unions/shielded-transactions-event.md) union
