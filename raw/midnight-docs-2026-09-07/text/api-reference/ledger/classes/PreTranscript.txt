# PreTranscript

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / PreTranscript

# Class: PreTranscript

A transcript prior to partitioning, consisting of the context to run it in, the program that will make up the transcript, and optionally a communication commitment to bind calls together.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new PreTranscript(

   context, 

   program, 

   comm_comm?): PreTranscript;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### context[​](#context "Direct link to context")

[`QueryContext`](/api-reference/ledger/classes/QueryContext.md)

##### program[​](#program "Direct link to program")

[`Op`](/api-reference/ledger/type-aliases/Op.md)<[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)>\[]

##### comm\_comm?[​](#comm_comm "Direct link to comm_comm?")

`string`

#### Returns[​](#returns "Direct link to Returns")

`PreTranscript`

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
