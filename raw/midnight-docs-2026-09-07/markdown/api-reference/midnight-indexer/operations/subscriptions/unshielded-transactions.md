# unshieldedTransactions

> For the complete documentation index, see [llms.txt](/llms.txt)

Subscribe unshielded transaction events for the given address and the given transaction ID or zero if omitted.

```
unshieldedTransactions(

  address: UnshieldedAddress!

  transactionId: Int

): UnshieldedTransactionsEvent!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`unshieldedTransactions.address`](#) ● [`UnshieldedAddress!`](/api-reference/midnight-indexer/types/scalars/unshielded-address.md) non-null scalar[​](#unshieldedtransactionsaddressunshieldedaddress-- "Direct link to unshieldedtransactionsaddressunshieldedaddress--")

#### [`unshieldedTransactions.transactionId`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#unshieldedtransactionstransactionidint- "Direct link to unshieldedtransactionstransactionidint-")

### Type[​](#type "Direct link to Type")

#### [`UnshieldedTransactionsEvent`](/api-reference/midnight-indexer/types/unions/unshielded-transactions-event.md) union[​](#unshieldedtransactionsevent- "Direct link to unshieldedtransactionsevent-")

An event of the unshielded transactions subscription.
