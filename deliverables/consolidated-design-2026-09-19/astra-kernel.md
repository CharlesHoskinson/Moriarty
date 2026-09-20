# Independent proposal: one language contract, explicit execution boundaries

Adviser role: distributed systems, programming languages and security boundaries. Requested route: GPT-6 Astra medium; orchestration metadata, not this self-description, determines actual model identity. Date: 2026-09-19. Advisory only; no implementation, build, proof campaign, deployment or canonical vault mutation performed.

**Recommendation.** Consolidate Moriarty around one versioned acceptance relation for owner-authorized financial stages. Moriarty defines and compiles that relation to Midnight ZKIRv3. The optional Federated DeFi Kernel coordinates candidate search, evidence, signing and external execution against it. Midnight enforces its own ledger boundary. Each foreign adapter supplies a separately specified correspondence and observation boundary. DeFiFormal remains a semantic reference library. This division preserves permissionless programming while making consent, cryptographic assumptions and unresolved settlement visible.

## Evidence and status

I read the complete 3,027-line frozen evidence packet in chunks, including product contract, roadmap, whole-language review, DeFiFormal reviews, Anoma synthesis, Daml security proposal, OWS/x402 synthesis, historical native-PCD decision, completion program, MPLR-001–035 and APSS studies. I additionally read repository AGENTS.md, the checked-in develop skill, `wiki/research/daml/kernel-boundaries.md`, and kernel-security design/implementation-plan. Citations below identify these supplied local sources; their embedded remote citations were not freshly acquired or independently revalidated.

Repository observation: required status CLI reports SP01.6 loan-swap-subset, implementation dispatch blocked by stale input bindings and missing current accounting/resource state, and no pending transaction notifications. That is an internal operational snapshot. It does not negate the separately retained September 17 scoped loan and swap receipts in ROADMAP.md. Source observation: local source/5, finite K comparisons and scoped Preview financial effects exist; the general rich-source-to-ledger relation, mandatory native history proofs, private composition and complete conformance remain open. I did not independently reproduce those retained experiments. [docs/MORIARTY-PRODUCT-CONTRACT.md; ROADMAP.md; evidence-packet.md sections for whole-language REVIEW.md and completion program]

Everything below is a recommendation or specified-only obligation unless explicitly labeled as a source observation. No adviser vote establishes soundness or acceptance.

## Ownership and APSS

| Boundary | Owns | Does not establish by itself | Concrete obligation |
|---|---|---|---|
| Moriarty language/Core | Typed values, exact arithmetic, invariants, permitted effects, signed-intent refinement, bounded stages, obligations, evidence and authority judgments | Network delivery, honest oracles, available witnesses, compiler correctness merely from a definition | One supported relation, explicit profile embeddings, source/Core/constraint/ledger correspondence |
| Public compiler and proof tooling | Elaboration, obligation reporting, lowering, native proof production and pinned artifact identity | The user's unexpressed wishes or acceptance on a different ledger/version | Reject unsupported or undischarged obligations; expose trust and supported scope |
| Midnight | Native proof verification, authenticated state access, ledger ordering/consumption and phase execution at the pinned version | Foreign finality, arbitrary recursive claims, business success from block inclusion | Bind actual verifier keys, operation identities, phase layout and all retained effects |
| Federated DeFi Kernel | Optional scheduling, matching, evidence gathering, constrained signing, reservations, retries and reconciliation | Authority to deploy public Moriarty programs; universal consensus or global atomicity from federation membership | Every released effect refines the signed policy under an explicit enforcement mechanism |
| External adapter | Exact transaction encoding/decoding, domain/account/assets, submission and outcome interpretation | Intent correctness from an API match or finality from an acknowledgment | Behavioral contract over actual bytes, effects and declared observation/finality assumptions |
| Wallet/owner/counterparty | Canonical grants, amendments, material obligation consent, disclosure/recovery choices | Program invariant from a valid signature; blanket authority from a transport token | Exact signed message, current scope and consumption state, inspectable display |
| DeFiFormal | Reference specifications, distinctions, conditional theorem statements and counterexamples | Deployed federated runtime, Lean dependency, native circuit proof, automatic transfer of theorems | Explicit adaptation map; separately discharge native implementation and correspondence |

