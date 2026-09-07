# registeredSpoSeries

> For the complete documentation index, see [llms.txt](/llms.txt)

Get registration statistics for an epoch range.

```
registeredSpoSeries(

  fromEpoch: Int!

  toEpoch: Int!

): [RegisteredStat!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`registeredSpoSeries.fromEpoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredsposeriesfromepochint-- "Direct link to registeredsposeriesfromepochint--")

#### [`registeredSpoSeries.toEpoch`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredsposeriestoepochint-- "Direct link to registeredsposeriestoepochint--")

### Type[​](#type "Direct link to Type")

#### [`RegisteredStat`](/api-reference/midnight-indexer/types/objects/registered-stat.md) object[​](#registeredstat- "Direct link to registeredstat-")

Registration statistics for an epoch.
