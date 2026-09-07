# UnshieldedUtxos

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / UnshieldedUtxos

# Type Alias: UnshieldedUtxos

> **UnshieldedUtxos** = `object`

Represents a collection of unshielded UTXOs, which are unspent transaction outputs that are not shielded. This type is used to manage and track the state of unshielded UTXOs.

## Properties[​](#properties "Direct link to Properties")

### created[​](#created "Direct link to created")

> `readonly` **created**: readonly [`UnshieldedUtxo`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedUtxo.md)\[]

Represents the unshielded UTXOs that have been created but not yet spent.

***

### spent[​](#spent "Direct link to spent")

> `readonly` **spent**: readonly [`UnshieldedUtxo`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/type-aliases/UnshieldedUtxo.md)\[]

Represents the unshielded UTXOs that have been spent.
