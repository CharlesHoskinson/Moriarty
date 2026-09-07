# transactions

> For the complete documentation index, see [llms.txt](/llms.txt)

Find transactions for the given offset.

```
transactions(

  offset: TransactionOffset!

): [Transaction!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`transactions.offset`](#) ● [`TransactionOffset!`](/api-reference/midnight-indexer/types/inputs/transaction-offset.md) non-null input[​](#transactionsoffsettransactionoffset-- "Direct link to transactionsoffsettransactionoffset--")

### Type[​](#type "Direct link to Type")

#### [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface[​](#transaction- "Direct link to transaction-")

A Midnight transaction.
