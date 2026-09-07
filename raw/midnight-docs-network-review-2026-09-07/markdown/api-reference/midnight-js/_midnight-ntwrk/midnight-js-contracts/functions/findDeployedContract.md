# findDeployedContract

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / findDeployedContract

# Function: findDeployedContract()

Creates an instance of [FoundContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md) given the address of a deployed contract and an optional private state ID at which an existing private state is stored. When given, the current value at the private state ID is used as the `initialPrivateState` value in the `finalizedDeployTxData` property of the returned `FoundContract`.

## Param[​](#param "Direct link to Param")

The providers used to manage transaction lifecycles.

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Throws[​](#throws "Direct link to Throws")

Error Improper `privateStateId` and `initialPrivateState` configuration.

## Throws[​](#throws-1 "Direct link to Throws")

Error No contract state could be found at `contractAddress`.

## Throws[​](#throws-2 "Direct link to Throws")

TypeError Thrown if `contractAddress` is not correctly formatted as a contract address.

## Throws[​](#throws-3 "Direct link to Throws")

ContractTypeError One or more circuits defined on `contract` are undefined on the contract state found at `contractAddress`, or have mis-matched verifier keys.

## Throws[​](#throws-4 "Direct link to Throws")

IncompleteFindContractPrivateStateConfig If an `initialPrivateState` is given but no `privateStateId` is given to store it under.

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **findDeployedContract**<`C`>(`providers`, `options`): `Promise`<[`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

### Parameters[​](#parameters "Direct link to Parameters")

#### providers[​](#providers "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`, `ProvableCircuitId`<`C`>, `unknown`>

#### options[​](#options "Direct link to options")

[`FindDeployedContractOptionsBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsBase.md)<`C`>

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **findDeployedContract**<`C`>(`providers`, `options`): `Promise`<[`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### providers[​](#providers-1 "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

#### options[​](#options-1 "Direct link to options")

[`FindDeployedContractOptionsExistingPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsExistingPrivateState.md)<`C`>

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`>>

## Call Signature[​](#call-signature-2 "Direct link to Call Signature")

> **findDeployedContract**<`C`>(`providers`, `options`): `Promise`<[`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`>>

### Type Parameters[​](#type-parameters-2 "Direct link to Type Parameters")

#### C[​](#c-2 "Direct link to C")

`C` *extends* `Any`

### Parameters[​](#parameters-2 "Direct link to Parameters")

#### providers[​](#providers-2 "Direct link to providers")

[`ContractProviders`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractProviders.md)<`C`>

#### options[​](#options-2 "Direct link to options")

[`FindDeployedContractOptionsStorePrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FindDeployedContractOptionsStorePrivateState.md)<`C`>

### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`FoundContract`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FoundContract.md)<`C`>>
