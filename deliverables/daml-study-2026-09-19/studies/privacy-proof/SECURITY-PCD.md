# From the security model to a bounded language and PCD obligations

Research proposal, 2026-09-19. The judgments below are requirements to investigate, not completed proofs, selected cryptographic constructions, or implemented ZKIRv3 artifacts. Daml evidence is indexed as DP01–DP16 in [claims.json](claims.json). The controlling Moriarty product contract requires open developer access, signed formal intention, partial and conditional transactions, a certified basis, and actual Midnight ZKIRv3 execution.

## Security objective and attacker

For a supported profile, acceptance should imply that the actual ledger outcome is one of the outcomes authorized by the signed intention and the program semantics, with all retained effects and residual duties represented. This is a safety claim. Availability of funds, secret witnesses, relay services, attestors, or inclusion is a separate progress premise. Truth of an external document is not obtained by proving that its issuer signed it.

The attacker can write supported programs, choose witness values, modify the prover, supply old or selectively disclosed contracts, reorder asynchronous results, select allowed implementation versions, compose valid proofs with misleading parent identities, and attempt concurrent double spending. The attacker may also observe public ledger statements, costs, error results, timing, routing/shape, or a hosting service, depending on the explicitly declared threat model.

Daml motivates this separation. Disclosure does not grant action authority (DP03); a locally valid view need not establish the whole ledger's validity against colluding signatories (DP07); key absence can mean invisibility (DP11); a stable interface can invoke changed behavior (DP09). None of those failures is fixed merely by replacing a runtime check with a proof of the same incomplete proposition.

## Proposed state and statement

Let the semantic profile `P` fix syntax, typing, transitions, numeric and resource semantics, and allowed external observations. Let `B` identify the source program/property, compiler route, certified primitive versions, target artifact, circuit/relation and verification key, ZKIRv3 version, and ledger/phase profile. Let `I` denote the canonical signed intention including an explicit amendment relation. Let `D` be the execution/replay domain.

A history state `S` separates assets/ownership, spending authority, reservations, liabilities, workflow stages and residual duties. These are distinct components: a proof artifact is reusable mathematical evidence, while a spendable capability is a ledger resource that must be consumed at most once. A signature is evidence of consent only for its bound scope and current lifecycle policy.

A proposed public statement `x` binds, directly or by binding commitments, `(P,B,I,D)`, predecessor statements and state roots, consumed resource identities/nullifiers, stage identity, accepted observations, resulting state/effects, residual duties, and claimed cumulative bounds. The disclosure profile decides which fields are public. Every hidden committed component must be connected to the checked relation; opacity does not excuse an unconstrained witness.

The construction must define its DAG/sequence structure and well-founded induction. It must not accept a cycle or a parent selected under an unrelated verification relation as evidence of provenance. Any upgrade relation used in parent/child composition is part of the bound semantics, not merely a compatible serialized type.

## Required judgments

These schematic judgments describe obligations independently of proof-system syntax.

**J0 — legitimate origin.** `Base(P,B,I,D,S0,E0)` checks the initial invariant, signed authority, and legitimate origin of state/resources under declared ledger or external evidence `E0`. A prover cannot choose an arbitrary funded or approved state and call it a base case. If a source program intentionally admits initial creation of liabilities or tokens, the relevant issuer/recipient authority and supply rules must be explicit. Initial state validity is stronger than validity of its field types.

**J1 — scoped authority and evidence.** `Auth(I,D,stage,parent_context,action,S)` checks the exact authority available at that stage and nested action, expiry/revocation and replay rules, plus consent to newly introduced duties. Read access and authenticated evidence have separate judgments. A disclosed contract can be authentic and active yet still provide no authority to consume it. Daml's child authorization context using signatories and controllers instead of all ambient callers is a concrete design to compare, not a theorem that every Moriarty application must use precisely that rule. [DP03, DP06, paper p9]

**J2 — semantic transition.** `Step(P,program,I,S,observation,S',effects,residual,bounds,outcome)` checks a permitted local transition, including failures and partial outcomes. Every mandatory condition must dominate acceptance: computing a Boolean and failing to assert it does not discharge the condition. Every required consequence belongs to the transition tree/effect frame; checking isolated nodes cannot detect a missing settlement leg. [DP06]

**J3 — state completeness and absence.** `Frame(root,domain,resources,claims)` authenticates the relevant state domain and its update/lookup policy. A private witness that contains no conflicting liability is not proof that none exists. An absence claim requires a sound nonmembership/completeness argument against the authenticated state authority for the declared domain, or an explicit stronger external assumption. Where global absence is unavailable, the language must expose unknown/conditional results. A proof over a party's projection cannot silently quantify over all hidden contracts. [DP07, DP11]

**J4 — recursive composition.** `Compose(x,parents,proofs)` verifies parent proofs under the intended relation/profile, opens or relates their state/effect commitments, and binds their outputs to the actual inputs of J2. At branches/joins it accounts for each consumed resource, every branch's effects, and all outstanding duties. Ledger-enforced nullifier or resource uniqueness remains necessary: two individually valid histories may attempt the same spend. PCD cannot create consensus finality by recursive verification alone.

**J5 — target correspondence and accepted-witness soundness.** For all candidate target witnesses `w`, not merely the honest generator's output, satisfaction of the bound target relation must imply a source-permitted outcome with matching effects, rejection/failure policy and bounds. Source-valid transitions should also have an appropriate target witness under stated representability and resource premises. The actual Midnight ledger must check the intended relation and produce effects corresponding to the constrained effects. Program hash, key hash, compiler version, public-input encoding, domain and phase layout must not be interchangeable independently.