APSS is a useful responsibility view, not four required hosted services. Applications define contracts and user-facing bargains. Permission expresses scoped consent, delegation, revocation and consumption. Solvers search candidates under immutable hard constraints. Settlement establishes actual outcomes under ledger-specific rules. Moriarty supplies the semantic interface across all four; the kernel optionally coordinates them. The certified basis underpins this interface rather than becoming an administrator of application identities. [APSS applications/permission/solvers/settlement/certified-basis explanations]

Keep three policy namespaces disjoint: (1) objective language/proof validity, (2) owner/application consent, and (3) optional provider admission and maintainer process. A private lending application may demand a lender signature or selected evidence issuer. A relay may refuse customers. Neither policy becomes a universal compile/prove/deploy requirement. Demonstrate this with a clean developer installation and two independent unregistered candidate sources, including direct submission without the federation. Supported primitive certification identifies semantics, not approved authors. [Product contract; MPLR-015/016]

## Proposed boundary contract

Use one canonically encoded, domain-separated `StageStatement` as the commitment target. This is proposed notation, not an existing API. It must permit private committed fields whose consistency is constrained; publishing every sensitive field is not required.

| Field group | Required fields and checks |
|---|---|
| Semantic identity | Program/source/Core commitments; semantic profile and property specification; module dependency closure; primitive certificate identities and preconditions; exact target instruction/version tuple; compiled artifact, circuit/key and verifier deployment identities |
| Intent and principal | Canonical signed intent; principal-to-key rule; direct/standing consent; permitted solver holes/substitutions; recipients; exact asset/network/issuer identities; amendment policy; domain-separated replay identity |
| Authority | Separate spend, prove, read/disclose, administer, initiate, complete, reconcile and recover rights; delegation parent and attenuation relation; effective revocation state; consumption scope; stage-specific expiry |
| Workflow and causality | Workflow/logical-request identity; stage and attempt identity; authenticated origin; predecessor state roots and relation IDs; branch roster/join policy; unique resource/nullifier references; continuation version; exact state-domain completeness witness |
| Financial state | Pre/post custody and reserves, complete effect frame, transfer legs and fee recipients, authorized mint/burn, gross debits, net outcomes, liability creation/accrual/discharge and remaining duties; cumulative spent/pending/reserved values without double counting |
| Conditions and evidence | Predicate AST/version; subject/document digest; issuer/verifier; domain/stage/epoch; freshness, expiry, revocation and consumption policy; evidence class; observation status; disclosure permissions and unavailable-witness consequences |
| Execution | Candidate plan parameters; exact target transaction digest and material effect interpretation; adapter/profile version; phase placement; permitted success, retained-failure and unresolved outcomes; typed time units and finality policy |
| Trust and evolution | Proof relation and cryptographic parameters; federation participants/epoch/threshold and corruption/availability assumptions; hardware/attestation roots, measurements and rollback assumptions; observer/finality policy; authorized changes and explicit fallback logic |
| Resources and recovery | Per-stage and cumulative work/fee bounds, authenticated reservations, closure reserve, recovery actor and scope, successor-episode rules, terminal conditions and residual duties |

Keep an accompanying evidence record: claim ID; exact predicate; status (`specified`, `source-inspected`, `tested`, `proved-under-assumptions`, `locally-accepted`, `Preview-finalized`); artifact/source hashes; command/environment or theorem statement; public-input encoding; verifier result including any native deferred checks; transaction/phase receipt; independently expected versus observed complete effects; remaining assumptions; negative controls; and scope exclusions. Status is per claim, not one global green badge. A proof digest is artifact identity, not evidence that the proof was checked.

The proposed acceptance relation is a conjunction of typed semantic transition, current authority/consent, signed-intent refinement, complete financial/resource accounting, evidence policy, legitimate history and pinned target correspondence. The deployed path must enforce all four existing mandatory claims: ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance. An external-effect release additionally needs the adapter's enforcement contract. Typed evidence must not coerce a weaker statement into a stronger one. [Daml security-provability; MPLR-017/018/022/023/027/029/030]

## Trust composition and delegation

