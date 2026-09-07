# GatherResult

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / GatherResult

# Type Alias: GatherResult

```
type GatherResult: {

  content: AlignedValue;

  tag: "read";

 } | {

  content: EncodedStateValue;

  tag: "log";

};
```

An individual result of observing the results of a non-verifying VM program execution
