# ProofServerContainer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Interface representing a proof server container that can be started and stopped.

## Methods[​](#methods "Direct link to Methods")

### getUrl()[​](#geturl "Direct link to getUrl()")

> **getUrl**(): `string`

Gets the URL where the proof server can be accessed.

#### Returns[​](#returns "Direct link to Returns")

`string`

The URL of the proof server

***

### stop()[​](#stop "Direct link to stop()")

> **stop**(): `Promise`<`void`>

Stops the proof server container.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when the container is stopped
