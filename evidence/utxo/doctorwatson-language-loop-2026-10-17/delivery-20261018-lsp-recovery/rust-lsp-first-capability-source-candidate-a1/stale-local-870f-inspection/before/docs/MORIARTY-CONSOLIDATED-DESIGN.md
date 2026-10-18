# Moriarty consolidated vision and design

Status: proposed target architecture, 2026-09-19. This proposal consolidates the research agenda; it does not claim the language or federated runtime is implemented. The product contract controls permissionlessness and the native Midnight target. The single delivery sequence is ROADMAP.md; OpenSpec records required behavior, EARS supplies observable acceptance, and Pel describes internal implementation work.

## Vision

Moriarty is a permissionless, bounded financial programming language in which a developer defines valid financial behavior and a user authorizes the precise outcomes, costs, obligations and disclosures they will accept. Independent solvers can find ways to satisfy that intention. Every accepted execution must establish that the actual effects satisfy both the program and the signed intention, while preserving unresolved duties and compliant history.

Compiled programs run on Midnight through ZKIRv3 and Midnight's native Halo2-derived PLONK/KZG proof stack. Moriarty supplies the semantics and enforced constraints; cryptography establishes those constraints, not unexpressed human wishes. Signing displays must make the canonical intention inspectable. No Moriarty project membership, reviewer vote, Foreman record, approved solver or central program registry is required to author, compile, prove or deploy a supported program.

The Federated DeFi Kernel is an execution and coordination system that can serve these programs. It discovers and coordinates solvers, obtains evidence, operates optional threshold custody, routes cross-chain effects, observes finality and executes recovery. It cannot silently strengthen its own authority or weaken the user's constraints. Moriarty can also run on Midnight without this federation. Asset-owner authorization and application-specific policies remain mandatory where applicable.

## One language, several layers

The small core contains exact typed values; bounded evaluation; explicit state and effects; checked authority; assertions and refinements; authenticated evidence; persistent obligations and continuations; and defined composition operators. Financial applications are source libraries over that core. A language version states exactly which constructors, bounds, numeric semantics and evidence relations its compiler supports.

Use the existing K semantic work as an executable reference, reconciled with the current typed source/Core. Finite agreement with a local evaluator is regression evidence, not a compiler theorem. Do not introduce a second mandatory semantics in Lean. DeFiFormal's Lean development supplies valuable reference contracts and examples; neither its tooling nor its theorems automatically become Moriarty dependencies or native certificates.

The public pipeline is: author and inspect → elaborate/type/effect/bound check → expose proof obligations → compile to pinned ZKIRv3 → generate native evidence → submit → check actual ledger results. Compact is permitted as an intermediate only with an explicit, pinned correspondence to the emitted ZKIRv3 and target verifier. A local evaluator or generated Compact file is not the terminal product.

## Responsibility boundary

| Concern | Moriarty language and acceptance relation | Federated DeFi Kernel | Midnight / external domain |
|---|---|---|---|
| Intent | Canonical signed constraints, version/domain binding, amendments and allowed completion holes | Collect/distribute authorized requests; propose plans | Signature and state consumption mechanisms actually used |
| Financial meaning | Gross debits, fees, minimum net outcomes, assets, custody, liabilities and residual duties | Find liquidity and counterparties; arrange execution | Execute and report authenticated complete effects |
| Permission | Owner consent, scoped delegation, application policy and replay rules | Optional service eligibility and operational admission | Native validity and chain-specific authority |
| Proofs | Exact mandatory relation, certified primitive obligations, source/target correspondence | Produce/transport/check evidence under pinned profiles | Native verifier checks the accepted relation and bound effects |
| Solver search | Preserve hard constraints through completion, aggregation and netting | Human or AI search, matching, scheduling, retries | No privileged solver bypass |
| Conditions | Typed condition/evidence policies, freshness, disclosure and conjunction/threshold rules | Collect documents, signatures, attestations and observations | Issuers/oracles/signers remain explicit trust assumptions |
| State/history | Authenticated genesis, predecessor identity, obligation persistence, uniqueness/consumption requirements | Maintain witness availability, coordinate continuation and reconciliation | Midnight authenticates local state; each external domain provides only its stated evidence |
| Atomicity | Distinguish atomic batch, committed prefix, independent fork/join and interleaving | Coordinate asynchronous multi-domain workflow and compensation | Actual transaction phases and each domain's finality determine atomic boundaries |
| ZK/MPC/TEE | Require named evidence for the same statement; expose assumptions | Operate provers, threshold signing and enclaves as configured | ZK soundness, corruption threshold and hardware attestation are separate properties |
| Recovery | Authorized remedy, residual duties, budgets, late results and exclusive terminal outcomes | Observe failure/unknown outcomes, reconcile and execute permitted remedy | Timeout alone proves neither nonexecution nor entitlement to refund |
| Privacy | Explicit permitted disclosure, private-state completeness and witness-handoff obligations | Minimize access; manage encrypted witnesses and recipients | Network, observer, hardware and public transcript leakage remain named |
| Release process | No maintainer workflow input in public acceptance | No project gate made universal through an SDK | Objective native fees and resource limits still apply |

