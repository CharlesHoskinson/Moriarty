# DeFi report review and plan crosswalk

Reviewed the complete immutable report, lines 1–1225. The report supports Moriarty's target-first direction, but it does not validate the current implementation. Its proposed broad kernel also permits unbounded state and delays proof-carrying integration. Those two proposals conflict with this project's explicit bounded-language and mandatory-PCD requirements and should not be adopted.

Moriarty is a Midnight-centric language. The report's runtime-neutral architecture is useful comparative analysis, but Midnight proof, privacy, witness, ledger and resource constraints govern the executable language. Other-chain holdouts contribute financial behaviors to model on Midnight; they do not authorize deployments or adapters for those chains.

This review reads the report as a source of claims, counterexamples and design recommendations. It does not independently reproduce the DeFi archive's mathematics or current protocol documentation. The graph marks report-explicit relationships as EXTRACTED; that label means the report says it, not that it is proved true.

## Required dispositions

| ID | Report evidence | Current evidence or plan | Disposition |
|---|---|---|---|
| DFI-01 | Lines 442–511 specify nominal Party/Asset/Domain/Claim/Capability identities, footprints and effects. | `experiments/moriarty-language/src/types.ts:4` stores UInt128, Text and Amount; `:20` limits effects to Transfer/Fee/DueCreated/DueSettled. The manifest at `:40` has no complete typed-port or assume-guarantee schema. | MC01 is a useful atomic subset. Before treating it as the final core, freeze a target-derived semantic extension contract for nominal authority/domain identity, declared footprints, effects and assumptions. Do not confuse Text equality with authenticated identity. |
| DFI-02 | Lines 622–654 and 783–793 require liabilities and claims to survive transfer, modification, discharge and default. | `spec/semantics.md:180` retains obligation IDs and settled tombstones. At `:205` partial settlement is explicitly unsupported; `:239` excludes Pending, split, join and continuation export. | Preserve these honest bounds. Before relation extension, define conditional payoff, due/expiry, residual amount, creditor/debtor change, default/loss allocation and pending transitions as versioned finite semantics. Reject unsupported forms rather than silently mapping them to token balance changes. |
| DFI-03 | Lines 558–620 distinguish five composition modes. | MC06 spec lines 14–30 requires split/join, obligation/authority preservation, global bounds and ledger uniqueness, but does not itself specify full shared-state or async composition semantics. | Add an operator matrix: sequential, disjoint parallel, shared-state interleaving, atomic synchronization, async messaging. State which forms are supported, their read/write and assumption contracts, and explicit rejection of unsupported modes. Split/join proof composition alone does not imply any of the other operators. |
| DFI-04 | Lines 658–678 allow unbounded numeric domains and compare behaviors by observations. | Moriarty uses frozen UInt128, source/state/expression bounds and a nonresetting lifetime; the user requires finite and bounded guarantees. | Reject the unbounded-domain proposal for the executable language. Every admitted collection, schedule, arithmetic loop and message history needs an explicit bound, bounded failure result and global lifecycle accounting. Finite descriptions and per-step termination alone do not prove bounded total history. |
| DFI-05 | Lines 1057–1138 call for normalized benchmarks, adversarial encodings and frozen-kernel holdouts before broad generalization. | MC07 design `:5` depends on MC01, MC04, MC05 and MC06. New financial semantics and pending obligations appear only at `:34`; prior proofs then require requalification at `:35` and `:54`. | Move semantic pressure tests and the coverage-oracle design ahead of final core/relation freeze. Keep full implementation, proof and Preview conformance in MC07. This reduces repeated changes to already built proof/ledger interfaces without weakening any required row. |
| DFI-06 | Lines 61–89 and 325–363 explain that 72 rows mix organizations, products and versions; lines 1063–1073 require normalized identities and independent annotation. | MC07 currently treats immutable 72 historical rows as its mandatory denominator and caps proof episodes at 349 (`design.md:35–37`). | Preserve all original rows and all 277 ACTUS fixtures. Add a many-to-many normalized product/version/deployment index, source pins, behavior IDs, facets, assumptions and ambiguity dispositions. Derive the number of required episodes from behaviors; 277+72 does not by itself establish an adequate episode count after row splitting. Reestimate resource allocations from the actual manifest before launch. |
| DFI-07 | Lines 680–799 name eight theorem families; lines 932–999 require recomputed judgments. | Current generic profile has numerical types, explicit effects and required-claim identities, but not proofs of the full open-kernel metatheory. MC04 focuses on the supported finite compiler/ledger domain. | Add a theorem ledger mapping type preservation, accounting, authority, frame, assume-guarantee, associativity, obligations and conservative extension to definitions, domains, assumptions and evidence. Report each as proposed, tested or mechanized. Claim IDs, policy prose and keyword names are not proofs. |
| DFI-08 | Lines 824–928 demand exact financial arithmetic, ordered redemption, bad debt, shared accounting, async settlement, margin and conditional claims. | Atomic loan/swap examples and MC07 NAM19/refinance/pending-redemption cases cover only part of this semantic pressure. `spec/target-crosswalk.json` honestly records many required extensions. | Create early bounded regression specifications for all eight report classes. Each must identify complete observations, valid cases, rejected mutations and the responsible library/profile. A bounded library algorithm can be unrolled or have a verified fixed iteration cap; it cannot inherit unrestricted host loops or hidden quantifiers. |
| DFI-09 | Lines 1001–1030 require differential execution, mutation testing and selected refinement proofs. | MC02 already requires independent full-effect comparison; MC04 requires finite-domain correspondence; MC07 requires independent expected fields. | Retain these gates and extend observations beyond token balances to debt, shares, fees, order, status, messages and claims. Mutation coverage must include delayed settlement made immediate, lost residual debt, authority escalation, fee/rounding direction, bad-debt socialization and provenance substitution. |
| DFI-10 | Line 1140 postpones proof-carrying transactions until broad generalization and presents them as optional where beneficial. | User requires correctness proofs in transaction acceptance; MC03–MC06 explicitly allocate native proof, verifier, mandatory-claim and history-composition work. | Reject optional/late PCD architecture. Design the mandatory proof/public-input/history contract with the bounded semantic kernel now. Keep initial MC03 fixture proof scoped as an experiment; require changed-domain proof and ledger requalification for every extension. Runtime proof, formal theorem, oracle attestation and legal assertion remain separate evidence classes. |
| DFI-11 | Line 1142 expressly disclaims architectural evidence about Moriarty, Compact and ZKIR. | MC04 has separate retained source analysis of native-IVC versus ledger-verifier compatibility. | Do not cite this report as proof of target capability. Keep MC04 source-backed Midnight compatibility work on the critical path, including the complete recursive verifier/decider and final accumulator obligations. Adopt an internal language/ledger boundary, not a chain-neutral product or multichain deployment roadmap. |
| DFI-12 | Lines 801–820 and 1032–1055 separate conditional economic guarantees, sampled execution and proofs. | Current plan already separates oracle truth, uniqueness, semantics and privacy. | Preserve these boundaries in claims, mock UI, receipts and release documentation. Bounded correctness says the stated finite semantics were obeyed under assumptions; it does not establish solvency, real-world truth or unconditional settlement liveness. |