A ZK proof establishes its precise relation under cryptographic and implementation assumptions. MPC distributes signing/computation under a declared adversary model. TEE attestation authenticates a measured component/result under hardware, measurement, freshness and rollback assumptions. Foreign consensus/finality and documentary truth retain their own assumptions. Bind every required item to the same statement, exact effect and epoch.

An AND policy requires all its evidentiary judgments and consistent bindings; it does not justify treating correlated operators as independent assurances. An OR fallback is acceptable only when explicitly signed and its weaker assumptions are visible. No timeout may enable an unsigned downgrade from proof-and-threshold authorization to attestation alone. A federation must state n, threshold, corruption and availability bounds, membership change authority, quorum intersection where needed, state ordering and equivocation treatment. A threshold signature is not automatically a global-state consensus certificate.

If a foreign account accepts only a bare threshold signature, threshold compromise can bypass off-chain proof checks. State that safety premise explicitly. Prefer destination-enforced constraints where supported, but do not pretend an unsupported adapter has them. A TEE that verifies policy before signing still needs measured-code correctness, current authenticated inputs, anti-rollback and exact-output binding; it cannot remove the external account's trust boundary. [kernel-boundaries.md]

Delegation means semantic subset preservation: every child-permitted effect must be allowed by the parent in the same authenticated context. Shared children must consume a joint budget, not each copy the parent's cap. Contract entry-point permission alone is inadequate for nested callbacks, liabilities or disclosure. Separate initiating new work from finishing already-authorized work, evidence reconciliation, recovery and amendments. Expiry of initiation authority does not delete debt; prior recovery authority or an authorized amendment must explain every late action.

OWS is an interoperability surface. Its inspected token-plus-disk key recovery and partial effect extraction cannot enforce cryptographic attenuation. Do not deliver unrestricted fund-control keys to an untrusted solver. Use a constrained destination/account or a policy-enforcing signer with the disclosed trust model. x402 authorization, payment submission, finality, result availability and recipient delivery remain separate; logical paid-request identity survives retries and failover. Reserve budget durably before concurrent commitments. [OWS/x402 explanation; APSS permission; MPLR-031–035]

## Partial settlement, atomicity and history

Use distinct types/judgments for an incomplete uncommitted candidate, an accepted partial stage and a terminal workflow result. Conditional settlement records a request and waits for an explicit combination of evidence; programmable escrow is its funded variant. A document digest identifies bytes, while a predicate or trusted attestation supplies the application's meaning. Recipient acceptance must bind material terms; passive receipt alone must not create debt or a performance duty.

For each leg retain custody, authorization and knowledge separately. Suggested knowledge states are not-yet-submitted, submitted/unknown, observed success, policy-final success, authenticated failure/nonexecution and disputed/reorganization handling. Keep actual observed facts rather than forcing every expired request back to unknown. A join's readiness predicate differs from its success predicate; late and unselected branches retain their duties.

Midnight local phase effects and multichain sagas need separate semantics. A fallible failure may retain guaranteed effects/fees. The signed policy and lowering must agree on nonce consumption, fees and recovery for every permitted phase outcome. Cross-chain coordination is a saga of accepted transitions unless a particular protocol proves stronger properties under named assumptions. Refund returns assets still controlled; compensation is a separately authorized new effect. Neither rewrites history. Settle/refund exclusivity requires actual entitlement consumption or destination enforcement, not merely two well-formed proofs. Timeout alone proves no remote nonexecution. [APSS settlement; MPLR-001–014/017/019/024–026]

Select a hybrid history design as the initial implementation hypothesis: ledger-head induction for supported on-Midnight steps, native certificate composition for explicitly selected imported/off-ledger/private-history cases. Publish which history properties each mechanism establishes. Legitimate genesis, exact relation identity, compatible parents, well-founded split/join, aggregate budgets and persistent duties are mandatory. Native ledger uniqueness remains necessary even with recursive proofs. This does not discharge MC03 recursion or MC06 private handoff. The September 11 categorical rejection of general DAG/per-transaction PCD must be superseded where it conflicts with the newer required scope; retain its measurements and interface findings with their dates. Do not infer present recursion availability from that historical source.

Native feasibility must inspect the exact pinned current interface before implementation. Historical transcript incompatibility, zero guards, deferred pairing checks and unbound context fields are targeted questions, not presumed current facts. Source/target relation must enforce adversarial witness constraints, including applicable PR17 WShape premises; an honest witness generator is insufficient. Lean is neither a required dependency nor a proposed substitute proving backend. [Native architecture decision; product contract; certified-basis explanation]

