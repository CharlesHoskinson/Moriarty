# R4 — derivatives coverage review

Reviewer R4 of nine. Read-only review. This file is the only artifact written.

**Status: review record, specified-only.** Nothing here closes a U0 predicate, accepts a
change, or establishes a capability. Under `AGENTS.md` ("What accepts work"), work is
accepted by evidence, not by review.

---

## 1. Scope and pins

### Commits

| Repo | Command run | Output |
|---|---|---|
| defiformal | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` |
| Moriarty | `git -C /home/charl/Moriarty log -1 --format='%H %ad %s'` | `8f73784042bd692733c296d0d49f5173be96725e Wed Sep 23 21:00:14 2026 -0600 U0 T7: add U0 exit gate` |

### Startup performed

`AGENTS.md:1-30` requires loading `moriarty-dev:develop`. The host did not expose the
skill, so I read and applied the checked-in
`plugins/moriarty-dev/skills/develop/SKILL.md`, plus `AGENTS.md` and `docs/FOOTGUNS.md`,
before forming conclusions.

I ran the guarded CLI status once, as `AGENTS.md:15-19` directs:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Actual output (abridged to its decision fields):

```json
{"capability": "SP01.6 loan-swap-subset",
 "blockedAction": "implementation/repair of loan-swap-subset",
 "reason": "operational history is unresolved or unverified; ...",
 "nextAction": "sp01-loan-report",
 "missingEvidence": ["binding-input-stale:openspec/sprints/sp01-...",
                     "candidate-input-stale:openspec/sprints/sp01-...",
                     "current-accounting-missing:.moriarty-dev/runtime/current-accounting.json",
                     "resource-live-state-unavailable:sp01-loan-swap-grok-01",
                     "operational-history"],
 "pendingTransactions": ["00a91ec05fd1ae30…", … 7 IDs …]}
