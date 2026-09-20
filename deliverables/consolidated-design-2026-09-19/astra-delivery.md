# Independent proposal: one delivery sequence for the Moriarty language

Author role: financial programming languages, reusable libraries and delivery planning. Requested reviewer: GPT-6 Astra, medium. Date: 2026-09-19. Advisory recommendation, not implementation approval, native feasibility evidence or a release decision.

## Recommendation

Build one permissionless language whose versioned acceptance relation connects source programs, signed intention, complete typed effects, bounded stages and compliant history to actual Midnight ZKIRv3 execution. Financial libraries exercise this relation; the Federated DeFi Kernel proposes and coordinates work across external systems. Neither a library catalog nor a federation admits developers. Use the native Halo2-derived PLONK/KZG stack; introduce no Lean dependency.

Replace P0–P7, C0–C4 and K0–K5 as execution queues with the single phase sequence below. Retain their requirement IDs and historical evidence as aliases into that sequence. Keep MC01–MC08 and SP01–SP12 as acceptance contracts and audit views, with their objective predicates intact. A new phase name cannot close an old gate. In particular, I2/MC02 integration remains uncertified until MC05; a new conditional settlement demonstration does not replace required loan/swap or complete conformance evidence.

This is a delivery recommendation inferred from the supplied evidence. It does not assert that the proposed language or proof path exists today.

## Evidence and present limits

I read the complete 3,027-line frozen evidence packet, including all MPLR-001..035 notes, and the three current implementation plans, sprint README, requirement indexes, the kernel Pel template and the experimental language package scripts. I loaded the repository development skill and ran only the read-only status command. No builds, tests, proofs, network queries or deployments were performed. Referenced original repositories and web sources were not independently reacquired or freshly verified here.

Observed status: capability `SP01.6 loan-swap-subset`; completed recorded stages `atomic-prepare`, `atomic-accept`, `rp01-mc02`. Dispatch is blocked by stale bindings to `openspec/sprints/sp01-financial-contract-and-execution-admission.md`, missing current accounting, unavailable resource live state and unresolved/unverified operational history. There are no pending transaction notifications. This blocks dependent maintainer dispatch, not this advisory work or an independent developer's right to use supported public tooling.

The packet reports useful scoped evidence: source/5 local lifecycle preparation; finite K comparisons; finalized scoped loan and swap effects. It also explicitly reports missing general source-to-ledger correspondence, mandatory recursive history, private composition and full financial conformance. Distinct source/Core representations and a restricted legacy Compact mapper are not one established pipeline. The September 11 recursion/interface findings are historical pins, not observations of September 19 network capability. No schedule may assume that an unreleased native interface now exists.

Primary local evidence: [product contract](/home/charl/Moriarty-aeon-study/docs/MORIARTY-PRODUCT-CONTRACT.md), [whole-language review](/home/charl/research/moriarty-whole-design-2026-09-19/REVIEW.md), [MC completion contract](/home/charl/Moriarty-aeon-study/openspec/MORIARTY-COMPLETION-PROGRAM.md), [sprint contract](/home/charl/Moriarty-aeon-study/openspec/sprints/README.md), and [frozen packet](/home/charl/research/moriarty-consolidation-2026-09-19/evidence-packet.md).

User-supplied planning assumption, received 2026-09-19: Midnight will have comprehensive recursion capabilities in about six months, approximately March 2027. Plan the target architecture for full native recursion, private multi-parent composition and the original MC03/MC06 obligations on that horizon. This is a product planning assumption, not a verified release date or current capability observation. Current interfaces justify an early scoped subset; they do not authorize permanent de-scoping, replacing MC03 with ledger induction or selecting another backend. Prepare native proof statements, parent/child interfaces, hostile fixtures and cost models now; qualify actual release interfaces when available.

## Concrete boundaries

