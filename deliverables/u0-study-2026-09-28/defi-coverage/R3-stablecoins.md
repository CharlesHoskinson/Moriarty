# R3 — DeFi coverage review: stablecoins and synthetic assets

**Reviewer:** R3, one of nine independent read-only reviewers.
**Date:** 2026-09-28.
**Status:** repository observation and inference. This review accepts nothing, closes no
predicate, and is not product acceptance. Under `AGENTS.md`, work is accepted by evidence,
not by review.

---

## 1. Scope and pins

### Commits

| Repo | Command run | Output |
|---|---|---|
| DeFi Kernel | `git -C /home/charl/projects/defiformal log -1 --format='%H %ad %s'` | `8c5dd103cd40369a763b02b1504441acce0ce3c2 Thu Sep 10 17:38:12 2026 -0600 Prepare independent Curve source-entry review` |
| Moriarty | `git -C /home/charl/Moriarty rev-parse HEAD` | `8f73784042bd692733c296d0d49f5173be96725e` |
| Moriarty (subject line) | `git -C /home/charl/Moriarty log -1 --format='%H %ad %s'` | `8f73784042bd692733c296d0d49f5173be96725e Wed Sep 23 21:00:14 2026 -0600 U0 T7: add U0 exit gate` |

All defiformal file:line citations below are at `8c5dd10`. All Moriarty file:line citations
are at `8f73784`.

### Startup performed

The host did not expose the `moriarty-dev:develop` skill. Per `AGENTS.md:8-11` I read and
applied the checked-in skill at `plugins/moriarty-dev/skills/develop/SKILL.md`, and read
`AGENTS.md` and `docs/FOOTGUNS.md` before forming conclusions.

I ran the guarded CLI status, read-only:

```
python3 plugins/moriarty-dev/scripts/moriarty_dev/cli.py --repo . status --json
```

Exit code 0. Actual output (abridged to the fields that matter here): `capability` =
`SP01.6 loan-swap-subset`; `blockedAction` = `implementation/repair of loan-swap-subset`;
`nextAction` = `sp01-loan-report`; `missingEvidence` includes
`binding-input-stale:openspec/sprints/sp01-financial-contract-and-execution-admission.md`,
`current-accounting-missing:.moriarty-dev/runtime/current-accounting.json`,
`resource-live-state-unavailable:sp01-loan-swap-grok-01`, `operational-history`; seven
pending transaction IDs were listed. I did not call `next`, `run`, `notify`, `deliver` or
`report`; I dispatched no campaign, edited no repository source, committed nothing, and
submitted nothing to any network. I did not post the pending transaction lines, because I
made no network status claim and this review is not a status report.

### What I read

**defiformal (abstraction level).** `README.md` (whole file); `algebra/MODEL.md` (whole
file); `algebra/REQUIREMENTS.md` (§R4, §R9, §R10, "Deliberately NOT required");
`algebra/THEOREMS.md:150-160`; `algebra/THEOREM-LEDGER.md:30-33`;
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md` (§3-§4 element registers, §8 discriminators, §9 bonds,
§10 laws L1-L29, §11 hazards X1-X19, §12.4 residue, §16.1 reflexivity);
`docs/unified-v0.1.md` (worked formulas);
`corpus50/lanes/lane3-rwa-options-stables-prediction.json:211-311`;
`expansion/03-cdp-stablecoins/01-research.md`; `expansion/03-cdp-stablecoins/SECTION-BRIEF.md`;
`expansion/11-fiat-stablecoins/SECTION-BRIEF.md`;
`research/positive-program/POSITIVE-PROGRAM-BRIEF.md`;
`research/positive-program/basis/REFUTATION.md`; `openspec/ROADMAP.md`; `wiki-llm/`.

**defiformal (Lean, read for the *model*, not the proof engineering).**
`lean/README.md`; `lean/DefiKernel/Typed/Transition.lean`; `lean/DefiKernel/Typed/Authority.lean`;
`lean/DefiKernel/Typed/Types.lean`; `lean/DefiKernel/Typed/Examples.lean`;
`lean/DefiKernel/Composition/Preservation.lean`; `lean/DefiKernel/Atomic/Policy.lean`;
`lean/DefiKernel/Atomic/Execution.lean`; `lean/DefiKernel/Core.lean`;
`lean/DefiKernel/Examples.lean`.

**Moriarty.** `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md` (whole file) plus the
nine per-lens reviews grepped for supply/mint terms; `docs/MORIARTY-CONSOLIDATED-DESIGN.md`;
`docs/MORIARTY-PRODUCT-CONTRACT.md`; `ROADMAP.md`;
`docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md`;
`deliverables/u0-semantic-contract-2026-09-23/` (`judgments.json`, `enforcement-map.json`,
`source-core-embeddings.json`, `trust-premises.json`, `numeric-profile.json`,
`k-reconciliation.json`, `EXIT-GATE.md`);
`openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json`;
`openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md`;
`openspec/changes/consolidated-language-kernel/traceability.md`;
`experiments/moriarty-language/spec/target-crosswalk.json`;
`experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf`;
`experiments/moriarty-language/spec/successor/financial-expression-source.md`;
`experiments/moriarty-language/src/successor/financial-lifecycle.ts` (type declarations);
`experiments/moriarty-language/src/successor/financial-expression-types-v1.ts`;
`experiments/moriarty-language/formal/k/` (file inventory and a targeted grep);
`deliverables/defi-language-design-2026-09-07/`;
`deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md`;
`deliverables/modern-defi-taxonomy-2026-09-08/`; `deliverables/defiformal-study-2026-09-19/`;
`wiki/security.md`.

### What I executed and did not execute

Executed: `git log`/`rev-parse` on both repos; the guarded `status --json` above; `grep`,
`sed`, `awk`, `ls`, `wc` reads; three `python3 -c` one-liners that only parse and print JSON
from `spec/target-crosswalk.json`, `trust-premises.json` and
`corpus50/lanes/lane3-rwa-options-stables-prediction.json`.

**Not executed:** I did not build or run Lean (`lake build`, `lake env lean`). I did not run
any Moriarty test suite, any U0 checker (`check_u0_exit_gate.py`,
`check_u0_numeric_profile.py`, `check_u0_target_pins.py`), the TypeScript evaluator, the K
semantics, the CLI `simulate`, any demo, or any prover. Every statement below about what code
*does* is read from source text, not from an observed run. No claim in this report is
supported by an execution I performed, other than the `status --json` output quoted above.

---

## 2. What the kernel abstraction requires for this category

The DeFi Kernel says two different kinds of thing about stablecoins and synthetics. The Lean
kernel supplies an **execution model** with a real supply abstraction. The Atlas/algebra layer
supplies a **requirement inventory** — including an explicit list of what its own vocabulary
cannot express. Both are "what the kernel says this category needs".

### 2.1 The Lean kernel's supply abstractions (present and load-bearing)

**S-1. A signed supply delta declared per (domain, asset) on the operation template.**
`lean/DefiKernel/Typed/Transition.lean:19-28` defines `Template` with the field
`supplyDeltas : List (SupplyDelta Party Asset Domain signature)`; `SupplyDelta` at
`:14-17` carries `domain`, `asset` and a signed amount expression. A mint and a burn are one
signed quantity, not two constructors. `README.md:39-40` states it in prose: an operation is a
registered template declaring "its arguments, execution conditions, balance changes, and
minting or burning of asset units."

**S-2. A capability that authorizes changing an asset's supply.**
`lean/DefiKernel/Typed/Authority.lean:8-12` defines
`inductive Right | invoke | debit (cell) | changeSupply (domain) (asset)`. Supply authority is
a *distinct right* from debit authority and from invoke authority. Grants are issued only by
the domain administrator (`issueCapability`, `:65`) and revoked by tombstone with the ID
retained (`revokeCapability`, `:77`). `README.md:35-38`: "Capabilities grant permission to
invoke an operation, debit a particular balance, or change an asset's supply. They can be
issued and revoked."

**S-3. Execution checks the supply capability and refuses by name.**
`lean/DefiKernel/Typed/Transition.lean:139-143` (`Evaluated.suppliesOK`) requires that for
every `(d,a)` with a nonzero net supply change, a live capability for
`.changeSupply d a` is presented. The refusal enum at `:44-60` contains `unauthorizedSupply`
and `accounting` as distinct constructors.

**S-4. Per-asset, per-domain conservation tying balance change to declared supply change.**
`lean/DefiKernel/Typed/Transition.lean:145-146`:
`Evaluated.accountingOK e := decide (∀ d a, ∑ p, e.effect (d, p, a) = e.supply d a)`.
Proved across a whole executed trace at
`lean/DefiKernel/Composition/Preservation.lean:155-159` (`run_accounting`), which states
`total (run …).world.state d a = total initial.state d a + traceSupply (run …).events d a`.
`README.md:41-43`: "For each asset in each domain, the total balance change must equal
declared minting minus burning."

**S-5. Atomic settlement forbids net supply change on a settlement lane.**
`lean/DefiKernel/Atomic/Policy.lean:89-91` (`checkSupply`) returns the first policy lane whose
receipt supply is nonzero; `lean/DefiKernel/Atomic/Execution.lean:116-121` turns that into a
`.laneSupply` abort. Batched netting is transfer-only; issuance cannot hide inside a batch.
`README.md:60-64` describes the settlement policy and its supply checks.

**S-6. A typed external observation feeding the collateralization guard.**
The kernel's CDP reference is `lean/DefiKernel/Examples.lean:66-86`: an `Oracle` record with
`feed`, `price`, `observedAt`, `now`; a `borrow` transition whose
`supplyChange := fun a ↦ if a = .debt then q.amount else 0` and whose guard requires
`2 * (debt + q.amount) ≤ collateral * oracle.price` plus a freshness window. `README.md:162-165`
records that the oracle is a trusted model input: "checking its type, sign, and age does not
establish its truth."

**S-7. A domain/account/asset ledger with asset-specific amount types and exact rationals.**
`README.md:23-27`. `lean/DefiKernel/Typed/Types.lean:26`:
`Cell := Domain × Party × Asset`.

**S-8. Collateral and debt are distinct assets in the same ledger.**
`lean/DefiKernel/Typed/Examples.lean:12-13` and `lean/DefiKernel/Core.lean:18-19`:
`inductive Asset | usd | share | collateral | debt`. `lean/README.md:56-57` qualifies it:
"Debt is represented as a distinct nonnegative obligation token in the reference example. This
is not a general party/claim lifecycle model."

### 2.2 The Atlas requirement inventory for this category

**A-1. Collateralized-debt minting is a named element.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:234` — `E015 | Cd | Collateralized-debt minting | R | Mint
a liability against locked collateral`.

