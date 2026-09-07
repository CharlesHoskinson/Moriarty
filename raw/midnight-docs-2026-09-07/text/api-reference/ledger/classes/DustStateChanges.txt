# DustStateChanges

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustStateChanges

# Class: DustStateChanges

## Properties[​](#properties "Direct link to Properties")

### receivedUtxos[​](#receivedutxos "Direct link to receivedUtxos")

```
readonly receivedUtxos: QualifiedDustOutput[];
```

The UTXOs that were received in this state change

***

### source[​](#source "Direct link to source")

```
readonly source: string;
```

The source of the state change, as a hex-encoded string

***

### spentUtxos[​](#spentutxos "Direct link to spentUtxos")

```
readonly spentUtxos: QualifiedDustOutput[];
```

The UTXOs that were spent in this state change
