# TransactionContext

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / TransactionContext

# Interface: TransactionContext\<C, PCK>

Encapsulates the context for managing a scoped contract transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`> = `Contract.ProvableCircuitId`<`C`>

## Properties[​](#properties "Direct link to Properties")

### \[CacheStates][​](#cachestates "Direct link to \[CacheStates]")

> `readonly` **\[CacheStates]**: (`states`, `identity`) => `void`

#### Parameters[​](#parameters "Direct link to Parameters")

##### states[​](#states "Direct link to states")

[`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md) | [`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PrivateState`<`C`>>

##### identity[​](#identity "Direct link to identity")

`CachedStateIdentity`

#### Returns[​](#returns "Direct link to Returns")

`void`

***

### \[GetCurrentStatesForIdentity][​](#getcurrentstatesforidentity "Direct link to \[GetCurrentStatesForIdentity]")

> `readonly` **\[GetCurrentStatesForIdentity]**: (`identity`) => [`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md) | [`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PrivateState`<`C`>> | `undefined`

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### identity[​](#identity-1 "Direct link to identity")

`CachedStateIdentity`

#### Returns[​](#returns-1 "Direct link to Returns")

[`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md) | [`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PrivateState`<`C`>> | `undefined`

***

### \[MergeUnsubmittedCallTxData][​](#mergeunsubmittedcalltxdata "Direct link to \[MergeUnsubmittedCallTxData]")

> `readonly` **\[MergeUnsubmittedCallTxData]**: (`circuitId`, `callData`, `privateStateId?`) => `void`

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### circuitId[​](#circuitid "Direct link to circuitId")

`PCK`

##### callData[​](#calldata "Direct link to callData")

[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>

##### privateStateId?[​](#privatestateid "Direct link to privateStateId?")

`string`

#### Returns[​](#returns-2 "Direct link to Returns")

`void`

***

### \[Submit][​](#submit "Direct link to \[Submit]")

> `readonly` **\[Submit]**: () => `Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<[`FinalizedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/FinalizedCallTxData.md)<`C`, `PCK`>>

***

### \[TypeId][​](#typeid "Direct link to \[TypeId]")

> `readonly` **\[TypeId]**: *typeof* `TypeId`

## Methods[​](#methods "Direct link to Methods")

### getAdditionalMappings()[​](#getadditionalmappings "Direct link to getAdditionalMappings()")

> **getAdditionalMappings**(): `ReadonlyMap`<`string`, `string`> | `undefined`

Gets the additional scoped CoinPublicKey to EncPublicKey mappings.

#### Returns[​](#returns-4 "Direct link to Returns")

`ReadonlyMap`<`string`, `string`> | `undefined`

A `ReadonlyMap <CoinPublicKey, EncPublicKey>` instance, or `undefined` if no additional mappings were specified for the current transaction context.

***

### getCurrentStates()[​](#getcurrentstates "Direct link to getCurrentStates()")

> **getCurrentStates**(): [`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md) | [`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PrivateState`<`C`>> | `undefined`

Gets the current cached contract states within the transaction context.

#### Returns[​](#returns-5 "Direct link to Returns")

[`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md) | [`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PrivateState`<`C`>> | `undefined`

A cached [ContractStates](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md) instance, or `undefined` if circuit calls are yet to be made.

#### Remarks[​](#remarks "Direct link to Remarks")

The returned states represent the unsubmitted *running* state of the contract within the transaction context, reflecting any unsubmitted circuit calls made to the contract during the scope of the transaction.

***

### getLastUnsubmittedCallTxDataToTransact()[​](#getlastunsubmittedcalltxdatatotransact "Direct link to getLastUnsubmittedCallTxDataToTransact()")

> **getLastUnsubmittedCallTxDataToTransact**(): \[[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>, `string`?] | `undefined`

Gets the last unsubmitted call transaction data.

#### Returns[​](#returns-6 "Direct link to Returns")

\[[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>, `string`?] | `undefined`

A tuple containing an [UnsubmittedCallTxData](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md) instance, and an optional private state ID, or `undefined` if circuit calls are yet to be made.
