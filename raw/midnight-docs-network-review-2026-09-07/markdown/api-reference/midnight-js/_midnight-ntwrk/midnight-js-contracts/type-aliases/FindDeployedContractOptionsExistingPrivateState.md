# FindDeployedContractOptionsExistingPrivateState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / FindDeployedContractOptionsExistingPrivateState

# Type Alias: FindDeployedContractOptionsExistingPrivateState\<C>

> **FindDeployedContractOptionsExistingPrivateState**<`C`> = [`FindDeployedContractOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsBase.md)<`C`> & `object`

[findDeployedContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/findDeployedContract.md) base configuration that includes an initial private state to store and the private state ID at which to store it. Only used if the intention is to overwrite the private state currently stored at the given private state ID.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### privateStateId[​](#privatestateid "Direct link to privateStateId")

> `readonly` **privateStateId**: `PrivateStateId`

An identifier for the private state of the contract being found.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
