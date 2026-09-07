# InvalidProtocolSchemeError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / InvalidProtocolSchemeError

# Class: InvalidProtocolSchemeError

An error describing an invalid protocol scheme.

## Extends[​](#extends "Direct link to Extends")

* `Error`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new InvalidProtocolSchemeError**(`invalidScheme`, `allowableSchemes`): `InvalidProtocolSchemeError`

#### Parameters[​](#parameters "Direct link to Parameters")

##### invalidScheme[​](#invalidscheme "Direct link to invalidScheme")

`string`

The invalid scheme.

##### allowableSchemes[​](#allowableschemes "Direct link to allowableSchemes")

`string`\[]

The valid schemes that are allowed.

#### Returns[​](#returns "Direct link to Returns")

`InvalidProtocolSchemeError`

#### Overrides[​](#overrides "Direct link to Overrides")

`Error.constructor`

## Properties[​](#properties "Direct link to Properties")

### allowableSchemes[​](#allowableschemes-1 "Direct link to allowableSchemes")

> `readonly` **allowableSchemes**: `string`\[]

The valid schemes that are allowed.

***

### invalidScheme[​](#invalidscheme-1 "Direct link to invalidScheme")

> `readonly` **invalidScheme**: `string`

The invalid scheme.
