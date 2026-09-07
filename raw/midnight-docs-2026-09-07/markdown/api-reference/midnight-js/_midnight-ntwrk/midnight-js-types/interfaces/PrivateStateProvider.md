# PrivateStateProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / PrivateStateProvider

# Interface: PrivateStateProvider\<PSI, PS>

Interface for a typed key-valued store containing contract private states.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PSI[​](#psi "Direct link to PSI")

`PSI` *extends* [`PrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateId.md) = [`PrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateId.md)

Parameter indicating the private state ID, sometimes a union of string literals.

### PS[​](#ps "Direct link to PS")

`PS` = `any`

Parameter indicating the private state type stored, sometimes a union of private state types.

## Methods[​](#methods "Direct link to Methods")

### clear()[​](#clear "Direct link to clear()")

> **clear**(): `Promise`<`void`>

Remove all contract private states.

#### Returns[​](#returns "Direct link to Returns")

`Promise`<`void`>

***

### clearSigningKeys()[​](#clearsigningkeys "Direct link to clearSigningKeys()")

> **clearSigningKeys**(): `Promise`<`void`>

Remove all contract signing keys.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`void`>

***

### exportPrivateStates()[​](#exportprivatestates "Direct link to exportPrivateStates()")

> **exportPrivateStates**(`options?`): `Promise`<[`PrivateStateExport`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PrivateStateExport.md)>

Export all private states as an encrypted JSON-serializable structure.

NOTE: This does NOT export signing keys for security reasons.

#### Parameters[​](#parameters "Direct link to Parameters")

##### options?[​](#options "Direct link to options?")

[`ExportPrivateStatesOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ExportPrivateStatesOptions.md)

Export options including optional custom password and state limit.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`PrivateStateExport`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PrivateStateExport.md)>

A JSON-serializable export structure that can be saved or transmitted.

#### Throws[​](#throws "Direct link to Throws")

If no states exist to export or limit exceeded.

***

### exportSigningKeys()[​](#exportsigningkeys "Direct link to exportSigningKeys()")

> **exportSigningKeys**(`options?`): `Promise`<[`SigningKeyExport`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/SigningKeyExport.md)>

Export all signing keys as an encrypted JSON-serializable structure.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### options?[​](#options-1 "Direct link to options?")

[`ExportSigningKeysOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ExportSigningKeysOptions.md)

Export options including optional custom password and key limit.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<[`SigningKeyExport`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/SigningKeyExport.md)>

A JSON-serializable export structure that can be saved or transmitted.

#### Throws[​](#throws-1 "Direct link to Throws")

If no keys exist to export or limit exceeded.

***

### get()[​](#get "Direct link to get()")

> **get**(`privateStateId`): `Promise`<`PS` | `null`>

Retrieve the private state at the given private state ID.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### privateStateId[​](#privatestateid "Direct link to privateStateId")

`PSI`

The private state identifier.

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`PS` | `null`>

***

### getSigningKey()[​](#getsigningkey "Direct link to getSigningKey()")

> **getSigningKey**(`address`): `Promise`<`string` | `null`>

Retrieve the signing key for a contract.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### address[​](#address "Direct link to address")

`string`

The address of the contract for which to get the signing key.

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<`string` | `null`>

***

### importPrivateStates()[​](#importprivatestates "Direct link to importPrivateStates()")

> **importPrivateStates**(`exportData`, `options?`): `Promise`<[`ImportPrivateStatesResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportPrivateStatesResult.md)>

Import private states from a previously exported structure.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### exportData[​](#exportdata "Direct link to exportData")

[`PrivateStateExport`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/PrivateStateExport.md)

The export data structure to import.

##### options?[​](#options-2 "Direct link to options?")

[`ImportPrivateStatesOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportPrivateStatesOptions.md)

Import options including password, conflict strategy, and state limit.

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<[`ImportPrivateStatesResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportPrivateStatesResult.md)>

Result indicating how many states were imported/skipped/overwritten.

#### Throws[​](#throws-2 "Direct link to Throws")

If decryption fails (wrong password or corrupted data).

#### Throws[​](#throws-3 "Direct link to Throws")

If the export format is invalid or unsupported.

#### Throws[​](#throws-4 "Direct link to Throws")

If conflictStrategy is 'error' and conflicts exist.

***

### importSigningKeys()[​](#importsigningkeys "Direct link to importSigningKeys()")

> **importSigningKeys**(`exportData`, `options?`): `Promise`<[`ImportSigningKeysResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportSigningKeysResult.md)>

Import signing keys from a previously exported structure.

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### exportData[​](#exportdata-1 "Direct link to exportData")

[`SigningKeyExport`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/SigningKeyExport.md)

The export data structure to import.

##### options?[​](#options-3 "Direct link to options?")

[`ImportSigningKeysOptions`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportSigningKeysOptions.md)

Import options including password, conflict strategy, and key limit.

#### Returns[​](#returns-7 "Direct link to Returns")

`Promise`<[`ImportSigningKeysResult`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ImportSigningKeysResult.md)>

Result indicating how many keys were imported/skipped/overwritten.

#### Throws[​](#throws-5 "Direct link to Throws")

If decryption fails (wrong password or corrupted data).

#### Throws[​](#throws-6 "Direct link to Throws")

If the export format is invalid or unsupported.

#### Throws[​](#throws-7 "Direct link to Throws")

If conflictStrategy is 'error' and conflicts exist.

***

### remove()[​](#remove "Direct link to remove()")

> **remove**(`privateStateId`): `Promise`<`void`>

Remove the value at the given private state ID.

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### privateStateId[​](#privatestateid-1 "Direct link to privateStateId")

`PSI`

The private state identifier.

#### Returns[​](#returns-8 "Direct link to Returns")

`Promise`<`void`>

***

### removeSigningKey()[​](#removesigningkey "Direct link to removeSigningKey()")

> **removeSigningKey**(`address`): `Promise`<`void`>

Remove the signing key for a contract.

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### address[​](#address-1 "Direct link to address")

`string`

The address of the contract for which to delete the signing key.

#### Returns[​](#returns-9 "Direct link to Returns")

`Promise`<`void`>

***

### set()[​](#set "Direct link to set()")

> **set**(`privateStateId`, `state`): `Promise`<`void`>

Store the given private state at the given private state ID.

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### privateStateId[​](#privatestateid-2 "Direct link to privateStateId")

`PSI`

The private state identifier.

##### state[​](#state "Direct link to state")

`PS`

The private state to store.

#### Returns[​](#returns-10 "Direct link to Returns")

`Promise`<`void`>

***

### setContractAddress()[​](#setcontractaddress "Direct link to setContractAddress()")

> **setContractAddress**(`address`): `void`

Set the contract address for scoping private state operations. Must be called before any get/set/remove operations on private states. This provides namespace isolation between different contracts.

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### address[​](#address-2 "Direct link to address")

`string`

The contract address to scope operations to.

#### Returns[​](#returns-11 "Direct link to Returns")

`void`

***

### setSigningKey()[​](#setsigningkey "Direct link to setSigningKey()")

> **setSigningKey**(`address`, `signingKey`): `Promise`<`void`>

Store the given signing key at the given address.

#### Parameters[​](#parameters-10 "Direct link to Parameters")

##### address[​](#address-3 "Direct link to address")

`string`

The address of the contract having the given signing key.

##### signingKey[​](#signingkey "Direct link to signingKey")

`string`

The signing key to store.

#### Returns[​](#returns-12 "Direct link to Returns")

`Promise`<`void`>
