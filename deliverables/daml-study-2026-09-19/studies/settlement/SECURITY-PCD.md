# Settlement security should shape the proof-carrying language

Research proposal, not implemented or proved behavior. This note follows the user's priority that bounded/Turing-incomplete computation and proof-carrying data should reduce attack paths. The concrete pressure cases come from [DS01–DS18](claims.json), especially exact authorization matching, explicit allocation consumption, committed funding, iterated settlement and cancellation of late arrivals.

## Attacker and trusted basis

Assume an attacker controls solver plans, supplied contract IDs, transfer legs and side labels, duplicate or omitted allocations, chosen recipients, claimed status, evidence order, callback correlation, requested fees, iteration parameters and timing of submissions. They may submit arbitrary proof witnesses, reuse a predecessor proof, or propose a program that is well-typed but violates the user's authorized economic policy. A malicious token implementation can satisfy an interface shape without implementing its promised effects. An adversarial sender may race cancel, settle and withdraw.

The proof relation must bind the signed canonical program/semantics/profile, intention, workflow/stage occurrence, predecessor commitment, current relevant state, asset identity and authority, evidence statement and verification policy, complete effects and cumulative bounds. Authentication requires unforgeability and canonical encoding. Proof acceptance assumes cryptographic soundness, correct verification keys and a correct verifier implementation. Actual spend uniqueness requires the target ledger's state/nullifier/consumption rules; a recursive proof alone does not stop two valid successor branches from being submitted against one predecessor.

External document truth, bank performance, token-admin honesty, oracle freshness, independent-chain finality, private witness availability and execution progress remain named assumptions or separately verified protocols. Neither Daml authorization nor PCD converts those assumptions into theorems. The library/ledger pin and compiler-to-ZKIRv3 correspondence are part of the trusted basis until discharged.

## Induction obligation: local checks versus composed history

Let a stage record identify its signed policy, economic state, outstanding duties and cumulative resource use. This is a semantic proposal, not adopted notation or a claim that one universal state record fits every application.

The base proof must establish valid origination: funded rights are justified, liability creation is authorized, initial evidence commitments are bound, and no predecessor duty is silently omitted. A transition proof must validate predecessor proofs/commitments, enforce the current stage's authority and conditions, and relate every input/output effect to the signed policy. A history proof may then establish the invariant by induction for every accepted prefix, provided the ledger enforces legal consumption and the compiler/constraint relation is sound.

Maintain separate invariant families:

- **Assets:** for each exact asset/custody domain, justified inputs, outputs, fees and authorized supply changes balance under the declared phase behavior. Netting must refine the gross authorized transfer graph, not erase it. [DS04, DS07, DS18]
- **Authority:** each effect uses a right authorized for that stage; delegated consent cannot amplify into arbitrary sender authority. Unused affine permissions may be discarded only where policy allows. Liability or recovery duty is not an affine permission that may simply be dropped. [DS02, DS05, DS14]
- **Duties:** partial execution, cancellation and expiry preserve every unpaid, recoverable or unresolved obligation until authenticated discharge. Late-created allocations require persistent cancellation state or another adequate protocol. [DS09, DS17]
- **Evidence:** every consumed signature, document predicate or proof refers to the right occurrence and terms; freshness/revocation use the signed validity cut. Evidence for a local transition is not evidence of remote delivery. [DS03, DS10, DS13, DS16]
- **Bounds:** iteration and retry cannot reset cumulative fees, debit limits, quantities, disclosure budgets or declared work. Sound integer/fixed-point arithmetic and explicit rounding prevent split-fill exploits. [DS07, DS09]

An inductive proof of these invariants narrows attacks that rely on individually plausible but globally inconsistent steps. It does not prove that the chosen specification is economically desirable or matches an unformalized wish. Inspectable signed intention and non-vacuity evidence remain necessary.

## Required negative and non-vacuity witnesses

