# UnshieldedUtxo

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / UnshieldedUtxo

# Type Alias: UnshieldedUtxo

> **UnshieldedUtxo** = `object`

Represents an unshielded UTXO (Unspent Transaction Output). Unshielded UTXOs are outputs that have not been shielded or encrypted, making them visible on the public ledger.

## Properties[​](#properties "Direct link to Properties")

### intentHash[​](#intenthash "Direct link to intentHash")

> `readonly` **intentHash**: `IntentHash`

The identifier of the intent associated with the unshielded UTXO. This is used to track the intent behind the creation or use of the UTXO.

***

### owner[​](#owner "Direct link to owner")

> `readonly` **owner**: `ContractAddress`

The unique identifier of the unshielded UTXO.

***

### tokenType[​](#tokentype "Direct link to tokenType")

> `readonly` **tokenType**: `RawTokenType`

The type of token associated with the unshielded UTXO. This indicates the kind of asset or currency represented by the UTXO.

***

### value[​](#value "Direct link to value")

> `readonly` **value**: `bigint`

The value of the unshielded UTXO, represented as a bigint.
