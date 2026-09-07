# End-to-end architecture

> For the complete documentation index, see [llms.txt](/llms.txt)

Your device computes a Midnight transaction privately and proves it correct; only then does the network verify it publicly. Private data participates in the computation but never reaches the chain. What crosses the boundary is the public record of execution, plus zero-knowledge proofs that it is correct.

This page connects the whole journey in one diagram. Each stage links to a page that covers it in depth.

## The complete flow[​](#the-complete-flow "Direct link to The complete flow")

<!-- -->

\* The updated private state persists on the device only after the transaction succeeds.

## What each layer can and cannot see[​](#what-each-layer-can-and-cannot-see "Direct link to What each layer can and cannot see")

| Layer                          | Can see                                                                                                                                                                                         | Cannot see                                                                                                                                                                                                                 |
| ------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Chain observer (anyone)        | Contract address, which circuit the transaction invoked, public transcripts, ledger operation arguments, disclosed values, commitments, nullifiers, block timing                                | Witness return values (unless disclosed into a public position), private state, internal computation, the value inserted into a Compact `MerkleTree` (guessable if low entropy), which commitment a given nullifier spends |
| Node (validator)               | Same as a chain observer, plus mempool timing                                                                                                                                                   | Same as a chain observer                                                                                                                                                                                                   |
| Indexer                        | Everything on chain (it indexes public data)                                                                                                                                                    | Private state, witness values; it cannot open commitments                                                                                                                                                                  |
| Indexer, with your viewing key | Your own shielded transaction history (decrypt-only access; the indexer operator learns it too)                                                                                                 | Other users' shielded data; a viewing key cannot spend or sign                                                                                                                                                             |
| Proof server                   | Everything in the proof request, including witness values. It is a trust boundary: run your own; with wallet-delegated proving, the proving step moves to whatever proof server the wallet uses | Nothing; it sees the full request                                                                                                                                                                                          |

## Go deeper into each stage[​](#go-deeper-into-each-stage "Direct link to Go deeper into each stage")

* Transaction anatomy, offers, and binding: [Building blocks](/concepts/how-midnight-works/building-blocks.md)
* How a contract splits into local, circuit, and ledger parts, and what a transcript is: [Smart contracts](/concepts/how-midnight-works/smart-contracts.md)
* Well-formedness, the guaranteed and fallible phases, and ledger state updates: [Transaction semantics](/concepts/how-midnight-works/semantics.md)
* Commitments, nullifiers, and the techniques that keep data private: [Keeping data private](/concepts/how-midnight-works/keeping-data-private.md)
* How transactions move through the node: [Transactions on the network](/concepts/network-architecture/transactions.md)
* Exactly what is visible on chain, and the proof server and indexer trust boundaries: [Security and best practices](/guides/security-best-practices.md)
* The same flow from the SDK's point of view, provider by provider: [Midnight.js](/sdks/official/midnight-js.md)
