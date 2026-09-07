# ShieldedTransactionsProgress

> For the complete documentation index, see [llms.txt](/llms.txt)

Information about the shielded transactions indexing progress.

```
type ShieldedTransactionsProgress {

  highestEndIndex: Int!

  highestCheckedEndIndex: Int!

  highestRelevantEndIndex: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ShieldedTransactionsProgress.highestEndIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#shieldedtransactionsprogresshighestendindexint-- "Direct link to shieldedtransactionsprogresshighestendindexint--")

The highest zswap state end index (see `endIndex` of `Transaction`) of all transactions. It represents the known state of the blockchain. A value of zero (completely unlikely) means that no shielded transactions have been indexed yet.

#### [`ShieldedTransactionsProgress.highestCheckedEndIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#shieldedtransactionsprogresshighestcheckedendindexint-- "Direct link to shieldedtransactionsprogresshighestcheckedendindexint--")

The highest zswap state end index (see `endIndex` of `Transaction`) of all transactions checked for relevance. Initially less than and eventually (when some wallet has been fully indexed) equal to `highest_end_index`. A value of zero (very unlikely) means that no wallet has subscribed before and indexing for the subscribing wallet has not yet started.

#### [`ShieldedTransactionsProgress.highestRelevantEndIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#shieldedtransactionsprogresshighestrelevantendindexint-- "Direct link to shieldedtransactionsprogresshighestrelevantendindexint--")

The highest zswap state end index (see `endIndex` of `Transaction`) of all relevant transactions for the subscribing wallet. Usually less than `highest_checked_end_index` unless the latest checked transaction is relevant for the subscribing wallet. A value of zero means that no relevant transactions have been indexed for the subscribing wallet.

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`ShieldedTransactionsEvent`](/api-reference/midnight-indexer/types/unions/shielded-transactions-event.md) union
