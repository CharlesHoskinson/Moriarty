# EpochInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

Current epoch information.

```
type EpochInfo {

  epochNo: Int!

  durationSeconds: Int!

  elapsedSeconds: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`EpochInfo.epochNo`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochinfoepochnoint-- "Direct link to epochinfoepochnoint--")

#### [`EpochInfo.durationSeconds`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochinfodurationsecondsint-- "Direct link to epochinfodurationsecondsint--")

#### [`EpochInfo.elapsedSeconds`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochinfoelapsedsecondsint-- "Direct link to epochinfoelapsedsecondsint--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`currentEpochInfo`](/api-reference/midnight-indexer/operations/queries/current-epoch-info.md) query
