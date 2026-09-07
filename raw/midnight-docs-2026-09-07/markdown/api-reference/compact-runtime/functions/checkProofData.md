# checkProofData

> For the complete documentation index, see [llms.txt](/llms.txt)

[**@midnight-ntwrk/compact-runtime v0.9.0**](/api-reference/compact-runtime.md)

***

[@midnight-ntwrk/compact-runtime](/api-reference/compact-runtime/globals.md) / checkProofData

# Function: checkProofData()

```
function checkProofData(zkir, proofData): void;
```

Verifies a given [ProofData](/api-reference/compact-runtime/interfaces/ProofData.md) satisfies the constrains of a ZK circuit descripted by given IR

## Parameters[​](#parameters "Direct link to Parameters")

### zkir[​](#zkir "Direct link to zkir")

`string`

### proofData[​](#proofdata "Direct link to proofData")

[`ProofData`](/api-reference/compact-runtime/interfaces/ProofData.md)

## Returns[​](#returns "Direct link to Returns")

`void`

## Throws[​](#throws "Direct link to Throws")

If the circuit is not satisfied
