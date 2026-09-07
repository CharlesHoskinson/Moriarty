# ContractCall

> For the complete documentation index, see [llms.txt](/llms.txt)

A contract call.

```
type ContractCall implements ContractAction {

  address: HexEncoded!

  state: HexEncoded!

  zswapState: HexEncoded!

  entryPoint: String!

  transaction: Transaction!

  deploy: ContractDeploy!

  unshieldedBalances: [ContractBalance!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ContractCall.address`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractcalladdresshexencoded-- "Direct link to contractcalladdresshexencoded--")

The hex-encoded serialized address.

#### [`ContractCall.state`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractcallstatehexencoded-- "Direct link to contractcallstatehexencoded--")

The hex-encoded serialized state.

#### [`ContractCall.zswapState`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractcallzswapstatehexencoded-- "Direct link to contractcallzswapstatehexencoded--")

The hex-encoded serialized contract-specific zswap state.

#### [`ContractCall.entryPoint`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#contractcallentrypointstring-- "Direct link to contractcallentrypointstring--")

The entry point.

#### [`ContractCall.transaction`](#) ● [`Transaction!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#contractcalltransactiontransaction-- "Direct link to contractcalltransactiontransaction--")

Transaction for this contract call.

#### [`ContractCall.deploy`](#) ● [`ContractDeploy!`](/api-reference/midnight-indexer/types/objects/contract-deploy.md) non-null object[​](#contractcalldeploycontractdeploy-- "Direct link to contractcalldeploycontractdeploy--")

Contract deploy for this contract call.

#### [`ContractCall.unshieldedBalances`](#) ● [`[ContractBalance!]!`](/api-reference/midnight-indexer/types/objects/contract-balance.md) non-null object[​](#contractcallunshieldedbalancescontractbalance-- "Direct link to contractcallunshieldedbalancescontractbalance--")

Unshielded token balances held by this contract.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface[​](#contractaction- "Direct link to contractaction-")

A contract action.
