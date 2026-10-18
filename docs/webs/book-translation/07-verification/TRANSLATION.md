# Layer 7 — Verification: who checks the claim, and what happens next?

A proof reaches people through the software that checks it. In a Moriarty Web, that check has two connected parts. When a proof claim is issued, the claim policy checks the proof and records a claim output. When an application transition settles, its validator collects every Strand required by the Web, rebuilds each statement from the transaction and the state being spent, and checks that the claim matches. It then enforces the owners’ signed limits and the transition’s financial rules.

The proof check answers a narrow question: does this proof establish the relation it names? It does not, by itself, establish who owns an input, which outputs the ledger will accept, whether the right people authorized the effects, or whether the transition settled. Those facts come from the transaction, its signatures, the validator’s checks, and the ledger’s acceptance rules. The application completes its own readback by confirming inclusion, waiting for the finality it requires, and reconciling actual outputs and remaining duties with the signed intents.

Issuing a proof claim and moving an application asset are separate events. Issuance records checked evidence in a proof-claim output; it does not mint the application’s asset or transfer application funds. Settlement consumes and burns the required claims while applying the financial effects allowed by the validator.

The seven layers help explain this design, but they are not seven isolated components or a one-way implementation pipeline. Verification economics and the target ledger can shape choices in the proof system and cryptographic primitives; those choices, in turn, shape what the verifier must check. The audience is a useful metaphor for the final check, while the actual Cardano path needs to be described in Cardano terms.

## Interactive proposal

Show one transition moving through four checks: proof claim issued; complete Web collected; validator accepts the transaction’s required claims and signed effects; application confirms inclusion, finality and readback. Let the reader inspect each check and see what it establishes and what it cannot establish. A missing required Strand should stop the modeled transition before settlement. The view should label this as an explanation of the design, not a live verifier, wallet, or ledger.

## Technical notes

The supplied CIP-0038 document describes Plutus support for BLS12-381 operations, pairing computation, implementation compatibility and preliminary costing. Its estimate that Groth16 verification used about 23% of the execution budget is a source-era estimate, not a current cost for this Web, a compiled claim policy, or a complete Cardano transaction. Cardano’s validator and transaction rules differ from Ethereum contracts, EVM gas, rollup data availability and Ethereum governance. Those examples in the book need separate Cardano evidence before they can be presented as facts about this design.

The supplied V2 copy describes the Web consumer, claim policy, proof issuance and settlement as a design. Its technical section says the supplied records cover one local invoice-partial Groth16 roundtrip and seven local compiled-wrapper cases; it records no on-chain transaction and says the runs are not pinned to the later V2 source. The required Strand inventory, deployed verifier identity, actual setup inventory, ledger costs, transaction acceptance, and end-to-end readback are not established by those records. Keep these status details here rather than repeating caveats in the main explanation.

The book’s Layer 7 account focuses on verifier economics, implementation faults, governance, aggregation and data availability. That is useful context, but it is not a Cardano implementation specification. Its claim that the layers are a directed acyclic graph also needs care: the chapter presents design pressures across layers, while the exact relation between co-design and dependency cycles is an open wording question. Do not call the layers independent.