## Conflicts and dispositions

| Conflict | Disposition |
|---|---|
| Older project admission/program registry requirements versus public language | Supersede public restrictions; retain internal resource/audit controls and objective proof/financial obligations |
| September 11 history-mechanism rejection versus MC03/MC06 and newer PCD obligations | Preserve semantic history requirements; explicitly scope ledger induction and native certificates; leave unmet recursion/split-join gates open |
| Local evaluator rollback versus Midnight partial success | Define phase-specific transitions and retained effects; no assertion of generic atomic lowering |
| DeFiFormal registry/capability models versus owner consent and permissionless deployment | Reuse semantic obligations, not its trusted template-admission model; grants do not substitute for signatures/consent |
| DeFiFormal net effects versus gross spending and persistent claims | Add explicit gross-to-net refinement and a separate liability store; claims roadmap is planned |
| Opposite price orientation | Normalize quote-per-base versus base-per-quote explicitly, with exact reciprocal/domain, scaling, rounding and range obligations |
| Intent expiry versus late settlement/recovery | Separate authority kinds and lifetimes; preserve established outcomes and duties |
| Local 128-ID caps and inaccessible ordinary closure reserve | Specify finite episodes or authenticated successor state; define reserve-consuming recovery or viable admission; PCD compression does not solve mutable-state growth |
| Multiple P/C/K roadmaps | One dependency graph and traceability matrix; preserve identifiers as requirement lineage, not competing delivery queues |
| “ZK+MPC+TEE” safety shorthand | Replace with property-specific assumptions, common statement and explicit conjunction/alternative rules |

## One roadmap with measurable exits

The sequence below consolidates delivery; it does not waive any existing objective MC/SP predicate. Keep one implementation slice active. Read-only source questions and small feasibility work may proceed independently under existing authority. Internal operational blockers must be reconciled before their dependent execution, without becoming public developer prerequisites.

| Step | Existing lineage | Deliverable and exit evidence |
|---|---|---|
| 0: semantic and target freeze | P0, C0, K0; MC01/SP01 semantics and target work | One Core/stage/intent/effect contract, exact target tuple, scoped history-mechanism matrix and adapter assumptions. Map every required behavior to a construct, relation and future witness. No unclassified competing Core or contradictory public admission rule |
| 1: certified basis and minimal native path | P1/P2, K1; MC01/03/04, existing native-feasibility objectives | Smallest supported new program beyond fixed fixtures; enforce arithmetic, consent, effects and mandatory conditions in pinned ZKIRv3. Record native cost, actual verifier behavior and negative controls. Separate compiler proof obligations from successful executions. Stop incompatible recursion adapter honestly; do not use a host verification bit |
| 2: staged vertical slice | P2/P4, C1/C2, K0/K2; MC02/04/05 | Two-asset conditional settlement with partial fill, recipient acceptance, document predicate and persistent residual duty. Bind source, Core, proof and every actual phase effect. Positive alternative routes and hostile mutations; actual Preview financial results plus mandatory claim enforcement required for acceptance |
| 3: concurrency, private composition and recovery | P4, C2/C3, K2; MC06 and retained SP composition/private obligations | Split/join, late-result/refund race, revocation/expiry, crash/retry and reserve exhaustion. Authenticated private-state completeness, witness handoff and observation policy; separate OS isolation and retained proof verification where required. Test state-cap boundary with unpaid duty and legitimate successor episode |
| 4: bounded optional kernel integration | K3/K4/K5, C3 | One exact-effect external adapter and selected OWS/x402 profiles using the same relation. Demonstrate two independent solvers, durable concurrent reservations, paid-result reconciliation, signer-compromise assumptions and failure to downgrade evidence. External chain observations and Midnight effects retained separately |
| 5: libraries, conformance and public release | P6/P7, C4; MC07/08 and corresponding SP conformance/release objectives | Composable libraries cover all retained ACTUS/DeFi rows with separate semantics/native/local/Preview denominators. New independent program, clean install, no reviewer metadata, actual proof and own-wallet submission. Full proof/history/privacy/conformance gaps remain release blockers |

