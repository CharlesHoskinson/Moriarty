> For the complete documentation index, see [llms.txt](/llms.txt)

# Onchain Runtime API

**@midnight-ntwrk/onchain-runtime v3.0.0**

***

# Midnight Onchain Runtime TypeScript API

This API provides a TypeScript interface to Midnight's onchain runtime, including the execution of VM instructions, and the primitives required to successfully use them.

Key parts of this API are:

* [ContractState](/api-reference/onchain-runtime/classes/ContractState.md), encapsulating the entirety of a smart contract's on-chain state
* [StateValue](/api-reference/onchain-runtime/classes/StateValue.md), encoding data a contract maintains on-chain
* [QueryContext](/api-reference/onchain-runtime/classes/QueryContext.md), providing an annotated view into the contract state, against which on-chain VM programs can be run
* [Op](/api-reference/onchain-runtime/type-aliases/Op.md), providing the TypeScript encoding of on-chain VM programs
* [AlignedValue](/api-reference/onchain-runtime/type-aliases/AlignedValue.md), the "base" value type that encodes all user data stored on-chain