| Attack witness | Required rejection or retained state | Positive witness needed to avoid vacuous safety |
|---|---|---|
| Omit one required DvP leg while balancing the remaining ledger | Reject missing complete effect/frame or keep workflow pending | Both required legs settle with valid allocations |
| Duplicate a leg authorization under a different contract ID | Reject duplicated semantic authorization, not only duplicate bytes | Two different legitimately authorized legs sharing an account succeed |
| Fork the same predecessor into two independently valid spends | Ledger rejects double consumption; proof binds intended input rights | Disjoint resources can progress concurrently |
| Interface body omits consuming allocation or returns Pending | No terminal delivery claim; reject an unmet implementation contract | Certified completing implementation succeeds; explicitly pending workflow remains usable |
| Split fills to reset fee/debit budget or introduce extra legs | Reject cumulative violation and retain unfilled duties | Two permitted fills satisfy aggregate bounds and final discharge |
| Cancel parent, then late allocation appears | Authenticated cancellation state rejects or recovers late value | Ordinary cancellation and valid late recovery succeed |
| Withdraw and settle compete | At most one valid consumption of the current funded right | Either authorized winner produces its specified outcome |
| Timeout despite authenticated late success | No timeout-only refund; account for known delivery | Proven nonexecution permits an authorized refund |
| Committed allocation with no unilateral expiry | Do not promise a unilateral refund or unconditional progress | Disclosed commitment settles/cancels under its required parties |
| Settlement field merely records preferred time | No false deadline theorem | Explicit guard permits before-boundary execution and rejects the forbidden side |
| Net-zero gross transfers add unauthorized fees/liabilities | Reject gross-policy violation despite net conservation | Authorized netting reduces funding while preserving obligations |
| Malicious prover uses invalid but satisfying target witness | Actual target constraints reject it | Source-valid execution produces a target witness that verifies |

These are proposed conformance/attack cases, not executed tests. Splice includes source tests for some duplicate, authority and deadline cases; the study did not run them. Finance's off-ledger acknowledgement case supplies a particularly clear negative witness against claiming external finality from a valid local transaction.

## Expressiveness restrictions that need explicit justification

Turing incompleteness supports bounded evaluation but is insufficient on its own. A terminating program may still overflow, leak data, spend twice across independent histories, charge unauthorized fees or verify the wrong predicate. Bound program size, transition work, data structures, transfer/evidence count, arithmetic domains, witness size and verifier cost. Use a decidable supported transition/evidence checking relation; general theorem discovery and liveness need not be decidable just because programs terminate.

Separate finite per-stage computation from potentially long-lived workflows. Either impose signed lifecycle bounds or carry a well-defined monotonically accumulated resource account and explicit replenishment authority. Recursive proof compression does not erase historical obligations or pay for infinite work. An unbounded business lifetime is not permission for unbounded computation in one proof step.

Candidate restrictions include no unchecked native side effects; typed external observations with named trust; no arbitrary prover-selected verification key; total checked arithmetic; bounded collection operations; explicit atomic-region membership; and mandatory semantic contracts for primitives and token adapters. These are research choices to test against all-Midnight-DeFi expressivity, not an excuse to replace the language with an approved template list. Certified jets may optimize a reference meaning only after proving relevant values, effects, rejection and costs correspond.

Before rejecting useful expressivity, identify the attack it permits and whether a type/effect rule, capability scope, bound or proof obligation suffices. Application-selected counterparties and attesters are legitimate signed policy. Project reviewer identities and service credentials are not public language validity conditions.

## What the proposed security story must not claim

PCD can prove lineage properties only for the exact relation, authenticated predecessor and available witnesses that it verifies. It does not by itself provide cross-chain atomicity, external truth, finality, availability, fair prices, liquidity, censorship resistance or eventual settlement. Canton reassignment is preparatory and non-atomic; independent chains need their own protocols. Midnight guaranteed/fallible phases must be included in the semantic relation, including retained fees and effects. Source atomicity cannot silently erase target outcomes.

The next research obligations are the unnumbered `pcd-settlement-induction`, `gross-to-net-refinement`, `implementation-semantic-contract` and `cancellation-residual-marker` proposals in [MPLR-candidates.json](MPLR-candidates.json). No compiler, proof system or mandatory acceptance path is certified by this note.