**A-2. `Cd` carries a mandatory obligor discriminator where backing is off-chain.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:238-239` and the discriminator register at `:506`:
`Uc, Ft, Cd(off-chain backing) | obligor | none · protocol-treasury ·
named-operating-entity · bankruptcy-remote-SPV · regulated-custodian · trust`.

**A-3. `Cd` must be bonded to a redemption right, a peg-swap, or liquidation capacity.**
Law L7, `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:747`: `Cd → Rd | Ps | liquidation capacity`.
Law L1, `:741`: `(Pl|Im|Cd|Pf|Op) → (Ex|Tp|At) + Ct + (Li|Ad|Sl|Bs)` — a leveraged obligation
needs a price source, a solvency test, and a loss-absorption mechanism.

**A-4. Solvency test, liquidation, socialized loss and backstop are separate elements.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:245-249` — `Ct` collateral-threshold solvency test, `Li`
incentivized liquidation, `Ad` auto-deleveraging, `Sl` socialized-loss allocation, `Bs`
staked backstop.

**A-5. Peg defence is three distinct elements.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:322-324` — `Rd` direct redemption right ("Redeem a
liability against backing at a defined rate"), `Ps` peg-swap module ("1:1 reserve-backed swap
with **mint/burn authority**"), `As` algorithmic supply adjustment ("Supply
expansion/contraction driven by measured price").

**A-6. Reserve attestation is a typed element with mandatory subject and assurance, and its
own bonding law.** `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:274` (`At` "A named party's statement
about backing or value"), `:276-278` (mandatory `subject={reserve,nav,borrower-financials}`,
`assurance={audit,review,agreed-upon-procedures,management-attestation}`), and law L27 at
`:767`: `At →` named attester + independence from the obligor + stated scope and assurance +
periodicity and staleness bound + recourse against the attester.

**A-7. Rebasing is a distinct accounting element, and mixing it with a balance-invariant
ledger is a fatal hazard.** `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:172` — `E003 | Rb | Rebasing
accounting | S0 | R | A | Nominal balances change through global scaling`. Hazard X4 at `:792`:
"`Rb` into a balance-invariant ledger without an adapter | F | — | interface contradiction".

**A-8. Cross-domain issuance carries a mandatory `form` isotope and its own conservation law.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:504` (`Xf | form | X12 fires on lock-mint and not on
burn-mint`) and law L9 at `:749`: `Xf → debit(source) = credit(destination)`.

**A-9. A party sort carrying obligor, attester and jurisdiction is the *minimal* enrichment
that separates two reserve-backed stablecoins.** `algebra/MODEL.md:128-135`: "No observation
over on-chain mechanism sets separates USDT from USD1. The minimal enrichment that does is a
party sort carrying obligor, attester and jurisdiction." Recorded as theorems P6/P7/P8 at
`algebra/THEOREM-LEDGER.md:31-33`, and as hard requirement R4 at `algebra/REQUIREMENTS.md:69-75`.

**A-10. Asset-indexed backing is required to express reflexive collateral.**
`algebra/MODEL.md:119-125`: the requirement relation over all 58 elements is a DAG with no
self-loops, "Over `El × Ast` under `backs := reads ; over⁻¹`, Terra is a 2-cycle — and crvUSD,
with a comparable element set, has none." `algebra/THEOREMS.md:153-158` (obligation C6, marked
*Mandatory*): "the killing loop is not expressible over element types; it becomes a 2-cycle
over `(element, asset)` pairs under `backs`." Hazard X1, `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:789`.

**A-11. Loss absorption and surplus allocation need a named residual claimant.**
Law L29, `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:769`. Hazards X3 (`:791`) and X17 (`:806`) both
turn on reflexive protocol-token backstops.

### 2.3 What the kernel itself records as missing for this category

These are requirements the kernel *states* by naming its own residue. They matter because a
Moriarty "gap" that matches one of them is a gap the reference model also has.

- **Debt ceiling / supply cap has no symbol.** `expansion/03-cdp-stablecoins/specs/sky.json`
  (F2 note): "`Ct` names the ratio test… It does not name `dust`, `line` or `Line`. Those
  ceilings are the instrument Sky actually uses to size a market… A quantity envelope on a
  permitted action recurs four times in this lane and has no symbol." The underlying protocol
  fact is at `expansion/03-cdp-stablecoins/01-research.md:38-41` (per-ilk `line`, global
  `Line`, per-vault floor `dust`, `spot`).
- **Emergency shutdown / global settlement has no element.** The protocol fact is recorded at
  `expansion/03-cdp-stablecoins/01-research.md:53-54` (`MCD_END`, "a global settlement that
  fixes prices and lets every holder claim collateral directly") and `:1008-1010` (Liquity V2
  per-branch shutdown that *improves* redemption terms). No element or law in
  `docs/UNIFIED-DEFI-ELEMENT-TABLE.md` names it; the nearest is L15 at `:755`,
  `Up → Tg | bounded emergency process`.
- **No price-of-credit element.** `research/positive-program/POSITIVE-PROGRAM-BRIEF.md:225-226`:
  "No element anywhere names an interest rate… Reached independently by the lending lane and
  the CDP-stablecoin lane." The four simultaneous Sky peg defences — stability fee, savings
  rate, PSM par arbitrage, liquidation — are at
  `expansion/03-cdp-stablecoins/01-research.md:43-47`.
- **No party sort in practice.** `research/positive-program/POSITIVE-PROGRAM-BRIEF.md:207-211`:
  "135 of 385 classified residue obligations (35.1%) turn on naming *who* holds an authority…
  USDT vs USD1 collide exactly, and the collision survives canonical forms."
