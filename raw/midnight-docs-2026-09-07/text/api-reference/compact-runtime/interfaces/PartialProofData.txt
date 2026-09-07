# PartialProofData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / PartialProofData

# Interface: PartialProofData

Encapsulates the data required to produce a zero-knowledge proof except the circuit output

## Extended by[​](#extended-by "Direct link to Extended by")

* [`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md)

## Properties[​](#properties "Direct link to Properties")

### input[​](#input "Direct link to input")

```
input: AlignedValue;
```

The inputs to a circuit

***

### privateTranscriptOutputs[​](#privatetranscriptoutputs "Direct link to privateTranscriptOutputs")

```
privateTranscriptOutputs: AlignedValue[];
```

The transcript of the witness call outputs

***

### publicTranscript[​](#publictranscript "Direct link to publicTranscript")

```
publicTranscript: Op<AlignedValue>[];
```

The public transcript of operations
