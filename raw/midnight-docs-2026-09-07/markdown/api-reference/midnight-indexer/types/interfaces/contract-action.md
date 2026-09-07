# ContractAction

> For the complete documentation index, see [llms.txt](/llms.txt)

A contract action.

```
interface ContractAction {

  address: HexEncoded!

  state: HexEncoded!

  zswapState: HexEncoded!

  transaction: Transaction!

  unshieldedBalances: [ContractBalance!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ContractAction.address`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractactionaddresshexencoded-- "Direct link to contractactionaddresshexencoded--")

#### [`ContractAction.state`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractactionstatehexencoded-- "Direct link to contractactionstatehexencoded--")

#### [`ContractAction.zswapState`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractactionzswapstatehexencoded-- "Direct link to contractactionzswapstatehexencoded--")

#### [`ContractAction.transaction`](#) ● [`Transaction!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#contractactiontransactiontransaction-- "Direct link to contractactiontransactiontransaction--")

#### [`ContractAction.unshieldedBalances`](#) ● [`[ContractBalance!]!`](/api-reference/midnight-indexer/types/objects/contract-balance.md) non-null object[​](#contractactionunshieldedbalancescontractbalance-- "Direct link to contractactionunshieldedbalancescontractbalance--")

### Returned By[​](#returned-by "Direct link to Returned By")

[`contractAction`](/api-reference/midnight-indexer/operations/queries/contract-action.md) query ● [`contractActions`](/api-reference/midnight-indexer/operations/subscriptions/contract-actions.md) subscription

### Member Of[​](#member-of "Direct link to Member Of")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object ● [`SystemTransaction`](/api-reference/midnight-indexer/types/objects/system-transaction.md) object ● [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface

### Implemented By[​](#implemented-by "Direct link to Implemented By")

[`ContractCall`](/api-reference/midnight-indexer/types/objects/contract-call.md) object ● [`ContractDeploy`](/api-reference/midnight-indexer/types/objects/contract-deploy.md) object ● [`ContractUpdate`](/api-reference/midnight-indexer/types/objects/contract-update.md) object