- **The reserve itself is unnameable; only the attestation report is.**
  `expansion/11-fiat-stablecoins/SECTION-BRIEF.md`: "There is exactly one symbol in all 58 that
  touches off-chain backing — `At` — and it names the attestation REPORT rather than the
  reserve." The per-protocol residue is at
  `corpus50/lanes/lane3-rwa-options-stables-prediction.json:220-221` ("THE RESERVE PORTFOLIO…
  there is no symbol for it"; "The ISSUER AS OBLIGOR… No element for the obligor and none for
  creditor priority against it") and `:225` (the chain-swap burn-on-one/issue-on-another
  operation). `:230` records that even `Ps` is a *forced* symbol for Tether: "there is no
  peg-swap MODULE."
- **Synthetic shared-debt-pool designs are outside the register entirely.**
  `expansion/03-cdp-stablecoins/SECTION-BRIEF.md`: the synthetic-dollar family "needs at minimum
  three new elements — off-exchange custody/mirroring, off-protocol revenue capture, and a
  maintained-neutrality invariant — none of which exist even as candidates."
- **The Lean claims package defers collateral, interest and liquidation.**
  `openspec/changes/claims-liability-lifecycle/proposal.md:9`: "Conditional/indexed payoffs,
  interest, collateral liquidation, debt-token correspondence, async settlement, third-party
  repayment, cryptographic authorization and deployed/legal fidelity remain separate work."
- **Off-chain reality is out of scope by declaration.** `algebra/REQUIREMENTS.md:126-127`:
  "Obligor, custody, register of record and legal recourse are out of scope by declaration,
  not by oversight." This sits in tension with A-9 above (`algebra/MODEL.md:128-135` calls the
  party sort the minimal separating enrichment); I treat A-9 as the controlling statement for
  what the category *needs*, and `REQUIREMENTS.md:126-127` as a statement about what the
  algebra work chose to build.
- **No liquidation, debt ceiling, supply cap, emergency shutdown or rebasing exists in the
  Lean tree.** My subagent's repo-wide greps under `lean/DefiKernel/` for `liquidat*`,
  `debtCeiling`, `supplyCap`, `shutdown`, `pause` and `rebas` returned zero hits; I did not
  re-run those greps myself (see §6).

---

## 3. Coverage verdict per requirement

Verdict vocabulary, applied strictly:

- `covered` — the abstraction exists in Moriarty at a level that makes the category expressible,
  and the report says at which of the four stages (designed / specified / implemented in the
  evaluator / demonstrated by executed evidence).
- `partial` — some part exists; the rest is named or absent.
- `absent` — no Moriarty construct exists.
- `out-of-scope-by-design` — a Moriarty document states this is deliberately excluded.

A name in a schema is not coverage. A checker exit of 0 is not a capability.

