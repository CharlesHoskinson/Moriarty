# DustLocalState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / DustLocalState

# Class: DustLocalState

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new DustLocalState(params): DustLocalState;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### params[​](#params "Direct link to params")

[`DustParameters`](/api-reference/ledger/classes/DustParameters.md)

#### Returns[​](#returns "Direct link to Returns")

`DustLocalState`

## Properties[​](#properties "Direct link to Properties")

### params[​](#params-1 "Direct link to params")

```
readonly params: DustParameters;
```

***

### syncTime[​](#synctime "Direct link to syncTime")

```
readonly syncTime: Date;
```

***

### utxos[​](#utxos "Direct link to utxos")

```
readonly utxos: QualifiedDustOutput[];
```

## Methods[​](#methods "Direct link to Methods")

### generationInfo()[​](#generationinfo "Direct link to generationInfo()")

```
generationInfo(qdo): 

  | undefined

  | DustGenerationInfo;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### qdo[​](#qdo "Direct link to qdo")

[`QualifiedDustOutput`](/api-reference/ledger/type-aliases/QualifiedDustOutput.md)

#### Returns[​](#returns-1 "Direct link to Returns")

\| `undefined` | [`DustGenerationInfo`](/api-reference/ledger/type-aliases/DustGenerationInfo.md)

***

### processTtls()[​](#processttls "Direct link to processTtls()")

```
processTtls(time): DustLocalState;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### time[​](#time "Direct link to time")

`Date`

#### Returns[​](#returns-2 "Direct link to Returns")

`DustLocalState`

***

### replayEvents()[​](#replayevents "Direct link to replayEvents()")

```
replayEvents(sk, events): DustLocalState;
```

#### Parameters[​](#parameters-3 "Direct link to Parameters")

##### sk[​](#sk "Direct link to sk")

[`DustSecretKey`](/api-reference/ledger/classes/DustSecretKey.md)

##### events[​](#events "Direct link to events")

[`Event`](/api-reference/ledger/classes/Event.md)\[]

#### Returns[​](#returns-3 "Direct link to Returns")

`DustLocalState`

***

### replayEventsWithChanges()[​](#replayeventswithchanges "Direct link to replayEventsWithChanges()")

```
replayEventsWithChanges(sk, events): DustLocalStateWithChanges;
```

#### Parameters[​](#parameters-4 "Direct link to Parameters")

##### sk[​](#sk-1 "Direct link to sk")

[`DustSecretKey`](/api-reference/ledger/classes/DustSecretKey.md)

##### events[​](#events-1 "Direct link to events")

[`Event`](/api-reference/ledger/classes/Event.md)\[]

#### Returns[​](#returns-4 "Direct link to Returns")

[`DustLocalStateWithChanges`](/api-reference/ledger/classes/DustLocalStateWithChanges.md)

***

### serialize()[​](#serialize "Direct link to serialize()")

```
serialize(): Uint8Array;
```

#### Returns[​](#returns-5 "Direct link to Returns")

`Uint8Array`

***

### spend()[​](#spend "Direct link to spend()")

```
spend(

   sk, 

   utxo, 

   vFee, 

   ctime): [DustLocalState, DustSpend<PreProof>];
```

#### Parameters[​](#parameters-5 "Direct link to Parameters")

##### sk[​](#sk-2 "Direct link to sk")

[`DustSecretKey`](/api-reference/ledger/classes/DustSecretKey.md)

##### utxo[​](#utxo "Direct link to utxo")

[`QualifiedDustOutput`](/api-reference/ledger/type-aliases/QualifiedDustOutput.md)

##### vFee[​](#vfee "Direct link to vFee")

`bigint`

##### ctime[​](#ctime "Direct link to ctime")

`Date`

#### Returns[​](#returns-6 "Direct link to Returns")

\[`DustLocalState`, [`DustSpend`](/api-reference/ledger/classes/DustSpend.md)<[`PreProof`](/api-reference/ledger/classes/PreProof.md)>]

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-6 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-7 "Direct link to Returns")

`string`

***

### walletBalance()[​](#walletbalance "Direct link to walletBalance()")

```
walletBalance(time): bigint;
```

#### Parameters[​](#parameters-7 "Direct link to Parameters")

##### time[​](#time-1 "Direct link to time")

`Date`

#### Returns[​](#returns-8 "Direct link to Returns")

`bigint`

***

### deserialize()[​](#deserialize "Direct link to deserialize()")

```
static deserialize(raw): DustLocalState;
```

#### Parameters[​](#parameters-8 "Direct link to Parameters")

##### raw[​](#raw "Direct link to raw")

`Uint8Array`

#### Returns[​](#returns-9 "Direct link to Returns")

`DustLocalState`
