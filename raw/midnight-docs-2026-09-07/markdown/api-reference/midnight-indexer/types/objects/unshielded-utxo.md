# UnshieldedUtxo

> For the complete documentation index, see [llms.txt](/llms.txt)

Represents an unshielded UTXO.

```
type UnshieldedUtxo {

  owner: UnshieldedAddress!

  tokenType: HexEncoded!

  value: String!

  intentHash: HexEncoded!

  outputIndex: Int!

  ctime: Int

  initialNonce: HexEncoded!

  registeredForDustGeneration: Boolean!

  createdAtTransaction: Transaction!

  spentAtTransaction: Transaction

}
```

### Fields[​](#fields "Direct link to Fields")

#### [`UnshieldedUtxo.owner`](#) ● [`UnshieldedAddress!`](/api-reference/midnight-indexer/types/scalars/unshielded-address.md) non-null scalar[​](#unshieldedutxoownerunshieldedaddress-- "Direct link to unshieldedutxoownerunshieldedaddress--")

Owner Bech32m-encoded address.

#### [`UnshieldedUtxo.tokenType`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#unshieldedutxotokentypehexencoded-- "Direct link to unshieldedutxotokentypehexencoded--")

Token hex-encoded serialized token type.

#### [`UnshieldedUtxo.value`](#) ● [`String!`](/api-reference/midnight-indexer/types/scalars/string.md) non-null scalar[​](#unshieldedutxovaluestring-- "Direct link to unshieldedutxovaluestring--")

UTXO value (quantity) as a string to support u128.

#### [`UnshieldedUtxo.intentHash`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#unshieldedutxointenthashhexencoded-- "Direct link to unshieldedutxointenthashhexencoded--")

The hex-encoded serialized intent hash.

#### [`UnshieldedUtxo.outputIndex`](#) ● [`Int!`](/api-reference/midnight-indexer/types/scalars/int.md) non-null scalar[​](#unshieldedutxooutputindexint-- "Direct link to unshieldedutxooutputindexint--")

Index of this output within its creating transaction.

#### [`UnshieldedUtxo.ctime`](#) ● [`Int`](/api-reference/midnight-indexer/types/scalars/int.md) scalar[​](#unshieldedutxoctimeint- "Direct link to unshieldedutxoctimeint-")

The creation time in seconds.

#### [`UnshieldedUtxo.initialNonce`](#) ● [`HexEncoded!`](/api-reference/midnight-indexer/types/scalars/hex-encoded.md) non-null scalar[​](#unshieldedutxoinitialnoncehexencoded-- "Direct link to unshieldedutxoinitialnoncehexencoded--")

The hex-encoded initial nonce for DUST generation tracking.

#### [`UnshieldedUtxo.registeredForDustGeneration`](#) ● [`Boolean!`](/api-reference/midnight-indexer/types/scalars/boolean.md) non-null scalar[​](#unshieldedutxoregisteredfordustgenerationboolean-- "Direct link to unshieldedutxoregisteredfordustgenerationboolean--")

Whether this UTXO is registered for DUST generation.

#### [`UnshieldedUtxo.createdAtTransaction`](#) ● [`Transaction!`](/api-reference/midnight-indexer/types/interfaces/transaction.md) non-null interface[​](#unshieldedutxocreatedattransactiontransaction-- "Direct link to unshieldedutxocreatedattransactiontransaction--")

Transaction that created this UTXO.

#### [`UnshieldedUtxo.spentAtTransaction`](#) ● [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface[​](#unshieldedutxospentattransactiontransaction- "Direct link to unshieldedutxospentattransactiontransaction-")

Transaction that spent this UTXO.

### Member Of[​](#member-of "Direct link to Member Of")

[`RegularTransaction`](/api-reference/midnight-indexer/types/objects/regular-transaction.md) object ● [`SystemTransaction`](/api-reference/midnight-indexer/types/objects/system-transaction.md) object ● [`Transaction`](/api-reference/midnight-indexer/types/interfaces/transaction.md) interface ● [`UnshieldedTransaction`](/api-reference/midnight-indexer/types/objects/unshielded-transaction.md) object
