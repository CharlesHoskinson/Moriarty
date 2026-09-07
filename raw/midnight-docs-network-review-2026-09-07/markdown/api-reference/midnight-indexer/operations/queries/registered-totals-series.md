# registeredTotalsSeries

> For the complete documentation index, see [llms.txt](/llms.txt)

Get cumulative registration totals for an epoch range.

```
registeredTotalsSeries(

  fromEpoch: Int!

  toEpoch: Int!

): [RegisteredTotals!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`registeredTotalsSeries.fromEpoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredtotalsseriesfromepochint-- "Direct link to registeredtotalsseriesfromepochint--")

#### [`registeredTotalsSeries.toEpoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredtotalsseriestoepochint-- "Direct link to registeredtotalsseriestoepochint--")

### Type[​](#type "Direct link to Type")

#### [`RegisteredTotals`](/api-reference/midnight-indexer/types/objects/registered-totals.md) object[​](#registeredtotals- "Direct link to registeredtotals-")

Cumulative registration totals for an epoch.
