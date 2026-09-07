# EncodedQualifiedShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / EncodedQualifiedShieldedCoinInfo

# Interface: EncodedQualifiedShieldedCoinInfo

A QualifiedCoinInfo with its fields encoded as byte strings. This representation is used internally by the contract executable.

## Extends[​](#extends "Direct link to Extends")

* [`EncodedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md)

## Properties[​](#properties "Direct link to Properties")

### color[​](#color "Direct link to color")

```
readonly color: Uint8Array;
```

The coin's type, identifying the currency it represents.

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`EncodedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md).[`color`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md#color)

***

### mt\_index[​](#mt_index "Direct link to mt_index")

```
readonly mt_index: bigint;
```

The coin's location in the chain's Merkle tree of coin commitments. Bounded to be a non-negative 64-bit integer.

***

### nonce[​](#nonce "Direct link to nonce")

```
readonly nonce: Uint8Array;
```

The coin's randomness, preventing it from colliding with other coins.

#### Inherited from[​](#inherited-from-1 "Direct link to Inherited from")

[`EncodedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md).[`nonce`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md#nonce)

***

### value[​](#value "Direct link to value")

```
readonly value: bigint;
```

The coin's value, in atomic units dependent on the currency. Bounded to be a non-negative 64-bit integer.

#### Inherited from[​](#inherited-from-2 "Direct link to Inherited from")

[`EncodedShieldedCoinInfo`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md).[`value`](/api-reference/compact-runtime/interfaces/EncodedShieldedCoinInfo.md#value)