P3 trust/obligation reports accompany Steps 0–2; exact static checks follow their supported encoding. P5 typed synthesis comes only after useful exact obligations and replayed counterexamples. Kernel breadth must not delay the general language proof path. Library research can expose primitive gaps early; it cannot substitute for the end-to-end slice.

The decisive slice uses 10 A for at least 20 B, maximum fee 1 A inside total debit 11 A, exact recipient and signed domains. Two independent solvers choose distinct permitted routes. One stage leaves a funded conditional duty; a delayed external outcome races recovery after initiation expiry. Reject extra fee, changed recipient/asset, fabricated genesis, wrong verifier, leaked private input, hidden reservation, duplicated entitlement and erased liability individually. Include a valid late success and an authorized still-controlled refund. This provides non-vacuity and discriminating tests; it is specified-only and not a universal theorem.

## EARS proposals and MPLR mapping

These refine existing IDs; no new register numbers are allocated.

| Proposed EARS requirement | MPLR mapping | Essential evidence |
|---|---|---|
| When a developer submits a supported program, the public pipeline shall decide validity without reviewer, provider or solver membership. | 015,016 | Clean new program and direct independent submission |
| When a candidate fills signed holes, acceptance shall preserve every applicable policy and introduced effect, disclosure and duty. | 018,019,023,031,035 | Two valid routes; single-property hostile mutations |
| When a stage uses delegated authority, acceptance shall establish current scope and joint consumption; any increase shall require authorized amendment. | 008,011,028,031,032 | Concurrent children, revocation ordering and callback laundering tests |
| When external signing is requested, the enforcing signer or destination shall bind its exact bytes and decoded effects to the verified stage statement. | 014,022,026,030,031 | Proof-for-X/signature-for-Y rejection; unsupported decoding rejection |
| When proof, attestation or federation policy changes, acceptance shall enforce signed evolution rules and preserve duties, recovery and replay state. | 028,030 | Epoch change, rollback and weaker-fallback controls |
| While a leg is unresolved, the workflow shall retain reservations and duties; timeout alone shall not authorize incompatible refund and settlement. | 001–007,010,017,024 | Late-success/refund race under declared finality assumptions |
| When a stage incurs retained phase effects, its receipt shall account for each fee, authority consumption and residual duty under the signed policy. | 009,012,014,017,018 | Guaranteed/fallible failure controls against actual target |
| When service payment or retrieval is retried, acceptance shall preserve logical request identity and distinguish payment, result availability and recipient delivery. | 032–034 | Crash/failover, replay and already-paid reconciliation |
| When history or private completeness is relied upon, acceptance shall establish legitimate origins, compatible parents, resource uniqueness and the authenticated complete domain. | 011,013,027,029 | Forged genesis, branch reuse, hidden reservation and witness-handoff tests |
| When optimization, lowering or netting changes representation, correspondence shall preserve exact values, rejection, authority, gross economics, effects and declared cost. | 014,017,020–023,025,026 | Native adversarial witnesses, primitive preconditions and gross-to-net controls |

Open theoretical work is concentrated, not solved by terminology: an effect/resource calculus separating affine rights from persistent duties; trace refinement with phase failures and external knowledge; semantic delegation attenuation and policy intersection; interference/completeness for private reservations; and native history composition with target-specific soundness. These need explicit assumptions and usable positive witnesses.

## Alternatives and limits

A single managed router could produce narrower operational evidence sooner, but cannot satisfy the public-language contract. A universally recursive multichain proof system would centralize history representation but currently assumes more native feasibility and external correspondence than the packet establishes. A wholly ledger-local implementation with no portable certificates reduces early cost but cannot silently retire private split/join or imported-history obligations. The proposed hybrid retains the smallest credible first path while keeping those unmet requirements visible.

The packet does not establish a deployed Federated DeFi Kernel, current native recursion support, full arbitrary-program compiler correctness, complete foreign effect decoding, hardware security, cryptographic federation robustness or an end-to-end privacy/liveness theorem. No fresh web acquisition, source audit of all cited external repositories, build or experiment was performed here. Therefore this report is an evidence-grounded architecture proposal, not a decision-grade implementation or release acceptance result.
