# createUnprovenCallTx

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / createUnprovenCallTx

# Function: createUnprovenCallTx()

Calls a circuit using states fetched from the public data provider and private state provider, then creates an unbalanced, unproven, unsubmitted, call transaction.

## Param[​](#param "Direct link to Param")

The providers to use to create the call transaction.

## Param[​](#param-1 "Direct link to Param")

Configuration.

## Param[​](#param-2 "Direct link to Param")

Optional scoped transaction context to participate in an existing transaction scope.

## Throws[​](#throws "Direct link to Throws")

IncompleteCallTxPrivateStateConfig If a `privateStateId` was given but a `privateStateProvider` was not. We assume that when a user gives a `privateStateId`, they want to update the private state store.

## Call Signature[​](#call-signature "Direct link to Call Signature")

> **createUnprovenCallTx**<`C`, `PCK`>(`providers`, `options`, `transactionContext?`): `Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters "Direct link to Type Parameters")

#### C[​](#c "Direct link to C")

`C` *extends* `Contract`<`undefined`, `Witnesses`<`undefined`>>

#### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters "Direct link to Parameters")

#### providers[​](#providers "Direct link to providers")

[`UnprovenCallTxProvidersBase`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenCallTxProvidersBase.md)

#### options[​](#options "Direct link to options")

[`CallOptionsWithArguments`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallOptionsWithArguments.md)<`C`, `PCK`>

#### transactionContext?[​](#transactioncontext "Direct link to transactionContext?")

[`TransactionContext`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/TransactionContext.md)<`C`, `PCK`>

### Returns[​](#returns "Direct link to Returns")

`Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>

## Call Signature[​](#call-signature-1 "Direct link to Call Signature")

> **createUnprovenCallTx**<`C`, `PCK`>(`providers`, `options`, `transactionContext?`): `Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>

### Type Parameters[​](#type-parameters-1 "Direct link to Type Parameters")

#### C[​](#c-1 "Direct link to C")

`C` *extends* `Any`

#### PCK[​](#pck-1 "Direct link to PCK")

`PCK` *extends* `string`

### Parameters[​](#parameters-1 "Direct link to Parameters")

#### providers[​](#providers-1 "Direct link to providers")

[`UnprovenCallTxProvidersWithPrivateState`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnprovenCallTxProvidersWithPrivateState.md)<`C`>

#### options[​](#options-1 "Direct link to options")

[`CallTxOptionsWithPrivateStateId`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/CallTxOptionsWithPrivateStateId.md)<`C`, `PCK`>

#### transactionContext?[​](#transactioncontext-1 "Direct link to transactionContext?")

[`TransactionContext`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/interfaces/TransactionContext.md)<`C`, `PCK`>

### Returns[​](#returns-1 "Direct link to Returns")

`Promise`<[`UnsubmittedCallTxData`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/type-aliases/UnsubmittedCallTxData.md)<`C`, `PCK`>>
