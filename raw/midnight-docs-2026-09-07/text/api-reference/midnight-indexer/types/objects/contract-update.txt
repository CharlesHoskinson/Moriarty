# ContractUpdate

> For the complete documentation index, see [llms.txt](/llms.txt)

A contract update.

```
type ContractUpdate implements ContractAction {

  address: HexEncoded!

  state: HexEncoded!

  zswapState: HexEncoded!

  transaction: Transaction!

  unshieldedBalances: [ContractBalance!]!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ContractUpdate.address`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractupdateaddresshexencoded-- "Direct link to contractupdateaddresshexencoded--")

The hex-encoded serialized address.

#### [`ContractUpdate.state`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractupdatestatehexencoded-- "Direct link to contractupdatestatehexencoded--")

The hex-encoded serialized state.

#### [`ContractUpdate.zswapState`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractupdatezswapstatehexencoded-- "Direct link to contractupdatezswapstatehexencoded--")

The hex-encoded serialized contract-specific zswap state.

#### [`ContractUpdate.transaction`](#) ● [`Transaction!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#contractupdatetransactiontransaction-- "Direct link to contractupdatetransactiontransaction--")

Transaction for this contract update.

#### [`ContractUpdate.unshieldedBalances`](#) ● [`[ContractBalance!]!`](/api-reference/midnight-indexer/types/objects/contract-balance.md) non-null object[​](#contractupdateunshieldedbalancescontractbalance-- "Direct link to contractupdateunshieldedbalancescontractbalance--")

Unshielded token balances held by this contract after the update.

### Interfaces[​](#interfaces "Direct link to Interfaces")

#### [`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface[​](#contractaction- "Direct link to contractaction-")

A contract action.
