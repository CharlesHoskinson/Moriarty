# Segment

> For the complete documentation index, see [llms.txt](/llms.txt)

One of many segments for a partially successful transaction result showing success for some segment.

```
type Segment {

  id: Int!

  success: Boolean!

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`Segment.id`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#segmentidint-- "Direct link to segmentidint--")

Segment ID.

#### [`Segment.success`](#) ● [`Boolean!`](/api-reference/midnight-indexer/types/scalars/boolean.md) non-null scalar[​](#segmentsuccessboolean-- "Direct link to segmentsuccessboolean--")

Successful or not.

### Member Of[​](#member-of "Direct link to Member Of")

[`TransactionResult`](/api-reference/midnight-indexer/types/objects/transaction-result.md) object
