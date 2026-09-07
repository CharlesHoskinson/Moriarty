> For the complete documentation index, see [llms.txt](/llms.txt)

# Compact runtime API

**@midnight-ntwrk/compact-runtime v0.19.0**

***

# Compact runtime library

This API provides runtime primitives used by Compact's TypeScript output, both for use by the compiler output directly, and to utilise it or reproduce its behaviour. This API re-exports a number of items from `@midnight-ntwrk/onchain-runtime-v2`, and wraps others in a more TypeScript-friendly API. Key parts of the API are:

* setNetworkId, required to ensure the right network is being targeted

* [CircuitContext](/api-reference/compact-runtime/interfaces/CircuitContext.md), and [CircuitResults](/api-reference/compact-runtime/interfaces/CircuitResults.md) part of the input and output definition of all circuits

* [WitnessContext](/api-reference/compact-runtime/interfaces/WitnessContext.md), part of the input definition of all circuits

* Built-in functions:

  <!-- -->

  * Hashing/commitment

    <!-- -->

    * [transientHash](/api-reference/compact-runtime/functions/transientHash.md)
    * [transientCommit](/api-reference/compact-runtime/functions/transientCommit.md)
    * [persistentHash](/api-reference/compact-runtime/functions/persistentHash.md)
    * [persistentCommit](/api-reference/compact-runtime/functions/persistentCommit.md)
    * [degradeToTransient](/api-reference/compact-runtime/functions/degradeToTransient.md)

  * Elliptic curve

    <!-- -->

    * [ecAdd](/api-reference/compact-runtime/functions/ecAdd.md)
    * [ecNeg](/api-reference/compact-runtime/functions/ecNeg.md)
    * [ecMul](/api-reference/compact-runtime/functions/ecMul.md)
    * [ecMulGenerator](/api-reference/compact-runtime/functions/ecMulGenerator.md)
    * [hashToCurve](/api-reference/compact-runtime/functions/hashToCurve.md)

* [ContractState](/api-reference/compact-runtime/classes/ContractState.md), encapsulating the entirety of a smart contract's on-chain state

* [StateValue](/api-reference/compact-runtime/classes/StateValue.md), encoding data a contract maintains on-chain

* [QueryContext](/api-reference/compact-runtime/classes/QueryContext.md), providing an annotated view into the contract state, against which on-chain VM programs can be run

* [CompactType](/api-reference/compact-runtime/interfaces/CompactType.md), providing a runtime representation of basic Compact datatypes

* Various TypeScript types matching same-named Compact types
