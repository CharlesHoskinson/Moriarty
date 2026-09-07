# ContractState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ContractState

# Class: ContractState

The state of a contract, consisting primarily of the [data](#data) accessible directly to the contract, and the map of [ContractOperation](/api-reference/compact-runtime/classes/ContractOperation.md)s that can be called on it, the keys of which can be accessed with [operations](#operations), and the individual operations can be read with [operation](#operation) and written to with [setOperation](#setoperation).

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ContractState(): ContractState;
```

Creates a blank contract state

#### Returns[​](#returns "Direct link to Returns")

`ContractState`

## Properties[​](#properties "Direct link to Properties")

### balance[​](#balance "Direct link to balance")

```
balance: Map<TokenType, bigint>;
```

The public balances held by this contract

***

### data[​](#data "Direct link to data")

```
data: ChargedState;
```

The current value of the primary state of the contract

***

### maintenanceAuthority[​](#maintenanceauthority "Direct link to maintenanceAuthority")

```
maintenanceAuthority: ContractMaintenanceAuthority;
```

The maintenance authority associated with this contract

## Methods[​](#methods "Direct link to Methods")

### operation()[​](#operation "Direct link to operation()")

```
operation(operation): ContractOperation | undefined;
```

Get the operation at a specific entry point name

#### Parameters[​](#parameters "Direct link to Parameters")

##### operation[​](#operation-1 "Direct link to operation")

`string` | `Uint8Array`<`ArrayBufferLike`>

#### Returns[​](#returns-1 "Direct link to Returns")

[`ContractOperation`](/api-reference/compact-runtime/classes/ContractOperation.md) | `undefined`

***

### operations()[​](#operations "Direct link to operations()")

```
operations(): (string | Uint8Array<ArrayBufferLike>)[];
```

Return a list of the entry points currently registered on this contract

#### Returns[​](#returns-2 "Direct link to Returns")

(`string` | `Uint8Array`<`ArrayBufferLike`>)\[]

***

### query()[​](#query "Direct link to query()")

```
query(query, cost_model): GatherResult[];
```

Runs a series of operations against the current state, and returns the results

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### query[​](#query-1 "Direct link to query")

[`Op`](/api-reference/compact-runtime/type-aliases/Op.md)<`null`>\[]

##### cost\_model[​](#cost_model "Direct link to cost_model")

[`CostModel`](/api-reference/compact-runtime/classes/CostModel.md)

#### Returns[​](#returns-3 "Direct link to Returns")

[`GatherResult`](/api-reference/compact-runtime/type-aliases/GatherResult.md)\[]

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-4 "Direct link to Returns")

`Uint8Array`

***

### setOperation()[​](#setoperation "Direct link to setOperation()")

```
setOperation(operation, value): void;
```

Set a specific entry point name to contain a given operation

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### operation[​](#operation-2 "Direct link to operation")

`string` | `Uint8Array`<`ArrayBufferLike`>

##### value[​](#value "Direct link to value")

[`ContractOperation`](/api-reference/compact-runtime/classes/ContractOperation.md)

#### Returns[​](#returns-5 "Direct link to Returns")

`void`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-6 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): ContractState;
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-7 "Direct link to Returns")

`ContractState`
