# TestEnvironment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Abstract base class for test environments. Provides common functionality for managing test wallets and environments.

## Extended by[​](#extended-by "Direct link to Extended by")

* [`LocalTestEnvironment`](/api-reference/testkit-js/classes/LocalTestEnvironment.md)
* [`RemoteTestEnvironment`](/api-reference/testkit-js/classes/RemoteTestEnvironment.md)

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new TestEnvironment**(`logger`): `TestEnvironment`

Creates a new TestEnvironment instance.

#### Parameters[​](#parameters "Direct link to Parameters")

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

#### Returns[​](#returns "Direct link to Returns")

`TestEnvironment`

## Methods[​](#methods "Direct link to Methods")

### getEnvironmentConfiguration()[​](#getenvironmentconfiguration "Direct link to getEnvironmentConfiguration()")

> `abstract` **getEnvironmentConfiguration**(): [`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

#### Returns[​](#returns-1 "Direct link to Returns")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

***

### getMidnightWalletProvider()[​](#getmidnightwalletprovider "Direct link to getMidnightWalletProvider()")

> **getMidnightWalletProvider**(): `Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)>

Starts a single wallet instance.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)>

A promise that resolves to the started wallet

#### Throws[​](#throws "Direct link to Throws")

If no wallet could be started

***

### shutdown()[​](#shutdown "Direct link to shutdown()")

> `abstract` **shutdown**(`saveWalletState?`): `Promise`<`void`>

Shuts down the test environment and cleans up resources.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### saveWalletState?[​](#savewalletstate "Direct link to saveWalletState?")

`boolean`

Optional flag to save the wallet state before shutdown

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when shutdown is complete

***

### start()[​](#start "Direct link to start()")

> `abstract` **start**(`maybeProofServerContainer?`): `Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

Start the test environment.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### maybeProofServerContainer?[​](#maybeproofservercontainer "Direct link to maybeProofServerContainer?")

[`ProofServerContainer`](/api-reference/testkit-js/interfaces/ProofServerContainer.md)

If defined, a container representing an already running proof server. If undefined, a proof server will be started automatically.

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)>

A promise that resolves to the environment configuration

***

### startMidnightWalletProviders()[​](#startmidnightwalletproviders "Direct link to startMidnightWalletProviders()")

> `abstract` **startMidnightWalletProviders**(`amount?`, `seeds?`): `Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)\[]>

Starts multiple wallet instances.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### amount?[​](#amount "Direct link to amount?")

`number`

Optional number of wallet instances to start

##### seeds?[​](#seeds "Direct link to seeds?")

`string`\[]

Optional array of seeds for the wallets

#### Returns[​](#returns-5 "Direct link to Returns")

`Promise`<[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)\[]>

A promise that resolves to an array of started wallets
