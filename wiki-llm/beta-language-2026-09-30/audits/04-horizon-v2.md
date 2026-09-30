# Audit 04 follow-up: typed financial horizon v2

**Verdict:** concur that the original H1/H2/M1/M2 presentation defects are substantially repaired. Reject unconditional acceptance of the new financial proposal until the two financial relations below are reconciled. This is document review, not beta implementation, grammar-freeze, usability, proof or ledger approval.

## Exact inputs and review boundary

Requested routing remains GPT-6.1 Sol, medium. This context supplies no machine-verifiable returned model/effort receipt; actual routing conformance remains unverified. No peer outputs were read. The original audit remains unchanged.

Verified exact SHA-256 inputs:

| Artifact | SHA-256 |
| --- | --- |
| PROGRAMMER-MOCKUP.md | `89ab5354e9ff0de7d0793318a8c9df93d617b71bfac13da120289402dc61dc65` |
| FULL-LANGUAGE-HORIZON.md | `7d2673703dfd271dd641b625c799dee3f536803827dacb7d66bc938cc82ec26f` |

The checked-in develop skill remains loaded. Guarded status was refreshed: dependent SP01.6 dispatch is blocked by stale inputs, missing accounting and unresolved operational history; no pending transactions. This authorized document review makes no dispatch claim. Scope is the revised mockup and horizon, compared with my own original findings and previously inspected requirements/Source6/Core5 baseline. Beta code now existing elsewhere does not establish any capability in this document-only audit. The separate six developer trials were not performed or assumed here.

## Disposition of the original findings

| Original finding | Disposition | Substantive evidence in new bytes |
| --- | --- | --- |
| H1: prose replacing typed financial interfaces | Repaired as a proposed interface | Horizon §§1–3 introduces nominal resources, dimensional prices/clocks, typed cells and footprints, verified evidence, finite holes/completion, explicit authority and kernel calls. §§5–12 provide typed transitions with pre/post, derived effects, failures and duties. No string-evaluated relation or generic host call is adopted. Grammar/checker/certification remain explicitly open. |
| H2: composed episode lacks a complete signed promise | Repaired structurally | §13 supplies one SignedIntent, program/policy pins, domain accounts, per-asset cumulative gross/fee bounds, holes, grants, paired economic identity and separate attempts. It gives recipient/fee/attempt/unknown-finality discriminators and three distinct terminal alternatives. DestinationReturn retains backing obligations; OriginRefund requires future-delivery exclusion. Net-floor selection needs the additional rule in F2 below. |
| M1: required family lifecycles missing from source | Repaired as visible proposed source | Lending now has origination, roll, funded repayment and residual-default liquidation; stablecoin has freeze/emergency payment; option has funding/fixing/exercise/settlement; staking has deposit/reward/unbond/slash/withdraw; governance adds veto. §§14–15 provide lifecycle/surface coverage and explicit proof obligations. Visibility of a transition is not evidence that its financial relation is consistent, implemented or proved. |
| M2: proposed surface conflated with existing local implementation | Repaired | Mockup distinguishes existing Source6/Core5 local preparation, proposed beta records and proposed horizon grammar. Tables explicitly label authoring support specified and require independent command results. The horizon header expressly says beta does not accept its grammar. These scoped statements do not claim beta code is absent; they simply establish no command evidence in this artifact. |
| M3: S0 numeric field bound omitted | Repaired | Mockup states UInt128 pure expressions/storage and S127 narrowing for S0 nominal fields; horizon provides narrow127 and H_NOMINAL_RANGE. The distinction is now explicit. |

The wider notation is now a useful language proposal rather than a collection of English-valued records. It preserves the eight-family financial horizon without imposing implementation of that horizon on the small beta. Nominal resource kinds, typed evidence interfaces and separate derived footprints remain appropriate choices. The proposed horizon's exhaustive match and bounded-library mechanisms still need their own grammar/type/totality specification; their open status is correctly disclosed.

## F1 — High: slash loss waterfall contradicts the worked withdrawal amount

**Repository observation:** horizon lines 1199–1203 specify that Slash losses first reduce subordinate withdrawal claims, then junior free claims according to rank. YieldPromise signs priority `[SeniorRestakingSlash,PendingWithdrawal,JuniorFreeShares]`. Lines 1309–1315 instead calculate a uniform proportional haircut: from backing1210/supply1100, an unbonded100-share claim110 becomes99 after slash121. That distributes11 loss to the pending claim and110 to free claims. The prose expressly calls this proportional loss.

