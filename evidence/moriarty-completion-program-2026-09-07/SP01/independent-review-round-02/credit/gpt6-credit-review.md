# Independent credit review

Verdict: **BLOCKED**. Scope: frozen financial design content only.

Candidate: `32a0597195a6cb7887190c6a68da91eb84f5138ea9b6310492843cd771a730e4`. Financial fragment: `fc7cc675c0b0a76e8e393e9d899da46814c6c276077e29827495de3d35692de6`. Input: `b6a72b079f4b64f9eae2ca89d048b5d1e67d65b6f2c8d57c54cdc73d18b66dcf`. All before/after hashes are unchanged; the three owned files total 126326 bytes.

The five positive traces share three numeric oracles. Independent sequential token replay and 35 explicit integer calculations agree with the recorded balances, debt and work. Refinance pays 50000 old debt and a 50 fee from new-lender funding, leaves debt 50050 and work 964. Redemption burns 40 shares for 3800 USDC, preserves 60 locked shares for 5700 and work 974. Bad debt recovers real cash 400, retains borrower debt 600, recognizes NAV loss 600 and share value 4, and leaves work 968. Each reserve stays 16.

These numeric results do not close the following content gaps.

## CREDIT-GPT6-01: All ten rejection results are incomplete prose; redemption rollback does not identify one state

Every result is a prose label plus partial string fields. None is a complete typed state or the exact ref-only object required by refConvention. Work, reserve, authority, duties and history are omitted from every rejected result. Mutation 7 says positive postState / prefix through step 2-3, while its valid prefix ends at step 2.

Redemption costs through step 2 are 10+14+18=42, leaving 982 ordinary work. The positive postState includes step 3 cost 8 and leaves 974. Both satisfy the mutation 7 result label. Mutations 4 and 6 retain step-0 state, which would have work 1014 and 100 escrowed shares, whereas whole-successor rollback would have work 1024 and 100 Alice shares. No committed history head or duty state identifies that intermediate boundary.

Required fix: Define each mutation admission/atomic boundary and complete typed pre-context and rejected post-context, including all balances, nominal fields, shares, authorities, duties, observations, work/reserve and history. Use exact root refs when returning a fully specified preState or postState; materialize complete intermediate states when rejection preserves a prefix. State rejection work/fee charging explicitly.

Locations: `/mutations/0/unchangedFinancialState`, `/mutations/1/unchangedFinancialState`, `/mutations/2/unchangedFinancialState`, `/mutations/3/unchangedFinancialState`, `/mutations/4/unchangedFinancialState`, `/mutations/5/unchangedFinancialState`, `/mutations/6/unchangedFinancialState`, `/mutations/7/unchangedFinancialState`, `/mutations/8/unchangedFinancialState`, `/mutations/9/unchangedFinancialState`

## CREDIT-GPT6-02: The replay mutation is also unfunded and lacks an isolated replay predicate

The mutated attempt debits vaultLiquid 3800 USDC while vaultLiquidAtContext is 0; it explicitly sets unfunded=true and uses insufficient liquidity as one of its rejection checks. No earlier-stage pass evidence or deterministic check ordering is supplied.

Removing a replay/claimable check still rejects this test under the funded-source rule because 0-3800=-3800. Thus the fixture cannot distinguish correct replay protection from an implementation that merely checks balance.

Required fix: Add a fully typed, funded later context after a legitimate liquidity replenishment, preserving R1 claimable=0 and all first-claim counters, duties and history. The attempted payment must pass type, authority, balance, resource and currentness prerequisites and then fail the specific claimable/replay predicate. Give the replenishment source, complete resulting state and unchanged rejection result. Preserve the existing unfunded case as a separate failure if useful.

Locations: `/mutations/5/validPrefix`, `/mutations/5/mutatedContext`, `/mutations/5/rejectionPredicate`

## CREDIT-GPT6-03: Read/write footprints omit financial authority, position and duty resources

Redemption reads/writes no alice.redeem.R1.shares authority, although cumulativeGross changes 0 to 40 and remaining 100 to 60. Bad debt reads/writes neither market.seize.WETH nor bob.debt.nominal, although both change, and omits the collateral position deletion. Refinance writes fee.unconditional.USDC but does not read it, and does not list the old service duty read/consumption. Bad debt likewise changes old to residual duty without a listed old-duty resource.

