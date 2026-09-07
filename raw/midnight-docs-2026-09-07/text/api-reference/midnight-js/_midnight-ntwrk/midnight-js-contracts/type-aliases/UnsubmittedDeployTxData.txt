# UnsubmittedDeployTxData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / UnsubmittedDeployTxData

# Type Alias: UnsubmittedDeployTxData\<C>

> **UnsubmittedDeployTxData**<`C`> = [`UnsubmittedDeployTxDataBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedDeployTxDataBase.md)<`C`> & `object`

Data for an unsubmitted deployment transaction.

## Type Declaration[​](#type-declaration "Direct link to Type Declaration")

### private[​](#private "Direct link to private")

> `readonly` **private**: [`UnsubmittedTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedTxData.md) & `object`

The data of this transaction that is only visible on the user device.

#### Type Declaration[​](#type-declaration-1 "Direct link to Type Declaration")

##### initialZswapState[​](#initialzswapstate "Direct link to initialZswapState")

> `readonly` **initialZswapState**: `ZswapLocalState`

The Zswap state produced as a result of running the contract constructor. Useful for when inputs or outputs are created in the contract constructor.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`
