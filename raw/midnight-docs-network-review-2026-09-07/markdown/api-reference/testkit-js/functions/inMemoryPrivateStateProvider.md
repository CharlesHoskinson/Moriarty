# inMemoryPrivateStateProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/testkit-js v4.0.4**](/api-reference/testkit-js.md)

***

> **inMemoryPrivateStateProvider**<`PSI`, `PS`>(): `PrivateStateProvider`<`PSI`, `PS`>

A simple in-memory implementation of private state provider. Makes it easy to capture and rewrite private state from deploy.

Note: Unlike `levelPrivateStateProvider`, this provider has no storage password configured. Therefore, export/import operations always require an explicit password in the options.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### PSI[​](#psi "Direct link to PSI")

`PSI` *extends* `string`

Type of the private state identifier.

### PS[​](#ps "Direct link to PS")

`PS` *extends* `unknown`

Type of the private state.

## Returns[​](#returns "Direct link to Returns")

`PrivateStateProvider`<`PSI`, `PS`>

An in-memory private state provider.
