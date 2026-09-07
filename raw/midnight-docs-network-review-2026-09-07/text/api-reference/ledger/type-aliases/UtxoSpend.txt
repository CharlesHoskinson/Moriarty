# UtxoSpend

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / UtxoSpend

# Type Alias: UtxoSpend

```
type UtxoSpend = {

  intentHash: IntentHash;

  outputNo: number;

  owner: SignatureVerifyingKey;

  type: RawTokenType;

  value: bigint;

};
```

An input appearing in an [Intent](/api-reference/ledger/classes/Intent.md), or a user's local book-keeping.

## Properties[​](#properties "Direct link to Properties")

### intentHash[​](#intenthash "Direct link to intentHash")

```
intentHash: IntentHash;
```

The hash of the intent outputting this UTXO

***

### outputNo[​](#outputno "Direct link to outputNo")

```
outputNo: number;
```

The output number of this UTXO in its parent [Intent](/api-reference/ledger/classes/Intent.md).

***

### owner[​](#owner "Direct link to owner")

```
owner: SignatureVerifyingKey;
```

The signing key owning these tokens.

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
