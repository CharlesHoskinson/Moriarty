# R2 — lending and borrowing: DeFi Kernel abstraction versus Moriarty language design

Reviewer R2 of nine. Read-only review. This file is the only artifact written.

**Status: review record. Nothing here accepts, closes or reassigns any U0–U7 predicate.**
Under `AGENTS.md:95-99` work is accepted by evidence, not by review.

---

## 1. Scope and pins

### Commits

| Repository | Command run | Output |
|---|---|---|
| defiformal | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` (`U0 T7: add U0 exit gate`, Wed Sep 23 21:00:14 2026 -0600) |

All defiformal file:line citations below are at `8c5dd10`; all Moriarty citations at `8f73784`.

### Required startup

`/home/charl/Moriarty/AGENTS.md:10-13` requires the `moriarty-dev:develop` skill. The host does not
expose it, so I read and applied the checked-in copy at
`plugins/moriarty-dev/skills/develop/SKILL.md`, then read `AGENTS.md` and `docs/FOOTGUNS.md`.

I ran the guarded CLI status (this is the only state-touching command I ran):

```bash
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Actual output (abridged to its fields): `capability` = `SP01.6 loan-swap-subset`;
`blockedAction` = `implementation/repair of loan-swap-subset`; `nextAction` = `sp01-loan-report`;
`missingEvidence` = `binding-input-stale:openspec/sprints/sp01-financial-contract-and-execution-admission.md`,
`candidate-input-stale:` (same file), `current-accounting-missing:.moriarty-dev/runtime/current-accounting.json`,
`resource-live-state-unavailable:sp01-loan-swap-grok-01`, `operational-history`;
seven `pendingTransactions` IDs. I did not run `next`, `run`, `report` or `doctor`, and I dispatched
no campaign.

### What I read

**defiformal:** `README.md`; `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` (§1–§18 read; §19–§20 skimmed);
`lean/README.md`; `lean/DefiKernel/Core.lean`, `Examples.lean`, `ContractExamples.lean`;
`algebra/REQUIREMENTS.md`; `algebra/MODEL.md` §1–§4; `openspec/ROADMAP.md`;
`corpus50/lanes/lane1-dex-lending-cdp-lsd.json` (Lending and CDP categories in full).

**Moriarty:** `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (all 288 lines);
`deliverables/u0-semantic-contract-2026-09-23/{judgments,numeric-profile,enforcement-map,trust-premises,source-core-embeddings}.json`;
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` (leaf enumeration);
`ROADMAP.md`; `docs/MORIARTY-CONSOLIDATED-DESIGN.md`; `docs/MORIARTY-PRODUCT-CONTRACT.md`;
`docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`;
`experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf`;
`spec/successor/examples/loan-lifecycle.mori`, `financial-vault-quote.mori`;
`src/successor/financial-lifecycle.ts` (type declarations, `convertNominal`, `allocateNominal`,
`applyAccrue`, `runActions`, `prepareFinancialLifecycle`); `formal/k/lifecycle-kernel.k` (all 56 lines);
`spec/target-crosswalk.json`; `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md` and
`action-targets.csv`; `deliverables/erc4626-vault-report-2026-09-08/{DESIGN-IMPLICATIONS.md,crosswalk.csv}`;
`wiki/defiformal-taxonomy.md` (lending-relevant claims); `wiki/defi-kernel-sdk-interface.md`;
`deliverables/k-lifecycle-execution-2026-09-17/RESULT.md`;
`deliverables/preview-loan-2026-09-17/recovery-run01/RESULT.md`;
`compact/generated/loan/kernel.compact` (head).

### What I did *not* execute

I did not build Lean, run `lake`, run the K suites, run the TypeScript evaluator, run any Moriarty
test file, or re-run the U0 checkers (`check_u0_exit_gate.py`, `check_u0_target_pins.py`,
`check_u0_numeric_profile.py`). I did not compile, prove, or submit anything. Every executed-evidence
claim below is read from a retained receipt and is labelled as such.

Other than the `status --json` call, my commands were `git log`/`rev-parse`, `ls`, `cat`, `sed`,
`grep`, `wc`, and short `python3 -c` JSON inspections. Where I state a count (84 leaves, 104 rows,
66 absent embeddings), it came from one of those inspections and I say so.

---

## 2. What the kernel abstraction requires for this category

The DeFi Kernel says two different kinds of thing about lending, and they must be kept apart.

### 2.1 The kernel's own model abstractions (what any lending model is built from)

1. **A ledger keyed by (domain, account, asset)** with nonnegative balances —
   `defiformal/README.md:23`. A domain is an execution context such as a modelled chain
   (`:24`).
2. **Asset-specific amount and price types; exact rational arithmetic** — `README.md:25-27`
   ("a share amount and a dollar amount cannot be interchanged without an explicit conversion").
3. **Registered templates** that declare arguments, execution conditions, balance changes, minting
   and burning, *and which ledger entries they may read and write and which external data they use*
   — `README.md:29-33`.
4. **Capability IDs**, separately granting invocation, debit of a particular balance, and change of
   an asset's supply; issuable and revocable — `README.md:35-38`. Debit and supply authority are
   distinct in the Lean type as well: `lean/DefiKernel/Core.lean:44-46`.
5. **Per-asset, per-domain conservation**: total balance change must equal declared minting minus
   burning — `README.md:41-42`; proved as `applyEffect_accounting` at
   `lean/DefiKernel/Core.lean:127-130`.
6. **Refusal semantics with a reason** — `README.md:43`; the reason enumeration is
   `lean/DefiKernel/Core.lean:56-58`.
7. **Components with private/shared state and typed ports; workflows; four execution modes**
   (sequential, disjoint parallel, shared-state interleaving, atomic) each with explicit failure
   behaviour — `README.md:45-47` (components and ports) and `README.md:51-57` (the mode table).
   Atomic settlement commits only when every tracked net obligation is zero (`README.md:62`).
8. **A declared oracle observation that is an input, not a truth.** The pilot's borrow transition
   carries `Oracle { feed, price, observedAt, now }` (`lean/DefiKernel/Examples.lean:68-72`) and its
   guard requires feed identity, positive price, non-future and at-most-five-old timestamps, and a
   200 % collateralisation inequality over the pre-state's declared locked collateral
   (`Examples.lean:78-87`). `lean/README.md:52-54`: "checking them does not establish provenance or
   market truth."
9. **A contract layer whose collateral and oracle conditions are checked independently of the
   proposal's own guard** — `lean/DefiKernel/ContractExamples.lean:34-44` (`BorrowConditions`) and
   `lean/README.md:44-46`: "replacing a proposal's own guard with `true` cannot remove those rules."
   The pilot ships the counterexample, `forgedBorrow`, at `ContractExamples.lean:53-54`.
