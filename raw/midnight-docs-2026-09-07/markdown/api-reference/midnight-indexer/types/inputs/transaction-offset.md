# TransactionOffset

> For the complete documentation index, see [llms.txt](/llms.txt)

Either a transaction hash or a transaction identifier.

```
input TransactionOffset {

  hash: HexEncoded

  identifier: HexEncoded

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`TransactionOffset.hash`](#) ● [`HexEncoded`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) scalar[​](#transactionoffsethashhexencoded- "Direct link to transactionoffsethashhexencoded-")

A hex-encoded transaction hash.

#### [`TransactionOffset.identifier`](#) ● [`HexEncoded`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) scalar[​](#transactionoffsetidentifierhexencoded- "Direct link to transactionoffsetidentifierhexencoded-")

A hex-encoded transaction identifier.

### Member Of[​](#member-of "Direct link to Member Of")

[`ContractActionOffset`](/api-reference/midnight-indexer/types/inputs/contract-action-offset.md) input ● [`transactions`](/api-reference/midnight-indexer/operations/queries/transactions.md) query