```

Per `SKILL.md` §"Standard Product Development Workflow", read-only diagnosis needs
`status` but no campaign or `next` dispatch. I ran no `next`, no `run`, no `notify`, no
`deliver`. I did **not** obtain current statuses for the seven pending transaction IDs and
therefore post no transaction notification line: this review submitted nothing to any
network.

### What I read

**defiformal** (all under `/home/charl/projects/defiformal`):
`README.md` (whole), `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` (§2–§18 in full, ~lines 60–1000),
`algebra/MODEL.md` (whole), `lean/README.md` (whole),
`lean/DefiKernel/Typed/Types.lean` (whole), `lean/DefiKernel/Typed/Transition.lean:1-140`,
`lean/DefiKernel/Typed/Examples.lean:118-160`, `lean/DefiKernel/Typed/Expr.lean` (grepped
time/env-read constructors), `lean/DefiKernel/Atomic/Policy.lean:1-80`,
`openspec/ROADMAP.md` (whole),
`openspec/changes/reusable-verification-platform-program/sprint-plan.md:270-312`,
`.../specs/source-bound-library-families/spec.md` (grepped),
`corpus50/lanes/lane2-perps-yield-bridges-intents.json` (perps category + vocabulary gaps,
extracted programmatically),
`corpus50/lanes/lane3-rwa-options-stables-prediction.json` (options/structured category +
vocabulary gaps). I listed but did not read `algebra/EXTENSIONS.md` beyond its headings, and
did not read `docs/unified-v0.1.md` beyond its headings (v0.1 is superseded by the v1.0
element table I did read).

**Moriarty** (all under `/home/charl/Moriarty`):
`AGENTS.md`, `docs/FOOTGUNS.md`, `plugins/moriarty-dev/skills/develop/SKILL.md`,
`ROADMAP.md`, `docs/MORIARTY-CONSOLIDATED-DESIGN.md`,
`docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`,
`deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (whole),
`deliverables/u0-semantic-contract-2026-09-23/{judgments,k-reconciliation,trust-premises,enforcement-map,numeric-profile}.json`,
`deliverables/u0-semantic-contract-2026-09-23/EXIT-GATE.md`,
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`,
`experiments/moriarty-language/spec/successor/{grammar.ebnf,syntax-profile.json}`,
`experiments/moriarty-language/spec/successor/repayment-kernel.md` (grepped),
`experiments/moriarty-language/spec/successor/financial-agreement-source.md:55-70`,
`experiments/moriarty-language/spec/successor/financial-agreement-source-v5.md` (grepped),
`experiments/moriarty-language/src/successor/financial-lifecycle.ts:36-125, 1755-1840`
(and grepped),
`experiments/moriarty-language/src/successor/financial-expression-types-v1.ts` (grepped),
`openspec/REPORT-RECONCILIATION-2026-09-07.md:45-70`,
`deliverables/defi-language-design-2026-09-07/{LANGUAGE-DESIGN.md,action-targets.csv}`,
`docs/research/2026-09-06-actus-defi-design-study.md:30-90`,
`wiki/defiformal-taxonomy.md` (grepped and read §F-families, §CLM-0110),
`wiki/marlowe-baseline.md` (grepped only — it returned no derivatives hits).
I did **not** read `docs/MORIARTY-PRODUCT-CONTRACT.md` in full (only the quotes carried in
`trust-premises.json`), `formal/k/*.k` source, or `deliverables/moriarty-design-sprint-2026-09-06/README.md`.

### What I did not execute

I did not build the Lean kernel, did not run `lake`, did not run any Moriarty checker
(`check_u0_*.py`), did not run the TypeScript evaluator, did not run K, did not compile or
prove anything, did not edit any repository source, and did not commit. Every Moriarty
checker count quoted below is **read from the recorded `EXIT-GATE.md`**, not re-run by me.

---

## 2. What the kernel abstraction requires for this category

The kernel gives two distinct statements about derivatives, and they disagree in an
instructive way.

### 2.1 The Atlas says derivatives are S3 risk-transfer elements with async-unsafe bonds

`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:255-262` places the whole category in **G07, risk
transfer**, role boundary "how is exposure moved between parties?":

- `E023 Pf` perpetual funding transfer — "Periodic payment tethering a perp to an index" (:259)
- `E024 Op` option payoff — "Strike, expiry, collateralized contingent settlement" (:260)
- `E025 Tr` tranche waterfall — "Declared seniority over a determined loss event" (:261)

Solvency is a separate group **G06** (`:240-249`): `Ct` collateral-threshold test (:245),
`Li` incentivized liquidation (:246), `Ad` auto-deleveraging — "Rank-ordered forced close
when buffers are exhausted" (:247), `Sl` socialized-loss allocation — "Losses assigned to an
explicit claim class" (:248), `Bs` staked backstop (:249). All G06 and G07 members are
stratum **S3**: "Contingent obligation, solvency, risk transfer, stability"
(`:419`). S3 sits above S2, "Externally measured or **time-conditioned** state" (`:418`),
which is where `Sr` streaming accrual, `Ep` epoch-gated transition and `Wq` withdrawal queue
live (`:287-289`).

The required-bond laws are what make this a *language* requirement rather than a library
requirement:

| Law | Text | Async-safe? |
|---|---|---|
| L1 (`:741`) | `(Pl\|Im\|Cd\|Pf\|Op) → (Ex\|Tp\|At) + Ct + (Li\|Ad\|Sl\|Bs)` | **No** — "cross-domain also needs a named finality assumption and a liquidation-latency bound" |
| L4 (`:744`) | `Pf → Ex + Ct + Li + (Ad\|Sl\|Bs)` | No |
| L6 (`:746`) | `Tr → (Sv \| mechanical trigger) + declared seniority + dispute forum + recovery-timing assumption` | **No** |

So the kernel's position is: you cannot write a perp or an option without simultaneously
binding (a) a truth source, (b) a margin test, (c) a named loss-absorption mechanism, and
(d) for anything crossing a settlement scope, a *finality assumption and a latency bound*.
The reaction-condition table restates (d) as measurable: "Oracle latency and confidence |
`Pl` `Cd` `Pf` `Pm` `Ct` | Is the value current relative to volatility and **closeout
time**?" (`:939`) and "Executable market depth | `Li` `Rd` `Ps` `Ct` | Can collateral sell
near its **marked** value?" (`:940`).

### 2.2 The corpus says the Atlas's own derivatives vocabulary is under-resolved

`corpus50/` is the kernel's evidence about its own adequacy, and the derivatives lanes carry
the heaviest residue in the project. These are the abstractions the kernel says it *needs and
does not have* — which is exactly the list a new language should be checked against:

- **Portfolio / scenario margin** — `lane3-rwa-options-stables-prediction.json:126`: "Derive
  stress-tests a whole account across 27+ joint spot-and-volatility shock scenarios and
  margins the worst case. `Ct` is a per-position threshold test. These are different objects
  — one is a scalar comparison, the other is a **min over a scenario grid**". Restated as a
  project-level gap at `:422`.
- **Margin scope** — `lane2-perps-yield-bridges-intents.json:25`: "there is no element for
  margin scope (cross vs isolated vs per-subaccount)", and `:128`: "margin scope (cross vs
  isolated)" listed among category-wide holes.
- **Terminal settlement-price fixing** — `lane3:129`: "No element for SETTLEMENT PRICE
  DETERMINATION at expiry (the expiry fixing window, the index construction). `Ex` names a
  price feed, **not a terminal fixing that irrevocably determines payoff**." Restated at `:427`.
- **Exercise style** — `lane3:205`: `Op` "collapses at least six orthogonal distinctions…
  (a) European vs American vs perpetual exercise; (b) cash vs physical settlement; (c)
  peer-to-peer vs peer-to-pool underwriting; (d) upfront vs streamed premium; (e) isolated vs
  portfolio/scenario margin; (f) model-priced vs order-book-priced."
- **Expiry-free options** — `lane3:194`: Panoptic "options never expire and are never
  exercised at a strike"; `Op`'s definition "is false for two of its three clauses here".
- **Instrument listing / expiry-cycle definition** — `lane3:128` and `:424`: "who decides
  which strikes, expiries, or markets exist… a first-class governed process at every
  derivatives and prediction venue."
- **Funding-tether shape** — `lane2:101`: Jupiter "has no funding rate… `Pf` is defined as a
  'payment tethering perp to index' **between position sides** and simply does not describe
  this"; generalized at `:128`.
- **Insurance fund ≠ staked backstop** — `lane2:536`: "Non-slashable insurance fund distinct
  from `Bs` staked backstop (I was forced into `Bs` five times)".
- **Keeper / automation liveness dependency** — `lane2:532`: "the fact that a mechanism
  requires an external caller to keep running, and degrades or liquidates if that caller
  stops."
- **Conditional-token split & merge** — `lane3:406`: "`Py` splits along time; nothing splits
  along state."

### 2.3 The Lean kernel's state/transition/capability/conservation model

The executable kernel (not the Atlas) fixes five abstractions, and each one bites on
derivatives:

1. **State is a non-negative balance map.** `lean/DefiKernel/Typed/Types.lean:32-34`:
   `State` is `balance : Cell → ℚ` with `nonneg : ∀ c, 0 ≤ balance c`, over
   `Cell := Domain × Party × Asset` (`:25`). There is **no signed position**. Debt is modelled
   as a separate non-negative obligation token — `lean/README.md:56-57`: "Debt is represented
   as a distinct nonnegative obligation token in the reference example. This is not a general
   party/claim lifecycle model."
2. **A transition is a registered template with declared read/write footprints and a guard.**
   `lean/DefiKernel/Typed/Transition.lean:19-28`: `Template` carries `guard`, `deltas`,
   `supplyDeltas`, `stateReads`, `envReads`, `writes`. Crucially, `lean/README.md:167-169`:
   "A template describes a **net** balance change. Calls needing separate intermediate states
   must be modelled as separate steps."
3. **Capabilities gate invoke, debit and supply.** `README.md:35-38`, enforced at
   `Transition.lean:129-140` (`debitsOK`, `suppliesOK`).
4. **Conservation is per-domain per-asset.** `README.md:41-43`: "For each asset in each
   domain, the total balance change must equal declared minting minus burning."
5. **Time is an environment input with a declared footprint, not ledger state and not a
   scheduler.** `Typed/Expr.lean:85-86` gives two expression constructors, `timestamp (key)`
   and `now`; `:98-100` gives `EnvRead.currentTime`; `:139` evaluates `now` from
   `ctx.now : Nat`. `Types.lean:124-126` gives `Observation` a `timestamp : Nat`. A staleness
   check is then an ordinary guard: `Typed/Examples.lean:133-134` requires
   `timestamp(priceKey) ≤ now ∧ now ≤ timestamp(priceKey) + 5`, with `envReads := [.observation
   priceKey, .currentTime]` (`:148`). Nothing in the kernel *triggers* a transition at a
   time; every transition is an externally submitted `Request` (`Transition.lean:36-42`).
   `README.md:163-165` is explicit that "checking its type, sign, and age does not establish
   its truth".
6. **Multilateral settlement nets to zero.** `README.md:60-64`: an atomic settlement policy
   "records each caller's net obligations from movements at those accounts and commits only
   when every tracked obligation is zero"; `lean/DefiKernel/Atomic/Policy.lean:17-27` is the
   `Policy`/`Outstanding`/`Residual` triple. `README.md:63-64` adds the honest limit: "The
   schedule is an input: running one schedule does not explore all possible orderings."

### 2.4 The kernel's own roadmap says derivatives are unbuilt

This is the single most important calibration for the whole R1–R9 exercise.
`openspec/ROADMAP.md:56`:

> `P27 — Signed margin, funding, unsettled PnL, liquidation, bankruptcy` | None hard;
> Resource: P17 | **Planned**: signed margin/funding/PnL/liquidation/bankruptcy and
> source-selected refusal.

Its sprint definition (`sprint-plan.md:284-291`) reads:

> - **Outputs:** pin record or block; **proposed** `lean/DefiKernel/Margin/`.
> - **Claim boundary:** pinned derivatives accounting. **Unsettled PnL is not cash unless the
>   source says so.**
> - **Checks:** funding success; allowed liquidation/bankruptcy as **exceptional success**;
>   actual rejection gate selected from the later pin.
> - **Stop:** inventing bankruptcy rules.

`P28` (`ROADMAP.md:57`, `sprint-plan.md:293-300`) covers conditional insurance and off-chain
claims, also planned. `P26` (`:275-282`) covers async lifecycle with "finality, replay
refusal, timeout, challenge, compensation" checks and stop rule "assuming synchronous
rollback". None of these has an accepted-and-delivered marker in `ROADMAP.md:56-58`, and
`lean/DefiKernel/` contains no `Margin/`, `Async/` or `ConditionalClaims/` directory (I
listed the tree; the directories present are `Arithmetic, Atomic, CapabilityProvenance,
Composition, ConcentratedLiquidity, Interface, Interleaving, Parallel, Typed, …`).

**Consequence for this review:** defiformal supplies for derivatives a *specification of what
the abstractions must be*, plus a documented inventory of what its own vocabulary cannot
express. It does not supply a worked derivatives kernel to check Moriarty against. I mark
requirements accordingly and never claim Moriarty fails to match something the kernel also
lacks.

### 2.5 The requirement list I check Moriarty against

| Id | Requirement (abstraction level) | Kernel citation |
|---|---|---|
| D1 | Signed position / margin state, with unsettled PnL as a distinct non-spendable class | `lean/DefiKernel/Typed/Types.lean:32-34`; `lean/README.md:56-57`; `openspec/ROADMAP.md:56`; `sprint-plan.md:287-288` |
| D2 | Periodic funding as a recurring, non-skippable, non-replayable stage tethered to an index | element table `:259`; L4 `:744`; `lane2:101,:128` |
| D3 | Expiry, exercise window and **terminal settlement fixing** distinct from a continuous feed | element table `:260`; `lane3:129,:205,:427` |
| D4 | Margin test with declared **scope** (isolated / cross / portfolio-scenario) | element table `:245`; `lane3:126,:422`; `lane2:25,:128` |
| D5 | Position lifecycle as separate committed stages (open / adjust / close / expire / exercise / settle) with surviving duties | `lean/README.md:167-169`; element table `:288` (`Ep` snapshot/cutoff/rollover) |
| D6 | Ordered loss absorption: liquidation → ADL → insurance fund → socialized loss, over an explicit claim class | element table `:246-249`; L1 `:741`; L4 `:744`; `lane2:536` |
| D7 | Tranching / declared seniority over a determined loss event, with a recovery-timing assumption | element table `:261`; L6 `:746` |
| D8 | Contingent payoff on an authenticated external observation, with the truth boundary named | element table `:268-272`; `README.md:163-165`; `openspec/ROADMAP.md:57` |
| D9 | Conservation under mint/burn of contingent claims, per domain and asset | `README.md:41-43` |
| D10 | Multilateral netting: commit only when every tracked net obligation is zero | `README.md:60-64`; `Atomic/Policy.lean:17-27` |
| D11 | Named finality assumption + liquidation-latency bound whenever the molecule is not single-scope | L1 `:741`; reaction conditions `:937,:939,:940` |
| D12 | Keeper/automation liveness as an explicit dependency, not an assumption of execution | `lane2:532`; `README.md:170-171` ("does not … guarantee eventual execution") |
| D13 | Model-priced payoff arithmetic (vol surface / non-linear payoff) under exact rationals or a stated finite profile | `README.md:25-27`; `lane3:127,:423` |
| D14 | Instrument listing / expiry-cycle definition as governed state | `lane3:128,:424` |
| D15 | State-partition of collateral into complementary contingent claims (split/merge) | `lane3:406` |

---

## 3. Coverage verdict per requirement

Evidence grades used in the "where" column, strictest first:
**(demonstrated)** an executed trace exists and is cited; **(implemented)** code exists in the
evaluator/kernel; **(specified)** a schema field, grammar production or normative sentence
exists; **(designed)** a design document names the obligation with no artifact.

| Id | Requirement | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| D1 | Signed position / margin state; unsettled PnL not cash | **partial** | Signed numeric types exist: `experiments/moriarty-language/src/successor/financial-expression-types-v1.ts:8` lists `SInt128, SignedScaledAmount, SignedAmount, NetAmount`; `:154` admits negative values for `SInt128/Quantity/Rate`. The requirement itself is written: `openspec/REPORT-RECONCILIATION-2026-09-07.md:64` — "Margin and funding \| **Unsettled P&L, funding debt and spendable balance; reject spending contingent profit as cash** \| MC05, MC07" (specified). The stage relation has no position object; `liabilities.opening/closing` entries are `{liabilityId, debtor, creditor, asset, amount}` (`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:391-455`), a one-directional debt. | No position state. No mark-to-market. Ledger-level amounts are non-negative: `experiments/moriarty-language/spec/successor/financial-agreement-source.md:64` requires "nonnegative signed128 range". The schema's `^-?[0-9]+$` amount pattern (`stage-relation.schema.json:299,323,347,371,413,445`) admits a negative, but `UNIFIED-PROPOSAL.md:35` records that as a **defect** (F4: "It allows negative amounts… The aeon −1 state is schema-valid"), and `UNIFIED-PROPOSAL.md:111` proposes S2 make amounts "canonical non-negative integers". So the one place a signed position could live is being closed, not opened. |
| D2 | Periodic funding as a recurring non-skippable stage | **partial** | This is the strongest positive finding of the review. `experiments/moriarty-language/src/successor/financial-lifecycle.ts:36-42` defines `AccrualTerms { numerator, denominator, rounding, periodSeconds, firstPeriodStart }`; `:120-125` defines `AccrueAction { accrualId, obligationId, periodIndex, observedTime }`; `:1761-1765` rejects `PERIOD_SEQUENCE` unless `periodIndex == lastAccruedPeriod + 1`; `:1767-1770` rejects `PERIOD_NOT_ELIGIBLE` unless `observedTime >= obligation.nextAccrualAt`; `:1803-1810` rolls `nextAccrualAt` forward by `(periodIndex+1) * periodSeconds`; `:1758-1760` rejects a duplicate `accrualId`. `Accrue` is reachable from source/5: `spec/successor/financial-agreement-source-v5.md:4` — "four protected operations Transfer, Repay, Originate and Accrue" (implemented). | Four gaps. (a) **Unsigned and unilateral**: accrual only *adds* to one debtor's liability (`:1793-1801`); a funding transfer reverses sign per period and moves value between two position sides — `lane2:101` is the kernel's own statement that a unilateral pool charge and a long/short transfer are different mechanisms. (b) **Rate is a constant in the terms**, not an observation: `numerator/denominator` are fixed at `Originate` (`:104-119`), so the index/mark tether of L4 (`:744`) has nowhere to enter. (c) **`observedTime` is a caller-supplied action field** (`:124`) with no authentication and no upper bound — a late accrual is always admissible. It is not a stage-relation leaf and therefore appears nowhere in the 84-leaf enforcement map. (d) No U-milestone in `ROADMAP.md:19-28` names periodic or recurring stages. |
| D3 | Expiry, exercise window, terminal settlement fixing | **absent** (as language), **designed** (as obligation) | The obligation is stated precisely and repeatedly: `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:83` — "**Separate observation timestamps, payment dates, exercise windows and authorization expiry.** Calendars, day-count and business-day policies are named financial definitions; ACTUS supplies their reference cases"; `docs/research/2026-09-06-actus-defi-design-study.md:49` — "OPTNS, FUTUR \| **Exercise versus settlement; option positive-part versus signed futures payoff; observed underlying and fixing time**"; `docs/MORIARTY-CONSOLIDATED-DESIGN.md:68` — "no fixed default expiry or perpetual spending right is implied". | No time or date type exists in the source language: `experiments/moriarty-language/spec/successor/grammar.ebnf:57` is the whole type production (`type = identifier, [ typeArgs ]`) and `:11-39` admits only `unit/party/asset/const/state/action` declarations. `signedIntent.validity` is a bare `"type": "string"` with no pattern (`stage-relation.schema.json:170-172`); a name in a schema is not coverage. `observations[].time` is likewise an unconstrained string (`:261-263`). Nothing distinguishes an *exercise window* from an *authority expiry* from a *settlement fixing*, which is the exact distinction `LANGUAGE-DESIGN.md:83` asks for. |
| D4 | Margin test with declared scope | **absent** | The only threshold machinery in the source language is the generic `requires expression;` guard (`grammar.ebnf:43,45`) and `ensures` postconditions (`:55`). `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` names "margin" as a planned library family, and `ROADMAP.md:48` repeats it. | No health/collateral-threshold construct, no notion of margin scope (isolated / cross / portfolio). The kernel's sharpest derivatives finding — that portfolio margin is "a min over a scenario grid" and a different object from a scalar test (`lane3:126`) — has no counterpart anywhere in Moriarty, and a min-over-a-scenario-grid is precisely the kind of construct that interacts with Moriarty's finite-execution bound (`docs/FOOTGUNS.md`, rule 5: "Require explicit sizes for … schedules, horizons … transaction work"). |
| D5 | Position lifecycle as separate committed stages with surviving duties | **partial** | Strongest design coverage in the repo. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:62` enumerates the required distinctions ("submitted without funding; funded into programmable escrow; partially fulfilled; waiting for evidence; eligible for release; in flight; delivered; in an unresolved state; or recovering… Evidence-ready is not delivery"); `:66` bounds each stage and routes long-lived workflows through "authenticated continuations"; `:70` keeps duties alive across expiry ("Revocation cannot erase outstanding duties"). The relation has `outcome.kind ∈ {continuation, terminal}` and `outcome.continuations[]` (`stage-relation.schema.json:533-552`), and `failurePolicy.{phasePolicy, retainedEffects, retainedFees}` (`:505-530`). `ROADMAP.md:24` assigns this to **U3**. | Every one of those fields is an unconstrained `"type": "string"` (`:519-528`, `:545-551`). No enum, no typed phase, no relation between a continuation and its predecessor beyond an opaque identifier. The K reconciliation records the semantic state honestly: `deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json` UNI-006 "Phases and partiality" = **not-covered**; UNI-008 "Recovery and state limits" = not-covered, note "K lacks recovery or successor transitions that preserve consent, cumulative budgets, replay state, and **duties at expiry**"; judgment `failure` = partial, "Missing: a partial-failure transition that retains only authorized phase effects and fees". |
| D6 | Ordered loss absorption (liquidation → ADL → insurance → socialized) | **partial** (requirement recorded), **absent** (as abstraction) | `openspec/REPORT-RECONCILIATION-2026-09-07.md:61` — "Bad debt \| Debt and share-value/loss allocation; **reject erased residual debt or missing socialized loss** \| MC05, MC07"; `:60` — "Ordered redemption \| Position/rate **ordering** and residual claim; reject out-of-order settlement \| MC06, MC07". `ROADMAP.md:48` and `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` name "redemption/loss allocation" as a library family. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:54` carries the load-bearing rule: "Default does not erase debt, and forgiveness is not repayment." | No ordering construct. `liabilities.opening/closing` is an unordered array of flat records with no seniority, no claim class and no waterfall position (`stage-relation.schema.json:378-455`). There is no rank-ordered forced-close abstraction (`Ad`, element table `:247`) and no "explicit claim class" to which a socialized loss attaches (`Sl`, `:248`). The distinction the kernel calls decisive — capital posted *in advance* (`Bs`) versus loss assigned *after* (`Sl`), element table `:994` — is not expressible. |
| D7 | Tranching / declared seniority | **absent** | Nothing found. `liabilities` is flat (above). No seniority, dispute forum or recovery-timing field anywhere in the stage relation or the grammar. | L6 (`element table:746`) requires four co-present things for a tranche: a determination trigger, declared seniority, a dispute forum, and a recovery-timing assumption. Moriarty has an adjacent piece of one of them — `docs/MORIARTY-CONSOLIDATED-DESIGN.md:64`, "Documents establish only the authenticated predicate actually checked" — and none of the other three. No milestone names tranching. |
| D8 | Contingent payoff on an authenticated observation, truth boundary named | **partial** | The trust boundary is named well and consistently: `deliverables/u0-semantic-contract-2026-09-23/trust-premises.json` **TP03** (`observations-finality`, status `open`) — "Issuers, oracles and signers remain explicit trust assumptions for observations, finality and oracle honesty", quoting `docs/MORIARTY-PRODUCT-CONTRACT.md` "External observations and attestations remain named assumptions; they cannot substitute for mandatory program/transition proofs". The relation carries `observations[] {kind, issuer, domain, time, finality}` (`stage-relation.schema.json:239-270`) and the effect judgment claims those fields (`judgments.json`, key `effect`). `ROADMAP.md:24` assigns conditional settlement to **U3**. | Three separate gaps. (a) The source language cannot declare an observation at all: `experiments/moriarty-language/spec/successor/syntax-profile.json:53-60` lists `"observation"` in `unsupportedDeclarations`, alongside `"settlement"`, `"status"` and `"reserve"` (`:61-64`). (b) `k-reconciliation.json` UNI-007 "Conditional settlement" = **not-covered** — "K lacks conditional settlement that separates request recording, funding, eligibility, in-flight effects, and actual delivery". (c) All four observation leaves read `NOT_ENFORCED` — `enforcement-map.json` has 84 rows and `Counter({'NOT_ENFORCED': 84})` (I computed this count from the file; the same number is recorded at `EXIT-GATE.md` line for the enforcement map: "0 enforced, 84 unenforced"). |
| D9 | Conservation under mint/burn of contingent claims | **partial** | `stage-relation.schema.json:273-377` gives `effects.{gross, fees, net, supplyChanges}`, each `{asset, account, amount}` — a supply-change channel exists, which is what a contingent claim's mint/burn needs. `UNIFIED-PROPOSAL.md:90` proposes law E1, "per-asset conservation: Σ gross = Σ supply changes = 0", and `:91` E2, "net is derived from gross and fees". | The law is proposed, not written. `UNIFIED-PROPOSAL.md:36` (F5, 6 of 9 reviewers): "**The six judgments are prose.** None states conservation, a cap inequality, non-negativity, liability roll-forward or replay freshness." I confirmed this directly: `judgments.json` defines each judgment as one English sentence plus a `schemaFields` list; the `effect` judgment's definition is "holds when the observations, gross effects, fees, net effects, supply changes, liabilities, and outcome are the authorized state transition" — no arithmetic. Gross lines are `{asset, account, amount}` with no counterparty, which `UNIFIED-PROPOSAL.md:35` flags ("one-account gross lines") and `:112` proposes to fix. |
| D10 | Multilateral netting to zero net obligation | **designed** → **partial** | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:56`: "Netting requires a relation preserving the authorized gross economics, liability ownership and residual duties. A refund cannot replenish authority to evade a gross cap unless that replenishment was explicitly authorized." `ROADMAP.md:48` lists "netting" as a library family. `:72` distinguishes the five composition regimes, matching the kernel's four modes (`defiformal README.md:51-58`). | No settlement-policy object, no lane/participant enumeration, no "commit only when every tracked obligation is zero" predicate. The kernel's `Policy {lanes, participants}` + `Outstanding` + `Residual` (`Atomic/Policy.lean:17-27`) has no Moriarty counterpart. For derivatives this is the clearinghouse abstraction, which the kernel separately flags as its own largest institutional gap (`lane3:406`, "CENTRAL COUNTERPARTY / CLEARING & NOVATION"). |
| D11 | Named finality assumption + liquidation-latency bound | **partial** | `observations[].finality` exists as a field (`stage-relation.schema.json:264-266`). `docs/MORIARTY-CONSOLIDATED-DESIGN.md:72`: "Cross-domain workflow uses authenticated progress, conditional release and compensation; it must never claim global rollback from a local model." `docs/MORIARTY-PRODUCT-CONTRACT.md` via `trust-premises.json` **TP05** (`timeout-not-nonexecution`, status `accepted-assumption`): "**Timeout is not evidence of nonexecution**", elaborated at `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:45` — "A timeout is an observation about time, not proof of nonexecution." | `finality` is an unconstrained string. There is **no latency bound abstraction at all** — nothing that lets a program state "closeout must complete within Δ of the mark observation", which L1 (`element table:741`) makes a hard requirement for any cross-scope `Pf`/`Op` molecule and `:939` makes measurable. TP05 is the sharp one for derivatives: an expiry that passes without an exercise call, or a funding period that passes without an accrual call, is *exactly* the shape TP05 says cannot be read as nonexecution. Moriarty is right about this and has no construct that turns the correct principle into a checkable obligation. |
| D12 | Keeper/automation liveness as an explicit dependency | **covered (as a named assumption)** | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:66`: "Bounded termination does not prove funds can always be recovered: **liquidity, actors, chain inclusion, finality and witness availability are explicit liveness assumptions.**" `:70`: "Under the signed policy, the program/ledger acceptance predicate for any claimed recovery guarantee must establish a viable supported closure/recovery path under named assumptions **or reject a workflow claiming that property**." | I record this as covered at the design level because it is the one D-row where Moriarty is stricter than the kernel: the kernel's `lane2:532` lists keeper liveness as an unnamed gap, whereas Moriarty names it and requires a claimed guarantee to be rejected if unsupported. It is not implemented — no artifact evaluates that rejection — so "covered" here means the abstraction exists and is correctly scoped, not that it is enforced. |
| D13 | Model-priced / non-linear payoff arithmetic under a stated finite profile | **partial** | `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json` fixes `priceOrientation.canonical = "base-per-quote"` with `units.representation = "exact-domain-qualified-integer"` and `fieldElementCoercion = "forbidden"`; widths `UInt128, SInt128, UInt256`. Price scale is bounded 0..18 (`financial-expression-types-v1.ts:95`). `docs/MORIARTY-CONSOLIDATED-DESIGN.md:52` requires an explicit dimensioned conversion for the DeFiFormal quote-per-base ↔ Moriarty base-per-quote mismatch. | `EXIT-GATE.md` records "17 primitives, 6 open gaps; reserve absent" and `UNIFIED-PROPOSAL.md:41` (F10, 3/3 L3 reviewers) that rule D2 is under-specified and `prorata-principal-share` misclassified. Nothing certifies a non-linear primitive. This is partly **out-of-scope-by-design at the language level** — a vol surface is program/library content, and `docs/FOOTGUNS.md` existing-rule 2 forbids promoting product categories into Core constructors — but the *bound* is a language obligation: `LANGUAGE-DESIGN.md:112` requires every profile to register bounds on "schedule events, observations, writes, effects, duties", and no such registry exists. |
| D14 | Instrument listing / expiry-cycle governance | **absent** | Nothing found. A Moriarty program is one `agreement` block (`grammar.ebnf:15`); there is no registry of instruments, strikes or expiry cycles, and no governance-of-listing abstraction. | I record this as **likely out-of-scope-by-design for U0–U3** and unowned thereafter: no milestone in `ROADMAP.md:19-28` names it, and the nearest family line (`:48`, "margin and async/conditional workflows") is a library, not a governed listing process. Flagged because the kernel ranks it a first-class process at *every* derivatives venue (`lane3:424`). |
| D15 | State-partition of collateral (conditional-token split/merge) | **absent** | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:83` has split/join, but it is **proof/history** composition ("bounded split/join and imported histories… predecessor fan-in (at least two for MC06)"), not a partition of collateral into complementary contingent claims. `ROADMAP.md:25` (U4) likewise scopes split/join to history. | The kernel's statement is `lane3:406` — "`Py` splits along time; nothing splits along state." Moriarty has the same shape of hole, one level up: it splits *proofs* along history and has no operator that splits *value* along outcome. Prediction-market and structured-payoff programs need the latter. Unowned. |

### Counts

| Verdict | Count | Ids |
|---|---|---|
| covered | 1 | D12 |
| partial | 8 | D1, D2, D5, D6, D8, D9, D11, D13 |
| absent | 5 | D3, D4, D7, D14, D15 |
| out-of-scope-by-design | 1 (D10 is scored partial; D14 carries a partial out-of-scope note) | — |

I scored D10 **partial** rather than out-of-scope because `ROADMAP.md:48` names netting as a
required library family, so it is in scope and undelivered. No requirement is cleanly
out-of-scope-by-design; D13 and D14 carry out-of-scope *notes* on their library-level
content while retaining a language-level obligation (bounds registration for D13).

Nothing in this table is scored on a checker exit code. All seven U0 checkers exited 0
(`EXIT-GATE.md`, "Checker runs"), and `UNIFIED-PROPOSAL.md:264` states the governing rule
directly: "Do not treat a checker exit of 0, a cited identifier, or a matching fixture as
coverage or enforcement."

---

## 4. Category verdict

**Derivatives are not covered by the Moriarty language design.** The one non-trivial
derivatives-shaped mechanism that actually exists in executable form is the periodic accrual
ratchet in `financial-lifecycle.ts:1755-1810` — a monotone period counter with
`PERIOD_SEQUENCE` and `PERIOD_NOT_ELIGIBLE` refusals and a duplicate-id check — and it is
unilateral, unsigned, fixed-rate, and gated on a caller-supplied `observedTime` that is not a
stage-relation leaf and therefore appears in none of the 84 enforcement rows. Everything else
the category needs is either a named design obligation with no artifact (position lifecycle,
loss allocation, netting, margin libraries) or absent (expiry/exercise/settlement-fixing
distinctions, margin scope, seniority, state-partition). Moriarty's three most relevant
constraints all cut *against* the category and are stated correctly: execution is
finite/bounded (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:66`), history is milestone-scoped
(`ROADMAP.md:25` puts native history at U4), and a timeout is not evidence of nonexecution
(`trust-premises.json` TP05) — which means a lapsed exercise window or a skipped funding
period must be modelled as an authorized transition, never inferred. That is the right
principle and there is no construct implementing it. Against the kernel's own state, the
honest summary is that **both** projects have derivatives as future work: defiformal's
`P27` (`openspec/ROADMAP.md:56`) and `P28` (`:57`) are planned, with `lean/DefiKernel/Margin/`
recorded as a *proposed* output (`sprint-plan.md:287`).

### Milestone ownership on the evidence found

| Id | Owner | Evidence for the assignment |
|---|---|---|
| D1 signed margin / unsettled PnL | **MC05, MC07** (not a U row) | `openspec/REPORT-RECONCILIATION-2026-09-07.md:64`. `ROADMAP.md:5` keeps MC01–MC08 acceptance obligations live. The nearest U row is U6 "margin" as a library family (`ROADMAP.md:48`). |
| D2 periodic funding | **unassigned** | No U row in `ROADMAP.md:19-28` names periodic, recurring or scheduled stages. `LANGUAGE-DESIGN.md:112` requires "schedule events" bounds without an owner. |
| D3 expiry / exercise / settlement fixing | **U3** for the condition half (`ROADMAP.md:24`, "Conditional settlement, partial progress and recovery"); **unassigned** for the time-type half | `LANGUAGE-DESIGN.md:83` states the four-way distinction with no milestone. |
| D4 margin scope | **unassigned**; library content at U6 (`ROADMAP.md:48`) | No document assigns cross-vs-isolated-vs-portfolio scope. |
| D5 position lifecycle / phases | **U3** | `ROADMAP.md:24`. Semantic status recorded at `k-reconciliation.json` UNI-006, UNI-008 (both not-covered). |
| D6 loss allocation ordering | **MC05/MC06 + MC07** | `REPORT-RECONCILIATION-2026-09-07.md:60-61`. U6 for the library (`ROADMAP.md:27,48`). |
| D7 tranching | **unassigned** | No row anywhere names seniority or a waterfall. |
| D8 conditional payoff / observations | **U3**; enforcement at **U2** | `ROADMAP.md:24`; `UNIFIED-PROPOSAL.md:192` puts the S0 `signedIntent.*`/`effects.*` cohort in U2 and "everything else" in U3/U4/U6 (`:193`). `REPORT-RECONCILIATION-2026-09-07.md:65` assigns the insurance row to MC05/MC07. |
| D9 conservation clause | **U0** | `ROADMAP.md:21` (U0 owns the judgments); `UNIFIED-PROPOSAL.md:119` task S3 makes them executable clauses. |
| D10 netting | **U6** library (`ROADMAP.md:48`); relation at **U0/U2** | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:56`. |
| D11 finality + latency bound | finality string at **U0**; latency bound **unassigned** | `ROADMAP.md:21` lists the stage relation; no document mentions a latency or closeout bound. |
| D12 keeper liveness | **U3** (recovery-guarantee rejection) | `docs/MORIARTY-CONSOLIDATED-DESIGN.md:70`; `ROADMAP.md:24`. |
| D13 numeric profile bounds | **U0** (profile) + **U1** (certified primitives) | `ROADMAP.md:21-22`. |
| D14 instrument listing | **unassigned** | — |
| D15 state-partition split/merge | **unassigned** (U4's split/join is history, `ROADMAP.md:25`) | — |

Five of fifteen requirements have no owner in any document I read: D2, D4, D7, D14, D15,
plus the latency-bound half of D11.

---

## 5. Gaps that would change the language design

Ranked by how much new machinery they need. Each of these requires a new construct,
judgment, numeric-profile entry or enforcement locus — not a library or an example.

### G1. There is no time type, and `validity` / `observations[].time` are opaque strings

New **language and kernel** machinery. `grammar.ebnf:57` admits only
`type = identifier, [typeArgs]` over the declarations at `:17-23`; there is no date, time,
period or window type, and `syntax-profile.json:53-60` puts `observation` in
`unsupportedDeclarations`. In the stage relation, `signedIntent.validity`
(`stage-relation.schema.json:170-172`) and `observations[].time` (`:261-263`) are
unconstrained `"type": "string"`. Every derivatives requirement that touches time —
D2 funding periods, D3 expiry and exercise windows, D3 terminal settlement fixing, D11
latency bounds — dies here. The specific distinction the design already asks for and the
schema cannot express is `LANGUAGE-DESIGN.md:83`: *observation timestamp ≠ payment date ≠
exercise window ≠ authorization expiry*. The kernel shows the minimal shape that works —
`now` and `timestamp(key)` as **declared environment reads** carried in the read footprint
(`Typed/Expr.lean:85-86, 98-100`; `Typed/Examples.lean:133-134, 148`) — which is compatible
with Moriarty's finite-execution constraint because it adds no scheduler. Moriarty's own
evaluator already has a partial version of this (`AccrualTerms.periodSeconds/firstPeriodStart`,
`financial-lifecycle.ts:36-42`) that the stage relation cannot see. Closing G1 means: a typed
time/window value, a numeric-profile entry for its width and monotonicity, a schema change,
and a judgment clause. It also forces a decision `UNIFIED-PROPOSAL.md:92` already gestures at
("stage time falls within the validity window") but no artifact enforces.

### G2. There is no position abstraction, and the schema is being closed against one

New **kernel and judgment** machinery. The kernel's requirement is
`openspec/ROADMAP.md:56`/`sprint-plan.md:287-288`: signed margin with "unsettled PnL is not
cash unless the source says so". Moriarty has the signed *types*
(`financial-expression-types-v1.ts:8,154`) and none of the *state*: `liabilities` is a flat
list of one-directional debts (`stage-relation.schema.json:391-455`) and ledger amounts are
required non-negative (`financial-agreement-source.md:64`). Note the direction of travel:
`UNIFIED-PROPOSAL.md:35` treats schema-admissible negative amounts as defect F4 and `:111`
proposes canonical non-negative integers. That fix is right for balances and wrong for
positions, and if S2 lands as written, the one place a signed position could be represented
closes. This needs an explicit decision in U0's numeric profile — a **signedness column**
distinguishing balance-like fields (non-negative) from position-like fields (signed) — which
`UNIFIED-PROPOSAL.md:150` already contemplates in a narrower form ("Add a signedness column
that flags SInt128 liability fields against D5"). Without it, the "reject spending contingent
profit as cash" requirement (`REPORT-RECONCILIATION-2026-09-07.md:64`) has no state to range
over.

### G3. Loss allocation has no ordering, and liabilities have no claim class

New **language** machinery. The kernel's G06 elements are all about *order*: `Ad` is
"**Rank-ordered** forced close when buffers are exhausted" (`element table:247`) and `Sl` is
"Losses assigned to an **explicit claim class**" (`:248`). `Tr` (`:261`) adds declared
seniority, and L6 (`:746`) makes a dispute forum and a recovery-timing assumption mandatory
companions. Moriarty's `liabilities.opening/closing` arrays carry no class, no rank, and no
ordering guarantee (`stage-relation.schema.json:378-455`); the requirement rows exist
(`REPORT-RECONCILIATION-2026-09-07.md:60-61`, "reject out-of-order settlement", "reject erased
residual debt or missing socialized loss") with nothing to check them against. The rule
Moriarty *does* have — "Default does not erase debt, and forgiveness is not repayment"
(`docs/MORIARTY-CONSOLIDATED-DESIGN.md:54`) — is the right invariant and is unenforceable
without a claim-class carrier. This is a schema and judgment change, not a library.

### G4. Judgments are prose, so no derivatives invariant is checkable

New **judgment** machinery, already proposed but not accepted.
`UNIFIED-PROPOSAL.md:36` (F5): "The six judgments are prose. None states conservation, a cap
inequality, non-negativity, liability roll-forward or replay freshness." I verified this
against `judgments.json`: each judgment is one sentence plus a `schemaFields` list. Every
derivatives invariant is an inequality or a roll-forward — margin ≥ maintenance, funding
debits = funding credits, closing = opening + accrual − discharge, payoff = max(S − K, 0) —
and none can be stated. `UNIFIED-PROPOSAL.md:119` (task S3) proposes clauses in "linear
integer arithmetic and set membership over relation paths"; note that a positive-part payoff
and a min-over-scenarios margin (`lane3:126`) are within linear integer arithmetic only with
explicit case splits, so the clause language's expressiveness must be decided against this
category before it is frozen.

### G5. No latency or closeout bound, while TP05 correctly forbids inferring nonexecution

New **enforcement** machinery, currently unowned. L1 (`element table:741`) requires "a named
finality assumption and a **liquidation-latency bound**" for any cross-scope `Pf`/`Op`
molecule; `:939` makes it measurable against "closeout time". Moriarty has `observations[].finality`
as a bare string (`stage-relation.schema.json:264-266`) and the correct principle that
"Timeout is not evidence of nonexecution" (`trust-premises.json` TP05,
`docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:45`). Those two facts together are a
design hazard for this category: TP05 rules out the *lazy* reading of a lapsed exercise
window or a missed funding period, and nothing supplies the *positive* obligation that
replaces it. The construct needed is a signed, checkable bound relating an observation's time
to the deadline of the transition it authorizes. No milestone owns it.

### G6. No multilateral settlement policy

New **kernel** machinery. The kernel's atomic mode commits "only when every tracked obligation
is zero and the policy's supply checks pass" over a declared `{lanes, participants}` policy
(`defiformal README.md:60-64`; `Atomic/Policy.lean:17-27`). Moriarty states the obligation
(`docs/MORIARTY-CONSOLIDATED-DESIGN.md:56`) and has no policy object. For derivatives this is
the clearing abstraction, and the kernel separately ranks the missing clearinghouse as a
project-level gap (`lane3:406`). Ranked below G1–G5 because `ROADMAP.md:48` at least assigns
it a home.

### Not gaps

I checked and am recording these as *correct as designed*, so no reviewer spends effort on
them:

- **No scheduler.** The kernel also has none: every transition is a submitted `Request`
  (`Typed/Transition.lean:36-42`) and time enters only as a declared env read. Moriarty's
  request-driven, bounded model is compatible with periodic funding provided G1 lands.
- **Liveness as an assumption.** `docs/MORIARTY-CONSOLIDATED-DESIGN.md:66,70` is stricter
  than the kernel, which lists keeper liveness as an unfilled gap (`lane2:532`).
- **Pricing models as library content.** Correct under `docs/FOOTGUNS.md` existing-rule 2
  ("Product types and taxonomy categories do not automatically become Core constructors"); the
  language obligation is the *bound registry* (`LANGUAGE-DESIGN.md:112`), not the model.

---

## 6. Limits of this review

1. **I executed nothing but one guarded `status` call.** No build, no checker, no evaluator,
   no K, no proof. Every count I quote for the U0 artifacts is read from
   `EXIT-GATE.md` or computed by me from the JSON in-place (the `Counter({'NOT_ENFORCED': 84})`
   over `enforcement-map.json` rows is mine; it agrees with the recorded line). I did not
   reproduce the seven checker exits.
2. **I did not verify that `financial-lifecycle.ts:1755-1810` behaves as read.** My D2 finding
   is a **source reading**, not an executed trace. `UNIFIED-PROPOSAL.md:284` records that no
   reviewer in the parent study ran the callable evaluator either. `docs/FOOTGUNS.md`
   stop-rule 4 asks for a traced production path before reporting candidate success; I have
   not traced one, so D2's "implemented" grade means "code exists and reads this way", not
   "demonstrated".
3. **Absence claims are search-scoped.** Where I write "nothing found" (D7, D14, D15, and the
   no-time-type claim), that is the result of targeted greps over `docs/`, `ROADMAP.md`,
   `openspec/`, `wiki/`, `experiments/moriarty-language/{spec,src}/` and the two U0
   deliverable directories. I excluded `raw/` and `.raw/` (immutable captures) and `evidence/`.
   This mirrors the limitation the repository itself states for its own searches
   (`EXIT-GATE.md`, Limitations: "absence means not found by this recorded search; it is not
   a proof of non-realisation").
4. **Line numbers in JSON corpus files** (`corpus50/lanes/*.json`) are physical line numbers in
   the pretty-printed file at the pinned commit, obtained by `grep -n`. They are stable at that
   commit only.
5. **I did not read `docs/MORIARTY-PRODUCT-CONTRACT.md` directly.** Its two quotations here
   (the ZKIRv3 target and TP05's "Timeout is not evidence of nonexecution") come through
   `trust-premises.json`, whose own limitation note says "Semantic fit of a quote to its
   statement label is a reviewed claim, not mechanically proven" (`EXIT-GATE.md`, Limitations).
6. **I did not read `algebra/EXTENSIONS.md` or `docs/unified-v0.1.md` in full**, only their
   headings. `EXTENSIONS.md` §"Group 5 — the order in which capital is destroyed" (its heading
   at `:156`) is plausibly load-bearing for G3 above and I did not verify its content; another
   reviewer should. `algebra/MODEL.md:1-9` carries a scope correction stating it is a
   historical research record superseded by the semantic-kernel migration, so I did not rely
   on its algebra for any coverage verdict.
7. **Milestone assignments are inferences** from `ROADMAP.md:19-28,48`,
   `REPORT-RECONCILIATION-2026-09-07.md:60-65` and `UNIFIED-PROPOSAL.md:190-196`. Where I
   write "unassigned", I mean *I found no document assigning it*, which is weaker than *no
   assignment exists*.
8. **`UNIFIED-PROPOSAL.md` is a proposal, not accepted work** (`:4`). Where I cite its tasks
   (S2, S3, N2, T5) as the route to closing a gap, that is a statement about a proposed plan,
   not about accepted scope.
9. **One factual tension I could not resolve.** The kernel's element table carries `Dp`
   "Directional position and hedge maintenance" as a *candidate* whose promotion gate is
   "Evidence it is not `Pf` plus `Ct` plus parameters" (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:356`),
   and `:366-371` records that decomposing Ethena hit central residue because "`Pf` is a
   funding transfer, **not the short perpetual position**". Whether a directional position is
   an irreducible abstraction or a derived one is therefore unsettled *in the kernel*. My D1
   verdict treats position state as required; if the kernel later demotes `Dp`, D1 weakens to
   a margin-plus-funding composition. I flag this rather than pick a side.

---

## Architecture axis (follow-up)

Added after a coordinator follow-up. Read in full for this section: `ROADMAP.md` (56 lines)
and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines). Section 1–6 above is unrevised.

**1. Placement.** Derivatives are placed as **source libraries over the core**, twice and
consistently. `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93`: "Build financial breadth as reusable
libraries: exact arithmetic/fees; payments and escrow; loans and claims; swaps and liquidity;
shares/vaults; ACTUS cash-flow contracts; redemption/loss allocation; **margin**; asynchronous
and conditional claims." `ROADMAP.md:48` repeats it: "Libraries supply ACTUS, loans/claims,
payments/escrow, AMMs, vaults, netting, redemption/loss allocation, **margin** and
async/conditional workflows." Not a core construct (`:15`), not a Federated DeFi Kernel service
(`:11` scopes the kernel to solver discovery, evidence, custody, routing, finality observation
and recovery), not a settlement adapter (`:38`). The placement is coherent and, for payoff
logic, correct.

**2. Core mechanisms needed.** The core enumeration at `:15` is: "exact typed values; bounded
evaluation; explicit state and effects; checked authority; assertions and refinements;
authenticated evidence; persistent obligations and continuations; and defined composition
operators."

| Need | Named in architecture? | Component designated? |
|---|---|---|
| Time/instant type | **No.** Absent from `:15`. `grep` over both documents returns no "clock" or "calendar"; "time" appears only as an observation *field* (`:42`) and in expiry prose (`:64,:68,:70`). | None |
| Expiry / exercise window | Partly: `:68` "no fixed default expiry or perpetual spending right is implied"; `:70` recovery grants "declare their own signed termination rule: an expiry or owner-chosen indefinite duration". Exercise windows: not named. | None. `:115` records the whole-language audit already found "expiry and roadmap conflicts" |
| Signed position with direction | **No.** `:42` binds "opening and closing liabilities" — one-directional. No position object anywhere. | None |
| Mark-to-market as a recurring stage | **No.** `:66` gives continuations but only as bounded progression; nothing recurring, periodic or time-triggered. | None |
| Loss allocation with seniority | Named as a library ("redemption/loss allocation", `:93`); seniority, claim class and ordering not named. | Library slot only |

"Conditional settlement and bounded stages" is the strongest section and still does not reach
the category. `:62` gives a nine-state workflow list and then explicitly declines to fix the
representation — "even if the final syntax represents them with several typed records rather
than one enum" — so position lifecycle has a requirement with a deliberately deferred
realization. `:64`'s timeout doctrine ("A timeout changes which authorized transitions may be
attempted. It does not prove another chain did not execute") is correct and is the doctrine
that makes a *positive* time construct mandatory rather than optional: if a lapsed exercise
window cannot be inferred, it must be asserted, and nothing in `:15` can assert it.

**3. Architectural holes — requirements stated with nothing on the other side.**

- **H1. The stage statement requires a time-bearing observation the core has no type for.**
  `:42` mandates every accepted stage bind "typed observations with issuer, domain, **time** and
  finality"; `:15` enumerates no time type and the Responsibility boundary table's Conditions
  row (`:30`) assigns "freshness" to the language column with no carrier. Freshness is a
  predicate over two instants. Neither instant has a type. This is the load-bearing hole for
  D2, D3 and D11.
- **H2. A recovery-guarantee decision procedure with no owner.** `:70`: "the program/ledger
  acceptance predicate for any claimed recovery guarantee must establish a viable supported
  closure/recovery path under named assumptions **or reject a workflow claiming that
  property**." That is a rejection obligation on the acceptance path. No row of the
  responsibility table (`:23-37`) owns it; the Recovery row (`:34`) assigns the language only
  "authorized remedy, residual duties, budgets, late results and exclusive terminal outcomes".
  For a derivative this predicate is closeout feasibility, and nothing computes it.
- **H3. Netting is required as a relation with no producer or checker.** `:56`: "Netting
  requires a relation preserving the authorized gross economics, liability ownership and
  residual duties." No component is named; `ROADMAP.md:48` lists netting as a *library*, but a
  preservation relation between gross and net is a judgment, not library content. Compare the
  kernel, which makes it a settlement policy object (`defiformal README.md:60-64`).
- **H4. Latency and closeout have no row at all.** The Atomicity row (`:32`) distinguishes
  "atomic batch, committed prefix, independent fork/join and interleaving" and assigns finality
  to the external domain. L1 (`defiformal docs/UNIFIED-DEFI-ELEMENT-TABLE.md:741`) requires a
  *liquidation-latency bound* for any cross-scope perp or option. No column of the table
  accepts that obligation.
- **H5. Recurring stages are excluded by omission, not by decision.** `:66` bounds every stage
  and routes long life through continuations, but no text says whether a continuation may be
  gated on time. The architecture neither grants nor forbids a periodic stage — yet
  `financial-lifecycle.ts:36-42,1761-1770` already implements one. An implemented mechanism with
  no architectural statement is the definition of a hole.

**4. Canonical stage statement.** `:42` binds profiles, identities, domain/state frame, signed
intention, lifecycle IDs, predecessor and obligation commitments, typed observations, gross/net
effects with fees and supply changes, opening/closing liabilities, authority and replay state,
resources, disclosures, failure/phase policy, and continuations or terminal outcome. Omissions
for this category: **no position or exposure**, **no collateral/margin state**, **no mark or
index value distinct from an observation**, **no schedule or next-eligible-event**, **no claim
class or seniority rank**, and **no time bound relating an observation's time to the deadline
of the transition it authorizes**. On signedness the architecture is **silent, not committed**:
`:52` fixes "canonical price orientation, unit dimensions, per-primitive rounding direction and
beneficiary policy" and forbids field-element coercion, but says nothing about sign; `:54`
makes liabilities persistent and one-directional. So the S2 proposal to make amounts canonical
non-negative integers (`UNIFIED-PROPOSAL.md:111`) would be a schema decision taken **without
architectural cover either way** — which is precisely why it should be decided here rather than
in a schema revision.

**5. Ownership of the architecture work.** Split, with the core piece unowned. U0 owns the stage
relation and numeric profile (`ROADMAP.md:21`), so H1's *field* and the signedness decision sit
there. U3 owns "Conditional settlement, partial progress and recovery" (`:24`), so the workflow
states of `:62` and H2 sit there. U6 owns the library families (`:27,:48`), so margin and loss
allocation as libraries sit there. **Unassigned: the core-extension architecture itself** — no
milestone in `ROADMAP.md:19-28` names extending the `:15` enumeration, no milestone names time,
recurring stages, positions or seniority, and H3–H5 have no owner. `ROADMAP.md:30` permits
independent preparation but assigns nobody.

**6. Minimum addition so derivatives can be built later without redesigning the core.** Five
statements, all architectural, none requiring an implementation now:

1. **Add a time/instant value to the `:15` core enumeration**, with its width and monotonicity
   registered in the numeric profile, and state that freshness at `:30` and "time" at `:42` are
   that type. This alone unblocks expiry, exercise windows and staleness.
2. **Take a position on signedness in `:52`**, distinguishing balance-like fields (non-negative)
   from position-like fields (signed), so a position has somewhere to live before S2 closes it.
3. **State whether a continuation may be time-gated**, and if so that its eligibility carries a
   non-skip / non-replay counter. This is the generalization of the accrual ratchet already in
   the evaluator, and it converts mark-to-market and funding from new core work into a library
   over an existing form.
4. **Give liabilities a claim-class / rank carrier in `:42`**, so `:54`'s "default does not erase
   debt" and `REPORT-RECONCILIATION-2026-09-07.md:60-61`'s ordering requirements have a target.
5. **Add one row to the responsibility table (`:23-37`) for timing bounds** — who states a
   closeout/latency bound, who checks it, and what the external domain contributes — closing H4
   and giving H2's "viable recovery path" predicate an owner.

With those five, margin, perps and options are genuinely library work at U6 as `:93` intends.
Without item 1 in particular, the library slot named at `:93` and `ROADMAP.md:48` cannot be
filled by anyone, because the core has no way to say when anything happens.

**Verdict on this axis:** the architecture **places** derivatives correctly and **does not
architect** them. The placement sentence exists; the mechanisms the placement presupposes do
not. This is weaker than a specification gap: for H1, H4 and H5 nobody currently knows what to
write, because no document says which component owns it.
