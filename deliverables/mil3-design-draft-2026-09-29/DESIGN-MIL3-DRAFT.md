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
