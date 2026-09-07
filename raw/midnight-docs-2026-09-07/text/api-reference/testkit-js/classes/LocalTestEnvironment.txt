# LocalTestEnvironment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Test environment for local development using Docker containers Manages containers for node, indexer and proof server components

## Extends[​](#extends "Direct link to Extends")

* [`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new LocalTestEnvironment**(`logger`): `LocalTestEnvironment`

Creates a new LocalTestEnvironment instance

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

#### Returns[​](#returns "Direct link to Returns")

`LocalTestEnvironment`

#### Overrides[​](#overrides "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`constructor`](/api-reference/testkit-js/classes/TestEnvironment.md#constructor)

## Properties[​](#properties "Direct link to Properties")

### dockerEnv[​](#dockerenv "Direct link to dockerEnv")

> **dockerEnv**: `StartedDockerComposeEnvironment`

***

### genesisMintWalletSeed[​](#genesismintwalletseed "Direct link to genesisMintWalletSeed")

> `readonly` **genesisMintWalletSeed**: `string`\[]

***

### MAX\_NUMBER\_OF\_WALLETS[​](#max_number_of_wallets "Direct link to MAX_NUMBER_OF_WALLETS")

> `readonly` `static` **MAX\_NUMBER\_OF\_WALLETS**: `4` = `4`

## Methods[​](#methods "Direct link to Methods")

### getEnvironmentConfiguration()[​](#getenvironmentconfiguration "Direct link to getEnvironmentConfiguration()")

> **getEnvironmentConfiguration**(): [`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Returns the configuration for the testnet environment services.

#### Returns[​](#returns-1 "Direct link to Returns")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Object containing URLs for testnet services:

* indexer: GraphQL API endpoint for the indexer
* indexerWS: WebSocket endpoint for the indexer
* node: RPC endpoint for the blockchain node
* faucet: API endpoint for requesting test tokens
* proofServer: URL for the proof generation server

#### Overrides[​](#overrides-1 "Direct link to Overrides")

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

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`getMidnightWalletProvider`](/api-reference/testkit-js/classes/TestEnvironment.md#getmidnightwalletprovider)

***

### shutdown()[​](#shutdown "Direct link to shutdown()")

> **shutdown**(`saveWalletState?`): `Promise`<`void`>

Shuts down the test environment, closing walletProviders and stopping containers

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### saveWalletState?[​](#savewalletstate "Direct link to saveWalletState?")

`boolean`

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`void`>

#### Overrides[​](#overrides-2 "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`shutdown`](/api-reference/testkit-js/classes/TestEnvironment.md#shutdown)

***

### start()[​](#start "Direct link to start()")

> **start**(`maybeProofServerContainer?`): `Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

Starts the test environment by creating and configuring Docker containers

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### maybeProofServerContainer?[​](#maybeproofservercontainer "Direct link to maybeProofServerContainer?")

[`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md)

Optional proof server container

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

The environment configuration

#### Throws[​](#throws-1 "Direct link to Throws")

If trying to inject proof server container when starting new environment

#### Overrides[​](#overrides-3 "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`start`](/api-reference/testkit-js/classes/TestEnvironment.md#start)

***

### startMidnightWalletProviders()[​](#startmidnightwalletproviders "Direct link to startMidnightWalletProviders()")

> **startMidnightWalletProviders**(`amount?`, `seeds?`): `Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)\[]>

Creates and starts the specified number of wallet providers

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### amount?[​](#amount "Direct link to amount?")

`number` = `1`

##### seeds?[​](#seeds "Direct link to seeds?")

`string`\[] | `undefined`

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)\[]>

A promise that resolves to an array of started wallets

#### Throws[​](#throws-2 "Direct link to Throws")

If requested amount exceeds maximum supported walletProviders

#### Overrides[​](#overrides-4 "Direct link to Overrides")

[`TestEnvironment`](/api-reference/testkit-js/classes/TestEnvironment.md).[`startMidnightWalletProviders`](/api-reference/testkit-js/classes/TestEnvironment.md#startmidnightwalletproviders)

***

### startWithInjectedEnvironment()[​](#startwithinjectedenvironment "Direct link to startWithInjectedEnvironment()")

> **startWithInjectedEnvironment**(`dockerEnv`, `ports`): `Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

Instead of starting the test environment by building the docker containers from the default configuration files in this package, start the test environment by passing an existing StartedDockerComposeEnvironment along with the ports for the containers in the environment.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### dockerEnv[​](#dockerenv-1 "Direct link to dockerEnv")

`StartedDockerComposeEnvironment`

A started docker compose environment

##### ports[​](#ports "Direct link to ports")

[`ComponentPortsConfiguration`](/api-reference/testkit-js/type-aliases/ComponentPortsConfiguration.md)

The ports of the containers in the given environment

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

The environment configuration