## Complete requirement inventory

The report's architecture separates empirical ontology, financial libraries, typed open transition kernel and runtime adapters, with assumptions orthogonal to all layers (lines 3–57). Retain ACTUS contract-event/obligation semantics, CDM product/lifecycle separation, Marlowe's semantics/ledger separation and resource/capability techniques as distinct influences. None fixes Moriarty syntax or implies a verified target adapter.

The historical corrective requirements are to withdraw the Q/Σ sort partition and four-primitive minimality claim; retain the atlas, residues and useful conditional interface results; distinguish syntactic generation from semantic completeness; repair the non-idempotent Delta terminology; distinguish induced lattice structure from set-operation closure; and separate sample denominators from exhaustive domains (lines 59–245). These corrections are report claims about another archive. They must not silently rewrite or promote its historical evidence.

The taxonomy requirements are stable organization/product/version/deployment identity; independent economic-function, instrument/claim, mechanism, execution/settlement and dependency/trust facets; graph relationships; explicit ambiguity and normalization rules; and two independent annotators with reusable disagreement dispositions (lines 247–394, 1063–1073). The proposed twelve extra holdouts are dYdX Chain, Osmosis, DeepBookV3, THORChain, Velocity, Kamino, Euler V2, Term Finance, UMA Optimistic Oracle, Nexus Mutual, Lightning and Balancer V3. They are a separately identified behavioral extension benchmark. Their relevant financial mechanisms must be encoded against Midnight constraints. They neither replace the mandatory 72 rows nor imply that those chains or deployments are execution targets.

The semantic requirements include typed interface ports, initialization, state, labeled transitions, observation map, read/write footprints, effect summaries, assumptions and guarantees (lines 442–475). Identity types include Party, Asset, Domain, Time, Claim, Capability, Message and Identifier. Quantity/price units, overflow, exactness and rounding must be explicit. The proposed effect vocabulary includes Transfer, Issue, Retire, CreateClaim, DischargeClaim, UseCapability, Observe, Send and Receive; these are semantic records, not promises that every backend directly exposes those instructions (lines 476–511).

Composition requires five separate operators and explicit compatibility rules. Async execution must retain Created, Observed, Finalized, Executable and Executed phases plus Expired, Reverted, Challenged and Compensated branches as applicable. Finality and attester correctness are assumptions. Claims retain debtor, creditor, asset/payoff, due time, condition and status. Conservation of spendable assets must remain distinct from liabilities, contingent value, accessibility and solvency (lines 558–654).

