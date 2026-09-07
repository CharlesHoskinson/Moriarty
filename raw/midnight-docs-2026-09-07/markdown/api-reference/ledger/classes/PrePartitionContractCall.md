# PrePartitionContractCall

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / PrePartitionContractCall

# Class: PrePartitionContractCall

A [ContractCall](/api-reference/ledger/classes/ContractCall.md) prior to being partitioned into guarnateed and fallible parts, for use with [Transaction.addCalls](/api-reference/ledger/classes/Transaction.md#addcalls).

Note that this is similar, but not the same as [ContractCall](/api-reference/ledger/classes/ContractCall.md), which assumes [partitionTranscripts](/api-reference/ledger/functions/partitionTranscripts.md) was already used. [Transaction.addCalls](/api-reference/ledger/classes/Transaction.md#addcalls) is a replacement for this that also handles Zswap components, and creates relevant intents when needed.

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new PrePartitionContractCall(

   address, 

   entry_point, 

   op, 

   pre_transcript, 

   private_transcript_outputs, 

   input, 

   output, 

   communication_commitment_rand, 

   key_location): PrePartitionContractCall;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### address[​](#address "Direct link to address")

`string`

##### entry\_point[​](#entry_point "Direct link to entry_point")

`string` | `Uint8Array`<`ArrayBufferLike`>

##### op[​](#op "Direct link to op")

[`ContractOperation`](/api-reference/ledger/classes/ContractOperation.md)

##### pre\_transcript[​](#pre_transcript "Direct link to pre_transcript")

[`PreTranscript`](/api-reference/ledger/classes/PreTranscript.md)

##### private\_transcript\_outputs[​](#private_transcript_outputs "Direct link to private_transcript_outputs")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)\[]

##### input[​](#input "Direct link to input")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

##### output[​](#output "Direct link to output")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

##### communication\_commitment\_rand[​](#communication_commitment_rand "Direct link to communication_commitment_rand")

`string`

##### key\_location[​](#key_location "Direct link to key_location")

`string`

#### Returns[​](#returns "Direct link to Returns")

`PrePartitionContractCall`

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
