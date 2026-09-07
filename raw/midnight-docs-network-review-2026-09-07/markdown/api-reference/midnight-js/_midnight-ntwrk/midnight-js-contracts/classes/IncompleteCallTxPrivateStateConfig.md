# IncompleteCallTxPrivateStateConfig

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / IncompleteCallTxPrivateStateConfig

# Class: IncompleteCallTxPrivateStateConfig

An error indicating that a private state ID was specified for a call transaction while a private state provider was not. We want to let the user know so that they aren't under the impression the private state of a contract was updated when it wasn't.

## Extends[​](#extends "Direct link to Extends")

* `Error`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new IncompleteCallTxPrivateStateConfig**(): `IncompleteCallTxPrivateStateConfig`

#### Returns[​](#returns "Direct link to Returns")

`IncompleteCallTxPrivateStateConfig`

#### Overrides[​](#overrides "Direct link to Overrides")

`Error.constructor`
