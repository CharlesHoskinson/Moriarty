# PublicAddress

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / PublicAddress

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
