# R1 — DeFi Kernel coverage review: AMMs and exchanges

Reviewer R1 of nine. Read-only review. This file is the only artifact written.

Status: independent review finding. It accepts nothing, closes no U0 predicate and
authorizes no work. Under `AGENTS.md:97-99` work is accepted by evidence, not by review.

---

## 1. Scope and pins

**Commits (both recorded by executed command).**

| Repository | Command run | Output |
|---|---|---|
| DeFi Kernel | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` (`U0 T7: add U0 exit gate`, Wed Sep 23 21:00:14 2026 -0600) |

**Startup.** The host did not expose `moriarty-dev:develop`, so I read and applied the
checked-in skill at `plugins/moriarty-dev/skills/develop/SKILL.md`, plus `AGENTS.md` and
`docs/FOOTGUNS.md`, before forming conclusions (`AGENTS.md:10-12`).

**Guarded CLI status — executed.** I ran
`python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json`.
Actual output: `capability: SP01.6 loan-swap-subset`; `blockedAction:
implementation/repair of loan-swap-subset`; `nextAction: sp01-loan-report`;
`missingEvidence` = binding-input-stale and candidate-input-stale on
`openspec/sprints/sp01-financial-contract-and-execution-admission.md`,
`current-accounting-missing:.moriarty-dev/runtime/current-accounting.json`,
`resource-live-state-unavailable:sp01-loan-swap-grok-01`, `operational-history`;
seven pending transaction IDs. I dispatched no campaign and ran no `next` or `run`.

**What I read on the defiformal side.** `README.md`; `docs/UNIFIED-DEFI-ELEMENT-TABLE.md`
(§1–§18, in full for §2–§4, §8–§18.2); `algebra/MODEL.md`; `algebra/REQUIREMENTS.md`;
`openspec/ROADMAP.md`; `corpus50/VERDICT.md`, `corpus50/decomp-contract.md`,
`corpus50/lanes/lane1-dex-lending-cdp-lsd.json` (Spot DEX / AMM category); the Lean
kernel structure definitions in `lean/DefiKernel/Typed/Types.lean`,
`Typed/Transition.lean`, `Typed/Authority.lean`, `Atomic/Policy.lean`,
`Atomic/Settlement.lean`, `ConcentratedLiquidity/{Types,SqrtPriceMath}.lean`, and the
`Arithmetic/` module listing.

**What I read on the Moriarty side.** `ROADMAP.md`; `docs/MORIARTY-PRODUCT-CONTRACT.md`;
`docs/MORIARTY-CONSOLIDATED-DESIGN.md`; `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`;
`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (all 288 lines);
`deliverables/u0-semantic-contract-2026-09-23/{EXIT-GATE.md, judgments.json,
numeric-profile.json, enforcement-map.json}` and part of `source-core-embeddings.json`;
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`
(top-level properties); `experiments/moriarty-language/spec/successor/
financial-agreement-source-v5-grammar.ebnf` and `financial-agreement-source-v5.md`;
`experiments/moriarty-language/spec/numeric-profile.json`;
`experiments/moriarty-language/spec/examples/swap.mori`;
`experiments/moriarty-language/compact/generated/swap/{kernel.compact,metadata.json}`;
`src/successor/{financial-lifecycle.ts (extracts), financial-expression-v1.ts (extracts),
expression-source-lower.ts, financial-expression-source-lower.ts,
financial-expression-source-v1.ts (extracts), frontend.ts (extracts)}`;
`formal/k/` module listing and the `FloorDiv`/`CeilDiv` rules in `lifecycle-v1.k`,
`expression-infer.k`, `expression-shape.k`; `deliverables/defi-language-design-2026-09-07/
{LANGUAGE-DESIGN.md, action-targets.csv}`; `deliverables/modern-defi-taxonomy-2026-09-08/
{categories.json, category-coverage.json, CATEGORIES.md}`;
`deliverables/defi-report8-comparison-2026-09-09/` listing plus greps;
`wiki/defiformal-taxonomy.md` (greps), `wiki/defi-kernel-protocol-graph.md`,
`wiki/defi-kernel-sdk-interface.md`; `deliverables/sp05-financial-integration-2026-09-09/
preview-swap-exit-01/RESULT.md`; `docs/research/2026-09-06-actus-defi-design-study.md:93-115`.

**What I executed, and what I did not.**

- Executed: the two `git` commands above; the guarded `status --json`; `node --version`
  (`v22.22.1`); file reads and greps.
- Executed and **failed**: `node experiments/moriarty-language/src/cli.ts check --profile
  moriarty-financial-agreement-source/5 .../loan-lifecycle.mori` →
  `ERR_UNKNOWN_FILE_EXTENSION` for `.ts`; retry with `--experimental-strip-types` →
  `Error [ERR_NO_TYPESCRIPT]: Node.js is not compiled with TypeScript support`. **This
  session therefore executed no Moriarty evaluator, checker, or test.** Every behavioural
  claim below about the TypeScript evaluator is read from source, not observed.
- Not run: any Lean build or `lake` command; any U0 checker script; any test suite; any
  campaign dispatch (`next`, `run`); any network request or Preview submission.
- Not edited: any repository source. No commit.

---

## 2. What the kernel abstraction requires for this category

The Atlas puts AMMs and exchanges in three of its fifteen groups, and the group boundary
is a *role* question, not a product name.

- **G02 Pool pricing — "how is a price derived from inventory?"** Five core elements:
  `Cp` constant product, `Wg` weighted-geometric, `St` stable-hybrid, `Cl` concentrated
  liquidity, `Pm` oracle-priced inventory curve
  (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:178-186`). Constant sum is **not** an isotope of
  `St` because an isotope must preserve the *failure family* and constant sum's does not:
  at parity break the pool is fully drainable on one side, "a qualitative change of state
  space, not a parameter move" (`:190-196`). It is written `St{→CSM}` and carries the
  mandatory `drain_regime` discriminator (`:510`).
- **G03 Execution — "how is a trade matched and cleared?"** `Ob` on-chain order book,
  `Rf` request for quote, `Ba` batch-auction clearing, `In` intent and solver execution
  (`:198-205`). All four carry a **mandatory** `market_structure` discriminator over
  `{open, gated, exclusive, vertically-integrated}` (`:207-211`, `:508`), because
  "solver-set size, exclusivity windows and vertical integration are *design* choices, not
  weather, and they determine who captures the informational rent."
- **G04 Liquidity catalysis.** `Ag` aggregation and routing; `Fl` atomic flash liquidity,
  the only element marked async-**impossible** (`:215-226`, `:852-856`). `Fl` is stated
  over an *abstract atomic settlement scope*, not an EVM transaction (`:223-226`).
- **G01 Claims.** `Sh` pro-rata share accounting is the LP claim primitive (`:170`).

The abstractions those elements sit on:

