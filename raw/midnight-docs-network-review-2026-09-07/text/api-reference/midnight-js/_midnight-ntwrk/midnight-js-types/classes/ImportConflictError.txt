# ImportConflictError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ImportConflictError

# Class: ImportConflictError

Error thrown when import conflicts with existing data and conflictStrategy is 'error'.

## Extends[​](#extends "Direct link to Extends")

* [`PrivateStateImportError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new ImportConflictError**(`conflictCount`, `entityName?`): `ImportConflictError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### conflictCount[​](#conflictcount "Direct link to conflictCount")

`number`

##### entityName?[​](#entityname "Direct link to entityName?")

`string` = `'private state'`

#### Returns[​](#returns "Direct link to Returns")

`ImportConflictError`

#### Overrides[​](#overrides "Direct link to Overrides")

[`PrivateStateImportError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md).[`constructor`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md#constructor)

## Properties[​](#properties "Direct link to Properties")

### cause?[​](#cause "Direct link to cause?")

> `readonly` `optional` **cause?**: [`PrivateStateImportErrorCause`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateImportErrorCause.md)

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`PrivateStateImportError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md).[`cause`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md#cause)

***

### conflictCount[​](#conflictcount-1 "Direct link to conflictCount")

> `readonly` **conflictCount**: `number`
