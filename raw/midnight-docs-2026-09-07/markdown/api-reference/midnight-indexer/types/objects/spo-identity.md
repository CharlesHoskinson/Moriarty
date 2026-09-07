# SpoIdentity

> For the complete documentation index, see [llms.txt](/llms.txt)

SPO identity information.

```
type SpoIdentity {

  poolIdHex: String!

  mainchainPubkeyHex: String!

  sidechainPubkeyHex: String!

  auraPubkeyHex: String

  validatorClass: String!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`SpoIdentity.poolIdHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spoidentitypoolidhexstring-- "Direct link to spoidentitypoolidhexstring--")

#### [`SpoIdentity.mainchainPubkeyHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spoidentitymainchainpubkeyhexstring-- "Direct link to spoidentitymainchainpubkeyhexstring--")

#### [`SpoIdentity.sidechainPubkeyHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spoidentitysidechainpubkeyhexstring-- "Direct link to spoidentitysidechainpubkeyhexstring--")

#### [`SpoIdentity.auraPubkeyHex`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#spoidentityaurapubkeyhexstring- "Direct link to spoidentityaurapubkeyhexstring-")

#### [`SpoIdentity.validatorClass`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#spoidentityvalidatorclassstring-- "Direct link to spoidentityvalidatorclassstring--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`spoIdentities`](/api-reference/midnight-indexer/operations/queries/spo-identities.md) query ● [`spoIdentityByPoolId`](/api-reference/midnight-indexer/operations/queries/spo-identity-by-pool-id.md) query

### Member Of[​](#member-of "Direct link to Member Of")

[`SpoComposite`](/api-reference/midnight-indexer/types/objects/spo-composite.md) object
