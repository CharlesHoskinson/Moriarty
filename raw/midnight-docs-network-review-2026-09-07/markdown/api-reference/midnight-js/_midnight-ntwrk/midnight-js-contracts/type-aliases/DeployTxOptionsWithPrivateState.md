# DeployTxOptionsWithPrivateState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployTxOptionsWithPrivateState

# Type Alias: DeployTxOptionsWithPrivateState\<C>

> **DeployTxOptionsWithPrivateState**<`C`> = [`DeployTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsBase.md)<`C`> & `object`

Configuration for creating deploy transactions for contracts with private state. This configuration used as a base type for the [DeployTxOptionsWithPrivateStateId](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateStateId.md) configuration. It is also used directly as parameter to [createUnprovenDeployTx](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/createUnprovenDeployTx.md) which doesn't need to save private state (and therefore doesn't need a private state ID) but does need to supply an initial private state to run the contract constructor against.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

> `readonly` **initialPrivateState**: `Contract.PrivateState`<`C`>

The private state to run the contract constructor against.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
