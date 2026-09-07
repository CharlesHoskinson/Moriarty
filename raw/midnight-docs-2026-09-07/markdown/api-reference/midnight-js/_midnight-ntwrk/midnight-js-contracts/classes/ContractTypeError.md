# ContractTypeError

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / ContractTypeError

# Class: ContractTypeError

The error that is thrown when there is a contract type mismatch between a given contract type, and the initial state that is deployed at a given contract address.

## Remarks[​](#remarks "Direct link to Remarks")

This error is typically thrown during calls to [findDeployedContract](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts/functions/findDeployedContract.md) where the supplied contract address represents a different type of contract to the contract type given.

## Extends[​](#extends "Direct link to Extends")

* `TypeError`

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

> **new ContractTypeError**(`contractState`, `circuitIds`): `ContractTypeError`

Initializes a new ContractTypeError.

#### Parameters[​](#parameters "Direct link to Parameters")

##### contractState[​](#contractstate "Direct link to contractState")

`ContractState`

The initial deployed contract state.

##### circuitIds[​](#circuitids "Direct link to circuitIds")

`string`\[]

The circuits that are undefined, or have a verifier key mismatch with the key present in `contractState`.

#### Returns[​](#returns "Direct link to Returns")

`ContractTypeError`

#### Overrides[​](#overrides "Direct link to Overrides")

`TypeError.constructor`

## Properties[​](#properties "Direct link to Properties")

### circuitIds[​](#circuitids-1 "Direct link to circuitIds")

> `readonly` **circuitIds**: `string`\[]

The circuits that are undefined, or have a verifier key mismatch with the key present in `contractState`.

***

### contractState[​](#contractstate-1 "Direct link to contractState")

> `readonly` **contractState**: `ContractState`

The initial deployed contract state.
