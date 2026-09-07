# getUnshieldedBalances

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / getUnshieldedBalances

# Function: getUnshieldedBalances()

> **getUnshieldedBalances**(`publicDataProvider`, `contractAddress`): `Promise`<`UnshieldedBalances`>

Fetches the unshielded balances associated with a specific contract address.

## Parameters[​](#parameters "Direct link to Parameters")

### publicDataProvider[​](#publicdataprovider "Direct link to publicDataProvider")

[`PublicDataProvider`](#)

The provider to use to fetch the unshielded balances from the blockchain.

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The ledger address of the contract.

## Returns[​](#returns "Direct link to Returns")

`Promise`<`UnshieldedBalances`>