This separates constraint soundness from prover correctness, cryptographic proof-system soundness, compiler correctness, and ledger integration. A forged witness can exploit a missing range/equality/shape constraint even if every honest evaluator trace passes. A sound proof system only certifies the relation actually encoded.

**J6 — evolution.** `Evolve(old_binding,new_binding,S,S',policy,consent)` proves the specifically authorized behavioral relation or checks fresh consent. Preserving a schema, stakeholder set, package name or interface signature is insufficient. Preserve or explicitly amend economic outcomes, evidence policy, disclosed information, costs, liabilities, recovery, and authority scope. Data migrations must preserve old obligations even if no new transaction uses the old implementation. SCU's documented 1-to-2 interface behavior is a minimal counterexample to identifying interface stability with semantic stability. [DP08–DP10]

**J7 — observation security.** `ObsSec(profile,program,proof_system,target)` is a relational property comparing executions under the chosen allowed leakage/declassification relation. A single execution's local compliance check does not establish noninterference by itself. Include outputs, public commitments and equality links, recipients, branch/continuation structure, fees/resource consumption, errors and external messages where in scope. Zero knowledge concerns the proof transcript relative to its public statement; it does not hide secrets already encoded in that statement or transferred to an authorized prover/participant. Canton's projection/encryption guarantees and Midnight's ZK mechanism need distinct arguments. [DP01–DP05, DP13]

## What boundedness should and should not buy

Daml-LF is intentionally Turing complete and suggests runtime cost management; its formal authorization semantics do not depend on totality (paper p9). Moriarty's certified profile can instead require total definitions or explicit bounded iteration and make bounds part of the accepted statement. A language restriction is useful only when its consequences are stated: termination of local evaluation, bounded circuit generation/execution, analyzable resource use, and exclusion of hidden unbounded dynamic calls.

Totality alone is insufficient. A finite function can overflow a poorly modeled integer, pay the wrong person, leak the entire witness, accept a forged document issuer, or perform an expensive computation beyond practical limits. A terminating step can also produce a continuation forever, so local termination does not prove global workflow completion. Finite state spaces can remain too large for automatic verification. PCD preserves a specified invariant across unbounded history only if the base and step/recursive obligations are sound; it does not infer the right invariant from a natural-language wish.

A defensible initial restriction set is an explicit supported basis with checked numeric ranges/units, no hidden I/O or ambient authority, finite per-stage computation, bounded dynamic dispatch over certified semantics, and explicit staged continuations. Off-chain search may remain unrestricted because candidate plans are checked by the same relation. Restrictions should be tested against usable DeFi/TradFi workflows rather than become a fixed application catalog or maintainer permission gate.

## Attacks and decisive counterexamples

| Attack | Minimal counterexample | Required judgment |
|---|---|---|
| Private state omission | Prover omits an existing hidden reservation and proves balance sufficiency over the remainder | J3 plus asset/reservation accounting in J2 |
| Disclosed evidence as spend authority | Receiver gets a valid document blob and uses its issuer identity as permission to debit issuer assets | J1 separates evidence provenance and acting authority |
| Summary privacy illusion | Audit returns one bit but the auditor sees the secret input or parent-visible fetch subtree | J7 and explicit disclosure effects |
| Upgrade drift | Same interface and old contract invoke a new method returning 2 instead of signed expectation 1 | J6 and bound implementation in J5 |
| Recursive proof substitution | Parent proof verifies for a different program or weaker predicate with the same-looking state hash | J4 binds relation, program, domain and state encoding |
| Forged origin | Prover begins from “all parties accepted” with no consent provenance | J0 |
| Replay/fork | Two proofs share the same valid funding parent and each spend its output | J4 plus ledger uniqueness; recursion alone does not exclude |
| Failed condition ignored | Witness correctly computes `valid_document=false` but acceptance never requires true | J2/J5 require the condition to constrain acceptance |
| Timeout refund and late delivery | A remote receipt is delayed; refund succeeds and a later remote success is ignored | J2 residual duties, J4 history join, ledger/external finality premises |
| Cost side channel | Both branches terminate, but gas or proof/circuit size reveals a confidential predicate | J7 and boundedness profile; totality alone does not help |

These are reasoned test cases; none has been executed against a constructed Moriarty implementation.

## Conditional settlement and certified primitives

Each stage should expose known success, known failure, pending/unknown external outcome, and authorized compensation as appropriate to the application. Continuations bind previous state, allowed effects, callback identity, evidence scope, per-stage authority, expiry and residual duties. A failed fallible Midnight phase can leave guaranteed effects; a source model cannot erase them by declaring the entire business workflow atomic. A later refund is a new transition with its own accounting, not a proof that prior delivery never happened.

For a certified jet, require a relation from its optimized implementation and constraint fragment to the reference operation for all admitted inputs/witnesses. Bind preconditions, definedness/range, result, rejection/failure behavior, complete effects and resource semantics. Composition proves call-site preconditions and frames. If an optimization changes an observable cost that the policy treats as meaningful, the profile must acknowledge it. Version or hash equality is neither sufficient certification nor developer approval.

The desired scoped history theorem is: a legitimate base, sound authority/evidence/step/frame judgments, sound recursive verification and target correspondence, and the stated ledger/external assumptions imply that every accepted history prefix has the specified invariant and retains exactly its modeled obligations. A separate relational theorem establishes the promised confidentiality profile. Separate progress theorems require availability and fairness assumptions. These are the claims the research should build, not guarantees supplied by a signature, compiler typecheck, Daml runtime, or PCD label alone.
