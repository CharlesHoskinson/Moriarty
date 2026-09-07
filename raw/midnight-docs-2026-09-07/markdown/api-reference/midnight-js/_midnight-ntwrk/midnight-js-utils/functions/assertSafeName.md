# assertSafeName

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / assertSafeName

# Function: assertSafeName()

> **assertSafeName**(`name`, `label`): `void`

Asserts that `name` is safe to use as a single path segment or URL path component. Rejects traversal payloads (`.`, `..`, separators), URL-encoded characters, null bytes, whitespace, empty strings, and names longer than [MAX\_SAFE\_NAME\_LENGTH](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils/variables/MAX_SAFE_NAME_LENGTH.md).

## Parameters[​](#parameters "Direct link to Parameters")

### name[​](#name "Direct link to name")

`string`

The value to validate.

### label[​](#label "Direct link to label")

`string`

Human-readable name of the parameter (for error messages).

## Returns[​](#returns "Direct link to Returns")

`void`

## Throws[​](#throws "Direct link to Throws")

Error if `name` fails validation.
