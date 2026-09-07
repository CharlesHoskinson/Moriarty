# decodeShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / decodeShieldedCoinInfo

# Function: decodeShieldedCoinInfo()

```
function decodeShieldedCoinInfo(coin): ShieldedCoinInfo;
```

Decode a [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) from Compact's `ShieldedCoinInfo` TypeScript representation

## Parameters[​](#parameters "Direct link to Parameters")

### coin[​](#coin "Direct link to coin")

#### color[​](#color "Direct link to color")

`Uint8Array`

#### nonce[​](#nonce "Direct link to nonce")

`Uint8Array`

#### value[​](#value "Direct link to value")

`bigint`

## Returns[​](#returns "Direct link to Returns")

[`ShieldedCoinInfo`](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md)
