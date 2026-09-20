![Moriarty](docs/assets/moriarty-banner.png)

# Moriarty

Moriarty is an intent language for financial agreements on Midnight. Its starting point is the owner's intention: the outcomes that are acceptable, the authority that may be exercised, the costs that may be incurred, and the duties that must remain after a partial result. A solver may choose how to satisfy these conditions. The purpose of the language is to make their satisfaction a condition of accepting the resulting financial effects.

Two references separate the design obligations from the current language:

- [Requirements: MPLRs, proposed ZKIRv4 and recursion](https://charleshoskinson.github.io/Moriarty/docs/requirements.html) — one paragraph for each language requirement, backend requirement and recursion refinement.
- [Current formal syntax and semantics](https://charleshoskinson.github.io/Moriarty/docs/language.html) — versioned source profiles, grammar, static judgments and operational rules.

## The problem

A financial operation often outlives the transaction that begins it. An order may fill in parts; a payment may wait for signatures, a document predicate or a proof; one chain may settle while another outcome remains unknown. In each case, an accepted step changes what is owned, what has been spent and what is still owed. Moriarty is designed to represent that continuing agreement, so that partial progress, conditional settlement, fees and recovery can be checked against the same authenticated intention.

This also gives solvers, including AI agents, a defined role. They search among the choices the owner has allowed. They do not acquire authority merely by finding a route, using a wallet interface or presenting a valid proof of an unrelated claim. The intended acceptance relation binds the chosen execution to its complete effects, gross spending, fees, typed liabilities, net outcomes, disclosures and residual duties. It proves the formalized intention; it cannot recover a wish that was never expressed.

## Design choices

Moriarty bounds computation within each stage. Exact arithmetic, explicit resource limits and a small core make the meaning and cost of a supported program analyzable. Financial libraries build on that core. Certified primitives may replace reference expressions only when they preserve their semantics, preconditions and failure behavior. This design gives termination and correspondence proofs a tractable subject, while leaving eventual completion of a distributed workflow as a separate question.

Programs target Midnight's native ZKIRv3 execution and proof infrastructure. Proof-carrying data is intended to connect each accepted transition to its authenticated origin and compliant predecessors. Comprehensive native recursion must extend this to private continuation and bounded multi-parent composition without losing earlier spending, consent or outstanding duties. ZKIRv4 names our proposed next-version requirements workstream; it is not a claim of an announced upstream release. These proof obligations target Midnight’s native stack without an additional Lean theorem-prover dependency.

The language is permissionless: supported programs must be open to authoring, compilation, proving and deployment without project approval or federation membership. An agreement may still require its owner's authorization, a particular counterparty's consent or evidence from a named issuer. Those are conditions of that agreement, enforced alongside objective proof and ledger rules.

## The Federated DeFi Kernel

The optional Federated DeFi Kernel supplies the coordination that a multichain agreement may need. It connects solvers, obtains evidence, manages constrained signing, observes settlement and carries unfinished work into recovery. Moriarty specifies which effects are permitted; the kernel arranges candidate executions and supplies evidence; Midnight checks the native proof relation and enforces its ledger's state and consumption rules. A supported direct Midnight program does not need to use the federation.

Zero-knowledge proofs, multi-party computation and trusted execution environments contribute different assurances to this arrangement. Their statements, signatures and attestations must refer to the same intention and effects, and each external adapter must expose its trust and finality assumptions. A local proof cannot make a foreign observer truthful or turn an acknowledgment into settlement. Open Wallet Standard (OWS) integration and x402 HTTP service payments fit within this boundary: delegated authority, cumulative budgets, payment and result delivery remain separately accountable.

## Where the project stands

Versioned parsers, local evaluators, scoped executable K definitions and scoped Midnight Preview financial results provide the present foundation. The full language-to-ledger correspondence, mandatory native recursive history, private composition and complete financial-library coverage remain work to establish. The approximately March 2027 horizon for comprehensive Midnight recursion is a project planning assumption recorded on September 19, 2026, rather than a verified release commitment. The two references above keep those proposed obligations distinct from the grammar and formalization available today.