**Why it matters:** a ranked first-loss waterfall and a proportional haircut authorize different beneficiary liabilities. Typed constructor names and an arithmetic certificate do not select which rule the owner signed. Withdraw's generic instruction to recalculate post-slash claim cannot repair this ambiguity; it must consume the exact certified rank-allocation result.

**Exact discriminator:** start with backing1210, supply1100, one pending100-share withdrawal owed110, and junior free claims worth1100; accept a121 slash with no prior losses. Under the stated first-loss order, the pending claim loses110 and becomes0, then free claims lose11. Under the worked proportional rule, pending loses11 and remains99, while free claims lose110. The specification must authorize exactly one result and reject the other's certificate/post-state. Run the same check at maturity before accepted withdrawal; maturity alone must not change the signed priority.

**Repair:** choose one explicit allocation relation. If proportional loss is intended, say which claims form the same risk tier, apply one ratio to that tier with explicit rounding/dust beneficiary, and state how senior tiers or competing restaking locks alter that ratio. Replace the contradictory first-loss comment and rank list with that policy. If the waterfall is intended, update the example, withdrawal valuation and priority-dependent share accounting to pay0 for this claim rather than deriving99 from a common backing/supply ratio. Specify the complete liability/claim effects and residual duties under either rule.

## F2 — Medium: terminal net floors need explicit branch and measurement scope

**Repository observation:** §2 lines 200–209 requires all source terms and per-asset bounds, defines net receipt after every deduction, and checks the selected terminal branch's N against net. Bounds themselves contain one untagged net field per subject/asset. The composition sensibly explains that USD origin refund, WUSD destination return and GOLD_F delivery have distinct floors, and expressly says DestinationReturn does not satisfy Delivered's GOLD floor. What rule makes a bounds-table net field branch-specific is not stated in the common data schema.

There is also a concrete lifecycle conflict outside composition: CoinPromise (lines 711–720) has a100 MUSD net floor and a0.190 GOLD net floor under one generic `SupplyDebtBackingConserved` terminal alternative. CoinLife mints100 then burns100 on both ordinary redemption and emergency settlement. Under “net receipt after every deduction” across that lifecycle, terminal MUSD receipt is0, not100. If GOLD is measured as whole-life cash change, depositing0.200 and receiving0.190 similarly yields −0.010, not0.190. LoanPromise's500 USD receipt floor also needs a named measurement boundary because the same episode later pays30. These examples appear intended to state operation-specific receipts rather than guaranteed positive whole-life returns.

**Exact discriminator:** from zero MUSD, execute the stated mint100/deposit0.200 followed by burn100/redeem0.190 with no fees. Supply/backing/debt conservation and the desired holder redemption both hold. Specify whether this is accepted or fails NetFloor, using a named signed field and evaluation scope. Separately, qualified OriginRefund with zero GOLD_F must satisfy its USD floor while never being labeled Delivered. An existing wallet balance must not satisfy a promised new net receipt.

**Repair:** define a typed net-receipt requirement keyed by terminal branch, recipient, asset and bounded measurement scope, such as the credit effects of a named accepted stage or the aggregate effects of the selected delivery leg. Include all relevant fees/deductions in that scope. Keep gross and fee limits cumulative across every accepted/retained effect in every branch; branch selection must not reset them. Give CoinPromise separate mint/ordinary-redemption/emergency receipt requirements, or change the lifecycle's untagged bounds to match its declared terminal promise. Bind overrides through signed data; do not infer them from the text of a constructor name. If whole-life net wealth is intended instead, use an explicitly signed signed-value floor and revise the examples accordingly.

This is a relation/schema ambiguity, not a request to lower a signed floor automatically. Composition's declared distinct outcomes are the right direction; the common rule should make their selection mechanically unambiguous.

## Acceptance and abstentions

Agree with the revised separation of beta from horizon, the typed intent/kernel authority boundary, explicit first-failure/no-new-effects rejection, durable unknown claims and qualified recovery. Agree that all eight family interfaces now have visible proposed lifecycle coverage. No remaining defect is asserted solely because these open features are unsupported by beta.

Do not claim complete financial consistency from the requirements checklist while F1 and F2 remain unresolved. They need concrete signed relations and updated worked cases, rather than another generic disclaimer that certification is open. Their resolution does not require production code changes or performing the future developer trials.

Abstain on usability, model training/AI accuracy, actual beta correctness, formal/native correspondence, evidence authentication, kernel qualification and ledger commitment. The document is a substantially improved specified interface; this follow-up does not supply experiments or proof evidence. Actual returned model/effort remains unverified pending a host receipt.
