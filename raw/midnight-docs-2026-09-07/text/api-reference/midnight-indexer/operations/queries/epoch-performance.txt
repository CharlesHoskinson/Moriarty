# epochPerformance

> For the complete documentation index, see [llms.txt](/llms.txt)

Get epoch performance for all SPOs.

```
epochPerformance(

  epoch: Int!

  limit: Int

  offset: Int

): [EpochPerf!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`epochPerformance.epoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochperformanceepochint-- "Direct link to epochperformanceepochint--")

#### [`epochPerformance.limit`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#epochperformancelimitint- "Direct link to epochperformancelimitint-")

#### [`epochPerformance.offset`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#epochperformanceoffsetint- "Direct link to epochperformanceoffsetint-")

### Type[​](#type "Direct link to Type")

#### [`EpochPerf`](/api-reference/midnight-indexer/types/objects/epoch-perf.md) object[​](#epochperf- "Direct link to epochperf-")

SPO performance for an epoch.
