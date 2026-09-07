# decodeQualifiedShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / decodeQualifiedShieldedCoinInfo

# Function: decodeQualifiedShieldedCoinInfo()

```
function decodeQualifiedShieldedCoinInfo(coin): QualifiedShieldedCoinInfo
```

Decode a [QualifiedShieldedCoinInfo](/api-reference/onchain-runtime/type-aliases/QualifiedShieldedCoinInfo.md) from Compact's `QualifiedShieldedCoinInfo` TypeScript representation

## Parameters[​](#parameters "Direct link to Parameters")

### coin[​](#coin "Direct link to coin")

#### color[​](#color "Direct link to color")

`Uint8Array`

#### mt\_index[​](#mt_index "Direct link to mt_index")

`bigint`

#### nonce[​](#nonce "Direct link to nonce")

`Uint8Array`

#### value[​](#value "Direct link to value")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`QualifiedShieldedCoinInfo`](/api-reference/onchain-runtime/type-aliases/QualifiedShieldedCoinInfo.md)
