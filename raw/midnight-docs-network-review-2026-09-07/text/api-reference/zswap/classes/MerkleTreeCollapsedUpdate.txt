# MerkleTreeCollapsedUpdate

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/zswap v4.0.0-rc**](/api-reference/zswap.md)

***

[@midnight/zswap](/api-reference/zswap/globals.md) / MerkleTreeCollapsedUpdate

# Class: MerkleTreeCollapsedUpdate

A compact delta on the coin commitments Merkle tree, used to keep local spending trees in sync with the global state without requiring receiving all transactions.

## Constructors[​](#constructors "Direct link to Constructors")

### new MerkleTreeCollapsedUpdate()[​](#new-merkletreecollapsedupdate "Direct link to new MerkleTreeCollapsedUpdate()")

```
new MerkleTreeCollapsedUpdate(

   state, 

   start, 

   end): MerkleTreeCollapsedUpdate
```

Create a new compact update from a non-compact state, and inclusive `start` and `end` indices

#### Parameters[​](#parameters "Direct link to Parameters")

##### state[​](#state "Direct link to state")

[`ZswapChainState`](/api-reference/zswap/classes/ZswapChainState.md)

##### start[​](#start "Direct link to start")

`bigint`

##### end[​](#end "Direct link to end")

`bigint`

#### Returns[​](#returns "Direct link to Returns")

[`MerkleTreeCollapsedUpdate`](/api-reference/zswap/classes/MerkleTreeCollapsedUpdate.md)

#### Throws[​](#throws "Direct link to Throws")

If the indices are out-of-bounds for the state, or `end < start`

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(netid): Uint8Array<ArrayBufferLike>
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### netid[​](#netid "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`<`ArrayBufferLike`>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw, netid): MerkleTreeCollapsedUpdate
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`<`ArrayBufferLike`>

##### netid[​](#netid-1 "Direct link to netid")

[`NetworkId`](/api-reference/zswap/enumerations/NetworkId.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`MerkleTreeCollapsedUpdate`](/api-reference/zswap/classes/MerkleTreeCollapsedUpdate.md)
