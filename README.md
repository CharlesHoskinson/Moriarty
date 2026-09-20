![Moriarty](docs/assets/moriarty-banner.png)

# Moriarty

Moriarty is a programming language for financial intention on Midnight. An owner specifies the outcomes they will accept, the authority they delegate, the costs they permit and the obligations that must survive incomplete execution. Solvers can then search for ways to realize that intention. The language's purpose is to make compliance with those conditions a prerequisite for accepting the resulting financial effects.

The intended scope extends from a direct private agreement on Midnight to a multichain financial workflow coordinated by an optional Federated DeFi Kernel. Partial transactions, conditional settlement with programmable escrow, continuing liabilities and recovery are part of the language design. Human developers and AI solvers work within the same authority and proof requirements.

Moriarty is under active development. This README describes the target design and its motivation; the final section distinguishes current implementation evidence from the work still required. Two references provide the detail:

- [Requirements: MPLRs, proposed ZKIRv4 and recursion](https://charleshoskinson.github.io/Moriarty/docs/requirements.html) — the language requirements, native backend obligations, recursion refinements and consolidated delivery direction.
- [Current formal syntax and semantics](https://charleshoskinson.github.io/Moriarty/docs/language.html) — versioned source profiles, grammar, static judgments and operational rules.

## From a transaction to an agreement

Consider a buyer who authorizes payment for goods once a seller accepts the terms, a designated issuer attests to a shipping document and a delivery condition is satisfied. The funds may already be in escrow. The seller may fulfill the order in parts. Payment may involve another chain whose outcome is temporarily unknown. A useful account of this agreement must say which conditions have been established, how much may be released, who still owes what and which remedies remain available.

A transaction records one step in this process. The agreement supplies the meaning of that step and the conditions under which another may follow. Moriarty is designed to preserve this meaning across the whole sequence. A partial delivery must leave an explicit remainder; a failed route must account for fees already incurred; a refund must not silently restore authority that the owner intended to spend only once.

This is the sense in which Moriarty aims to be a language of **provable intention**. The intention is an authenticated formal contract over acceptable behavior. A proof must establish that the proposed effects satisfy it, including the duties left behind. The guarantee is limited to what the contract expresses and the evidence assumptions it names. No proof can recover an unexpressed preference or establish the truth of an external claim merely because someone supplied a document hash.

For developers, the attraction is a common foundation for financial applications whose safety conditions would otherwise be reconstructed separately. For owners, it is a precise boundary around delegated discretion. For solvers, it is a defined search space: they may select routes, counterparties or other allowed choices while preserving every participating owner's constraints.

## The language design

### Bounded computation, continuing agreements

Moriarty is deliberately Turing incomplete within each stage. Computation has explicit bounds, arithmetic has defined behavior and supported operations have inspectable semantics. This makes termination, resource accounting and correspondence between a program and its proof obligations tractable subjects for verification. It also gives developers a place to state failures such as overflow, insufficient authority or an unsatisfied condition.

A bounded stage can produce an authenticated continuation. A loan can therefore accrue, receive payments and eventually close through successive bounded transitions. A trade can wait for evidence and resume when that evidence arrives. The target does not impose one fixed depth on all histories; each stage and each composition step must remain bounded, with cumulative authority and work limits preserved across authorized continuations.

Termination of a stage does not establish completion of the agreement. A counterparty can disappear, liquidity can vanish and a chain can stop including transactions. The language must expose the assumptions under which progress or recovery is possible and represent unresolved obligations when those assumptions fail.

### Financial meaning belongs in the program

Amounts need asset identities, units and domains. Prices need a declared orientation. Fees, rounding and overflow need exact rules. Otherwise, a perfectly valid proof may establish arithmetic over values whose financial interpretation was wrong at the outset. Moriarty's design therefore treats these choices as part of the semantic contract carried into compilation and verification.

Authority and liability have different lifecycles. Unused spending authority may be left unused; a consumed receipt must not be consumed again; a debt remains until it is discharged, transferred with the required consent, amended or explicitly forgiven. Default does not erase that debt. A transfer to an address cannot by itself impose a new obligation on its owner, though ordinary receipt of value need not require an interactive acceptance ceremony.

Gross spending, fees and net outcomes are recorded separately. A strategy that spends, receives a refund and spends again must still respect the owner's cumulative gross limit. Several concurrent solvers must share an accounting of spent and reserved authority. A net-positive balance alone cannot establish that all of these conditions were respected.

### A small core and certified financial libraries

The design places exact values, bounded evaluation, state transitions, authority, evidence and continuing obligations in a small semantic foundation. Payments, loans, swaps, liquidity positions, vault shares, margin and other financial families can then be developed as reusable libraries with explicit preconditions and effects. Broad financial coverage requires separate conformance work for each family and its compositions.

Optimized primitives follow the lesson of Simplicity's jets: an efficient implementation must retain a precise reference meaning. Its certificate must cover preconditions, outputs, failure behavior, complete effects and the declared cost relation. Certification is tied to the actual target constraints; testing an honest implementation does not exclude a malicious witness that exploits an underconstrained circuit.

Developer assistance can build on the same foundation. Refinement checks, counterexamples, typed holes and bounded synthesis should help an author discover and discharge obligations. A generated solution must pass the same checks as a manually written program. Unsupported checks, timeouts and inconsistent assumptions must remain distinguishable from success.

### Midnight as the execution target

Compiled Moriarty contracts must run on Midnight and target its native ZKIRv3 execution and proof infrastructure. The intended path is to author and inspect a program, check its types, effects and bounds, expose its proof obligations, compile it for a pinned target, generate native evidence and submit it for ledger acceptance. Each transformation must preserve the relevant meaning through to the actual effects.

Midnight's native PLONK/KZG stack is the proof foundation for this design. Moriarty does not introduce a Lean dependency. Executable K definitions provide a reference for scoped language behavior; agreement between an evaluator and a finite set of K executions remains evidence about those cases, rather than a general compiler correctness theorem.

Proof-carrying data connects an accepted transition to an authenticated origin and compliant predecessors. Comprehensive native recursion is intended to support portable history, private handoff and composition with multiple predecessors. Such composition must preserve consent, cumulative spending and every outstanding duty. Combining proofs without checking those relationships would only compress evidence for disconnected claims.

The requirements page calls the next-version backend workstream **ZKIRv4**. This is a proposed requirements label, not a claim that an upstream release with that name has been announced. The approximately March 2027 horizon for comprehensive Midnight recursion is a project planning assumption recorded on September 19, 2026, rather than a verified release commitment.

## The security model

The central adversarial case is a party proposing an execution that appears to meet the owner's request while exercising more authority, hiding an effect or discarding an obligation. The solver, witness producer, relayer or counterparty may be malicious. The target acceptance relation must constrain every accepted witness, including ones that the normal software would never generate.

Four obligations organize that relation: the program satisfies its required contract properties; its chosen behavior refines the authenticated intention; its actual state transition and effects are valid; and its history is compliant. Each obligation needs an enforcement point in native constraints, authenticated state or a justified ledger mechanism. A host-side check, an unused Boolean or a label saying “verified” cannot supply a missing obligation.

### Bind the proof to the execution

An accepted proof must concern the right program, semantic profile, intention, domain, stage, state and effects. It must also use the authorized circuit and verifier identity. These bindings prevent a valid proof for another agreement, chain or verification key from being substituted for the required claim. Source, intermediate representation and emitted target code have different identities; their connection requires an established correspondence, rather than an assumption that matching names imply matching behavior.

Complete effects matter here. A proof that checks the requested output while omitting another debit, a fee or a newly created liability leaves the owner exposed. The implementation must bind what the ledger actually accepts to the complete financial relation. Required checks must govern acceptance on every relevant path, including any phase that retains effects after later work fails.

### Preserve authority and history

Signatures and delegation establish who may authorize a transition and within which scope. Replay protection and unique consumption prevent an accepted authorization or receipt from being reused. Authenticated genesis prevents a prover from inventing a favorable starting state. A continuation must inherit cumulative spending, outstanding liabilities and the conditions governing future action.

Amendment, disclosure, reconciliation and recovery require their own authority where applicable. Learning that an external payment succeeded does not authorize another transfer. Revocation can end a delegated right without erasing debts already incurred. Recovery may remain available after ordinary execution authority expires, but only under its own signed scope and termination policy.

These conditions also apply to private histories. Hiding a position must not make it possible to omit that position from an aggregate liability or collateral claim. Private composition requires evidence of completeness and consumption, together with a workable means of transferring the necessary witness material. A commitment to unavailable data can preserve integrity while leaving the next participant unable to proceed.

### Make external trust explicit

An external observation has an issuer, a domain, a freshness rule and a finality meaning. A transaction's inclusion in a block may establish neither successful application execution nor final settlement. The adapter must relate the exact submitted transaction and its authenticated result to the financial effects Moriarty accounts for.

Conditional release can depend on signatures, document predicates, proofs, recipient actions or combinations of these conditions. The policy must specify what each item establishes. An attestation may establish that a named issuer made a claim; its truth depends on the issuer and verification policy. Legal enforceability and physical delivery require assumptions beyond the existence of a cryptographic commitment.

Uncertainty must survive into recovery. A timeout does not prove that another chain failed to execute. A refund and a late successful payment must not both discharge the same escrow claim. The design therefore needs explicit unresolved states, reconciliation rules and exclusive terminal outcomes. Compensation is a new authorized action whose costs and effects are recorded; it cannot reverse history across unrelated ledgers.

### Separate safety, privacy and availability

Zero knowledge can limit disclosure while proving a defined relation. It does not automatically hide timing, network traffic, public amounts or information revealed to counterparties. Each workflow needs a disclosure policy and a statement of what remains observable.

Safety means that an accepted step stays within the specified rules. Availability and liveness concern whether a valid step can be produced and included at all. Bounded evaluation helps control computation, but proving costs, witness availability, state growth, resource exhaustion and chain access remain engineering and protocol obligations. Recursive proof compression alone does not bound the storage required by a long-lived agreement.

The design reduces opportunities for unauthorized spending, replay, omitted liabilities, arithmetic ambiguity and invalid continuation when the corresponding obligations are correctly implemented. It does not promise profitable strategies, honest oracles, optimal execution, freedom from ordering manipulation or eventual settlement. An owner can constrain price, deadlines and exposure; proving those constraints still leaves the risks that the owner chose to accept.

## How the Federated DeFi Kernel works with Moriarty

The optional Federated DeFi Kernel coordinates work that extends beyond one local agreement. It can collect intentions, connect solvers, obtain evidence, arrange constrained signing, submit external transactions, observe their outcomes and manage authorized recovery. Moriarty defines acceptable behavior and the evidence required for it. Midnight checks the native proof relation and enforces its own ledger's state and consumption rules. Each external domain retains its own execution and finality assumptions.

This separates three kinds of work. Applications express the financial purpose and required consent. Solvers search for candidate plans within that authority. Settlement mechanisms execute effects and establish what occurred. The kernel coordinates these activities without acquiring a right to weaken the agreement. In the CAKE chain-abstraction framework's Applications, Permission, Solvers and Settlement model, permission refers to user and application authority; it is not a license to deploy a Moriarty program.

A typical coordinated workflow begins with an owner signing constraints and allowed evidence policies. A solver proposes a route. The relevant budgets are reserved, the candidate is checked against the agreement, and each permitted stage executes under its stated conditions. Authenticated results update the continuing state. Further action depends on those results, with unresolved outcomes retained for reconciliation and recovery rather than treated as success or failure by convenience.

The kernel's ZK, multi-party computation and trusted execution environments provide different assurances. ZK proves the specified relation. MPC or threshold signing distributes control under a stated corruption threshold. A TEE adds claims about an attested execution environment, subject to hardware, freshness and rollback assumptions. These mechanisms must refer to the same intention, program, domain, stage, epoch and effects; placing them together does not establish independent security when operators or infrastructure are shared.

The destination's enforcement boundary is decisive. If a foreign account accepts a threshold signature alone, compromise of that threshold may bypass the policy honest signers would have checked. A Moriarty proof cannot force that account to verify a relation its native rules do not require. Federation membership changes, signing thresholds, evidence policies and failure remedies therefore need explicit treatment in each supported integration.

The local protected financial evaluator checks a bounded stage and its financial actions as part of the language implementation. The Federated DeFi Kernel is an optional coordination system. Supported direct Midnight programs, including the target private handoff capability, must be usable without federation membership. Authoring, compiling, proving and deploying a supported Moriarty program requires no project, council, registry or provider approval, and no privileged solver. Owner consent, application conditions and objective ledger validity rules continue to apply.

## Use cases

The following examples describe intended capabilities and the obligations they motivate. They are design illustrations, not claims that each end-to-end application is implemented today.

### Conditional settlement and programmable escrow

A buyer can authorize release only when specified signatures, an authenticated document predicate and a recipient action have all been established. The agreement can allow partial release for partial fulfillment while retaining the balance and outstanding duties. This supports trade finance, staged purchases and other arrangements in which submitting a payment request and becoming entitled to payment are separate events. Expiry and dispute remedies must be authorized explicitly; the evidence policy determines what the program can actually establish about the underlying goods or services.

### Partial fills and multichain execution

An owner can permit an order to fill in several stages while constraining the assets, minimum net return, total fees and cumulative spending. A solver may combine liquidity sources or coordinate settlement across domains. Each accepted fill reduces the remaining order and records its costs. If one leg settles while another remains unknown, the continuation preserves the resulting exposure and permitted remedies. Global atomicity is available only where the actual participating mechanisms establish it.

### Loans, collateral and continuing claims

A lending agreement can track principal, accrued amounts, repayment and collateral duties as distinct quantities. Refinancing or transferring a claim must preserve the relevant liabilities and obtain required consent. A liquidation path must establish its trigger, price policy and resulting allocation, including any remaining debt. This is why token conservation alone is insufficient: moving the expected tokens does not prove that the agreement accounted for every claim.

### Treasury management by AI solvers

A treasury can delegate a bounded task with approved assets, venues, price constraints and spending limits. Several AI solvers may search concurrently, provided reservations and completed spending are checked against the same authority. Open Wallet Standard integration supplies a wallet interface; the program supplies the limits within which that interface may be used. A solver can exercise the discretion the treasury granted without obtaining general custody authority from its ability to propose a valid strategy.

### Paid services and automated procurement

A strategy may purchase data or computation through x402 HTTP service payments. The agreement must distinguish authorization to pay, final payment, receipt of a result and satisfaction of the result's predicate. A retry retains the logical request identity so that uncertainty about delivery does not silently authorize duplicate charges. The evidence available for a service determines which delivery claims can be proven and which require a trusted issuer or an explicit remedy.

### Private institutional agreements

Participants may need to establish collateral adequacy, authorized participation or compliant transfers while limiting disclosure of positions and counterparties. The target private composition model allows a continuing agreement to move between independently controlled participants without discarding its history or duties. Selective disclosure can support an agreed audit policy. Completeness remains essential: a private proof of adequate collateral is useful only if the relation also accounts for the liabilities it is supposed to cover.

## Where the project stands

The repository contains versioned parsers and local evaluators, scoped executable K definitions, financial lifecycle behavior and scoped Midnight Preview loan and swap results. These establish concrete foundations for the language. They do not yet establish the full path from arbitrary supported source programs to mandatory native proofs of their complete ledger effects.

The principal remaining obligations are language-to-ledger correspondence, enforcement of the complete authenticated intention, native recursive history, private handoff and composition, conditional settlement with sound recovery, and broader financial-library conformance. Federation and solver integrations must preserve those obligations at their external boundaries. Each step needs evidence appropriate to the claim: finite semantic comparisons, adversarial witness tests, compiler correspondence arguments, resource measurements and actual ledger results establish different things.

The immediate direction is to connect a newly authored bounded financial program through the public compilation and proving path to observed Midnight effects, with controls that reject altered intention, effects and history. That same relation must then extend to partial fulfillment, conditional escrow and late-result recovery. Recursion and private composition expand how compliant history can be carried between stages and participants. The requirements reference above records these obligations; the syntax and semantics reference records the language profiles available for inspection today.
