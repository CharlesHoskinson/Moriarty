# EncodedShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / EncodedShieldedCoinInfo

# Interface: EncodedShieldedCoinInfo

A [ShieldedCoinInfo](/api-reference/compact-runtime/type-aliases/ShieldedCoinInfo.md) with its fields encoded as byte strings. This representation is used internally by the contract executable.

## Extended by[​](#extended-by "Direct link to Extended by")

* [`EncodedQualifiedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedQualifiedShieldedCoinInfo.md)

## Properties[​](#properties "Direct link to Properties")

### color[​](#color "Direct link to color")

```
readonly color: Uint8Array;
```

The coin's type, identifying the currency it represents.

***

### nonce[​](#nonce "Direct link to nonce")

```
readonly nonce: Uint8Array;
```

The coin's randomness, preventing it from colliding with other coins.

***

### value[​](#value "Direct link to value")

```
readonly value: bigint;
```

The coin's value, in atomic units dependent on the currency. Bounded to be a non-negative 64-bit integer.
