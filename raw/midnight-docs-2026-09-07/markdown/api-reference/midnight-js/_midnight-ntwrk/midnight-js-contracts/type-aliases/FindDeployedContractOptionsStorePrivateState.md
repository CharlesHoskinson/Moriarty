# FindDeployedContractOptionsStorePrivateState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / FindDeployedContractOptionsStorePrivateState

# Type Alias: FindDeployedContractOptionsStorePrivateState\<C>

> **FindDeployedContractOptionsStorePrivateState**<`C`> = [`FindDeployedContractOptionsExistingPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsExistingPrivateState.md)<`C`> & `object`

[findDeployedContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/findDeployedContract.md) configuration that includes an initial private state to store and the private state ID at which to store it. Only used if the intention is to overwrite the private state currently stored at the given private state ID.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

> `readonly` **initialPrivateState**: `Contract.PrivateState`<`C`>

For types of contract that make no use of private state and or witnesses that operate upon it, this property may be `undefined`. Otherwise, the value provided via this property should be same initial state that was used when calling [deployContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/deployContract.md).

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
