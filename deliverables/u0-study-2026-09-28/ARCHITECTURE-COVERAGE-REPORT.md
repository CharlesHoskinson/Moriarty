# Does the roadmap have an architecture for DeFi?

**Date:** 2026-09-28
**Status:** specified-only. A review of two documents against nine category studies. Nothing here is an accepted change, and nothing was committed.

**Question asked.** Not whether the DeFi categories are built or specified — that is the [coverage report](DEFI-COVERAGE-REPORT.md) — but whether the language **roadmap and architecture** say how each category *will* be built and how the components fit together. Nothing has to exist yet; there has to be a design.

**Sources.** `ROADMAP.md` (56 lines) and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines), read in full, plus `docs/MORIARTY-PRODUCT-CONTRACT.md` and `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md` where they control. The nine reviewers each appended an `## Architecture axis (follow-up)` section to their report in [`defi-coverage/`](defi-coverage/).

**Path convention.** Bare paths are Moriarty at `8f73784042bd692733c296d0d49f5173be96725e`. Paths under `lean/`, `algebra/`, `corpus50/`, `quint-models/` and `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` are **defiformal** at `8c5dd103cd40369a763b02b1504441acce0ce3c2` and are marked *[df]*. A bare `:N` inside a paragraph refers to `docs/MORIARTY-CONSOLIDATED-DESIGN.md`.

---

## 1. Headline

**Yes — there is a real architecture, and it is substantially better than the coverage report implied.** Scoring the kernel abstractions by whether the design *names a component responsible*, rather than by whether anything works: **16 designated, 4 partially designated, 8 with none designated, 1 excluded by design.** On the build axis the same 29 scored 3 expressed and 12 absent. The architecture covers more than the specification does, which is the right direction for a project at U0.

The three-column responsibility boundary (`:23-36`) is the strongest artifact in either repository. Twelve concerns, each split across Moriarty / Federated DeFi Kernel / Midnight, each row stating what a layer may and may not claim. Every concern that could be smuggled between layers has a row.

**What is missing is narrower and sharper than "DeFi isn't covered".** Two structural gaps, one convergent gap, and a class of smaller holes:

1. **No notion of a region anywhere.** This costs independence checking, interference reasoning and framing at once.
2. **No artifact joining the nine library families to the core's constructors.** The core inventory exists; the join does not.
3. **The authority model is closed and too narrow** — four reviewers reached this independently from four different categories.

And a structural observation about the roadmap itself: **`ROADMAP.md` assigns demonstrations and evidence, never designs.** Every U row's closing column is phrased as artifacts and receipts. No row says "produce the architecture for X". That is why six of the seven holes below are unassigned.

## 2. The architecture as stated

Seven components, reconstructed from the design:

1. **The small core** (`:15`) — eight enumerated capacities: exact typed values; bounded evaluation; explicit state and effects; checked authority; assertions and refinements; authenticated evidence; persistent obligations and continuations; defined composition operators. This is the load-bearing sentence: the only place the core's responsibilities are enumerated.
2. **Source libraries over the core** (`:15`, `:93`) — nine families, each to receive "a scope matrix, independent source-derived expectations and held-out compositions".
3. **A language version** (`:15`) — "states exactly which constructors, bounds, numeric semantics and evidence relations its compiler supports". This is the designated carrier of the core/library interface.
4. **The public pipeline** (`:19`) — author and inspect → elaborate/type/effect/bound check → expose proof obligations → compile to pinned ZKIRv3 → generate native evidence → submit → check actual ledger results.
5. **The acceptance relation** — the canonical stage statement (`:42`), the four judgments (`:48`), and the enforcement map (`:44`, `:46`) tying each bound field to an actual circuit constraint, authenticated state read, signature commitment or ledger check.
6. **Certified primitives / jets** (`:91`) — Simplicity-style, with call sites discharging preconditions and framing.
7. **The three-column responsibility boundary** (`:23-36`), mapped to CAKE's APSS at `:38`.

## 3. Where each category is placed

