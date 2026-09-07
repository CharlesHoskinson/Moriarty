# queryLedgerState

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.19.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / queryLedgerState

# Function: queryLedgerState()

```
function queryLedgerState(

   circuitContext, 

   partialProofData, 

   program): AlignedValue | undefined;
```

Runs a program (query) against the current ledger state in the given circuit context. Records the transcript in the given partial proof data.

## Parameters[​](#parameters "Direct link to Parameters")

### circuitContext[​](#circuitcontext "Direct link to circuitContext")

[`CircuitContext`](/api-reference/compact-runtime/interfaces/CircuitContext.md)

The context for the currently executing circuit.

### partialProofData[​](#partialproofdata "Direct link to partialProofData")

[`PartialProofData`](/api-reference/compact-runtime/interfaces/PartialProofData.md)

The partial proof data to insert the query results into.

### program[​](#program "Direct link to program")

[`Op`](/api-reference/compact-runtime/type-aliases/Op.md)<`null`>\[]

The query to run.

## Returns[​](#returns "Direct link to Returns")

[`AlignedValue`](/api-reference/compact-runtime/type-aliases/AlignedValue.md) | `undefined`
