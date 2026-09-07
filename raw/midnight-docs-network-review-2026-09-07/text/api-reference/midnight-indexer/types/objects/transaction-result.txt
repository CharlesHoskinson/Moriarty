# TransactionResult

> For the complete documentation index, see [llms.txt](/llms.txt)

The result of applying a transaction to the ledger state. In case of a partial success (status), there will be segments.

```
type TransactionResult {

  status: TransactionResultStatus!

  segments: [Segment!]

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`TransactionResult.status`](#) ● [`TransactionResultStatus!`](/api-reference/midnight-indexer/types/enums/transaction-result-status.md) non-null enum[​](#transactionresultstatustransactionresultstatus-- "Direct link to transactionresultstatustransactionresultstatus--")

#### [`TransactionResult.segments`](#) ● [`[Segment!]`](/api-reference/midnight-indexer/types/objects/segment.md) list object[​](#transactionresultsegmentssegment-- "Direct link to transactionresultsegmentssegment--")

### Member Of[​](#member-of "Direct link to Member Of")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object
