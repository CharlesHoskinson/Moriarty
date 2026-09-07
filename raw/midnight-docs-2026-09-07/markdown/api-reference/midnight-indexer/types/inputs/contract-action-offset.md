# ContractActionOffset

> For the complete documentation index, see [llms.txt](/llms.txt)

Either a block offset or a transaction offset.

```
input ContractActionOffset {

  blockOffset: BlockOffset

  transactionOffset: TransactionOffset

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`ContractActionOffset.blockOffset`](#) ● [`BlockOffset`](/api-reference/midnight-indexer/types/inputs/block-offset.md) input[​](#contractactionoffsetblockoffsetblockoffset- "Direct link to contractactionoffsetblockoffsetblockoffset-")

Either a block hash or a block height.

#### [`ContractActionOffset.transactionOffset`](#) ● [`TransactionOffset`](/api-reference/midnight-indexer/types/inputs/transaction-offset.md) input[​](#contractactionoffsettransactionoffsettransactionoffset- "Direct link to contractactionoffsettransactionoffsettransactionoffset-")

Either a transaction hash or a transaction identifier.

### Member Of[​](#member-of "Direct link to Member Of")

[`contractAction`](/api-reference/midnight-indexer/operations/queries/contract-action.md) query
