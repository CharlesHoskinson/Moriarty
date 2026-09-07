> For the complete documentation index, see [llms.txt](/llms.txt)

# Ledger data types

Compact language version 0.26.0, compiler version 0.34.0.

## Kernel[​](#kernel "Direct link to Kernel")

This ADT is a special ADT defining various built-in operations and valid only as a top-level ADT type.

### balance[​](#balance "Direct link to balance")

```
balance(token_type: Either<Bytes<32>, Bytes<32>>): Uint<128>
```

Returns the current contract's balance of the unshielded token of the given token type. The balance is not updated during contract execution as a result of unshielded sends and receives. It is always fixed to the value provided at the start of execution.

### balanceGreaterThan[​](#balancegreaterthan "Direct link to balanceGreaterThan")

```
balanceGreaterThan(token_type: Either<Bytes<32>, Bytes<32>>, amount: Uint<128>): Boolean
```

Checks whether the current balance of the unshielded token of the given type is greater than the given amount.

### balanceLessThan[​](#balancelessthan "Direct link to balanceLessThan")

```
balanceLessThan(token_type: Either<Bytes<32>, Bytes<32>>, amount: Uint<128>): Boolean
```

Checks whether the current balance of the unshielded token of the given type is less than the given amount.

### blockTimeGreaterThan[​](#blocktimegreaterthan "Direct link to blockTimeGreaterThan")

```
blockTimeGreaterThan(time: Uint<64>): Boolean
```

Checks whether the current block time (measured in seconds since the Unix epoch) is greater than the given amount.

### blockTimeLessThan[​](#blocktimelessthan "Direct link to blockTimeLessThan")

```
blockTimeLessThan(time: Uint<64>): Boolean
```

Checks whether the current block time (measured in seconds since the Unix epoch) is less than the given amount.

### checkpoint[​](#checkpoint "Direct link to checkpoint")

```
checkpoint(): []
```

Marks all execution up to this point as being a single atomic unit, allowing partial transaction failures to be split across it.

### claimContractCall[​](#claimcontractcall "Direct link to claimContractCall")

```
claimContractCall(addr: Bytes<32>, entry_point: Bytes<32>, comm: Field): []
```

Require the presence of another contract call in the containing transaction, with a match address, entry point hash, and communication commitment, that is not claimed by any other call.

### claimUnshieldedCoinSpend[​](#claimunshieldedcoinspend "Direct link to claimUnshieldedCoinSpend")

```
claimUnshieldedCoinSpend(token_type: Either<Bytes<32>, Bytes<32>>, address: Either<ContractAddress, UserAddress>, amount: Uint<128>): []
```

Claims an unshielded coin spend - authorizes an unshielded coin of the given token type to be transferred to the given address.

### claimZswapCoinReceive[​](#claimzswapcoinreceive "Direct link to claimZswapCoinReceive")

```
claimZswapCoinReceive(note: Bytes<32>): []
```

Requires the presence of a commitment in the containing transaction and that no other call claims it as a receive.

### claimZswapCoinSpend[​](#claimzswapcoinspend "Direct link to claimZswapCoinSpend")

```
claimZswapCoinSpend(note: Bytes<32>): []
```

Requires the presence of a commitment in the containing transaction and that no other call claims it as a spend.

### claimZswapNullifier[​](#claimzswapnullifier "Direct link to claimZswapNullifier")

```
claimZswapNullifier(nul: Bytes<32>): []
```

Requires the presence of a nullifier in the containing transaction and that no other call claims it.

### incUnshieldedInputs[​](#incunshieldedinputs "Direct link to incUnshieldedInputs")

```
incUnshieldedInputs(token_type: Either<Bytes<32>, Bytes<32>>, amount: Uint<128>): []
```

Increments the unshielded input for the token of the given token type by the given amount - used when receiving tokens.

### incUnshieldedOutputs[​](#incunshieldedoutputs "Direct link to incUnshieldedOutputs")

```
incUnshieldedOutputs(token_type: Either<Bytes<32>, Bytes<32>>, amount: Uint<128>): []
```