| Category | Architectural placement | Stated? |
| --- | --- | --- |
| AMMs / exchanges | Pool pricing → library family (`ROADMAP.md:48` "AMMs", `:93` "swaps and liquidity"). Matching and routing → Federated Kernel (`:26`, `:29`), which is optional (`:11`). Order books, batch auctions, RFQ → **nowhere**. | Partly |
| Lending | Library, split across **three** families — "loans and claims", "redemption/loss allocation", "margin" (`:93`). No document states an interface between them. | Yes |
| Stablecoins / issuance | **Nothing places it.** Neither family list contains an issuance or monetary family; the Kernel's financial row is "Find liquidity and counterparties" (`:26`); adapters establish effects (`:38`). Its only appearance is as U6 conformance rows — a test denominator, not a home. | No |
| Derivatives | Library, twice and consistently — "margin" and "asynchronous and conditional claims" (`:93`, `ROADMAP.md:48`). | Yes |
| Oracles / observations | Three-way split, and the split *is* the design: policy → core and the Moriarty column (`:15`, `:30`); collection → Kernel (`:30`); truth → external trust assumptions (`:30`). | Yes |
| Governance | **Nowhere.** No administrative library family, no core capacity, not in the Kernel charter. Only "application policy" (`:27`) and "amendments" (`:25`) appear, both in the Moriarty column. | No |
| Bridges / cross-domain | Federated Kernel and settlement adapters, explicitly and optionally (`:11`, `:38`, `ROADMAP.md:26`). Coherent and largely correct. | Yes |
| Staking / yield | Library family via its **vault half only** (`:93` "shares/vaults"). The staking half — delegation, slashing, unbonding, rewards — is placed nowhere. Across both documents `stak*`, `slash*`, `reward` and `emission` occur **zero times**; `share|vault|pool|accru` occur on exactly two lines. I ran both greps. | Partly |

**Placement is mostly right where it exists.** Derivatives, oracles and bridges are placed correctly. The failure is rarely placement — it is the mechanisms that placement presupposes. R4's formulation is the sharpest: the architecture **places** derivatives and does not **architect** them.

## 4. Five shapes of architectural hole

The nine reviewers produced 51 hole findings. They consolidate into five recurring shapes, which is more useful than the list.

**Shape 1 — a bound field no layer is responsible for producing.** `:42` binds "complete gross and net effects including fees and **supply changes**" into every accepted stage. The Financial meaning row (`:26`) that allocates responsibility across the three columns omits supply changes entirely. No core capacity, no library family, no Kernel duty. The canonical stage statement requires a field nothing is designated to produce. *(R3 H1, R8 H2, R9 H6, R1.)*

**Shape 2 — a property asserted with no operand.** `:72` requires disjoint parallel composition to have "checked independence", and nothing anywhere produces a read/write footprint to check it with. `:30` assigns "freshness" to the Moriarty column while `:42` binds no observed-at time to compute freshness against. `:54` asserts "Default does not erase debt" and neither document ever defines default — no maturity, threshold, event or trigger — while `ROADMAP.md:40` lists "erased debt" among the hostile controls U0–U3 must test. *(R9 H1, R5 H2, R2 H1.)*

**Shape 3 — a library family over a core that cannot type it.** `:93` designates a "shares/vaults" family over a core whose arithmetic admits no claim algebra: `Shares<vault,holder>` has no `Mul`, `FloorDiv` or `CeilDiv` rule and `Add` only within one identical type, so a total supply is not summable and the `mulDiv` shape the category runs on is not typeable. Neither document says the core suffices or names who extends it. A library family the core cannot type is a hole, not a missing library. *(R8 H1.)*

**Shape 4 — a rule carried for one case and silently not its sibling.** `:46` demands that every bound field identify its actual enforcement locus, and `:44` supplies that sentence **for signed intent only**; observations are bound at `:42` with no locus named anywhere. The design carries the anchored-versus-imported rule for imported *history* (`:79` vs `:83`, realized by U4) and not for imported *facts*. *(R5 H1, R7 H3.)*

**Shape 5 — a negative definition with no positive one.** `:93` states "Narrow token0 pricing is not a full AMM; stable-time vault conversion is not accrual." Both are correct and neither is answered: nothing states what a full AMM requires or what accrual requires. The "scope matrix" `:93` promises to each family has, repo-wide, exactly two occurrences — `:93` itself and its own draft at `deliverables/consolidated-design-2026-09-19/design-draft.md:80`. I ran that grep. The design knows the boundary is wrong and names nobody to draw the right one. *(R1 H1, R8 H3, R9 H4.)*

## 5. The convergent gap: the authority model

Four reviewers reached the same conclusion from four categories, which makes it the most reliable finding in this pass.