| Component | Owns | Does not establish |
|---|---|---|
| Small typed Core | Exact quantities and asset/domain identities; bounded evaluation; state/effect/resource judgments; explicit rejection; authority, obligations and evidence interfaces | Financial correctness from totality alone; truth of external observations |
| Compiler, native proof relation and verifier integration | Source/Core/constraint correspondence, arbitrary accepted-witness soundness, valid-execution completeness, complete statement binding, actual phase projection | Ledger finality from successful proof generation; native recursion from an interface name |
| Financial libraries | ACTUS schedules/payoffs; AMM/liquidity formulas; vault/share conversion; credit/default/liquidation; conditional claims; explicitly bounded financial algorithms | New unchecked effects or permission to widen a signed intention |
| Certified primitive/jet layer | Optimizations with exact preconditions, reference/host/target correspondence, rejection/effect/work preservation and separately reported circuit cost | Certification from tests, hashes, shared invariants or a reference expression alone |
| Federated DeFi Kernel | Discovery, candidate construction, external observations, durable reservations, signing coordination, service purchase and recovery under declared policies | Public deployment authority; replacing mandatory program proofs with MPC signatures or TEE attestations |
| Domain adapters | Exact signed-byte interpretation, supported effect projection, evidence/finality verification and native/external identity binding | Universal cross-chain rollback or finality; economic postconditions from matching an API |
| Public tools | Author/check/simulate, inspect signing terms and proof obligations, prove and submit with developer-owned environment | Project campaign/review requirements as public validity rules |
| DeFiFormal | Source-pinned reference models, useful theorem statements, examples and research obligations | A deployed federation, a Moriarty backend, owner-authenticated consent or automatic theorem transfer |

The semantic unit should be one `AcceptStage` relation over program/profile, authenticated policy, predecessor/state domain, candidate, typed evidence and target phase layout, producing complete effects, authority consumption, costs and residual duties. The four mandatory claims—ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance—must constrain that same accepted relation. Distinct modules may discharge them; independently named predicates with inconsistent inputs are insufficient.

Distinguish uncommitted candidates, Midnight guaranteed/fallible phase outcomes, and committed multi-transaction workflow stages. Each has different failure semantics. Use an authenticated effect frame; a convenient subset of balances cannot prove absence of hidden reservations or liabilities. Bound work per stage, fan-in, collections and a finite supported episode measure. For longer-lived services, specify authenticated rollover preserving cumulative limits, replay protection and outstanding duties; do not confuse proof compression with state compaction.

## Financial breadth without a large financial Core

Adopt a small basis selected by actual conformance needs, not an unverified historical estimate of fifteen primitives. Keep asset quantities, shares, rates, signed deltas, liabilities, consumable receipts and affine authority distinct. A library's effect contract must identify its footprint, preconditions, possible failure results, costs and obligation evolution. A module replacement needs behavioral refinement, not just a matching signature.

Exact arithmetic means mathematical integer/rational meaning with explicit bounded representation, checked intermediate widths, defined division-by-zero behavior, directed rounding and rejection. Native field arithmetic must be range-constrained to that meaning. Require overflow/carry mutations, out-of-range values and malicious witness assignments at each primitive boundary. Prove composition preconditions at call sites. Recovery work reserves remain separate from ordinary execution budget.

**Price orientation requires an explicit conversion.** The supplied DeFiFormal study says `Price(base,quote)` is quote per base; Moriarty `Price<A,B,S>` is base per quote. If reference price is positive `n/d` quote/base, the exact Moriarty ratio is `d/n` base/quote before scale conversion and the required directed rounding. Zero is noninvertible; machine encodings require checked cross-products and unit/decimal conversion. Do not invert an already rounded integer or reuse the type spelling. Bind orientation and scale in the library API, intent display and artifact identity. Exchange tests should verify both exact-output and minimum-output predicates separately.

The source pin `33b9a9550ac1ed12c83c32d15277e530741787db` provides narrow DeFiFormal arithmetic, token0 concentrated-liquidity next-price logic and stable-time sUSDS vault operations. It does not provide an implemented `DefiKernel.Claims` lifecycle. Its broader Curve/redemption/loss, shared-vault/hooks, async/cross-domain, margin and conditional-claim namespaces are planned at that pin. The vault restriction `timestamp == rho`, positive `chi`, scoped D0 deposit adapter and reported supply-wrap mismatch must remain attached to any reuse claim. Neither partial concentrated-liquidity arithmetic nor a source-preparation receipt closes a full pool or family.