Increments the unshielded output for the token of the given token type by the given amount - used when sending tokens.

### mintShielded[​](#mintshielded "Direct link to mintShielded")

```
mintShielded(domain_sep: Bytes<32>, amount: Uint<64>): []
```

Mints a given amount of shielded coins with a token type derived from the contract's address, and a given domain separator.

### mintUnshielded[​](#mintunshielded "Direct link to mintUnshielded")

```
mintUnshielded(domain_sep: Bytes<32>, amount: Uint<64>): []
```

Mints a given amount of unshielded coins with a token type derived from the contract's address, and a given domain separator.

### self[​](#self "Direct link to self")

```
self(): ContractAddress
```

Returns the current contract's address. ContractAddress is defined in CompactStandardLibrary.

## Cell\<value\_type>[​](#cellvalue_type "Direct link to Cell<value_type>")

This ADT is a single Cell containing a value of type value\_type and is used implicitly when the ledger field type is an ordinary Compact type. Programmers cannot write Cell explicitly when declaring a ledger field..

### read[​](#read "Direct link to read")

```
read(): value_type
```

Returns the current contents of this Cell.

*available from Typescript as a getter on the ledger field*

### resetToDefault[​](#resettodefault "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this Cell to the default value of its type.

### write[​](#write "Direct link to write")

```
write(value: value_type): []
```

Overwrites the content of this Cell with the given value.

### writeCoin[​](#writecoin "Direct link to writeCoin")

```
writeCoin(coin: ShieldedCoinInfo, recipient: Either<ZswapCoinPublicKey, ContractAddress>): []
```

Writes a ShieldedCoinInfo to this Cell, which is transformed into a QualifiedShieldedCoinInfo at runtime by looking up the relevant Merkle tree index. This index must have been allocated within the current transaction or this write fails. ShieldedCoinInfo, ContractAddress, Either, and ZswapCoinPublicKey are defined in CompactStandardLibrary.

**available only for QualifiedShieldedCoinInfo value\_type**

## Counter[​](#counter "Direct link to Counter")

This ADT is a simple counter.

### decrement[​](#decrement "Direct link to decrement")

```
decrement(amount: Uint<16>): []
```

Decrements the counter by a given amount. Decrementing below zero results in a run-time error.

### increment[​](#increment "Direct link to increment")

```
increment(amount: Uint<16>): []
```

Increments the counter by the given amount.

### lessThan[​](#lessthan "Direct link to lessThan")

```
lessThan(threshold: Uint<64>): Boolean
```

Returns if the counter is less than the given threshold value.

### read[​](#read-1 "Direct link to read")

```
read(): Uint<64>
```

Retrieves the current value of the counter.

*available from Typescript as a getter on the ledger field*

### resetToDefault[​](#resettodefault-1 "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this Counter to its default value of 0.

## Set\<value\_type>[​](#setvalue_type "Direct link to Set<value_type>")

This ADT is an unbounded set of values of type value\_type.

### insert[​](#insert "Direct link to insert")

```
insert(elem: value_type): []
```

Updates this Set to include a given element.

### insertCoin[​](#insertcoin "Direct link to insertCoin")

```
insertCoin(coin: ShieldedCoinInfo, recipient: Either<ZswapCoinPublicKey, ContractAddress>): []
```

Inserts a ShieldedCoinInfo into this Set, which is transformed into a QualifiedShieldedCoinInfo at runtime by looking up the relevant Merkle tree index. This index must have been allocated within the current transaction or this insertion fails. ShieldedCoinInfo, ContractAddress, Either, and ZswapCoinPublicKey are defined in CompactStandardLibrary.

**available only for QualifiedShieldedCoinInfo value\_type**

### isEmpty[​](#isempty "Direct link to isEmpty")

```
isEmpty(): Boolean
```

Returns whether this Set is the empty set.

*available from Typescript as `isEmpty(): boolean`*

### member[​](#member "Direct link to member")

```
member(elem: value_type): Boolean
```

Returns if an element is contained within this Set.

