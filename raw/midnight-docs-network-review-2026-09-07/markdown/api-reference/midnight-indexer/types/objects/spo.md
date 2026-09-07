# Spo

> For the complete documentation index, see [llms.txt](/llms.txt)

SPO with optional metadata.

```
type Spo {

  poolIdHex: String!

  validatorClass: String!

  sidechainPubkeyHex: String!

  auraPubkeyHex: String

  name: String

  ticker: String

  homepageUrl: String

  logoUrl: String

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`Spo.poolIdHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spopoolidhexstring-- "Direct link to spopoolidhexstring--")

#### [`Spo.validatorClass`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spovalidatorclassstring-- "Direct link to spovalidatorclassstring--")

#### [`Spo.sidechainPubkeyHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#sposidechainpubkeyhexstring-- "Direct link to sposidechainpubkeyhexstring--")

#### [`Spo.auraPubkeyHex`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#spoaurapubkeyhexstring- "Direct link to spoaurapubkeyhexstring-")

#### [`Spo.name`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#sponamestring- "Direct link to sponamestring-")

#### [`Spo.ticker`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#spotickerstring- "Direct link to spotickerstring-")

#### [`Spo.homepageUrl`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#spohomepageurlstring- "Direct link to spohomepageurlstring-")

#### [`Spo.logoUrl`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#spologourlstring- "Direct link to spologourlstring-")

### Returned By[​](#returned-by "Direct link to Returned By")

[`spoByPoolId`](/api-reference/midnight-indexer/operations/queries/spo-by-pool-id.md) query ● [`spoList`](/api-reference/midnight-indexer/operations/queries/spo-list.md) query
