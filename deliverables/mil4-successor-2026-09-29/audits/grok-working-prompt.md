You are an independent Grok 4.6 high-effort read-only reviewer. Audit the exact full MIL/4 WORKING-DRAFT candidate with SHA-256 94c89e3dc1f74a2bf1495d4d6225b66186c74f02d7c316a4c51755d415ce8134. All required context is inlined. No tools. Give APPROVE/WARN/BLOCK for working-draft coherence only, material anchored findings and exact fixes. Do not treat a working-draft approval as a MIL/4 design vote, U0 exit, implementation, proof or Midnight acceptance. Record actual returned model identity and terminal state if available.

FULL CANDIDATE
# Moriarty Intent Language (MIL/4) — successor working design

**Date:** 2026-09-29. **Status:** working successor proposal. This document starts from the [MIL/3 specified-only draft](../mil3-design-draft-2026-09-29/DESIGN-MIL3-DRAFT.md) and uses its [decision and implementation workflow](../mil3-design-draft-2026-09-29/FINALIZATION-AND-IMPLEMENTATION-WORKFLOW.md). It records a proposed semantic delta; it is not an adopted language profile, a U0 exit, a proof result or a Midnight ledger result. Earlier MIL/2 and MIL/3 texts and review receipts remain intact.

## 1. Role of MIL/4

MIL/4 is a successor contract for the eight DeFi families previously reviewed by separate category agents. It keeps MIL/3's six U0 judgments (`stage`, `intent`, `effect`, `authority`, `history`, `failure`), selected-program rule, exact prepared effects, typed resources and persistent duties as the starting proposal. It makes the missing state, arithmetic, authority and admission rules explicit. Every family has a first profile and a separate deferred boundary. A specified profile is not automatically an admitted, implemented or native accepted profile.

The first implementation slice remains the [U0 S0 proposal](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md): literal-fee transfer and funded AccrualFirst repayment of an existing obligation. The MIL/3 workflow's W-D0–W-D4 decisions, design gate M3-D, reference gate M3-R and native gate M3-N remain open. MIL/4 cannot inherit a passing result from those gates by changing the version number. Broader DeFi profiles enter only after their own source/Core/K/native/ledger evidence under U1–U6.

## 2. Shared successor decisions

These are live MIL/4 decisions, not implicit changes to the recorded U0 policy. Each issue record will retain its alternatives, affected clauses, evidence, two reviewer verdicts, dissent and final disposition. None is disposed by this working text. M4-C1–C5 supplement the MIL/3 workflow's W-D0–W-D6 register; W-D0–W-D4 still gate S0, while W-D5 and W-D6 gate the applicable evidence and category profile. A category cannot inherit a favorable review of another row.

| ID and inherited issue | Proposed direction and live alternative | Discriminator before selection |
| --- | --- | --- |
| M4-C1 / W-D2 | **Canonical names and bytes.** Reuse U0's base-per-quote orientation. Write `Price<Base,Quote,Scale>` everywhere, name both nominal assets and domains, and reject implicit reciprocals. Alternative: keep MIL/3's shorter notation but prove its `b,q` positions map to base and quote without ambiguity. `/3`→`/4` encoding, unknown-tag rejection, migration and source/Core round trips are open. | Invert a typed price, rename a constructor or add an unknown tag under both versions; require one canonical digest or a deterministic reject/migration result. |
| M4-C2 / W-D4 | **Rounding and dust.** Retain the [U0 numeric decision](../../docs/decisions/u0-numeric-profile-decision.md) as controlling policy. State each primitive's direction and beneficiary. Alternative: a versioned owner amendment if the policy cannot be represented. A fractional quotient remainder is not a ledger Qty; the exact accounting meaning of “dust accrues to protocol reserve” remains open. | Compute a nonzero fractional remainder and a separate one-unit allocation residue, then name every integer effect and reserve change. A hash-bound policy change needs a new decision. |
| M4-C3 / W-D4 | **Closed financial arithmetic.** Propose a closed certified Ω in the selected program while general nonlinear intent formulas remain deferred. Alternative: defer all Ω to U4 as MIL/2 proposed. Input/intermediate widths, total rejection, exact output and target cost are open. | One feasible result and one otherwise valid forged quotient or overflow must reach the arithmetic rule on the pinned target. Host computation alone is insufficient. |
| M4-C4 / W-D3 | **State and footprints.** Register every aggregate, reserve, custody, supply, claim, receipt and policy cell; derive complete footprints after holes resolve and authenticate the required heads. Alternative: a smaller first profile with fixed-empty cells and explicit rejection. | Delete one derived endpoint, aggregate or policy read from an otherwise valid stage; reject before acceptance. |
| M4-C5 / W-D1, W-D5 | **Authority, evidence and partial progress.** The signature contract must name scheme, bytes, digest, verifier locus and public inputs before it can bind recipients, fee scope, duties, evidence and failure branches. Imported evidence names a verifier premise. Alternative: defer a foreign-dependent profile until a qualified verifier exists. | With a valid envelope, substitute one signed recipient byte, revoke a grant, replay a receipt or omit a retained duty. Name the rejecting judgment and accepted-state result. |

For every family profile `p`, instantiate the inherited MIL/3 stage relation as six explicit obligations:

```
stage_p    : selected program, authenticated heads, typed pre-state, prepared transition
intent_p   : exact signed bytes, filling, recipients, bounds, failure and disclosure
effect_p   : complete canonical lines, derived footprint, conservation and post-state
authority_p: current grant, budget, issuer or verifier right, and replay consumption
history_p  : authenticated predecessor, head extension and retained claim identity
failure_p  : named rejection with no accepted writes, or signed retained phase effects/duties
```

These lines are an obligation map, not six proved judgments. A profile must give the exact cells, effects, rejection precedence and positive/hostile traces for each line before its design gate. Changing only a predecessor commitment or an unpaid duty in an otherwise feasible stage must select `history_p` or `failure_p`, respectively, rather than pass through `effect_p` alone.

The [September 23 U0 exit](../u0-semantic-contract-2026-09-23/EXIT-GATE.md) retains its recorded absent embeddings, uncovered K rows, unenforced native leaves and target-pin gap. MIL/4 decisions do not re-score it. A later result record must distinguish source, Core, K, native proof and Preview ledger effects.

## 3. Eight DeFi profile comparisons

The [eight MIL/2 category reports](../mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md) and their Opus recommendation syntheses are research inputs. The MIL/3 §8 schemas are the specified-only starting point. The following are proposed MIL/4 contract additions and evidence cases; they are not coverage verdicts.

| Family and proposed first profile | MIL/3 already specifies | MIL/4 decision to close | First positive / well-formed hostile case | Later boundary |
| --- | --- | --- | --- | --- |
| **AMM and exchanges** `amm-cp/1` | One-hop exact-input constant-product quote, fresh pool head, four balance effects, signed fee scope and surplus. | Bind authenticated reserves to custody and pool head, fee units and retained value, exact quotient/tightness, widths, pool authority and full footprint. Select exact rational fee within the quote or upfront rounded fee as a separate W-D4 decision. | Funded 11 A for exact 23 B at 1000/2200 reserves and 3/1000 fee; reject otherwise valid 24 B output despite the trader's 20 B floor. A separate otherwise identical 23 B case distinguishes the fee policies because upfront one-unit fee permits only 21 B in the draft example. | Exact output, LP issue/burn, routes, partial fills, maker orders and multisigner clearing. |
| **Lending and borrowing** `loan-fixed/1`, then `loan-coll/1` | Funded lifecycle and AccrualFirst repayment; collateral lock and liquidation sketch. | Fix period/rate/rounding, obligation status, matched funded changes and one-shot consent. Separately fix aggregate lock transitions, selected current price, close factor, bounded seizure, surviving shortfall and loss waterfall. | Repay 30 from principal 1000/accrued 10 to principal 980/accrued 0 with 30 paid; reject the same debt reduction without the creditor payment. | Variable rates, pooled shares, flash liquidity and portfolio liquidation. |
| **Stablecoins and synthetics** `cdp/1` | Positive mint with issue grant, matching debt/lock, global ceiling, exact supply and balance, typed price policy. | Register authenticated aggregate ceiling cell; define canonical mint/burn effects, grant quota and per-line authority, atomic debt/lock/supply correspondence and bounded health comparison. | Lock 150 collateral at 3:2 requirement and mint 100 units under current cap; reject positive supply without matching debt or lock. | Redemption duty and settlement burn, reserve-backed issuance, shutdown, rebasing, shared debt and cross-domain supply. |
| **Derivatives** `opt-capped/1` | Finite funded call reserve, capped payoff, selected fixing, one-shot exercise and retained unpaid duty. | Fix instrument/claim IDs, premium and writer funding, exact selected round, integer payoff/rounding, explicit versus automatic exercise and state transitions through payment/reclaim. | Proposed case: size 1.5 underlying, strike 3000 USDC, cap 3500, fixed price 3123.456789, six-decimal USDC and funded 750 USDC maximum reserve. Floor payout is 185.185183 USDC; reject a one-micro-USDC underpayment with otherwise conserving effects. The fixing policy and premium still require exact terms. | Perpetual funding, margin, liquidation and social loss. |
| **Oracles and observations** `obs/1` | Typed observation admission, selected current round, policy and verified provenance label. | Bind canonical observation tuple and exact bytes, verification locus, key epoch, authenticated time/status and branch-independent admission. | Admit selected current typed round; reject a still-fresh but nonselected older round with a valid financial envelope. | Bounded median/TWAP and imported verifier profiles. |
| **Governance** `gov/1` | Queue/execute/cancel, grant epoch and policy head, protected existing duties. | Fix distinct approval set and consumption, exact queue/action digest, execution-time grant/head check, pause mask and policy migration of duties. | Apply authorized amendment under current head; reject execution after grant revocation with unchanged signed action bytes. | Voting snapshot and general multisigner stage admission. |
| **Bridges and cross-domain settlement** `bridge-pair/1` | Unique source claim, one-shot destination receive/timeout, refund under named nonreceipt premise; partial delivery is permitted in prose. | Fix claim/message/nullifier bytes, verifier/finality and epoch premise, installment count and remaining entitlement, exact conversion, authenticated destination terminal state and refund after partial delivery. MIL/3's no-receipt timeout does not settle a partly received claim. If no qualified foreign verifier exists, this profile remains specified-only. | Propose source lock of 100 and verified one-shot destination delivery of 99 under a fee bound of 1; reject source refund after time elapses while destination nonreceipt is unknown. | Bonded fast fill, additional representation ratios and reorg-loss policy. |
| **Staking, restaking and yield** `vault/1` | One-asset/one-class conversion, virtual offset, custody surplus, authorized recognition and later withdrawal/slash lifecycle. | Fix share issue authority, exact floor/ceil widths, donation and recognition beneficiary, virtual value ownership, managed/custody/surplus accounting and complete effects. Treat withdrawal, rewards and slash as later profiles. | With managed=999, supply=1998, virtual offsets=1 and deposit=500, accept 999 shares with remainder 500; reject claimed 998 shares/remainder 1500 because remainder must be below divisor 1000. | Validator/AVS integration, reward indexing, withdrawal and slash lifecycles, strategy mandates, principal/yield splits. |

