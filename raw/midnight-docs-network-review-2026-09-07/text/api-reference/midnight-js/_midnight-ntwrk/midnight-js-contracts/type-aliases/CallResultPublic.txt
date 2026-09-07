# CallResultPublic

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallResultPublic

# Type Alias: CallResultPublic

> **CallResultPublic** = `object`

The public portions of the call result.

## Properties[​](#properties "Direct link to Properties")

### nextContractState[​](#nextcontractstate "Direct link to nextContractState")

> `readonly` **nextContractState**: `StateValue`

The public state resulting from executing the circuit.

***

### partitionedTranscript[​](#partitionedtranscript "Direct link to partitionedTranscript")

> `readonly` **partitionedTranscript**: `PartitionedTranscript`

A [publicTranscript](#publictranscript) partitioned into guaranteed and fallible sections. The guaranteed section of a public transcript must succeed for the corresponding transaction to be considered valid. The fallible section of a public transcript can fail without invalidating the transaction, as long as the guaranteed section succeeds.

***

### publicTranscript[​](#publictranscript "Direct link to publicTranscript")

> `readonly` **publicTranscript**: `Op`<`AlignedValue`>\[]

The public transcript resulting from executing the circuit.
