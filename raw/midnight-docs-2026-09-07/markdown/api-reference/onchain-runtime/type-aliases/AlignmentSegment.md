# AlignmentSegment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / AlignmentSegment

# Type Alias: AlignmentSegment

```
type AlignmentSegment: {

  tag: "option";

  value: Alignment[];

 } | {

  tag: "atom";

  value: AlignmentAtom;

};
```

A segment in a larger [Alignment](/api-reference/onchain-runtime/type-aliases/Alignment.md).
