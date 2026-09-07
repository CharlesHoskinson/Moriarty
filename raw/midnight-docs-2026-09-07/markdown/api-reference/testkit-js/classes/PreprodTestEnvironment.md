# PreprodTestEnvironment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Test environment configuration for the Midnight preprod network. Provides URLs and endpoints for preprod network services.

## Extends[​](#extends "Direct link to Extends")

* [`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new PreprodTestEnvironment**(`logger`): `PreprodTestEnvironment`

Creates a new TestEnvironment instance.

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

#### Returns[​](#returns "Direct link to Returns")

`PreprodTestEnvironment`

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`constructor`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#constructor)

## Methods[​](#methods "Direct link to Methods")

### getEnvironmentConfiguration()[​](#getenvironmentconfiguration "Direct link to getEnvironmentConfiguration()")

> **getEnvironmentConfiguration**(): [`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Returns the configuration for the preprod environment services.

#### Returns[​](#returns-1 "Direct link to Returns")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Object containing URLs for preprod services:

* indexer: GraphQL API endpoint for the indexer
* indexerWS: WebSocket endpoint for the indexer
* node: RPC endpoint for the blockchain node
* faucet: API endpoint for requesting test tokens
* proofServer: URL for the proof generation server

#### Overrides[​](#overrides "Direct link to Overrides")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`getEnvironmentConfiguration`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#getenvironmentconfiguration)

***

### getMidnightWalletProvider()[​](#getmidnightwalletprovider "Direct link to getMidnightWalletProvider()")

> **getMidnightWalletProvider**(): `Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)>

Starts a single wallet instance.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)>

A promise that resolves to the started wallet

#### Throws[​](#throws "Direct link to Throws")

If no wallet could be started

#### Inherited from[​](#inherited-from-1 "Direct link to Inherited from")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`getMidnightWalletProvider`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#getmidnightwalletprovider)

***

### healthCheck()[​](#healthcheck "Direct link to healthCheck()")

> **healthCheck**(): `Promise`<`void`>

Performs a health check for the environment. Checks the health of the node, indexer, and optionally the faucet services.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when the health check is complete.

#### Inherited from[​](#inherited-from-2 "Direct link to Inherited from")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`healthCheck`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#healthcheck)

***

### shutdown()[​](#shutdown "Direct link to shutdown()")

> **shutdown**(`saveWalletState?`): `Promise`<`void`>

Shuts down the test environment by closing all walletProviders and stopping the proof server.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### saveWalletState?[​](#savewalletstate "Direct link to saveWalletState?")

`boolean`

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`void`>

#### Inherited from[​](#inherited-from-3 "Direct link to Inherited from")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`shutdown`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#shutdown)

***

### start()[​](#start "Direct link to start()")

> **start**(`maybeProofServerContainer?`): `Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

Starts the test environment by initializing the proof server and environment configuration.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### maybeProofServerContainer?[​](#maybeproofservercontainer "Direct link to maybeProofServerContainer?")

[`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md)

Optional proof server container to use instead of creating a new one

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

The environment configuration

#### Inherited from[​](#inherited-from-4 "Direct link to Inherited from")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`start`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#start)

***

### startMidnightWalletProviders()[​](#startmidnightwalletproviders "Direct link to startMidnightWalletProviders()")

> **startMidnightWalletProviders**(`amount?`, `seeds?`): `Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)\[]>

Creates and starts the specified number of wallet providers.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### amount?[​](#amount "Direct link to amount?")

`number` = `1`

##### seeds?[​](#seeds "Direct link to seeds?")

`string`\[] | `undefined`

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)\[]>

Array of started wallet providers

#### Inherited from[​](#inherited-from-5 "Direct link to Inherited from")

[`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md).[`startMidnightWalletProviders`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md#startmidnightwalletproviders)
