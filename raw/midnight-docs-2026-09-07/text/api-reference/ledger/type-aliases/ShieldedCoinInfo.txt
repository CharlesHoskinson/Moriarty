# ShieldedCoinInfo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ShieldedCoinInfo

# Type Alias: ShieldedCoinInfo

```
type ShieldedCoinInfo = {

  nonce: Nonce;

  type: RawTokenType;

  value: bigint;

};
```

Information required to create a new coin, alongside details about the recipient

## Properties[​](#properties "Direct link to Properties")

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
