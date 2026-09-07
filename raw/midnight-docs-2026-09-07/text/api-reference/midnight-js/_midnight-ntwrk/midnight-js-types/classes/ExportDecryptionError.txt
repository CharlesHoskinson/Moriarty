# ExportDecryptionError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / ExportDecryptionError

# Class: ExportDecryptionError

Error thrown when decryption of export data fails. This could be due to wrong password, corrupted data, or tampered content. The specific cause is intentionally not disclosed to prevent oracle attacks.

## Extends[​](#extends "Direct link to Extends")

* [`PrivateStateImportError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new ExportDecryptionError**(): `ExportDecryptionError`

#### Returns[​](#returns "Direct link to Returns")

`ExportDecryptionError`

#### Overrides[​](#overrides "Direct link to Overrides")

[`PrivateStateImportError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md).[`constructor`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md#constructor)

## Properties[​](#properties "Direct link to Properties")

### cause?[​](#cause "Direct link to cause?")

> `readonly` `optional` **cause?**: [`PrivateStateImportErrorCause`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/PrivateStateImportErrorCause.md)

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`PrivateStateImportError`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md).[`cause`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/classes/PrivateStateImportError.md#cause)
