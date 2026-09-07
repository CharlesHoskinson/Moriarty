# dappConnectorProvingProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-dapp-connector-proof-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider.md) / dappConnectorProvingProvider

# Function: dappConnectorProvingProvider()

> **dappConnectorProvingProvider**<`K`>(`api`, `zkConfigProvider`): `Promise`<`ProvingProvider`>

Obtains a ProvingProvider from the DApp Connector wallet.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### K[​](#k "Direct link to K")

`K` *extends* `string`

Union of circuit identifier strings defined by the contract.

## Parameters[​](#parameters "Direct link to Parameters")

### api[​](#api "Direct link to api")

[`DAppConnectorProvingAPI`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider/type-aliases/DAppConnectorProvingAPI.md)

DApp Connector wallet API exposing `getProvingProvider`.

### zkConfigProvider[​](#zkconfigprovider "Direct link to zkConfigProvider")

[`ZKConfigProvider`](#)<`K`>

Provider that supplies ZK configuration artifacts and key material.

## Returns[​](#returns "Direct link to Returns")

`Promise`<`ProvingProvider`>

A ProvingProvider backed by the wallet.

## Remarks[​](#remarks "Direct link to Remarks")

Extracts key material from the given `zkConfigProvider` and passes it to the wallet's `getProvingProvider` method. Use this when you need direct, circuit-level access to the wallet's proving capabilities without cost model integration.