| # | Requirement (from §2) | Verdict | Where in Moriarty (file:line) | What is missing |
|---|---|---|---|---|
| R1 | **S-1** A declared supply change per asset, on the effect record | `partial` (specified only, as a schema field) | `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json:352-375` defines `effects.supplyChanges[]` as `{asset, account, amount}` with `amount` a signed decimal string; it is `required` at `:277`. Named in the effect judgment at `deliverables/u0-semantic-contract-2026-09-23/judgments.json:52,63-65`. Design-level requirement at `docs/MORIARTY-PRODUCT-CONTRACT.md:43` ("authorized mint/burn supply changes") and `docs/MORIARTY-CONSOLIDATED-DESIGN.md:42`. | The field has no producer. `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json:311-345` records all three leaves as `"realisation": "absent", "present": false` with the note "No supply-change account/amount/asset exists in source/5 or core/1." The Core lifecycle state (`experiments/moriarty-language/src/successor/financial-lifecycle.ts:129-139`) has `balances`, `allowances`, `obligations`, used-ID sets and `work` — no supply. Its four actions (`:127`) are `Transfer | Repay | Originate | Accrue`. Shape differs from the kernel's: Moriarty's supply line is per-*account*, the kernel's `SupplyDelta` is per-(domain,asset) with no account (`Transition.lean:14-17`). |
| R2 | **S-2** A distinct capability/right authorizing a supply change | `absent` | Nothing. `signedIntent` (`stage-relation.schema.json:116-179`) carries `signer`, `consentPolicy`, `delegationPolicy`, `assetIdentities[]`, `recipients[]`, three integer caps, `validity`, `replayPolicy`, `recoveryPolicy` — no supply right. `authority` is `consumed`/`remaining`/`replayState` only (`judgments.json:89-95`). | There is no analogue of `Right.changeSupply(domain, asset)`. Supply authority is not separable from spend authority, so "who may mint" cannot be signed, delegated or revoked. `deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md:36` states the intent — "Arbitrary minting remains outside the initial Core. Issuance/burn can require a separately admitted native capability" — as a candidate, not a construct. `deliverables/defiformal-study-2026-09-19/composition-review.md:15` describes the kernel's `changeSupply` right as *reference material*, explicitly not adopted (`RESULT.md:5`: "Their theorems do not automatically transfer to Moriarty or Midnight circuits"). |
| R3 | **S-3** A refusal that names unauthorized supply change | `absent` | No such rejection code exists in `financial-lifecycle.ts` or the `/5` contract. | Nothing to enforce. |
| R4 | **S-4** Per-asset conservation relating balance change to declared supply change | `partial` (prose in one place, an arithmetically narrower law in another; not executable) | Design requirement: `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:47-48` (UNI-005, "acceptance SHALL account separately for assets and authorized supply…"); `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:41`; `deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md:42` (invariant 1, "Typed conservation"). Proposed law: `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:90` — "E1 \| per-asset conservation: Σ gross = Σ supply changes = 0". | (a) `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:36` (finding F5, 6 of 9 reviewers) records that the six judgments are prose and "None states conservation". (b) E1 as written at `:90` forces supply changes to **zero**; the kernel's law is `Σ balance change = Σ supply change`, which is nonzero exactly when something is minted. The single-reviewer source is narrower and correct in context: `deliverables/u0-study-2026-09-28/opus55-L1.md:155` writes "per asset, Σ gross = Σ supplyChanges (0 in MSS)". The unified restatement generalises badly. (c) No evaluator or K rule checks any conservation predicate; `experiments/moriarty-language/formal/k/` contains no occurrence of supply, mint or burn (grep over `*.k`). |
| R5 | **S-5** Settlement/netting must forbid hidden issuance | `absent` | The nearest statement is `docs/MORIARTY-CONSOLIDATED-DESIGN.md:56` ("Netting requires a relation preserving the authorized gross economics, liability ownership and residual duties") and `spec.md:54-56` (UNI-005 hostile witness: "Netting hides a fee or an omitted liability/reservation"). | Neither mentions supply. There is no analogue of `checkSupply`/`.laneSupply` (`Atomic/Policy.lean:89-91`, `Atomic/Execution.lean:116-121`). `openspec/changes/consolidated-language-kernel/traceability.md:81` assigns "Netting preserves gross economics" (MPLR-025) to **U6**. |
| R6 | **S-6** Typed price/attestation observation feeding a collateral guard | `partial` (record shape specified; no guard, no evaluator) | `stage-relation.schema.json:239-251` requires `observations[]` items to carry `{kind, issuer, domain, time, finality}`. `wiki/security.md:33` states the control shape: "Signed typed observation: source, feed, unit, timestamp, freshness, sequence, bounds, fallback." Trust premise TP03 (`deliverables/u0-semantic-contract-2026-09-23/trust-premises.json:104-156`), status `open`: "Issuers, oracles and signers remain explicit trust assumptions for observations, finality and oracle honesty." | `kind` and `issuer` are free strings; no observation *kinds* are enumerated, so `price` and `reserve-attestation` are not distinguishable types. The `/5` grammar has no price or observation read at all (see R11). `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:74-75` excludes observations and price from slice S0 entirely. |
| R7 | **S-7 / S-8** Multi-asset ledger with distinct asset identities and exact units | `partial` (implemented in the evaluator, but with a narrower asset model) | `experiments/moriarty-language/src/successor/financial-lifecycle.ts:44-48` (`Balance {party, asset, amount}`) and `:63-83` (`LifecycleObligation` with distinct `denomination` and `settlementAsset` plus a `conversion`). `financial-expression-types-v1.ts:49` declares `assets` as a list of bare identifiers; `Amount` is indexed by one asset identifier (`:27-28`). Numeric profile `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json:5-17,302-304` pins price orientation. | Assets are bare schema identifiers with no domain qualification and no issuer. `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:8` records the limitation in the repo's own words: "The current same-name source-unit/settlement-asset restriction is narrower than the target's domain-qualified financial model." The obligation's `denomination`/`settlementAsset` split (`:67-68`) is the closest existing mechanism to "debt denominated in one unit, settled in another" — genuinely relevant to CDP issuance — but it applies to a *liability*, not to a minted asset. |
| R8 | **A-1** Collateralized-debt minting as an expressible operation | `absent` in the language; `designed` only as a target row | Target row `DEFI:12` Sky (ex-MakerDAO) at `experiments/moriarty-language/spec/target-crosswalk.json:1185`, `DEFI:14` USDD `:1247`, `DEFI:15` Lista CDP `:1278`, `DEFI:16` Liquity `:1309`, `DEFI:17` crvUSD `:1340` — all `inherited_subtype: collateralized_debt_issuer`, all `"candidate_first_profile_support": "unsupported-row-conformance"`, all `"mc07_mandatory": true`. Action target `DA10` at `deliverables/defi-language-design-2026-09-07/action-targets.csv:11`: "issue / burn debt-backed stablecoin … issuance authority plus debt and collateral … **needs-extension**". One-sentence principle at `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:94`: "Mint/burn requires a defining authority and accounting rule." | No syntax, no typing rule, no judgment. The `/5` grammar (`experiments/moriarty-language/spec/successor/financial-agreement-source-v5-grammar.ebnf:124-131`) admits exactly six financial reads — `outstanding`, `principal`, `accrued`, `balance`, `allowance_remaining`, `allowance_spent` — and six post-reads. There is no `total_supply` read, no `collateral` read, no price read, and no mint or burn statement form. `experiments/moriarty-language/spec/successor/financial-expression-source.md:173-175` says it directly: "constructing amounts or shares and emitting a descriptor confers no funding, minting, availability, debit, signing or transfer authority." |
| R9 | **A-2 / A-9** Asset identity carrying an issuer/obligor (the USDT-vs-USD1 problem) | `partial` (a proposed record, never an accepted type) | Proposed semantic record at `deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md:27`: "Asset descriptor \| Domain/network, **issuer or policy**, asset identifier, denomination and version. **Identical tickers are not identity.**" Adjacent row `:28` proposes a Position/claim record with "Holder and **obligor**". `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:44`: "A unit name is not itself an asset identifier." Design-level requirement at `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:41`: "Assets require domain-qualified identity". | `DESIGN-IMPLICATIONS.md:34` marks these as "proposed semantic records, not new accepted EBNF keywords". In the frozen contract, `signedIntent.assetIdentities[]` is `array of string` (`stage-relation.schema.json:146-151`) and every `effects.*[].asset` is `string` (`:291,315,339,363`). No issuer, obligor, attester or jurisdiction field exists anywhere in the stage relation, the Core, or the `/5` schema. Under the kernel's own theorem (`algebra/MODEL.md:128-135`), USDT and USD1 are indistinguishable in Moriarty's current asset model. `DESIGN-IMPLICATIONS.md:49` already disclaims the semantics that would be needed: a signature "does not establish the truth of an issuer's external facts or legal enforceability." |
| R10 | **A-3** Law L7: a collateralized-debt mint must be bonded to redemption, peg-swap or liquidation capacity | `absent` | Nothing. | There is no bonding/co-requirement mechanism of any kind in the Moriarty language design — no construct that says "operation X is only well-formed in the presence of operation Y". The nearest adjacent idea is the six judgments (`judgments.json`), which are per-stage predicates, not per-program structural requirements. |
| R11 | **A-4** Collateral-ratio solvency test, liquidation, loss allocation | `absent` in the language; `designed` as a target row and one numeric rule | Target rows: the `collateralized_credit` and `collateralized_debt_issuer` rows above, whose `semantic_requirements` read "Accrual; health; liquidation thresholds/order; bad debt and loss bearer" (e.g. `spec/target-crosswalk.json:1186-1215` for DEFI:12). Action target `DA07` at `deliverables/defi-language-design-2026-09-07/action-targets.csv:8`: "liquidate / recognize default … authorized seizure; loss and residual debt allocation … **needs-pinned-protocol-fixture**". Worked (unexecuted) example AT02 at `deliverables/security-token-transformations-2026-09-09/DESIGN-IMPLICATIONS.md:81`. One numeric rule anticipates collateral: `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json:231` — D2 requires ceil "when the result is an amount owed, a fee, or a **collateral charge**". | No collateral read, no ratio comparison, no health predicate, no liquidation transition, no loss-bearer assignment exists in the grammar, Core, evaluator or K. The one collateral-aware artifact is a rounding direction for an amount that no construct can currently compute. |
| R12 | **A-5** Peg maintenance: stability fee, savings rate, redemption at par, arbitrage window | `absent` | "Peg" is defined once as vocabulary at `deliverables/modern-defi-taxonomy-2026-09-08/VOCABULARY.md:32`: "A target price relationship supported by mechanisms and incentives; **not a guaranteed accounting equality**." "Stability fee" appears only as a Maker case descriptor at `deliverables/modern-defi-taxonomy-2026-09-08/CATEGORIES.md:237`. | No savings-rate concept appears anywhere. No redemption-at-par or arbitrage-window construct exists. The one general accrual mechanism, `AccrualTerms` (`financial-lifecycle.ts:36-42`: numerator/denominator/rounding/periodSeconds/firstPeriodStart) is a per-obligation interest accrual — it could express a stability fee on a *single* CDP obligation, but there is no supply-side or system-wide rate, and no mechanism paying a rate to holders. The kernel records the same gap on its own side (`research/positive-program/POSITIVE-PROGRAM-BRIEF.md:225-226`, "no price-of-credit element"), so this is a mutual gap, not only a Moriarty one. |
| R13 | Depeg / reserve shortfall handling | `absent` | "Depeg" appears once, as a risk-taxonomy label (`deliverables/modern-defi-taxonomy-2026-09-08/VALIDATION.md:175`, reported by subagent; see §6). | No detection, no response transition, no shortfall accounting. |
| R14 | **A-7** Rebasing / elastic supply | `absent` (named and explicitly distinguished; never specified) | `deliverables/modern-defi-taxonomy-2026-09-08/VOCABULARY.md:35`: "Rebasing \| Updating account quantities through balance/index rules; distinct from a fixed share balance with changing conversion rate." `deliverables/modern-defi-taxonomy-2026-09-08/SEMANTIC-BOUNDARIES.md:34` warns that "a naive `balanceOf(vault)/totalSupply` model is not universally correct"; `:99` lists "rebasing balances" among counterexamples to attempt and labels them "**specified-only test targets**, not new executed Moriarty tests." `deliverables/defi-language-design-2026-09-07/README.md:43`: "Rebase accounting differs from share-rate accounting." | Moriarty's Core has no global-factor balance update. Every balance change goes through a `TransferAction` with explicit `from`/`to` (`financial-lifecycle.ts:85-92`), and the `/5` source's only state write is `next.<field> = <expr>` (grammar `:61`). Under the kernel's hazard X4 (`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:792`), Moriarty is currently a balance-invariant ledger with no rebasing adapter — which is the failing side of that hazard, not the safe side. The kernel cannot express it either (`Template` requires per-cell deltas), so this is a mutual gap. |
| R15 | **A-6** Reserve/custody attestation as a typed observation | `partial` (a generic observation record; no attestation type) | `stage-relation.schema.json:239-251`; `wiki/security.md:33`; TP03 open. | No `subject`/`assurance` discriminators (the kernel makes both **mandatory** on `At`, `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:276-278`), no attester identity separate from `issuer`, no staleness bound, no independence-from-obligor condition, no recourse (all of law L27, `:767`). The reserve-backed rows `DEFI:63-67` (`spec/target-crosswalk.json:2766,2797,2828,2859,2890`) name exactly these capabilities — "CustodyAttestation; RegistryEvidence; IdentityCredential; LegalDetermination" — as **required MC07 extensions**, i.e. not present. |
| R16 | Mint/burn *request-confirm* lifecycle and redemption with settlement lag (fiat-backed issuance) | `absent` in the language; `designed` as a target row | `spec/target-crosswalk.json:2766-2796` (DEFI:63 Tether USDT) and the four sibling rows: `package_behaviors` "Issuer/obligor claims; **mint/burn requests**; redemption"; `semantic_requirements` "Eligibility; **request-confirm state**; registry reconciliation; **settlement lag**"; `row_specific_requirement` for USDC (`:2797`) "**Minter allowance**; blacklist; external banking/reserves; float recipient". All `unsupported-row-conformance`, all `mc07_mandatory: true`. | The request-confirm/pending/settled state machine is a stated *design* requirement elsewhere (`docs/MORIARTY-PRODUCT-CONTRACT.md:47`: "External asynchronous effects require explicit pending, partial, unknown, settled and recovered states") but it is not implemented and not tied to supply. A "minter allowance" — a delegated mint *quantity* rather than a role, which is the actual USDC mechanism (`expansion/11-fiat-stablecoins/SECTION-BRIEF.md`) — has no representation: Moriarty's `Allowance` (`financial-lifecycle.ts:50-55`) bounds a party's *spending* of an existing balance, not issuance. |
| R17 | Supply cap / debt ceiling as an enforced bound | `partial`, and only per-obligation | `financial-lifecycle.ts:78-79`: `nominalLiabilityCap` and `liabilityIncurred` on a single obligation. `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:87` law L5: "every counter and every credit fits its unsigned width (checked add, **or a supply bound**)". | The only bound is per-obligation and width-derived. There is no per-asset ceiling, no per-collateral-type ceiling, no global ceiling. This matches the kernel's own residue finding (`expansion/03-cdp-stablecoins/specs/sky.json`: "A quantity envelope on a permitted action recurs four times in this lane and has no symbol"), so neither side has it — but Moriarty's target rows make it mandatory (`spec/target-crosswalk.json:1010-1013`, DEFI:6/Aave `row_specific_requirement` includes "supply/borrow/debt caps"). |
| R18 | Emergency shutdown / global settlement / final redemption at a settlement price | `absent` | Nothing names it. The nearest machinery is the general recovery model at `docs/MORIARTY-CONSOLIDATED-DESIGN.md:68-70` (separate recovery authority, exclusive terminal outcomes, tombstoning) and `docs/MORIARTY-PRODUCT-CONTRACT.md:47`. | The general recovery model is per-workflow, not per-*asset-class*. Global settlement is a system-wide transition that freezes prices and converts every outstanding liability to a pro-rata collateral claim (`expansion/03-cdp-stablecoins/01-research.md:53-54`). Nothing in Moriarty expresses a transition over *all* outstanding obligations of an asset. The kernel has no element for it either. |
| R19 | **A-10** Reflexive/circular backing expressible at all | `absent` | Nothing. | No backing relation between assets exists — `assets` is a flat list of identifiers (`financial-expression-types-v1.ts:49`). Under `algebra/THEOREMS.md:153-158`, a carrier of flat asset identities provably cannot exhibit the Terra 2-cycle. The kernel states this as *mandatory* obligation C6. Moriarty's design documents never name reflexivity, Terra, or circular backing (searched; see §6). |
| R20 | Synthetic asset minting against collateral / shared debt pool | `absent` | `deliverables/modern-defi-taxonomy-2026-09-08/VOCABULARY.md:29` defines "Synthetic" narrowly. `spec/target-crosswalk.json:1216-1246` (DEFI:13 Ethena) is the only synthetic-issuer row, `derivative_hedged_issuer`, `unsupported-row-conformance`. | No shared/pooled debt ledger construct; no "mint X against locked Y" form. The kernel corpus has no Synthetix-style entry at all, and flags the whole synthetic-dollar family as needing three elements that "do not exist even as candidates" (`expansion/03-cdp-stablecoins/SECTION-BRIEF.md`). Mutual gap. |
| R21 | Cross-domain burn-mint issuance with a supply invariant (**A-8**) | `absent` | `spec/target-crosswalk.json:2766` (DEFI:63) names "chain-swap issuance" as a required extension; `DEFI:38-43` are `custodial_wrapped_claim` / `message_verified_bridge` rows, all unsupported. `stage-relation.schema.json` has a single `domain` object (`:13-15` in `judgments.json`), one chain per stage. | No cross-domain supply invariant (kernel law L9, `docs/UNIFIED-DEFI-ELEMENT-TABLE.md:749`), no `form={lock-mint,burn-mint,custodial-release}` distinction (`:504`). A single-domain stage relation cannot state `debit(source) = credit(destination)` across domains at all. |
| R22 | Supply changes admitted in the first slice | `out-of-scope-by-design` for S0 | `deliverables/u0-study-2026-09-28/UNIFIED-PROPOSAL.md:74`: S0 binds "Empty supply changes, observations and disclosures, each bound as an explicit empty set that **rejects any non-empty value**." Restated as a disposition class at `deliverables/u0-study-2026-09-28/opus55-L1.md:121`: "`empty-with-rejection`: e.g. `supplyChanges=[]`, where any supply change is rejected." | This is a correct and deliberate scoping decision, and I record it as such. It is only a *problem* in combination with R23: the exclusion is explicit, but the re-entry point is not. |
| R23 | A milestone owns supply changes | `partial` — assigned indirectly, never named in the unified proposal | `ROADMAP.md:21` puts the U0 semantic contract's scope at judgments and embeddings. `ROADMAP.md:27` puts "All retained ACTUS fixtures/fields, **DeFi action rows** and held-out behaviors" in **U6**; `openspec/changes/consolidated-language-kernel/traceability.md:49` confirms "DeFi all 72 original rows, DA01–24 … remain U6 scope". UNI-005 ("assets and authorized supply", `spec.md:47-48`) maps through MPLR-017 to **U2/U3** (`traceability.md:73`). | The unified proposal's own T5 tiering says only "Everything else: owners U3, U4 or U6" (`UNIFIED-PROPOSAL.md:193`) — it never names supply changes. The single reviewer who did name them is `deliverables/u0-study-2026-09-28/grok47-L2.md:69`: "Cohort C: everything else, with owners U3 …, U4 …, **U6 (supply changes)**." That proposal was not carried into the unified §5 T5 text. So on the evidence in the unified proposal, supply changes are **unassigned**; on the evidence in ROADMAP/traceability, the DeFi rows that need them land in U6 while the accounting requirement that mentions them (UNI-005) lands in U2/U3. That split is itself the gap. |
| R24 | Enforcement of any supply leaf | `absent` (and honestly recorded as such) | `deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json:139-156`: all three `effects.supplyChanges[].*` leaves are `"status": "NOT_ENFORCED"` with `"mechanisms": []`. All 84 leaves in that file are `NOT_ENFORCED` (counted: `grep -o '"status": "[^"]*"' \| sort \| uniq -c` → `84 "status": "NOT_ENFORCED"`). | Consistent with `UNIFIED-PROPOSAL.md:39` (finding F8). Nothing to add; I note it so that no reader mistakes the schema field for enforcement. |
| R25 | K-semantics coverage of supply | `absent`, and recorded | `deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json:95`: "Missing: separate authorized supply, fees, and residual duties over an authenticated complete state domain"; `:276`: "Missing: separate fees, net effects, **supply changes**, and the observation records of the effect judgment." My own grep over `experiments/moriarty-language/formal/k/*.k` for `supply\|mint\|burn\|collateral` returned no matches. | Both the K reference and the TypeScript Core lack it, so the planned S5 TypeScript/K differential (`UNIFIED-PROPOSAL.md:131`) cannot exercise supply at any scope. |

