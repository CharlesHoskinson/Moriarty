# parseHex

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / parseHex

# Function: parseHex()

> **parseHex**(`source`): [`ParsedHexString`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/type-aliases/ParsedHexString.md)

Parses a string as a hex-encoded string.

## Parameters[​](#parameters "Direct link to Parameters")

### source[​](#source "Direct link to source")

`string`

The source string to parse.

## Returns[​](#returns "Direct link to Returns")

[`ParsedHexString`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/type-aliases/ParsedHexString.md)

A [ParsedHexString](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/type-aliases/ParsedHexString.md) describing the parsed elements of `source`.

## Examples[​](#examples "Direct link to Examples")

```
parseHex('Hello') =>

  {

    hasPrefix: false,

    incompleteChars: 'Hello'

  }
```

```
parseHex('ab12e') =>

  {

    hasPrefix: false,

    byteChars: 'ab12'

    incompleteChars: 'e'

  }
```

```
parseHex('0xab12') =>

  {

    hasPrefix: true,

    byteChars: 'ab12'

    incompleteChars: ''

  }
```
