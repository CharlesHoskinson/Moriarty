# assertSemVer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / assertSemVer

# Function: assertSemVer()

> **assertSemVer**(`version`, `label`): `void`

Asserts that `version` is a valid SemVer-style version string of the shape `MAJOR.MINOR.PATCH` with an optional pre-release suffix (`-[A-Za-z0-9._-]+`). Build metadata (`+...`) is intentionally not supported because compactc releases do not use it.

## Parameters[​](#parameters "Direct link to Parameters")

### version[​](#version "Direct link to version")

`string`

The version string to validate.

### label[​](#label "Direct link to label")

`string`

Human-readable name of the parameter (for error messages).

## Returns[​](#returns "Direct link to Returns")

`void`

## Throws[​](#throws "Direct link to Throws")

Error if `version` is not SemVer-shaped.