**Counts.** 25 requirement rows: `covered` **0**; `partial` **8** (R1, R4, R6, R7, R9, R15, R17, R23); `absent` **16** (R2, R3, R5, R8, R10, R11, R12, R13, R14, R16, R18, R19, R20, R21, R24, R25); `out-of-scope-by-design` **1** (R22).

**Stage of maturity, for the eight `partial` rows.** None is *implemented in the evaluator* for
this category and none is *demonstrated by executed evidence*. R7 is the only row with
evaluator-level code, and its coverage is of the multi-asset ledger generally, not of issuance.
R1, R4, R6, R9, R15, R17 and R23 are *designed* or *specified* only.

---

## 4. Category verdict

**Stablecoins and synthetic assets are not covered by the Moriarty language design, at any of
the four stages, and the one abstraction that would anchor the category — supply authority — is
named in the contract but has no producer, no capability, no law and no owner.** Moriarty has a
schema field `effects.supplyChanges[]` (`stage-relation.schema.json:352-375`) and two prose
sentences requiring "authorized mint/burn supply changes"
(`docs/MORIARTY-PRODUCT-CONTRACT.md:43`, `docs/MORIARTY-CONSOLIDATED-DESIGN.md:42`), and that is
the whole of it: the repository's own records say the field is absent from source/5 and core/1
(`source-core-embeddings.json:311-345`), unenforced (`enforcement-map.json:139-156`), and missing
from the K reference (`k-reconciliation.json:95,276`). The `/5` grammar admits six financial
reads and no supply, price or collateral read
(`financial-agreement-source-v5-grammar.ebnf:124-131`), and the Core lifecycle has four actions —
Transfer, Repay, Originate, Accrue — over a state of balances, allowances and obligations
(`financial-lifecycle.ts:127,129-139`). Against the kernel's execution model, Moriarty is missing
the `changeSupply` right (`Authority.lean:8-12`), the executable conservation check
(`Transition.lean:145-146`), the supply refusal (`:44-60`), and the settlement lane supply check
(`Atomic/Policy.lean:89-91`). Against the kernel's requirement inventory, Moriarty is additionally
missing everything downstream of issuance: the collateral solvency test and liquidation
(`element table:245-249`), the three peg-defence elements (`:322-324`), the attestation law L27
(`:767`), and — most consequentially — the party sort and the asset-indexed backing relation that
the kernel proves are the *minimal* enrichments needed to separate two reserve-backed
stablecoins (`algebra/MODEL.md:128-135`) and to express reflexive collateral
(`algebra/THEOREMS.md:153-158`). Excluding supply from S0 is a correct scoping decision
(`UNIFIED-PROPOSAL.md:74`); the defect is that the exclusion has no named re-entry.

