# UnshieldedTransactionsProgress

> For the complete documentation index, see [llms.txt](/llms.txt)

Information about the unshielded indexing progress.

```
type UnshieldedTransactionsProgress {

  highestTransactionId: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`UnshieldedTransactionsProgress.highestTransactionId`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#unshieldedtransactionsprogresshighesttransactionidint-- "Direct link to unshieldedtransactionsprogresshighesttransactionidint--")

The highest transaction ID of all currently known transactions for a subscribed address.

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`UnshieldedTransactionsEvent`](/api-reference/midnight-indexer/types/unions/unshielded-transactions-event.md) union