10. **Debt as a distinct nonnegative obligation token** in the reference example, with the explicit
    caveat that this "is not a general party/claim lifecycle model" — `lean/README.md:56-58`.
11. **A party/obligor sort is mathematically necessary.** `algebra/REQUIREMENTS.md:69-76` (R4) and
    `algebra/MODEL.md:133-135`: "No observation over on-chain mechanism sets separates USDT from
    USD1. The minimal enrichment that does is a party sort carrying obligor, attester and
    jurisdiction." Reflexive collateral cycles need `(element, asset)` indexing
    (`algebra/MODEL.md:122-126`).
12. **Prohibitions must be global constraints, not per-element conditions** —
    `algebra/REQUIREMENTS.md:104-108` (R9).
13. **Per-molecule residue is a required output, never hidden** —
    `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:30` and `:729-733` (§12.4).

### 2.2 The Atlas elements, discriminators and laws this category depends on

| Atlas item | Where |
|---|---|
| `Pl` pooled lending, `Im` isolated lending market, `Cd` collateralized-debt minting, `Uc` undercollateralized credit, `Ft` fixed-term debt (group G05, all S3) | `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:232-238` |
| `Ct` collateral-threshold solvency test, `Li` incentivized liquidation, `Ad` auto-deleveraging, `Sl` socialized-loss allocation, `Bs` staked backstop (group G06, all S3) | `:245-251` |
| `Sh` pro-rata share accounting, `Ix` index-based accrual, `Rb` rebasing (group G01, S0) | `:170-172` |
| `Fl` atomic flash liquidity — "borrow and repay within one settlement scope or revert", the **only** async-impossible element | `:218`, `:851-856` |
| `Ex` / `Tp` / `Oa` / `At` truth sources, all S2 | `:272-277` |
| Mandatory isotope `Li{mode = auction, fixed-bonus, partial}` | `:455` |
| Mandatory isotope `Im{mode = isolation, e-mode, single-base}` | `:460` |
| Mandatory discriminator `obligor` on `Uc`, `Ft` and off-chain-backed `Cd` | `:506` |
| Law **L1** `(Pl\|Im\|Cd\|Pf\|Op) → (Ex\|Tp\|At) + Ct + (Li\|Ad\|Sl\|Bs)`; not async-safe — cross-domain also needs a named finality assumption and a liquidation-latency bound | `:741` |
| Law **L2** `Pl → (Sh\|Ix) + "exit-liquidity"` | `:742` |
| Law **L3** `Uc → Aw + At{subject=borrower-financials} + (Bs\|Tr) + obligor` | `:743` |
| Law **L7** `Cd → Rd \| Ps \| liquidation capacity` | `:747` |
| Law **L13** `Ex → freshness validation` | `:753` |
| Law **L14** illiquid backing `→ Wq \| bounded liquidity reserve` | `:754` |
| Hazards **X2** (flash + manipulable price + `Pl`/`Cd` where manipulation cost < position value), **X8** (shared collateral across nominally isolated markets), **X18** (`Oa` as sole truth for high-frequency liquidation) | `:790`, `:796`, `:807` |
| **Standing rejection:** ERC-4626 is an interface bond, a socket, not an element | `:143-147` |
| **Standing rejection:** a health factor is a *derived observation* — "it changes nothing" | `:155-156` |

Note the direction of L1: it is a **well-formedness law over a program's declarations**, not a
runtime check. A molecule that creates a liability without declaring a truth source, a solvency test
and a loss allocator is *ill-formed*, and `:52-54` states the point explicitly — "ill-formedness is
where the risk was hiding."

### 2.3 What the kernel says lending needs that the Atlas itself cannot express

The corpus50 lending lane records these as residue, i.e. the kernel's own declared gaps. They matter
here because Moriarty inherits them as targets, not as solved problems:

- **The interest-rate model.** "no element for the utilization-indexed interest-rate curve … `Ix`
  says an index accrues, nothing says what sets the rate at which it accrues" —
  `corpus50/lanes/lane1-dex-lending-cdp-lsd.json:105`. The category summary calls it "absent and
  load-bearing in all six" (`:203`).
- **The close factor** — "the cap on how much of an underwater position one liquidation may repay.
  This is a distinct solvency mechanism from `Ct` and `Li`" — `:159`.
- **The two-threshold `Ct`** (open vs close collateral factor) — `:193`.
- **Credit delegation** — namable only by the provisional `Cg` — `:108`, with `Cg` listed provisional
  at `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:384`.
- **Supply caps, borrow caps and debt ceilings** as risk primitives — `:107`.
- **Correlated-asset margin relief (E-Mode)** — `:106`.
- **Delegated risk curation** — `:125`.
- **Recapitalisation by governance-token dilution** as distinct from `Sl` and `Bs` — `:217`.
- **Continuous soft liquidation** — `:309`.

`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:809-813` adds that the hazard rows themselves still contain
undefined terms and are not yet measurable predicates.

---

## 3. Coverage verdict per requirement

Verdict vocabulary, applied strictly:

- **covered** — the abstraction exists in Moriarty at the layer stated, and I can cite either
  evaluator/K code that implements it or a retained receipt that executed it.
- **partial** — a named aspect exists; a named aspect is missing.
- **absent** — no declaration found in the source language, the Core/lifecycle kernel, the K
  semantics, the stage relation or the numeric profile.
- **out-of-scope-by-design** — a Moriarty document states the divergence and gives a reason.

I also grade *where* coverage sits: **designed** (prose/proposal only), **specified** (frozen schema
or contract), **implemented** (evaluator or K code), **demonstrated** (a retained executed receipt).

### 3.1 Kernel model abstractions