CAKE's APSS separates Applications, Permission, Solvers and Settlement. Moriarty expresses the enforceable contracts across those layers; it is not the whole stack. Permission means user/application authority, not Moriarty deployment licensing. Solvers supply candidate solutions. Settlement adapters establish concrete effects and their domain limits. The federated runtime coordinates them.

## Canonical stage statement

Every accepted stage binds: semantic and numeric profile; source/Core/program identity and deployed entry point; circuit/verifier/key identity; chain/domain and authenticated state frame; signed intention and consent/delegation policy; lifecycle/stage/logical-request IDs; predecessor and obligation commitments; typed observations with issuer, domain, time and finality; complete gross and net effects including fees and supply changes; opening and closing liabilities; authority consumption and replay state; resource certificate and cumulative reservations; permitted disclosures; selected failure/phase policy; and resulting continuations or terminal outcome.

Source, Core, emitted ZKIR and verifier keys have distinct identities. Equality is required only for the same canonical representation; transformations are linked by pinned correspondence relations and commitments, not literal equality of their hashes. The enforcement map must also state whether signed-intent authentication occurs in the circuit, a bound ledger primitive or another explicitly justified native boundary.

These are logical obligations, not a claim that the current native transcript binds each field automatically. The implementation must identify for each field the actual circuit constraint, authenticated state read, signature commitment or ledger check. A host JSON field or host-computed verification Boolean is insufficient. The owner commits to the policy and allowed evidence predicates before execution; a prover cannot choose a weaker relation or substitute a key.

Separate the four required judgments: contract properties, intent refinement, valid state/effect transition, and compliant history. Checking a predicate and discarding its Boolean does not enforce it. Bind actual complete effects rather than a convenient projection selected by a prover. Rejection/partial failure is itself a specified transition with authority, fees and duties accounted for.

## Assets, authority, obligations and arithmetic

Assets and domain-qualified quantities have exact identities and units. Checked finite-width arithmetic, overflow, rounding and prices are explicit. U0 freezes canonical price orientation, unit dimensions, per-primitive rounding direction and beneficiary policy in the numeric profile; U1 certificates bind that profile. Amounts must not silently become field elements modulo the proof field. DeFiFormal prices are quote-per-base while the current Moriarty convention is base-per-quote: adaptation requires an explicit dimensioned conversion and directed rounding, not a rename.

Consumed receipts are linear resources. Spending permission may be affine: unused authority need not be exercised. Financial liabilities are persistent until discharged, transferred with applicable consent, amended or explicitly forgiven. Default does not erase debt, and forgiveness is not repayment. Creating a liability requires applicable consent from the party made liable. Token conservation alone cannot prove these properties.

Track gross debit and fee limits separately from net outcomes. Netting requires a relation preserving the authorized gross economics, liability ownership and residual duties. A refund cannot replenish authority to evade a gross cap unless that replenishment was explicitly authorized. Concurrent solvers reserve spent plus pending exposure against the same authenticated budget.

## Conditional settlement and bounded stages

An uncommitted candidate alone changes no ledger state or obligations. Any accepted phase, including a retained guaranteed phase, can create or preserve only explicitly authorized duties. Recording a request does not impose a duty on an unconsenting recipient.

A workflow may be submitted without funding; funded into programmable escrow; partially fulfilled; waiting for evidence; eligible for release; in flight; delivered; in an unresolved state; or recovering. The product must distinguish these facts, even if the final syntax represents them with several typed records rather than one enum. Evidence-ready is not delivery, and payment-final is not service-delivered.

