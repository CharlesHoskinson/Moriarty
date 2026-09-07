# FinalizedTxData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / FinalizedTxData

# Interface: FinalizedTxData

Data for any finalized transaction.

## Properties[​](#properties "Direct link to Properties")

### blockAuthor[​](#blockauthor "Direct link to blockAuthor")

> `readonly` **blockAuthor**: `string` | `null`

The author of the block in which the transaction was included.

***

### blockHash[​](#blockhash "Direct link to blockHash")

> `readonly` **blockHash**: `string`

The block hash of the block in which the transaction was included.

***

### blockHeight[​](#blockheight "Direct link to blockHeight")

> `readonly` **blockHeight**: `number`

The block height of the block in which the transaction was included.

***

### blockTimestamp[​](#blocktimestamp "Direct link to blockTimestamp")

> `readonly` **blockTimestamp**: `number`

The timestamp of the block in which the transaction was included.

***

### fees[​](#fees "Direct link to fees")

> `readonly` **fees**: [`Fees`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/Fees.md)

The fees associated with the transaction, including both paid and estimated fees.

***

### identifiers[​](#identifiers "Direct link to identifiers")

> `readonly` **identifiers**: readonly `string`\[]

All transaction IDs of the submitted transaction.

***

### indexerId[​](#indexerid "Direct link to indexerId")

> `readonly` **indexerId**: `number`

The indexer internal db ID.

***

### protocolVersion[​](#protocolversion "Direct link to protocolVersion")

> `readonly` **protocolVersion**: `number`

The protocol version of the transaction.

***

### segmentStatusMap[​](#segmentstatusmap "Direct link to segmentStatusMap")

> `readonly` **segmentStatusMap**: `Map`<`number`, [`SegmentStatus`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/SegmentStatus.md)> | `undefined`

The map that associates segment identifiers (numbers) with their corresponding status [SegmentStatus](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/SegmentStatus.md). The segment identifier is represented as a number (key in the map), and the status indicates the success or failure of the transaction update.

***

### status[​](#status "Direct link to status")

> `readonly` **status**: [`TxStatus`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/TxStatus.md)

The status of a submitted transaction.

***

### tx[​](#tx "Direct link to tx")

> `readonly` **tx**: `Transaction`<`SignatureEnabled`, `Proof`, `Binding`>

The transaction that was finalized.

***

### txHash[​](#txhash "Direct link to txHash")

> `readonly` **txHash**: `string`

The transaction hash of the transaction in which the original transaction was included.

***

### txId[​](#txid "Direct link to txId")

> `readonly` **txId**: `string`

One of the transaction ID of the submitted transaction.

***

### unshielded[​](#unshielded "Direct link to unshielded")

> `readonly` **unshielded**: [`UnshieldedUtxos`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedUtxos.md)

Represents the unshielded outputs, typically used for transactions or operations involving data or values that are not encrypted or concealed.
