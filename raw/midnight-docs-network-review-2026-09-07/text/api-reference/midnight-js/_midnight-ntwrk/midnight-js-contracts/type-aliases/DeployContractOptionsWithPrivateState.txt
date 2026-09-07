# DeployContractOptionsWithPrivateState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployContractOptionsWithPrivateState

# Type Alias: DeployContractOptionsWithPrivateState\<C>

> **DeployContractOptionsWithPrivateState**<`C`> = [`DeployContractOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployContractOptionsBase.md)<`C`> & `object`

[deployContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/deployContract.md) base options with information needed to store private states; only used if the contract being deployed has a private state.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

> `readonly` **initialPrivateState**: `Contract.PrivateState`<`C`>

The private state to run the circuit against.

### privateStateId[​](#privatestateid "Direct link to privateStateId")

> `readonly` **privateStateId**: `PrivateStateId`

An identifier for the private state of the contract being found.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
