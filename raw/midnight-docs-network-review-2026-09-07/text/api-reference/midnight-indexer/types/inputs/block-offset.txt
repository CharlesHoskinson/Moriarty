# BlockOffset

> For the complete documentation index, see [llms.txt](/llms.txt)

Either a block hash or a block height.

```
input BlockOffset {

  hash: HexEncoded

  height: Int

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`BlockOffset.hash`](#) ● [`HexEncoded`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) scalar[​](#blockoffsethashhexencoded- "Direct link to blockoffsethashhexencoded-")

A hex-encoded block hash.

#### [`BlockOffset.height`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#blockoffsetheightint- "Direct link to blockoffsetheightint-")

A block height.

### Member Of[​](#member-of "Direct link to Member Of")

[`block`](/api-reference/midnight-indexer/operations/queries/block.md) query ● [`blocks`](/api-reference/midnight-indexer/operations/subscriptions/blocks.md) subscription ● [`ContractActionOffset`](/api-reference/midnight-indexer/types/inputs/contract-action-offset.md) input ● [`contractActions`](/api-reference/midnight-indexer/operations/subscriptions/contract-actions.md) subscription
