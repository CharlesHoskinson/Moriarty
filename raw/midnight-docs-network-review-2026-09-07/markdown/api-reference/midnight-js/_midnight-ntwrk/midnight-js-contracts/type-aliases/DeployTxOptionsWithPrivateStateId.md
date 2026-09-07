# DeployTxOptionsWithPrivateStateId

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployTxOptionsWithPrivateStateId

# Type Alias: DeployTxOptionsWithPrivateStateId\<C>

> **DeployTxOptionsWithPrivateStateId**<`C`> = [`DeployTxOptionsWithPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateState.md)<`C`> & `object`

Configuration for creating deploy transactions for contracts with private state. This configuration is used when a deployment transaction is created and an initial private state needs to be stored, as is the case in [submitDeployTx](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/submitDeployTx.md).

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### privateStateId[​](#privatestateid "Direct link to privateStateId")

> `readonly` **privateStateId**: `PrivateStateId`

The identifier for the private state of the contract.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
