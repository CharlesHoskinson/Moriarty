# Market and prior-art assessment for Moriarty

Research snapshot: 2026-09-19. The answer is yes: several systems pursue substantial parts of Moriarty's purpose. Anoma is the closest broad architectural comparison found in this bounded review; Valence is a close comparison for authorized, proof-based cross-domain execution. There is no basis here to claim that Moriarty invents provable intents or private programmable financial execution. No inspected source established a ready-made product with the entire proposed Moriarty language/kernel contract. That is a coverage-limited finding, not a proof of uniqueness.

## Closest comparisons

| System | Relevant overlap | What the reviewed evidence does not establish |
|---|---|---|
| Anoma / ARM / Juvix | Resource logic, compliance and balance proofs; solver completion of intents; programmable privacy; adapters to existing chains | The full Moriarty financial lifecycle and certified Midnight ZKIRv3 compiler/proof path |
| Valence | Programmable authorizations, processors and coprocessor proof verification; cross-domain architecture | All advertised cross-chain state coordination is deployed, or every conceptual example proves the whole authorization relation |
| Daml / Canton | Explicit parties, obligations, choice authority, private multiparty financial workflows and atomic transaction semantics | Mandatory public proof-carrying execution on Midnight or a total Moriarty-style core |
| Miden | Asset-bearing notes with consumption predicates; separate creation/consumption; private local proofs; Guardian coordination | A verified live mainnet in this source set or a complete independently signed financial-intention refinement system |
| Aztec / Noir | Private programmable execution, precise authwits, domain binding and nullifier replay protection | Mature end-to-end assurance for the entire Moriarty agenda; official Alpha docs explicitly retain limitations |
| Aleo / Leo | Mainnet language and execution proof stack, private records, constrained transitions and explicit finalize/fee outcomes | The proposed general financial-obligation language and its independently specified intention theorem |
| NEAR Intents / Chain Signatures | Intent fulfillment, solver competition and external signing infrastructure | Signatures or verifier success alone prove all downstream effects and residual obligations |
| Marlowe, Move, Scilla, Simplicity, Pact, Clarity | Valuable financial DSL, resource safety, formal specification, boundedness or postcondition precedents, depending on system | One uniform feature set or an equally current operational product; consult per-system status evidence |
| Agent, routing and TEE products | Agent workflows, policy enforcement, route construction, paid tools and/or attested computation | A universal theorem that accepted financial behavior realizes the user's complete formal intention |

These rows are comparisons of evidenced scope, not assertions that other platforms are incapable of implementing additional properties. Exact primary sources, availability qualifications and contradictions are in the four track reports and source ledgers.

## What actually distinguishes the proposed agenda

The strongest candidate is a coherent financial language contract: the user signs a formal intention; every accepted stage preserves its authority, assets, liabilities, fees, evidence conditions and residual duties; the proof is linked to the actual Midnight artifact and ledger effects. Solvers can choose among admissible realizations without gaining authority to change that contract. A composed workflow must preserve the contract through partial progress, asynchronous evidence, cancellation and authorized evolution.

This combines capabilities with substantial prior art. Its merit depends on making them compositional, useful and demonstrably enforced. The proposition is weaker if it becomes only a nicer syntax over ordinary signatures, a prover of whatever circuit happened to be generated, or a coordinator whose offchain checks a signer can bypass.

Midnight/ZKIRv3 is a meaningful deployment requirement and ecosystem advantage, but it does not by itself create PL novelty. Likewise, Turing incompleteness, ZK, MPC, TEE, intents, agents and escrow are individually established ideas. Novelty, if any, lies in the precise semantic/proof integration and demonstrable developer capability, not the vocabulary.

## Fair comparison tests

Use the same six workflows on Moriarty and the closest alternatives:

1. A conditional DvP waits for recipient acceptance, a document predicate and an external proof, with scoped disclosure.
2. Two solvers concurrently spend one delegated budget while fees and pending reservations remain bounded.
3. A partial fill completes one leg, another outcome remains unknown, and a late result arrives after cancellation begins.
4. Netting preserves gross consent, fees, issuer liabilities and residual obligations rather than only final balances.
5. A continuation migrates after a verifier or operator change without silently changing beneficiary or recovery rights.
6. A hostile prover supplies a valid proof for the wrong artifact, missing predicate, forged genesis or mismatched external effect.

For each, identify code already available, added application work, proof statement, trusted parties, failure/recovery behavior, privacy leakage, and deployment evidence. Count both useful positive executions and rejected hostile traces. Do not let blanket refusal count as correctness.

## Research priorities

First, perform the user-requested Anoma code/specification teardown. Separate the common ARM interface, each concrete proof implementation, Juvix compilation and actual deployed adapters. In particular, distinguish an unbalanced intent completed before settlement from an already-committed partial workflow. Second, examine Valence proof-message and domain-state binding. Third, compare Miden note semantics and Daml obligation formation directly against Moriarty's staged lifecycle.

The current market dossier does not choose a dependency or authorize copying code. Reuse decisions require pinned licenses, compatible semantics and target proof obligations. The Anoma teardown records those facts before proposing transfers into the design. Moriarty's existing implementation remains substantially short of its intended agenda; the comparison must not present future requirements as delivered advantages.
