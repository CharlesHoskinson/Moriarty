# ContractBalance

> For the complete documentation index, see [llms.txt](/llms.txt)

Represents a token balance held by a contract. This type is exposed through the GraphQL API to allow clients to query unshielded token balances for any contract action (Deploy, Call, Update).

```
type ContractBalance {

  tokenType: HexEncoded!

  amount: String!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ContractBalance.tokenType`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractbalancetokentypehexencoded-- "Direct link to contractbalancetokentypehexencoded--")

Hex-encoded token type identifier.

#### [`ContractBalance.amount`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#contractbalanceamountstring-- "Direct link to contractbalanceamountstring--")

Balance amount as string to support larger integer values (up to 16 bytes).

### Member Of[​](#member-of "Direct link to Member Of")

[`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface ● [`ContractCall`](/api-reference/midnight-indexer/types/objects/contract-call.md) object ● [`ContractDeploy`](/api-reference/midnight-indexer/types/objects/contract-deploy.md) object ● [`ContractUpdate`](/api-reference/midnight-indexer/types/objects/contract-update.md) object
