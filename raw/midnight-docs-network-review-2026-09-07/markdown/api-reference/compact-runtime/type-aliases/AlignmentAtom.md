# AlignmentAtom

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / AlignmentAtom

# Type Alias: AlignmentAtom

```
type AlignmentAtom = 

  | {

  tag: "compress";

}

  | {

  tag: "field";

}

  | {

  length: number;

  tag: "bytes";

};
```

A atom in a larger [Alignment](/api-reference/compact-runtime/type-aliases/Alignment.md).
