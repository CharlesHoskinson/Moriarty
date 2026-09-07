# AlignmentSegment

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / AlignmentSegment

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

A segment in a larger [Alignment](/api-reference/ledger/type-aliases/Alignment.md).
