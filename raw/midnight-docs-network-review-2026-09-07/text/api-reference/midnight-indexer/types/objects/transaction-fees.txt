# TransactionFees

> For the complete documentation index, see [llms.txt](/llms.txt)

Fees information for a transaction, including both paid and estimated fees.

```
type TransactionFees {

  paidFees: String!

  estimatedFees: String!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`TransactionFees.paidFees`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#transactionfeespaidfeesstring-- "Direct link to transactionfeespaidfeesstring--")

The actual fees paid for this transaction in DUST.

#### [`TransactionFees.estimatedFees`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#transactionfeesestimatedfeesstring-- "Direct link to transactionfeesestimatedfeesstring--")

The estimated fees that was calculated for this transaction in DUST.

### Member Of[​](#member-of "Direct link to Member Of")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object