| # | Kernel/Atlas abstraction this category depends on | Citation |
|---|---|---|
| K1 | **Template** = signature + domain + party arity + `guard` + `deltas` (cell deltas) + `supplyDeltas` + declared `stateReads`/`envReads`/`writes`. A swap is one registered template with a guard and a declared read/write footprint. | `lean/DefiKernel/Typed/Transition.lean:19-28` |
| K2 | **Capability classes**: a request carries `capabilityIds`; capabilities grant permission to *invoke* an operation, *debit* a particular balance, or *change an asset's supply*, and can be issued and revoked. Refusals are typed: `unauthorizedInvoke`, `unauthorizedDebit`, `unauthorizedSupply`. | `README.md:35-38`; `Typed/Transition.lean:44-60`; `Typed/Authority.lean:8,21,28,65,77` |
| K3 | **Supply authority** is a first-class, separately authorized effect. LP-share minting and burning are supply changes, not transfers. | `README.md:35-38,41-43`; `Typed/Transition.lean:14-17,25` |
| K4 | **Conservation law**: "For each asset in each domain, the total balance change must equal declared minting minus burning", and resulting balances must stay nonnegative. | `README.md:41-43`; `Typed/Transition.lean:139-148` (`suppliesOK`, `accountingOK`, `debitsOK`) |
| K5 | **Exact rationals in the ledger** (`ℚ` deltas and supplies), with asset-specific amount types — "a share amount and a dollar amount cannot be interchanged without an explicit conversion". | `README.md:23-27`; `Typed/Transition.lean:65-66` |
| K6 | **Bounded-width word arithmetic as a separate substrate for `Cl`**: `U128/U160/U256`, `Q96 = 2^96`, an explicit `Failure` enum (`divisionByZero`, `quotientOverflow`, `subUnderflow`, `addOverflow`, `uint160Overflow`), `divRoundingUp`, `wrap256`, `bareToUint160` truncation vs checked `toUint160`. | `lean/DefiKernel/ConcentratedLiquidity/Types.lean:6-18`; `SqrtPriceMath.lean:30-47` |
| K7 | **Directed rounding with a named remainder recipient**: "Direction of rounding always favours the pool"; and the empty-pool mint invariant / `Sh{offset}` for first-depositor share inflation. | `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:896-897` |
| K8 | **Four execution modes with explicit failure behaviour** (sequential, disjoint parallel, shared-state interleaving, atomic), chosen to match intended ordering and rollback. | `README.md:51-58,145-148` |
| K9 | **Atomic settlement policy** for batch clearing: named vault lanes and tracked participants, per-caller net obligations recorded from movements at those accounts, commit only when every tracked obligation is zero and supply checks pass; the schedule is an *input*, so one run does not explore all orderings. | `README.md:60-64`; `lean/DefiKernel/Atomic/Policy.lean:8-29,56,79-89` |
| K10 | **Ports and components**: named typed inputs/outputs, private and shared state; a workflow feeds a recorded output into another call and its trace records refusals. | `README.md:45-48` |
| K11 | **Atomicity spectrum**: same transaction / same block / shared sequencer / same-ecosystem / cross-ecosystem / optimistic, each with a named repair. "Anything marketed atomic below 'same transaction' must name its repair bond." | `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:841-856`, `:1025` |
| K12 | **Four separately satisfied bond types**, including the **informational** bond `—n→`: "who observes amount, urgency, route, constraints or wallet state before settlement?", failing as adverse selection and order-flow capture. Every `—n→` edge must carry a tag from a **closed catalog**: `adverse_selection · frontrunning · orderflow_capture · last_look · exclusivity_rent · sandwich · liquidation_targeting · deanonymization`. | `:530-540`, `:553-557` |
| K13 | **Required-bond law L24**: `(In\|Rf\|Ba) →` an explicit `—n→` edge with a catalog tag. **L29**: `(In\|Ba\|Rf\|Of) →` a declared **surplus-allocation rule naming the residual claimant**. L12: `In →` signed constraints + settlement verifier + (solver\|fallback) + timeout. | `:752`, `:764`, `:769` |
| K14 | **Valence is computed from the laws**, not asserted. `Cp`'s vector is: interface = token interface; economic = arbitrage depth; informational = **mempool visibility**. | `:579-590` |
| K15 | **Reaction conditions** bound the operating range: "Mempool and MEV regime" is a mandatory condition row for `Cp Cl Ob Ba In Fl`; "Executable market depth" for `Li Rd Ps Ct`. Each row touching a present element needs a stated bound or an explicit "unbounded". | `:938`, `:940`, `:963` |
| K16 | **Hazard rules** as global constraints: X2 `Fl*` + manipulable `Cp`/`Cl` price + `Pl`/`Cd` **where manipulation cost < position value**; X5 `drain_regime=fully-drainable` with unbounded inventory; X10 `Pm` with stale reference and unrestricted inventory; X14 `market_structure=exclusive` plus a price-improvement claim with **no named benchmark or commitment device**. Screen row 4: "Price the manipulation against *pool depth*, not the attacker's balance." | `:790`, `:793`, `:798`, `:803`, `:1023` |
| K17 | **Residue is a mandatory per-molecule finding** — "what the vocabulary could not express about this protocol" — and recurring residue is the primary evidence for a new element. | `:729-733` |
| K18 | **Iteration/convergence and refusal** are owned kernel work for the curve family (Curve), separately from the CL library and the Balancer vault/hooks/transient accounting. | `openspec/ROADMAP.md:50-51,54` (P21, P22, P25) |

**The kernel's own verdict on this category's residue** is the strongest single input, and
it is empirical rather than aspirational. Lane 1 of the 69-protocol benchmark decomposed
the top-5 spot DEX/AMMs and recorded, as category residue: (1) no element for a **pluggable
hook / extension point that executes third-party code inside the swap settlement path**;
(2) no element for **singleton-vault flash accounting** — "run arbitrary operations, then
settle the net"; (3) no element for **non-fungible per-range LP position accounting** with
its own fee-growth checkpoint; (4) no element for a **fee-tier / pool-parameter registry**;
(5) no element for the **CryptoSwap repegging invariant**, for an **auto-rebalancing range**,
or for **a position that is simultaneously credit and AMM inventory**
(`corpus50/lanes/lane1-dex-lending-cdp-lsd.json`, "Spot DEX / AMM" category, per-protocol
`residue` arrays and `category_residue`). The overall verdict is
`corpus50/VERDICT.md:12-14`: "the vocabulary is a good vocabulary of on-chain state
machines and it is not a basis for DeFi. Not one of the 69 protocols was fully
expressible." DEX lane coverage is recorded at `~73% clean, 13% forced, 13% none`
(`corpus50/VERDICT.md:20`).

**The kernel's own Lean implementation of this category is thin**, and that bounds what
"compare against the kernel" can mean. I grepped `lean/` for `swap`, `amm`,
`constant.?product` and `pool`: the only `constant-product` hit is a data row in
`lean/DefiHistorical/Convex/Data.lean:47` (`E004 Cp`), the only `swap` hit is
`Data.lean:95` (`E044 Ps`), and there is **no AMM or swap `Template` anywhere in
`lean/DefiKernel/`**. What exists is `lean/DefiKernel/ConcentratedLiquidity/`, 1,305 lines
of unsigned token0/FullMath word arithmetic; `openspec/ROADMAP.md:50` records P21 —
"Residual concentrated-liquidity library, token1/delta, TickMath, SwapMath,
bitmap/factory, full fixtures/mutations, and tick traversal" — as **Open**. So for this
category the kernel is a *specified model* plus an arithmetic substrate, not a worked
pool model. I compare Moriarty against the model, as instructed.