The DeFiFormal kernel's net-effect accounting and tracked-net clearance cannot replace Moriarty gross debit limits or persistent liability semantics. A refund does not replenish the consumed gross budget. Restoring a vault balance does not prove repayment by the obligated debtor, clearance of fees or discharge of every claim. DeFiFormal grants also do not establish owner consent. See [library review](/home/charl/research/defiformal-moriarty-scope-2026-09-19/libraries-review.md) and [composition review](/home/charl/research/defiformal-moriarty-scope-2026-09-19/composition-review.md).

| Coverage batch | Library work | Required distinct gap/observation |
|---|---|---|
| Common financial base | Checked arithmetic, fees, scale/price conversion, typed accounting, scopes and evidence | Reusable primitive certificate plus actual target representation; no field-wrap equivalence assumption |
| ACTUS lifecycle | Calendars, year fractions, event ordering, rounding, principal/interest, capitalization, repayment/refinance and contingent events | All 277 fixtures across 18 executable types and all expected fields; 32 taxonomy dispositions and DS-01..07 remain separate; DS-03 missing primary ANN formula needs explicit derivation and independent checks |
| Swaps and liquidity | Exact-in/out, minimum output, fees, invariant changes, LP mint/burn, concentrated-liquidity steps | Complete price/rounding/branch behavior; source-pinned tick crossing and exhaustion; partial token0 coverage is not full concentrated liquidity |
| Credit and loss | Origination, accrual, collateral, repayment, default, forgiveness, liquidation, redemption and loss allocation | Cash movement versus debt discharge; default does not erase debt; loss equals allocations plus remainder; gross-to-net authorization |
| Vaults and transient systems | Deposit/mint/withdraw/redeem, asynchronous requests, shares, shared vaults/hooks and flash-style duties | Assets versus shares; claimable versus claimed; hook footprint/interference; ordinary share vault is not shared-vault transient accounting |
| Derivatives and conditional claims | Funding/PnL, margin, bankruptcy residuals, conditional payoff and evidence | PnL versus spendable cash; oracle assumptions; non-vacuous exercise/settlement and recovery |
| Staged/external workflows | Conditional escrow, partial fills, continuations, joins, payment/service workflows | Pending/unknown/final distinctions, at-most-once consumption, compatible settle/refund branches and remaining obligations |

These batches partition work, not redefine the denominator. Preserve all 72 original DeFi rows, DA01..DA24, adopted supplemental cases, three held-outs, eight intent cases, eight regression classes and five composition operators with stable identities. Source acquisition and independent expected-result derivation can proceed before backend completion. Final qualification cannot. Required source gaps must not become silently excluded rows. A row has separate semantic, theorem/certificate, compiled-target, local-ledger and Preview evidence cells. A universal profile theorem may cover several rows only through checked domain instantiations; representative proofs alone do not.

## One phased roadmap

Phase labels below are proposed planning labels, not new acceptance authority. Each has one semantic owner and may contain bounded candidates. One delivery implementation item remains in progress; independent expected-result work and one bounded native feasibility discriminator may proceed without competing implementation queues. No calendar duration is promised before measurement.

