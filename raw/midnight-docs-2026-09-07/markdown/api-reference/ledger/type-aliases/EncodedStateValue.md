# EncodedStateValue

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / EncodedStateValue

# Type Alias: EncodedStateValue

```
type EncodedStateValue = 

  | {

  tag: "null";

}

  | {

  content: AlignedValue;

  tag: "cell";

}

  | {

  content: Map<AlignedValue, EncodedStateValue>;

  tag: "map";

}

  | {

  content: EncodedStateValue[];

  tag: "array";

}

  | {

  content: [number, Map<bigint, [Uint8Array, undefined]>];

  tag: "boundedMerkleTree";

};
```

An alternative encoding of [StateValue](/api-reference/ledger/classes/StateValue.md) for use in [Op](/api-reference/ledger/type-aliases/Op.md) for technical reasons
