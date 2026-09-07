# createProofProvider

> For the complete documentation index, see [llms.txt](/llms.txt)

[**Midnight.js API Reference v4.0.4**](/api-reference/midnight-js.md)

***

[Midnight.js API Reference](/api-reference/midnight-js/packages.md) / [@midnight-ntwrk/midnight-js-types](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types.md) / createProofProvider

# Function: createProofProvider()

> **createProofProvider**(`provingProvider`, `costModel?`): [`ProofProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md)

Creates a [ProofProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md) from a [ProvingProvider](#). The returned provider proves transactions using the initial cost model.

## Parameters[​](#parameters "Direct link to Parameters")

### provingProvider[​](#provingprovider "Direct link to provingProvider")

[`ProvingProvider`](#)

The underlying proving provider used to generate proofs.

### costModel?[​](#costmodel "Direct link to costModel?")

`CostModel` = `...`

Optional cost model to use for proof generation. Defaults to the initial cost model if not provided.

## Returns[​](#returns "Direct link to Returns")

[`ProofProvider`](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md)

A [ProofProvider](/api-reference/midnight-js/@midnight-ntwrk/midnight-js-types/interfaces/ProofProvider.md) that delegates proof generation to the given proving provider.
