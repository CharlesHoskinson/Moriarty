# UnshieldedTransactionsEvent

> For the complete documentation index, see [llms.txt](/llms.txt)

An event of the unshielded transactions subscription.

```
union UnshieldedTransactionsEvent =

  | UnshieldedTransaction

  | UnshieldedTransactionsProgress
```

### Possible types[​](#possible-types "Direct link to Possible types")

#### [`UnshieldedTransactionsEvent.UnshieldedTransaction`](/api-reference/midnight-indexer/types/objects/unshielded-transaction.md) object[​](#unshieldedtransactionseventunshieldedtransaction- "Direct link to unshieldedtransactionseventunshieldedtransaction-")

A transaction that created and/or spent UTXOs alongside these and other information.

#### [`UnshieldedTransactionsEvent.UnshieldedTransactionsProgress`](/api-reference/midnight-indexer/types/objects/unshielded-transactions-progress.md) object[​](#unshieldedtransactionseventunshieldedtransactionsprogress- "Direct link to unshieldedtransactionseventunshieldedtransactionsprogress-")

Information about the unshielded indexing progress.

### Returned By[​](#returned-by "Direct link to Returned By")

[`unshieldedTransactions`](/api-reference/midnight-indexer/operations/subscriptions/unshielded-transactions.md) subscription
