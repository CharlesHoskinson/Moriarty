# shieldedTransactions

> For the complete documentation index, see [llms.txt](/llms.txt)

Subscribe to shielded transaction events for the given session ID starting at the given index or at zero if omitted.

```
shieldedTransactions(

  sessionId: HexEncoded!

  index: Int

): ShieldedTransactionsEvent!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`shieldedTransactions.sessionId`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#shieldedtransactionssessionidhexencoded-- "Direct link to shieldedtransactionssessionidhexencoded--")

#### [`shieldedTransactions.index`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#shieldedtransactionsindexint- "Direct link to shieldedtransactionsindexint-")

### Type[​](#type "Direct link to Type")

#### [`ShieldedTransactionsEvent`](/api-reference/midnight-indexer/types/unions/shielded-transactions-event.md) union[​](#shieldedtransactionsevent- "Direct link to shieldedtransactionsevent-")

An event of the shielded transactions subscription.