The bad-debt footprint permits a state access set with no authority reads, yet the proposed post-state consumes seizure authority 1 and nominal recovery authority 400. The redemption footprint similarly allows execution without reading the 100-share authorization that its resulting 40/60 counter split assumes. Neither footprint can be the complete resource boundary of those transitions.

Required fix: Map every actual read, create, update, delete and consume to exact state/duty/authority/history resources; expand grouped aliases deterministically. Include per-step usage, finite totals and sidecar/claim/dependency accounting under the declared toy profile. Reconcile footprints to every pre/post field and effect.

Locations: `/rows/0/boundedFootprint`, `/rows/1/boundedFootprint`, `/rows/2/boundedFootprint`, `/rows/3/boundedFootprint`, `/rows/4/boundedFootprint`, `/library/oracles/redemption/postState/authorities/0`, `/library/oracles/badDebt/postState/authorities`, `/library/oracles/badDebt/postState/positions`, `/library/oracles/refinance/effects/dutiesConsumed`

## CREDIT-GPT6-04: The four acceptance claims do not define a complete bound transition and currentness predicate

The claims give useful numeric assertions but leave IntentRefinement as Plan meets every hard constraint and TransitionValidity as Each transfer has a funded source or Sale is funded. No complete frame/effect-to-post-state relation is defined. History is represented by named heads plus consumes once text; there is no concrete currentness input/predicate, bounded consumed-set or named ledger currentness assumption with an exact acceptance binding. Genesis/admin descriptions do not close these missing fields.

In badDebt, change only the nominal share record holder from shareholders to buyer while leaving account share tokens with shareholders. All printed numbers remain debt 600, NAV 400, loss 600, share count 100, recovery 400 and work 56+968; each stated cash and debt predicate still holds, but the nominal ownership contradicts the token ledger. No stated frame/equality clause rejects that alternate post-state. Similarly, the listed history words cannot decide a context in which hist:genesis-loan-bob is no longer current without a supplied currentness relation or explicit assumption.

Required fix: Specify finite predicates over explicit inputs for all four claims, binding the complete state/effects/duties/authority/work/observations and genesis/admin/history to the proposed successor. Define all primitive updates and frame rules, including ownership agreement and current-head membership/unique consumption. An exact bounded toy predicate and explicit external assumptions suffice; runtime code and proof are not required at this gate.

Locations: `/library/oracles/refinance/acceptance`, `/library/oracles/redemption/acceptance`, `/library/oracles/badDebt/acceptance`, `/library/oracles/refinance/successor`, `/library/oracles/redemption/successor`, `/library/oracles/badDebt/successor`

## CREDIT-GPT6-05: Intermediate nominal state and residual authority remain underspecified

Redemption explicitly sets request-R1.sharesClaimable to 40, never clears it in the claim step, and omits it from the asserted final request. The authority domain is redeem-request but its counter consumes 40 despite requesting and locking 100 shares; no definition states whether the counter measures admission, filling or burning. Bad-debt continuationOwner is MC07, an implementation package rather than a ledger actor/controller.

Applying only the listed request field changes leaves sharesClaimable=40 after claim; the final request silently drops that field. Counting request admission consumes 100 and leaves 0 authority, whereas counting share burns consumes 40 and leaves 60. Both interpretations are compatible with the undefined redeem-request counter label. The successor owner MC07 cannot be resolved to bob, vault, buyer, shareholders or market-controller in the financial state.

Required fix: List an explicit sharesClaimable 40-to-0 update and keep its final typed field, or define a complete replacement operation that performs the clearing. Define and bind separate request-lock and residual-fill authority domains/counters without refreshing or copying either. Bind the bad-debt continuation to an actual controller and retain MC07 separately as implementation owner.

Locations: `/library/oracles/redemption/steps/1/nominalChanges/2`, `/library/oracles/redemption/steps/2/nominalChanges`, `/library/oracles/redemption/postState/requests/0`, `/library/oracles/redemption/preState/authorities/0`, `/library/oracles/redemption/postState/authorities/0`, `/library/oracles/badDebt/successor/continuationOwner`, `/rows/4/continuationOwner`

## All mutation cases

