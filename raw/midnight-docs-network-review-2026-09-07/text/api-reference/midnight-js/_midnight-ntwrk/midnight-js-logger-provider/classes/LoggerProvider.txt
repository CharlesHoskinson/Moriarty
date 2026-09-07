# LoggerProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-logger-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-logger-provider.md) / LoggerProvider

# Class: LoggerProvider

Implementation of LoggerProvider that returns a [Logger](#) instance.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new LoggerProvider**(`logger`): `LoggerProvider`

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

[`Logger`](#)

#### Returns[​](#returns "Direct link to Returns")

`LoggerProvider`

## Properties[​](#properties "Direct link to Properties")

### debug[​](#debug "Direct link to debug")

> **debug**: `LogFn`

***

### error[​](#error "Direct link to error")

> **error**: `LogFn`

***

### fatal[​](#fatal "Direct link to fatal")

> **fatal**: `LogFn`

***

### info[​](#info "Direct link to info")

> **info**: `LogFn`

***

### trace[​](#trace "Direct link to trace")

> **trace**: `LogFn`

***

### warn[​](#warn "Direct link to warn")

> **warn**: `LogFn`

## Methods[​](#methods "Direct link to Methods")

### isLevelEnabled()[​](#islevelenabled "Direct link to isLevelEnabled()")

> **isLevelEnabled**(`level`): `boolean`

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### level[​](#level "Direct link to level")

`LogLevel`

#### Returns[​](#returns-1 "Direct link to Returns")

`boolean`
