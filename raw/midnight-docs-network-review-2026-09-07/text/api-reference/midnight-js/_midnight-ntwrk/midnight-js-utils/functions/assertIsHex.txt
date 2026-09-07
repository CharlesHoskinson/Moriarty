# assertIsHex

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / assertIsHex

# Function: assertIsHex()

> **assertIsHex**(`source`, `byteLen?`): `asserts source is string`

Asserts that a string represents a hex-encoded sequence of bytes.

## Parameters[​](#parameters "Direct link to Parameters")

### source[​](#source "Direct link to source")

`string`

The source string.

### byteLen?[​](#bytelen "Direct link to byteLen?")

`number`

An optional number of bytes that `source` should represent. If not specified then any number of bytes can be represented by `source`.

## Returns[​](#returns "Direct link to Returns")

`asserts source is string`

## Throws[​](#throws "Direct link to Throws")

`Error` `byteLen` is <= zero. Valid hex-strings will be required to have at least one byte.

## Throws[​](#throws-1 "Direct link to Throws")

`TypeError` `source` is not a hex-encoded string because it:

* is empty,
* contains invalid or incomplete characters, or
* does not represent `byteLen` bytes.
