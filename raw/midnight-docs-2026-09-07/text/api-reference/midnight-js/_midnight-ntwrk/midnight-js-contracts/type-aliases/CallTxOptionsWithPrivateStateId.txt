# CallTxOptionsWithPrivateStateId

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallTxOptionsWithPrivateStateId

# Type Alias: CallTxOptionsWithPrivateStateId\<C, PCK>

> **CallTxOptionsWithPrivateStateId**<`C`, `PCK`> = [`CallTxOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsBase.md)<`C`, `PCK`> & `object`

Call transaction options with the private state ID to use to store the new private state resulting from the circuit call. Since a private state should already be stored at the given private state ID, we don't need an 'initialPrivateState' like in [DeployTxOptionsWithPrivateState](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/DeployTxOptionsWithPrivateState.md).

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### privateStateId[​](#privatestateid "Direct link to privateStateId")

> `readonly` **privateStateId**: `PrivateStateId`

The identifier for the private state of the contract.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>
