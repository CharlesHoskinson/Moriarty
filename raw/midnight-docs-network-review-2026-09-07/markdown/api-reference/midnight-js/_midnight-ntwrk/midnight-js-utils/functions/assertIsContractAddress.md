# assertIsContractAddress

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / assertIsContractAddress

# Function: assertIsContractAddress()

> **assertIsContractAddress**(`contractAddress`): `asserts contractAddress is string`

**`Internal`**

Asserts that a string represents a hex-encoded contract address.

## Parameters[​](#parameters "Direct link to Parameters")

### contractAddress[​](#contractaddress "Direct link to contractAddress")

`string`

The source string.

## Returns[​](#returns "Direct link to Returns")

`asserts contractAddress is string`

## Throws[​](#throws "Direct link to Throws")

`TypeError` `contractAddress` is not a correctly formatted [ContractAddress](#).
