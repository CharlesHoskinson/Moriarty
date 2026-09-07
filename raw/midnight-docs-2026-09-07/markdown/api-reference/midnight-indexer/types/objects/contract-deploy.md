# ContractDeploy

> For the complete documentation index, see [llms.txt](/llms.txt)

A contract deployment.

```
type ContractDeploy implements ContractAction {

  address: HexEncoded!

  state: HexEncoded!

  zswapState: HexEncoded!

  transaction: Transaction!

  unshieldedBalances: [ContractBalance!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ContractDeploy.address`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractdeployaddresshexencoded-- "Direct link to contractdeployaddresshexencoded--")

The hex-encoded serialized address.

#### [`ContractDeploy.state`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractdeploystatehexencoded-- "Direct link to contractdeploystatehexencoded--")

The hex-encoded serialized state.

#### [`ContractDeploy.zswapState`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractdeployzswapstatehexencoded-- "Direct link to contractdeployzswapstatehexencoded--")

The hex-encoded serialized contract-specific zswap state.

#### [`ContractDeploy.transaction`](#) ● [`Transaction!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#contractdeploytransactiontransaction-- "Direct link to contractdeploytransactiontransaction--")

Transaction for this contract deploy.

#### [`ContractDeploy.unshieldedBalances`](#) ● [`[ContractBalance!]!`](/api-reference/midnight-indexer/types/objects/contract-balance.md) non-null object[​](#contractdeployunshieldedbalancescontractbalance-- "Direct link to contractdeployunshieldedbalancescontractbalance--")

Unshielded token balances held by this contract.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface[​](#contractaction- "Direct link to contractaction-")

A contract action.

### Member Of[​](#member-of "Direct link to Member Of")

[`ContractCall`](/api-reference/midnight-indexer/types/objects/contract-call.md) object
