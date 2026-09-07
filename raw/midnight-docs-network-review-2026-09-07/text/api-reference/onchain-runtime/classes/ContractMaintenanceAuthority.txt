# ContractMaintenanceAuthority

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/onchain-runtime v3.0.0**](/api-reference/onchain-runtime.md)

***

[@midnight-ntwrk/onchain-runtime](/api-reference/onchain-runtime/globals.md) / ContractMaintenanceAuthority

# Class: ContractMaintenanceAuthority

A committee permitted to make changes to this contract. If a threshold of the public keys in this committee sign off, they can change the rules of this contract, or recompile it for a new version.

If the threshold is greater than the number of committee members, it is impossible for them to sign anything.

## Constructors[​](#constructors "Direct link to Constructors")

### new ContractMaintenanceAuthority()[​](#new-contractmaintenanceauthority "Direct link to new ContractMaintenanceAuthority()")

```
new ContractMaintenanceAuthority(

   committee, 

   threshold, 

   counter?): ContractMaintenanceAuthority
```

Constructs a new authority from its components

If not supplied, `counter` will default to `0n`. Values should be non-negative, and at most 2^32 - 1.

At deployment, `counter` must be `0n`, and any subsequent update should set counter to exactly one greater than the current value.

#### Parameters[​](#parameters "Direct link to Parameters")

##### committee[​](#committee "Direct link to committee")

`string`\[]

##### threshold[​](#threshold "Direct link to threshold")

`number`

##### counter?[​](#counter "Direct link to counter?")

`bigint`

#### Returns[​](#returns "Direct link to Returns")

[`ContractMaintenanceAuthority`](/api-reference/onchain-runtime/classes/ContractMaintenanceAuthority.md)

## Properties[​](#properties "Direct link to Properties")

### committee[​](#committee-1 "Direct link to committee")

```
readonly committee: string[];
```

The committee public keys

***

### counter[​](#counter-1 "Direct link to counter")

```
readonly counter: bigint;
```

The replay protection counter

***

### threshold[​](#threshold-1 "Direct link to threshold")

```
readonly threshold: number;
```

How many keys must sign rule changes

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): ContractState
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

[`ContractState`](/api-reference/onchain-runtime/classes/ContractState.md)
