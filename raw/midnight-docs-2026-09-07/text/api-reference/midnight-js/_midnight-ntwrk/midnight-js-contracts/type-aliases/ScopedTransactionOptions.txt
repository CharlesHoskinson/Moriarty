# ScopedTransactionOptions

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / ScopedTransactionOptions

# Type Alias: ScopedTransactionOptions

> **ScopedTransactionOptions** = `object`

Options for use when creating scoped transactions.

## Properties[​](#properties "Direct link to Properties")

### additionalCoinEncPublicKeyMappings?[​](#additionalcoinencpublickeymappings "Direct link to additionalCoinEncPublicKeyMappings?")

> `readonly` `optional` **additionalCoinEncPublicKeyMappings?**: `ReadonlyMap`<`CoinPublicKey`, `EncPublicKey`>

An optional mapping of CoinPublicKey to EncPublicKey that can be used to resolve encryption keys for coins created during circuit execution.

***

### scopeName?[​](#scopename "Direct link to scopeName?")

> `readonly` `optional` **scopeName?**: `string`

An optional name for the transaction scope.
