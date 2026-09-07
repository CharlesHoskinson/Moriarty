# RegisteredStat

> For the complete documentation index, see [llms.txt](/llms.txt)

Registration statistics for an epoch.

```
type RegisteredStat {

  epochNo: Int!

  federatedValidCount: Int!

  federatedInvalidCount: Int!

  registeredValidCount: Int!

  registeredInvalidCount: Int!

  dparam: Float

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`RegisteredStat.epochNo`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredstatepochnoint-- "Direct link to registeredstatepochnoint--")

#### [`RegisteredStat.federatedValidCount`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredstatfederatedvalidcountint-- "Direct link to registeredstatfederatedvalidcountint--")

#### [`RegisteredStat.federatedInvalidCount`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredstatfederatedinvalidcountint-- "Direct link to registeredstatfederatedinvalidcountint--")

#### [`RegisteredStat.registeredValidCount`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredstatregisteredvalidcountint-- "Direct link to registeredstatregisteredvalidcountint--")

#### [`RegisteredStat.registeredInvalidCount`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#registeredstatregisteredinvalidcountint-- "Direct link to registeredstatregisteredinvalidcountint--")

#### [`RegisteredStat.dparam`](#) ● [`Float`](/api-reference/midnight-indexer/types/scalars/float.md) scalar[​](#registeredstatdparamfloat- "Direct link to registeredstatdparamfloat-")

### Returned By[​](#returned-by "Direct link to Returned By")

[`registeredSpoSeries`](/api-reference/midnight-indexer/operations/queries/registered-spo-series.md) query