| Phase | Dependency and canonical deliverable | Measurable exit and evidence | Migrated work / acceptance preserved |
|---|---|---|---|
| U0: Freeze the contract and target | Current product contract; one supported-Core/stage contract; source/profile embeddings; pinned target, key, verifier and phase matrix; traceability register | Every used operation, public binding, arbitrary-witness premise and phase effect has a checked mapping or named blocker. Produce actual command invocations from inspected pinned tools. No unresolved item is marked proved. | P0, C0, K0; MC01 and early MC04 interface questions; SP01 and SP04/F0 preparation |
| U1: Establish the common basis | U0; checked UInt128 addition as first candidate, then only primitives needed by the first financial relation; canonical intent/effect/frame encoding; initial obligation reporting | Certificate separates reference/host/constraints and includes rejection, range, frame and work conditions; valid and hostile witnesses; theorem assumptions and circuit measurements. Report any unsatisfied proof premise as conditional. | P1, essential P3 reporting, K1 foundation; MC01, MC03 feasibility, SP02/03/04 subsets |
| U2: Deliver one native public financial path | U0/U1 supported domain; one source-to-Core-to-ZKIRv3 relation carrying authenticated intent and complete phase outcomes; first conditional two-asset episode | Independent new program and two satisfying proposal paths; real proofs and actual target enforcement; invalid program/intent/effect/predecessor variants reject. Finalized Preview effects and independent readback match expected accounting for the stated subset. | P2, minimal C1/C2 and K2; MC02/04/05 subset evidence, SP05/09 subset. Loan/swap MC02 remains separately required; no full MC05 closure from a subset |
| U3: Qualify history, recovery and private composition | U2 relation; full native recursion and private multi-parent composition targeted for the user-assumed ~March 2027 capability horizon; native certificate/base/preservation interfaces checked early; persistent continuations, split/join, state completeness, bounded recovery and consent-preserving migration | Real retained native recursive financial proofs, independent final verification, correct genesis/parents, durable unique consumption, late-result/race controls and private handoff across isolated principals. Every branch accounts for duties. Missing native interface blocks these claims. | Remaining P4, C3, K1/K2; MC03/04/05/06, SP06/09/10; foundational semantics specified already in U0, not invented after U2 |
| U4: Qualify the retained financial corpus | U2 for simple families, U3 for dependent composition; libraries and necessary explicit profile extensions | Identity-preserving coverage manifest; every required row has complete independent expectations and supported proof/target instantiation; full mutation matrices on pinned local target plus required scoped Preview evidence. Requalify changed upstream predicates. | P6, financial C4, SP07/08/11, MC07; prepare source/library work earlier without claiming this exit |
| U5: Integrate optional federated solver applications | Relevant U2/U3 guarantees; OWS-compatible constrained signing boundary; selected x402 schemes; exact-effect domain adapters; common evidence statement | Independent solver accepted; raw token possession cannot bypass policy; concurrent reservations survive crash/retry; scheme-specific payment, result availability and delivery evidenced separately. Actual external effects and trust assumptions recorded. | K3/K4/K5 integration, remaining C4. This is required for advertised federation capability but never a prerequisite for ordinary language deployment |
| U6: Publish reproducible developer and release evidence | U2 public flow; U3/U4 for full promised language scope; U5 only for federation claims included in that release | Two clean builders; independent non-fixture authoring without project records; retained proof/receipt verification; full MC/G crosswalk, pilots, licenses, baseline comparison and privacy evidence; exact unresolved blockers | P7, developer C4, K5 evidence where advertised; MC08/SP12 and G01..G24 |

Early native feasibility is part of U0/U1 and must resolve before resource-intensive dependent proving. U2 may produce a useful scoped language result before U3; that result must not be described as complete mandatory historical PCD. Retain the original MC dependencies for full closure. SP completion dependencies remain acceptance dependencies, not reasons to postpone independent preparation.

Advisory exact refinements from P3 may improve U1 onward when they help the frozen relation. P5 synthesis is an optional tooling package after measured checker usefulness and a manual-template baseline. MPLR-035 solver completion is a semantic prerequisite for U2 route choice; AI program synthesis is not. Neither P3's SMT checker nor P5 is a prerequisite for financial-library conformance or manual developer release.

## Next meaningful financial vertical slice

Use a **source-authored two-asset conditional exchange with a persistent remainder**, extending the actual integration path rather than adding another hard-coded application profile. Retain I2 complete-effect loan/swap reconciliation once as the existing baseline. The next implementation package should close a concrete source/Core/effect or lowerer gap toward this episode, not attempt the entire roadmap at once.

Specify two authorized offers: 10 A exchanged for at least 20 B net at the intended recipient; fees at most 1 A within a total 11 A gross debit cap. Bind exact asset/domain identity, phase fees, allowed partial-fill rules, document predicate, recipient acceptance, deadlines and recovery authority. The counterparty's give/receive and obligation constraints also apply. Exact proportional minimums for partial fills and rounding must be signed rules rather than assumed from the overall ratio.

The minimal episode has a funded pending stage, one accepted partial fill leaving a precisely measured residual entitlement/duty, and completion or authorized recovery. Provide a separate unfunded request variant to demonstrate that conditional settlement is broader than escrow. First demonstrate same-Midnight-domain effects; then extend the same relation with an explicitly authenticated delayed external outcome. An unregistered second solver may choose an alternative permitted route. No trusted membership list or solver-specific proof branch may be used.