The architecture's authority model is two sentences. `:54`: "Consumed receipts are linear resources. Spending permission may be affine." `:68`: "Initiate, complete, reconcile, recover, disclose and amend rights have separate scopes." Six rights, all over one's own workflow, and two resource disciplines.

- **Stablecoins (R3):** there is no slot a right to change supply could occupy, yet `docs/MORIARTY-PRODUCT-CONTRACT.md:43` — made controlling by `ROADMAP.md:3` — requires "**authorized** mint/burn supply changes". I verified that line. The contract asserts an authority relation the authority model cannot express.
- **Lending (R2):** there is no seize, enforce or forced-close scope, and `:54`'s consent rule actively obstructs one. Liquidation is therefore not merely unspecified but inconsistent with the stated model.
- **Governance (R6):** `:68` *states* the six-way distinction that the schema lacks, and designates nothing to carry a right-kind — it appears in none of `:15`, `:42`, `:48` or `:93`.
- **Kernel (R9):** the same, as H6.

Two aggravating facts, both verified here. The schema freezes `authority.*` as three opaque strings (`stage-relation.schema.json:452-470`), read by the judgments as a single value budget. And the six judgments map onto only five design-document judgments: the effect and authority judgments both resolve to `designDocJudgment: "valid state/effect transition"`, while `ROADMAP.md:21` bills authority as a separate U0 exit.

**A right-kind tag is the one addition that cannot be retrofitted**, because U0 freezes the stage relation and U2 enforces it. Adding a right kind afterwards changes the authority model, the signed-intent grammar and the authority judgment together.

## 6. The library/core interface — and a correction to my own earlier account

I told the owner earlier that no document says which core constructors any library family requires. That was half wrong, and the correct version is sharper.

**The constructor inventory exists and is well-formed.** `experiments/moriarty-language/spec/successor/expression-signatures.json` holds **40** base Core constructors — I parsed it — each carrying operands, result, typing, reduction, rejections, frame, entry work and sources. Eight financial constructors at `spec/successor/financial-expression-source.md:67-90` bring it to 48. So `:15`'s designated carrier — the language version stating exactly which constructors the compiler supports — has a real instance.

**What does not exist is the join.** No artifact maps a library family to the constructors it needs:

- `ROADMAP.md:21`'s U0 exit criteria list embeddings, six judgments, numeric profile, K reconciliation, target pins, enforcement map and trust premises — and no constructor inventory.
- The nearest family-keyed artifact, `deliverables/defiformal-study-2026-09-19/libraries-review.md:5`, has a "Language primitive" column whose cells are one-line prose sketches, and it classifies defiformal's families rather than Moriarty's core.
- `deliverables/defi-language-design-2026-09-07/action-targets.csv` maps 24 targets to prose semantic requirements and a disposition, with no constructor column.
- **The two family lists do not even agree.** `:93` has "exact arithmetic/fees", which `ROADMAP.md:48` lacks; `ROADMAP.md:48` has "netting", which `:93` lacks. Both are nine long. The library inventory is not a single list.
- **One worked precedent exists and was never generalised.** `deliverables/sp02-financial-pure-expression-2026-09-10/inputs/financial-pure-extensions.md:44-50` runs the right direction — it lowers the ERC-4626 rule `floor(a·S/Va)` to an exact 9-node Core tree and reads off the 8 constructors the financial interface needs. That is one library rule, for one family.
- **The obligation is filed and owned by a non-owner.** `docs/ROADMAP-RECONCILIATION-2026-09-19.md:145` lists "enumerate exact supported constructors" with an owner column reading "P0/P2" — and `ROADMAP.md:5` says P0–P7 are "retained requirement/provenance aliases, **not parallel work queues**". I verified both lines.

Until that join exists, "financial applications are source libraries over that core" (`:15`) is not a checkable claim, and nobody — including all nine reviewers — can ask whether the core suffices to build a perpetual, a CDP or a bridge and get an answer that is not prose. This is upstream of all eight category reports.

