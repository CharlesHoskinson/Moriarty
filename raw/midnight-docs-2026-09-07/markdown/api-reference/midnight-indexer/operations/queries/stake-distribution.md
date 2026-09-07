# stakeDistribution

> For the complete documentation index, see [llms.txt](/llms.txt)

Get stake distribution with search and ordering.

```
stakeDistribution(

  limit: Int

  offset: Int

  search: String

  orderByStakeDesc: Boolean

): [StakeShare!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`stakeDistribution.limit`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#stakedistributionlimitint- "Direct link to stakedistributionlimitint-")

#### [`stakeDistribution.offset`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#stakedistributionoffsetint- "Direct link to stakedistributionoffsetint-")

#### [`stakeDistribution.search`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakedistributionsearchstring- "Direct link to stakedistributionsearchstring-")

#### [`stakeDistribution.orderByStakeDesc`](#) ● [`Boolean`](/api-reference/midnight-indexer/types/scalars/boolean.md) scalar[​](#stakedistributionorderbystakedescboolean- "Direct link to stakedistributionorderbystakedescboolean-")

### Type[​](#type "Direct link to Type")

#### [`StakeShare`](/api-reference/midnight-indexer/types/objects/stake-share.md) object[​](#stakeshare- "Direct link to stakeshare-")

Stake share information for an SPO.

Values are sourced from mainchain pool data (e.g., Blockfrost) and keyed by Cardano pool\_id.
