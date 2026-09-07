# decodeShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / decodeShieldedCoinInfo

# Function: decodeShieldedCoinInfo()

```
function decodeShieldedCoinInfo(coin): ShieldedCoinInfo;
```

Decode a [ShieldedCoinInfo](/api-reference/compact-runtime/type-aliases/ShieldedCoinInfo.md) from Compact's `ShieldedCoinInfo` TypeScript representation

## Parameters[​](#parameters "Direct link to Parameters")

### coin[​](#coin "Direct link to coin")

#### color[​](#color "Direct link to color")

`Uint8Array`

#### nonce[​](#nonce "Direct link to nonce")

`Uint8Array`

#### value[​](#value "Direct link to value")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`ShieldedCoinInfo`](/api-reference/compact-runtime/type-aliases/ShieldedCoinInfo.md)
