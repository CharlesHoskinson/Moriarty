# spoPerformanceBySpoSk

> For the complete documentation index, see [llms.txt](/llms.txt)

Get SPO performance by SPO key.

```
spoPerformanceBySpoSk(

  spoSkHex: String!

  limit: Int

  offset: Int

): [EpochPerf!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`spoPerformanceBySpoSk.spoSkHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spoperformancebysposksposkhexstring-- "Direct link to spoperformancebysposksposkhexstring--")

#### [`spoPerformanceBySpoSk.limit`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#spoperformancebysposklimitint- "Direct link to spoperformancebysposklimitint-")

#### [`spoPerformanceBySpoSk.offset`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#spoperformancebysposkoffsetint- "Direct link to spoperformancebysposkoffsetint-")

### Type[​](#type "Direct link to Type")

#### [`EpochPerf`](/api-reference/midnight-indexer/types/objects/epoch-perf.md) object[​](#epochperf- "Direct link to epochperf-")

SPO performance for an epoch.