---

## 3. Coverage verdict per requirement

Legend for the maturity words, used strictly:
**designed** = a document states the intent; **specified** = a versioned schema, grammar or
profile fixes it; **implemented in the evaluator** = TypeScript/Core code computes it;
**demonstrated by executed evidence** = a run recorded in the repository produced it.
I executed nothing this session (§1), so every "demonstrated" below is a *repository record
I read*, not a run I observed.

| # | Requirement (from §2) | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| R1 | **CPMM exact-input price from inventory** (`Cp`, K1) | **partial** | `experiments/moriarty-language/spec/examples/swap.mori:75-85` computes `floor_div((dx·997)·reserve_b, reserve_a·1000 + dx·997)`; the Core type rule `AmountProduct / Amount → Amount` is at `src/successor/financial-expression-v1.ts:344-350`; a recorded test asserts the formula yields `19743` at type `Amount<B>` (`tests/financial-expression-source-v1.test.mjs:113`); the lowered circuit is `compact/generated/swap/kernel.compact:53-70` | `swap.mori` is in the **legacy** `moriarty-bounded-atomic/1` profile, not the current `/5` (`swap.mori:1` vs `spec/successor/examples/*.mori` profiles). It is fixture-pinned: `swap.mori:79` `guard output_calculated == const.expected_output`, and the profile's own numeric record says `"protocolConformance": false` and `"scope": "fixed-vector one-transition fixture; expected_output prevents reuse for a second input"` (`spec/numeric-profile.json`, `firstExamples.swapOutput`). No pool state or swap operation exists in `/5` (`spec/successor/financial-agreement-source-v5.md:3-5`: four protected operations Transfer, Repay, Originate, Accrue) |
| R2 | **Invariant preservation as a checkable property** (`x·y ≥ k`, K1) | **absent** | — | No judgment states any inequality or conservation. `deliverables/u0-semantic-contract-2026-09-23/judgments.json` has six judgments, all prose field lists. `UNIFIED-PROPOSAL.md:36` (F5, 6/9 reviewers): "The six judgments are prose. None states conservation, a cap inequality, non-negativity, liability roll-forward or replay freshness." The proposed executable clauses (S3) are **design intent only** (`UNIFIED-PROPOSAL.md:4,119`) |
| R3 | **Concentrated liquidity**: ranges, ticks, per-position fee growth, sqrt-price (`Cl`, K6) | **absent** | `deliverables/defi-language-design-2026-09-07/action-targets.csv:4` (DA03) records disposition `needs-pinned-protocol-fixture` | No tick, range, position identity, sqrt-price or Q96 anywhere. I grepped for `sqrt` across `experiments/moriarty-language/src`, `spec`, `docs/` and `ROADMAP.md`: **zero hits**. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` concedes the general principle: "Narrow token0 pricing is not a full AMM" |
| R4 | **LP share issuance and redemption**, pro-rata (`Sh`, K3) | **partial** | `shares<T,P>(...)` is a source constructor (`financial-agreement-source-v5-grammar.ebnf:118`), typed at `financial-expression-v1.ts:379-380` (`expression-checked-construct-shares` in `numeric-profile.json`); `spec/successor/examples/financial-vault-quote.mori:6-9` computes `floor_div(deposit·supply, valuation)` at UInt256 | No share **supply** is tracked; no mint/burn; no empty-pool/first-depositor invariant (K7); no `sqrt` for initial CPMM mint; no `min` over two ratios for subsequent mints. `action-targets.csv:3` (DA02 provide/remove liquidity) disposition: `needs-extension` |
| R5 | **Supply authority as a separate capability class** (K2, K3) | **absent** | The stage relation carries `effects.supplyChanges[].{account,amount,asset}` (`judgments.json`, effect judgment) | I grepped `src/successor/*.ts` for `'Mint'`, `'Burn'`, `supplyChange`, `mintAuthority`: **zero hits**. `/5` has four protected operations and none changes supply (`financial-agreement-source-v5.md:3-5`). The schema leaf exists with no producer and no enforcement: all 84 leaves are `NOT_ENFORCED` (`enforcement-map.json`, 84/84; `EXIT-GATE.md:12,24`) |
| R6 | **Invoke / debit capability classes and typed refusals** (K2) | **partial** | Source reads `allowance_remaining`, `allowance_spent` (`v5-grammar.ebnf:124-126`); the authority judgment covers `authority.{consumed,remaining,replayState}` and `resources.*` (`judgments.json`); signed intent carries `grossDebitCap`, `feeCap`, `minNetOutcome`, `recipients[]`, `assetIdentities[]` (`judgments.json`, intent judgment) | Authority is prose, not clauses (F5). No supply-class capability (R5). Nothing binds any of these natively: `enforcement-map.json` row `authority.remaining` notes the generated loan and swap kernels "assert `remaining > 0` on a program-local lifetime counter, at loan `kernel.compact:15` and swap `kernel.compact:15`. That counter is not this stage-relation field" |
| R7 | **Per-asset conservation** (K4) | **partial (designed only)** | `UNIFIED-PROPOSAL.md:90` law E1: "per-asset conservation: Σ gross = Σ supply changes = 0"; `:91` E2: "net is derived from gross and fees" | E1/E2 are a **proposal**, explicitly "specified-only … not an accepted change" (`UNIFIED-PROPOSAL.md:4`). The frozen `judgments.json` states no conservation. `UNIFIED-PROPOSAL.md:35` (F4, 3/3 L1 reviewers): "The schema admits financially invalid instances. It allows negative amounts, one-account gross lines …". A one-account gross line is exactly an unbalanced swap leg |
| R8 | **Directed rounding with a named remainder recipient**; rounding favours the pool (K7) | **partial** | `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json`, `defaultPolicy`: `obligation: ceil`, `receipt: floor`, `exact: none`, `remainderBeneficiary: protocol-reserve`; 17 primitives inventoried; the legacy profile requires a six-field policy per Amount-valued target (`spec/numeric-profile.json`, `fieldPolicyRequired`: unit, derivation, roundingNode, remainderDisposition, comparisonPolicy, proofStatement), instantiated at `swap.mori:27-42` | `reserveMechanism.status` is `"absent"` with empty citations: "Every ceil or floor primitive is an open conformance gap because the rounding remainder has nowhere to post." Six of seventeen primitives are `open-gap` (`EXIT-GATE.md:9,21`). There is **no pool party**, so "rounding always favours the pool" is not statable. `Conversion.rounding` is required to be both floor (origination) and ceil (repayment) at one call site (`UNIFIED-PROPOSAL.md:149,277`, citing `financial-lifecycle.ts:1376`) |
| R9 | **Slippage limit / minimum output** | **covered (at intent and program level)** | `signedIntent.minNetOutcome` is an intent-judgment field (`judgments.json`); `swap.mori:80` `guard arg.min_out <= output_calculated`; lowered to `compact/generated/swap/kernel.compact:71-74` `assert(e46, "minimum output not met")` | Enforcement is not established: `enforcement-map.json` is 0/84 and its `limitation` says the generated kernels "constrain anonymous Uint fields of two restricted Core programs. They do not name a canonical stage-relation leaf" |
| R10 | **Price-impact limit priced against pool depth** (K16, screen row 4) | **absent** | — | Nothing in Moriarty references pool depth, manipulation cost, or a bound on price impact. Grep for `price impact`, `slippage` across `docs/`, `ROADMAP.md`, `UNIFIED-PROPOSAL.md` returns only two historical mock-plan lines (`docs/superpowers/plans/2026-09-06-developer-mock-implementation.md:25,51`) and one design-study row |
| R11 | **Fee tiers** as pool parameters with an LP beneficiary (K7) | **partial** | `swap.mori:9-10` `fee_numerator 997 / fee_denominator 1000` as program constants; `signedIntent.feeCap` and `effects.fees[]` in the stage relation (`judgments.json`) | A fee tier is a *registry* choice with a beneficiary and an LP fee-growth accrual; Moriarty has only a literal constant and a payer-side cap. S0 excludes computed fees entirely: "A literal fee needs no rounding. A computed fee is outside S0" (`UNIFIED-PROPOSAL.md:73`) |
| R12 | **Exact-output swaps** (reverse direction, ceil rounding) | **partial (specified, no example)** | The type rule admits both directions — `AmountProduct / Amount` returns the *other* asset (`financial-expression-v1.ts:345`), and `CeilDiv` exists in Core (`:344`), in K (`formal/k/lifecycle-v1.k:156`) and as source `ceil_div` (`src/successor/expression-source-lower.ts:53-56`) | No exact-output example, fixture or test anywhere; no statement of which direction each leg must round |
| R13 | **On-chain order book** (`Ob`, K-G03) | **absent** | Named only as a taxonomy mechanism: `deliverables/modern-defi-taxonomy-2026-09-08/categories.json`, `FIN-EXC.mechanisms` = `["AMM","order book","RFQ","batch auction","solver routing"]`; `CATEGORIES.md:29` | No language construct for resting orders, price-time priority, cancellation, partial fills against a book, or a matching step. `action-targets.csv` has **no row** for order books (I read all 24 rows). No openspec requirement: grep of `openspec/changes/consolidated-language-kernel/*.md` for `order.?book`/`auction`/`amm` returns nothing |
| R14 | **Batch-auction / uniform clearing** (`Ba`, K9) | **absent** | — | No multi-party stage, no clearing price, no lane/participant net-obligation abstraction. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:72` names "Atomic publication applies only within a declared domain and its actual phase semantics" as a specified-only obligation. The kernel's `Atomic/Policy.lean:8-29` (lanes, participants, `zeroOutstanding`, `checkSupply`) has no Moriarty counterpart |
| R15 | **RFQ / intent-and-filler** (`Rf`, `In`, L12) | **partial (designed)** | Signed outcome intents with gross debit, fee and liability bounds, recipients, minimum net outcomes, validity, replay and revocation, and allowed partial completion (`docs/MORIARTY-PRODUCT-CONTRACT.md:41`); solver proposal without a privileged bypass (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:29`); two authorization modes (`deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:128`) | This is `In`-shaped only. No maker quote, no `last_look`, no filler inventory or solvency, no `Of` optimistic advance. L12's "solver \| fallback + timeout" is not a typed obligation. `action-targets.csv:22` (DA21) disposition: `local restricted profiles only` |
| R16 | **`market_structure` as a mandatory discriminator** (K-G03, X14) | **absent** | — | No construct records whether the solver set is open, gated, exclusive or vertically integrated. Nothing anywhere in Moriarty names solver-set exclusivity as a *design* fact of a program |
| R17 | **Aggregation and routing** (`Ag`, K-G04) | **partial (designed)** | `ROADMAP.md:38`: "Two independently produced candidates may supply different authorized routes … this checks objective route neutrality without U5 infrastructure"; `docs/MORIARTY-CONSOLIDATED-DESIGN.md:29` solver search preserves hard constraints; U5 owns adapters (`ROADMAP.md:26`) | No representation of a multi-hop or multi-venue route as a typed object, no venue identity, no per-hop accounting. `wiki/defi-kernel-sdk-interface.md:13,19` places the router in *optional managed-service research*, explicitly not the language interface |
| R18 | **Atomic flash liquidity / net-settled multi-leg scope** (`Fl`, K11) | **absent** | `action-targets.csv:9` (DA08 "flash borrow / repay atomically") disposition: `needs-normative-atomicity-fixture` | No borrow-and-repay-within-one-scope construct; no deferred net settlement. This is also the kernel lane's #2 DEX residue ("singleton-vault flash accounting") and it blocks expressing hazard X2 at all |
| R19 | **Informational bond — who observes the order before settlement** (K12, L24) | **absent** | The stage relation has `disclosures[].party` and `disclosures[].fields[]` (`judgments.json`, intent judgment) | `disclosures` is a *privacy* permission (what a party may see of the accepted stage), not a pre-settlement observation channel. I grepped `docs/`, `ROADMAP.md` and `UNIFIED-PROPOSAL.md` for `mev`, `front.?run`, `sandwich`, `order.?flow`: **zero hits** in all three. There is no catalog of `adverse_selection / frontrunning / orderflow_capture / last_look / exclusivity_rent / sandwich`, and no obligation to declare one |
| R20 | **Surplus-allocation rule naming the residual claimant** (L29) | **absent** | — | Moriarty binds the *payer's* caps (`grossDebitCap`, `feeCap`) and the *recipient's* floor (`minNetOutcome`) (`judgments.json`), but nothing states who receives the difference when a solver beats the floor. `docs/FOOTGUNS.md:198-199` states the adjacent rule ("a gross receipt before fees is not the net promised delivery"), which is a payer-protection rule, not a residual-claimant rule. This is a **required-bond law** in the Atlas (`:769`), so its absence makes any `In`/`Rf`/`Ba` molecule ill-formed by the kernel's own grammar |
| R21 | **MEV / ordering as a bounded reaction condition** (K15) | **absent** | — | No construct bounds or even names the mempool/MEV regime. Nearest adjacent design text is `docs/MORIARTY-CONSOLIDATED-DESIGN.md:32` ("Distinguish atomic batch, committed prefix, independent fork/join and interleaving") — ordering as a *composition* question, not as an adversarial-extraction bound |
| R22 | **Exact rationals vs bounded-width words** (K5, K6) | **partial** | Widths `UInt64/UInt128/UInt256/SInt128` (`numeric-profile.json`, `units.widths`); `fieldElementCoercion: forbidden`; `to_uint<256>` widening (`v5-grammar.ebnf:121`); checked add/sub/mul with `ARITH_RANGE` and `ARITH_DENOMINATOR` (`financial-expression-v1.ts:397-407`); legacy profile requires `reject-overflow-even-when-a-later-quotient-would-fit` (`spec/numeric-profile.json`, `integer.widthBoundary`) | No `mulDiv` with a wide intermediate (the CL substrate needs 512-bit intermediates and `Q96`); no fixed-point sqrt; no signed tick arithmetic. The legacy profile lists `signed-values-and-negative-rounding` and `wider-intermediates` under `extensionsRequiringNewVersion`. Amounts are integers, not `ℚ`: the kernel's exact-rational ledger and Moriarty's integer ledger are different objects and the crosswalk is not written |
| R23 | **Dimensional typing of the swap formula** (K5) | **covered (implemented in the evaluator; demonstrated by a recorded test I did not execute)** | `financial-expression-v1.ts:335-357`: `Mul` on two `Amount`s yields `AmountProduct[a,b]` (`:340`); `FloorDiv`/`CeilDiv` of `AmountProduct` by one of its `Amount`s yields the *other* `Amount` (`:345`); `Quantity ∘ Quantity` adds/subtracts unit exponents (`:351-356`). `spec/numeric-profile.json`, `units`: `multiplication: add-unit-exponents`, `division: subtract-unit-exponents` | This is the one place where Moriarty genuinely meets a kernel requirement the kernel itself does not implement in Lean. Caveat: `ScaledAmount` descaling requires the divisor to be a **literal** `10^scale` (`:347`, `TYPE_SCALE_DIVISOR`), so a state-dependent price divisor is not descalable — a pool price is state-dependent by definition |
| R24 | **Iteration with a convergence and tolerance policy, and refusal on non-convergence** (K18, StableSwap/`St`) | **absent** | `spec/numeric-profile.json`, `extensionsRequiringNewVersion` lists `bounded-convergence-with-iteration-and-tolerance-policy` and `user-functions-recursion-or-loops` | The `/5` grammar has no loop or fold: `statement = requirement \| binding \| update \| emission` (`v5-grammar.ebnf:55`). `St`, `Wg` (weighted-geometric needs a power/root) and `Pm` are therefore inexpressible. Kernel sprint P22 owns this on the defiformal side (`openspec/ROADMAP.md:51`) |
| R25 | **Hooks / pluggable settlement logic** (lane-1 category residue #1) | **absent** | — | Not named anywhere in Moriarty. The Atlas rules it out as an element ("Hooks and callbacks are bonding sites", `:148-152`), so this is a residue finding on *both* sides — but it is the single feature the top three AMMs differentiate on |
| R26 | **Residue discipline per program** (K17) | **partial** | `deliverables/defi-language-design-2026-09-07/action-targets.csv` is a 24-row coverage matrix with an explicit `implementation_disposition` per row; `docs/FOOTGUNS.md:118-124` requires "Keep the complete target coverage matrix visible" | The matrix is a *family*-level register, not a per-program residue line, and it is dated 2026-09-07 — it predates `/5`, the U0 contract and the consolidated roadmap. Nothing requires a new Moriarty program to declare what its model does not express |
| R27 | **Four execution modes / atomicity spectrum placement** (K8, K11) | **partial (designed)** | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:72`: sequential retains committed prefixes; disjoint parallel requires checked independence and complete joining; shared-state interleaving requires interference reasoning; atomic publication is domain-scoped. `action-targets.csv:25` (DA24) disposition: `specified-only`. Midnight guaranteed/fallible phases are a separate, mandatory model (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`) | Five named operators, none implemented; no counterpart to the kernel's schedule-as-input caveat (`README.md:64`); nothing places a program on the atomicity spectrum or names its repair bond (K11) |
| R28 | **Price orientation crosswalk to the kernel** | **partial (specified, unexercised)** | `numeric-profile.json`, `defiformalConversion`: source `quote-per-base` → target `base-per-quote`, formula `10^(s+t)/M` with floor for receipts and ceil for obligations, plus four test vectors; restated at `docs/MORIARTY-CONSOLIDATED-DESIGN.md:52` and `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:57` | The four vectors are recorded expectations, not observed outputs — `UNIFIED-PROPOSAL.md:154` (N3) exists precisely to add an `observed.json`, which "turns recorded gaps into demonstrated gaps and closes nothing". An AMM price is derived from *inventory*, not quoted, so this crosswalk does not by itself carry a pool price |

**Counts (28 requirements): covered 2 · partial 12 · absent 14 · out-of-scope-by-design 0.**

Of the 2 `covered`, one (R23 dimensional typing) rests on a recorded test I did not run,
and one (R9 minimum output) is covered at the program and intent level while being
`NOT_ENFORCED` at the native level like all 84 leaves.

Three strictness notes, applied above and worth restating:

1. **A name in a schema is not coverage.** `effects.supplyChanges` (R5) and `disclosures`
   (R19) appear in the stage relation and neither has a producer, a clause or an
   enforcement locus.
2. **A checker exit of 0 is not a capability.** All seven U0 checkers exit 0
   (`EXIT-GATE.md:20-26`) while every capability line reads `OPEN` and the enforcement map
   reads `0 enforced, 84 NOT_ENFORCED`. `UNIFIED-PROPOSAL.md:264` states the same rule.
3. **A proposal is coverage of design intent only.** Everything cited from
   `UNIFIED-PROPOSAL.md` — including the S0 laws E1/E2 (R7) and the executable-clause plan
   S3 (R2) — is marked "specified-only … not an accepted change" at `:4`.

---

## 4. Category verdict

**AMMs and exchanges are the weakest-covered of the categories Moriarty names as targets,
and the shortfall is concentrated in the two halves of the category that are not
constant-product pricing.** Moriarty can express one constant-product exact-input swap
with unit-checked dimensional typing, a floor-rounded quotient, a minimum-output guard and
two transfer effects, and it carried exactly that program to Midnight Preview once
(`deliverables/sp05-financial-integration-2026-09-09/preview-swap-exit-01/RESULT.md:5-12`:
trader spends 10,000 A for 19,743 B; provider closes at 1,010,000 A and 1,980,257 B).
That is real and it is narrow: the program lives in the legacy `moriarty-bounded-atomic/1`
profile, its output is pinned to a constant so it cannot be reused for a second input
(`spec/numeric-profile.json`, `firstExamples.swapOutput.scope`), its pool has exactly one
liquidity provider and no shares, and the current `/5` profile that the U0 contract is
written against has no pool, no swap and only four protected operations
(`financial-agreement-source-v5.md:3-5`). Beyond that one shape: no invariant judgment, no
concentrated liquidity, no LP share supply, no order book, no batch clearing, no RFQ, no
flash scope, no market-structure discriminator, no informational-bond declaration and no
surplus-allocation rule. Two of those absences — L24 and L29 — are *required-bond laws* in
the Atlas (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:764,769`), which means a Moriarty
intent-and-solver program, as currently designed, would be **ill-formed** under the
kernel's own grammar rather than merely incomplete. The kernel is itself thin here (no AMM
template in `lean/DefiKernel/`, P21/P22/P25 open), so this review compares a Moriarty
implementation against a kernel *model*, not against kernel code.

**Milestone ownership, from documents that actually assign one.**

| Gap | Owner | Evidence for the assignment |
|---|---|---|
| Rounding direction, remainder beneficiary, price orientation, unit dimensions (R8, R22, R28) | **U0** | `ROADMAP.md:21` lists "canonical price orientation, unit dimensions, per-primitive rounding direction and beneficiary policy in the numeric profile" as U0 closing evidence |
| Certified arithmetic primitives the swap needs (R22) | **U1** | `ROADMAP.md:22`: "Native certificate for the first exact arithmetic primitive and every primitive needed by the initial slice". Note S0 deliberately excludes division (`UNIFIED-PROPOSAL.md:75-76`), so no division primitive is in the U1 first set |
| A general single-stage swap on the supported path (R1, R9, R12) | **U2** | `ROADMAP.md:23`: "Existing loan/swap requirements remain" |
| Partial fill, two-asset escrow, residual duties (partial fills against a book; R13 in part) | **U3** | `ROADMAP.md:24` |
| Routing and adapters (R17) | **U5** | `ROADMAP.md:26` "One exact-byte external adapter … two independent solvers" |
| AMM libraries and the DeFi action rows (R1–R4, R11, R18 via DA01/DA02/DA03/DA08) | **U6** | `ROADMAP.md:27` "All retained ACTUS fixtures/fields, **DeFi action rows** and held-out behaviors"; `ROADMAP.md:48` "Libraries supply … AMMs, vaults …" |
| Conservation and invariant clauses (R2, R7) | **U0 if the proposal is accepted; otherwise unassigned** | `UNIFIED-PROPOSAL.md:119` (S3) and `:90` (E1) place them in a U0 contract freeze, but `:4` marks the whole document specified-only |
| Supply / mint-burn (R5) | **unassigned** (proposal would put it in "U3, U4 or U6") | `UNIFIED-PROPOSAL.md:192-193` puts `effects.{gross,net,fees}` in the U2-slice cohort and "everything else: owners U3, U4 or U6". `effects.supplyChanges` is not in the U2 list. This is a proposal |
| Order books, batch auctions, market structure, price-impact bounds, MEV/informational bond, surplus allocation, hooks, iteration/convergence (R10, R13, R14, R16, R19, R20, R21, R24, R25) | **unassigned** | No Moriarty document I read assigns an owner. `action-targets.csv` has no row for order books, auctions, RFQ, routing or MEV; `openspec/changes/consolidated-language-kernel/` contains no requirement matching `amm`, `order.?book`, `auction`, `slippage` or `liquidity pool` |

---

## 5. Gaps that would change the language design

These cannot be closed by a library or an example. Ranked by how much new
language/kernel/judgment/numeric/enforcement machinery each demands, and by how much of
the category it unblocks.

1. **A judgment language that can state an inequality over pre- and post-state.**
   Every AMM property is an inequality on a derived quantity: `x'·y' ≥ x·y`, `Σ shares'
   entitlement ≥ Σ shares entitlement`, `output ≤ reserve`. Moriarty's six judgments are
   prose field lists (`judgments.json`; `UNIFIED-PROPOSAL.md:36`), and `ensures` clauses
   are *program-local* postconditions, not stage-relation judgments. Until a clause
   language with `holdsWhen`/`violatedWhen` over relation paths exists
   (`UNIFIED-PROPOSAL.md:119`, still a proposal), `swap.mori:79` can only pin the answer to
   a constant, which is what it does. **This is the gate for R1, R2, R4 and R7 together.**

2. **Supply authority and a share-supply abstraction as first-class effects.** The kernel
   makes minting/burning a separately authorized delta with a conservation identity
   (`README.md:41-43`; `Typed/Transition.lean:14-17,139-148`). Moriarty has a schema leaf
   with no producer, no capability class and no operation (R5). LP shares, and therefore
   `Sh`, `Pl` and every vault-shaped family, are blocked on this. It needs a new protected
   operation, a new capability class, a conservation clause, and an empty-pool mint
   invariant — four separate pieces of machinery, none of which is a library.

3. **The informational bond and the surplus-allocation rule (L24, L29).** These are
   *required* Atlas laws for `In`, `Rf`, `Ba`, `Ob` (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:
   764,769`) and Moriarty has neither, in any document I read (R19, R20). A signed intent
   that names a payer cap and a recipient floor but not (a) who observes the order before
   settlement, from a closed tag catalog, and (b) who is the residual claimant of any
   improvement, is under-specified in exactly the place where the money moves. This needs a
   new signed-intent field with a closed enumeration and a new judgment clause, plus an
   enforcement locus decision — it cannot be a library convention, because a solver chooses
   the disclosure and the surplus.

4. **A bounded-iteration construct with a convergence and tolerance policy and a typed
   non-convergence refusal.** `St` (StableSwap) and `Wg` (weighted-geometric) are not
   expressible without it, and the legacy profile already names it as version-breaking
   (`spec/numeric-profile.json`, `extensionsRequiringNewVersion`). The `/5` grammar has no
   loop (`v5-grammar.ebnf:55`). This interacts with the bounded-execution guarantee
   (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:66`) and with circuit cost, so it is a joint
   language/numeric-profile/target decision, not a syntax addition.

5. **Wider intermediates, signed arithmetic and a `mulDiv` primitive with a stated rounding
   direction.** Concentrated liquidity needs `Q96` fixed point, 512-bit intermediates and
   signed tick deltas; the kernel's own CL substrate is built exactly that way
   (`ConcentratedLiquidity/Types.lean:6-18`; `SqrtPriceMath.lean:30-47`). Moriarty has
   `UInt256` and `SInt128` but no `mulDiv`, no sqrt and an explicit "reject overflow even
   when a later quotient would fit" rule (`spec/numeric-profile.json`,
   `integer.widthBoundary`) that forbids the standard CL idiom. This changes the numeric
   profile and the certified-primitive basis (`ROADMAP.md:22`), not a library.

6. **A remainder recipient that exists.** `numeric-profile.json` names
   `remainderBeneficiary: protocol-reserve` and then records `reserveMechanism.status:
   "absent"`, which makes six of seventeen primitives open gaps. "Rounding always favours
   the pool" (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:897`) is not even statable because there
   is no pool party in the model. `UNIFIED-PROPOSAL.md:267` explicitly forbids declaring a
   reserve type before the N1 decision, because a bare declaration would falsely close all
   six gaps. So this is an **owner decision** blocking the whole rounding story.

7. **A multi-party stage with net obligations.** Batch clearing, order-book matching and
   RFQ settlement all need more than one signer's authority consumed in one accepted stage,
   with per-participant net obligations that must clear to zero — which is precisely the
   kernel's atomic settlement policy (`Atomic/Policy.lean:8-29`; `README.md:60-64`).
   Moriarty's canonical stage statement is written around *one* signed intention
   (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:42`). Making it n-party changes the stage
   relation, the authority judgment and the failure policy simultaneously.

8. **A same-scope net-settlement (flash) construct.** `Fl` is the only async-impossible
   element (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:852-856`) and it is the enabler for hazard
   X2. Without it Moriarty cannot express the borrow-operate-settle-net shape, and cannot
   state the hazard either. Ranked below the others because `action-targets.csv:9` already
   flags it as needing a normative atomicity fixture, i.e. it is a known open item.

9. **A per-program residue declaration.** The kernel makes residue a mandatory output of
   every decomposition (`:729-733`) and its own benchmark found the AMM category's residue
   is concentrated in what protocols compete on (`corpus50/VERDICT.md:24-26`). Moriarty's
   nearest mechanism is a 2026-09-07 family matrix that predates `/5`. A per-program "what
   this model does not express" line would be cheap and would change what a Moriarty
   program *claims*, which is a language-contract change even though it is not a construct.

---

## 6. Limits of this review

- **No Moriarty code was executed.** Two attempts to run the `/5` CLI checker failed on
  this host: `ERR_UNKNOWN_FILE_EXTENSION` for `.ts`, then `Error [ERR_NO_TYPESCRIPT]:
  Node.js is not compiled with TypeScript support` under `--experimental-strip-types`
  (Node `v22.22.1`). So every statement about what the evaluator computes — R1, R4, R12,
  R22, R23 — is **read from source**, and the `19743` result at
  `tests/financial-expression-source-v1.test.mjs:113` is a test *source line I read*, not a
  test run I observed. This is the same limit the U0 study records for itself
  (`UNIFIED-PROPOSAL.md:284`).
- **No Lean was built.** I did not run `lake build` or any `Audit.lean`. Claims about the
  kernel's Lean layer come from structure declarations and `wc`/`grep`, so "there is no AMM
  template in `lean/DefiKernel/`" is a statement about a recorded search: I grepped for
  `swap`, `amm`, `constant.?product` and `pool` case-insensitively across `lean/` and found
  only the `DefiHistorical/Convex/Data.lean` element-table rows and unrelated `.pool`
  account identifiers. Absence from that search is not proof of non-existence.
- **No U0 checker was run.** The counts in `EXIT-GATE.md` (0 enforced / 84 NOT_ENFORCED;
  17 primitives, 6 open gaps; 0 covered / 5 partial / 18 not-covered K rows) are **read
  from the recorded file**, not reproduced. The U0 study reports that the checkers exited 0
  for every reviewer (`UNIFIED-PROPOSAL.md:286`); I did not confirm that.
- **Asserted by inference, not by a cited line.** (a) That `floor_div`/`ceil_div` are
  available inside `moriarty-financial-agreement-source/5`. The `/5` EBNF does not list
  them as keywords, but `/5` lowers actions through `lowerAndCheckFinancialAction`
  (`financial-agreement-source-compiler.ts:617`) → `createFinancialSourceLowering`
  (`financial-expression-source-v1.ts:129`) → `createSourceLowering`
  (`financial-expression-source-lower.ts:10`), which handles `floor_div`/`ceil_div`
  unconditionally at `expression-source-lower.ts:53-56`. That is a source-path argument,
  not an executed check, and I could not execute it. (b) That the `Cl` verdict is `absent`
  rather than `partial` — I inferred it from the total absence of tick/range/sqrt
  vocabulary, not from a document stating "Moriarty has no concentrated liquidity".
  (c) That `moriarty-bounded-atomic/1` is legacy rather than current. Evidence:
  `source-core-embeddings.json:825` says "source/5 does not use that profile", and the
  whole-language audit describes "older bounded-atomic intent/runtime/lowerer"
  (`deliverables/whole-language-review-2026-09-19/astra-review.md:5`). I did not find a
  document that formally deprecates it, and the CLI may still accept it.
- **Not read.** `deliverables/defiformal-study-2026-09-19/` beyond `RESULT.md:7`
  (its `composition-review.md` and `libraries-review.md` may contain AMM-relevant findings
  I have not weighed); the full `source-core-embeddings.json`; `trust-premises.json` and
  `target-pins.json` beyond what `UNIFIED-PROPOSAL.md` quotes; the nine per-lens U0 reviews
  in this directory; `formal/k/` beyond the arithmetic rules; defiformal's `quint-models*`,
  `paper/`, `research/` and `wiki-llm/`.
- **Scope boundary I did not cross.** Perpetuals, options and margin sit in FIN-DER and
  belong to another reviewer, even where they share the order-book and clearing machinery
  I marked absent (R13, R14). Vault share accounting (DA17/DA18, ERC-4626/7540) is adjacent
  to R4 and I touched it only where LP share math is the same computation.
- **The comparison is asymmetric by construction.** The kernel side of this category is a
  model plus an arithmetic substrate, with P21/P22/P25 open (`openspec/ROADMAP.md:50-51,54`)
  and its own 69-protocol benchmark concluding "not a basis for DeFi"
  (`corpus50/VERDICT.md:12-14`). A Moriarty gap against this kernel is a gap against a
  specification, not against working reference code.

---

## Architecture axis (follow-up)

Additional axis requested by the coordinator: does the roadmap/architecture say *how* this
category will be built, not whether it is built. Read in full for this pass: `ROADMAP.md`
(56 lines) and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines). Sections 1–6 above are
unrevised.

**1. Placement.** The category is split across three places and one of the three splits is
empty. Pool pricing is a **source-library family**: "Libraries supply ACTUS, loans/claims,
payments/escrow, **AMMs**, vaults, netting, redemption/loss allocation, margin and
async/conditional workflows" (`ROADMAP.md:48`), restated as "swaps and liquidity"
(`docs/MORIARTY-CONSOLIDATED-DESIGN.md:93`), over a core whose "Financial applications are
source libraries over that core" (`:15`). Matching and routing are placed **outside the
language**, in the Federated DeFi Kernel column: "Find liquidity and counterparties;
arrange execution" (`:26`) and "Human or AI search, **matching**, scheduling, retries"
(`:29`) — and that kernel is optional, "Moriarty can also run on Midnight without this
federation" (`:11`). Order books, batch auctions and RFQ are **placed nowhere**: they are
not a library family at `:93` or `ROADMAP.md:48`, not a responsibility-table row, and not
a settlement adapter (adapters are `ROADMAP.md:26`, U5, "one exact-byte external adapter").

**2. Core mechanisms needed.** The core list is `:15` — exact typed values; bounded
evaluation; explicit state and effects; checked authority; assertions and refinements;
authenticated evidence; persistent obligations and continuations; defined composition
operators.

| Mechanism this category needs | Named in the architecture? | Component designated to realize it? |
|---|---|---|
| Directed-rounding division with units and price orientation | Yes — `:52`, and U0 "freezes canonical price orientation, unit dimensions, per-primitive rounding direction and beneficiary policy" | Yes: U0 numeric profile (`ROADMAP.md:21`), U1 certificates (`:22`), jets (`:91`) |
| Price **derived from inventory** (pool state), not quoted | No | None |
| Supply changes for LP shares | Yes, as a bound field — "complete effects including fees and supply changes" (`:42`) | None. `:26` "Financial meaning" lists gross debits, fees, net outcomes, assets, custody, liabilities, residual duties — supply is absent from the row |
| Invariant assertion over pre/post state | Partly — "assertions and refinements" (`:15`), "contract properties" (`:48`) | Judgment framework, but `:42` binds fields, not relations (see 4) |
| Bounded iteration with a tolerance and a non-convergence refusal | No. `:66` bounds termination and forbids unbounded recursion; no iteration construct is named | None |
| Multi-party clearing / netting | Yes — "netting" as a library (`ROADMAP.md:48`), "aggregation and netting" (`:29`), "Netting requires a relation preserving the authorized gross economics, liability ownership and residual duties" (`:56`) | None — see H2 |
| Order observation / ordering | No | None |

**3. Architectural holes.** Requirements stated with nothing on the other side:

- **H1 — the negative definition with no positive one.** `:93` says "Narrow token0 pricing
  is not a full AMM; stable-time vault conversion is not accrual." Nothing anywhere states
  what a full AMM *does* require. I searched `ROADMAP.md` and
  `docs/MORIARTY-CONSOLIDATED-DESIGN.md` in full: `:93` and `ROADMAP.md:48` are the only
  two lines that mention AMMs/swaps as a family, and neither enumerates a component. The
  scope matrix promised at `:93` ("Each family receives a scope matrix") has no owner named
  in either document. This is the sharpest hole: the design knows the boundary is wrong and
  names no one to draw the right one.
- **H2 — netting asserted, no n-party stage.** `:29` makes the *language* responsible for
  preserving hard constraints "through completion, aggregation and netting", and `:56`
  requires "a relation preserving the authorized gross economics". But the canonical stage
  binds one "signed intention" (`:42`), and `:72` scopes "Atomic publication … only within
  a declared domain and its actual phase semantics". No component is named that composes
  several parties' intents into one cleared stage. Batch auctions, order-book fills and RFQ
  settlement all land in this hole.
- **H3 — shared budget across concurrent solvers.** `:56` asserts "Concurrent solvers
  reserve spent plus pending exposure against the same authenticated budget." No row of the
  responsibility table (`:23-36`) owns reservation arbitration; `ROADMAP.md:26` gives U5
  "durable concurrent reservations, retries and failover" as *kernel* evidence, while the
  requirement is stated as a language-side invariant. Two different sides, no seam.
- **H4 — the venue has no architectural role.** The table has Intent, Financial meaning,
  Permission, Proofs, Solver search, Conditions, State/history, Atomicity, ZK/MPC/TEE,
  Recovery, Privacy, Release process (`:25-36`). None is the *counterparty program*. A pool
  or a book is a deployed program that is the other side of a trade; the architecture
  describes the caller's stage and the kernel's search and nothing in between. With the
  kernel optional (`:11`), a non-federated deployment has nothing that "finds liquidity".
- **H5 — a swap-shaped acceptance test with no producer.** `ROADMAP.md:38` sets the U3
  discriminator "spend at most 11 A including at most 1 A fees to receive at least 20 B …
  Two independently produced candidates may supply different authorized **routes**". A route
  terminates at a venue; no component supplies the 20 B, and "route" is otherwise undefined
  in either document.
- **H6 — "No privileged solver bypass" (`:29`) with no disclosure or surplus rule.** The
  Privacy row (`:35`) governs what is hidden *from* observers; nothing governs who observes
  an order *before* settlement, or who receives improvement above `minNetOutcome`. The
  Conditions row (`:30`) covers evidence freshness and disclosure of documents, not order
  visibility. This is the architectural form of gap 3 in §5.

**4. Canonical stage statement (`:42`).** It does bind four things this category needs:
supply changes, complete gross/net effects including fees, typed observations with issuer,
domain, time and finality (enough to carry an external `Pm` reference price), and the
authenticated state frame (enough to hold reserves). Exact omissions: (a) **one** signed
intention — no participant set and no per-participant net obligation, so no clearing;
(b) it binds *fields*, not *relations* — there is no slot for a derived quantity or an
invariant over pre-state and post-state, so `x·y ≥ k` has nowhere to live even though
`:48` demands a contract-properties judgment; (c) no counterparty/venue identity distinct
from `programIdentity` — "recipients" is a payee role, not a counterparty role; (d) no
ordering position, observer set or residual claimant; (e) "obligation commitments" but no
invariant commitment, so a pool's state cannot be carried forward as a checked quantity;
(f) observations are issuer-attested external facts — no typing for a price *computed from*
the bound state frame.

**5. Ownership of the architecture work.** Assigned: numeric profile, price orientation and
rounding architecture → **U0** (`ROADMAP.md:21`); certified-primitive basis → **U1**
(`:22`) with the jet contract at `:91`; the family scope matrices and DeFi action rows →
**U6** (`ROADMAP.md:27`, "All retained ACTUS fixtures/fields, DeFi action rows and held-out
behaviors"); routing, matching services and adapters → **U5** (`ROADMAP.md:26`);
partial fill and joins → **U3** (`:24`). **Unassigned**: what a full AMM requires (H1) —
`ROADMAP.md:30` says "Conditional reference examples and library source analysis can inform
U0", which is informing, not owning; the n-party stage and atomic batch (H2) — `:72` states
the obligation with no milestone and `action-targets.csv:25` marks the composition
operators `specified-only`; concurrent-budget arbitration (H3); the venue role (H4);
disclosure and surplus allocation (H6); order books, auctions, RFQ and market structure,
which appear in no U row at all.

**6. Minimum addition to avoid a later core redesign.** Six items, each an architecture
statement rather than an implementation:

1. **A relation slot in the canonical stage statement** — a named invariant over pre-state,
   post-state and effects that the contract-properties judgment evaluates, so pool
   invariants are stage-bound rather than program-local `ensures`. Without it, every AMM
   library re-enters through H1 and every family that needs an inequality repeats the work.
2. **Supply as a responsibility-table concern** — add supply changes and a supply capability
   class to the "Financial meaning" row (`:26`), so shares/vaults (`:93`) and AMMs share one
   mechanism instead of two.
3. **An n-party stage profile** named as the realization of "aggregation and netting"
   (`:29`, `:56`) and "atomic batch" (`:32`): a participant set with per-participant net
   obligations that must clear. This is the one addition that cannot be retrofitted — a
   single-signer stage relation is load-bearing in `:42`.
4. **A declared numeric-profile extension point**: which arithmetic shapes a family may add
   (wider intermediates, signed values, bounded iteration with a tolerance) and which
   component certifies them. The vocabulary already exists in the legacy profile
   (`experiments/moriarty-language/spec/numeric-profile.json`,
   `extensionsRequiringNewVersion`); neither controlling document adopts it, so today
   `St`/`Wg`/`Cl` would each be a core change.
5. **A disclosure-and-surplus row in the responsibility table**: who observes an order
   before settlement, and who is the residual claimant of improvement — assigned to the
   language as signed-intent fields, not left between the optional kernel and the chain.
6. **A venue/counterparty role** in the boundary table, so a pool or book is a first-class
   deployed program with its own identity and its own state frame, and "find liquidity"
   (`:26`) has something to find when the federation is absent (`:11`).

**Architecture verdict.** The roadmap has an architecture for *pool pricing as a library
over a certified arithmetic core* — placement, numeric ownership and certification are all
stated with owners. It has **no architecture for the rest of the category**: matching,
clearing, order books and routing are either delegated to an explicitly optional service or
unplaced, and the four assertions that would need them (`:26`, `:29`, `:32`, `:56`) name no
component. The design's own line "Narrow token0 pricing is not a full AMM" (`:93`) is
correct and, as far as these two documents go, unanswered.
