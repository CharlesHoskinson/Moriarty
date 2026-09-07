# initializeMidnightProviders

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

> **initializeMidnightProviders**<`PCK`, `PS`>(`midnightWalletProvider`, `environmentConfiguration`, `contractConfiguration`): `MidnightProviders`<`PCK`, `string`, `PS`>

Configures and returns the required providers for a Midnight contract.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

Type parameter for the input circuit key string

### PS[​](#ps "Direct link to PS")

`PS`

Type parameter for the private state

## Parameters[​](#parameters "Direct link to Parameters")

### midnightWalletProvider[​](#midnightwalletprovider "Direct link to midnightWalletProvider")

[`MidnightWalletProvider`](/api-reference/testkit-js/classes/MidnightWalletProvider.md)

The midnightWalletProvider provider instance to use for transactions

### environmentConfiguration[​](#environmentconfiguration "Direct link to environmentConfiguration")

[`EnvironmentConfiguration`](/api-reference/testkit-js/interfaces/EnvironmentConfiguration.md)

Configuration for the environment including indexer and proof server details

### contractConfiguration[​](#contractconfiguration "Direct link to contractConfiguration")

[`ContractConfiguration`](/api-reference/testkit-js/interfaces/ContractConfiguration.md)

Configuration specific to the contract including storage names and ZK config path

## Returns[​](#returns "Direct link to Returns")

`MidnightProviders`<`PCK`, `string`, `PS`>

An object containing all configured providers:

* privateStateProvider: For managing private contract state
* publicDataProvider: For accessing public blockchain data
* zkConfigProvider: For zero-knowledge proof configurations
* proofProvider: For generating and verifying proofs
* walletProvider: For midnightWalletProvider operations
* midnightProvider: For Midnight-specific operations
