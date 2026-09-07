# PublicDataProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / PublicDataProvider

# Interface: PublicDataProvider

Interface for a public data service. This service retrieves public data from the blockchain. TODO: Add timeouts or retry limits to 'watchFor' queries.

## Methods[​](#methods "Direct link to Methods")

### contractStateObservable()[​](#contractstateobservable "Direct link to contractStateObservable()")

> **contractStateObservable**(`address`, `config`): `Observable`<`ContractState`>

Creates a stream of contract states. The observable emits a value every time a state is either created or updated at the given address. Waits indefinitely for matching data to appear.

#### Parameters[​](#parameters "Direct link to Parameters")

##### address[​](#address "Direct link to address")

`string`

The address of the contract of interest.

##### config[​](#config "Direct link to config")

[`ContractStateObservableConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ContractStateObservableConfig.md)

The configuration for the observable.

#### Returns[​](#returns "Direct link to Returns")

`Observable`<`ContractState`>

***

### queryContractState()[​](#querycontractstate "Direct link to queryContractState()")

> **queryContractState**(`contractAddress`, `config?`): `Promise`<`ContractState` | `null`>

Retrieves the on-chain state of a contract. If no block hash or block height are provided, the contract state at the address in the latest block is returned. Immediately returns null if no matching data is found.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The address of the contract of interest.

##### config?[​](#config-1 "Direct link to config?")

[`BlockHeightConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHeightConfig.md) | [`BlockHashConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHashConfig.md)

The configuration of the query. If `undefined` returns the latest states.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`ContractState` | `null`>

***

### queryDeployContractState()[​](#querydeploycontractstate "Direct link to queryDeployContractState()")

> **queryDeployContractState**(`contractAddress`): `Promise`<`ContractState` | `null`>

Retrieves the contract state included in the deployment of the contract at the given contract address. Immediately returns null if no matching data is found.

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### contractAddress[​](#contractaddress-1 "Direct link to contractAddress")

`string`

The address of the contract of interest.

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`ContractState` | `null`>

***

### queryUnshieldedBalances()[​](#queryunshieldedbalances "Direct link to queryUnshieldedBalances()")

> **queryUnshieldedBalances**(`contractAddress`, `config?`): `Promise`<[`UnshieldedBalances`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md) | `null`>

Retrieves the unshielded balances associated with a specific contract address.

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### contractAddress[​](#contractaddress-2 "Direct link to contractAddress")

`string`

The address of the contract of interest.

##### config?[​](#config-2 "Direct link to config?")

[`BlockHeightConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHeightConfig.md) | [`BlockHashConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHashConfig.md)

The configuration of the query. If `undefined` returns the latest states.

#### Returns[​](#returns-3 "Direct link to Returns")

`Promise`<[`UnshieldedBalances`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md) | `null`>

***

### queryZSwapAndContractState()[​](#queryzswapandcontractstate "Direct link to queryZSwapAndContractState()")

> **queryZSwapAndContractState**(`contractAddress`, `config?`): `Promise`<\[`ZswapChainState`, `ContractState`, `LedgerParameters`] | `null`>

Retrieves the zswap chain state (token balances), the contract state of the contract at the given address, and the ledger parameters in effect on the associated block. Both states are retrieved in a single query to ensure consistency between the two. Immediately returns null if no matching data is found.

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### contractAddress[​](#contractaddress-3 "Direct link to contractAddress")

`string`

The address of the contract of interest.

##### config?[​](#config-3 "Direct link to config?")

[`BlockHeightConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHeightConfig.md) | [`BlockHashConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/BlockHashConfig.md)

The configuration of the query. If `undefined` returns the latest states.

#### Returns[​](#returns-4 "Direct link to Returns")

`Promise`<\[`ZswapChainState`, `ContractState`, `LedgerParameters`] | `null`>

***

### unshieldedBalancesObservable()[​](#unshieldedbalancesobservable "Direct link to unshieldedBalancesObservable()")

> **unshieldedBalancesObservable**(`address`, `config`): `Observable`<[`UnshieldedBalances`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md)>

Retrieves an observable that tracks the unshielded balances for a specific contract address.

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### address[​](#address-1 "Direct link to address")

`string`

The contract address for which unshielded balances are being observed.

##### config[​](#config-4 "Direct link to config")

[`ContractStateObservableConfig`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/ContractStateObservableConfig.md)

The configuration object for observing contract state changes.

#### Returns[​](#returns-5 "Direct link to Returns")

`Observable`<[`UnshieldedBalances`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md)>

An observable that emits the unshielded balances for the provided address.

***

### watchForContractState()[​](#watchforcontractstate "Direct link to watchForContractState()")

> **watchForContractState**(`contractAddress`): `Promise`<`ContractState`>

Retrieves the contract state of the contract with the given address. Waits indefinitely for matching data to appear.

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### contractAddress[​](#contractaddress-4 "Direct link to contractAddress")

`string`

The address of the contract of interest.

#### Returns[​](#returns-6 "Direct link to Returns")

`Promise`<`ContractState`>

***

### watchForDeployTxData()[​](#watchfordeploytxdata "Direct link to watchForDeployTxData()")

> **watchForDeployTxData**(`contractAddress`): `Promise`<[`FinalizedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/FinalizedTxData.md)>

