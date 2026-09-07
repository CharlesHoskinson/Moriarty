# ParsedHexString

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / ParsedHexString

# Type Alias: ParsedHexString

> **ParsedHexString** = `object`

The result of parsing a string as a hex-encoded string.

## Properties[​](#properties "Direct link to Properties")

### byteChars[​](#bytechars "Direct link to byteChars")

> `readonly` **byteChars**: `string`

The captured sequence of *whole* bytes found in the source string.

***

### hasPrefix[​](#hasprefix "Direct link to hasPrefix")

> `readonly` **hasPrefix**: `boolean`

A flag indicating if the hex-string has a `'0x'` prefix.

***

### incompleteChars[​](#incompletechars "Direct link to incompleteChars")

> `readonly` **incompleteChars**: `string`

The remaining characters of incomplete bytes and/or the non hexadecimal characters found in the source string.