Independent expected traces must state every party's A/B balances, custody/reserves, fee recipients, gross consumed debit, minimum net credits, liabilities, authority, work reserves, logical request IDs and residual obligations after every stage. Terminal branch exclusion must be checked against current authenticated state, not merely a historical proof.

Required hostile cases: extra fee leg, wrong recipient, wrong price orientation/scale, overflow, partial-fill fee reset, hidden reservation, unauthorized liability, forged document success, missing recipient consent, stale/replayed evidence, wrong domain/key/program, dropped remainder, duplicate branch consumption and expired initiation authority reused for new work. The recovery extension races a late authenticated success with refund, tests depleted ordinary work with a closure reserve, and reaches the ID cap with an outstanding duty. Known success remains known; timeout alone cannot authorize a contradictory refund. Recovery must spend still-controlled custody under explicit surviving authority or obtain an amendment.

U2 accepts only the implemented subset with actual target evidence. U3 closes the stronger history/private/join claims. Positive paths are essential: two valid routes, a valid partial fill and a funded authorized recovery prevent a vacuously rejecting design from appearing sound. Do not add every financial family to this first circuit.

## Artifact and command exits

Existing inspected commands are suitable for local regression or plan checks, with their limited meanings:

```text
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
npm --prefix experiments/moriarty-language run typecheck
npm --prefix experiments/moriarty-language test
python3 openspec/sprints/verify.py
openspec validate --all --strict
```

Only the first was run for this report. The npm scripts were inspected; they do not verify the proposed target relation. OpenSpec/sprint commands are declared plan checks, not financial evidence. Exact target/proof/ledger commands are unresolved inputs to U0, as the inspected P0/P2 plans themselves state. Inventing a plausible executable path now would weaken the delivery contract.

For each U-phase candidate, require a concrete execution manifest containing existing executable and working directory, exact argv/tool versions/input digests, owned outputs, environment prerequisites, timeout/resource ceiling, independent expected observations, negative controls and the exact acceptance predicate. No placeholder command, symbolic role or `candidate-full` string counts as executable readiness. Proposed deliverable names such as `target-manifest.json`, `stage-relation.md`, `primitive-certificate.json`, `coverage.json` and `result.json` remain proposed until their paths and schemas are bound to an actual candidate.

The target exit is an artifact set: source and canonical Core; signed policy and phase layout; target instructions/code; circuit/key/SRS identities; statement/proof bytes; independently replayable verification; complete actual effect projection; finalized receipt/readback where required; costs and residual duties; and negative-control outcomes. Each evidence record says observed, proved under named assumptions, tested only, specified only, blocked or rejected. Store randomized proof bytes for independent verification; do not demand identical randomized proofs from two builders.

Pel should bind an immutable approved specification, actual implementer/reviewer identities, exact owned files and real verification commands to one candidate. The inspected template skips review after failed verification and permits one correction; preserve its bounded stop behavior. Model approval and static Pel validation do not establish any financial predicate. Internal campaign accounting must be reconciled before dependent dispatch; preserve old charges and resource failures. This review supplies no new proof or transaction budget.

## Requirements: EARS/OpenSpec/MPLR/Pel traceability

Keep the existing MOR/AEO and MPLR identities. Make one canonical row per behavior with: source authority; EARS requirement; Core judgment; target obligation; positive/hostile witness; phase owner; MC/SP acceptance link; artifact/command; evidence status; dependency and residual assumption. The three OpenSpec changes can remain historical/source views, but their task lists must point to one canonical task rather than recreate it. MPLR-035 currently has the research relation; add its concrete OpenSpec/EARS row during consolidation instead of treating a study link as execution readiness.

The following are proposed canonical EARS clauses, with a primary delivery owner and distinguishing rejection witness. All are future obligations, not passed checks. Every applicable requirement also needs its positive witness and target correspondence; no table row closes through prose alone.

