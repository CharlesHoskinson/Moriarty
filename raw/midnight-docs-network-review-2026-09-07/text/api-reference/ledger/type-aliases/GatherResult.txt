# GatherResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / GatherResult

# Type Alias: GatherResult

```
type GatherResult = 

  | {

  content: AlignedValue;

  tag: "read";

}

  | {

  content: EncodedStateValue;

  tag: "log";

};
```

An individual result of observing the results of a non-verifying VM program execution
