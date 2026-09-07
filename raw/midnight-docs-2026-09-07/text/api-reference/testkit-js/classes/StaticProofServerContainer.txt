# StaticProofServerContainer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

A proof server that is currently running on a specific port. Used for connecting to an existing proof server instance.

## Implements[​](#implements "Direct link to Implements")

* [`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new StaticProofServerContainer**(`port?`): `StaticProofServerContainer`

Creates a new StaticProofServerContainer instance.

#### Parameters[​](#parameters "Direct link to Parameters")

##### port?[​](#port "Direct link to port?")

`number` = `6300`

The port number where the proof server is running (default: 6300)

#### Returns[​](#returns "Direct link to Returns")

`StaticProofServerContainer`

## Properties[​](#properties "Direct link to Properties")

### port[​](#port-1 "Direct link to port")

> **port**: `number`

The port number where the proof server is running

## Methods[​](#methods "Direct link to Methods")

### getUrl()[​](#geturl "Direct link to getUrl()")

> **getUrl**(): `string`

Gets the URL where the proof server can be accessed.

#### Returns[​](#returns-1 "Direct link to Returns")

`string`

The URL of the proof server

#### Implementation of[​](#implementation-of "Direct link to Implementation of")

[`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md).[`getUrl`](/api-reference/testkit-js/interfaces/ProofServerContainer.md#geturl)

***

### stop()[​](#stop "Direct link to stop()")

> **stop**(): `Promise`<`void`>

No-op stop method since this represents an external proof server.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`void`>

A resolved promise

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

[`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md).[`stop`](/api-reference/testkit-js/interfaces/ProofServerContainer.md#stop)
