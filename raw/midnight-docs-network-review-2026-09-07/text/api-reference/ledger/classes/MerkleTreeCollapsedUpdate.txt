# MerkleTreeCollapsedUpdate

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / MerkleTreeCollapsedUpdate

# Class: MerkleTreeCollapsedUpdate

A compact delta on the coin commitments Merkle tree, used to keep local spending trees in sync with the global state without requiring receiving all transactions.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new MerkleTreeCollapsedUpdate(

   state, 

   start, 

   end): MerkleTreeCollapsedUpdate;
```

Create a new compact update from a non-compact state, and inclusive `start` and `end` indices

#### Parameters[​](#parameters "Direct link to Parameters")

##### state[​](#state "Direct link to state")

[`ZswapChainState`](/api-reference/ledger/classes/ZswapChainState.md)

##### start[​](#start "Direct link to start")

`bigint`

##### end[​](#end "Direct link to end")

`bigint`

#### Returns[​](#returns "Direct link to Returns")

`MerkleTreeCollapsedUpdate`

#### Throws[​](#throws "Direct link to Throws")

If the indices are out-of-bounds for the state, or `end < start`

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): MerkleTreeCollapsedUpdate;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`MerkleTreeCollapsedUpdate`
