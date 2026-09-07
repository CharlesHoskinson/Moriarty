# levelPrivateStateProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-level-private-state-provider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider.md) / levelPrivateStateProvider

# Function: levelPrivateStateProvider()

> **levelPrivateStateProvider**<`PSI`, `PS`>(`config`): [`PrivateStateProvider`](#)<`PSI`, `PS`> & `object`

Constructs an instance of [PrivateStateProvider](#) based on [Level](#) database.

⚠️ WARNING

RISK: This provider lacks a recovery mechanism. Clearing browser cache or deleting local files permanently destroys the private state (contract state/keys). For assets with real-world value, this may result in irreversible financial loss. DO NOT use for production applications requiring data persistence.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PSI[​](#psi "Direct link to PSI")

`PSI` *extends* `string`

### PS[​](#ps "Direct link to PS")

`PS` = `any`

## Parameters[​](#parameters "Direct link to Parameters")

### config[​](#config "Direct link to config")

`Partial`<[`LevelPrivateStateProviderConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/LevelPrivateStateProviderConfig.md)> & `Pick`<[`LevelPrivateStateProviderConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-level-private-state-provider/interfaces/LevelPrivateStateProviderConfig.md), `"privateStoragePasswordProvider"` | `"accountId"`>

Database configuration options.

## Returns[​](#returns "Direct link to Returns")

[`PrivateStateProvider`](#)<`PSI`, `PS`> & `object`
