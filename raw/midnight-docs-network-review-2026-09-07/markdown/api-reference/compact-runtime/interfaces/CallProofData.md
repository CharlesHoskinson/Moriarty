# CallProofData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / CallProofData

# Interface: CallProofData

Encapsulates the data required to produce a zero-knowledge proof

## Extends[​](#extends "Direct link to Extends")

* [`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md)

## Properties[​](#properties "Direct link to Properties")

### circuitId[​](#circuitid "Direct link to circuitId")

```
circuitId: string;
```

The ID of the circuit that was called.

***

### commCommData?[​](#commcommdata "Direct link to commCommData?")

```
optional commCommData: CommunicationCommitmentData;
```

Data included by the parent call only if this was a sub-call

***

### contractAddress[​](#contractaddress "Direct link to contractAddress")

```
contractAddress: string;
```

The address of the contract defining the circuit for which this proof data is pertinent.

***

### finalQueryContext[​](#finalquerycontext "Direct link to finalQueryContext")

```
finalQueryContext: QueryContext;
```

The ledger state of the contract when the circuit finished.

***

### initialQueryContext[​](#initialquerycontext "Direct link to initialQueryContext")

```
initialQueryContext: QueryContext;
```

The ledger state of the contract before the circuit was called.

***

### input[​](#input "Direct link to input")

```
input: AlignedValue;
```

The inputs to a circuit

#### Inherited from[​](#inherited-from "Direct link to Inherited from")

[`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md).[`input`](/api-reference/compact-runtime/interfaces/ProofData.md#input)

***

### output[​](#output "Direct link to output")

```
output: AlignedValue;
```

The outputs from a circuit

#### Inherited from[​](#inherited-from-1 "Direct link to Inherited from")

[`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md).[`output`](/api-reference/compact-runtime/interfaces/ProofData.md#output)

***

### privateTranscriptOutputs[​](#privatetranscriptoutputs "Direct link to privateTranscriptOutputs")

```
privateTranscriptOutputs: AlignedValue[];
```

The transcript of the witness call outputs

#### Inherited from[​](#inherited-from-2 "Direct link to Inherited from")

[`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md).[`privateTranscriptOutputs`](/api-reference/compact-runtime/interfaces/ProofData.md#privatetranscriptoutputs)

***

### publicTranscript[​](#publictranscript "Direct link to publicTranscript")

```
publicTranscript: Op<AlignedValue>[];
```

The public transcript of operations

#### Inherited from[​](#inherited-from-3 "Direct link to Inherited from")

[`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md).[`publicTranscript`](/api-reference/compact-runtime/interfaces/ProofData.md#publictranscript)

***

### zswapLocalState[​](#zswaplocalstate "Direct link to zswapLocalState")

```
zswapLocalState: EncodedZswapLocalState;
```

The Zswap local state this contract accumulated during the call — the shielded coins it consumed and produced. Recorded per call, not just for the root, so transaction assembly can build one offer contribution per call and bind each contract-owned input and output to the contract that actually made it.