| # | Requirement (defiformal cite) | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| K1 | Ledger keyed by **(domain, account, asset)**, nonnegative (`README.md:23-24`) | partial | `experiments/moriarty-language/src/successor/financial-lifecycle.ts:44-48` — `Balance {party, asset, amount}`, no domain. Domain exists only at stage level: `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` leaves `domain.chainId`, `domain.domainId`, `domain.stateFrameRef` | No domain key on a balance. `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:8` states the current same-name source-unit/settlement-asset restriction "is narrower than the target's domain-qualified financial model" |
| K2 | Asset-specific amount/price types; **exact rationals** (`README.md:25-27`) | partial / out-of-scope-by-design for rationals | Typed: `financial-expression-types-v1.ts:8` (`Amount`, `Shares`, `Rate`, `Price`, `Quantity`). Rationals replaced by checked unsigned integers plus declared rounding: `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json` primitives `checked-add-u128`, `convertNominal`, `allocateNominal` | Rational exactness is deliberately traded for finite-width + directed rounding (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:52`), which also records the unresolved orientation mismatch: "DeFiFormal prices are quote-per-base while the current Moriarty convention is base-per-quote". `UNIFIED-PROPOSAL.md:146` records that unit *dimensions* are still absent from the profile |
| K3 | Template declares **read and write footprint and external data used** (`README.md:29-33`) | partial | Writes: `financial-agreement-source-v5-grammar.ebnf:61` (`next.field = expr`). Effects: `:63`. Reads: `:124-131` financial reads | No declared read footprint; no declared external-data set. The source/5 compiler hard-codes `observations: {}` (`src/successor/financial-agreement-source-compiler.ts:606`) |
| K4 | **Capability IDs** for invoke / debit / supply, issuable and revocable (`README.md:35-38`; `Core.lean:44-46`) | partial | `financial-lifecycle.ts:50-55` — `Allowance {party, asset, remaining, spent}` is a per-(party,asset) debit budget | No capability object, no invocation permission, no supply authority, no issue/revoke. `source-core-embeddings.json` classifies `authority.consumed` and `authority.remaining` as `partial` with the note "there is no `authority.consumed` record" |
| K5 | **Per-asset conservation**: Σ balance change = mint − burn (`README.md:41-42`; `Core.lean:127-130`) | partial (designed) | Proposed as law E1, `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:90` ("per-asset conservation: Σ gross = Σ supply changes = 0") | Not in the frozen contract. `judgments.json` `effect` is a prose definition plus a field list; it states no conservation equation — this is the study's own finding F5 (`UNIFIED-PROPOSAL.md:36`). The lifecycle kernel emits **no supply changes at all**: `source-core-embeddings.json` marks all three `effects.supplyChanges[].*` leaves `absent` with the note "No supply-change account exists in source/5 or core/1" |
| K6 | **Refusal with a reason** (`README.md:43`) | covered — implemented and demonstrated | `financial-lifecycle.ts:336` `rejected(code, actionIndex)`; `formal/k/lifecycle-kernel.k:6` `lcFailed(String)`; codes `EXCEEDS_OUTSTANDING`, `INSUFFICIENT_BALANCE`, `ALLOCATION_COMPONENT`, `LIABILITY_CAP_EXCEEDED` at `lifecycle-kernel.k:52` and `financial-lifecycle.ts:1427, 1461, 1569, 1687`. Executed: `deliverables/k-lifecycle-execution-2026-09-17/RESULT.md:3` — "Comparisons cover … rejection codes and rollback", 104 lifecycle cases | Refusal is atomic-only. The four execution modes' distinct failure behaviours are not modelled (see K7) |
| K7 | Components/ports/workflows and the **four execution modes**, incl. atomic net-obligation-zero settlement (`README.md:45-66`) | partial (designed) | Named as five composition operators, spellings unspecified: `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:122`. Restated as target interfaces: `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:37` | Implemented behaviour is single-stage all-or-nothing: `financial-lifecycle.ts:1885-1906` rejects on the first failing action and discards the tentative post-state. No ports, no shared state, no interleaving schedule, no settlement policy. `ROADMAP.md` assigns conditional settlement/partial progress to U3 |
| K8 | **Declared oracle observation** with feed identity and a staleness bound, treated as input not truth (`Examples.lean:68-87`; `lean/README.md:52-54`) | absent (in source/5) | `source-core-embeddings.json`: all five `observations[].{domain,finality,issuer,kind,time}` leaves are `absent`; the note for `observations[].kind` reads "source/5 sets observations to an empty object". The older bounded-atomic Compact kernel carries exactly one scalar time observation: `compact/generated/loan/kernel.compact:8` `KernelObservations { o0 }`, asserted `< 2000000000` at `:15` | No price observation, no feed identity, no issuer, no freshness, no finality. `deliverables/defi-language-design-2026-09-07/action-targets.csv:21` (DA20) disposition is "simulated observations only". TP03 is open (`trust-premises.json`, TP03 `observations-finality`, status `open`) |
| K9 | **Contract layer independent of the proposal's own guard** (`ContractExamples.lean:34-44`; `lean/README.md:44-46`) | partial, and inverted | The Moriarty analogue of the `forgedBorrow` counterexample is finding F2: the written source guards at `spec/successor/examples/loan-lifecycle.mori:119-121` do not carry the safety argument; the Core evaluator and K do — `UNIFIED-PROPOSAL.md:33` | Moriarty has the kernel checks but **no language-level way to state them as the program's own conditions and no independent contract selection**. The proposal lifts L1–L7 into judgment clauses at `UNIFIED-PROPOSAL.md:119` (S3) and `:155-161` (N4); both are unexecuted |
| K10 | **Debt as a distinct nonnegative obligation** with an explicit "not a general claim lifecycle" caveat (`lean/README.md:56-58`) | covered at lifecycle scope — implemented and demonstrated | `financial-lifecycle.ts:63-84` `LifecycleObligation` with `principal`, `accrued`, `outstanding`, `status`, `nominalLiabilityCap`; K equivalent `lifecycle-kernel.k:33` (the obligation record built by `lcOriginateChecked`). Executed: `k-lifecycle-execution-2026-09-17/RESULT.md:3` — outstanding follows 0 → 100 → 110 → 80 → 0 across Originate/Accrue/partial Repay/final Repay | This is Moriarty's strongest lending abstraction and it is genuinely stronger than the Lean pilot's single debt token. It is still not projected into the frozen stage relation: `source-core-embeddings.json` marks every `liabilities.opening[].*` and `liabilities.closing[].*` leaf `partial` with "there is no opening-liability array and no liabilityId" |
| K11 | **Party / obligor sort** carrying obligor, attester, jurisdiction (`algebra/REQUIREMENTS.md:69-76`; `algebra/MODEL.md:133-135`) | partial | `financial-agreement-source-v5-grammar.ebnf:36` (`party` declaration); `financial-lifecycle.ts:63-84` carries `debtor` and `creditor` as `Identifier` (a string) | Parties are bare identifiers. No obligor class, no `claim_perfection`, no `record_authority`, no attester. `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:33` states as a *target* interface that "Creating or increasing an obligation requires the affected party's applicable consent" — designed only |
| K12 | **Prohibitions as global constraints** (`algebra/REQUIREMENTS.md:104-108`) | absent | — | No hazard, prohibition or global-constraint construct anywhere in the grammar, the lifecycle kernel, the K semantics or the stage relation |
| K13 | **Residue reported per molecule, never hidden** (`UNIFIED-DEFI-ELEMENT-TABLE.md:30-32`, `:729-733`) | covered at the register level, absent at the program level | `experiments/moriarty-language/spec/target-crosswalk.json` carries `unsupported_behavior` and `necessary_extension` per row, e.g. `:1025` for Aave V3 | It is a coverage register, not a language output. A Moriarty program emits no statement of what it failed to express |

### 3.2 Atlas element and law requirements

| # | Requirement (defiformal cite) | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| A1 | `Pl` **pooled lending** — shared pool, many lenders and borrowers (`:232`) | absent | `spec/target-crosswalk.json:999` DEFI:6 Aave V3 → `candidate_first_profile_support: unsupported-row-conformance`; `deliverables/defi-language-design-2026-09-07/action-targets.csv:5` DA04 "supply / redeem lending claims" → `needs-extension` | `LifecycleState` (`financial-lifecycle.ts:129-139`) has `balances`, `allowances`, `obligations`, used-ID sets and `work`. There is no pool, no lender claim, no share supply, no utilization |
| A2 | `Im` **isolated lending market** + mandatory `mode{isolation, e-mode, single-base}` (`:233`, `:460`) | absent | `spec/target-crosswalk.json:1030` DEFI:7 Morpho → unsupported; required extension text at `:1056` names "Market-selected oracle/IRM; isolated risk; curator/market-cap management overlay" | No market partition. Obligations are a flat array bounded at 128 (`financial-lifecycle.ts:14` `collectionCapacity: 128`). Nothing distinguishes isolated from cross exposure, so hazard X8 is not even statable |
| A3 | `Cd` **collateralized-debt minting** — mint a liability against locked collateral (`:234`) | absent | `action-targets.csv:10` DA10 "issue / burn debt-backed stablecoin" → `needs-extension`; `spec/target-crosswalk.json:1185` DEFI:12 Sky → unsupported | No collateral lock, no supply change. `Originate` (`financial-lifecycle.ts:103-118`) requires a matching funded `Transfer` in the same step (`lifecycle-kernel.k:31` `TRANSFER_NOT_IN_STEP`, `TRANSFER_AMOUNT_MISMATCH`) — it is a funded loan, not a mint against collateral |
| A4 | `Uc` **undercollateralized credit** + mandatory `obligor` (`:235`, `:506`), law L3 (`:743`) | absent | `spec/target-crosswalk.json:1123` DEFI:10 Maple → unsupported; required extension names "F2+F5 obligor; underwriting; custody; servicing/refinance/write-down authority; recourse" at `:1149` | No obligor, no attestation, no recourse, no underwriting record. Hazard X11a (`Uc` with none of `Aw`/`At`/collateral/reputation, class **F**, `:799`) cannot be checked |
| A5 | `Ft` **fixed-term debt** — maturity-dated claim with a discount factor (`:236`) | partial | `financial-lifecycle.ts:36-42` `AccrualTerms {numerator, denominator, rounding, periodSeconds, firstPeriodStart}`; period sequencing enforced at `lifecycle-kernel.k:36` (`PERIOD_SEQUENCE`, `PERIOD_NOT_ELIGIBLE`) | A schedule exists; a **maturity** does not, and neither does a discount factor, a default event or a forfeiture rule. `action-targets.csv:8` (DA07) names "maturity before forfeiture" as the distinguishing test and dispositions the row `needs-pinned-protocol-fixture` |
| A6 | `Ct` **collateral-threshold solvency test** — the margin/LTV/health computation and its threshold (`:245`) | absent | The whole-repo grep for `collateral\|liquidat\|health.?factor\|LTV\|encumbr` over `experiments/moriarty-language/{spec,src,formal}` and `openspec/changes/consolidated-language-kernel` returns **only** `spec/target-crosswalk.json` rows — i.e. the coverage register, never an implementation. Required-extension text at `spec/target-crosswalk.json:1011`: "Accrual; health; liquidation thresholds/order; bad debt and loss bearer" | No collateral state, no price observation, no threshold inequality. The one appearance of the word in the numeric profile is a *rounding-direction role*: `numeric-profile.json` primitive `expression-obligation-division` — "D2 requires ceil when the result is an amount owed, a fee, or a collateral charge" — which presumes a collateral charge the language cannot produce |
| A7 | `Li` **incentivized liquidation** + mandatory `mode{auction, fixed-bonus, partial}` (`:246`, `:455`); close factor and bonus | absent | `action-targets.csv:8` DA07 "liquidate / recognize default" → `needs-pinned-protocol-fixture`; its distinguishing test is "partial liquidation and close factor" | No seizure, no bonus, no close factor, no auction, no keeper. Note the kernel's own gap here: the close factor is Atlas residue (`corpus50/.../lane1:159`), so this needs a *new* abstraction on both sides |
| A8 | `Ad` **auto-deleveraging** (`:247`) | absent | No DA row; no crosswalk row names it | Nothing. No U milestone or DA row names auto-deleveraging (**unassigned**) |
| A9 | `Sl` **socialized-loss allocation to an explicit claim class** / bad debt (`:248`) | absent (designed) | Named as a planned library family: `docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` "redemption/loss allocation"; `LANGUAGE-DESIGN.md:118` "Cancellation, default, write-off and loss allocation need named authority and effects"; required extension "bad debt and loss bearer" at `spec/target-crosswalk.json:1011` | No claim class, no waterfall, no write-down transition. `AGENTS.md:83` states the opposing invariant ("Residual duties survive partial progress"), and `lifecycle-kernel.k:47-48` `lcStatus` only maps outstanding 0 → `Settled`; there is no impaired or written-off status |
| A10 | `Bs` **staked backstop** / insurance reserve + `trigger` isotope (`:249`) | absent | — | No first-loss capital, no slashable stake, no reserve fund. `Ir` insurance reserve fund is itself provisional in the Atlas (`:384`), so again both sides lack it. **Unassigned** in Moriarty beyond the blanket MC07 row text at `spec/target-crosswalk.json:1013` ("liquidation fee/backstop") |
| A11 | `Ix` **index-based accrual** — "a global exchange-rate or debt index changes claim value" (`:171`) | partial, and materially narrower | `financial-lifecycle.ts` `applyAccrue`; `numeric-profile.json` primitive `accrual-interest`, which cites `financial-lifecycle.ts:1779`: "applyAccrue divides **principal** times numerator by the accrual denominator"; K equivalent `lifecycle-kernel.k:36`, where every accrual term reads `ejGet(O,"principal")` | Three separate shortfalls. (a) The accrual basis is `principal`, not `outstanding`, so **compound interest is not expressible**. (b) Accrual is per-obligation, not a **global index**, so `Ix`'s defining property — one index moving every claim — is absent. (c) `AccrualTerms` is fixed at origination and no action updates it, so **variable rates are not expressible** |
| A12 | **Variable vs fixed rates; the interest-rate model** (Atlas residue, `corpus50/.../lane1:105`, `:203`) | absent | Required extension text repeated on every lending row, e.g. `spec/target-crosswalk.json:1025`: "RateObservation/IRM policy … Model omits IRM/compounded-rate calculation; **supplied interest is not rate-model coverage**" | No rate observation, no utilization, no kinked curve, no rate-update authority. Moriarty's own crosswalk already says the supplied accrual does not count |
| A13 | `Sh` **pro-rata share accounting** and **ERC-4626-style vault claims** (`:170`; ERC-4626 as interface bond `:143-147`) | partial (expression layer only) | `spec/successor/examples/financial-vault-quote.mori:6-10` computes a share allocation with `floor_div` and `shares<Vault, Alice>(…)`, under profile `moriarty-financial-expression-source/1` (`:1`). Type exists at `financial-expression-types-v1.ts:8`. `action-targets.csv:18` DA17 → "source-defined-target; implementation-open" | The lifecycle kernel has no shares at all (`LifecycleState`, `financial-lifecycle.ts:129-139`). No `totalAssets`/`totalSupply`, no four conversion directions, no donation/inflation boundary. The vault-quote example emits a `Notice`, not a claim |
| A14 | **Vault share used as collateral** (architecture D) | absent | `deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md:18` names architecture D — "Separate upstream claim accounting from downstream borrowing and seizure"; `:7` requires "an explicit policy and assumption at each conversion from an asset/share quantity to debt capacity". `crosswalk.csv` VT04 requires separating "debt, collateral, liquidation recovery and residual loss" | All of it is descriptor-level. `wiki/defiformal-taxonomy.md:223` (CLM-0932) is explicit: "Keep A–H and T01–T11 as **descriptors, not new Core constructors** or replacement coverage denominators" |
| A15 | `Fl` **atomic flash liquidity** — borrow and repay within one settlement scope **or revert** (`:218`); the only async-impossible element (`:851-856`) | partial | Structurally representable: `financial-lifecycle.ts:141-145` `LifecycleInput.actions[]` admits several actions in one stage, and `runActions` (`:1885-1906`) rejects the whole stage on the first failure. `action-targets.csv:9` DA08 → `needs-normative-atomicity-fixture` | **There is no discharge obligation.** `runActions:1907-1914` performs no end-of-stage check that obligations are `Settled` or that step transfers are fully allocated. A stage may originate a debt and simply end. The "or revert" half of `Fl` is therefore not enforced |
| A16 | **Credit delegation** (Atlas residue `corpus50/.../lane1:108`; provisional `Cg`, `:384`) | absent | — | `Allowance` (`financial-lifecycle.ts:50-55`) is a budget for spending a party's **own** balance. It is not a right for another party to borrow against your collateral. No delegation of borrowing power exists. **Unassigned**: no DA row covers it |
| A17 | **Isolated vs cross margin collateral**; hazard X8 (`:796`) | absent | — | No margin account, no collateral partition, no shared-collateral concept. See A2 |
| A18 | **Law L1** — a credit element *requires* a truth source + `Ct` + a loss allocator, as a well-formedness condition (`:741`) | absent | — | Nothing in the grammar, the static semantics or the six judgments imposes any requirement of one declaration on another. The nearest thing Moriarty has is `judgments.json`, whose six entries are prose definitions plus field lists — study finding F5, `UNIFIED-PROPOSAL.md:36` |
| A19 | **Law L2** — `Pl → (Sh\|Ix) + exit liquidity` (`:742`) | absent | — | No pool, so no exit-liquidity condition. `LANGUAGE-DESIGN.md:118` names "queue capacity and continuation work are bounded" as designed; law L14's `Wq` analogue is not implemented |
| A20 | **Law L7** — `Cd → Rd \| Ps \| liquidation capacity` (`:747`) | absent | — | No redemption, no peg-stability module, no liquidation capacity |
| A21 | **Law L13** — `Ex → freshness validation` (`:753`) | absent | — | See K8. The Lean pilot enforces `observedAt ≤ now ≤ observedAt + 5` (`Examples.lean:83-85`); Moriarty has no observation to validate |
| A22 | **Hazards X2 / X8 / X18** as checkable predicates (`:790`, `:796`, `:807`) | absent | — | See K12. Note the kernel concedes these are not yet measurable predicates either (`:809-813`) |
| A23 | **Health factor is derived, not state** (`:155-156`) | out-of-scope-by-design, vacuously | — | Moriarty models neither, so it cannot violate this. When `Ct` arrives (A6), the constraint applies: a health factor must be a computed expression, not a `state` field. `financial-agreement-source-v5-grammar.ebnf:46` currently permits any `state identifier : type`, so nothing would prevent the mistake |
| A24 | **`obligor`, `claim_perfection`, `record_authority` discriminators** (`:483`, `:506`, `:508`) | absent | — | See K11 |

### 3.3 Tally

Over the 37 rows in §3.1 and §3.2: **covered 2** (K6, K10), **partial 12** (K1, K2, K3, K4, K5, K7, K9, K11, K13, A5, A11, A13, A15 — 13 by list, see note), **absent 21**, **out-of-scope-by-design 2** (K2's rational clause, which I have also counted as partial because the correspondence obligation is open, and A23).

To avoid double counting: K2 is graded **partial** in the table with an out-of-scope-by-design
component, and A23 is the only pure out-of-scope-by-design row. Counting each row once by its
primary verdict: **covered 2, partial 13, absent 21, out-of-scope-by-design 1**.

No row is graded **demonstrated** except K6 and K10, both at lifecycle scope only, on the retained
`k-lifecycle-execution-2026-09-17` receipt. Every one of the 84 stage-relation leaves reads
`NOT_ENFORCED` — I counted them: `enforcement-map.json` has 84 rows, all with
`status: "NOT_ENFORCED"`, and its `limitation` field says signed-intent authentication "was not found
in a circuit, a bound ledger primitive, or another native boundary under the declared native roots."

---

## 4. Category verdict

**Moriarty's language design covers the *obligation* half of lending and none of the *collateral*
half.** The `LifecycleObligation` type and its four actions — Originate, Accrue, partial Repay,
Settle — are a real, executable, K-and-TypeScript-agreed model of an amortising debt with a
principal/accrued split, three allocation rules, a liability cap and identifier-replay protection,
demonstrated over 104 K lifecycle cases (`k-lifecycle-execution-2026-09-17/RESULT.md:3`) and over one
four-stage Preview settlement of a *different, older* fixed loan (`preview-loan-2026-09-17/recovery-run01/RESULT.md:3-13`).
That is more than the DeFi Kernel's Lean pilot has for debt lifecycle, and the comparison should say
so. But the Atlas's G06 solvency group is empty in Moriarty: there is no collateral, no price
observation, no threshold test, no liquidation, no loss allocation and no backstop. Law L1 — the
Atlas's central lending law — is not merely unsatisfied, it is *unstatable*, because Moriarty has no
construct that makes one declaration require another. Accrual is simple interest on original
principal with terms fixed at origination, so `Ix` (index-based accrual), compounding and variable
rates are all out of reach, and Moriarty's own crosswalk already concedes this: "supplied interest is
not rate-model coverage" (`spec/target-crosswalk.json:1025`). Every lending protocol row in the
crosswalk — DEFI:6 Aave V3 through DEFI:12 Sky, lines 999–1185 — carries
`candidate_first_profile_support: unsupported-row-conformance`; of the 104 rows, I counted exactly 2
at `partial-pilot`, and neither is a lending row.

**Milestone ownership, on the evidence found:**

- **U0** owns the repay preconditions L1–L7 as judgment clauses and the numeric-profile freeze
  (`UNIFIED-PROPOSAL.md:119` S3, `:155-161` N4, `:145-153` N2). This is the only lending-adjacent
  work any current document schedules, and it is a *proposal*, not accepted (`UNIFIED-PROPOSAL.md:4`).
- **U3** owns partial progress, recovery and conditional settlement (`ROADMAP.md`, U3 row), which is
  the natural home for partial liquidation and close-factor shapes, though no document says so.
  `UNIFIED-PROPOSAL.md:206` assigns TP03 (observations/oracle honesty) to block U3, "or U2 only if a
  program claims an observation" — so the **price observation** required by A6/A21 is U3-blocked.
- **U6** owns the DeFi rows. `openspec/changes/consolidated-language-kernel/traceability.md:34` maps
  MC07 → U6, and `spec/target-crosswalk.json:5` states "Every retained ACTUS and DeFi row is
  mandatory for MC07". So A1–A7, A11–A14 and A17 land in U6 by that chain. `ROADMAP.md` U6 row
  independently requires "All retained ACTUS fixtures/fields, DeFi action rows and held-out
  behaviors".
- **Unassigned:** `Ad` auto-deleveraging (A8), `Bs` staked backstop (A10) beyond one phrase of
  crosswalk row text, credit delegation (A16), the flash-loan discharge obligation (A15, DA08 says
  only "needs-normative-atomicity-fixture"), the required-bond law L1 (A18), and hazard predicates
  (A22, K12). None of these has a named owner milestone in any document I read.

---

## 5. Gaps that would change the language design

Ranked by how much new machinery they require. Each of these needs language, kernel, judgment,
numeric-profile or enforcement work — a library or an example cannot supply them.

1. **A collateral / encumbrance sort, and a solvency judgment over it.** The frozen stage relation
   has 84 leaves and not one of them names collateral, encumbrance or a threshold test. Adding `Ct`
   needs: (a) a new state sort for an encumbered position that is distinct from a balance, since a
   balance you may spend is not collateral; (b) a **price observation** as a first-class relation
   field with issuer, domain, time and finality — all five `observations[].*` leaves are currently
   classified `absent` in `source-core-embeddings.json`; (c) a seventh judgment, or an extension of
   `effect`, that evaluates a threshold inequality against that observation. This is the single
   largest gap and it is prerequisite to A7, A8, A10, A14 and A17.
   *Kernel anchor:* `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:245`; `lean/DefiKernel/Examples.lean:78-87`.
   *Moriarty anchor:* `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`
   (no such leaf); `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json`
   (`observations[].kind` → `absent`, "source/5 sets observations to an empty object").

2. **A required-bond / well-formedness law over declarations.** Atlas L1 says a program that creates
   a liability is *ill-formed* unless it also declares a truth source, a solvency test and a loss
   allocator (`:741`), and the Atlas's own framing is that "ill-formedness is where the risk was
   hiding" (`:52-54`). Moriarty has no construct — not in the grammar, not in the static semantics,
   not in the six judgments — that makes one declaration require another. This is a static admission
   rule, not a runtime check and not a library. It also needs a home for **hazards as global
   constraints** (`algebra/REQUIREMENTS.md:104-108`), which is a second, separate mechanism: L1-style
   requirements are dual-Horn and hazards are Horn, and `algebra/REQUIREMENTS.md:53-57` (R2) records
   that their conjunction is a lattice under neither.

3. **Whole-program invariants over a collection of obligations.** `financial-agreement-source-v5-grammar.ebnf:78-135`
   gives expressions with no quantifier and no fold; `postfix` (`:96`) allows indexing but there is no
   iteration form. Consequently `L1: outstanding = principal + accrued` and `L2: all components ≥ 0`
   (`UNIFIED-PROPOSAL.md:83-84`) can be written as `ensures` clauses only against a **literal**
   obligation id — as `loan-lifecycle.mori:89-91` does with the string `"Loan1"`. A lending market
   with many positions cannot state its own solvency invariant. Lifting L1–L7 into judgment clauses
   (S3) fixes the *contract*; it does not give the source language the quantifier.

4. **Index-based accrual and rate mutability.** `applyAccrue` computes interest on `principal`
   (`numeric-profile.json`, primitive `accrual-interest`; `lifecycle-kernel.k:36` reads
   `ejGet(O,"principal")` in every accrual term), and `AccrualTerms` is set once at origination with
   no update action. Supporting `Ix` (`:171`) and variable rates needs a shared index state, an
   accrual basis that can be `outstanding`, and a rate-update transition with its own authority and
   observation. Moriarty's own register already anticipates this and calls the current mechanism
   insufficient: "Model omits IRM/compounded-rate calculation; supplied interest is not rate-model
   coverage" (`spec/target-crosswalk.json:1025`).

5. **Supply changes that some code actually emits.** `effects.supplyChanges[].{account,amount,asset}`
   are frozen stage-relation leaves, and all three are classified `absent` in
   `source-core-embeddings.json` — "No supply-change account exists in source/5 or core/1". Without
   mint and burn there is no `Cd` (A3), no `Sh` (A13) and no per-asset conservation check with
   anything on its right-hand side (K5; the proposed law E1 at `UNIFIED-PROPOSAL.md:90` reads
   `Σ gross = Σ supply changes = 0`, and today the second term is always the empty sum). This needs a
   supply-authority capability distinct from debit authority, as
   `lean/DefiKernel/Core.lean:44-46` has.

6. **A loss-allocation transition and an impaired obligation status.** `lcStatus`
   (`lifecycle-kernel.k:47-48`) maps outstanding 0 → `Settled` and anything else → `Outstanding`.
   There is no written-off, impaired or socialised state, and no transition that moves a shortfall to
   a named claim class. This needs new statuses, a claim-class sort and a named write-down authority —
   `LANGUAGE-DESIGN.md:118` already says so in prose ("Cancellation, default, write-off and loss
   allocation need named authority and effects") and nothing implements it.

7. **An end-of-stage discharge obligation for `Fl`.** `runActions:1907-1914` finishes a stage with
   no check that obligations are settled or that step transfers are fully allocated. `Fl`'s defining
   property is "borrow and repay within one settlement scope **or revert**" (`:218`). Expressing it
   needs a stage-terminal predicate — a new kind of check, since every existing check is per-action.
   `action-targets.csv:9` (DA08) currently asks only for a fixture.

8. **A party sort carrying obligor, attester and jurisdiction.** `algebra/MODEL.md:133-135` states
   this as a theorem: mechanism inventory does not determine credit. Moriarty's `debtor`/`creditor`
   are `Identifier` strings (`financial-lifecycle.ts:63-84`). Without this sort, `Uc` (A4), law L3
   and hazard X11a are permanently out of reach, and the numeric profile's beneficiary rules have no
   party to attach to.

---

## 6. Limits of this review

- **I executed nothing but the guarded CLI `status --json`.** I did not run the Moriarty test
  suites, the K suites, the TypeScript evaluator, the U0 checkers, or any Lean build. Where I call
  something "demonstrated", the demonstration is a retained receipt I read
  (`k-lifecycle-execution-2026-09-17/RESULT.md`, `preview-loan-2026-09-17/recovery-run01/RESULT.md`),
  not a run I performed. Those receipts state their own limits — the K result "do[es] not establish
  mandatory proof-carrying transactions; and do[es] not connect the newer lifecycle to Midnight", and
  the Preview result says "the newer four-step language lifecycle … remain[s] open".
- **Absence claims rest on grep plus targeted reading, not on exhaustive enumeration.** My "absent"
  verdicts for collateral, liquidation, health factor, close factor, encumbrance, flash loan, credit
  delegation, bad debt and utilization come from a case-insensitive grep over
  `experiments/moriarty-language/{spec,src,formal}` and
  `openspec/changes/consolidated-language-kernel`, whose only hits were in
  `spec/target-crosswalk.json` (the coverage register). I did not grep `deliverables/`, `evidence/`
  or `raw/` for these terms, so a historical experiment could exist that I did not find. I assert
  absence **from the current language surface, Core kernel, K semantics and frozen stage relation**,
  which is what the question asks, and not absence from the repository.
- **I did not verify the defiformal Lean proofs.** I read `Examples.lean`, `ContractExamples.lean`
  and `Core.lean` as source. I did not run `lake build` or the axiom audits, so the pilot's borrow
  theorems are, for this review, statements I read rather than checks I confirmed.
- **The corpus50 lending residues are the corpus author's judgements**, captured 2026-08-04
  (`corpus50/lanes/lane1-dex-lending-cdp-lsd.json`, `source` field). I treat them as the kernel's
  declared gaps because that is their stated role, not as independently verified facts about Aave,
  Morpho or Compound.
- **Milestone ownership is inference in three places.** The chain
  crosswalk `mc07_mandatory` → `traceability.md:34` (MC07 → U6) is explicit. The claim that U3 owns
  partial liquidation is **my inference** from `ROADMAP.md`'s U3 row and `UNIFIED-PROPOSAL.md:206`;
  no document says it. "Unassigned" means I found no document assigning it, not that none exists.
- **`UNIFIED-PROPOSAL.md` is a proposal.** Its own line 4 says "specified-only … Nothing in it closes
  a U0 predicate." Everywhere I cite S3, S4, N1, N2 or N4 as a plan, that is a plan, and I have not
  treated any of it as coverage.
- **I did not review the Moriarty↔defiformal adapter from the defiformal side.** `openspec/ROADMAP.md`
  in defiformal schedules P31 "Runtime adapter contracts for Moriarty, Compact, ZKIR, and
  proof-carrying transactions" (status: Open) and P24 "Morpho bad-debt loss allocation" (Planned,
  resource-gated on P17). Neither is Moriarty work and I did not assess either.

---

## Architecture axis (follow-up)

Additional axis requested by the coordinator. §1–§6 above are unchanged. Sources read in full for
this section: `ROADMAP.md` (56 lines) and `docs/MORIARTY-CONSOLIDATED-DESIGN.md` (123 lines), both at
`8f73784`. I executed nothing for this section.

**1. Placement.** Lending is placed as **source libraries over the small core**, split across three
named families. `CONSOLIDATED-DESIGN.md:15`: "Financial applications are source libraries over that
core." `CONSOLIDATED-DESIGN.md:93`: "Build financial breadth as reusable libraries: exact
arithmetic/fees; payments and escrow; **loans and claims**; swaps and liquidity; shares/vaults; ACTUS
cash-flow contracts; **redemption/loss allocation**; **margin**; asynchronous and conditional
claims." `ROADMAP.md:48` repeats the list. It is **not** a core construct, not a Federated DeFi
Kernel service and not a settlement adapter: the Kernel is optional (`CONSOLIDATED-DESIGN.md:11`,
"Moriarty can also run on Midnight without this federation") and the responsibility boundary keeps
"liabilities and residual duties" in the Moriarty column while the Kernel column gets only "Find
liquidity and counterparties; arrange execution" (`:26`). Note the placement is itself a
decomposition — lending is *three* families — and no document states an interface between them.

**2. Core mechanisms needed.** The core is enumerated at `CONSOLIDATED-DESIGN.md:15`: "exact typed
values; bounded evaluation; explicit state and effects; checked authority; assertions and
refinements; authenticated evidence; persistent obligations and continuations; and defined
composition operators."

- *Persistent liabilities* — **named, component designated.** `:15` ("persistent obligations"),
  strengthened at `:54`: "Financial liabilities are persistent until discharged, transferred with
  applicable consent, amended or explicitly forgiven. **Default does not erase debt**, and
  forgiveness is not repayment." The canonical stage statement binds "opening and closing
  liabilities" (`:42`).
- *Consent to being made liable* — **named, no component.** `:54`: "Creating a liability requires
  applicable consent from the party made liable"; `:60`: "Recording a request does not impose a duty
  on an unconsenting recipient." The stage statement binds one "signed intention and consent/
  delegation policy" (`:42`) — the signer's. Nothing binds a second party's consent.
- *Collateral / encumbrance* — **not named anywhere in either document.** The nearest core mechanism
  is `:54` "Consumed receipts are linear resources. Spending permission may be affine", which is the
  right shape (an encumbrance is a linear resource the owner may not spend), but the design never
  makes that connection and designates no component.
- *Solvency predicate over an observed price* — **half-named, unjoined.** Observations exist
  ("authenticated evidence", `:15`; "typed observations with issuer, domain, time and finality",
  `:42`; "Typed condition/evidence policies, freshness … threshold rules", `:30`) and prices exist
  ("Checked finite-width arithmetic, overflow, rounding and prices are explicit", `:52`). No line
  says a program may condition an obligation's admissibility on an observed price. "Assertions and
  refinements" (`:15`) is the nearest core mechanism and no component is designated.
- *Liquidation as an authorized third-party action* — **not named.** The authority taxonomy is
  closed and six-valued: "Initiate, complete, reconcile, recover, disclose and amend rights have
  separate scopes" (`:68`). All six are rights over one's own workflow. There is no seize, enforce or
  forced-close scope, and `:54`/`:60`'s consent rules actively obstruct one.

**3. Architectural holes — requirements stated with nothing on the other side.**

- **H1. "Default does not erase debt" (`:54`) — nothing defines default.** No maturity, no threshold,
  no default event, no trigger appears in either document. A property is asserted about an event the
  architecture never introduces. `ROADMAP.md:40` compounds this by listing "**erased debt**" among
  the hostile controls U0–U3 must test, while no component is designated to prevent it (the U0 study's
  F2 records the concrete instance: the kernel prevents it, the source cannot state it —
  `UNIFIED-PROPOSAL.md:33`).
- **H2. Multi-party consent is required and unbuilt.** `:54` and `:60` impose a consent rule; `:42`
  binds exactly one signer's intention. At liquidation or loss allocation a party's liability changes
  without that party signing the stage. No component reconciles the rule with the statement.
- **H3. "redemption/loss allocation" is a named family (`:93`, `ROADMAP.md:48`) with no loss-bearer
  concept.** `:56` requires netting to preserve "the authorized gross economics, **liability
  ownership** and residual duties" — loss allocation is precisely a transfer of liability ownership
  to a non-signer. The responsibility boundary table (`:23-36`) has twelve rows and none asks who
  bears an unrecoverable shortfall.
- **H4. "margin" is a named family (`:93`) resting on three mechanisms none of which is in the
  core.** Margin needs collateral, a solvency predicate and a forced close; `:15` has none of them and
  the boundary table has no row for any.
- **H5. Pooled lending is the shared-state case and the architecture's only word for it is a
  caveat.** `:72`: "Shared-state interleaving requires interference reasoning." No component supplies
  that reasoning. A pool is many borrowers over one reserve — the exact case named and unstaffed.
- **H6. Oracle latency has no column.** `:30` assigns oracles to the external column as "explicit
  trust assumptions". Atlas L1 additionally requires "a liquidation-latency bound"
  (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:741`, and L1 is marked async-unsafe). No document column owns
  the decision that a price is too stale to act on.
