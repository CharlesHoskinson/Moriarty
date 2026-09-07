# PublicAddress

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / PublicAddress

# Type Alias: PublicAddress

```
type PublicAddress: {

  address: UserAddress;

  tag: "user";

 } | {

  address: ContractAddress;

  tag: "contract";

};
```

A public address that an entity can be identified by
