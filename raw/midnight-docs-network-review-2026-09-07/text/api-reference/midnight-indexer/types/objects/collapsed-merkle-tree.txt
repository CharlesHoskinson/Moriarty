# CollapsedMerkleTree

> For the complete documentation index, see [llms.txt](/llms.txt)

No description

```
type CollapsedMerkleTree {

  startIndex: Int!

  endIndex: Int!

  update: HexEncoded!

  protocolVersion: Int!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`CollapsedMerkleTree.startIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#collapsedmerkletreestartindexint-- "Direct link to collapsedmerkletreestartindexint--")

The zswap state start index.

#### [`CollapsedMerkleTree.endIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#collapsedmerkletreeendindexint-- "Direct link to collapsedmerkletreeendindexint--")

The zswap state end index.

#### [`CollapsedMerkleTree.update`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#collapsedmerkletreeupdatehexencoded-- "Direct link to collapsedmerkletreeupdatehexencoded--")

The hex-encoded value.

#### [`CollapsedMerkleTree.protocolVersion`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#collapsedmerkletreeprotocolversionint-- "Direct link to collapsedmerkletreeprotocolversionint--")

The protocol version.

### Member Of[​](#member-of "Direct link to Member Of")

[`RelevantTransaction`](/api-reference/midnight-indexer/types/objects/relevant-transaction.md) object
