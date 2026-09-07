# dustGenerationStatus

> For the complete documentation index, see [llms.txt](/llms.txt)

Get DUST generation status for specific Cardano reward addresses.

```
dustGenerationStatus(

  cardanoRewardAddresses: [CardanoRewardAddress!]!

): [DustGenerationStatus!]!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`dustGenerationStatus.cardanoRewardAddresses`](#) ● [`[CardanoRewardAddress!]!`](/api-reference/midnight-indexer/types/scalars/cardano-reward-address.md) non-null scalar[​](#dustgenerationstatuscardanorewardaddressescardanorewardaddress-- "Direct link to dustgenerationstatuscardanorewardaddressescardanorewardaddress--")

### Type[​](#type "Direct link to Type")

#### [`DustGenerationStatus`](/api-reference/midnight-indexer/types/objects/dust-generation-status.md) object[​](#dustgenerationstatus- "Direct link to dustgenerationstatus-")

DUST generation status for a specific Cardano reward address.
