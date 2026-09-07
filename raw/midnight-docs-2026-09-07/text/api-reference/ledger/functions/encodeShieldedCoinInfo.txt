# encodeShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / encodeShieldedCoinInfo

# Function: encodeShieldedCoinInfo()

```
function encodeShieldedCoinInfo(coin): {

  color: Uint8Array;

  nonce: Uint8Array;

  value: bigint;

};
```

Encode a [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) into a Compact's `ShieldedCoinInfo` TypeScript representation

## Parameters[​](#parameters "Direct link to Parameters")

### coin[​](#coin "Direct link to coin")

[`ShieldedCoinInfo`](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md)

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
