# ContractDeploy

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ContractDeploy

# Class: ContractDeploy

A contract deployment segment, instructing the creation of a new contract address, if not already present

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ContractDeploy(initial_state): ContractDeploy;
```

Creates a deployment for an arbitrary contract state

The deployment and its address are randomised.

#### Parameters[​](#parameters "Direct link to Parameters")

##### initial\_state[​](#initial_state "Direct link to initial_state")

[`ContractState`](/api-reference/ledger/classes/ContractState.md)

#### Returns[​](#returns "Direct link to Returns")

`ContractDeploy`

## Properties[​](#properties "Direct link to Properties")

### address[​](#address "Direct link to address")

```
readonly address: string;
```

The address this deployment will attempt to create

***

### initialState[​](#initialstate "Direct link to initialState")

```
readonly initialState: ContractState;
```

## Methods[​](#methods "Direct link to Methods")

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-1 "Direct link to Returns")

`string`
