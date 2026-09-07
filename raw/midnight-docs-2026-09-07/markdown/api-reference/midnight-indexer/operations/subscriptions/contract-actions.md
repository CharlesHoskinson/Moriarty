# contractActions

> For the complete documentation index, see [llms.txt](/llms.txt)

Subscribe to contract actions with the given address starting at the given offset or at the latest block if the offset is omitted.

```
contractActions(

  address: HexEncoded!

  offset: BlockOffset

): ContractAction!
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`contractActions.address`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractactionsaddresshexencoded-- "Direct link to contractactionsaddresshexencoded--")

#### [`contractActions.offset`](#) ● [`BlockOffset`](/api-reference/midnight-indexer/types/inputs/block-offset.md) input[​](#contractactionsoffsetblockoffset- "Direct link to contractactionsoffsetblockoffset-")

### Type[​](#type "Direct link to Type")

#### [`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface[​](#contractaction- "Direct link to contractaction-")

A contract action.
