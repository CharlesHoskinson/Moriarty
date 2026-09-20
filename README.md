![Moriarty](docs/assets/moriarty-banner.png)

# Moriarty

Moriarty is a programming language for financial intention on the Midnight blockchain. An owner specifies the outcomes they will accept, the authority they delegate, the costs they permit and the obligations that must survive incomplete execution. A solver is a program or service that proposes how to carry out that intention, choosing among the options the owner allows. The language's purpose is to make compliance with those conditions a prerequisite for accepting the resulting financial effects.

The intended scope extends from a direct private agreement on Midnight to a multichain financial workflow coordinated by the Federated DeFi Kernel, a separate, optional system for coordinating solvers and settlement. Partial transactions, conditional settlement with programmable escrow, continuing liabilities and recovery are part of the language design. Human developers and AI solvers work within the same authority and proof requirements.

Moriarty is experimental. Developers can author and simulate supported source programs today; the full proof and settlement path described here remains under development. The repository is not a production SDK or an audited deployment. Two references separate the design requirements from the current language:

- [Requirements: Moriarty Programming Language Requirements (MPLRs), proposed ZKIRv4 and recursion](https://charleshoskinson.github.io/Moriarty/docs/requirements.html): the language requirements, native backend obligations, recursion refinements and consolidated delivery direction.
- [Current formal syntax and semantics](https://charleshoskinson.github.io/Moriarty/docs/language.html): versioned source profiles, grammar, static judgments and operational rules.

## From a transaction to an agreement

Consider a buyer who authorizes payment for goods once a seller accepts the terms, a designated issuer attests to a shipping document and a delivery condition is satisfied. The funds may already be in escrow. The seller may fulfill the order in parts. Payment may involve another chain whose outcome is temporarily unknown. A useful account of this agreement must say which conditions have been established, how much may be released, who still owes what and which remedies remain available.

A transaction records one step in this process. The agreement supplies the meaning of that step and the conditions under which another may follow. Moriarty is designed to preserve this meaning across the whole sequence. A partial delivery must leave an explicit remainder; a failed route must account for fees already incurred; a refund must not silently restore authority that the owner intended to spend only once.

This is the sense in which Moriarty aims to be a language of **provable intention**. The intention is an authenticated formal contract over acceptable behavior. A proof must establish that the proposed effects satisfy it, including the duties left behind. The guarantee is limited to what the contract expresses and the evidence assumptions it names. No proof can recover an unexpressed preference or establish the truth of an external claim merely because someone supplied a document hash.

For developers, the attraction is a common foundation for financial applications whose safety conditions would otherwise be reconstructed separately. For owners, it is a precise boundary around delegated discretion. For solvers, it is a defined search space: they may select routes, counterparties or other allowed choices while preserving every participating owner's constraints.

## The language design

A `.mori` source file defines an agreement. For a loan, the source describes how origination, interest and repayment work. An instance binds that agreement to a particular lender, borrower and current debt. Calling its repayment action proposes a change to that debt and the associated payment.

<details>
<summary>A local repayment example and source profiles</summary>

A source profile specifies the syntax and operations the frontend supports. Start with `moriarty-financial-agreement-source/5` to explore the current local loan lifecycle. The older `moriarty-bounded-atomic/1` profile supports the original loan/swap examples and restricted compilation. These are separate language versions with different supported operations. The local developer quickstart below exercises both; the syntax and semantics reference supplies their complete grammar.

For example, the repayment action in `experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori` records the debt discharge alongside the transfer. This excerpt names the obligation and links the repayment to its transfer; the complete source also declares the record types, emits the transfer and checks postconditions:

```text
emit Repay {
  allocationId: allocationId,
  transferId: transferId,
  obligationId: "Loan1",
  payer: "Borrower",
  nominalAmount: nominal
};
```

Here `emit` declares a financial operation for the evaluator to check. It does not send a ledger transaction. `nominalAmount` is the amount of debt to discharge; the associated transfer supplies its settlement payment. The complete source file shows the enclosing `agreement` and `action` declarations; its accompanying demo constructs the instance state and participant inputs.

</details>


Authorization can fix an **exact plan**, including its action, state updates and financial effects, or permit an **outcome intent**, within which a solver chooses a plan. An owner who wants a particular transfer need not delegate route selection. An owner who wants a minimum return can leave choices open while bounding gross spending, fees, recipients and allowed actions. The local atomic evaluator already checks both forms as authority supplied alongside an action, using simulated authentication. Binding that authority cryptographically to ledger acceptance and durable replay protection remains implementation work.

### Bounded computation, continuing agreements

A stage is a bounded step in an agreement's execution, checked before its proposed effects are accepted. In the current local demos, evaluating an action supplies such a step. The language deliberately excludes unrestricted loops and source recursion within a stage, a restriction called Turing incompleteness. Arithmetic has defined behavior and supported operations have inspectable semantics. This makes termination, resource accounting and correspondence between a program and its proof obligations tractable subjects for verification. It also gives developers a place to state failures such as overflow, insufficient authority or an unsatisfied condition.

A completed stage can leave a continuation: authenticated state that records what remains to be done and the conditions for the next step. A loan can therefore accrue, receive payments and eventually close through successive bounded transitions. A trade can wait for evidence and resume when that evidence arrives. The target does not impose one fixed depth on all histories; each stage and each composition step must remain bounded. Authorized continuations must also preserve cumulative work limits.

Termination of a stage does not establish completion of the agreement. A counterparty can disappear, liquidity can vanish and a chain can stop including transactions. The language must expose the assumptions under which progress or recovery is possible and represent unresolved obligations when those assumptions fail.

### Financial meaning belongs in the program

Amounts need asset identities, units and domains. Prices need a declared orientation. Fees, rounding and overflow need exact rules. Settlement bindings must specify how nominal obligations convert into ledger asset quantities. Otherwise, a perfectly valid proof may establish arithmetic over values whose financial interpretation was wrong at the outset. Moriarty's design therefore treats these choices as part of the semantic contract carried into compilation and verification.

Authority and liability have different lifecycles. Unused spending authority need not be exercised. A consumed receipt must not be consumed again, and a debt remains until it is discharged, transferred with the required consent, amended or explicitly forgiven. Default does not erase that debt. A transfer to an address cannot by itself impose a new obligation on its owner, though ordinary receipt of value need not require an interactive acceptance ceremony.

Gross spending, fees and net outcomes are recorded separately. A strategy that spends, receives a refund and spends again must still respect the owner's cumulative gross limit. Several concurrent solvers must share an accounting of spent and reserved authority. A net-positive balance alone cannot establish that all of these conditions were respected.

### A small core and certified financial libraries

The design places exact values, bounded evaluation, state transitions, authority, evidence and continuing obligations in a small semantic foundation. Payments, loans, swaps, liquidity positions, vault shares, margin and other financial families can then be developed as reusable libraries with explicit preconditions and effects. Broad financial coverage requires separate conformance work for each family and its compositions.

ACTUS, the Algorithmic Contract Types Unified Standards, supplies reference behavior for scheduled financial events and cash flows, including interest, principal repayment and maturity. The project's DeFi research, indexed in `wiki/index.md`, catalogues swaps, liquidity and lending as implementation targets. Together they give the language concrete conformance targets across DeFi and traditional finance; dates, rounding and residual claims must agree with the selected financial contract. Marlowe supplies an earlier example of a financial DSL organized around analyzable agreements. These references guide the work; full conformance remains unfinished.

A frequently used operation can have a faster implementation, provided a certificate establishes that it retains the reference operation's meaning. Simplicity, Blockstream's language for verifiable programs, calls such optimized implementations jets. For Moriarty, the certificate must cover preconditions, outputs, failure behavior, complete effects and the declared cost relation. Testing the normal implementation is insufficient: the proof constraints must also rule out fabricated inputs that would make an incorrect execution appear valid. This lets a financial library optimize common calculations while preserving the meaning an application relies on.

Planned developer tools would check a candidate program against its constraints and show a counterexample when a check finds a violation. An author could also leave a gap with a specified type and ask a bounded search to fill it. The generated solution would face the same checks as a manually written program. Unsupported checks, timeouts and inconsistent assumptions must remain distinguishable from success.

### Midnight as the execution target

Compiled Moriarty contracts must run on Midnight and use ZKIRv3, Midnight's circuit intermediate representation, for native proofs bound to ledger execution. The intended path is to author and inspect a program, check its types, effects and bounds, expose its proof obligations, compile it for a pinned target, generate native evidence and submit it for ledger acceptance. Each transformation must preserve the relevant meaning through to the actual effects.

<details>
<summary>Compiler and formal-semantics details</summary>

Compact is Midnight's smart-contract language. It can serve as an intermediate when its relationship to the emitted ZKIR and target verifier is pinned and checked. Moriarty's current mapper translates a restricted part of the older atomic profile into Compact circuits that compute numeric state and proposed financial effects. Those circuits do not themselves move assets or implement the full acceptance protocol. The richer local source profiles need their own demonstrated path to native proof and settlement.

Moriarty uses Midnight's native proof system, based on PLONK/KZG. Its guarantees inherit the cryptographic, setup and verifier assumptions of the pinned backend. K, a framework for executable language semantics, supplies an independent reference for selected language behavior. Comparing the evaluator with K can expose differences in tested cases; proving that compilation preserves meaning for all supported programs requires further work. Lean, a separate theorem prover, is not a Moriarty dependency.

</details>

Proof-carrying data (PCD) is data accompanied by evidence that it and the predecessors it depends on were produced according to specified rules. For Moriarty, the target is a transition certificate that connects the proposed state and effects to an authenticated origin and compliant predecessors. Proof recursion means verifying predecessor proofs inside a new proof; source computation stays bounded while the evidence certifies earlier stages. Comprehensive native recursion is intended to support portable history and private handoff, where one participant passes a continuing agreement and the required private data to another authorized participant. It must also support combining histories with multiple predecessors. Such composition must preserve consent, cumulative spending and every outstanding duty. Combining proofs without checking those relationships would only compress evidence for disconnected claims.

The requirements page calls the next-version backend workstream **ZKIRv4**. This is a proposed requirements label, not a claim that an upstream release with that name has been announced. The approximately March 2027 horizon for comprehensive Midnight recursion is a project planning assumption recorded on September 19, 2026, rather than a verified release commitment.

## The security model

The central adversarial case is a party proposing an execution that appears to meet the owner's request while exercising more authority, hiding an effect or discarding an obligation. A solver, proof producer, transaction forwarder or counterparty may be malicious. A witness is the supporting data supplied to a proof, including private inputs when the relation permits them. The acceptance rules must constrain every accepted witness, including ones that the normal software would never generate.

Acceptance requires four kinds of evidence. The agreement's required properties must hold over their stated domain. The chosen execution must stay within the owner's authenticated authorization, a requirement called intent refinement. The next state and financial effects must follow the program's rules. Finally, the transition must extend a compliant history from a legitimate origin. Each obligation needs an enforcement point in native constraints, authenticated state or a justified ledger mechanism. A host-side check, an unused Boolean or a label saying "verified" cannot supply a missing obligation. If an owner's signed authorization requires evidence of a spending limit, a prover cannot omit that requirement or substitute a check that does not enforce it. Acceptance must reject missing evidence, unsupported required checks and unresolved dependencies.

### Bind the proof to the execution

An accepted proof must concern the right program, semantic profile, intention, domain, stage, state and effects. It must also use the authorized circuit and verifier identity. These bindings prevent a valid proof for another agreement, chain or verification key from being substituted for the required claim. Source, intermediate representation and emitted target code have different identities; their connection requires an established correspondence, rather than an assumption that matching names imply matching behavior.

Complete effects matter here. A proof that checks the requested output while omitting another debit, a fee or a newly created liability leaves the owner exposed. The implementation must bind what the ledger accepts to the complete financial relation. Required checks must govern acceptance on every relevant path, including any phase that retains effects after later work fails.

### Preserve authority and history

An attacker must not be able to reuse a payment authorization, invent a favorable starting balance or restart a continuation with its spending counter reset. Signatures and delegation bind the permitted action to its owner. Replay protection and unique consumption prevent reuse of an authorization or receipt, authenticated genesis establishes the legitimate starting state, and the continuing state carries forward the spending and liability accounting described above.

A recovery action needs the same care as an ordinary payment. Learning that an external payment succeeded permits the agreement to record that fact; it does not authorize another transfer. Amendments and disclosures also require their applicable authority. Revocation can end a delegated right while leaving existing debts intact. A separately signed recovery policy may remain usable after ordinary execution authority expires, within its own scope and termination rule.

Privacy introduces another attack: a borrower might prove adequate collateral while concealing a debt that should count against it. A private agreement therefore needs evidence that all relevant positions were included and that no consumed claim was reused. The next participant also needs access to the supporting private data their proof requires. A commitment is a cryptographic record that binds the proof to particular data. Recording it does not give the next participant access to that data, so the agreement can still become unable to progress.

### Make external trust explicit

An external observation has an issuer, a domain, a freshness rule and a finality meaning. A transaction's inclusion in a block may establish neither successful application execution nor final settlement. The adapter must relate the exact submitted transaction and its authenticated result to the financial effects Moriarty accounts for.

Conditional release can depend on signatures, document predicates, proofs, recipient actions or combinations of these conditions. The policy must specify what each item establishes. An attestation may establish that a named issuer made a claim; its truth depends on the issuer and verification policy. Legal enforceability and physical delivery require assumptions beyond the existence of a cryptographic commitment.

Uncertainty must survive into recovery. A timeout does not prove that another chain failed to execute. A refund and a late successful payment must not both discharge the same escrow claim. The design therefore needs explicit unresolved states, reconciliation rules and exclusive terminal outcomes. Compensation is a new authorized action whose costs and effects are recorded; it cannot reverse history across unrelated ledgers.

### Separate safety, privacy and availability

Zero knowledge can limit disclosure while proving a defined relation. It does not automatically hide timing, network traffic, public amounts or information revealed to counterparties. Each workflow needs a disclosure policy and a statement of what remains observable.

Safety means that an accepted step stays within the specified rules. Availability and liveness concern whether a valid step can be produced and included at all. Bounded evaluation helps control computation, but proving costs, witness availability, state growth, resource exhaustion and chain access remain engineering and protocol obligations. Recursive proof compression alone does not bound the storage required by a long-lived agreement.

A proof of compliance does not establish profitability or optimal execution. Price, deadline and exposure constraints bound acceptable behavior; they do not by themselves prevent transaction-ordering manipulation.

## How the Federated DeFi Kernel works with Moriarty

The proposed Federated DeFi Kernel coordinates work that extends beyond one local agreement. Its intended responsibilities include collecting intentions, connecting solvers, obtaining evidence, arranging constrained signing, submitting external transactions and managing authorized recovery. Moriarty defines acceptable behavior and the evidence required for it. Midnight checks the native proof relation and enforces its own ledger's state and consumption rules. Each external domain retains its own execution and finality assumptions.

Kernel integration and solver connections remain design work in this Moriarty repository. Planned adapters cover the Open Wallet Standard (OWS) for wallet operations and x402 for HTTP service payments. The local evaluator APIs are the available starting point. An [interactive illustration of the kernel](https://charleshoskinson.github.io/Moriarty/kernel.html) walks one two-asset agreement through partial delivery, an unknown outcome and its alternative endings; it is an educational model of this target design, not a third reference and not a deployed service.

The CAKE chain-abstraction framework distinguishes four concerns. Applications express the financial purpose. Permission records user and application authority, including required consent. Solvers search for candidate plans within that authority. Settlement mechanisms execute effects and establish what occurred. The kernel coordinates these activities without acquiring a right to weaken the agreement.

A typical coordinated workflow begins with an owner signing constraints and allowed evidence policies. A solver proposes a route. The relevant budgets are reserved, the candidate is checked against the agreement, and each permitted stage executes under its stated conditions. Authenticated results update the continuing state. Further action depends on those results, with unresolved outcomes retained for reconciliation and recovery rather than treated as either success or failure.

Applications specify the evidence policies under which an agreement accepts results; federation operators run the associated signing and attestation services. The kernel's zero-knowledge (ZK) proofs, multi-party computation (MPC) and trusted execution environment (TEE) mechanisms provide different assurances. ZK proves the specified relation. MPC or threshold signing distributes control under a stated corruption threshold. A TEE adds claims about an attested execution environment, subject to hardware, freshness and rollback assumptions. These mechanisms must refer to the same intention, program, domain, stage, epoch and effects; placing them together does not establish independent security when operators or infrastructure are shared.

The destination's enforcement boundary is decisive. If a foreign account accepts a threshold signature alone, compromise of that threshold may bypass the policy honest signers would have checked. A Moriarty proof cannot force that account to verify a relation its native rules do not require. Federation membership changes, signing thresholds, evidence policies and failure remedies therefore need explicit treatment in each supported integration.

The local evaluator used by the demos computes a bounded stage and checks its financial actions. The optional federation coordinates participants and external systems. Supported direct Midnight programs, including the target private handoff capability, must be usable without federation membership. Authoring, compiling, proving and deploying a supported Moriarty program requires no project, council, registry or provider approval, and no privileged solver. Owner consent, application conditions and objective ledger validity rules continue to apply.

## Use cases

The following examples describe intended capabilities and the obligations they motivate. They are design illustrations, not claims that each end-to-end application is implemented today.

### Conditional settlement and programmable escrow

A buyer can authorize release only when specified signatures, an authenticated document predicate and a recipient action have all been established. The agreement can allow partial release for partial fulfillment while retaining the balance and outstanding duties. This supports trade finance, staged purchases and other arrangements in which submitting a payment request and becoming entitled to payment are separate events. Expiry and dispute remedies must be authorized explicitly; the evidence policy determines what the program can establish about the underlying goods or services.

### Partial fills and multichain execution

An owner can permit an order to fill in several stages while constraining the assets, minimum net return, total fees and cumulative spending. A solver may combine liquidity sources or coordinate settlement across domains. Each accepted fill reduces the remaining order and records its costs. If one leg settles while another remains unknown, the continuation preserves the resulting exposure and permitted remedies. Global atomicity is available only where the actual participating mechanisms establish it.

### Loans, collateral and continuing claims

Principal, accrued amounts, repayment and collateral duties are distinct quantities that a lending agreement must account for. Refinancing or transferring a claim must preserve the relevant liabilities and obtain required consent. A liquidation path must establish its trigger, price policy and resulting allocation, including any remaining debt. This is why token conservation alone is insufficient: moving the expected tokens does not prove that the agreement accounted for every claim.

### Treasury management by AI solvers

A treasury can delegate a bounded task with approved assets, venues, price constraints and spending limits. Several AI solvers may search concurrently, provided reservations and completed spending are checked against the same authority. The intended Open Wallet Standard integration gives solvers a common interface for wallet operations; the program defines the limits within which those operations may be authorized. A solver can exercise the discretion the treasury granted without obtaining general custody authority from its ability to propose a valid strategy.

### Paid services and automated procurement

Buying data or computation through x402 HTTP service payments requires separate records for authorization to pay, final payment, receipt of a result and satisfaction of the result's predicate. A retry retains the logical request identity so that uncertainty about delivery does not authorize duplicate charges. The evidence available for a service determines which delivery claims can be proven and which require a trusted issuer or an explicit remedy.

### Private institutional agreements

Participants may need to establish collateral adequacy, authorized participation or compliant transfers while limiting disclosure of positions and counterparties. The target design allows independently controlled participants to combine private agreement histories or take over a continuing agreement without discarding its duties. Combining histories requires evidence that the resulting agreement preserves the relevant obligations and authorizations; a handoff also requires access to the private data needed for its next stage. Selective disclosure can support an agreed audit policy. Completeness remains essential: a private proof of adequate collateral is useful only if the relation also accounts for the liabilities it is supposed to cover.

## Where the project stands

Developers can parse, check and locally evaluate supported source programs, including a loan's origination, accrual, repayment and settlement. Selected executions have also been compared with the independent K semantics. Fixed loan and swap examples have executed on Midnight Preview, the project's network environment for testing ledger integration. In that loan example, the payment discharged an amount due while leaving principal outstanding; it did not close the whole loan.

These results demonstrate particular programs and financial effects. They do not yet establish the full path from arbitrary supported source programs to mandatory native proofs of their complete ledger effects. The scoped records remain in `deliverables/`; kernel and wallet integration interfaces described above are still planned.

The full path still needs proof that compilation preserves the language's meaning and that acceptance enforces the complete authenticated intention. Native recursive history, private handoff and composition must carry those guarantees between stages and participants. Conditional settlement, recovery and broader financial libraries need their own conformance evidence, including the behavior of federation and solver integrations at external boundaries.

Each claim needs an appropriate check. Semantic comparisons test selected cases; compiler correspondence addresses preservation of meaning; adversarial witness tests look for invalid executions that constraints admit. Resource measurements and ledger results establish further, distinct facts. Before claiming a financial capability for release, the project requires its Midnight Preview effects and resulting state to be observed and checked. That evidence requirement governs project claims, not users' permission to deploy programs.

The immediate direction is to connect a newly authored bounded financial program through the public compilation and proving path to observed Midnight effects, with controls that reject altered intention, effects and history. That same relation must then extend to partial fulfillment, conditional escrow and late-result recovery. Recursion and private composition expand how compliant history can be carried between stages and participants. The requirements reference above records these obligations; the syntax and semantics reference records the language profiles available for inspection today.

## Local developer quickstart

Use Node.js 24. The local demos require no wallet, faucet, network access or npm dependency installation after obtaining the repository:

```sh
git clone https://github.com/CharlesHoskinson/Moriarty.git
cd Moriarty
npm --prefix experiments/moriarty-language run demo
npm --prefix experiments/moriarty-language run loan-lifecycle-demo
```

The `demo` command evaluates the existing atomic-profile loan and swap examples. It includes deliberately rejected inputs and an acceptance attempt without a proof backend. Those rejections are expected. Run `node experiments/moriarty-language/examples/simulate.mjs --json` for the structured inputs and candidate results.

The lifecycle demo uses `moriarty-financial-agreement-source/5`. It originates 100 units of principal, accrues 10, repays 30 and settles the remaining 80 through successive local evaluations. It also rejects duplicate accrual and settlement after the debt reaches zero. These are simulator results: the commands do not sign, generate proofs, submit transactions or move ledger assets.

To check that example's source directly, run this from the repository root:

```sh
node experiments/moriarty-language/src/cli.ts check --profile moriarty-financial-agreement-source/5 experiments/moriarty-language/spec/successor/examples/loan-lifecycle.mori
```

The language package guide at `experiments/moriarty-language/README.md` covers the APIs and local workflows. Profile-specific specifications and fixtures live under `experiments/moriarty-language/spec/successor/`; restricted compilation is documented in `experiments/moriarty-language/compact/MAPPING.md`. The browser developer mock is a separate prototype with simulated authority and certificates.

## Finding the implementation and research

`experiments/moriarty-language/` contains the source language, evaluator, examples, formal definitions and Compact mapping. `experiments/moriarty-midnight-network/` contains network integration work; `experiments/moriarty-native-ivc-r3/` retains the native recursion experiments and their limits.

The repository also holds the project research vault. `wiki/index.md` and `wiki/overview.md` lead to the findings and their sources; `docs/OBSIDIAN.md` explains the vault workflow. Obsidian is an optional viewer. `openspec/` holds implementation requirements, while `deliverables/`, `evidence/` and `raw/` retain scoped results, reviews and source material. Historical plans and receipts describe their recorded versions; the requirements page above carries the consolidated design direction. `docs/ARCHIVE.md` locates superseded work.

Contributing agents follow the startup instructions in `AGENTS.md`, including loading `moriarty-dev:develop` and checking current status. Those instructions govern work in this repository; they confer no authority over users' financial programs.
