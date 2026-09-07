# isHex

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / isHex

# Function: isHex()

> **isHex**(`source`, `byteLen?`): `boolean`

Determines if a string represents a hex-encoded sequence of bytes.

## Parameters[​](#parameters "Direct link to Parameters")

### source[​](#source "Direct link to source")

`string`

The source string.

### byteLen?[​](#bytelen "Direct link to byteLen?")

`number`

An optional number of bytes that `source` should represent. If not specified then any number of bytes can be represented by `source`.

## Returns[​](#returns "Direct link to Returns")

`boolean`

`true` if the `source` string is parsable as a hex-string, of non-zero length, and of the optional byte length of `byteLen`; otherwise `false`.
