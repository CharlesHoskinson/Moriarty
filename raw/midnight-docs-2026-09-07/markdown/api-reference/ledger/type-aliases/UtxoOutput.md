# UtxoOutput

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / UtxoOutput

# Type Alias: UtxoOutput

```
type UtxoOutput = {

  owner: UserAddress;

  type: RawTokenType;

  value: bigint;

};
```

An output appearing in an [Intent](/api-reference/ledger/classes/Intent.md).

## Properties[​](#properties "Direct link to Properties")

### owner[​](#owner "Direct link to owner")

```
owner: UserAddress;
```

The address owning these tokens.

***

### type[​](#type "Direct link to type")

```
type: RawTokenType;
```

The token type of this UTXO

***

### value[​](#value "Direct link to value")

```
value: bigint;
```

The amount of tokens this UTXO represents
