# ContractCallPrototype

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / ContractCallPrototype

# Class: ContractCallPrototype

A [ContractCall](/api-reference/ledger/classes/ContractCall.md) still being assembled

## Constructors[​](#constructors "Direct link to Constructors")

### Constructor[​](#constructor "Direct link to Constructor")

```
new ContractCallPrototype(

   address, 

   entry_point, 

   op, 

   guaranteed_public_transcript, 

   fallible_public_transcript, 

   private_transcript_outputs, 

   input, 

   output, 

   communication_commitment_rand, 

   key_location): ContractCallPrototype;
```

#### Parameters[​](#parameters "Direct link to Parameters")

##### address[​](#address "Direct link to address")

`string`

The address being called

##### entry\_point[​](#entry_point "Direct link to entry_point")

The entry point being called

`string` | `Uint8Array`<`ArrayBufferLike`>

##### op[​](#op "Direct link to op")

[`ContractOperation`](/api-reference/ledger/classes/ContractOperation.md)

The operation expected at this entry point

##### guaranteed\_public\_transcript[​](#guaranteed_public_transcript "Direct link to guaranteed_public_transcript")

The guaranteed transcript computed for this call

`undefined` | [`Transcript`](/api-reference/ledger/type-aliases/Transcript.md)<[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)>

##### fallible\_public\_transcript[​](#fallible_public_transcript "Direct link to fallible_public_transcript")

The fallible transcript computed for this call

`undefined` | [`Transcript`](/api-reference/ledger/type-aliases/Transcript.md)<[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)>

##### private\_transcript\_outputs[​](#private_transcript_outputs "Direct link to private_transcript_outputs")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)\[]

The private transcript recorded for this call

##### input[​](#input "Direct link to input")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

The input(s) provided to this call

##### output[​](#output "Direct link to output")

[`AlignedValue`](/api-reference/ledger/type-aliases/AlignedValue.md)

The output(s) computed from this call

##### communication\_commitment\_rand[​](#communication_commitment_rand "Direct link to communication_commitment_rand")

`string`

The communication randomness used for this call

##### key\_location[​](#key_location "Direct link to key_location")

`string`

An identifier for how the key for this call may be looked up

#### Returns[​](#returns "Direct link to Returns")

`ContractCallPrototype`

## Methods[​](#methods "Direct link to Methods")

### intoCall()[​](#intocall "Direct link to intoCall()")

```
intoCall(parentBinding): ContractCall<PreProof>;
```

#### Parameters[​](#parameters-1 "Direct link to Parameters")

##### parentBinding[​](#parentbinding "Direct link to parentBinding")

[`PreBinding`](/api-reference/ledger/classes/PreBinding.md)

#### Returns[​](#returns-1 "Direct link to Returns")

[`ContractCall`](/api-reference/ledger/classes/ContractCall.md)<[`PreProof`](/api-reference/ledger/classes/PreProof.md)>

***

### toString()[​](#tostring "Direct link to toString()")

```
toString(compact?): string;
```

#### Parameters[​](#parameters-2 "Direct link to Parameters")

##### compact?[​](#compact "Direct link to compact?")

`boolean`

#### Returns[​](#returns-2 "Direct link to Returns")

`string`
