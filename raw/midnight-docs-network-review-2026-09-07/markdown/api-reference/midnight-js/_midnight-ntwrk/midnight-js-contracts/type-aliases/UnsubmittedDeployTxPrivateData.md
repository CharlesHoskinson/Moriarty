# UnsubmittedDeployTxPrivateData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / UnsubmittedDeployTxPrivateData

# Type Alias: UnsubmittedDeployTxPrivateData\<C>

> **UnsubmittedDeployTxPrivateData**<`C`> = `object`

Base type for private data relevant to an unsubmitted deployment transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

## Properties[​](#properties "Direct link to Properties")

### initialPrivateState[​](#initialprivatestate "Direct link to initialPrivateState")

> `readonly` **initialPrivateState**: `Contract.PrivateState`<`C`>

The initial private state of the contract deployed to the blockchain. This value is persisted if the transaction succeeds.

***

### signingKey[​](#signingkey "Direct link to signingKey")

> `readonly` **signingKey**: `SigningKey`

The signing key that was added as the deployed contract's maintenance authority.