**Second correction to my earlier account.** I called the four composition modes an architectural hole. That was wrong, and R9 corrected it: `:15` names "defined composition operators" as a core capacity and `:32` assigns mode-distinction to the Moriarty column, so the *operator* is designated. What is missing is the *premise* — independence is decidable only from a footprint, and there is no region notion in the core's eight capacities. A per-constructor `frame` field does exist in `expression-signatures.json`, and I checked its value set: four values, 37 of 40 constructors reading `unchanged`. Those classify which machine component a constructor mutates, not which ledger cells it touches — two transfers between disjoint accounts have identical frames. That strengthens the hole rather than closing it: a frame notion exists at a granularity that can never decide independence, and no document reconciles it with `:91`'s call-site framing obligation or `:72`'s independence requirement.

## 7. The canonical stage statement as an architecture

`:42` opens "Every accepted stage binds:" and enumerates exhaustively, with `additionalProperties: false` throughout its schema realisation. Three things it **structurally excludes** — no added field fixes them, because the shape forbids them:

1. **Everything below stage granularity.** One value per field, one signed intention, one failure policy. An intra-stage trace, a per-step receipt, an intermediate commitment or a located refusal has no place, because it would be a sequence of stages-worth of data inside one stage.
2. **Everything above stage arity.** The enumeration ends "resulting continuations **or** terminal outcome" — one outcome. A fork/join produces two branch outcomes merged into one world; interleaving produces two local states plus a schedule. Neither is a field of a stage.
3. **Residue.** An effect the relation cannot record makes the instance *invalid*, not *residual*. The Atlas separates those two failures *[df]* `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:689-692` vs `:731-733`; Moriarty has only the first.

Correctly and deliberately excluded: multi-domain within one stage, with cross-domain pushed to a sequence of stages (`:72`). Merely omitted and additively fixable: read footprint, capability IDs, supply-authority reference, observed value, domain on the effect line.

## 8. Owner decision: the intent language carries conditional settlement and programmable escrow

**Recorded 2026-09-28.** Conditions governing funding, release, refund and partial progress are expressed and signed as part of the canonical intention — not left to program logic alone, and not delegated to the optional Federated DeFi Kernel.

This is a placement decision, not a restatement. The design already *describes* conditional settlement: `:62` names "funded into programmable escrow" among nine workflow states, `:64` lists what conditions may combine, and `ROADMAP.md:24` makes two-asset programmable escrow a U3 exit. None of it says which component carries a condition. The signed intention as frozen has twelve fields — `intentId`, `signer`, `consentPolicy`, `delegationPolicy`, `assetIdentities`, `recipients`, `grossDebitCap`, `feeCap`, `minNetOutcome`, `validity`, `replayPolicy`, `recoveryPolicy` — caps, sets and policies, with no condition, predicate, escrow or release construct. I verified the field list.

**It closes holes this study found.** It supplies the missing component for the conjunction and threshold rules the design assigns to the Moriarty column at `:30` with no operator, core construct or library family (R5 H3); for the conditional-release and compensation branch of cross-domain workflow (`:72`, R7); and for the conditional-payoff shape that options, escrowed delivery and document-guarded settlement all need (R4).

**It obliges work inside the U0 freeze, not after it.**

- **The signed-intent syntax task must design for conditions.** It is scheduled in the freeze phase. If it admits only the twelve cap-and-policy fields, U3's escrow forces a grammar break and new signing semantics over an already-frozen intent.
- **One predicate language, not two.** The U0 plan already proposes a small fixed predicate language — linear integer arithmetic and set membership over relation paths — for executable judgment clauses. Intent conditions need a predicate language too. If these diverge, the product acquires two predicate surfaces, two evaluators, two certifications and two correspondence arguments.
- **The intent judgment grows** from cap inequalities to conditions.
- **The observation gap moves onto the escrow critical path.** A condition over an observed value cannot be written while `:42` binds observations as issuer, domain, time and finality with no value and no observed-at time. R5's finding is reclassified from a category concern to a blocker for this decision.
- **In-circuit authentication now covers a recursive predicate structure**, so its canonical encoding must be bounded before U1 can measure the cost.
- **Escrow needs custody and a release/refund branch** in a relation carrying only effects and a failure policy.
- **Recipient consent is unresolved.** `:54` requires consent from a party made liable and `:60` says recording a request imposes no duty on an unconsenting recipient — but the recipient does not sign the stage. R2 raised the same problem for liquidation as H2.

## 9. Ownership

`ROADMAP.md` assigns demonstrations and evidence; it does not assign designs. Every U row's closing column is artifacts and receipts, and no row says "produce the architecture for X". The consequence:

