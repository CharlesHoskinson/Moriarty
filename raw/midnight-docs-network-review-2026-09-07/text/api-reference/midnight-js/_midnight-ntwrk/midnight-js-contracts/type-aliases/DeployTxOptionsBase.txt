# DeployTxOptionsBase

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployTxOptionsBase

# Type Alias: DeployTxOptionsBase\<C>

> **DeployTxOptionsBase**<`C`> = [`ContractConstructorOptionsWithArguments`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithArguments.md)<`C`> & `object`

Base type for deploy transaction configuration.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### additionalCoinEncPublicKeyMappings?[​](#additionalcoinencpublickeymappings "Direct link to additionalCoinEncPublicKeyMappings?")

> `readonly` `optional` **additionalCoinEncPublicKeyMappings?**: `ReadonlyMap`<`CoinPublicKey`, `EncPublicKey`>

An optional mapping of CoinPublicKey to EncPublicKey that can be used to resolve encryption keys for coins created in the contract constructor. This is useful in cases where the constructor creates outputs to addresses that don't belong to the current user.

### signingKey[​](#signingkey "Direct link to signingKey")

> `readonly` **signingKey**: `SigningKey`

The signing key to add as the to-be-deployed contract's maintenance authority.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