Retrieves data of the deployment transaction for the contract at the given contract address.

**IMPORTANT: This method waits indefinitely** until the deployment transaction appears on the blockchain. It will never timeout or reject unless an error occurs.

Custom implementations MUST maintain this indefinite waiting behavior to ensure consistency across all PublicDataProvider implementations. Do not implement timeouts in this method.

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### contractAddress[​](#contractaddress-5 "Direct link to contractAddress")

`string`

The address of the contract of interest.

#### Returns[​](#returns-7 "Direct link to Returns")

`Promise`<[`FinalizedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/FinalizedTxData.md)>

A promise that resolves with finalized transaction data when the deployment appears on-chain. The promise never rejects due to timeout.

***

### watchForTxData()[​](#watchfortxdata "Direct link to watchForTxData()")

> **watchForTxData**(`txId`): `Promise`<[`FinalizedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/FinalizedTxData.md)>

Retrieves data of the transaction containing the call or deployment with the given identifier.

**IMPORTANT: This method waits indefinitely** until the transaction appears on the blockchain. It will never timeout or reject unless an error occurs.

Custom implementations MUST maintain this indefinite waiting behavior to ensure consistency across all PublicDataProvider implementations. Do not implement timeouts in this method.

Applications using this method should be aware that:

* The promise will not resolve until the transaction appears on-chain
* If a transaction is invalid and never appears, this will never return
* Consider using application-level timeouts or cancellation mechanisms if needed

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### txId[​](#txid "Direct link to txId")

`string`

The identifier of the call or deployment of interest.

#### Returns[​](#returns-8 "Direct link to Returns")

`Promise`<[`FinalizedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/FinalizedTxData.md)>

A promise that resolves with finalized transaction data when the transaction appears on-chain. The promise never rejects due to timeout.

***

### watchForUnshieldedBalances()[​](#watchforunshieldedbalances "Direct link to watchForUnshieldedBalances()")

> **watchForUnshieldedBalances**(`contractAddress`): `Promise`<[`UnshieldedBalances`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md)>

Monitors for any unshielded balances associated with a specific contract address.

#### Parameters[​](#parameters-9 "Direct link to Parameters")

##### contractAddress[​](#contractaddress-6 "Direct link to contractAddress")

`string`

The address of the contract to monitor for unshielded balances.

#### Returns[​](#returns-9 "Direct link to Returns")

`Promise`<[`UnshieldedBalances`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedBalances.md)>

A promise that resolves to the detected unshielded balances.