*available from Typescript as `member(elem: value_type): boolean`*

### remove[​](#remove "Direct link to remove")

```
remove(elem: value_type): []
```

Update this Set to not include a given element.

### resetToDefault[​](#resettodefault-2 "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this Set to the empty set.

### size[​](#size "Direct link to size")

```
size(): Uint<64>
```

Returns the number of unique entries in this Set.

*available from Typescript as `size(): bigint`*

### \[Symbol.iterator][​](#symboliterator "Direct link to \[Symbol.iterator]")

*callable only from TypeScript*

```
[Symbol.iterator](): Iterator<value_type>
```

Iterates over the entries in this Set.

## Map\<key\_type, value\_type>[​](#mapkey_type-value_type "Direct link to Map<key_type, value_type>")

This ADT is an unbounded set of mappings between values of type key\_type and values of type value\_type.

### insert[​](#insert-1 "Direct link to insert")

```
insert(key: key_type, value: value_type): []
```

Updates this Map to include a new value at a given key.

### insertCoin[​](#insertcoin-1 "Direct link to insertCoin")

```
insertCoin(key: key_type, coin: ShieldedCoinInfo, recipient: Either<ZswapCoinPublicKey, ContractAddress>): []
```

Inserts a ShieldedCoinInfo into this Map at a given key, where the ShieldedCoinInfo is transformed into a QualifiedShieldedCoinInfo at runtime by looking up the relevant Merkle tree index. This index must have been allocated within the current transaction or this insertion fails. ShieldedCoinInfo, ContractAddress, Either, and ZswapCoinPublicKey are defined in CompactStandardLibrary.

**available only for QualifiedShieldedCoinInfo value\_type**

### insertDefault[​](#insertdefault "Direct link to insertDefault")

```
insertDefault(key: key_type): []
```

Updates this Map to include the value type's default value at a given key.

### isEmpty[​](#isempty-1 "Direct link to isEmpty")

```
isEmpty(): Boolean
```

Returns if this Map is the empty map.

*available from Typescript as `isEmpty(): boolean`*

### lookup[​](#lookup "Direct link to lookup")

```
lookup(key: key_type): value_type
```

Looks up the value of a key within this Map. The returned value may be another ADT.

*available from Typescript as `lookup(key: key_type): value_type`*

### member[​](#member-1 "Direct link to member")

```
member(key: key_type): Boolean
```

Returns if a key is contained within this Map.

*available from Typescript as `member(key: key_type): boolean`*

### remove[​](#remove-1 "Direct link to remove")

```
remove(key: key_type): []
```

Updates this Map to not include a given key.

### resetToDefault[​](#resettodefault-3 "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this Map to the empty map.

### size[​](#size-1 "Direct link to size")

```
size(): Uint<64>
```

Returns the number of entries in this Map.

*available from Typescript as `size(): bigint`*

### \[Symbol.iterator][​](#symboliterator-1 "Direct link to \[Symbol.iterator]")

*callable only from TypeScript*

```
[Symbol.iterator](): Iterator<[key_type, value_type]>
```

Iterates over the key-value pairs contained in this Map.

## List\<value\_type>[​](#listvalue_type "Direct link to List<value_type>")

This ADT is an unbounded list of values of type value\_type.

### head[​](#head "Direct link to head")

```
head(): Maybe<value_type>
```

Retrieves the head of this List, returning a Maybe, ensuring this call succeeds on the empty list. Maybe is defined in CompactStandardLibrary (compact-runtime from Typescript).

*available from Typescript as `head(): Maybe<value_type>`*

### isEmpty[​](#isempty-2 "Direct link to isEmpty")

```
isEmpty(): Boolean
```

Returns if this List is the empty list.

*available from Typescript as `isEmpty(): boolean`*

### length[​](#length "Direct link to length")

```
length(): Uint<64>
```

Returns the number of elements contained in this List.

*available from Typescript as `length(): bigint`*

### popFront[​](#popfront "Direct link to popFront")

```
popFront(): []
```

Removes the first element from the front of this list.

### pushFront[​](#pushfront "Direct link to pushFront")

