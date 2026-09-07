# DeployContractOptionsBase

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / DeployContractOptionsBase

# Type Alias: DeployContractOptionsBase\<C>

> **DeployContractOptionsBase**<`C`> = [`ContractConstructorOptionsWithArguments`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithArguments.md)<`C`> & `object`

Base type for configuration for [deployContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/deployContract.md); identical to [ContractConstructorOptionsWithArguments](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/ContractConstructorOptionsWithArguments.md) except the `signingKey` is now optional, since [deployContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/deployContract.md) will generate a fresh signing key in the event that `signingKey` is undefined.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### additionalCoinEncPublicKeyMappings?[​](#additionalcoinencpublickeymappings "Direct link to additionalCoinEncPublicKeyMappings?")

> `readonly` `optional` **additionalCoinEncPublicKeyMappings?**: `ReadonlyMap`<`CoinPublicKey`, `EncPublicKey`>

An optional mapping of CoinPublicKey to EncPublicKey that can be used to resolve encryption keys for coins created in the contract constructor. This is useful in cases where the constructor creates outputs to addresses that don't belong to the current user.

### signingKey?[​](#signingkey "Direct link to signingKey?")

> `readonly` `optional` **signingKey?**: `SigningKey`

The signing key to add as the to-be-deployed contract's maintenance authority. If undefined, a new signing key is sampled and used as the CMA then stored in the private state provider under the newly deployed contract's address. Otherwise, the passed signing key is added as the CMA. The second case is useful when you want to use the same CMA for two different contracts.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