**Milestone ownership on the evidence found.**

| Gap | Owning milestone | Evidence |
|---|---|---|
| Supply-change *accounting* as part of complete financial state (UNI-005) | **U2/U3** | `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md:47-48`; `traceability.md:73` (MPLR-017 → U2/U3) |
| The `effects.supplyChanges[]` leaves themselves | **unassigned** in the unified proposal (`UNIFIED-PROPOSAL.md:193` says only "U3, U4 or U6"); **U6** in one unadopted single-reviewer proposal (`grok47-L2.md:69`) |
| All CDP and reserve-backed stablecoin target rows (`DEFI:12-17`, `DEFI:63-67`) | **U6** | `ROADMAP.md:27`; `traceability.md:49` |
| Netting must not hide issuance | **U6** | `traceability.md:81` (MPLR-025 → U6) |
| Domain-qualified asset identity | **U0** to freeze, later to implement | `docs/MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:11,41,61`; `ROADMAP.md:21` |
| Typed observation kinds (price, attestation) and oracle trust | **U3**, or U2 if a program claims an observation | `UNIFIED-PROPOSAL.md:206` (TP03 blocking) |
| Supply capability / mint authority as a signed, delegable right | **unassigned** — no document assigns it to any milestone |
| Collateral ratio, liquidation, loss allocation | **unassigned** as language work; the target rows land in **U6** | `action-targets.csv:8` (`needs-pinned-protocol-fixture`, no milestone); `ROADMAP.md:27` |
| Peg maintenance, depeg, emergency shutdown, rebasing, reflexive backing, shared debt pool | **unassigned** — no Moriarty document names them as work at all |

---

## 5. Gaps that would change the language design

Ranked by how much new language, kernel, judgment, numeric-profile or enforcement machinery
they require. Library or example work is excluded.

**G1. There is no supply authority, and adding it changes the authority model, not a library.**
Moriarty's authority is a scalar-plus-replay-set: `authority.consumed`, `authority.remaining`,
`authority.replayState` (`judgments.json:89-95`), which the unified proposal proposes to widen
to "per-asset vectors plus a replay-ID set" (`UNIFIED-PROPOSAL.md:114`). Spending authority and
issuance authority are different rights over different objects — the kernel separates them at
the constructor level (`Authority.lean:8-12`) and proves the separation binds at execution
(`Transition.lean:348-359`, `execute_supply_authority`). Retrofitting this means a new right in
the signed-intent grammar, a new leaf family in the stage relation, a new clause in the
authority judgment, and a new enforcement locus decision (T5, `UNIFIED-PROPOSAL.md:190-196`).
It cannot be added as a library because the signer must be able to authorize *minting* without
thereby authorizing *spending*, and no current field can express that.

**G2. The conservation law E1 as unified is arithmetically specialised to the no-mint slice, and
will not generalise.** `UNIFIED-PROPOSAL.md:90` states "per-asset conservation: Σ gross =
Σ supply changes = 0". The kernel's law is `∑_p effect(d,p,a) = supply(d,a)`
(`Transition.lean:145-146`), which is *nonzero* precisely when issuance occurs. If E1 is written
into the S3 executable judgments (`UNIFIED-PROPOSAL.md:119`) in the `= 0` form, every later
issuance program falsifies the accepted law, and the fix is a semantic amendment rather than an
extension. The correct general form should be frozen in U0 even though S0 instantiates it at
zero. This is a judgment-language change, and it is cheap **now** and expensive after U0-F
hash-binds the clause set.

**G3. Asset identity has no issuer, and the kernel proves that no amount of mechanism modelling
recovers it.** `signedIntent.assetIdentities[]` and every `effects.*[].asset` are plain strings
(`stage-relation.schema.json:146-151, 291, 315, 339, 363`). `algebra/MODEL.md:128-135` states the
sharp result: no observation over on-chain mechanism separates USDT from USD1, and the minimal
enrichment that does is a party sort carrying obligor, attester and jurisdiction. Moriarty's own
design already asks for "Domain/network, issuer or policy, asset identifier, denomination and
version" (`DESIGN-IMPLICATIONS.md:27`) and for "domain-qualified identity"
(`MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:41`), and already records the current restriction
as narrower than the target (`:8`). This is a **type-system** change — asset identity is a
structured value, not a string — and it propagates into the numeric profile (units are indexed by
asset), the signed intent (an intent over "USDC" must say *whose* USDC), and the enforcement map
(a circuit must constrain the structured identity, not a string hash). Doing it after U0-F means
re-hashing the contract.

