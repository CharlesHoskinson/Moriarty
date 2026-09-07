# AlignmentSegment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / AlignmentSegment

# Type Alias: AlignmentSegment

```
type AlignmentSegment = 

  | {

  tag: "option";

  value: Alignment[];

}

  | {

  tag: "atom";

  value: AlignmentAtom;

};
```

A segment in a larger [Alignment](/api-reference/compact-runtime/type-aliases/Alignment.md).