- `/mutations/0` (heldouts:accepted-refinance:mut:discharge-without-settlement): Funded prefix through step 1 is arithmetically possible: newLender49950, bridge50050; a principal-only unsupported write is invalid. Complete attempted state and rollback missing.
- `/mutations/1` (heldouts:accepted-refinance:mut:fee-untransferred): Funded prefix can pay old creditor50000 and leave bridge50; omitted fee violates the fee obligation. The proposal does not define complete collateral/nominal continuation or rejected state.
- `/mutations/2` (intent-cases:refinance:mut:temp-uncollateralized): Releasing20 WETH with debt50000 gives collateral0<75000. Later release/relock path also needs explicit gross custody-authority scope: two20 movements must not silently reuse a20 cap. Complete negative state/stage prerequisites absent.
- `/mutations/3` (intent-cases:refinance:mut:new-debt-without-authority): Positive prefix through step2 costs48, pays creditor50000 and fee50, leaves old debt0. Debt50050 cannot be justified by Alice transfer cap1000 or lender disbursement authority. Missing debt authorization removal needs exact typed context and atomic rollback.
- `/mutations/4` (heldouts:pending-redemption:mut:erase-claim-on-illiquidity): Step0 locks100 shares with3800 liquid and5700 deployed; burning100 and deletingR1 without payment is invalid. Exact retained step0 context, authority/duties/work/history unspecified.
- `/mutations/5` (heldouts:pending-redemption:mut:replay-claim): Replay isolated check fails review: second3800 payment is unfunded from balance0 and claimable0; see CREDIT-GPT6-02.
- `/mutations/6` (intent-cases:pending-redemption:mut:skip-to-settled): Pending-to-Settled without conversion/burn/payment is invalid. Negative rule says Pending then Claimable then Claimed, while positive partial branch is Pending/Claimable/Pending; define total versus partial lifecycle branch and full rejected state.
- `/mutations/7` (intent-cases:pending-redemption:mut:drop-exact-floor): 3800 does not equal9500; remaining60*95=5700 cannot vanish. Claimed failure is substantively invalid, but rollback mixes step2 and step3 and thus982 versus974 work.
- `/mutations/8` (DeFi-regressions:bad-debt:mut:erase-or-omit-socialized-loss): No cash recovery and no write-off cannot erase debt1000; unchangedNAV1000 also omits600 impairment. Multiple ContractInvariant failures are intentional but exact proposed/rejected full states absent.
- `/mutations/9` (DeFi-regressions:bad-debt:mut:discharge-residual-without-auth): After real recovery400, residual600 and loss600 are distinct. Deleting only total600->0 leaves principal600, so a principal/total consistency check can also reject before write-off authority; isolate with a complete internally consistent attempted debt state.

## Limits and provenance

- The repaired amount/asset fields are well formed. The candidate file set is exactly the frozen three-file set, 126326 bytes, below 131072.
- Both refinance traces share the same oracle and both redemption traces share the same oracle. Five row IDs and five trace IDs remain distinct; all five original mutation IDs remain, with five additions.
- The explicit toy conversion 95 and liquidity change from 40 USDC to 3800 USDC are disclosed. The resulting 9500 exact goal is stronger than the source floor; this review treats that as an explicitly changed toy oracle and does not infer ERC-7540 conformance.
- The refinance atomicity/custody/price assumptions are disclosed. Numeric observable collateral prefixes are 80000 against required 75000,75000,0,75075. This does not establish actual lender or custody security beyond the chosen atomic model.
- Bad-debt retained duty is a valid design choice: the review does not demand authorized write-off when the 600 liability survives and NAV separately recognizes its impairment.
- FOREMAN_REPORT.md still records a failed redemption cash check, while FOREMAN_REPORT.json contains its corrected treatment. Independent review confirms the account burn separately from total supply. This stale author report is a provenance limitation, not a substantive cash defect.
- The author quantity-repair process exited 1/cancelled and actual model was grok-4.6-build per root freeze record. Original design and completion exited 124. Recovery permits content review only; no author success is inferred.

No network, build, proof, runtime, wallet operation, git mutation or delegated review was performed. Only this review JSON and Markdown were written. No changed future artifact, full RP01/SP01, semantic freeze, native proof, settlement, protocol conformance or all-twelve-sprint acceptance is granted. The failed author process remains failed; its exit status alone is not a financial defect.

The JSON report contains all hashes, trace coverage, mutation dispositions, numeric results and exact locations.