- **H7. Keeper arrival is named as an assumption and left there.** `:66`: "Bounded termination does
  not prove funds can always be recovered: liquidity, **actors**, chain inclusion, finality and
  witness availability are explicit liveness assumptions." Lending solvency depends on a keeper
  arriving. The architecture has no incentive, bonded-capital or backstop concept, so "actors" is a
  named assumption with nothing that could discharge it.

The distinction the coordinator asked for holds: H1–H7 are not "someone knows what to write". Each is
a stated obligation with no addressee.

**4. Canonical stage statement.** `CONSOLIDATED-DESIGN.md:42` binds, among others, "typed
observations with issuer, domain, time and finality; complete gross and net effects including fees
and supply changes; **opening and closing liabilities**; … predecessor and obligation commitments".
For lending it omits:

- **The arrow between observations and liabilities.** Both are bound; nothing binds *which*
  observation discharged *which* obligation's admissibility. This is the sharpest omission, and `:46`
  shows the design knows why it matters — "A host JSON field or host-computed verification Boolean is
  insufficient."
- **Accrual and discharge events.** Only opening/closing snapshots are bound, so the roll-forward
  (`opening + accrual − discharge = closing`, proposed as D5 at `UNIFIED-PROPOSAL.md:143`) is not a
  bound relation and a fresh borrowing is indistinguishable from an accrual.
