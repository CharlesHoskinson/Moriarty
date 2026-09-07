# partitionTranscripts

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight/ledger v8.0.3**](/api-reference/ledger.md)

***

[@midnight/ledger](/api-reference/ledger/globals.md) / partitionTranscripts

# Function: partitionTranscripts()

```
function partitionTranscripts(calls, params): PartitionedTranscript[];
```

Finalizes a set of programs against their initial contexts, resulting in guaranteed and fallible [Transcript](/api-reference/ledger/type-aliases/Transcript.md)s, optimally allocated, and heuristically covered for gas fees.

## Parameters[​](#parameters "Direct link to Parameters")

### calls[​](#calls "Direct link to calls")

[`PreTranscript`](/api-reference/ledger/classes/PreTranscript.md)\[]

### params[​](#params "Direct link to params")

[`LedgerParameters`](/api-reference/ledger/classes/LedgerParameters.md)

## Returns[​](#returns "Direct link to Returns")

[`PartitionedTranscript`](/api-reference/ledger/type-aliases/PartitionedTranscript.md)\[]
