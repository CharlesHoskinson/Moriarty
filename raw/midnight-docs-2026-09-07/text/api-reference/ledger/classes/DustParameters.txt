# DustParameters

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustParameters

# Class: DustParameters

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new DustParameters(

   nightDustRatio, 

   generationDecayRate, 

   dustGracePeriodSeconds): DustParameters;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### nightDustRatio[​](#nightdustratio "Direct link to nightDustRatio")

`bigint`

##### generationDecayRate[​](#generationdecayrate "Direct link to generationDecayRate")

`bigint`

##### dustGracePeriodSeconds[​](#dustgraceperiodseconds "Direct link to dustGracePeriodSeconds")

`bigint`

#### Returns[​](#returns "Direct link to Returns")

`DustParameters`

## Properties[​](#properties "Direct link to Properties")

### dustGracePeriodSeconds[​](#dustgraceperiodseconds-1 "Direct link to dustGracePeriodSeconds")

```
dustGracePeriodSeconds: bigint;
```

***

### generationDecayRate[​](#generationdecayrate-1 "Direct link to generationDecayRate")

```
generationDecayRate: bigint;
```

***

### nightDustRatio[​](#nightdustratio-1 "Direct link to nightDustRatio")

```
nightDustRatio: bigint;
```

***

### timeToCapSeconds[​](#timetocapseconds "Direct link to timeToCapSeconds")

```
readonly timeToCapSeconds: bigint;
```

## Methods[​](#methods "Direct link to Methods")

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-1 "Direct link to Returns")

`Uint8Array`

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): DustParameters;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-3 "Direct link to Returns")

`DustParameters`