| MPLR | Proposed canonical clause | Owner; distinguishing witness |
|---|---|---|
| 001 | When a workflow commits a stage, the system shall record its effects and outstanding state. | U2; reject reporting a successful prefix as complete |
| 002 | When a stage suspends, the system shall persist a typed continuation bound to its program, predecessor, evidence, authority and duties. | U3; reject unrelated callback/reuse |
| 003 | When delivery has conditions, acceptance shall require the bound evidence combination before delivery. | U2; reject digest-only or expired evidence |
| 004 | When partial fulfillment occurs, acceptance shall preserve cumulative bounds and residual duties. | U2/U3; reject fee reset or erased remainder |
| 005 | When recovery executes, the system shall distinguish refund of controlled assets from new compensation. | U3; reject refund after incompatible completed delivery |
| 006 | When an outcome resumes work, acceptance shall verify origin, correlation, finality policy and consumption. | U3; reject forged success or duplicate effect |
| 007 | When branches join, acceptance shall apply separate readiness/success policies and retain late-branch duties. | U3; reject discarded losing-branch debit |
| 008 | When a stage or nested action executes, acceptance shall enforce only its current scoped consent. | U2/U3; reject inherited ambient authority |
| 009 | When stages consume resources, accounting shall include cumulative work, retained fees and protected reserves. | U2/U3; reject retry budget reset |
| 010 | When a deadline passes, acceptance shall apply signed expiry rules without inferring nonexecution from silence. | U3; reject timeout-only refund after late success |
| 011 | When work resumes amid interference, acceptance shall revalidate current state, authority and unique consumption. | U3; reject stale snapshot reuse |
| 012 | When tools report results, they shall distinguish simulated, proved, unresolved and accepted stages. | U1 onward; reject skipped effect shown as settled |
| 013 | Where data is private, the profile shall specify observers and authorized continuation witness availability. | U3; reject leakage outside the stated observation model |
| 014 | When compilation lowers a stage, correspondence shall preserve permitted traces, failure, effects and declared cost. | U1/U2/U3; reject honest-witness-only soundness argument |
| 015 | When a new program uses supported constructs, public validation shall depend on those rules rather than example membership. | U2/U6; reject registry-only denial |
| 016 | Where an application restricts participants, the system shall enforce that policy without adding project deployment permission. | U2/U6; reject mandatory reviewer receipt |
| 017 | When a stage is accepted, its authenticated frame shall account separately for assets, supply, custody, fees and liabilities. | U2/U3; reject debt folded into token conservation |
| 018 | When signing a conditional intent, the display shall expose the canonical destination, conditions, evidence, partial outcomes and recovery. | U2/U6; reject unbound automatic-refund promise |
| 019 | When a party acquires a material obligation, acceptance shall establish its applicable consent. | U2/U3; reject debt created by passive receipt |
| 020 | When a primitive is substituted, its bound certificate shall preserve preconditions, values, rejection, effects and relevant costs. | U1; reject missing carry/assertion |
| 021 | Before accepting bounded execution, verification shall establish the bound for the concrete artifact and late-bound components. | U1/U3; reject unbounded extension or unsafe state rollover |
| 022 | When accepting program/evidence substitutions, verification shall preserve each distinct commitment role. | U1/U2; reject cross-program/stage/domain substitution |
| 023 | When a predicate is mandatory, target acceptance shall depend on its successful enforcement. | U2; reject ignored false signature predicate |
| 024 | When atomic settlement is claimed, the claim shall identify its domain and exact effects. | U2/U5; reject external relocation called local atomic delivery |
| 025 | When obligations are netted, acceptance shall establish an authorized gross-to-net relation preserving fees and duties. | U3/U4; reject equal net balances hiding gross excess |
| 026 | When a settlement implementation is substituted, acceptance shall establish its behavioral postconditions. | U3/U5; reject API-compatible Pending as Delivered |
| 027 | When history is recursive, verification shall establish valid genesis, compatible parents, well-founded lineage and resource/duty preservation. | U3; reject forged funded origin/cycle/reused spend |
| 028 | When code, policy or continuation evolves, acceptance shall preserve signed semantics and duties or require an authorized amendment. | U3; reject interface-compatible beneficiary/recovery change |
| 029 | When a claim relies on absence or completeness, verification shall bind the authenticated current domain and establish that property. | U3; reject omitted private reservation |
| 030 | When federation evidence is combined, acceptance shall bind a common intention/domain/stage/epoch/artifact/effect statement and expose assumptions. | U5; reject proof for X plus signature for Y |
| 031 | When a solver requests an effect, the enforcement boundary shall constrain it by owner delegation. | U2/U5; reject token possession as arbitrary signing authority |
| 032 | When concurrent work reserves funds, accounting shall include spent, pending, fees and residual commitments durably. | U3/U5; reject individually valid combined overspend |
| 033 | When a service is purchased, the system shall track payment finality, result availability and recipient delivery separately. | U5; reject TEE-held result as delivery |
| 034 | When a request retries or recovers, the system shall retain logical request identity and distinguish retrieval from new purchase. | U5; reject replayed transport granting new paid service |
| 035 | When a solver fills holes or combines policies, acceptance shall prove refinement of every applicable signed constraint and preserve commitments, disclosure and duties. | U2/U3; reject balanced extra-fee route; accept another permitted route |

