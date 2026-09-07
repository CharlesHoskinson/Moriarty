# DustGenerationStatus

> For the complete documentation index, see [llms.txt](/llms.txt)

DUST generation status for a specific Cardano reward address.

```
type DustGenerationStatus {

  cardanoRewardAddress: CardanoRewardAddress!

  dustAddress: DustAddress

  registered: Boolean!

  nightBalance: String!

  generationRate: String!

  maxCapacity: String!

  currentCapacity: String!

  utxoTxHash: HexEncoded

  utxoOutputIndex: Int

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`DustGenerationStatus.cardanoRewardAddress`](#) ● [`CardanoRewardAddress!`](/api-reference/midnight-indexer/types/scalars/cardano-reward-address.md) non-null scalar[​](#dustgenerationstatuscardanorewardaddresscardanorewardaddress-- "Direct link to dustgenerationstatuscardanorewardaddresscardanorewardaddress--")

The Bech32-encoded Cardano reward address (e.g., stake\_test1... or stake1...).

#### [`DustGenerationStatus.dustAddress`](#) ● [`DustAddress`](/api-reference/midnight-indexer/types/scalars/dust-address.md) scalar[​](#dustgenerationstatusdustaddressdustaddress- "Direct link to dustgenerationstatusdustaddressdustaddress-")

The Bech32m-encoded associated DUST address if registered.

#### [`DustGenerationStatus.registered`](#) ● [`Boolean!`](/api-reference/midnight-indexer/types/scalars/boolean.md) non-null scalar[​](#dustgenerationstatusregisteredboolean-- "Direct link to dustgenerationstatusregisteredboolean--")

Whether this reward address is registered.

#### [`DustGenerationStatus.nightBalance`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#dustgenerationstatusnightbalancestring-- "Direct link to dustgenerationstatusnightbalancestring--")

NIGHT balance backing generation in STAR.

#### [`DustGenerationStatus.generationRate`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#dustgenerationstatusgenerationratestring-- "Direct link to dustgenerationstatusgenerationratestring--")

DUST generation rate in SPECK per second.

#### [`DustGenerationStatus.maxCapacity`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#dustgenerationstatusmaxcapacitystring-- "Direct link to dustgenerationstatusmaxcapacitystring--")

Maximum DUST capacity in SPECK.

#### [`DustGenerationStatus.currentCapacity`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#dustgenerationstatuscurrentcapacitystring-- "Direct link to dustgenerationstatuscurrentcapacitystring--")

Current generated DUST capacity in SPECK.

#### [`DustGenerationStatus.utxoTxHash`](#) ● [`HexEncoded`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) scalar[​](#dustgenerationstatusutxotxhashhexencoded- "Direct link to dustgenerationstatusutxotxhashhexencoded-")

Cardano UTXO transaction hash for update/unregister operations.

#### [`DustGenerationStatus.utxoOutputIndex`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#dustgenerationstatusutxooutputindexint- "Direct link to dustgenerationstatusutxooutputindexint-")

Cardano UTXO output index for update/unregister operations.

### Returned By[​](#returned-by "Direct link to Returned By")

[`dustGenerationStatus`](/api-reference/midnight-indexer/operations/queries/dust-generation-status.md) query
