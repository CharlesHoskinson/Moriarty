# EpochPerf

> For the complete documentation index, see [llms.txt](/llms.txt)

SPO performance for an epoch.

```
type EpochPerf {

  epochNo: Int!

  spoSkHex: String!

  produced: Int!

  expected: Int!

  identityLabel: String

  stakeSnapshot: String

  poolIdHex: String

  validatorClass: String

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`EpochPerf.epochNo`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochperfepochnoint-- "Direct link to epochperfepochnoint--")

#### [`EpochPerf.spoSkHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#epochperfsposkhexstring-- "Direct link to epochperfsposkhexstring--")

#### [`EpochPerf.produced`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochperfproducedint-- "Direct link to epochperfproducedint--")

#### [`EpochPerf.expected`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#epochperfexpectedint-- "Direct link to epochperfexpectedint--")

#### [`EpochPerf.identityLabel`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#epochperfidentitylabelstring- "Direct link to epochperfidentitylabelstring-")

#### [`EpochPerf.stakeSnapshot`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#epochperfstakesnapshotstring- "Direct link to epochperfstakesnapshotstring-")

#### [`EpochPerf.poolIdHex`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#epochperfpoolidhexstring- "Direct link to epochperfpoolidhexstring-")

#### [`EpochPerf.validatorClass`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#epochperfvalidatorclassstring- "Direct link to epochperfvalidatorclassstring-")

### Returned By[​](#returned-by "Direct link to Returned By")

[`epochPerformance`](/api-reference/midnight-indexer/operations/queries/epoch-performance.md) query ● [`spoPerformanceBySpoSk`](/api-reference/midnight-indexer/operations/queries/spo-performance-by-spo-sk.md) query ● [`spoPerformanceLatest`](/api-reference/midnight-indexer/operations/queries/spo-performance-latest.md) query

### Member Of[​](#member-of "Direct link to Member Of")

[`SpoComposite`](/api-reference/midnight-indexer/types/objects/spo-composite.md) object