Conditions may combine signatures, recipient acceptance, document commitments and predicates, proofs or supported external evidence. Documents establish only the authenticated predicate actually checked; their existence does not imply legal truth or universal enforceability. A timeout changes which authorized transitions may be attempted. It does not prove another chain did not execute.

Every stage terminates within a checked bound. A long-lived workflow progresses through authenticated continuations; it does not obtain unbounded recursion inside a stage. Bounded termination does not prove funds can always be recovered: liquidity, actors, chain inclusion, finality and witness availability are explicit liveness assumptions.

Initiate, complete, reconcile, recover, disclose and amend rights have separate scopes; no fixed default expiry or perpetual spending right is implied. Knowledge reconciliation does not itself authorize a new transfer.

Ordinary authority can expire while a narrowly scoped recovery authority remains usable under its signed conditions. Recovery grants declare their own signed termination rule: an expiry or owner-chosen indefinite duration with scoped revocation. Exclusive terminal outcomes consume/tombstone the applicable authority. Revocation cannot erase outstanding duties; unavailable recovery remains explicit or uses a separately authorized remedy. Optional kernel service eligibility can refuse service but is not program validity and cannot override Midnight acceptance. Reserve work cannot be consumed by ordinary progress. Under the signed policy, the program/ledger acceptance predicate for any claimed recovery guarantee must establish a viable supported closure/recovery path under named assumptions or reject a workflow claiming that property. Local finite arrays and append-only IDs require either explicit finite episodes or authenticated rollover preserving cumulative budgets, obligations, replay and terminal tombstones. Proof compression alone does not solve state growth.

Sequential composition retains committed prefixes. Disjoint parallel composition requires checked independence and complete joining. Shared-state interleaving requires interference reasoning. Atomic publication applies only within a declared domain and its actual phase semantics. Cross-domain workflow uses authenticated progress, conditional release and compensation; it must never claim global rollback from a local model.

## Native proof-carrying history

Planning assumption supplied by the user on 2026-09-19: Midnight is assumed by the user to have comprehensive recursion in about six months, approximately March 2027. The target architecture therefore includes full native recursive compliance, portable history, private handoff and bounded multi-parent composition. Prepare these relations, interfaces and tests now; qualify the actual released interfaces before reporting support. This is a planning assumption, not a verified delivery date. Current limitations determine the early implementation subset, not the ultimate language scope.


Use one history meaning and explicitly distinguish two evidence mechanisms. For a Midnight-resident lineage, authenticated genesis, mandatory preserving transitions and unique current-head consumption can support ledger induction. This is not a recursively verified predecessor proof.

MC06 private handoff and split/join must be demonstrable between independently controlled participants without the federated kernel. Optional kernel-assisted joint proving is an additional profile with UNI-011 assumptions, not the sole language evidence.

For portable/off-ledger segments, private handoff, bounded split/join and imported histories, require the compatible native certificate relation and legitimate origin/predecessor proofs under the target profile. The target bounds each stage and predecessor fan-in (at least two for MC06), not all history to one fixed-size DAG. It supports growing finite histories across stages under well-founded composition. Each signed episode retains its declared finite global work/authority limits; longer-lived continuation requires authenticated authorized successor episodes without resetting an existing signed lifetime budget. No literally infinite execution, unbounded per-stage work or automatic unlimited-depth machine counter is promised.

Native proof aggregation alone does not establish historical compliance. An external chain inclusion proof alone does not prove Moriarty semantics. Any bridge or federation attestation has the exact conditional trust meaning specified by its policy.

The earlier categorical rejection of general DAG history is superseded as a scope decision: bounded causal composition remains required. Per-transaction recursive verification of a previous ledger contract-call proof is not assumed available. Transcript/key/statement compatibility must be established. MC03 native recursion and MC06 private composition remain open acceptance obligations; local ledger induction cannot be used to mark them complete. Full-agenda release requires the retained native recursion/private-history evidence, or a future explicit scope decision—not silent substitution.

## Certified primitives, libraries and developer assistance

Follow Simplicity's lesson: small explicit meanings, with optimized jets certified against those meanings. A certificate must cover values, failure behavior, complete effects, preconditions and the declared cost relation, plus valid-execution completeness and invalid-witness exclusion for the pinned target constraints. Call sites discharge preconditions and framing. A recognized hash identifies semantics, not permission to deploy.

