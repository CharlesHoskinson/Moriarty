# ACTUS, DeFi Kernel and proof-carrying DSL — new design cycle

Date: 2026-09-06. Status: S2 design with the first developer mock authorized by the user’s “begin”.
Authority: [user reset](../../../raw/assignments/moriarty-target-first-reset-2026-09-06.md).
Read [footguns](../../FOOTGUNS.md), [postmortem](../../postmortems/2026-09-06-moriarty-verification-detour.md)
and [PCD research](../../research/2026-09-06-pcd-bounded-dsl.md) first.

## Current execution amendment

The [intents report](../../research/2026-09-06-intents-report-integration.md)
now refines R2 as exact-plan authorization and adds R2b outcome IntentIR,
authority/PlanIR/receipt separation, gross spend versus net goals, replay and
residual obligations. Keep ACTUS and DeFi implementation coverage, mandatory
PCD and native Midnight priority. The report's IKL name, other-backend-first
calendar and optional-only proof recommendation do not replace this program.

The design study/proposal is complete at its documented scope, and the user
approved starting the mock. The supplied PCD report and Midnight recursion
clarification now revise implementation order through
[R0–R6](../../research/2026-09-06-pcd-report-integration.md#revised-sprint-sequence-and-exit-evidence).
This amendment controls over the original stage table below: finish the honest
mock, define typed mandatory claims and the shared loan/swap semantic slice,
prioritize Midnight-native IVC then its ledger adapter, and test private witness
handoff plus bounded split/join histories. Required PCD remains fail closed;
optional acceleration never exempts required evidence. No A4/A5 loop resumes.

## Intended product and current boundary

Moriarty is to be a Turing-incomplete financial contract and transaction-intent
language, with unified semantics for ACTUS and the DeFi Kernel. Developers
should be able to describe contracts, inspect deterministic behavior, establish
named properties under explicit finite bounds, and produce transactions whose
acceptance requires a proof of compliance with the contract and predecessor
history. Compact/ZKIR remains the original deployment target to evaluate.

This cycle produces a target study, a semantic proposal and a developer mock
design. It does not presume Candidate A, K, Quint or any proving system is the
selected language architecture. Existing tools and experiments are candidates
for reuse after the requirements identify their role. No A4/A5 continuation is
part of this plan.

## Study already completed and remaining coverage

Initial read-only sampling checked the pinned ACTUS fixtures, dictionary and
technical specification, DeFiFormal models and the 72-row crosswalk. This is
semantic reconnaissance, not a complete target study or executed compatibility
test. The full assignment still requires all 18 executable ACTUS types and all
277 fixtures (276 contract fixtures plus one analysis-date fixture), two
independent semantics, and comparison of every present ordered result field.
No fixture may be excluded to fit a candidate Core.

| Inspected example | Requirement exposed |
| --- | --- |
| ACTUS `lam01` | Principal amortization, accrued interest, observed rate resets and same-time PR/IP/RR ordering. |
| `ann01`, including the separate analysis-date file | Annuity calculation, accrued intermediate state, lossless decimal/date import and source-qualified fixture identity. |
| `lax01` | Bounded array schedules, day-count, end-of-month and business-day conventions. |
| `option02` | Exercise and settlement are different transitions; exercise state matters even when immediate payoff is zero. |
| `swaps01`, `swppv01` | Child identities, merged event ordering, role signs, fixed/floating legs and observations. |
| `guarantee02` | An external referenced-contract event can trigger exercise and settlement; its authenticity needs an explicit capability. |
| DeFiFormal Uniswap v2 | Integer square root, fee and LP-share rounding, locked minimum liquidity and reserve bounds. |
| Morpho Blue | Virtual assets/shares, directional conversions, collateral health, liquidation incentives and who bears bad debt. |
| MetaMorpho | Distinct owner/curator/allocator authority, caps and bounded reallocation. |
| Polymarket | Split/merge, partial-fill state, outcome authority, resolution and redemption. |

The inspected source inventory and full pins are preserved in the
[target observation](../../../evidence/moriarty-reset-2026-09-06/target-study.json).
The ACTUS technical specification's business-day wording conflicts with the
dictionary; [the contradiction register](../../../wiki/contradictions.md)
records a provisional disposition. The next study must resolve the relevant
conventions before claiming compatibility. Model omissions in DeFiFormal remain
explicit boundaries; a taxonomy row is not a proved implementation.

## Sequence and reviewable outputs

| Stage | Work and output | Exit condition |
| --- | --- | --- |
| 1. Study ACTUS and the kernel | Extend the initial samples into one requirements/coverage matrix over all 18 executable types, 32 taxonomy dispositions and 72 DeFi rows. Read schedule generation, payoff/state functions, authority, arithmetic and capability boundaries. Record missing evidence and source contradictions. | Every required target has a disposition and named semantic requirements; detailed examples span loans, contingent contracts, composition, AMM, lending, mandates and conditional tokens. No constructor or backend selected merely from category names. |
| 2. Compare shared semantics | Propose 2–3 concrete alternatives: bounded event/state transitions; finite contract combinators with shared financial libraries; a clearly delimited hybrid. Express the same ACTUS repayment and DeFi examples in each, including adverse behavior. Compare arithmetic, composition, lifecycle bounds and effect projection. | A reviewer can see common primitives, package boundaries, missing expressivity and tradeoffs. The recommended alternative follows examples and coverage, with unresolved choices listed. |
| 3. Specify assurance and PCD | Define types, state, observations, events, transition/effect rules, initial states, resource bounds, contract properties and the local compliance relation. Specify genesis, predecessor joins/splits, identity, authority and ledger consumption. Compare sequential IVC, bounded DAG PCD and a target-native route. | Every advertised guarantee has a predicate, quantification, assumptions, bound and proposed proof method. Explain how contract certificates, execution proofs and history proofs connect. No assumed backend interoperability. |
| 4. Design the developer mock | Produce a reviewable interface/wire-schema proposal and a small interactive mock after the semantic proposal is reviewed. Use actual ACTUS fixtures and DeFi examples, with visible diagnostics, counterexamples and proof status. | A developer can walk through author/import, inspect, simulate, prepare/sign and verify/submit preview. Mock evidence is unmistakable and cannot become real acceptance. |
| 5. Propose the first implementation slice | Present the semantic choice and mock together. Specify one bounded experiment for the actual target verifier, with a real positive proof and altered-proof/public-input rejection. Identify compiler correspondence obligations and the smallest target-spanning implementation slice. | User review can approve a concrete design and bounded experiment. Larger implementation and verification work follows that decision. |

These stages are dependency gates, not an open-ended goal loop. Stages 1–3
must not expand into exhaustive native verification of a provisional candidate.
Record targeted source searches and stop once the decision has sufficient
evidence or a clearly identified gap. Before stage 5 execution, specify command,
input size, memory/runtime ceiling, expected evidence and cumulative effort
limit. A failure stops that experiment for assessment; it does not automatically
authorize a larger heap or another campaign.

## Proposed shared semantic boundary

**Design hypothesis to test:** use a small deterministic transition Core with
typed, versioned financial packages. Candidate shared facilities include typed
amounts/units, bounded rounded arithmetic, ordered events, finite collections,
state, observations, authority and effects. Core membership requires an example
showing why a library definition is insufficient.

ACTUS amortization, day-count, calendars, rate resets, exercise and payoff rules
belong in packages over that machinery where possible. DeFi pricing curves,
share conversion, liquidation, mandates and conditional positions need equally
precise packages. Do not hide a financial rule in an unconstrained witness or
create a special compiler path per product. Composition must preserve child
identity, ordering, asset units and authority across packages.

External prices, referenced contract events, reserves/custody, consensus and
cross-domain finality are explicit capabilities. The proof checks the declared
attestation and policy; it does not establish the external fact's truth by
itself. Adopt numeric precision and lifecycle bounds from requirements and
proof/backend feasibility, not from the largest fixture or a model's temporary
integer ceiling.

## Developer mock proposal

The following is a proposed interaction contract, not final DSL syntax or a
working SDK. Begin with ACTUS `lam01` and a DeFi swap; add an option with deferred
settlement and a lending/liquidation example to challenge the shared design.

```text
Contract workspace
  Source: ACTUS fixture / typed package / developer-authored agreement
  Terms and units | Observations and authority | Bounds and assumptions
  Ordered events: calculation date, payment date, child identity, state changes

Analysis
  Named property -> proved / counterexample / unknown / unavailable
  Scope: program version, input domains, lifetime horizon, method
  Counterexample -> inspect the exact events, balances and policy failure

Transaction preview
  Selected predecessor states -> action -> resulting states and asset effects
  Intent digest -> participant signatures -> proof of authorized transition
  Proof status -> statement verification -> live ledger checks -> submission
```

The mock should let developers change dates, rate observations, roles and
rounding-sensitive values and see their effect. ACTUS comparison displays all
ordered fields, including notional and accrued/exercise state. The DeFi view
shows fees, LP shares, collateral health or loss allocation as appropriate.

Proposed API operations are `import`, `elaborate`, `analyze`, `simulate`,
`prepareIntent`, `signIntent`, `proveTransition`, `verifyBundle` and `submit`.
Names and signatures remain open. Preserve the existing assignment's explicit
`verified / rejected / unavailable` verification outcomes. A mock transport
uses a separate simulated-evidence type; it must never return a real verification
certificate or enable real submission. An unavailable prover is not success.

The signed intent commits to the authorized statement and relevant policy,
excluding later proof bytes to avoid a signature/proof cycle. A final proof
must bind and check authorization. Any pre-signing simulation or certificate is
a separately named preview. Decide canonical encoding, domain separation and
version binding before turning this ordering into a wire protocol.

Required mock scenarios: valid repayment; changed event order; missing/stale
observation; overflow or precision mismatch; incorrect authority; wrong asset
effect; missing/altered proof; wrong semantics/network; stale predecessor;
two individually valid branches attempting duplicate consumption; and a
multi-parent join. Show the financial-rule failure separately from the live
ledger conflict. Include cancellation, partial fills and recovery where the
selected DeFi example requires them.

## Acceptance checklist for the new cycle

- [x] Postmortem and standing footguns written; current roadmap reset.
- [x] Primary PCD research and initial ACTUS/DeFi source sampling completed.
- [x] Account for all target rows and record source-discrepancy dispositions;
  package algorithms and conformance gaps remain explicit.
- [x] Compare semantic alternatives on the same financial examples.
- [x] Propose a closed finite-bound profile and named correctness properties;
  numeric profile values still require target/backend evidence.
- [x] Propose PCD relation, topology, genesis and ledger acceptance boundary.
- [x] User approved beginning the developer mock from the semantic/interface proposal; unresolved numeric and proof choices remain open.
- [x] Build and exercise the first local developer mock, with simulated evidence clearly typed; complete semantic scenario checking remains R2+.
- [ ] Review a bounded real-backend feasibility experiment and implementation slice.

The approved design sprint produced the
[review package](../../../deliverables/moriarty-design-sprint-2026-09-06/README.md).
Checked items denote study/design outputs, not a completed
DSL or cryptographic backend. The mock is authorized and tracked in its implementation plan. Real proof
backend, completed ACTUS algorithms and conformance remain open under R2–R5.
