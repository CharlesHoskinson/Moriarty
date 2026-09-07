# getStates

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / getStates

# Function: getStates()

> **getStates**<`PS`>(`publicDataProvider`, `privateStateProvider`, `contractAddress`, `privateStateId`): `Promise`<[`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PS`>>

Retrieves the Zswap, ledger, and private states of the contract corresponding to the given identifier using the given providers.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PS[​](#ps "Direct link to PS")

`PS`

## Parameters[​](#parameters "Direct link to Parameters")

### publicDataProvider[​](#publicdataprovider "Direct link to publicDataProvider")

[`PublicDataProvider`](#)

The provider to use to fetch the public states (Zswap and ledger) from the blockchain.

### privateStateProvider[​](#privatestateprovider "Direct link to privateStateProvider")

[`PrivateStateProvider`](#)<`string`, `PS`>

The provider to use to fetch the private state.

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The ledger address of the contract.

### privateStateId[​](#privatestateid "Direct link to privateStateId")

`string`

The identifier for the private state of the contract.

## Returns[​](#returns "Direct link to Returns")

`Promise`<[`ContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractStates.md)<`PS`>>
