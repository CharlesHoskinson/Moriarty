# contractAction

> For the complete documentation index, see [llms.txt](/llms.txt)

Find a contract action for the given address and optional offset.

```
contractAction(

  address: HexEncoded!

  offset: ContractActionOffset

): ContractAction
```

### Arguments[​](#arguments "Direct link to Arguments")

#### [`contractAction.address`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#contractactionaddresshexencoded-- "Direct link to contractactionaddresshexencoded--")

#### [`contractAction.offset`](#) ● [`ContractActionOffset`](/api-reference/midnight-indexer/types/inputs/contract-action-offset.md) input[​](#contractactionoffsetcontractactionoffset- "Direct link to contractactionoffsetcontractactionoffset-")

### Type[​](#type "Direct link to Type")

#### [`ContractAction`](/api-reference/midnight-indexer/types/interfaces/contract-action.md) interface[​](#contractaction- "Direct link to contractaction-")

A contract action.
