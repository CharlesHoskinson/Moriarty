# IncompleteFindContractPrivateStateConfig

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / IncompleteFindContractPrivateStateConfig

# Class: IncompleteFindContractPrivateStateConfig

An error indicating that an initial private state was specified for a contract find while a private state ID was not. We can't store the initial private state if we don't have a private state ID, and we need to let the user know that.

## Extends[​](#extends "Direct link to Extends")

* `Error`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new IncompleteFindContractPrivateStateConfig**(): `IncompleteFindContractPrivateStateConfig`

#### Returns[​](#returns "Direct link to Returns")

`IncompleteFindContractPrivateStateConfig`

#### Overrides[​](#overrides "Direct link to Overrides")

`Error.constructor`
