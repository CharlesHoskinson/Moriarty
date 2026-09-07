# QualifiedShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / QualifiedShieldedCoinInfo

# Type Alias: QualifiedShieldedCoinInfo

```
type QualifiedShieldedCoinInfo = {

  mt_index: bigint;

  nonce: Nonce;

  type: RawTokenType;

  value: bigint;

};
```

Information required to spend an existing coin, alongside authorization of the owner

## Properties[​](#properties "Direct link to Properties")

### mt\_index[​](#mt_index "Direct link to mt_index")

```
mt_index: bigint;
```

The coin's location in the chain's Merkle tree of coin commitments

Bounded to be a non-negative 64-bit integer

***

### nonce[​](#nonce "Direct link to nonce")

```
nonce: Nonce;
```

The coin's randomness, preventing it from colliding with other coins

***

### type[​](#type "Direct link to type")

```
type: RawTokenType;
```

The coin's type, identifying the currency it represents

***

### value[​](#value "Direct link to value")

```
value: bigint;
```

The coin's value, in atomic units dependent on the currency

Bounded to be a non-negative 64-bit integer