```
pushFront(value: value_type): []
```

Pushes a new element onto the front of this list.

### pushFrontCoin[​](#pushfrontcoin "Direct link to pushFrontCoin")

```
pushFrontCoin(coin: ShieldedCoinInfo, recipient: Either<ZswapCoinPublicKey, ContractAddress>): []
```

Pushes a ShieldedCoinInfo onto the front of this List, where the ShieldedCoinInfo is transformed into a QualifiedShieldedCoinInfo at runtime by looking up the relevant Merkle tree index. This index must have been allocated within the current transaction or this push fails. ShieldedCoinInfo, ContractAddress, Either, and ZswapCoinPublicKey are defined in CompactStandardLibrary.

**available only for QualifiedShieldedCoinInfo value\_type**

### resetToDefault[​](#resettodefault-4 "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this List to the empty list.

### \[Symbol.iterator][​](#symboliterator-2 "Direct link to \[Symbol.iterator]")

*callable only from TypeScript*

```
[Symbol.iterator](): Iterator<value_type>
```

Iterates over the entries in this List.

## MerkleTree\<nat, value\_type>[​](#merkletreenat-value_type "Direct link to MerkleTree<nat, value_type>")

This ADT is a bounded Merkle tree of depth nat where 2 `<=` nat `<= 32` containing values of type value\_type.

### checkRoot[​](#checkroot "Direct link to checkRoot")

```
checkRoot(rt: MerkleTreeDigest): Boolean
```

Tests if the given Merkle tree root is the root for this Merkle tree. MerkleTreeDigest is defined in CompactStandardLibrary (compact-runtime from Typescript).

*available from Typescript as `checkRoot(rt: MerkleTreeDigest): boolean`*

### insert[​](#insert-2 "Direct link to insert")

```
insert(item: value_type): []
```

Inserts a new leaf at the first free index in this Merkle tree.

### insertHash[​](#inserthash "Direct link to insertHash")

```
insertHash(hash: Bytes<32>): []
```

Inserts a new leaf with a given hash at the first free index in this Merkle tree.

### insertHashIndex[​](#inserthashindex "Direct link to insertHashIndex")

```
insertHashIndex(hash: Bytes<32>, index: Uint<64>): []
```

Inserts a new leaf with a given hash at a specific index in this Merkle tree.

### insertIndex[​](#insertindex "Direct link to insertIndex")

```
insertIndex(item: value_type, index: Uint<64>): []
```

Inserts a new leaf at a specific index in this Merkle tree.

### insertIndexDefault[​](#insertindexdefault "Direct link to insertIndexDefault")

```
insertIndexDefault(index: Uint<64>): []
```

Inserts a default value leaf at a specific index in this Merkle tree. This can be used to emulate a removal from the tree.

### isFull[​](#isfull "Direct link to isFull")

```
isFull(): Boolean
```

Returns if this Merkle tree is full and further items cannot be directly inserted.

*available from Typescript as `isFull(): boolean`*

### resetToDefault[​](#resettodefault-5 "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this Merkle tree to the empty Merkle tree.

### findPathForLeaf[​](#findpathforleaf "Direct link to findPathForLeaf")

*callable only from TypeScript*

```
findPathForLeaf(leaf: value_type): MerkleTreePath<value_type> | undefined
```

Finds the path for a given leaf in a Merkle tree. Be warned that this is O(n) and should be avoided for large trees. Returns undefined if no such leaf exists. MerkleTreePath is defined in compact-runtime.

### firstFree[​](#firstfree "Direct link to firstFree")

*callable only from TypeScript*

```
firstFree(): bigint
```

Retrieves the first (guaranteed) free index in the Merkle tree.

### pathForLeaf[​](#pathforleaf "Direct link to pathForLeaf")

*callable only from TypeScript*

```
pathForLeaf(index: bigint, leaf: value_type): MerkleTreePath<value_type>
```

Returns the Merkle path, given the knowledge that a specified leaf is at the given index. It is an error to call this if this leaf is not contained at the given index. MerkleTreePath is defined in compact-runtime.

