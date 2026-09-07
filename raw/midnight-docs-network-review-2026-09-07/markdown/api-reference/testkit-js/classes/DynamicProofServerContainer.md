# DynamicProofServerContainer

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

A proof server container that is started and stopped dynamically by the test suite on random port.

## Implements[​](#implements "Direct link to Implements")

* [`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md)

## Properties[​](#properties "Direct link to Properties")

### dockerEnv[​](#dockerenv "Direct link to dockerEnv")

> **dockerEnv**: `StartedDockerComposeEnvironment`

The Docker Compose environment running the container

## Methods[​](#methods "Direct link to Methods")

### getMappedPort()[​](#getmappedport "Direct link to getMappedPort()")

> **getMappedPort**(): `number`

Gets the mapped port number for the container.

#### Returns[​](#returns "Direct link to Returns")

`number`

The mapped port number

***

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

Stops the proof server container.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when the container is stopped

#### Implementation of[​](#implementation-of-1 "Direct link to Implementation of")

[`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md).[`stop`](/api-reference/testkit-js/interfaces/ProofServerContainer.md#stop)

***

### start()[​](#start "Direct link to start()")

> `static` **start**(`logger`, `maybeUID?`, `maybeNetworkId?`): `Promise`<`DynamicProofServerContainer`>

Starts a new proof server container.

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

##### maybeUID?[​](#maybeuid "Direct link to maybeUID?")

`string`

Optional unique identifier for the container

##### maybeNetworkId?[​](#maybenetworkid "Direct link to maybeNetworkId?")

`string`

Optional network ID for the container

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`DynamicProofServerContainer`>

A promise that resolves to the new container instance
