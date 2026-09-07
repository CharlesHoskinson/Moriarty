# decodeShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / decodeShieldedCoinInfo

# Function: decodeShieldedCoinInfo()

```
function decodeShieldedCoinInfo(coin): ShieldedCoinInfo
```

Decode a [ShieldedCoinInfo](/api-reference/onchain-runtime/type-aliases/ShieldedCoinInfo.md) from Compact's `ShieldedCoinInfo` TypeScript representation

## Parameters[​](#parameters "Direct link to Parameters")

### coin[​](#coin "Direct link to coin")

#### color[​](#color "Direct link to color")

`Uint8Array`

#### nonce[​](#nonce "Direct link to nonce")

`Uint8Array`

#### value[​](#value "Direct link to value")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`ShieldedCoinInfo`](/api-reference/onchain-runtime/type-aliases/ShieldedCoinInfo.md)