### root[​](#root "Direct link to root")

*callable only from TypeScript*

```
root(): MerkleTreeDigest
```

Retrieves the root of the Merkle tree. MerkleTreeDigest is defined in compact-runtime.

## HistoricMerkleTree\<nat, value\_type>[​](#historicmerkletreenat-value_type "Direct link to HistoricMerkleTree<nat, value_type>")

This ADT is a bounded Merkle tree of depth nat where 2 `<=` nat `<=` 32 containing values of type value\_type, with history.

### checkRoot[​](#checkroot-1 "Direct link to checkRoot")

```
checkRoot(rt: MerkleTreeDigest): Boolean
```

Tests if the given Merkle tree root is one of the past roots for this Merkle tree. MerkleTreeDigest is defined in CompactStandardLibrary (compact-runtime from Typescript).

*available from Typescript as `checkRoot(rt: MerkleTreeDigest): boolean`*

### insert[​](#insert-3 "Direct link to insert")

```
insert(item: value_type): []
```

Inserts a new leaf at the first free index in this Merkle tree.

### insertHash[​](#inserthash-1 "Direct link to insertHash")

```
insertHash(hash: Bytes<32>): []
```

Inserts a new leaf with a given hash at the first free index in this Merkle tree.

### insertHashIndex[​](#inserthashindex-1 "Direct link to insertHashIndex")

```
insertHashIndex(hash: Bytes<32>, index: Uint<64>): []
```

Inserts a new leaf with a given hash at a specific index in this Merkle tree.

### insertIndex[​](#insertindex-1 "Direct link to insertIndex")

```
insertIndex(item: value_type, index: Uint<64>): []
```

Inserts a new leaf at a specific index in this Merkle tree.

### insertIndexDefault[​](#insertindexdefault-1 "Direct link to insertIndexDefault")

```
insertIndexDefault(index: Uint<64>): []
```

Inserts a default value leaf at a specific index in this Merkle tree. This can be used to emulate a removal from the tree.

### isFull[​](#isfull-1 "Direct link to isFull")

```
isFull(): Boolean
```

Returns if this Merkle tree is full and further items cannot be directly inserted.

*available from Typescript as `isFull(): boolean`*

### resetHistory[​](#resethistory "Direct link to resetHistory")

```
resetHistory(): []
```

Resets the history for this Merkle tree, leaving only the current root valid.

### resetToDefault[​](#resettodefault-6 "Direct link to resetToDefault")

```
resetToDefault(): []
```

Resets this Merkle tree to the empty Merkle tree.

### findPathForLeaf[​](#findpathforleaf-1 "Direct link to findPathForLeaf")

*callable only from TypeScript*

```
findPathForLeaf(leaf: value_type): MerkleTreePath<value_type> | undefined
```

Finds the path for a given leaf in a Merkle tree. Be warned that this is O(n) and should be avoided for large trees. Returns undefined if no such leaf exists. MerkleTreePath is defined in compact-runtime.

### firstFree[​](#firstfree-1 "Direct link to firstFree")

*callable only from TypeScript*

```
firstFree(): bigint
```

Retrieves the first (guaranteed) free index in the Merkle tree.

### history[​](#history "Direct link to history")

*callable only from TypeScript*

```
history(): Iterator<MerkleTreeDigest>
```

An iterator over the roots that are considered valid past roots for this Merkle tree. MerkleTreeDigest is defined in compact-runtime.

### pathForLeaf[​](#pathforleaf-1 "Direct link to pathForLeaf")

*callable only from TypeScript*

```
pathForLeaf(index: bigint, leaf: value_type): MerkleTreePath<value_type>
```

Returns the Merkle path, given the knowledge that a specified leaf is at the given index. It is an error to call this if the index is out of bounds. MerkleTreePath is defined in compact-runtime.

### root[​](#root-1 "Direct link to root")

*callable only from TypeScript*

```
root(): MerkleTreeDigest
```

Retrieves the root of the Merkle tree. MerkleTreeDigest is defined in compact-runtime.