- **Principal/accrued decomposition** (proposed by S2, `UNIFIED-PROPOSAL.md:113`).
- **Encumbrance/collateral set**; **solvency-test result and its threshold parameter**; **a rate or
  rate-change binding**; **a seizure effect class distinct from `gross`**; **a third-party consent or
  liability-transfer field** (H2/H3).

**5. Ownership of the architecture work.** Three answers, and the gap is between them.

- *Library delivery*: **U6** — `ROADMAP.md:27`, "All retained ACTUS fixtures/fields, DeFi action rows
  and held-out behaviors", confirmed by `openspec/changes/consolidated-language-kernel/traceability.md:34`
  (MC07 → U6). U6's evidence column is conformance, not design.
- *The stage relation and judgments that would have to carry lending*: **U0** — `ROADMAP.md:21`,
  "stage, intent, effect, authority, history and failure judgments; … field-by-field enforcement
  map". If collateral, solvency and seizure fields are not added at U0, U6 inherits a relation that
  cannot express them.
- *The core mechanisms themselves*: **unassigned.** No line in either document assigns the
  architecture of collateral, the solvency predicate, liquidation authority, loss-bearer selection or
  a latency bound to any milestone. `ROADMAP.md:32` is the only sentence that comes close — "U6 owns
  later financial extensions and must requalify affected source/Core, native, consumption and
  mandatory-acceptance predicates" — and it frames U6 as *requalifying* an existing core, i.e. it
  assumes the core does not change. U3 (`ROADMAP.md:24`) is the plausible home for a forced close
  ("persistent duty and continuation, joins, … separate recovery authority") but its discriminator is
  a two-asset escrow (`ROADMAP.md:38`), and no line assigns credit to it.

