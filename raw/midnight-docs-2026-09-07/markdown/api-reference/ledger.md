> For the complete documentation index, see [llms.txt](/llms.txt)

# Ledger API

**@midnight/ledger v8.0.3**

***

# Ledger TypeScript API

This document outlines the flow of transaction assembly and usage with the ledger TS API.

## Proof stages[​](#proof-stages "Direct link to Proof stages")

Most transaction components will be in one of three stages: `X`, `UnprovenX`, or `ProofErasedX`. The `UnprovenX` stage is *always* the first one. It is possible to transition to the `X` stage by proving an `UnprovenTransaction` through the proof server. For testing, and where proofs aren't necessary, the `ProofErasedX` stage is used, which can be reached via `eraseProof[s]` from the other two stages.

## Transaction structure[​](#transaction-structure "Direct link to Transaction structure")

A [Transaction](/api-reference/ledger/classes/Transaction.md) runs in two phases: a *guaranteed* segment, handling fee payments and fast-to-verify operations, and a series of *fallible segments*. Each segment may fail atomically, separately from the guaranteed segment. It therefore contains:

* A "guaranteed" [ZswapOffer](/api-reference/ledger/classes/ZswapOffer.md)
* A map of segment IDs to "fallible" [ZswapOffer](/api-reference/ledger/classes/ZswapOffer.md)s.
* A map of segment IDs to [Intent](/api-reference/ledger/classes/Intent.md)s, which include [UnshieldedOffer](/api-reference/ledger/classes/UnshieldedOffer.md)s and [ContractAction](/api-reference/ledger/type-aliases/ContractAction.md)s.

It also contains additional cryptographic glue that will be omitted in this document.

### Zswap[​](#zswap "Direct link to Zswap")

A [ZswapOffer](/api-reference/ledger/classes/ZswapOffer.md) consists of:

* A set of [ZswapInput](/api-reference/ledger/classes/ZswapInput.md)s, burning coins.
* A set of [ZswapOutput](/api-reference/ledger/classes/ZswapOutput.md)s, creating coins.
* A set of [ZswapTransient](/api-reference/ledger/classes/ZswapTransient.md)s, indicating a coin that is created and burnt in the same transaction.
* A mapping from [RawTokenType](/api-reference/ledger/type-aliases/RawTokenType.md)s to offer balance, positive when there are more inputs than outputs and vice versa.

[ZswapInput](/api-reference/ledger/classes/ZswapInput.md)s can be created either from a [QualifiedShieldedCoinInfo](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md) and a contract address, if the coin is contract-owned, or from a [QualifiedShieldedCoinInfo](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md) and a [ZswapLocalState](/api-reference/ledger/classes/ZswapLocalState.md), if it is user-owned. Similarly, [ZswapOutput](/api-reference/ledger/classes/ZswapOutput.md)s can be created from a [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) and a contract address for contract-owned coins, or from a [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) and a user's public key(s), if it is user-owned. A [ZswapTransient](/api-reference/ledger/classes/ZswapTransient.md) is created similarly to a [ZswapInput](/api-reference/ledger/classes/ZswapInput.md), but directly converts an existing [ZswapOutput](/api-reference/ledger/classes/ZswapOutput.md).

A [QualifiedShieldedCoinInfo](/api-reference/ledger/type-aliases/QualifiedShieldedCoinInfo.md) is a [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) with an index into the Merkle tree of coin commitments that can be used to find the relevant coin to spend, while a [ShieldedCoinInfo](/api-reference/ledger/type-aliases/ShieldedCoinInfo.md) consists of a coin's [RawTokenType](/api-reference/ledger/type-aliases/RawTokenType.md), value, and a nonce.

### Calls[​](#calls "Direct link to Calls")

A [ContractDeploy](/api-reference/ledger/classes/ContractDeploy.md) consists of an initial contract state, and a nonce.

A [ContractCall](/api-reference/ledger/classes/ContractCall.md) consists of a contract's address, the entry point used on this contract, a guaranteed and a fallible public oracle transcript, a communication commitment, and a proof. [ContractCall](/api-reference/ledger/classes/ContractCall.md)s are constructed via [ContractCallPrototype](/api-reference/ledger/classes/ContractCallPrototype.md)s, which consist of the following raw pieces of data:

* The contract address
* The contract's entry point
* The contract operation expected (that is, the verifier key and transcript shape expected to be at this contract address and entry point)
* The guaranteed transcript (as produced by the generated JS code)
* The fallible transcript (as produced by the generated JS code)
* The outputs of the private oracle calls (As a FAB [AlignedValue](/api-reference/ledger/type-aliases/AlignedValue.md)s)
* The input(s) to the call, concatenated together (As a FAB [AlignedValue](/api-reference/ledger/type-aliases/AlignedValue.md))
* The output(s) to the call, concatenated together (As a FAB [AlignedValue](/api-reference/ledger/type-aliases/AlignedValue.md))
* The communications commitment randomness (As a hex-encoded field element string)
* A unique identifier for the ZK circuit used (used by the proof server to index for the prover key)

NOTE: currently the JS code only generates a single transcript. We probably just want a canonical way to split this into guaranteed/fallible?

A [Intent](/api-reference/ledger/classes/Intent.md) object is assembed, and [ContractCallPrototype](/api-reference/ledger/classes/ContractCallPrototype.md)s / [ContractDeploy](/api-reference/ledger/classes/ContractDeploy.md)s are added to this directly. This can then be inserted into an [Transaction](/api-reference/ledger/classes/Transaction.md).

## State Structure[​](#state-structure "Direct link to State Structure")

The [LedgerState](/api-reference/ledger/classes/LedgerState.md) is the primary entry point for Midnight's ledger state, and it consists of a [ZswapChainState](/api-reference/ledger/classes/ZswapChainState.md), as well as a mapping from [ContractAddress](/api-reference/ledger/type-aliases/ContractAddress.md)es to [ContractState](/api-reference/ledger/classes/ContractState.md)s. States are immutable, and applying transactions always produce new outputs states.