Each first profile requires a complete constructor and cell register, a positive trace and a hostile trace that reaches the intended semantic rule. A broad family name does not admit the later boundary. Imported primary sources are comparative anchors, not evidence that a Moriarty rule works.

### Numeric and effect conditions on the proposed traces

- **AMM.** The 23 B result uses an exact rational 3/1000 fee inside the pool quote. The charged fee is 0.033 A as a rational calculation, not an atomic 0.033 A ledger transfer. An upfront ceil fee of 1 A gives the alternative 21 B quote. The issue record must name the fee-cap unit, beneficiary and whether the fee stays in pool custody or causes an additional protocol-transfer effect. Four balance effects suffice only for the retained-in-pool form.
- **Loan.** The 30-unit repayment has no fee, debits the debtor 30, credits the creditor 30 and changes debt from principal 1000/accrued 10 to principal 980/accrued 0. An impairment entry cannot substitute for the funded discharge.
- **CDP.** The 150-for-100 equality case assumes one collateral asset, one synthetic asset, compatible declared decimals, a current base-per-quote price of one synthetic unit per collateral unit, a 3/2 requirement and a `≥` health rule. Those assumptions, their exact scaled comparison, grant use, price policy, global debt head and any haircut/fee must be signed and authenticated before the case can count as positive.
- **Option.** Size 1.5 times `min(max(3123.456789−3000,0),3500−3000)` is 185.1851835 USDC; floor to six decimals gives 185.185183. Size times the cap spread is 750 USDC. The stage still needs a declared scale convention, premium, reserve funding, selected fixing round, exercise rule, exact payout and residual reserve effects. The one-micro-unit underpayment isolates exact payoff while preserving gross conservation.
- **Bridge.** A 100-unit source lock and 99-unit destination credit do not by themselves explain the remaining one unit. The first profile must name representation ratio/decimals, whether it is a charged fee or still-undelivered entitlement, its beneficiary, cumulative delivery and the maximum later refund. A partial 99-unit receipt needs a state distinct from both no receipt and completed delivery; destination terminal nonreceipt must refer to the remaining entitlement.
- **Vault.** In the deposit case, `500·1999 = 999·1000 + 500`; the remainder 500 is in product units, not an independent 500-unit asset or share payment. Post managed assets are 1499 and share supply 2997. Custody, surplus, virtual value ownership and the beneficiary of retained rounding must be stated. A claimed quotient 998 with remainder 1500 satisfies the equation but violates `0≤r<1000`.

