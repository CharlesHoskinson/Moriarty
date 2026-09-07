# ShieldedTransactionsEvent

> For the complete documentation index, see [llms.txt](/llms.txt)

An event of the shielded transactions subscription.

```
union ShieldedTransactionsEvent =

  | RelevantTransaction

  | ShieldedTransactionsProgress
```

### Possible types[​](#possible-types "Direct link to Possible types")

#### [`ShieldedTransactionsEvent.RelevantTransaction`](/api-reference/midnight-indexer/types/objects/relevant-transaction.md) object[​](#shieldedtransactionseventrelevanttransaction- "Direct link to shieldedtransactionseventrelevanttransaction-")

A transaction relevant for the subscribing wallet and an optional collapsed merkle tree.

#### [`ShieldedTransactionsEvent.ShieldedTransactionsProgress`](/api-reference/midnight-indexer/types/objects/shielded-transactions-progress.md) object[​](#shieldedtransactionseventshieldedtransactionsprogress- "Direct link to shieldedtransactionseventshieldedtransactionsprogress-")

Information about the shielded transactions indexing progress.

### Returned By[​](#returned-by "Direct link to Returned By")

[`shieldedTransactions`](/api-reference/midnight-indexer/operations/subscriptions/shielded-transactions.md) subscription
