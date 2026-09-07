# AlignmentAtom

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / AlignmentAtom

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

A atom in a larger [Alignment](/api-reference/ledger/type-aliases/Alignment.md).
