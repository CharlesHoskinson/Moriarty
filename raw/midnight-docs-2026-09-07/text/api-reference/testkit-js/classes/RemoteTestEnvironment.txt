# RemoteTestEnvironment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Base class for remote test environments that connect to external network services. Provides functionality for managing walletProviders and a proof server container.

## Extends[​](#extends "Direct link to Extends")

* [`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md)

## Extended by[​](#extended-by "Direct link to Extended by")

* [`EnvVarRemoteTestEnvironment`](/api-reference/testkit-js/classes/EnvVarRemoteTestEnvironment.md)
* [`PreprodTestEnvironment`](/api-reference/testkit-js/classes/PreprodTestEnvironment.md)
* [`PreviewTestEnvironment`](/api-reference/testkit-js/classes/PreviewTestEnvironment.md)
* [`QanetTestEnvironment`](/api-reference/testkit-js/classes/QanetTestEnvironment.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new RemoteTestEnvironment**(`logger`): `RemoteTestEnvironment`

Creates a new TestEnvironment instance.

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

#### Returns[​](#returns "Direct link to Returns")

`RemoteTestEnvironment`

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`constructor`](/api-reference/testkit-js/classes/TestEnvironment.md#constructor)

## Methods[​](#methods "Direct link to Methods")

### getEnvironmentConfiguration()[​](#getenvironmentconfiguration "Direct link to getEnvironmentConfiguration()")

> `abstract` **getEnvironmentConfiguration**(): [`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Abstract method that must be implemented by subclasses to provide environment configuration.

#### Returns[​](#returns-1 "Direct link to Returns")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Configuration object containing service URLs and endpoints

#### Overrides[​](#overrides "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`getEnvironmentConfiguration`](/api-reference/testkit-js/classes/TestEnvironment.md#getenvironmentconfiguration)

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

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`getMidnightWalletProvider`](/api-reference/testkit-js/classes/TestEnvironment.md#getmidnightwalletprovider)

***

### healthCheck()[​](#healthcheck "Direct link to healthCheck()")

> **healthCheck**(): `Promise`<`void`>

Performs a health check for the environment. Checks the health of the node, indexer, and optionally the faucet services.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when the health check is complete.

***

### shutdown()[​](#shutdown "Direct link to shutdown()")

> **shutdown**(`saveWalletState?`): `Promise`<`void`>

Shuts down the test environment by closing all walletProviders and stopping the proof server.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### saveWalletState?[​](#savewalletstate "Direct link to saveWalletState?")

`boolean`

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<`void`>

#### Overrides[​](#overrides-1 "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`shutdown`](/api-reference/testkit-js/classes/TestEnvironment.md#shutdown)

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

#### Overrides[​](#overrides-2 "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`start`](/api-reference/testkit-js/classes/TestEnvironment.md#start)

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

#### Overrides[​](#overrides-3 "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`startMidnightWalletProviders`](/api-reference/testkit-js/classes/TestEnvironment.md#startmidnightwalletproviders)