Build financial breadth as reusable libraries: exact arithmetic/fees; payments and escrow; loans and claims; swaps and liquidity; shares/vaults; ACTUS cash-flow contracts; redemption/loss allocation; margin; asynchronous and conditional claims. Each family receives a scope matrix, independent source-derived expectations and held-out compositions. Narrow token0 pricing is not a full AMM; stable-time vault conversion is not accrual. All original ACTUS/DeFi denominators and release obligations remain visible.

Adopt Aeon's useful authoring ideas in order: obligation/trust reports, exact advisory refinement checking with replayed counterexamples, then typed holes and bounded synthesis. Unknown, unsupported, timeout and inconsistent assumptions are distinct outcomes. A generated program follows the same public pipeline. Solver completion may choose only authorized degrees of freedom and must preserve every constituent intention. Synthesis is optional for manual authoring; proof-carrying execution is not.

OWS provides wallet interoperability, not unrestricted financial authority. x402 provides scheme-specific payment flows, not proof of recipient delivery. Logical paid-request identity survives retries; reconciliation precedes any authorized new charge. ZK/MPC/TEE evidence must name the same program, intention, epoch, domain, stage and effects. A foreign destination that accepts only a threshold signature can be bypassed if that threshold is compromised, even when honest signers check proofs. State that conditional enforcement boundary and the federation’s membership, threshold, ordering, equivocation, availability and epoch-change rules explicitly. Signed fallback policies must not become an unsigned evidence downgrade.

A TEE attestation cannot replace a missing program proof; threshold signatures cannot establish the truth of an oracle.

## Evidence and decisions still required

Current source/5 evaluation, finite K comparisons and scoped Preview loan/swap results are separate foundations. The whole-language audit found a disconnected rich-source/legacy-lowering path, recovery-reserve and lifetime-state gaps, and incompatible history/expiry prose. This proposal resolves ownership and scope; it does not repair those implementations.

Before dependent implementation, pin the exact native compiler/ZKIR/verifier/key/ledger tuple, signed failure policy, public input coverage and witness constraints. Demonstrate native recursion compatibility and fit before claiming portable certified history. Define private completeness/consumption and recovery availability explicitly. The R3 k17 failure and every prior resource debit remain evidence; a new roadmap does not authorize an unbounded retry.

The next product slice should connect one newly authored bounded financial program through the public pipeline to actual Midnight effects, with signed intent and all applicable mandatory claims. Include a contrasting valid program to expose fixture dispatch and tampered intent/effects/history controls. Separately track uncertified integration experiments. Expand this same relation to conditional two-asset escrow, partial fulfillment, residual obligations and late-result recovery, then private history and federated execution. Do not create another application-only compiler.

## Backend evolution contract

[Next ZKIR and recursion requirements](MORIARTY-BACKEND-REQUIREMENTS.md) separate mandatory correctness contracts, required capabilities and measured usability targets. They assign obligations to ZKIR, compiler, proof runtime, ledger and Moriarty rather than assuming new instructions alone solve authorization, privacy or settlement. The initial execution target remains ZKIRv3; requests for the next version do not claim a ZKIRv4 release or prescribe its numbering.

## Source basis

- [Whole-language audit](../deliverables/whole-language-review-2026-09-19/REVIEW.md): integration, reserves, lifetime state, expiry and roadmap conflicts.
- [DeFiFormal](../deliverables/defiformal-study-2026-09-19/RESULT.md): composition and reusable financial reference contracts.
- [Aeon consensus](../deliverables/aeon-study-2026-09-19/review/FINAL-CONSENSUS.md) and [Anoma study](../deliverables/anoma-study-2026-09-19/SYNTHESIS.md): refinements, certified basis and constraint-preserving solver completion.
- [MPLR register](../wiki/research/mplr/index.md): NEAR, Daml, Simplicity, APSS, OWS/x402 and Anoma requirements and source provenance.
- [Six independent proposals and synthesis](../deliverables/consolidated-design-2026-09-19/CONSENSUS.md): exact scope, disagreement dispositions and final review.

Historical native-interface findings remain tied to their original pins and dates. No fresh backend build or network execution is claimed by this consolidation.

The [Mina case study](../deliverables/mina-recursion-study-2026-09-19/RESULT.md) provides source-backed recursion failure cases and eight mandatory backend refinements. It strengthens this native Midnight architecture without changing the proof stack or relaxing the six-month full-recursion target.
