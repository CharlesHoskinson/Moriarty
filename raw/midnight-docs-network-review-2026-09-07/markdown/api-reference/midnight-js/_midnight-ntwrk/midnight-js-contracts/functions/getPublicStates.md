# getPublicStates

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / getPublicStates

# Function: getPublicStates()

> **getPublicStates**(`publicDataProvider`, `contractAddress`): `Promise`<[`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md)>

Fetches only the public visible (Zswap and ledger) states of a contract.

## Parameters[​](#parameters "Direct link to Parameters")

### publicDataProvider[​](#publicdataprovider "Direct link to publicDataProvider")

[`PublicDataProvider`](#)

The provider to use to fetch the public states (Zswap and ledger) from the blockchain.

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The ledger address of the contract.

## Returns[​](#returns "Direct link to Returns")

`Promise`<[`PublicContractStates`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/PublicContractStates.md)>
