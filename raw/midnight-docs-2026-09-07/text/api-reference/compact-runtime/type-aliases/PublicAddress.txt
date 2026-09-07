# PublicAddress

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / PublicAddress

# Type Alias: PublicAddress

```
type PublicAddress = 

  | {

  address: UserAddress;

  tag: "user";

}

  | {

  address: ContractAddress;

  tag: "contract";

};
```

A public address that an entity can be identified by
