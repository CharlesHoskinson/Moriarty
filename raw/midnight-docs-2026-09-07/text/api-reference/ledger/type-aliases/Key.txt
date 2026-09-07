# Key

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / Key

# Type Alias: Key

```
type Key = 

  | {

  tag: "value";

  value: AlignedValue;

}

  | {

  tag: "stack";

};
```

A key used to index into an array or map in the onchain VM
