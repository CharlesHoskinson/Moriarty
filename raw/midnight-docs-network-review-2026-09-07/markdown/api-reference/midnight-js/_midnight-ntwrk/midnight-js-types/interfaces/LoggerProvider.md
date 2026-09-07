# LoggerProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / LoggerProvider

# Interface: LoggerProvider

A provider for logging functions.

## Properties[​](#properties "Direct link to Properties")

### debug?[​](#debug "Direct link to debug?")

> `optional` **debug?**: `LogFn`

***

### error?[​](#error "Direct link to error?")

> `optional` **error?**: `LogFn`

***

### fatal?[​](#fatal "Direct link to fatal?")

> `optional` **fatal?**: `LogFn`

***

### info?[​](#info "Direct link to info?")

> `optional` **info?**: `LogFn`

***

### warn?[​](#warn "Direct link to warn?")

> `optional` **warn?**: `LogFn`

## Methods[​](#methods "Direct link to Methods")

### isLevelEnabled()[​](#islevelenabled "Direct link to isLevelEnabled()")

> **isLevelEnabled**(`level`): `boolean`

#### Parameters[​](#parameters "Direct link to Parameters")

##### level[​](#level "Direct link to level")

[`LogLevel`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/enumerations/LogLevel.md)

#### Returns[​](#returns "Direct link to Returns")

`boolean`
