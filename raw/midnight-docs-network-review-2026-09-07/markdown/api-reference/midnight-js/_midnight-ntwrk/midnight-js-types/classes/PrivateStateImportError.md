# PrivateStateImportError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / PrivateStateImportError

# Class: PrivateStateImportError

Base error thrown when importing private states fails.

## Extends[​](#extends "Direct link to Extends")

* `Error`

## Extended by[​](#extended-by "Direct link to Extended by")

* [`ExportDecryptionError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ExportDecryptionError.md)
* [`InvalidExportFormatError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/InvalidExportFormatError.md)
* [`ImportConflictError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/ImportConflictError.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new PrivateStateImportError**(`message`, `cause?`): `PrivateStateImportError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### message[​](#message "Direct link to message")

`string`

##### cause?[​](#cause "Direct link to cause?")

[`PrivateStateImportErrorCause`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateImportErrorCause.md)

#### Returns[​](#returns "Direct link to Returns")

`PrivateStateImportError`

#### Overrides[​](#overrides "Direct link to Overrides")

`Error.constructor`

## Properties[​](#properties "Direct link to Properties")

### cause?[​](#cause-1 "Direct link to cause?")

> `readonly` `optional` **cause?**: [`PrivateStateImportErrorCause`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateImportErrorCause.md)

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

`Error.cause`
