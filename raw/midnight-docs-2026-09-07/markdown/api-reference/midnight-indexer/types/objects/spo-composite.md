# SpoComposite

> For the complete documentation index, see [llms.txt](/llms.txt)

Composite SPO data (identity + metadata + performance).

```
type SpoComposite {

  identity: SpoIdentity

  metadata: PoolMetadata

  performance: [EpochPerf!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`SpoComposite.identity`](#) ● [`SpoIdentity`](/api-reference/midnight-indexer/types/objects/spo-identity.md) object[​](#spocompositeidentityspoidentity- "Direct link to spocompositeidentityspoidentity-")

#### [`SpoComposite.metadata`](#) ● [`PoolMetadata`](/api-reference/midnight-indexer/types/objects/pool-metadata.md) object[​](#spocompositemetadatapoolmetadata- "Direct link to spocompositemetadatapoolmetadata-")

#### [`SpoComposite.performance`](#) ● [`[EpochPerf!]!`](/api-reference/midnight-indexer/types/objects/epoch-perf.md) non-null object[​](#spocompositeperformanceepochperf-- "Direct link to spocompositeperformanceepochperf--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`spoCompositeByPoolId`](/api-reference/midnight-indexer/operations/queries/spo-composite-by-pool-id.md) query