MOR-009..012 supply cross-cutting target, certified-basis, phase and accounting obligations across U0–U4. AEO-001..003 inform explicit reports/checker work; AEO-004 synthesis remains optional tooling. This mapping preserves each MPLR identity without prematurely choosing a theory or proof tool for every open question.

## Migration and rejected alternatives

| Conflict or tempting alternative | Disposition |
|---|---|
| Keep three independent P/C/K queues | Replace with canonical phase tasks; retain original IDs as aliases and evidence provenance. One writer owns each shared Core/acceptance rule. |
| Rebrand MC/SP as complete because a new phase passed | Preserve all predicates and candidate scopes. Many-to-many crosswalks never imply inherited acceptance without predicate-specific evidence. |
| Old optional Lean bridges and planned `.lean` paths | Supersede as delivery dependencies. Retain historical research receipts. K remains executable semantic evidence where applicable; native Midnight remains the proof/target route. |
| Old blanket rejection of recursive/DAG history | Preserve as a historical architectural decision; reconcile newer MC03/MC06/MPLR-027 obligations explicitly. Ledger induction may cover named early local-history claims; target full native recursion and private multi-parent composition under the user-assumed ~March 2027 horizon. Preserve MC03/MC06 without reduction. Actual interface qualification remains open. |
| DeFiFormal trusted templates become the public program catalog | Reject. Reuse semantic obligations, not administrative template admission. Its source models require native correspondence and owner-authentication work. |
| Broad library build before native target check | Reject. Discover unsupported statement/ledger boundaries early; prepare independent source expectations in parallel. |
| Prover-only prototype before meaningful finance | Reject. The first relation must include complete fees, assets, authority, liabilities and actual financial effects. |
| Finish all federation adapters before language release | Reject as a dependency. Qualify language and optional service scope separately while keeping requested federation delivery on the roadmap. |
| Treat synthesis as the core product | Defer until exact checker usefulness; preserve manual and independent solvers with identical objective acceptance. |
| Treat a stage cap as full lifetime semantics | Require explicit bounded episode plus authorized rollover/recovery, or state the finite lifetime honestly. PCD compression does not solve replay/state growth. |
| Promise unconditional escrow release or cross-chain atomicity | Reject. State the evidence, liquidity, witness, availability and finality assumptions supporting each enabled remedy. |

Migration should first freeze the current requirement/evidence snapshot and mark the P/C/K task lists superseded by the consolidated execution view, without deleting raw studies or historical receipts. Assign every existing task to one canonical task, preserve unresolved source gaps, then rewrite roadmap navigation and development focus to point to the same next artifact. Recompute plan consistency and inspect the next candidate's actual command readiness before dispatch. Do not build a new scheduler, approval service or orchestration framework to accomplish consolidation.

The unresolved decisions that can change this recommendation are narrow and empirical: the pinned native recursion/finalizer interface, actual signature and certified-arithmetic constraint costs, complete source/Core embedding, private state completeness/witness availability, and the recovery/rollover rules. Their current absence stops dependent acceptance claims while design and implementation preparation continue toward the user-assumed full-recursion horizon; it does not justify weakening permissionlessness, moving to a foreign backend, substituting Lean, or declaring the full financial corpus complete.
