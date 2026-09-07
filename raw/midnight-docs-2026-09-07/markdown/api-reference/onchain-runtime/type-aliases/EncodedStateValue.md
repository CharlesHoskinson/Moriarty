# EncodedStateValue

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / EncodedStateValue

# Type Alias: EncodedStateValue

```
type EncodedStateValue: 

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

An alternative encoding of [StateValue](/api-reference/onchain-runtime/classes/StateValue.md) for use in [Op](/api-reference/onchain-runtime/type-aliases/Op.md) for technical reasons
