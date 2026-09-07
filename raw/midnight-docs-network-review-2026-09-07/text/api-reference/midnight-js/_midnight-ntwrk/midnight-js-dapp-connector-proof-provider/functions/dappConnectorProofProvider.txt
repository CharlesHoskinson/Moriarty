# dappConnectorProofProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-dapp-connector-proof-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider.md) / dappConnectorProofProvider

# Function: dappConnectorProofProvider()

> **dappConnectorProofProvider**<`K`>(`api`, `zkConfigProvider`, `costModel`): `Promise`<[`ProofProvider`](#)>

Creates a [ProofProvider](#) that delegates proving to a DApp Connector wallet.

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

### costModel[​](#costmodel "Direct link to costModel")

`CostModel`

Cost model applied during transaction proving.

## Returns[​](#returns "Direct link to Returns")

`Promise`<[`ProofProvider`](#)>

A [ProofProvider](#) whose `proveTx` method delegates to the wallet.

## Remarks[​](#remarks "Direct link to Remarks")

Combines a wallet-backed [dappConnectorProvingProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-dapp-connector-proof-provider/functions/dappConnectorProvingProvider.md) with the given `costModel` to produce a transaction-level proof provider. The wallet's proving provider is obtained once during initialization and reused for all subsequent `proveTx` calls.
