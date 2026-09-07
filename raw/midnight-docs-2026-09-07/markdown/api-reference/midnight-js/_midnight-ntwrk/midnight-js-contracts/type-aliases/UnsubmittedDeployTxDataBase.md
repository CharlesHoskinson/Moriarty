# UnsubmittedDeployTxDataBase

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / UnsubmittedDeployTxDataBase

# Type Alias: UnsubmittedDeployTxDataBase\<C>

> **UnsubmittedDeployTxDataBase**<`C`> = `object`

Base type for data relevant to an unsubmitted deployment transaction.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

## Properties[​](#properties "Direct link to Properties")

### private[​](#private "Direct link to private")

> `readonly` **private**: [`UnsubmittedDeployTxPrivateData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxPrivateData.md)<`C`>

The private data (data that will not be revealed upon tx submission) relevant to the deployment transaction.

***

### public[​](#public "Direct link to public")

> `readonly` **public**: [`UnsubmittedDeployTxPublicData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxPublicData.md)

The public data (data that will be revealed upon tx submission) relevant to the deployment transaction.
