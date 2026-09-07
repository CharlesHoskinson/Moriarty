# EncodedStateValue

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / EncodedStateValue

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

An alternative encoding of [StateValue](/api-reference/compact-runtime/classes/StateValue.md) for use in [Op](/api-reference/compact-runtime/type-aliases/Op.md) for technical reasons