**G4. There is no bonding or co-requirement mechanism, so kernel law L7 cannot be stated.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:747`: `Cd → Rd | Ps | liquidation capacity`. Every Moriarty
judgment is a predicate over one stage relation instance (`judgments.json`). A law of the form
"a program that mints a liability is ill-formed unless it also provides a redemption path" is a
*program-level static* obligation, not a stage predicate. Supporting it needs a seventh judgment
class (a program well-formedness judgment) or an explicit decision to push these to library
contracts with a stated argument for why the language need not enforce them. Either way it is a
U0-scope decision about what the six judgments are for, and the unified proposal does not raise
it.

**G5. Observation kinds are unenumerated strings, so a price and a reserve attestation are the
same type.** `stage-relation.schema.json:239-251` requires `{kind, issuer, domain, time,
finality}` with `kind` free. The kernel makes `subject` and `assurance` **mandatory**
discriminators on attestation (`element table:276-278`) and gives it bonding law L27 (`:767`:
named attester, independence from the obligor, scope and assurance, staleness bound, recourse).
A CDP guard needs a *price* observation with a freshness bound (`Examples.lean:83-86`); a
fiat-backed redemption needs a *reserve attestation* whose attester is independent of the
obligor. Without an enumerated, discriminated observation type, neither guard can be typed and
neither can be enforced. This is a schema-plus-judgment change with a numeric-profile
consequence (price scale and orientation are already pinned at
`numeric-profile.json:5-17, 302-304`, but only for a `Price` *value*, not for an observation).

**G6. Supply caps and debt ceilings are a quantity envelope on a permitted action, and Moriarty
has no such construct.** The only bound is per-obligation
(`financial-lifecycle.ts:78-79`, `nominalLiabilityCap`) and the only global-sounding law is a
width bound (`UNIFIED-PROPOSAL.md:87`, L5). The kernel names this as unsymbolised recurring
residue (`expansion/03-cdp-stablecoins/specs/sky.json`) and Sky's actual instrument is three
nested envelopes — `dust`, `line`, `Line` (`expansion/03-cdp-stablecoins/01-research.md:38-41`).
A per-asset, per-collateral-type and global ceiling is a new kind of state-plus-invariant that
the signed intent must be able to reference and the circuit must be able to constrain. It is not
a library, because the cap must bind the *accepting path*, not the program's own arithmetic.

**G7. Rebasing is not expressible, and Moriarty is currently on the failing side of hazard X4.**
`docs/UNIFIED-DEFI-ELEMENT-TABLE.md:792` classes "`Rb` into a balance-invariant ledger without an
adapter" as a fatal interface contradiction. Moriarty's only balance mutation is an explicit
`from`/`to` transfer (`financial-lifecycle.ts:85-92`). Supporting elastic supply needs either a
global index in the ledger state (so a balance is `shares × index`) or an explicit declaration
that rebasing assets are represented as share-rate assets with a stated adapter and a named
conversion/rounding rule. The taxonomy already flags the distinction
(`SEMANTIC-BOUNDARIES.md:34`) and already lists it as a specified-only test target (`:99`). The
choice between "index in the ledger" and "adapter at the boundary" is a language-design decision
that should be recorded, because it determines whether `balance` remains a primitive read.

**G8. Emergency shutdown / global settlement is a transition over *all* outstanding obligations
of an asset, and no Moriarty stage can quantify over them.** Every judgment is over the
obligations named in one stage relation; `liabilities.opening[]`/`closing[]` are explicit lists
(`stage-relation.schema.json:378-400`). Global settlement fixes prices and converts every
outstanding claim at once (`expansion/03-cdp-stablecoins/01-research.md:53-54`), and Liquity's
variant *improves* redemption terms on shutdown (`:1008-1010`). Expressing it needs an
authenticated-completeness predicate over a state domain, which Moriarty already identifies as
an open U4 obligation (`MORIARTY-LANGUAGE-REQUIREMENTS-ALIGNMENT.md:41`: "local arrays do not
establish authenticated absence or completeness of private positions"). I rank it below the
others because it inherits an existing, already-owned gap rather than creating a new one.

**G9. Reflexive backing cannot be expressed, and the kernel says that is a carrier defect, not a
missing check.** `algebra/THEOREMS.md:153-158` (obligation C6, *Mandatory*) and
`algebra/MODEL.md:119-125`. Moriarty's assets are a flat identifier list
(`financial-expression-types-v1.ts:49`), which is exactly the carrier the kernel proves cannot
exhibit the Terra 2-cycle. If Moriarty ever wants to *reject* an endogenously-collateralized
issuance, it needs a `backs` relation over (operation, asset) pairs — which is downstream of G3
(structured asset identity) and probably of G4 (program-level well-formedness). I rank it last
of the language-changing gaps only because nothing in the current roadmap depends on it; I rank
it *in* the list because it is the one gap whose fix is not additive.

**Explicitly not in this list (library or example work):** individual protocol rows
`DEFI:12-17` and `DEFI:63-67`; stability-fee and savings-rate *parameterisation*; auction
mechanics; the `sin`/`heal`/`bump`/`hump` surplus machinery
(`expansion/03-cdp-stablecoins/01-research.md:102-107`). Each of these is expressible once G1-G6
exist.

---

## 6. Limits of this review

1. **I executed no Moriarty or Lean code.** No build, no test, no checker, no evaluator, no K
   run, no proof. Every statement about behaviour is read from source text. The single exception
   is the guarded `status --json` run reported in §1. In particular I did **not** confirm that
   the evaluator rejects anything; `UNIFIED-PROPOSAL.md:284` records that no reviewer in the U0
   study did either.

2. **Three findings rest on subagent greps I did not re-run myself.** (a) The absence of
   `liquidat*`, `debtCeiling`, `supplyCap`, `shutdown`, `pause` and `rebas` anywhere under
   `lean/DefiKernel/`. (b) The absence of "emergency shutdown", "global settlement" and
   "settlement price" anywhere in the Moriarty deliverables I delegated. (c) The absence of
   "reflexiv", "Terra", "Luna" and circular-backing discussion in those same Moriarty
   deliverables. I verified by hand every *positive* citation I use from those searches
   (`action-targets.csv:8,11`; `LANGUAGE-DESIGN.md:44,94`; `DESIGN-IMPLICATIONS.md:27,28,36,42,49`;
   `composition-review.md:11,15`; `RESULT.md:5,11`; `VOCABULARY.md:32,35`; `CATEGORIES.md:237`;
   `SEMANTIC-BOUNDARIES.md:34,99`; `libraries-review.md:11`;
   `defi-language-design-2026-09-07/README.md:43,90`; `wiki/security.md:33`) plus every
   defiformal citation I use from §2.2 (`element table:172,234,238-239,244-250,272-279,318-327,
   504-508,739-770,787-808`; `algebra/REQUIREMENTS.md:69-76,126-127`; `algebra/MODEL.md:119-135`;
   `algebra/THEOREMS.md:150-160`; `expansion/03-cdp-stablecoins/01-research.md:36-58`;
   `corpus50/lanes/lane3-…json:211,215,220-221,225,230`) and every Lean citation
   (`Transition.lean:14-28,139-146`; `Authority.lean:8-12`; `Preservation.lean:153-159`;
   `Atomic/Policy.lean:85-92`; `Examples.lean:66-92`; `lean/README.md:54-57`). The one
   citation I use *without* hand verification is
   `deliverables/modern-defi-taxonomy-2026-09-08/VALIDATION.md:175` (the single "depeg"
   occurrence, R13) and the `expansion/…/SECTION-BRIEF.md` and `sky.json` residue quotes, which
   I quote as reported because those files have no line numbers in the reported form.

3. **I did not read the full 1367-line element table, the full 663-line `unified-v0.1.md`, or
   the whole corpus.** I read the registers, discriminators, bonds, laws, hazards and residue
   sections, plus targeted greps. An element or law relevant to this category could exist in a
   section I did not open.

4. **I did not verify that `experiments/moriarty-language/spec/target-crosswalk.json` is the
   controlling coverage manifest.** It declares `"status": "specified-only"` at `:4` and
   `traceability.md:49` says to "Validate these inherited counts against the frozen coverage
   manifest before implementation" — I did not locate or check that frozen manifest. My
   milestone assignment for the DeFi rows rests on `ROADMAP.md:27` and `traceability.md:49`,
   which I did read.

5. **By inference, not by observation:** (a) that `effects.supplyChanges[]` has no producer
   anywhere in Moriarty — I checked source/5, the Core lifecycle types, the `/5` grammar, the K
   files and the embeddings record, but I did not enumerate every file in
   `experiments/moriarty-language/src/`. (b) That the E1 statement at `UNIFIED-PROPOSAL.md:90`
   will not generalise — this is my reading of the arithmetic, not a quoted correction from any
   document; the unified proposal itself never flags it. (c) That the tension between
   `algebra/REQUIREMENTS.md:126-127` (obligor out of scope by declaration) and
   `algebra/MODEL.md:128-135` (party sort is the minimal separating enrichment) resolves in
   favour of the latter for the purpose of "what the category needs" — that is my judgment about
   two documents that disagree, and `MODEL.md:1-8` marks itself a historical record partly
   overridden by a claim disposition I did not read.

6. **The Moriarty checkout is not a git repository from this tool's perspective** (the
   environment reports `Is a git repository: false` for the working directory), yet
   `git -C /home/charl/Moriarty rev-parse HEAD` returned a commit. I report the commit as
   returned and note the discrepancy rather than resolving it.

7. **I stayed inside my review scope.** I did not assess AMMs, lending, derivatives, bridges,
   staking, RWAs, prediction markets or solver/intent rows except where a stablecoin requirement
   traverses them (R21 cross-domain issuance). Overlap with other reviewers on
   `effects.supplyChanges[]`, asset identity and the conservation judgment is likely and
   deliberate.

---

## Architecture axis (follow-up)

Read in full for this section: `ROADMAP.md` (56 lines) and `docs/MORIARTY-CONSOLIDATED-DESIGN.md`
(123 lines). §1-§6 above are unchanged.

**1. Placement — nothing places stablecoin issuance.** The architecture has exactly two slots for
financial behaviour: the small core (`docs/MORIARTY-CONSOLIDATED-DESIGN.md:15`) and source libraries
over it — "Financial applications are source libraries over that core." The library families are
enumerated twice, and neither list contains an issuance or monetary family:
`docs/MORIARTY-CONSOLIDATED-DESIGN.md:93` ("exact arithmetic/fees; payments and escrow; loans and
claims; swaps and liquidity; shares/vaults; ACTUS cash-flow contracts; redemption/loss allocation;
margin; asynchronous and conditional claims") and `ROADMAP.md:48` ("ACTUS, loans/claims,
payments/escrow, AMMs, vaults, netting, redemption/loss allocation, margin and async/conditional
workflows"). It is not a Federated Kernel service either: the kernel's row for financial meaning is
"Find liquidity and counterparties; arrange execution" (`:26`), and nothing in `:11` or `:38` gives it
issuance. Nor a settlement adapter (`:38`: adapters "establish concrete effects and their domain
limits"). The category's only placement anywhere is as conformance *rows* owned by U6
(`ROADMAP.md:27`) — that is a test denominator, not an architectural home.

**2. Core mechanisms needed, against what the architecture names.**

| Mechanism issuance needs | Named in the architecture? | Component designated to realize it? |
|---|---|---|
| A supply-change effect on the accepted stage | Yes — `:42` "complete gross and net effects including fees and supply changes" | **No.** The Responsibility boundary's Financial meaning row (`:26`) allocates "Gross debits, fees, minimum net outcomes, assets, custody, liabilities and residual duties" — supply changes are absent from the row that assigns ownership. |
| **A right to change supply** | **No.** | **No.** The authority model at `:54` admits exactly two disciplines: "Consumed receipts are linear resources. Spending permission may be affine." The Permission row (`:27`) is "Owner consent, scoped delegation, application policy and replay rules". The rights are enumerated at `:68` — "Initiate, complete, reconcile, recover, disclose and amend rights have separate scopes" — six scopes, none of which is issue or mint. There is no slot the kernel's `changeSupply(domain,asset)` capability could occupy. |
| Per-asset conservation (balance change vs declared supply change) | No law. The only conservation sentences are `:54` "Token conservation alone cannot prove these properties" (a disclaimer about liabilities) and `:56` netting "preserving the authorized gross economics, liability ownership and residual duties". | No. |
| Domain-qualified asset identity with an issuer | Asserted at `:52` — "Assets and domain-qualified quantities have exact identities and units." | **Partly.** The only owner named is the U0 numeric profile, whose own enumeration at `:52` covers "canonical price orientation, unit dimensions, per-primitive rounding direction and beneficiary policy" — not asset identity structure, and not issuer. |
| Typed price / reserve-attestation observation | Yes — `:42` "typed observations with issuer, domain, time and finality"; Conditions row `:30`. | Yes, split: language types them, kernel collects them, issuers stay trust assumptions (`:30`). Kinds are not enumerated. |
| Collateral guard, liquidation, peg defence, supply cap, shutdown | **Not named anywhere** in either document. | — |

**3. Architectural holes** — a requirement stated with nothing on the other side of it.

- **H1 (the hole).** `:42` binds supply changes into every accepted stage. `:26` omits them from the
  row that allocates financial meaning across the three columns. `:15` gives them no core construct.
  `:93` and `ROADMAP.md:48` give them no library family. So the canonical stage statement requires a
  field that no layer of the architecture is responsible for producing. This is not a spec gap:
  nobody has been told to write it.
- **H2.** `docs/MORIARTY-PRODUCT-CONTRACT.md:43` (which `ROADMAP.md:3` makes controlling) requires
  "**authorized** mint/burn supply changes". "Authorized" asserts an authority relation that `:54`,
  `:27` and `:68` provide no kind of right to express. A property is asserted whose authority model
  has no slot for it.
- **H3.** `:52` asserts domain-qualified asset identity; the numeric profile is the only owner named
  and by its own scope does not cover identity structure. Issuer, obligor and jurisdiction have no
  owning component at all — and the kernel proves that without them USDT and USD1 are the same asset
  (`defiformal algebra/MODEL.md:128-135`).
- **H4.** `:93` names "redemption/loss allocation" as a library family. A redemption library over a
  core with no issuance operation and no supply state is half a pair; the architecture placed the
  exit and not the entry.
- **H5.** `:48` requires binding "actual complete effects rather than a convenient projection
  selected by a prover". With supply changes having no producer, a projection that omits them is
  indistinguishable from a complete one, and no component is designated to detect that.

**4. Canonical stage statement.** `:42` binds, relevantly: supply changes (as effects), opening and
closing liabilities, observations with issuer/domain/time/finality, authority consumption and replay
state, failure policy. Exact omissions for this category: (a) **supply authority** — only "authority
consumption and replay state" is bound, never a right to issue; (b) **issuer on the asset** — `:42`
gives `issuer` to *observations*, not to assets; (c) **no cap on supply** — the caps in the
architecture are gross-debit and fee caps (`:56`), so a debt ceiling has nothing to bind to;
(d) **no relation** between the bound effects and the bound supply changes; (e) no collateral, price
or health field; (f) one "chain/domain" per stage, so cross-domain burn-mint has no invariant to
bind. **E1 re-checked:** the design states no per-asset conservation law at all. `:54`'s only
conservation sentence is a *negative* one ("Token conservation alone cannot prove these
properties"), and `:56` is about netting, not supply. So `UNIFIED-PROPOSAL.md:90`
("Σ gross = Σ supply changes = 0") is not a restatement of an architectural law — it is the first
statement of one, made in the degenerate no-mint form, with no design-level general form to be
checked against. My §3 R4 / §5 G2 finding stands and is stronger on this axis: the specialisation
cannot be caught by comparison with the architecture, because the architecture is silent.
`docs/MORIARTY-PRODUCT-CONTRACT.md:43` "Debt is not token supply" is the one architectural
separation E1 does respect.

**5. Ownership of the architecture work.** U0 owns freezing what already exists: the stage relation,
judgments, numeric profile and enforcement map (`ROADMAP.md:21`). U1 owns certificates binding that
profile (`:52`). U6 owns the library families and the DeFi conformance rows (`ROADMAP.md:27,48`).
**The architecture work itself — deciding whether issuance is a core mechanism, a library family or a
kernel service — is unassigned.** No line in either document assigns it, and both forward plans skip
it: `ROADMAP.md:38` goes to a two-asset escrow discriminator, and
`docs/MORIARTY-CONSOLIDATED-DESIGN.md:107` sequences "conditional two-asset escrow, partial
fulfillment, residual obligations and late-result recovery, then private history and federated
execution". Issuance appears in neither sequence.

**6. Minimum addition so issuance can be built later without redesigning the core.** Five lines,
four of them in documents U0 is already freezing:

1. **A third authority discipline at `:54`**, alongside linear consumed receipts and affine spending
   permission: a supply authority that is neither, plus a matching scope in `:68`'s right list. This
   is the one item that *must* land before U0-F, because adding a right kind afterwards changes the
   authority model, the signed-intent grammar and the authority judgment together.
2. **Supply in the Financial meaning row at `:26`**, so the responsibility boundary actually
   allocates the field `:42` already requires. Without it H1 survives every later milestone.
3. **A named library family** in `:93` and `ROADMAP.md:48` — "issuance and redemption" — paired with
   the existing "redemption/loss allocation", so U6 has something to own.
4. **A general per-asset conservation law at design level** (declared supply change equals the sum of
   balance changes for that asset and domain), so U0's E1 *instantiates* it at zero for S0 rather than
   defining conservation in the no-mint form.
5. **Asset identity declared a structured value** at `:52` — domain, issuer or policy, identifier,
   denomination, version — with a named owner. `docs/MORIARTY-CONSOLIDATED-DESIGN.md` already asserts
   the property; only the owner is missing.

Items 2, 3 and 5 are allocations, not new mechanisms. Item 1 is the only core change, and it is
cheap now and structural later. Item 4 is a correction to a law that is about to be frozen.

**Limits of this section.** I read both documents in full and cite only them, plus
`docs/MORIARTY-PRODUCT-CONTRACT.md:43` (made controlling by `ROADMAP.md:3`) and one defiformal line
already verified in §2. I executed nothing for this section. The claim that no *other* Moriarty
document places issuance architecturally is scoped to the files listed in §1 of this report; I did
not re-search the repository for this axis.
