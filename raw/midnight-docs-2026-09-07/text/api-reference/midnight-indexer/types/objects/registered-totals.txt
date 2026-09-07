# RegisteredTotals

> For the complete documentation index, see [llms.txt](/llms.txt)

Cumulative registration totals for an epoch.

```
type RegisteredTotals {

  epochNo: Int!

  totalRegistered: Int!

  newlyRegistered: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`RegisteredTotals.epochNo`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredtotalsepochnoint-- "Direct link to registeredtotalsepochnoint--")

#### [`RegisteredTotals.totalRegistered`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredtotalstotalregisteredint-- "Direct link to registeredtotalstotalregisteredint--")

#### [`RegisteredTotals.newlyRegistered`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredtotalsnewlyregisteredint-- "Direct link to registeredtotalsnewlyregisteredint--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`registeredTotalsSeries`](/api-reference/midnight-indexer/operations/queries/registered-totals-series.md) query