The theorem program covers type preservation; per-asset conservation including issuance, retirement and external boundaries; capability safety; frame/noninterference; assume-guarantee discharge; associativity up to typed interface/state renaming; obligation preservation; and conservative extension (lines 680–799). Equivalence must declare observations and internal-event hiding (lines 666–678). Kernel simplicity must account for syntax, semantic definitions, trusted code, proof and annotation burden, and unrestricted host escape hatches (lines 513–556).

The verification requirements are recomputed TypeCorrect, FootprintCorrect, AuthorityCorrect, AccountingCorrect, AssumptionsDeclared, CompositionCompatible, LibraryTheoremsInstantiated and SourceRefinementObligations judgments; source spans and exact arithmetic-library hashes; complete observation comparisons; deliberate semantic mutations; and selected concrete-to-IR simulation proofs (lines 932–1030). Linear AST traversal does not justify linear proof-generation claims. The report's Python/Node reproductions and unavailable Lean/Quint toolchains remain explicitly scoped (lines 1032–1055).

## Evidence limitations

The report has two `sandbox:/mnt/data/` links, for a proposed 72-row CSV crosswalk and taxonomy JSON Schema. Neither attachment is contained in this Markdown file. Their contents were not reviewed or imported. Reconstruct any adopted schema and mapping in the repo with source-linked evidence rather than assuming those links resolve.

Its web citations are opaque `turn…search…` tokens, with no resolvable bibliography in the supplied file. The graph preserves every unique token as an unresolved citation, with no invented URL or paper identity. Specific claims about current deployments need separately retained primary sources before they become conformance requirements. Historical quoted file paths and reported executions are retained as report references, not independently verified results.

The graph contains meaningful concepts, all named holdouts, theorem and certificate families, effects, relevant archive references and opaque web citations. Its token counters are required schema placeholders: actual input/output usage is unavailable, not zero cost. The extraction was checked for valid node IDs, closed endpoints and legal confidence values.

## Reading coverage

The following intervals partition the entire report by Markdown headings. Every interval was read. The title interval also covers the source document identity. Bold subsection labels inside these intervals were read and represented by their named concepts where substantive.

| Lines | Heading |
|---|---|
| 1–2 | Rebuilding the DeFi Kernel: Simpler Semantics, Stronger Mathematics, and an Evidence-Based DeFi Taxonomy |
| 3–58 | Executive recommendation |
| 59–60 | What the repository actually establishes |
| 61–90 | The atlas, corpus, algebra, and positive program are different experiments |
| 91–124 | The original closure/kernel algebra contains acknowledged contradictions |
| 125–155 | The positive program refuted its own four-primitive premise |
| 156–199 | “Generation” is useful measurement, but not semantic completeness |
| 200–246 | Several Lean results are good, but narrower than their names suggest |
| 247–248 | DeFi taxonomy rebuilt |
| 249–293 | The right ontology is faceted and graph-backed |
| 294–324 | A revised economic-function layer |
| 325–364 | What the 72 records look like under the revised model |
| 365–395 | Holdouts expose what the current categories miss |
| 396–397 | The replacement kernel |
| 398–441 | Comparing the candidate architectures |
| 442–512 | A proposed DeFi Semantic Kernel |
| 513–557 | Why this is simpler despite having more visible structure |
| 558–621 | Composition must be several operators, not one overloaded bond |
| 622–655 | The correct role of obligations |
| 656–657 | Mathematical program and protocol encodings |
| 658–679 | Define completeness before discussing minimality |
| 680–800 | The positive theorem ledger |
| 801–823 | What should explicitly not be promised |
| 824–929 | Worked regression encodings |
| 930–931 | Validation, certificates, and implementation plan |
| 932–1000 | A real certificate should certify judgments, not names |
| 1001–1031 | Fidelity should become a first-class proof obligation |
| 1032–1056 | Reproducibility status of this investigation |
| 1057–1143 | Migration plan |
| 1144–1225 | Open questions and final disposition |

## Suggested execution order

1. Close the bounded semantic extension, normalized target manifest, adversarial observation specifications and mandatory PCD public-input contract. Reuse existing MC01 code where its semantics match; keep profile versioning explicit.
2. Continue MC01 acceptance fixes and independent audits. In parallel, resolve the MC04 complete-verifier compatibility decision from actual target sources.
3. Run the small, explicitly scoped MC02 financial settlement and MC03 native-proof feasibility experiments only against their frozen supported profiles. Neither accepts the full kernel.
4. Extend and requalify MC04–MC06 for the selected real acceptance, authority, private handoff and composition semantics.
5. Execute the complete MC07 financial coverage and separately frozen holdout behaviors on the Midnight target, then MC08 end-to-end and release audits. Revisit earlier theorem/proof/ledger evidence when the supported semantic domain changes.
