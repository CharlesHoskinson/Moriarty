# parseEncPublicKeyToHex

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-utils](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-utils.md) / parseEncPublicKeyToHex

# Function: parseEncPublicKeyToHex()

> **parseEncPublicKeyToHex**(`possibleBech32`, `zswapNetworkId`): `string`

Parses an encryption public key (in Bech32m or hex format) into a hex formatted string.

## Parameters[​](#parameters "Direct link to Parameters")

### possibleBech32[​](#possiblebech32 "Direct link to possibleBech32")

`string`

The input string, which can be a Bech32m-encoded encryption public key or a hex string.

### zswapNetworkId[​](#zswapnetworkid "Direct link to zswapNetworkId")

`string`

The network ID used for decoding the Bech32m formatted string.

## Returns[​](#returns "Direct link to Returns")

`string`

The hex string representation of the encryption public key.

## Throws[​](#throws "Direct link to Throws")

`Error` If the input string is not a valid hex string or a valid Bech32m-encoded encryption public key.
