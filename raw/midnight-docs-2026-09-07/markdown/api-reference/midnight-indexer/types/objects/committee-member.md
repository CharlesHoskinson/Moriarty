# CommitteeMember

> For the complete documentation index, see [llms.txt](/llms.txt)

Committee member for an epoch.

```
type CommitteeMember {

  epochNo: Int!

  position: Int!

  sidechainPubkeyHex: String!

  expectedSlots: Int!

  auraPubkeyHex: String

  poolIdHex: String

  spoSkHex: String

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`CommitteeMember.epochNo`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#committeememberepochnoint-- "Direct link to committeememberepochnoint--")

#### [`CommitteeMember.position`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#committeememberpositionint-- "Direct link to committeememberpositionint--")

#### [`CommitteeMember.sidechainPubkeyHex`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#committeemembersidechainpubkeyhexstring-- "Direct link to committeemembersidechainpubkeyhexstring--")

#### [`CommitteeMember.expectedSlots`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#committeememberexpectedslotsint-- "Direct link to committeememberexpectedslotsint--")

#### [`CommitteeMember.auraPubkeyHex`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#committeememberaurapubkeyhexstring- "Direct link to committeememberaurapubkeyhexstring-")

#### [`CommitteeMember.poolIdHex`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#committeememberpoolidhexstring- "Direct link to committeememberpoolidhexstring-")

#### [`CommitteeMember.spoSkHex`](#) ● [`String`](/api-reference/midnight-indexer/types/scalars/string.md) scalar[​](#committeemembersposkhexstring- "Direct link to committeemembersposkhexstring-")

### Returned By[​](#returned-by "Direct link to Returned By")

[`committee`](/api-reference/midnight-indexer/operations/queries/committee.md) query