The [U0 study's remainder classes](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md) distinguish a charged liability increment, sub-unit residual and whole atomic-unit protocol reserve. MIL/4 must reconcile them with the owner's [U0 numeric decision](../../docs/decisions/u0-numeric-profile-decision.md) for every primitive. A protocol-reserve line is possible only for an identified whole unit with matching conservation effects. Any different beneficiary is an explicit versioned policy override; it is not silently inferred from floor or ceil.

The category-specific agent assignments used these retained baseline reports and five-review recommendation sets. Each current assignment compared MIL/3 with its category; the recommendations are research input, not the two independent full-candidate design votes:

| Family | Baseline report | Recommendation synthesis |
| --- | --- | --- |
| AMM and exchanges | [MIL/2 category 1](../mil2-deep-research-2026-09-29/category-review/01-amm-exchange.md) | [AMM](../mil2-deep-research-2026-09-29/opus55-amm-recommendations/SYNTHESIS.md) |
| Lending and borrowing | [MIL/2 category 2](../mil2-deep-research-2026-09-29/category-review/02-lending.md) | [Lending](../mil2-deep-research-2026-09-29/opus55-lending-recommendations/SYNTHESIS.md) |
| Stablecoins and synthetics | [MIL/2 category 3](../mil2-deep-research-2026-09-29/category-review/03-stablecoins.md) | [Stablecoins](../mil2-deep-research-2026-09-29/opus55-stablecoins-recommendations/SYNTHESIS.md) |
| Derivatives | [MIL/2 category 4](../mil2-deep-research-2026-09-29/category-review/04-derivatives.md) | [Derivatives](../mil2-deep-research-2026-09-29/opus55-derivatives-recommendations/SYNTHESIS.md) |
| Oracles and observations | [MIL/2 category 5](../mil2-deep-research-2026-09-29/category-review/05-oracles.md) | [Oracles](../mil2-deep-research-2026-09-29/opus55-oracles-recommendations/SYNTHESIS.md) |
| Governance | [MIL/2 category 6](../mil2-deep-research-2026-09-29/category-review/06-governance.md) | [Governance](../mil2-deep-research-2026-09-29/opus55-governance-recommendations/SYNTHESIS.md) |
| Bridges and cross-domain settlement | [MIL/2 category 7](../mil2-deep-research-2026-09-29/category-review/07-bridges.md) | [Bridges](../mil2-deep-research-2026-09-29/opus55-bridges-recommendations/SYNTHESIS.md) |
| Staking, restaking and yield | [MIL/2 category 8](../mil2-deep-research-2026-09-29/category-review/08-staking-yield.md) | [Staking and yield](../mil2-deep-research-2026-09-29/opus55-staking_yield-recommendations/SYNTHESIS.md) |

## 4. Decision, implementation and audit order

1. Complete the MIL/3 workflow's S0 decision contract. Record any MIL/4 change to an S0 rule as a new versioned issue, not an inherited approval.
2. For each MIL/4 category row, let its assigned agent produce an independent need-by-need comparison against MIL/3, the prior category report and primary sources. The lead records disagreement and the smallest discriminating example. A formal semantics reviewer checks the combined stage relation, and a financial reviewer checks economics and units.
3. Freeze one full candidate and send the same bytes to fresh Grok 4.6 high and GPT-6 Astra auditors. Record requested and returned identity, effort, digest, terminal status, verdict and dissent. Two substantive agreeing votes are required for a consequential design choice under `AGENTS.md`. Changed normative bytes require new audits.
4. Implement only the first admitted profile in the actual source/5 → Core/4 consumer after its design gate and one-active-delivery-item check. Compare TypeScript and K on the same feasible corpus. Then certify each used primitive at U1 and bind proof, current state and complete ledger effects at U2. Repeat qualification for each later family under U6.
5. Keep SP01.6 blocked history and the existing guarded campaign rules. Independent source research is allowed; a new delivery implementation item cannot occupy the current active slot without an existing guarded transition. No missing native interface, reviewer or result is replaced by a host assertion.

## 5. Evidence status

This first MIL/4 text is a working proposal. It records no design vote, no profile admission and no implemented or native result. Eight category-specific assignments and formal and financial preparation reviews are complete; full-candidate audits remain open. No test, proof, compiler or Preview transaction was run for this document.


MIL/3 DESIGN DRAFT
# Moriarty Intent Language (MIL/3) — integrated design draft

**Date:** 2026-09-29. **Status:** specified-only candidate, not an adopted successor to [MIL/2](../../concepts/intent-language/DESIGN-MIL2.md). **Basis:** the eight [DeFi coverage reviews](../mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md), forty sequential Opus 5.5 category recommendations, five independent MIL/3 drafting assignments, the [separate MIL/2 semantics](../mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex), and the [consolidated roadmap](../../ROADMAP.md). The five drafting receipts and independent texts are under [agents](agents/). This document reconciles proposals for review. It does not close U0, change a frozen profile, certify a primitive, or show Midnight acceptance.

## 0. Claim and notation conventions

- **[checked]** means checked against a cited repository file or follows directly from a stated definition. It does not mean a compiler, proof, circuit or ledger check ran.
- **[obligation]** is a property that must be proved or executed against the relevant source/Core/K/native/ledger boundary before freeze or acceptance.
- **[deferred]** is outside this candidate's admitted profile. No untagged formal claim is load-bearing.
- `Reject(c)` is a named rejection with no accepted state change. `Pending` is a valid persistent state. An accepted phase failure has exactly its signed retained effects and duties.

This candidate keeps the repo's [six U0 judgment keys](../../ROADMAP.md): `stage`, `intent`, `effect`, `authority`, `history`, and `failure`. Category profiles can refine them but cannot omit one. The project's Grok 4.6 and GPT-6 Astra routing for consequential decisions remains separate from these user-requested Opus proposals; no design vote is asserted here.

## 1. Purpose, layers and limits

MIL/3 is a signed template for acceptable financial outcomes. A solver may fill typed holes but cannot change the signed recipient, fee scope, evidence policy, duties, authority, disclosure or failure terms. A selected program must also accept the same transition. This closes the gap between “the trader received enough” and “the pool or lender's rules were followed.”

```text
MIL/3 template I + admissible filling σ
           │ canonical bytes, digest, typed elaboration
           ▼
Intent Core + selected versioned Program Core P
           │ one stage, one executing domain, concrete pre-state
           ▼
Stage candidate: selected branch, prepared effects, post-state
           │ stage ∧ intent ∧ effect ∧ authority ∧ history ∧ failure
           ▼
Native statement + ledger validity and complete effect readback [obligation]
```

An Episode is a ledger-linked sequence of stages, each with its own proof and authenticated predecessor. There is no global rollback across stages or domains. `|signed intents| = 1` and one program invocation per stage are the initial profile; arity and composition fields are reserved for later admission. Bounded in-stage flash traces, shared-write multisigner clearing, concentrated liquidity, general nonlinear Φ₁ predicates, and recursive parent proof verification remain **[deferred]**. The U4 recursion and private handoff target is not replaced by ledger induction.

## 2. Core sorts and state

```
AssetId                 opaque nominal ledger identity
Asset                   {id, domain, issuer, repr, decimals, checked metadata}
Qty<a>                  checked unsigned integer, in asset a's smallest unit
Delta<a>                signed change of a balance or supply cell
Price<b,q,s>            quantity of b per quantity of q, scale 10^s
Share<p,c>              Qty<shareAsset(p,c)> for pool p and class c
Position<instrument>    exposure or paired claim; never a spendable balance
Instant<clock>, Duration<clock>
ObsId, EvidencePolicyId, GrantId, ClaimId, ObligationId, EpisodeId
```

The core cell registry includes `balance(d,holder,a)`, `supply(d,a)`, `obligation(o)`, `encumbrance(k)`, `lockTotal(d,holder,a)`, `poolHead(p)`, `managed(p,a)`, `shareSupply(p,c)`, `observation(i)`, `grant(g)`, `policyHead(v)`, `receipt(r)`, `claim(x)`, `replay(key)`, `episodeHead(e)`, and `tombstone(id)`. A profile registers any additional cell kind, its concrete read/write derivation and a versioned migration before that registry is frozen. A dynamic cell selected by a hole has a statically bounded resolution or rejects. **[obligation]** The initial list and migrations must be reconciled with MIL/2's U0 vocabulary and actual target limits.

Asset identity is nominal: matching symbol, issuer and decimals never makes two ids equal. `Share<p,c>` has its own nominal asset id; its issue right is scoped to the pool program. An obligation is a liability, not negative supply. A position is exposure or a claim, not a spendable balance. **[checked by construction]** These distinctions make supply, collateral and payoff checks separate.

## 3. Signed templates, completion and Φ

```
intent ::= version signer domain validity assets policy refs
           budget gross/fees/net evidence authority disclosure failure
           holes* machine? surplus? residue?
hole   ::= name : sort where φG [sources permitted] [finite cell range]
φG     ::= typed Φ₀ formula over literals, holes, pre(cells), admitted observations,
           authenticated stage time and bounded set predicates
φE     ::= φG | post(cell) | outcome(effect) | completed-program result
```

`Route` and `Program` holes select effect structures; they are not Φ terms. Φ₀ permits finite checked sums, literal coefficients, bounded `min/max`, `k_of_n`, Boolean connectives and atomic negation. General variable products and division remain Φ₁. A profile may register a **closed certified transition operation** Ω, such as `divmod` or `mulCompare`; Ω runs inside a selected program transition on a concrete witness. Its result may be named in a Φ₀ outcome bound. Ω does not make arbitrary nonlinear intent predicates available. This is a proposed change to MIL/2's U4 placement of all pool arithmetic, not a feasibility result.

`surplus` is mandatory for a swap or solver completion that sets a minimum credited output while permitting a greater output. It names the owner, completer or a literal split, with no implicit default. The clause allocates the difference between the selected program's **tight** output and the signed floor. It cannot be satisfied by first under-delivering from the pool and then splitting only the smaller delivered amount. Tightness is conditional on the authenticated pre-state and selected venue; it does not claim best execution across venues or protect against transaction ordering. The remaining ordering and MEV exposure belongs in the signed residue.

Typing has two strata. A guard, hole bound and branch priority uses `φG` and cannot read its own stage's `post` or effects. Program ensures and signed outcome checks use `φE` after prepared effects exist. An attempted `post` in a guard rejects `TYPE_POST_GUARD`. This prevents a self-fulfilling release condition. Terms compare only matching assets, price orientations, scales and clocks; an explicit conversion operation is needed to cross them.

Evaluation is total into `Value | Reject(code)`. Checked subtraction underflow, zero divisor, overflow, bad time, invalid threshold `k`, unsupported operation or out-of-range witness each has a code. `Reject` is neither true nor false and never triggers a fallback branch. Boolean evaluation may short-circuit after static type checking; every observation named by the stage is admitted before any branch evaluation, including observations in a skipped arm. **[obligation]** Prove totality and source-set preservation, then compare TypeScript and K behavior over the admitted grammar.

Let `Fill(I,σ,h)` require every hole filled exactly once, correct sort and bound at authenticated head `h`, permitted evidence sources, unchanged fixed bytes and canonical encoding. Let `πI` retain the signer's effects, recipients, fees, duties, authority consumption, disclosure and terminal status while hiding only declared route internals. Then

```
Tr(I) = { πI(r) | ∃σ,h. Fill(I,σ,h) ∧ Stage(I,σ,h,r) }.
```

For a fixed filling, `Tr(I,σ) ⊆ Tr(I)` follows by definition **[checked]**. The substantive **[obligation]** is `NativeAccept(I,σ,w,ledger) ⇒ Fill(I,σ,h) ∧ πI(ledger) ∈ Tr(I)`, with exact signed bytes, prepared effects and authenticated reads bound at the native and ledger boundary. A numerically bounded hole that redirects a recipient or downgrades evidence is a hostile control.

## 4. Stage relation, effects and failure

For one executing domain `d`, authenticated pre-state `s@h`, selected branch `b`, observations `O`, prepared canonical effects `eP` and post-state `s'`, propose:

```
Stage(I,σ,P,h,O,b,e,s') :=
  Admit(I,σ,h,O) ∧ Select(b, pre(s), O) ∧ ProgramValid(P,s,O,σ,b,eP)
  ∧ ExactEffects(e = canonical(eP ⊎ eI)) ∧ Promise(I,e,s')
  ∧ ResourceSafe(s,e,s') ∧ AuthorityFresh(s,e) ∧ FootprintSound(I,P,e)
  ∧ HistoryLink(h,e,s') ∧ FailurePolicy(I,b,e,s').
```

`eI` contains only explicitly allowed intent-side lines, such as a signed solver fee. It cannot alter the selected program's custody, supply or liability effects. `ProgramValid` requires the selected versioned program/Core identity at `h`, successful reduction including all `Ensure` conditions, protected-operation preparation of `eP`, and exact correspondence between prepared and submitted effects. `s' = apply(s,e)` on every derived write cell and `s' = s` elsewhere. This added conjunct is necessary for AMMs: a trader's minimum output can hold while a forged pool transition drains LP reserves. **[obligation]** Native acceptance must bind the selected program and all effect lines to one statement and actual ledger readback.

Every effect constructor yields a complete derived read/write footprint. A transfer reads and writes both endpoint balance cells; mint/burn also touch supply and scoped issue authority; debt changes touch obligation and funding; a debit touches aggregate lock state. Resolve aliases after completion, expand registered dependencies, and require declared `R ⊇ Rderived`, `W ⊇ Wderived`. An opaque `escrow(E)` cell is not permission to omit actual recipient balances. The MIL/2 `AcquireB` showcase as written fails this rule because its A-recipient writes are absent. **[checked against the showcase and the separate semantics draft]**

Reject before a state transition for ill-formed syntax, failed admission, evaluation `Reject`, false program/promise/resource conditions, stale heads or incomplete footprints. `Pending` is a valid stage-linked state. A ledger phase failure can be accepted only when its signed failure branch names exactly the retained fees, effects and continuing duties. No local evaluator rejection is silently turned into that branch. A fixed failure-code order is proposed for source/Core/K/native comparison; the wire codes themselves remain **[obligation]**.

## 5. Resource and numeric contract

For each domain `d` and nominal asset `a`, the full effect set must satisfy

```
Σholder Δbalance(d,holder,a) = Δsupply(d,a) = Σmint(d,a) − Σburn(d,a).   (E1)
```

Holders include users, custody, reserves and fee recipients. `Δsupply` is derived from typed mint/burn lines, not separately claimed. A positive supply effect needs a current, budgeted `issue(d,a)` grant. A synthetic mint also names a backing mode: CDP debt and lock, reserve under a named verification premise, or an explicitly admitted unbacked profile. No unbacked profile is admitted by this draft. Share supply is E1 applied to `shareAsset(p,c)`; share issue is allowed only to the versioned pool program. **[obligation]** Prove E1 against all constructors and actual ledger effects.

For an obligation `o`, `outstanding = principal + accrued`. Origination requires debtor consent and a funded creditor-to-debtor transfer. Accrual adds a checked amount under the pinned terms. Repayment `0<n≤outstanding` requires a matching funded transfer and discharges accrued first: `da=min(n,accrued)`, `dp=n−da`. Forgiveness is a separate creditor-authorized effect. **Impairment records a loss state and bearer; it does not by itself erase the outstanding claim.** A later settlement or authorized forgiveness changes the legal amount. A deadline, pause or failed phase never deletes a nonzero duty. **[obligation]** Define the exact loss waterfall for each lending or issuance profile.

An encumbrance has reserved, committed, challenged, seized, released or cancelled state. `lockTotal(d,p,a)` includes every reservation or commitment that can still settle. Every debit from `(d,p,a)` checks `lockTotal' ≤ balance'`, and updates both the aggregate and affected lock cells atomically. One lock backing several obligations cannot be released merely because one obligation discharged. Seizure is replay-keyed and bounded by both its allocation and the locked amount. Competing accepted stages serialize on authenticated cell heads. **[obligation]** Inductively establish the lock invariant under all effects, including plain transfers and phase failures.

The numeric profile records `Qty` width, every input/intermediate bound, price orientation, exact operation-specific rounding direction, and the holder who benefits from a rounded unit. A witness for division supplies `(q,r)` with `n=q·d+r`, `0≤r<d`, `d>0`; round down returns `q`, round up returns `q+[r>0]`. A forged quotient, `d=0` or width overflow rejects. The role is determined by the operation (`deposit` and `redeem` down; `mint` and `withdraw` up; AMM exact-input output down; exact-output input up) and cannot be a hole. **A fractional `r/d` is not a ledger unit and has no transfer line.** In a multi-recipient allocation, any *integer* residue `X−Σallocations` is a real amount and must go to a named holder. This resolves the conflicting “post every remainder” proposal in [agent 4](agents/draft-04-profiles.md) in favor of the unit distinction in [agent 2](agents/draft-02-finance.md).

No field multiplication stands for integer multiplication unless bounds establish no wrap. `u128×u128` can require 256 bits; the effective comparison limit on the pinned ZKIR target and the two-limb cost are open. The fifth review found that the captured `LessThan` rule can enforce a rounded-up even-bit bound when its requested bit count is odd; it cannot by itself establish the exact producer range premise. The first numeric profile therefore requires explicit bit constraints on every operand and intermediate, including post-state sums, and an even comparison width until the pinned target is reconciled. A narrow `u112` pool/vault candidate is proposed only for a measured first slice. **[obligation]** U1 must certify each Ω relation with valid and well-formed hostile witnesses, caller preconditions and target cost. General Φ₁ stays **[deferred]**.

Gross debit counts actual owner-controlled outflows, including explicit fees, once across an Episode. A committed collateral reservation is separately exposed as `encumbered`, not double-counted as a realized debit; seizure or payout counts when it actually leaves. Fee attribution includes signed explicit, venue, protocol and retained LP fee categories, with a declared scope and unit. A refund never resets cumulative gross or fees. Net return is the actual credited amount to the signed recipient after deductions. An absent or ambiguous fee scope rejects rather than silently excluding retained pool fees.

## 6. Evidence, authority and history

An observation has canonical `ObsId = H(tag || canonical record)`, typed subject/value/unit, origin domain, assertor, round, observed time, policy digest and authenticated bytes commitment. Admission verifies every named observation before Boolean evaluation. The verifier or authenticated ledger read assigns its provenance label; a witness cannot call its own record `anchored`. Source sets carry `ObsId`s through all term and Boolean constructors and are checked against position-specific policy. A `fresh` value needs an authenticated stage clock and a selection rule, normally the head-current round; freshness alone cannot justify choosing a favorable older round. An imported label names its verifier and finality premise; it is not proof of the foreign assertion's truth. **[obligation]** Bind IDs, exact values, units, status, policy, time and label to source/Core/native acceptance.

Authority is an authenticated grant cell with right kind, scope, expiry, remaining budget, epoch and active/revoked status. Delegation attenuates scope and budget and cannot duplicate spending power. A queued governance action stores its approval set and policy head; execution rechecks current grant and head. Every persistent debt, position, escrow, bridge claim and vault request pins the policy that created it. An amendment governs new actions unless a preservation relation or affected-party consent migrates an existing object. Revocation and pause cannot erase repayment, challenge, withdrawal or recovery duties. Whether emergency suspension is a separate right kind remains open.

An Episode link contains `episodeId`, `predecessorHead`, `intentDigest`, cumulative gross and fees, liability roll-forward, authority and replay consumption, continuation commitment, and terminal tombstone. All links are authenticated current-head writes. A fork partitions linear receipts; branches sharing an affine budget or cell require serialization. An accepted failure preserves its declared residual duties. **[obligation]** Prove unique consumption and duty preservation by induction over the admitted stage and failure transitions, then bind each link field natively.

## 7. Conditional workflows and cross-domain safety

A local escrow moves `unfunded → pending → released | refunded` with exactly one terminal tombstone. Guards use only pre-state and admitted evidence. When both hold at the same authenticated head, a signed strict priority chooses one. An invalid higher-priority guard is `Reject`, not a reason to fall through. An authoring claim that some branch is eventually enabled needs a sound check over its actual guard grammar: difference-logic negative-cycle checking covers only a restricted fragment, while full Φ₀ requires a separate complete bounded procedure or a checkable certificate. Failure or timeout of that procedure returns `unsupported`; it cannot certify recovery. This corrects MIL/2's broad “difference logic, no solver” claim.

A bridge is a pair of local claim machines. Its immutable transfer identity binds source/destination domains, asset representation and ratio, transfer form, amount, owner/recipient, nonce, timeout and verifier policy. The destination receipt is one-shot and decides `received` versus `timedOut`; source refund requires authenticated destination nonreceipt or a receipt state that excludes future delivery. **A source deadline alone does not prove nonreceipt.** An unknown or partitioned outcome stays pending. Partial receipt reduces the remaining entitlement; a fast filler advances its own funds and receives a bounded reimbursement debt rather than creating an unbacked mint. Local conservation does not imply cross-domain backing; paired safety is conditional on the named foreign verifier, finality and availability premises. U3 can specify local asynchronous rules; a proven foreign bridge remains dependent on the U4 verifier path or an explicitly qualified earlier interface.

## 8. Eight versioned DeFi profiles

Each profile inherits §§2–7. Its first slice is a **candidate for future evidence**, not a coverage verdict. Profile parameters, policy digests and every new cell kind must be in the U0 registry before freeze or handled by a versioned migration.

| Family | First operational rule | First positive and hostile control | Deferred boundary |
| --- | --- | --- | --- |
| AMM/exchange `amm-cp/1` | A single-domain, one-hop, exact-input swap reads a fresh pool head and authenticated reserves. A certified quote or fee-adjusted invariant plus tightness fixes output; four balance effects are complete. Signed fee scope, recipient and input/output bounds still apply. | Accept a funded swap satisfying pool and trader. Reject one-unit excess output with a well-formed envelope even though the trader floor still holds. | LP mint/burn, weighted/concentrated liquidity, multi-hop atomicity, genuine multisigner clearing. |
| Lending `loan-fixed/1` | Funded originate, literal-rate accrue by period, partial repay and discharge update an authenticated obligation. A separate `loan-coll/1` adds aggregate locks, head-current typed price, close factor, funded liquidation and impairment waterfall. | Accept partial payment. Reject debt reduction without the matched creditor transfer or a debit that violates aggregate locks. | Variable rates, pooled credit, flash liquidity, general portfolio liquidation. |
| Stablecoin `cdp/1` | A positive mint has a scoped issue grant, matching CDP debt and lock, current debt ceiling, exact supply/balance effects and pinned price policy. Burn/repay and redemption have their own settlement points. | Accept a bounded funded draw. Reject positive supply with no matching debt/lock, or a stale global ceiling. | Unbacked/rebasing/algorithmic modes, shutdown pro-rata distribution until claimant snapshot and loss policy are specified. |
| Derivative `opt-capped/1` | Immutable instrument terms include strike, cap, expiry, settlement asset and fixing policy. A **capped** cash-settled call has finite maximum payoff locked at write; fixing is write-once, exercise consumes its claim, unpaid payoff remains a duty. | Accept settlement from the specified fixing round. Reject a wrong round or repeated exercise. | Uncapped cash-settled call, perpetual funding, portfolio margin and socialized losses. |
| Oracle `obs/1` | Admit a canonical observation against head-current feed cell, typed unit, status, round, time and evidence policy; assign provenance only after verification. | Accept one bound current round. Reject an old-but-fresh alternative round or revoked key. | Median/TWAP until bounded collection or accumulator semantics exist. |
| Governance `gov/1` | Grant/revoke and queue/execute/cancel use epoch/head checks, distinct approvals, pinned policy and protected existing duties. | Accept an exact authorized amendment. Reject execution after grant revocation or beneficiary change. | Voting snapshots and general threshold process as separate library rules; multisigner stage admission. |
| Bridge `bridge-pair/1` | Source lock/burn creates a unique claim; destination one-shot receive or timeout resolves it under a named verifier; source refund follows verified nonreceipt. | Accept a locally valid source lock and a later authenticated claim. Reject a refund based only on a source deadline or replayed foreign message. | Foreign verifier/finality assurance, bonded fast fill and reorg loss rule. |
| Staking/yield `vault/1` | One-asset, one-class deposit/redeem reads accounted managed assets and share supply, uses fixed bootstrap and role rounding, with complete share and asset effects. Later request/finalize/claim, reward index/checkpoint, replay-keyed slash and mandate are separate transitions. | Accept a funded deposit. Reject forged quotient, zero-output deposit or custody donation treated as managed gain. | Validator and AVS integration, reward curves, complex strategy and principal/yield splits. |

An uncapped cash-settled call cannot be described as “fully collateralized” by a finite amount of quote asset when its payoff is unbounded. The capped profile repairs that first-slice recommendation; an underlying-settled call or bounded put are alternatives. Pool donations and vault donations may follow different declared policies; neither can silently change an authenticated quote input.

### 8.1 Candidate transition schemas

The following schemas are **specified-only** and inherit the stage relation in §4. `pre` values come from one authenticated head; all listed updates form one exact effect set on the executing domain. A profile must reject an omitted effect, stale version, overflow, unauthorized write or mismatched policy digest. Numbers below are integer quantities in each nominal asset's smallest unit.

**AMM exact input.** Let `x,y>0` be authenticated, accounted pool reserves in input asset `A` and output asset `B`, and let fee fraction be `f/F`, with `0≤f<F`. For input `dx>0`, the candidate first slice fixes

```
dy = floor(y · (F−f) · dx / (x·F + (F−f)·dx)),   0 < dy < y.
```

The stage transfers `dx A` owner→pool and allocates the full `dy B` pool output according to the signed surplus clause; the initial one-recipient slice sends all `dy` to the owner. It writes the pool head and accounts for any declared fee attribution without moving a fictional fractional fee. The formula puts the **exact rational fee** inside the invariant. Rounding a whole-unit fee up *before* the invariant is a different economic policy: at `x=1000`, `y=2200`, `dx=11`, `f/F=30/10000`, the former permits `dy=23`, while the latter permits only `dy=21` when the rounded fee is one A. These rules are not equivalent. A fee-adjusted post-state invariant can be checked as a redundant condition; it cannot replace exact output without a tightness proof. The input and every intermediate sum and product must meet the certified width profile. A direct donation to pool custody is either included by a named reserve-sync transition or excluded as surplus; the program cannot silently read a different reserve basis. LP share issue/redemption is a separate profile.

**Loan origination, accrual and payment.** `originate(o,p)` requires signed debtor consent to the terms, a funded `p` asset transfer from creditor to debtor, `principal'=p`, `accrued'=0`, `outstanding'=p`, and a pinned policy. `accrue(o,k)` requires the next period index and computes the exact profile interest with certified role rounding; it cannot replay `k`. `repay(o,n)` requires a same-stage funded transfer of `n` to the creditor and applies `da=min(n,accrued)`, `dp=n−da`, `accrued'=accrued−da`, `principal'=principal−dp`. It closes the obligation only when outstanding is zero. A collateralized extension additionally reads aggregate lock state and a policy-selected current observation. `liquidate` requires unhealthy state, bounded close factor, funded repayment, bounded collateral seizure and a surviving shortfall claim or a separately authorized loss waterfall. A favorable but merely fresh old price is insufficient.

**CDP issuance and redemption.** `draw(v,n)` requires active mode, current issuer grant, head-current global debt ceiling and collateral lock; it increases CDP debt and aggregate debt by `n` and mints exactly `n` synthetic units to the signed owner. The collateral test uses a certified typed price comparison with conservative rounding. `repayBurn(v,n)` burns exactly the units supplied by the payer and reduces debt by `n`; collateral release checks the resulting ratio. A redemption request that cannot settle immediately persists as a claim; burning at request time is allowed only if the profile states who owns the pending backing and how failure returns value. Guarded/shutdown modes and complete claimant snapshots are later transitions.

**Capped cash-settled call.** Instrument identity binds underlying, settlement asset, size, strike `K`, cap `Kc>K`, expiry, fixing rule and policy. A write funds a quote-asset reserve at least the maximum profile payout. Fixing takes exactly the selected authenticated round and writes once. For fixed price `S` at declared scale, payout is

```
pay = floor(size · min(max(S−K,0), Kc−K) / scale).
```

Exercise consumes one claim, transfers `pay` from reserve and leaves any unpaid amount as a duty. Reclaim returns only unreserved collateral. A wrong-round fixing, second exercise or early reclaim rejects. An uncapped call needs a different collateral and loss model.

**Oracle admission.** `admit(o)` is a read/admission rule: verify canonical `ObsId`, feed and typed value/unit, exact bytes commitment, current round-selection rule, policy version, key status, observed time and authenticated stage time. It assigns a label only after those checks. Each named observation is admitted even if its Boolean branch is skipped. A median or TWAP is not implied by a single `Obs<T>`; it needs a finite collection or authenticated accumulator with duplicate, interval and weighting rules.

**Governance amendment.** `queue(q,action)` consumes a digest-bound approval set of distinct eligible keys, stores action bytes, policy head, grant epoch and earliest execution time. `execute(q)` rechecks the same current head and authority, verifies the time window and applies exactly the stored action once. `cancel(q)` competes for the same queue cell and cannot both win at one head. Existing duties remain under their pinned policy unless a stated preservation rule or affected-party consent migrates them. A pause has a declared action mask and cannot silently block owed repayment or recovery.

**Bridge paired claim.** `sourceCommit(x,n)` locks or burns source assets and stores a unique transfer claim. `destReceive(x,m)` verifies the named source-message policy, checks remaining entitlement and an unconsumed destination receipt, credits or mints `m` under the declared representation ratio, and consumes the foreign message identity rather than its proof encoding. `destTimeout(x)` is a one-shot transition after the destination deadline only if no receipt exists; it excludes later receive under the destination program. `sourceRefund(x)` needs authenticated proof of that terminal destination state or an equivalent nonreceipt proof under a named policy. `unknown` remains pending. Each local stage conserves its own domain; a cross-domain backing bound is conditional on the verifier premise and paired-claim invariant.

**Vault deposit, redeem, request and loss.** For one asset and one share class, let `A=managed`, `S=shareSupply`, and let immutable positive virtual values be `vA,vS`. A funded `deposit(a)` mints `sh=floor(a·(S+vS)/(A+vA))>0`; a `redeem(sh)` burns shares and pays `a=floor(sh·(A+vA)/(S+vS))`. `mint` and `withdraw` use the corresponding ceiling direction when later admitted. An unrequested custody transfer changes `surplus`, not `managed`, until an authorized `reconcile` transition recognizes it. That transition itself can create a predictable price jump, so the profile must bind its authority, inclusion timing, beneficiary and any smoothing policy; mere accounting separation is insufficient. Virtual shares also capture a fraction of recognized gain that no holder can redeem, so the bootstrap policy must state where that economic value remains. A withdrawal request locks its shares and stays slashable; finalization reserves a payout under a signed rate policy, and claim consumes it once. Reward accrual is funded and indexed, with checkpoint settlement before every share write. A slash consumes one typed verdict, reduces a bounded allocation and writes down managed assets once. Strategy operations must satisfy a digest-bound mandate and preserve existing exits under amendment.

## 9. Worked discriminator and rejection

Suppose a pool holds `x=1000 A`, `y=2200 B`, with fee multiplier `γ=997/1000`. The signed owner permits at most `11 A` gross and asks at least `20 B` to the owner. A one-hop exact-input candidate transfers 11 A to pool custody and proposes `dy=23 B` to the owner. The quotient of `2200·997·11 / (1000·1000+997·11)` is 23; the complete four balance effects conserve both assets. Subject to the stated width/head/fee policy, this is a **specified positive trace**, not an executed result.

Change only `dy` to 24 with all signatures, inputs and envelope fields still valid. The trader promise remains true, but the exact quote or fee-adjusted pool rule fails. `ProgramValid` rejects the stage, and no pool or owner balance changes. A second hostile trace submits a 23 B effect while the selected program prepared 24 B; exact effect correspondence rejects it. These controls distinguish a semantic check from malformed-envelope rejection. The comparative [Uniswap v2 pair](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol) informs the fee-adjusted form; its contract is not Moriarty evidence.

## 10. Decisions and dissent retained for review

| ID | Proposed direction | Live alternative and required evidence |
| --- | --- | --- |
| D1 arithmetic placement | Reserve a closed Ω transition basis in U0 and certify it at U1, before general Φ₁ authoring. | Keep MIL/2's U4 placement. Measure valid/hostile native witnesses and cost; a consequential owner decision is needed before changing the canonical profile. |
| D2 width | Try a narrow pool/vault operand profile first while keeping `Qty=u128` elsewhere. | Two-limb u128. Prove all intermediate bounds and measure both encodings on the pinned target; the captured comparison limit itself needs reconciliation. |
| D3 pool quote | Require exact divmod output, with the fee-adjusted invariant as a redundant cross-check if affordable. | Invariant plus `dy+1` tightness **under the same fee placement**. Prove equivalence only after fixing the fee formula, nonzero reserves and widths, then compare costs. |
| D4 surplus and fees | Sign surplus owner/completer allocation and explicit fee scope, including retained LP economics. | Owner default or explicit-fee-only. Test the U3 gross/fee/net discriminator under both interpretations. |
| D5 staking conversion | Accounted managed assets, fixed bootstrap, operation-directed rounding. | Custody-based totals or dead shares instead of virtual offset. Run donation/inflation and round-trip traces; choose an offset with asset-decimal analysis. |
| D6 seizure order | Ledger head serialization and origination/consent priority, with replay-keyed evidence. | MIL/2 signed total order. Test two-keeper and correlated-restaking traces against the intended loss bearer. |
| D7 imported evidence | Reject unsupported foreign verification in early profiles; allow a narrowly named trusted premise only when signed and visibly qualified. | MIL/2's broader imported label. Pin an actual verifier and native binding before any bridge or price truth claim. |
| D8 recovery and governance | Keep local release priority, destination-decided bridge receipt, current-head authority and protected remedies. | Alternate pause right, revocation override and rate-freeze policies. Test late result, revoked queued approval, compromised recovery key and withdrawal slash races. |
| D9 first derivative | Capped cash-settled call with bounded payoff. | Underlying-settled call or cash-settled put. Verify payout, fixing round selection, collateral reserve and one-shot exercise. |
| D10 AMM fee placement | Use exact rational fee inside the certified invariant, then count its economic value under the signed fee cap. | Round fee to a whole unit before the invariant. The fifth review's 23 B versus 21 B counterexample shows different outcomes. Decide which party owns that rounding value and measure realistic-decimal losses. |
| D11 vault gain recognition | Require current `reconcile` authority, exact managed/surplus effects and a stated gain beneficiary. | Permissionless recognition with vesting or another smoothing rule. Analyze front-running of the recognition stage and the virtual offset's locked value. |

These are recommendations, not recorded project decisions. Preserve each independent [agent draft](agents/) and the [eight category syntheses](../mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md) when evaluating alternatives.

## 11. Proof and delivery obligations

The original MIL/2 obligations remain open: (O1) total typed evaluation; (O2) evidence source preservation and non-laundering; (O3) aggregate lock safety; (O4) completion/native acceptance refinement; (O5) local terminal safety and conditional recovery; (O6) complete derived footprints and linear fork/join consumption. MIL/3 adds explicit checks for (O7) `ProgramValid` and exact effects; (O8) certified arithmetic and no field wrap; (O9) share/supply/debt separation and funded liability roll-forward; (O10) policy pinning and duty preservation; (O11) replay and paired bridge safety under a named verifier premise; (O12) deterministic rejection/failure phase semantics. None is proved here.

The [roadmap](../../ROADMAP.md) still governs delivery: U0 reconciles this candidate with current source/Core/K contracts, numeric and target matrices; U1 certifies the first required Ω operations and costs; U2 accepts two contrasting newly authored programs with exact proof and ledger effect readback; U3 demonstrates partial progress, recovery and late races; U4 supplies retained native recursive/private history and any claimed foreign verification; U6 qualifies each financial family independently. Existing preview loan/swap results do not automatically validate this language. A family counts as covered only when its own source expectations, K/native behavior, positive and well-formed hostile controls, and actual ledger effects have the required evidence.

## 12. Sources and review boundary

Repository anchors: [MIL/2 design](../../concepts/intent-language/DESIGN-MIL2.md), [proposed semantics](../mil2-deep-research-2026-09-29/MIL2-PROPOSED-SEMANTICS.tex), [research findings](../mil2-deep-research-2026-09-29/RESEARCH-FINDINGS.md), [category coverage and recommendations](../mil2-deep-research-2026-09-29/CATEGORY-COVERAGE.md), [roadmap](../../ROADMAP.md), and [product contract](../../docs/MORIARTY-PRODUCT-CONTRACT.md). The primary capture list with hashes is [sources.json](../mil2-deep-research-2026-09-29/sources.json). Comparative primary references include [SMT-LIB logics](https://smt-lib.org/logics-all.shtml), [ERC-4626](https://eips.ethereum.org/EIPS/eip-4626), [Uniswap v2 pair](https://github.com/Uniswap/v2-core/blob/master/contracts/UniswapV2Pair.sol), and [IBC ICS-004](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md). These sources illustrate external designs; none proves MIL/3 correctness or Midnight feasibility.

No tests, proofs, compiler runs, circuit measurements or Preview transactions were performed for this draft.


MIL/3 WORKFLOW
# MIL/3 decision and implementation workflow

**Date:** 2026-09-29. **Status:** proposed operating plan. **Baseline:** Moriarty `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. This plan uses the existing [U0–U7 roadmap](../../ROADMAP.md), [product contract](../../docs/MORIARTY-PRODUCT-CONTRACT.md), [Moriarty development workflow](../../plugins/moriarty-dev/skills/develop/SKILL.md), and [orchestration stop rules](../../docs/FOOTGUNS.md). It does not create a new approval service, runner, public compile gate or parallel delivery queue.

## 1. Outcome and current boundary

The outcome is a final, versioned MIL/3 contract followed by an implemented, independently audited source-to-Midnight path for its admitted profile. Financial profiles are added only when their own semantics and evidence pass the applicable roadmap gate. A final document is not a proof, and a passing local evaluator is not a native ledger result.

The current [MIL/3 draft](DESIGN-MIL3-DRAFT.md) is a proposal. The [U0 exit record](../u0-semantic-contract-2026-09-23/EXIT-GATE.md) reports incomplete source/Core embeddings, zero covered K rows, zero native-enforced stage leaves and unresolved target pins. The guarded Moriarty status reports an old loan/swap campaign blocked by stale binding and candidate inputs, missing current accounting and unavailable resource state. Those facts block dependent dispatches. They do not block a read-only MIL/3 decision review or an independently scoped source repair.

The first observable capability is **S0 local stage preparation**: an authored program can produce a repay stage and a contrasting literal-fee transfer stage whose complete effects satisfy the signed bounds, or receive a specific semantic rejection. The earlier [U0 unified proposal](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md) defines S0 precisely. Its admitted constructors are literal-fee transfer and funded AccrualFirst repayment of an existing obligation (`da=min(n,accrued)`, `dp=n−da`) with exact 1:1 settlement. Originate, accrue, price, division, Ω/divmod, AMM, vault and other draft §8 families are excluded from S0. The final product capability remains actual pinned ZKIRv3 proof verification and accepted Midnight effects.

## 2. Decision method

Use one versioned candidate and one issue table. Keep all prior drafts, source captures and raw review receipts immutable. An issue row contains: proposition, live alternatives, scope, repository and primary-source anchors, a smallest discriminating example, affected normative clauses, implementation consequence, dependencies, reviewer verdicts, dissent and final disposition. Use IDs W-D0–W-D6 for the table below. The draft's [D1–D11 recommendations](DESIGN-MIL3-DRAFT.md#10-decisions-and-dissent-retained-for-review) remain source labels, not a second live decision register. No draft recommendation becomes a disposition by being copied.

| Live issue | Draft and source labels |
| --- | --- |
| W-D0 first slice | U0 S0; absent from draft §10 |
| W-D1 signature | MIL/2 §11; U0 in-circuit proposal |
| W-D2 language contract | Draft §3 |
| W-D3 stage and failure | Draft §4 and proof obligations O7/O12 |
| W-D4 numeric and economics | Draft D1, D2, D3, D4 and D10 |
| W-D5 evidence and duties | Draft D6, D7 and D8 |
| W-D6 category admission | Draft D5, D9 and D11; eight draft §8 families |

| Order | Issue to settle | Evidence before a decision | Immediate scope |
| --- | --- | --- | --- |
| W-D0 | **First admitted slice.** Keep U0 S0 repay plus literal-fee transfer, or replace it with an evidenced alternative. | Compare required primitives, source/Core path and U1/U2 target cost. An exact-input AMM or vault needs certified division and additional state. | Propose S0 for I0. A transfer-only implementation is a named sub-slice, not completion of S0. |
| W-D1 | **Signature binding.** State the signature scheme, signed bytes, digest, verifier locus, stage public inputs and ledger binding. | Reconcile [MIL/2 §11](../../concepts/intent-language/DESIGN-MIL2.md) with the U0 study's different in-circuit proposal. IR curve instructions do not by themselves verify a wallet signature. Require a valid and a substituted-intent negative control. | Blocks native signed acceptance, not local reference preparation. |
| W-D2 | **Canonical language contract.** State grammar, sorts, encoding, domain tags, versions, unknown-tag rejection, caps and `/2`→`/3` migration. | Derive source/Core round trips and byte-level ambiguity cases. Measure caps before fixing their values. | Blocks digest freeze and U2 native statement identity. |
| W-D3 | **Stage and failure relation.** Select the exact `ProgramValid`, effect, footprint, head, authority, history and retained-phase rules. | Give positive and well-formed hostile traces for each of the six U0 judgments. Reconcile the existing source/Core/K rules. | Blocks reference and native implementation claims. |
| W-D4 | **Numeric and economic rules.** Set S0 checked `u128` arithmetic first, then decide Ω placement, possible later narrow widths, AMM fee placement, fee cap, quote tightness and surplus, vault bootstrap and gain recognition. | Use integer counterexamples and target cost measurements. The draft's 23 B versus 21 B swap is a decision discriminator, not an equivalence proof. | S0 can proceed without pool division. Later profiles wait for U1 certificates. |
| W-D5 | **Evidence, persistent duties and cross-domain claims.** Define observation admission, grant/current-head checks, policy pinning, one-shot receipt, partial bridge installments and refund proof. | Model late receipt, timeout, replay, revoked authority, partial delivery and retained duties. Name each external verifier premise. | Local U3 rules can be specified before a foreign verifier, but cannot claim a complete bridge. |
| W-D6 | **Category admission.** Give each of the eight DeFi families a profile ID, constructors, state cells, numeric operations, trust premises, first positive/hostile pair and owner milestone. | Compare the eight reviews to the actual supported grammar and effect path. A family remains specified-only until its own evidence is present. | No broad U6 coverage claim from a common core alone. |

Resolve issues in dependency order. A reviewer can recommend a choice but cannot close an empirical question with prose. An experiment can reject a choice. It does not by itself select a policy beneficiary or trust premise. **W-D0–W-D3 and the S0 part of W-D4 block the first admitted profile.** W-D1 must specify the exact authentication contract even if native enforcement remains open. The AMM/vault part of W-D4, cross-domain part of W-D5 and every later category in W-D6 may be explicitly deferred with a profile owner and rejection rule. They do not block S0's local reference work. No later profile is silently admitted by that deferral. Approval of this workflow disposes no W-D0–W-D6 row and is not M3-D.

The numeric issue must explicitly reconcile [the recorded U0 dust decision](../../docs/decisions/u0-numeric-profile-decision.md) with MIL/3's distinction between fractional remainders and whole-unit residue. Do not silently reinterpret a hash-bound numeric profile. Record any change as a new version and review its effect on the admitted slice.

### Multiagent decision round

1. The lead freezes the exact candidate bytes and issue row. The lead names one expected positive trace and one well-formed hostile trace before implementation.
2. A formal semantics agent checks formation, total evaluation, refinement, program transition and failure paths. A financial agent checks units, conservation, fees, rounding, obligations and loss. An evidence agent checks authentication, provenance, authority, replay, history and recovery. These preparation agents provide independent objections. They do not vote on the final candidate.
3. Two fresh, independent reviewers receive the **same full candidate**, issue dispositions and source evidence. The current repository routing calls for **Grok 4.6 at high effort** and **GPT-6 Astra**. Record requested and returned model identity, effort, terminal state, exact candidate digest, verdict and dissent. An absent or failed reviewer supplies no vote. Do not substitute a prior Opus recommendation for a current audit.
4. The lead evaluates the reviews against the evidence. A consequential decision needs two substantive agreeing votes under the repo's majority rule. A disagreement causes a narrower discriminator or a revised candidate, followed by fresh review of changed bytes. The lead records the selected rule and dissent in the existing decision material.
5. A separate full-candidate audit checks internal consistency after issue-level decisions. A changed normative byte invalidates the prior full-candidate audit. Do not treat an author review as independent.

For this workflow, this checkout's `AGENTS.md` governs I1: GPT-6 implements; fresh Grok 4.6 high and GPT-6 Astra audit the same full candidate, independently and concurrently. Record Astra's actual effort; `AGENTS.md` does not set it. The checked-in development skill contains a conflicting September 10 general-product route. Record that conflict in the I0 decision record before I1; the current checkout instruction and [September 9 routing record](../../raw/assignments/moriarty-grok-review-routing-2026-09-09.md) select the route above. A changed instruction requires a new route decision. Do not silently choose a convenient model.

## 3. Three gates, each with a distinct claim

| Gate | Exact claim allowed after exit | Required evidence | Current status |
| --- | --- | --- | --- |
| **M3-D: design selected** | “MIL/3 has a reviewed contract for its admitted S0 profile.” | All S0-blocking issue rows disposed; complete S0 grammar, encoding, signature contract, numeric policy, stage/failure relation, trust premises and migration; full-candidate independent audits and required decision votes. Every deferred construct has an owner and rejection rule. Any S0 subset freeze uses U0-F and leaves the September 23 U0 exit counts unchanged; M3-D does not close, replace or re-score that exit. | Open. |
| **M3-R: reference implemented** | “The named MIL/3 profile works through the actual local source/Core consumer and agrees with its K reference on the defined corpus.” | Feasible positive and hostile inputs through a callable path; complete effects and rejection state; all six judgments over the profile; TypeScript/K differential; repair and fresh audits of frozen implementation bytes. | Open. |
| **M3-N: native accepted** | “The named MIL/3 profile is proof-carrying on Midnight under the pinned tuple.” | Joint compiler/ZKIRv3/verifier/key/ledger pins; U1 native certificate for every primitive used; bound signed intent/program/predecessor/effects; valid and hostile witnesses; actual proof verification and ledger effect readback; independent result audit. | Open. |

`M3-D` selects and freezes **review candidate bytes** for S0. It can support a scoped U0-F semantic contract after its own exit is met. It is not U0-S, U0-T, a public-profile finalization or a claim that MIL/2's pre-freeze proof obligations passed. Any hash-bound public profile is held until the required obligations for its admitted subset have reference evidence and the target binding is demonstrated. The [U0 study](../u0-study-2026-09-28/UNIFIED-PROPOSAL.md) separates contract freeze, reference demonstration and target qualification as U0-F, U0-S and U0-T. Every result record must use one of those labels. U1/U2 and later native acceptance remain separate.

## 4. Implementation sequence

Keep one delivery implementation item active. Other agents can prepare independent expectations or audit it. Use an isolated worktree and keep the current dirty workspace intact. **I1 is a delivery implementation item and remains blocked while SP01.6 occupies that slot.** Before I1, record the active slot and use an existing guarded transition if a registered campaign change is needed. If no such transition frees the slot, I1 stays blocked; do not invent a `cli.py run` action or start a second active item. Preserve SP01.6's blocked history. Use the guarded Moriarty CLI for any **registered** campaign action. `AGENTS.md` and the development skill allow routine authorized source edits without a new campaign; guarded failure blocks its dependent dispatch, not all independent source repair. That routine repair exception does not cover I1 delivery edits, turn an edit into accepted implementation, bypass an existing registered action, or close SP01.6.

| Step | Deliverable | Observable exit and audit |
| --- | --- | --- |
| I0 | Dispose S0-blocking decisions and freeze the **review candidate bytes** for the S0 source/Core contract. Specify the signature boundary, including signed bytes and verifier locus; leave native enforcement as an explicit open obligation. | The S0 issue rows, versioned grammar, stage/failure relation and exact positive/hostile examples agree. The source/Core embedding map names every admitted leaf and every fixed-empty field. Two fresh full-candidate design audits finish. This is M3-D, not U0-S/U0-T or public-profile finalization. |
| I1 | After I0 and the one-active-item check, have GPT-6 integrate **actual S0 stage preparation** with the existing source/5 selected-action evaluator and Core financial lifecycle. Start with the transfer sub-slice, then funded repay. Use named fixture snapshots and pre-states from the frozen S0 examples; a repay fixture starts with an existing obligation and does not admit originate or accrue. Live resource state and SP01.6 accounting remain unavailable. | A public callable path produces a local prepared-stage candidate from named fixture pre-state. It rejects wrong recipient, excess fee/gross debit, missing funding, excess repayment, inconsistent debt components, replay and incomplete transfer effects. Mark each draft `Stage` conjunct implemented or open. Transfer-only is reported as a sub-slice. `HistoryLink`, live-head authentication, K, native and ledger acceptance remain open on this path until separately evidenced. |
| I2 | Reconcile the reference relation with K. | The same named fixture corpus has matching acceptance, complete post-state and error classification in TypeScript and K. A hostile case must reach the semantic rule with a valid envelope. Every `Stage` conjunct and all six U0 judgment keys are implemented or explicitly open. M3-R stays open until both transfer and repay share the relation and match K. |
| I3 | Qualify the pinned target and certify the S0 primitive basis. | The target record is one compatible tuple, not historical pins combined by name. The first checked-add-u128 and any other required primitive have valid and hostile native certificates with caller premises and measured cost. Official [Compact releases](https://github.com/midnightntwrk/compact/releases) and [Midnight ledger](https://github.com/midnightntwrk/midnight-ledger) are moving primary sources; re-pin exact artifacts before using them. |
| I4 | Connect source/Core stage, signed intent and actual effects to the native verifier and Midnight ledger. | Newly authored repay and literal-fee transfer programs compile to the selected ZKIRv3 target; native proof and ledger acceptance bind exact statement bytes, current state, program, signer, gross/net/fees, replay and complete effects. Read back balances and liabilities. |
| I5 | Add U3 conditional settlement, U4 history/private composition and U6 financial profiles in dependency order. | Each family gets its own profile, positive and hostile native controls, persistent-duty and trust evidence. U5 federation remains optional for direct use. U7 independent developer release follows the roadmap. |

The existing older `lower-compact.ts` route and pure generated loan/swap kernels are useful baselines. They do not by themselves prove that source/5 lowers to the current native stage relation. An implementation must connect the actual consumer and read back effects, rather than add an unused wrapper. If a required native interface is unavailable, keep I3/I4 open and continue eligible local I1/I2 work.

The proposed I1 file boundary is `experiments/moriarty-language/src/successor/mil3-stage-v1.ts`, called by the existing `src/cli.ts` check/simulate route. It must invoke `createFinancialAgreementSourceV5().evaluate` in `financial-agreement-source-v5.ts`, which already reaches the shared Core/4 evaluator and lifecycle preparation. It must bind source, selected action, Core identity and named fixture snapshot/pre-state. Caller-proposed effects and post-state must equal the prepared result. Transfer footprints include both balances, sender allowance, transfer replay state, work counters and the dependencies of ordinary `ReadPre`, `NextWrite` and `Ensure` operations. A two-balance-only footprint is incomplete. This is an implementation proposal, not an existing MIL/3 API or a claim about live state.

### Implementation audit loop

Before an author edits, an independent agent defines expected outputs and adversarial cases from the contract. The selected implementer works only on the current step. The lead inspects the public entry point through the effect consumer, runs the authorized checks for that step, and saves complete output. Fresh Grok 4.6 high and GPT-6 Astra auditors inspect the same frozen full candidate. They check semantics, financial accounting, security/authority, target binding and actual result evidence. A code review does not replace a result audit. If the implementation changes, rerun the affected checks and audits against new bytes.

The audit must answer five questions for each accepted path:

1. Did the signer authorize these exact bytes, assets, recipients, fees and disclosure?
2. Did the selected program prepare the exact effects and post-state that the ledger accepted?
3. Did every balance, supply, lock and liability change follow its typed resource rule?
4. Did the stage extend an authenticated predecessor without replay or duty erasure?
5. Did a well-formed hostile witness fail at the intended semantic or native rule, rather than at an unrelated envelope check?

Report local, K, native-proof and Preview results in separate rows. Record failed or unavailable checks as open. Do not infer full product support from an S0 success.

## 5. Stop and recovery rules

- After two failed cycles of one defect class, reproduce one defect through the callable path. Change the implementation approach before another broad correction.
- After two process-only cycles or thirty minutes of administration, stop making packets. Move to executable diagnosis or an eligible implementation step.
- If a reviewer or target interface is unavailable, preserve the missing identity or interface as a blocker. Continue independent work that does not depend on it. Do not substitute another model or a host Boolean.
- If a candidate exceeds a recorded resource limit, stop that run. Seek a reviewed encoding or bounded resource amendment. Do not use a larger unreviewed budget to force a pass.
- Preserve the old loan/swap campaign history. Run the guarded `next`/`run` path only for its registered actions. Do not claim its old candidate as MIL/3 evidence until its stale inputs and accounting are reconciled.
- Preserve failed traces and dissent. A clean rewrite never deletes contrary evidence.

## 6. First decision packet and next action

The next decision packet covers **W-D0–W-D3 and the S0 part of W-D4**, not an AMM implementation packet. It proposes literal-fee transfer and funded AccrualFirst repayment of an existing principal/accrued obligation as the only admitted constructors. Originate, accrue, price, division, Ω/divmod, AMM, vault and the other draft §8 families stay out. It resolves the MIL/2/U0 signature-contract conflict at the specification level, while native enforcement stays open until demonstrated. It specifies `/3` canonical bytes, unknown-tag rejection and the S0 numeric policy: checked add/subtract, comparison and min on `u128` quantities, with no division. Later pool/vault narrow widths and Ω/divmod remain outside S0. It quotes and reconciles the [U0 numeric decision](../../docs/decisions/u0-numeric-profile-decision.md), whose whole-unit dust goes to the protocol reserve; a fractional quotient remainder is not a ledger unit. Any change to a hash-bound profile requires a new versioned decision. The lead requests two fresh full-candidate audits after these clauses are written. Only two agreeing substantive decision votes plus the frozen S0 contract make I0 complete. Then I1 can start under the user-directed capability after the one-active-item check and guarded admission for any registered action. Agreement on this workflow document disposes no W-D row and does not start I1. If the reviewers disagree, run the smallest byte-substitution or caller-path discriminator and revise the affected clause.

The exact next technical capability is: invoke an authored source/5 transfer through a callable local path and obtain a stage record whose prepared effects, signed recipient and literal fee match, while a one-unit extra fee or redirected recipient is rejected. Then add funded AccrualFirst repayment under the same relation. Neither result is a native-proof or Preview acceptance claim.

No test, proof, compiler or Preview transaction was run to write this workflow. The primary target references above support the need to pin current toolchain artifacts; they do not establish that a compatible tuple is available.


U0 NUMERIC DECISION
# U0 numeric profile decision

Status: decided 2026-09-23 by the project owner for roadmap milestone U0. This file is the only source of policy values for `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json`; the profile may refine these decisions into per-primitive rows but may not change them.

## D1 — Canonical price orientation

Moriarty's canonical price orientation is **base-per-quote**, the existing `moriarty-financial-agreement-source/5` convention. A price `Price<A,B,S>` states how many units of the base asset one unit of the quote asset is worth, at the declared scale.

DeFiFormal and other quote-per-base sources are adapted only through an explicit, dimensioned conversion with directed rounding at the boundary. A reciprocal, rename or implicit reinterpretation is not a conversion.

## D2 — Rounding direction and beneficiary policy

The default policy is **protocol-favoring**:

- Any computed amount a party owes, or that increases or preserves a liability (interest accrual, repayment due, fees, collateral requirements, debit caps checked against a computed charge), rounds **up** (`ceil`).
- Any computed amount a party receives, or that reduces a liability in the payer's favour (payouts, withdrawals, conversions into a received amount, refunds, liquidation proceeds to the borrower), rounds **down** (`floor`).
- Exact results use no rounding (`none`) and must remain exact; a primitive that can lose precision may not use `none`.
- The rounding remainder (dust) accrues to the **protocol reserve**. It is never silently dropped, burned or credited to the prover or solver.

Per-primitive overrides are allowed only when declared explicitly in the profile with a rationale. Later libraries (for example ACTUS in U6) must declare their overrides the same way; none are declared in U0.

Where the current source/5 implementation lets a program author select `none`, `floor` or `ceil` independently of this policy, the profile records that primitive's required direction and marks the author-selectable behaviour as an open conformance gap. U0 does not change the implementation.

## D3 — Scope of the U0 primitive slice

The U0 numeric profile covers exactly the arithmetic primitives the current successor implementation (`experiments/moriarty-language/src/successor/`) actually implements. AMM, vault, ACTUS and other library primitives are U6 scope and are not listed.

## D4 — Units

Amounts are exact domain-qualified integers in each asset's smallest unit, with checked finite-width arithmetic. They never become field elements modulo the proof field. Each asset's decimals and domain are part of its identity.


U0 EXIT
# U0 exit gate — 2026-09-23

**U0 contract recorded; capabilities open**

| Item | Evidence file(s) | Evidence RECORDED | Capability status | Counts | Open reason |
| --- | --- | --- | --- | --- | --- |
| Versioned source/Core embeddings | `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json`, `openspec/changes/consolidated-language-kernel/schemas/stage-relation.schema.json` | yes | OPEN | 1 present, 17 partial, 66 absent | Source/Core leaf embeddings are incomplete |
| Stage/intent/effect/authority/history/failure judgments | `deliverables/u0-semantic-contract-2026-09-23/judgments.json`, `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json` | yes | OPEN | 6 judgments; 17 partial, 66 absent embeddings | Judgments are recorded; source/Core realization remains partial |
| Numeric profile | `deliverables/u0-semantic-contract-2026-09-23/numeric-profile.json` | yes | OPEN | 17 primitives, 6 open gaps; reserve absent | Primitive conformance or reserve posting remains open |
| K-reference reconciliation | `deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json` | yes | OPEN | 0 covered, 5 partial, 18 not covered | K reference coverage remains incomplete |
| Compiler/ZKIRv3/verifier/key/ledger pins | `deliverables/u0-semantic-contract-2026-09-23/target-pins.json` | yes | OPEN | 11 historical, 10 unresolved | Historical pins are not reverified; compatible tuple or source alignment is open |
| Field-by-field enforcement map | `deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json` | yes | OPEN | 0 enforced, 84 unenforced | Native enforcement is not established for every leaf |
| Trust and unresolved interface premises | `deliverables/u0-semantic-contract-2026-09-23/trust-premises.json` | yes | OPEN | 9 premises, 6 open | Unresolved trust or interface premises remain |
| Next-backend requirement matrix | `deliverables/u0-semantic-contract-2026-09-23/backend-requirement-matrix.json` | yes | OPEN | 24 rows, 24 specified-only | Backend requirements are specified, not demonstrated |

## Checker runs

| Command | Exit code | OK line |
| --- | --- | --- |
| `python3 scripts/check_u0_stage_schema.py --root .` | 0 | OK: 84 leaf fields, 1 present, 17 partial, 66 absent |
| `python3 scripts/check_u0_numeric_profile.py --root .` | 0 | OK: 17 primitives, 6 open conformance gaps |
| `python3 scripts/check_u0_k_reconciliation.py --root .` | 0 | OK: 23 rows, 0 covered, 5 partial, 18 not-covered |
| `python3 scripts/check_u0_target_pins.py --root .` | 0 | OK: 11 historical, 0 absent, compatible tuple NOT established; absent means no pin found by this recorded search; not a proof that none exists |
| `python3 scripts/check_u0_enforcement_map.py --root .` | 0 | OK: 84 leaf fields, 0 enforced, 0 host-only, 84 NOT_ENFORCED |
| `python3 scripts/check_u0_trust_backend_matrix.py --root .` | 0 | OK: 24 backend rows specified-only, 9 trust premises |
| `python3 scripts/build_u0_backend_matrix.py --root . --check` | 0 | OK: backend matrix matches source |

## Limitations

- `deliverables/u0-semantic-contract-2026-09-23/source-core-embeddings.json`: “absence means not found by this recorded search; it is not a proof of non-realisation. Semantic adequacy of a cited declaration is a reviewed claim; the checker proves declaration, context, and profile membership only.”
- `deliverables/u0-semantic-contract-2026-09-23/k-reconciliation.json`: “Coverage is a reviewed claim; the checker proves citations and evidence quotes only.”
- `deliverables/u0-semantic-contract-2026-09-23/target-pins.json`: “absent means no pin found by this recorded search; not a proof that none exists”
- `deliverables/u0-semantic-contract-2026-09-23/enforcement-map.json`: “Whether a cited line truly enforces the field is a reviewed claim. Mechanisms cite no line. Signed-intent authentication was not found in a circuit, a bound ledger primitive, or another native boundary under the declared native roots. experiments/moriarty-language/compact is included. Its arithmetic helpers and generated loan and swap kernels constrain anonymous Uint fields of two restricted Core programs. They do not name a canonical stage-relation leaf. Copies under deliverables/preview-loan-2026-09-17 are snapshots, not native roots.”
- `deliverables/u0-semantic-contract-2026-09-23/trust-premises.json`: “Semantic fit of a quote to its statement label is a reviewed claim, not mechanically proven. The checker verifies structure, literal quote occurrence, identifier resolution, required topics, label length, one sentence without ', and' or a semicolon, and closure-word absence.”


EIGHT CATEGORY BASELINE
# MIL/2 against the eight DeFi categories

**Date:** 2026-09-29. **Baseline:** `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. **Status:** eight independent source reviews, one per category. This evaluates the [MIL/2 specified-only design](../../concepts/intent-language/DESIGN-MIL2.md), not an implemented language or a Midnight product. The older [MIL/1 category map](../../concepts/intent-language/CATEGORY-MAP.md) supplied the taxonomy but its expressibility verdicts were re-derived, as the [execution summary](../../concepts/intent-language/EXECUTION-SUMMARY.md) requires. Each linked report gives realistic cases, exact repository anchors, need-by-need verdicts, and the missing proof or native check.

## Coverage matrix

| Category and review | How MIL/2 addresses its central needs | Present design boundary |
| --- | --- | --- |
| [1. AMMs and exchanges](category-review/01-amm-exchange.md) | The `AcquireB` intent can bind input, output, fees, recipient, deadline and bounded route completion. Typed `pre/post` cells, conservation, shares and a reserved multi-signer arity provide places for pool and exchange rules. | **Partial for AMMs; open for general exchanges.** Live reserve invariant, two-asset pool transition, LP accounting, fee math and exact-output rounding are missing or require deferred Φ₁ under the current design. Route and venue semantics are incomplete. Shared-write n-party clearing is unspecified and deferred beyond U4. Five [independent Opus 5.5 recommendations](opus55-amm-recommendations/SYNTHESIS.md) propose a narrower certified pool operation before general Φ₁; that is not an adopted design change. |
| [2. Lending and borrowing](category-review/02-lending.md) | `Obligation`, debtor consent, `Encumbrance`, scoped enforcement, typed price evidence and persistent episode links can frame a bounded fixed-rate collateralized loan. | **Partial.** Lock lifecycle and aggregate reservation proof, payment-funded roll-forward, liquidation economics, default and loss waterfall need rules. Variable/index rates and pooled shares need deferred arithmetic. Flash liquidity is an explicit product cut. Five [Opus 5.5 recommendations](opus55-lending-recommendations/SYNTHESIS.md) give candidate transition and evidence rules. |
| [3. Stablecoins and synthetics](category-review/03-stablecoins.md) | Nominal asset identity, domain/asset-scoped `issue`, supply deltas, conservation, debt, collateral and guarded redemption give issuance a coherent set of carriers. | **Partial.** Typed mint/burn and CDP transitions, reserve and debt linkage, redemption settlement and impairment waterfall are missing. System-wide shutdown, rebasing, reflexive backing and shared debt-pool hedges are open. Five [Opus 5.5 recommendations](opus55-stablecoins-recommendations/SYNTHESIS.md) propose candidate rules. |
| [4. Derivatives](category-review/04-derivatives.md) | Instrument-indexed signed positions, clock-qualified windows, collateral, observation provenance, authority and conditional stages can support a narrowly specified fully collateralized option. | **Partial.** No position lifecycle, payoff/fixing rule, funding ratchet, margin valuation, liquidation, bankruptcy waterfall or multilateral clearing is defined. A perpetual is not covered by the position type alone. Five [Opus 5.5 recommendations](opus55-derivatives-recommendations/SYNTHESIS.md) propose a first cash-settled option slice. |
| [5. Oracles and observations](category-review/05-oracles.md) | Typed observations, source-set effects, freshness/finality predicates and declared reads state what a stage should depend on and which source class is allowed. | **Partial.** Observation identity/value/unit and authenticated-read binding must reach the actual stage and native statement. Bounded observation collections, TWAP, median aggregation and fallback are open. A provenance label does not prove the reported value. Five [Opus 5.5 recommendations](opus55-oracles-recommendations/SYNTHESIS.md) propose verification and aggregation rules. |
| [6. Governance](category-review/06-governance.md) | Scoped, expiring, revocable rights and policy/allowance/replay cells can constrain issue, enforce, amend and recover actions; time and threshold guards can be attached to a stage. | **Partial.** Policy migration, administrative state transitions, pause/recovery interaction, timelock queue/veto, vote snapshots and verified multi-signer authority are not specified. Existing signed duties need protection across policy changes. Five [Opus 5.5 recommendations](opus55-governance-recommendations/SYNTHESIS.md) propose authority and migration rules. |
| [7. Bridges and cross-domain settlement](category-review/07-bridges.md) | One-domain stages, nominal canonical/wrapped identities, typed imported evidence and pending escrow make the local side of an asynchronous bridge intelligible. | **Partial locally; open as a complete bridge.** No unique paired source/destination claim, transfer-form rule, verifier policy, cross-domain backing invariant, bonded fast-fill economics or reorg response is defined. A timeout does not prove foreign nonreceipt. Five [Opus 5.5 recommendations](opus55-bridges-recommendations/SYNTHESIS.md) propose paired claim and verifier rules. |
| [8. Staking, restaking and yield](category-review/08-staking-yield.md) | Pool/class shares, both conversion directions, encumbrances against several obligations, scoped slash authority and pending episodes can carry vault, unbonding and restaking contracts. | **Partial.** Share conversion is Φ₁; method-specific rounding, mint/burn, rewards, validator and withdrawal-queue lifecycle, slash waterfall and strategy mandate are missing. The aggregate lock invariant remains unproved. Five [Opus 5.5 recommendations](opus55-staking_yield-recommendations/SYNTHESIS.md) propose candidate vault and lifecycle rules. |

Here **partial** means the design gives some typed or relational carriers but lacks a necessary category rule. It does not mean the current parser, compiler, K model, ZKIR circuit, or ledger enforces that family. The strongest narrow proposed cases are the bounded trader outcome and a single-domain collateralized fixed-rate loan; neither has MIL/2 native acceptance evidence. All eight reports keep the six [pre-freeze obligations](../../concepts/intent-language/DESIGN-MIL2.md) open.

## What the design covers across categories

1. **Signed outcome and authority boundaries.** A user can propose bounded debit, fees, recipient, time, evidence class and recovery policy, while a solver fills constrained holes. `issue`, `enforce`, `amend` and `recover` are distinct scoped rights. This is useful for swaps, loans, issuance, governance and strategy actions. Completion refinement, grant/revocation semantics and native binding still need to show that a submitted candidate cannot widen the signed promise.
2. **Typed financial resources.** Nominal assets, unsigned balances, signed deltas, instrument positions, pool/class shares, debts, locks and linear receipts avoid several MIL/1 conflations. They let each family name its state and authority. They do not supply family-specific transitions such as a pool swap, liquidation, vault redemption, derivative payoff or slash allocation.
3. **Stage-local evidence and effects.** `pre/post` cells, source-set effects and derived footprints can state which authenticated inputs a stage reads and which balance, supply, liability and policy cells it writes. Total Φ evaluation, non-laundering and footprint containment are still proof obligations. The [separate semantics draft](MIL2-PROPOSED-SEMANTICS.pdf) also finds an incomplete write footprint in the MIL/2 showcase.
4. **Asynchronous duties.** Ledger-linked episodes, a legitimate pending escrow state, signed late-race priority and a terminal tombstone give conditional workflows a plausible local safety structure. Cross-domain progress and refund require named evidence and liveness assumptions; no global rollback follows from those local rules.

## The shared design work that unlocks the most categories

| Priority | Required semantic artifact | Categories directly affected |
| --- | --- | --- |
| 1 | Correct Φ₀ guard/checker class; specify typed total evaluation, numeric widths and every rejection. Measure and later admit Φ₁ share and nonlinear pool math under a separate profile. | AMMs, lending, stablecoins, derivatives, staking/yield |
| 2 | Specify exact resource/effect constructors and derive complete footprints, including all transfer balance cells, supply, shares, receipts, locks and policy state. | All eight |
| 3 | Bind observation ID, authenticated value, unit, source domain, policy, time and status through source/Core/native acceptance; define bounded collections if aggregation is claimed. | Lending, stablecoins, derivatives, oracles, bridges, staking/yield |
| 4 | Define committed and reserved lock transitions, debt roll-forward, complete effect accounting and category-specific loss/rounding beneficiaries. | Lending, stablecoins, derivatives, AMMs, staking/yield |
| 5 | Give unfilled intents a trace denotation and prove each completion refines the signed template; define one local recovery relation and a separate paired cross-domain claim relation. | AMMs, bridges, lending, stablecoins, governance |
| 6 | Define small, reusable category transitions: pool swap/share issue, fixed-rate loan/liquidation, mint/burn/CDP, cash-settled option, observation acceptance, policy amendment, bridge claim and vault request/finalize/claim. | One concrete reviewable slice per category |

These are recommendations inferred from the eight reviews. They are not adopted MIL/2 rules or completed roadmap milestones. The [research sprint plan](SPRINT-PLAN.md) addresses the first five shared obligations. The [roadmap](../../ROADMAP.md) requires U0 semantic integration, U1 certified native primitives, U2 a general stage path, U3 conditional settlement, and U6 retained DeFi conformance. Each category needs its own executable source/Core/K/native/ledger evidence and adversarial controls before a product coverage claim.

## Review limits

The eight agents reviewed repository design and prior requirement inventories. They did not execute tests, proofs, compilation, or Preview transactions. The [primary-source corpus](sources.json) and [Graphify map](graph-corpus/graphify-out/GRAPH_REPORT.md) support the separate deeper research sprint; graph links are inferred leads, not proofs of category coverage. Individual reports distinguish repository facts from proposed examples and inferences.