**6. Minimum addition so lending can be built later without redesigning the core.** Four items, each
an addition to the `:15` core list and the `:42` stage statement; none touches the proof stack, the
composition operators or the ZKIRv3 target.

1. **An encumbrance resource kind.** A linear resource held against a named duty, unspendable by its
   owner and seizable by a named authority. `:54` already has linear and affine resources; this is one
   more kind, not a new theory.
2. **A seventh authority scope at `:68`: enforcement.** A right exercisable by a non-owner against a
   defaulting counterparty, bounded by a policy the debtor consented to at origination. Without it,
   `:54`'s consent rule and any liquidation are mutually contradictory, and H2 cannot close.
3. **Observation-conditioned admissibility in the stage statement.** A bound link from a named
   observation to the predicate it discharged, plus that predicate's threshold as a bound parameter.
   This is what turns a solvency test into an enforced constraint rather than the host Boolean `:46`
   rejects, and it is the arrow §4 found missing.
4. **Liability event leaves with a declared loss bearer.** Accrual and discharge as bound effect
   classes beside opening/closing liabilities, plus a loss-bearer field for the shortfall case. This
   closes the roll-forward and gives "redemption/loss allocation" (`:93`) somewhere to write.

With those four, "loans and claims", "margin" and "redemption/loss allocation" become library work
over an unchanged core. Without them, at least the first three are core changes deferred into a
milestone (U6) whose own charter assumes the core is already frozen.
