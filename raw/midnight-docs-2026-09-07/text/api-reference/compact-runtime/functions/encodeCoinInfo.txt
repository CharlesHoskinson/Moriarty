# encodeCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.9.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / encodeCoinInfo

# Function: encodeCoinInfo()

```
function encodeCoinInfo(coin): {

  color: Uint8Array;

  nonce: Uint8Array;

  value: bigint;

};
```

Encode a [CoinInfo](/api-reference/compact-runtime/type-aliases/CoinInfo.md) into a Compact's `CoinInfo` TypeScript representation

## Parameters[​](#parameters "Direct link to Parameters")

### coin[​](#coin "Direct link to coin")

[`CoinInfo`](/api-reference/compact-runtime/type-aliases/CoinInfo.md)

## Returns[​](#returns "Direct link to Returns")

```
{

  color: Uint8Array;

  nonce: Uint8Array;

  value: bigint;

}
```

### color[​](#color "Direct link to color")

```
color: Uint8Array;
```

### nonce[​](#nonce "Direct link to nonce")

```
nonce: Uint8Array;
```

### value[​](#value "Direct link to value")

```
value: bigint;
```