| Architectural work | Owner |
| --- | --- |
| Region / footprint / frame notion | **unassigned** — U3 owns "joins" and U4 owns "every retained composition operator demonstrated"; both own demonstration, neither the premise. U1 owns "caller preconditions", explicitly not framing |
| Family → constructor map and scope matrices | **unassigned** — U6 owns family *evidence*; the filed obligation is owned by "P0/P2", which the roadmap says is not a work queue |
| Authority right-kinds, incl. supply and enforcement | **unassigned** — U0 owns the authority judgment and is silent on kind |
| Nine workflow states, carrier and representation | **partial: U3**; the nine-way distinction unassigned |
| Recovery-guarantee viability predicate | **partial: U3**; the static predicate unassigned |
| Intent condition grammar (owner decision, §8) | **unassigned** — the U0 signed-intent syntax task is the natural home |
| Observation value, observed-at time, evidence class | **unassigned** — U0's evidence list never mentions observations |
| Domain-qualified effect lines; cross-domain conservation or its refusal | **unassigned** — routing and adapters are U5, but these are language-side |
| What a full AMM requires; order books, auctions, RFQ | **unassigned** — absent from every U row |
| Issuance: core mechanism, library family or kernel service | **unassigned** — absent from both forward plans |
| Staking: delegation, slashing, unbonding | **unassigned** — zero occurrences in either document |

## 10. What to add before U0-F, ranked

Each is an architectural statement. None requires an implementation now, and none touches the proof stack or the ZKIRv3 target.

1. **A region notion in the core's capacity list (`:15`)** — a declared read/write footprint per operation. Closes independence, interference and framing together. **First because it cannot be added later**: independence is not a field you add to a stage, it is something operations must carry, so every constructor designed before it needs revisiting.
2. **A right-kind tag on authority**, in the canonical stage statement and the authority judgment, ranging over `:68`'s six scopes with an open extension point, and extended now with **issuance** and **enforcement**. Second because U0 freezes the relation and U2 enforces it. Closes the §5 convergent gap and unblocks stablecoins, staking, lending liquidation and governance.
3. **One family → constructor map, with a milestone owner that is not a provenance alias.** The inventory exists; the method is demonstrated once. Repeat it for one rule per family and the artifact exists. Without it, "libraries over the core" is not a checkable claim.
4. **The intent condition grammar** (§8), unified with the judgment predicate language, designed now even though slice S0 uses none.
5. **An observed value with its declared unit, and an observed-at time distinct from stage time**, added to `:42`. Gives freshness an operand, puts the value under the numeric profile, and unblocks item 4.
6. **A decision on structure above and below the stage** — either a composite unit that a schedule, a join and two branch outcomes can belong to, plus an ordered step record inside a stage; or an explicit statement that neither exists and that liquidations, margin settlements and cross-protocol actions must be multi-stage. Silence does not resolve §7.
7. **A domain on the effect line**, so a debit in one domain and a credit in another can be stated, with the stage's domain becoming the *executing* domain; plus the cross-domain conservation position recorded as a decision rather than an omission.
8. **A residue slot on the stage statement.** Cheapest of the set, and the one that makes every later review honest: it lets a reviewer record "this protocol does X and the relation cannot represent it" without the instance becoming invalid.

Items 2, 3, 5, 7 and 8 are allocations or fields. Items 1, 4 and 6 are the genuine architecture work.

## 11. Limits

- **Two documents, plus what the reviewers cite.** Neither I nor most reviewers read `openspec/changes/consolidated-language-kernel/{proposal,design,requirements}.md`, `docs/MORIARTY-BACKEND-REQUIREMENTS.md` in full, or `docs/superpowers/`. An architecture for any hole above could exist in one of those. Every "not found" here should be read as "not found by a recorded search", not as proof of absence.
- **Nothing was executed for this axis.** Document reads and greps only. The greps I ran myself are named in the text.
- **The hole-versus-spec-gap boundary is a judgment call.** The closest is supply authority, which could be read as a specification gap if one holds that "amend" in `:68` covers issuance.
- **Two corrections to my own earlier reporting** are recorded in §6: the constructor inventory exists, and the composition modes are designated — the missing piece is the independence premise, not the operator.
- **One cross-report correction.** `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` does not exist in the Moriarty repository; it is a defiformal path. I confirmed by search. Citations to it elsewhere in this directory should carry the defiformal repo and commit, per the path convention above.
