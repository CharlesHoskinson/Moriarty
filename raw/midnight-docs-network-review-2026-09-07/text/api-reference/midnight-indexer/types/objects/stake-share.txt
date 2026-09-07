# StakeShare

> For the complete documentation index, see [llms.txt](/llms.txt)

Stake share information for an SPO.

Values are sourced from mainchain pool data (e.g., Blockfrost) and keyed by Cardano pool\_id.

```
type StakeShare {

  poolIdHex: String!

  name: String

  ticker: String

  homepageUrl: String

  logoUrl: String

  liveStake: String

  activeStake: String

  liveDelegators: Int

  liveSaturation: Float

  declaredPledge: String

  livePledge: String

  stakeShare: Float

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`StakeShare.poolIdHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#stakesharepoolidhexstring-- "Direct link to stakesharepoolidhexstring--")

Cardano pool ID (56-character hex string).

#### [`StakeShare.name`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharenamestring- "Direct link to stakesharenamestring-")

Pool name from metadata.

#### [`StakeShare.ticker`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharetickerstring- "Direct link to stakesharetickerstring-")

Pool ticker from metadata.

#### [`StakeShare.homepageUrl`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharehomepageurlstring- "Direct link to stakesharehomepageurlstring-")

Pool homepage URL from metadata.

#### [`StakeShare.logoUrl`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharelogourlstring- "Direct link to stakesharelogourlstring-")

Pool logo URL from metadata.

#### [`StakeShare.liveStake`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharelivestakestring- "Direct link to stakesharelivestakestring-")

Current live stake in lovelace.

#### [`StakeShare.activeStake`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakeshareactivestakestring- "Direct link to stakeshareactivestakestring-")

Current active stake in lovelace.

#### [`StakeShare.liveDelegators`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#stakesharelivedelegatorsint- "Direct link to stakesharelivedelegatorsint-")

Number of live delegators.

#### [`StakeShare.liveSaturation`](#) ● [`Float`](/api-reference/midnight-indexer/types/scalars/float.md) scalar[​](#stakesharelivesaturationfloat- "Direct link to stakesharelivesaturationfloat-")

Saturation ratio (0.0 to 1.0+).

#### [`StakeShare.declaredPledge`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharedeclaredpledgestring- "Direct link to stakesharedeclaredpledgestring-")

Declared pledge in lovelace.

#### [`StakeShare.livePledge`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#stakesharelivepledgestring- "Direct link to stakesharelivepledgestring-")

Current live pledge in lovelace.

#### [`StakeShare.stakeShare`](#) ● [`Float`](/api-reference/midnight-indexer/types/scalars/float.md) scalar[​](#stakesharestakesharefloat- "Direct link to stakesharestakesharefloat-")

Stake share as a fraction of total stake.

### Returned By[​](#returned-by "Direct link to Returned By")

[`stakeDistribution`](/api-reference/midnight-indexer/operations/queries/stake-distribution.md) query
