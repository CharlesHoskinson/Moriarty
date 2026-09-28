# DeFi kernel coverage: does the Moriarty language design cover the categories?

**Date:** 2026-09-28
**Status:** specified-only. This is a review, not an accepted change. Nothing here closes a U0 predicate, and no reviewer agreement completes a capability. Nothing was committed.

**Question asked.** Study the DeFi kernel concept at the abstraction level in [defiformal](https://github.com/CharlesHoskinson/defiformal), then check whether the Moriarty language design — principally the [U0 unified proposal](UNIFIED-PROPOSAL.md) — covers each DeFi category, saying for each whether it is covered, where, and what gaps remain.

**Pins.** defiformal `8c5dd103cd40369a763b02b1504441acce0ce3c2` (2026-09-10, clean tree, branch `semantic-kernel-pivot`, a depth-1 shallow clone). Moriarty `8f73784042bd692733c296d0d49f5173be96725e`. The U0 study directory itself is untracked, so `UNIFIED-PROPOSAL.md` is not at HEAD.

**Companion.** This report scores the *implementation* axis: is each category built or specified. The [architecture report](ARCHITECTURE-COVERAGE-REPORT.md) scores the axis that matters more at U0 — whether the roadmap says how each category *will* be built and how the components fit together. It answers more favourably, and it corrects two statements made here by implication. Read it alongside this one.

**Path convention.** Bare paths are Moriarty at `8f73784042bd692733c296d0d49f5173be96725e`. Paths under `lean/`, `algebra/`, `corpus50/`, `quint-models/` and `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` are **defiformal** at `8c5dd103cd40369a763b02b1504441acce0ce3c2`. `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` does not exist in the Moriarty repository; a search confirmed it, and any citation of it as a Moriarty-relative path is wrong.

**Method.** Nine independent read-only reviewers, eight on a category and one on the kernel abstraction itself. Each recorded its own pins, ran the guarded `develop` CLI `status --json`, read both projects, and wrote a report with a per-requirement verdict table. Individual reports are in [`defi-coverage/`](defi-coverage/): R1 AMMs and exchanges (388 lines), R2 lending (405), R3 stablecoins (552), R4 derivatives (522), R5 oracles (408), R6 governance (543), R7 bridges (176), R8 staking and yield (818), R9 kernel abstraction (997).

---

## 1. Headline

**No DeFi category is covered.** The eight category reviewers scored 181 requirements between them: **11 covered, 73 partial, 88 absent, 7 out-of-scope-by-design, 1 named-only, 1 unclassified.** The ninth reviewer scored the kernel abstraction on its own scale — 29 abstractions: 3 expressed, 5 expressed differently (one of them a gain), 8 partial, 12 absent, 1 out-of-scope. **Not one row in any category reached "demonstrated by executed evidence" at category scope.**

That number needs two qualifications, in opposite directions.

Against it: the verdicts are mostly about the *specified* level, not the *designed* level. R9 found that Moriarty's consolidated design already restates all four of the kernel's execution modes in the kernel's own words (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:72`). The design has largely decided it needs the kernel's model. The stage relation cannot yet express it.

For it: this is not the reviewers' arithmetic. Moriarty's own pre-existing register agrees. `experiments/moriarty-language/spec/target-crosswalk.json` holds 104 rows over the same protocol corpus — 32 ACTUS, 72 DeFi — and scores **102 as `unsupported-row-conformance` against 2 `partial-pilot`** (a LAM loan pilot and a constant-product swap pilot). Every one of the 104 rows carries an `evidence_status` of "specified-only; no frontend, conformance run, mathematical proof, native proof, or ledger acceptance". I parsed the file directly to confirm those counts.

The honest summary is that Moriarty today is a **bounded obligation language with one settled slice**, not a DeFi language. It models principal, accrued and outstanding debt well enough that K and TypeScript agree over 104 executed lifecycle cases. Everything that makes a category a category — pools, collateral, supply, positions, observations, policy state, domains — is either absent or a name in a schema with no producer.

## 2. Coverage by category

Counts are each reviewer's own scoring of its own requirement set, so the denominators differ; they are not comparable across rows as percentages.

| Category | Covered | Partial | Absent | Out-of-scope | Verdict |
| --- | ---: | ---: | ---: | ---: | --- |
| AMMs and exchanges (R1) | 2 | 12 | 14 | 0 | Not covered. One CPMM exact-input swap exists, in the legacy profile, fixture-pinned. |
| Lending and borrowing (R2) | 2 | 13 | 21 | 1 | Obligation half covered, collateral half absent. The strongest category. |
| Stablecoins and synthetics (R3) | 0 | 8 | 16 | 1 | Absent. No supply authority at all. |
| Derivatives (R4) | 1 | 8 | 5 | 0 | Not covered. No time type, no position state. |
| Oracles and observations (R5) | 4 | 12 | 6 | 2 | Partial. The stated trust boundary is sharp; the machinery behind it is not. |
| Governance (R6) | 0 | 7 | 7 | 1 | Not covered, roughly half of it deliberately. |
| Bridges and cross-domain (R7) | 1 | 8 | 7 | 1 | Absent as language, partial as design. |
| Staking and yield (R8) | 1 | 5 | 12 | 1 | Least covered. S0 excludes the arithmetic the category runs on. |
| Kernel abstraction (R9) | 3 expressed | 8 partial | 12 absent | 1 | 5 expressed differently, one of them a gain. None demonstrated. |

R6 scored a sixteenth requirement as "named-only" and R9 scored 5 abstractions as "expressed differently"; neither fits the four columns above.

### What exists, precisely

- **Lending obligations.** `LifecycleObligation` with principal, accrued and outstanding, and four actions — Transfer, Repay, Originate, Accrue (`experiments/moriarty-language/src/successor/financial-lifecycle.ts:86-120`). R2 judges this a real debt model, arguably stronger than defiformal's own Lean debt token, agreed by TypeScript and K over the 104 cases in `deliverables/k-lifecycle-execution-2026-09-17/RESULT.md`.
- **One swap.** `spec/examples/swap.mori:75-85`, a two-asset constant-product exact-input swap with a 997/1000 multiplier and a minOut guard, Preview-settled once. It sits in the legacy `moriarty-bounded-atomic/1` profile, and R1 found it fixture-pinned: `swap.mori:79` guards `output_calculated == const.expected_output`, and the repo's own `spec/numeric-profile.json` records `"protocolConformance": false` for it.
- **A periodic schedule.** R4's best positive finding: `financial-lifecycle.ts:36-42` and `:1755-1810` implement `AccrualTerms{periodSeconds, firstPeriodStart}` with `PERIOD_SEQUENCE` (index must be last+1 — no skip, no replay) and `PERIOD_NOT_ELIGIBLE`. It is unilateral, unsigned and fixed-rate, and its `observedTime` is a caller-supplied argument that is not a stage-relation leaf, so it appears in none of the 84 enforcement rows.
- **A sharp stated trust boundary.** R5 rates Moriarty's written boundary (TP03, the product contract, `spec/static-semantics.md:91-94`) as equal to the kernel's.
- **One thing Moriarty is ahead on.** R6 found that UNI-015's hostile scenario rejects an upgrade that "changes beneficiary or resurrects consumed authority" (`spec.md:164-166`) — the in-flight-obligation question that neither defiformal layer states.

## 3. The seven structural gaps

These recur across categories. Each needs new language, kernel semantics, judgment clauses, numeric work or enforcement machinery — none can be closed by writing a library or an example.

1. **No judgment language that states an inequality over pre- and post-state.** `judgments.json` is six prose sentences plus field lists; the U0 study's own finding F5 says so with 6 of 9 reviewers. Without it, `x·y ≥ k` is unstatable — which is exactly why the one swap pins its answer to a constant. R1 rates this as gating AMMs, lending, derivatives and bridges simultaneously. The proposal's S3 task is the fix, and it is not yet written.
2. **No supply authority, and `effects.supplyChanges` has no producer.** The kernel makes `changeSupply(domain, asset)` a right distinct from debit, admin-issued and revocable (`lean/DefiKernel/Typed/Authority.lean:8-12`), checked at execution (`Typed/Transition.lean:139-143`). In Moriarty the leaf exists in the schema (`stage-relation.schema.json:352-375`), is `absent` in both source/5 and core/1, is `NOT_ENFORCED`, is missing from K, and — R8's grep — is read or written by no `.ts`, `.k` or `.py` in the repository. Nothing can mint or burn. This alone blocks stablecoins, LP shares and liquid staking tokens. Owner: the proposal says only "U3, U4 or U6".
3. **No observation value, and no per-observation time.** An observation is `{kind, issuer, domain, time, finality}` — five free strings (`stage-relation.schema.json:239-268`). The kernel's minimum is `{value: PackedValue, timestamp}` with the unit checked at lookup (`lean/DefiKernel/Typed/Types.lean:114-128`). R5 adds that provider authenticity is a caller-supplied boolean required to be `true` (`src/evaluate.ts:128,202`). So freshness, staleness, deviation and any price-dependent decision are inexpressible, which also blocks lending solvency (R2) and derivatives settlement (R4).
4. **No read/write footprint anywhere, so composition cannot be admitted.** R9's executed grep on the stage-relation schema for `read`, `reads` and `footprint` returns zero hits each, and S2 does not add one. The kernel needs footprints for parallel admission, the join, and all frame reasoning (`Typed/Transition.lean:26-28,121-125,148-149`). The four execution modes are named in three Moriarty documents and specified in none; the only composition document declares itself "a PROPOSAL, not registered. No runtime implements this document". Owner: unassigned.
5. **No time type.** `grammar.ebnf:57` is the whole type production, `syntax-profile.json` lists `observation` as unsupported, and both `signedIntent.validity` and `observations[].time` are unconstrained strings. R4: this kills expiry, exercise windows, settlement fixing and latency bounds at once — while `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:83` already demands exactly that distinction.
6. **Capabilities are absent and allowances are not a substitute.** The kernel's capability is issuable, revocable, and scoped to a balance or a supply (`Authority.lean:8-12,21-30,65-85`). Moriarty has three untyped strings (`stage-relation.schema.json:452-470`) read as one value budget. R6 adds the governance consequence: there is no authority *kind*, so "may mint but not spend" and "may amend" are equally inexpressible, though `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:33` demands six kinds including amendment.
7. **The numeric layer cannot type a claim algebra.** `Shares<vault,holder>` exists (`financial-expression-types-v1.ts:29-30,87`) with no `Mul`, `FloorDiv` or `CeilDiv` rule in `arithmeticType`, and `Add` only within one identical type — so a total supply is not summable and the `mulDiv` shape that share accounting runs on is not typeable (R8). Separately, all six division sites remain open conformance gaps with `reserveMechanism.status: "absent"`, which every AMM, accrual and pro-rata computation inherits.

Two smaller gaps with wide blast radius: **asset identity is a bare string** (R3, R7 — wBTC-on-X and canonical BTC are the same identifier today, and `algebra/MODEL.md:128-135` argues no on-chain observation separates USDT from USD1), and **`domain` is a stage-level tag while effect lines carry no domain at all** (R7 — Moriarty can say which domain ran, not that value moved between two).

## 4. Four corrections to the U0 proposal

These are defects in the proposal under review, not coverage gaps. I re-verified each directly; they are recorded in the wiki as CLM-0969 and CLM-0973–CLM-0975.

1. **Law E1 is misstated, and it matters because S3 will hash-bind it.** `UNIFIED-PROPOSAL.md:90` writes "per-asset conservation: Σ gross = Σ supply changes = 0". Its source says "Σ gross = Σ supplyChanges (0 in MSS)" (`opus55-L1.md:155`), and the kernel's law is Σ effect = supply (`Typed/Transition.lean:145-146`), non-zero exactly when minting. R3 and R9 caught this independently; I confirmed both lines. The unqualified form is true only for a slice that mints nothing, so freezing it would make every later issuance program falsify an accepted law. **Restate E1 in the general form with the zero as an S0 consequence.**
2. **The reserve-declaration claim is refuted.** The study's §10 carries a single-reviewer claim that declaring a protocol-reserve type would close all six numeric gaps. It would close one. `conformance_reasons` (`scripts/check_u0_numeric_profile.py:1529-1547`) accumulates reasons, and a reserve declaration removes only "protocol-reserve posting is absent". Five of the six open rows carry `authorSelectable: true`, so "author can select the rounding" survives; only `prorata-principal-share` is `authorSelectable: false` with its fixed direction equal to its required direction. R8 found this; I read the checker and all six rows to confirm. The recommendation not to declare a reserve before the remainder-class decision still stands, for a different reason.
3. **Task N4's blocker was environmental and has cleared.** The study reports that no reviewer could run the evaluator against the repay counterexample because the host `/tmp` was full. `/tmp` now has 32 GB free. Node v22.22.1 on this host cannot import the `.ts` sources at all (`ERR_UNKNOWN_FILE_EXTENSION`; `ERR_NO_TYPESCRIPT` under `--experimental-strip-types`), which is why R1 reported the evaluator unrunnable — but `bun` 1.4.2 is installed and runs it. I ran it: `tests/successor-source-repayment.test.mjs` passes **57 of 57**, and the full suite is **903 pass, 8 fail, 1 error across 46 files in 14 s**. The failures are confined to `tests/lowering.test.mjs` (which needs `@midnight-ntwrk/compact-runtime` resolved through a `.worktrees/r3-native` path that does not exist), `tests/legacy-funded-result-types.test.mjs`, and a README grammar-drift test. No lifecycle or repayment test failed. **N4's evaluator replay is executable now.**
4. **S0's empty `observations[]` freezes a regression.** R5 found that the older atomic profile has observation declarations, genesis provider bindings, evidence digests and an authenticity gate (`src/evaluate.ts:84,128,202-203`), while source/5 — the profile S0 sits on — hard-codes `observations: {}` (`src/successor/financial-agreement-source-compiler.ts:606`) and lists `observation` under `unsupportedDeclarations`. I confirmed both. Presenting the empty binding as a neutral scoping choice is not accurate; it should be recorded as a deferred capability with an owner. R5 also established, and I confirmed by grep, that **no brief given to the nine U0 reviewers mentions oracles or observations** — the study's silence on that surface is an artefact of its briefs.

## 5. Calibration: what defiformal has not built either

Several reviewers insisted on this, and the report would be misleading without it.

- The Lean kernel **does not implement its own Atlas**. R6: `Right = invoke|debit|changeSupply` (`Authority.lean:8-12`), `AuthorityConfig` is immutable, execution steps are only `invoke|issue|revoke`, and none of its 37 openspec sprints is a governance sprint.
- Derivatives (P27), conditional insurance (P28), async lifecycle (P26) and cross-domain workflow (P29) are all **Planned**, with no `lean/DefiKernel/{Margin,Async,ConditionalClaims,CrossDomain}/` on disk (R4, R7). Its atomic settlement fixture deliberately aborts a cross-domain value return as `unsettled`.
- Its own corpus verdict says the corpus is "not a basis for DeFi" (`corpus50/VERDICT.md:12-14`), and lane 2 records that the top three bridges by TVL get identical decompositions — defiformal's own residue.
- R9's warning on citations: `lean/Core.lean` is the four-account pilot, disclaimed at `lean/README.md:33-39`. The real model is `Typed/` plus `Composition/`, `Parallel/`, `Interleaving/` and `Atomic/`.

So the correct frame is: **judge Moriarty against the Atlas laws and the corpus residue, not against a kernel implementation that mostly does not exist.** Where both are missing something, that is a shared open problem, not a Moriarty deficit.

One genuine incompatibility rather than a gap: **prices are inverted between the two projects.** The kernel is quote-per-base (`Typed/Types.lean:40`); Moriarty's decision D1 is base-per-quote. R9 notes the conversion is arithmetic with directed rounding, not a rename.

## 6. Ownership

The reviewers found a consistent pattern: the gaps that would change the language have no live owner.

- **Unassigned by any document read:** read/write footprints and the specification of the four execution modes (R9); authority kind and policy state (R6); periodic funding, margin scope, tranching, instrument listing, state-partition split and merge, and the latency half of finality (R4); order books, auctions, RFQ, routing and MEV, which have no row in the only per-family matrix (R1); the empty-pool bootstrap and virtual-offset counterexample VX01, whose listed owners SP08/SP09/SP11 are provenance aliases under `ROADMAP.md:5` (R8).
- **Assigned but ambiguous:** supply changes read "U3, U4 or U6" in the proposal, and only one source review named U6 (R3).
- **Assigned:** DeFi action rows to U6 (`ROADMAP.md:27,48`); MC07 to U6; TP03 blocks U3.

## 7. What to do before U0-F freezes the contract

The single most actionable finding is that **the schema-v2 task (S2) is the choke point**. Six categories independently found that S2 as proposed closes a door they need, and S2 is scheduled in Phase B, before any of this was known:

- S2 fixes amount *typing* but leaves **authority kind** untouched, foreclosing UNI-015, DA22 and recovery authority (R6).
- S2 proposes **canonical non-negative integers**, which closes the only place a signed margin position could live (R4).
- S2 does not add a **read/write footprint**, without which no composition mode can be admitted later (R9).
- S2 leaves **asset identity** a bare string, so no category can distinguish a wrapped asset from its canonical form (R3, R7).
- Several leaves that S3 clauses must evaluate — `observations[].finality`, `observations[].domain`, all three `failurePolicy` fields — are **free strings with no enum**, and the proposed clause language is limited to linear integer arithmetic and set membership over relation paths, so no clause can evaluate them (R7).

Recommended sequencing change, for the owner:

1. **Correct E1 before S3**, per §4.1. This is small and it prevents freezing a false law.
2. **Add four decisions to Phase A**, alongside the six the U0 study already lists: authority kind; amount signedness; asset identity structure; and whether every leaf an S3 clause must evaluate carries an enum or a typed union. All four are cheap now and expensive after U0-F.
3. **Run N4's evaluator replay now** — it is executable, per §4.3, and it closes the study's single biggest evidence gap.
4. **Give the observation surface an owner** rather than an empty binding, per §4.4.
5. **Leave S0 alone.** Every reviewer who considered it agreed S0 is correctly scoped; R6 explicitly recommends no S0 change. The gaps above are about what the *contract* forecloses, not about the slice.

## 8. Limits of this review

- **Nothing in any category was demonstrated.** No reviewer executed a Lean build, a K run, a U0 checker, a proof or a network call. Every verdict rests on reading source and schemas, plus the guarded `status --json`.
- **The counts are not commensurable.** Each reviewer built its own requirement set, so the §1 totals are a sum of eight different denominators and must not be read as a percentage. R9's kernel-abstraction scale is reported separately for the same reason. One arithmetic discrepancy is unresolved: R4 states 15 requirements but its four verdict counts total 14, so one derivatives row is unaccounted for in the summary and is included above as "unclassified".
- **The defiformal checkout is a depth-1 shallow clone**, so the pin `33b9a955` cited by `deliverables/defiformal-study-2026-09-19/libraries-review.md:3` could not be resolved and that study's citations could not be re-verified this run (R8).
- **The four corrections in §4 are the only claims I verified personally.** Everything else is reviewer-reported with the anchors they cite; I did not re-check every file:line in 4,809 lines of reports.
- **One kernel finding is itself undecided:** the Atlas's `Dp` directional-position element is a candidate whose promotion gate is "evidence it is not Pf plus Ct plus parameters". If it demotes, R4's first derivatives requirement weakens to a composition.
- This report reviews a proposal against a model. Neither side is deployed, and no coverage verdict here implies anything about Midnight acceptance.
