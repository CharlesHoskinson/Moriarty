# CallResultPrivate

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-contracts](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-contracts.md) / CallResultPrivate

# Type Alias: CallResultPrivate\<C, PCK>

> **CallResultPrivate**<`C`, `PCK`> = `object`

The private (sensitive) portions of the call result.

## Type Parameters[​](#type-parameters "Direct link to Type Parameters")

### C[​](#c "Direct link to C")

`C` *extends* `Contract.Any`

### PCK[​](#pck "Direct link to PCK")

`PCK` *extends* `Contract.ProvableCircuitId`<`C`>

## Properties[​](#properties "Direct link to Properties")

### input[​](#input "Direct link to input")

> `readonly` **input**: `AlignedValue`

ZK representation of the circuit arguments.

***

### nextPrivateState[​](#nextprivatestate "Direct link to nextPrivateState")

> `readonly` **nextPrivateState**: `Contract.PrivateState`<`C`>

The private state resulting from executing the circuit.

***

### nextZswapLocalState[​](#nextzswaplocalstate "Direct link to nextZswapLocalState")

> `readonly` **nextZswapLocalState**: `ZswapLocalState`

The Zswap local state resulting from executing the circuit.

***

### output[​](#output "Direct link to output")

> `readonly` **output**: `AlignedValue`

ZK representation of the circuit result.

***

### privateTranscriptOutputs[​](#privatetranscriptoutputs "Direct link to privateTranscriptOutputs")

> `readonly` **privateTranscriptOutputs**: `AlignedValue`\[]

ZK representation of the circuit witness call results.

***

### result[​](#result "Direct link to result")

> `readonly` **result**: `Contract.CircuitReturnType`<`C`, `PCK`>

The JS representation of the input to the circuit.
