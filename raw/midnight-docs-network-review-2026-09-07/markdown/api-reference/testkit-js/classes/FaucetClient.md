# FaucetClient

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

Client for interacting with the Midnight faucet service. Provides functionality to request test tokens for wallet addresses.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new FaucetClient**(`faucetUrl`, `logger`): `FaucetClient`

Creates a new FaucetClient instance.

#### Parameters[​](#parameters "Direct link to Parameters")

##### faucetUrl[​](#fauceturl "Direct link to faucetUrl")

`string`

The URL of the faucet service endpoint

##### logger[​](#logger "Direct link to logger")

`Logger`

Logger instance for recording operations

#### Returns[​](#returns "Direct link to Returns")

`FaucetClient`

## Properties[​](#properties "Direct link to Properties")

### faucetUrl[​](#fauceturl-1 "Direct link to faucetUrl")

> `readonly` **faucetUrl**: `string`

## Methods[​](#methods "Direct link to Methods")

### health()[​](#health "Direct link to health()")

> **health**(): `Promise`<`AxiosResponse`<`any`, `any`, { }>>

Checks the health status of the faucet service. Makes a GET request to the health endpoint of the faucet service.

#### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<`AxiosResponse`<`any`, `any`, { }>>

A promise that resolves to the response of the health check or logs an error if the request fails

***

### requestTokens()[​](#requesttokens "Direct link to requestTokens()")

> **requestTokens**(`walletAddress`): `Promise`<`void`>

Requests test tokens from the faucet for a specified wallet address. Makes a POST request to the faucet service with the wallet address.

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### walletAddress[​](#walletaddress "Direct link to walletAddress")

`string`

The address to receive the test tokens

#### Returns[​](#returns-2 "Direct link to Returns")

`Promise`<`void`>

A promise that resolves when the request is complete

#### Throws[​](#throws "Direct link to Throws")

Will log but not throw if the request fails
