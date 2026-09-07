# ProofData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / ProofData

# Interface: ProofData

Encapsulates the data required to produce a zero-knowledge proof

## Extends[​](#extends "Direct link to Extends")

* [`PartialProofData`](/api-reference/compact-runtime/interfaces/PartialProofData.md)

## Extended by[​](#extended-by "Direct link to Extended by")

* [`CallProofData`](/api-reference/compact-runtime/interfaces/CallProofData.md)

## Properties[​](#properties "Direct link to Properties")

### input[​](#input "Direct link to input")

```
input: AlignedValue;
```

The inputs to a circuit

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`PartialProofData`](/api-reference/compact-runtime/interfaces/PartialProofData.md).[`input`](/api-reference/compact-runtime/interfaces/PartialProofData.md#input)

***

### output[​](#output "Direct link to output")

```
output: AlignedValue;
```

The outputs from a circuit

***

### privateTranscriptOutputs[​](#privatetranscriptoutputs "Direct link to privateTranscriptOutputs")

```
privateTranscriptOutputs: AlignedValue[];
```

The transcript of the witness call outputs

#### Inherited from[​](#inherited-from-1 "Direct link to Inherited from")

[`PartialProofData`](/api-reference/compact-runtime/interfaces/PartialProofData.md).[`privateTranscriptOutputs`](/api-reference/compact-runtime/interfaces/PartialProofData.md#privatetranscriptoutputs)

***

### publicTranscript[​](#publictranscript "Direct link to publicTranscript")

```
publicTranscript: Op<AlignedValue>[];
```

The public transcript of operations

#### Inherited from[​](#inherited-from-2 "Direct link to Inherited from")

[`PartialProofData`](/api-reference/compact-runtime/interfaces/PartialProofData.md).[`publicTranscript`](/api-reference/compact-runtime/interfaces/PartialProofData.md#publictranscript)
