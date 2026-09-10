# RP01 loss, default and funded recovery challenge

This is a **generic design oracle awaiting independent review** for the original blocking R01 finding. It is not Morpho, Maple, ACTUS or any other deployed protocol's behavior. It does not change an accepted language profile, SP05 expectations, canonical successor semantics or proof predicate. The recourse, impairment, fee incidence and pari-passu assumptions below are proposals for this case; no design vote or independent acceptance is asserted.

The original R01 counterexample starts with debt 1000 and collateral valued at 400, recognizes loss 600, and then reports debt zero without a sale, funded repayment, release or successor liability. This case makes each of those distinctions observable. Accounting impairment alone leaves all 1000 nominal debt intact. Cash proceeds fund a later reduction of 400. A subsequent funded payment reduces another 150. The remaining borrower duty is 450, even though its accounting carrying value is zero. No forgiveness is included or authorized.

## Scope and chosen assumptions

The initial state is a supplied snapshot of an already outstanding loan, not a demonstration of loan origination or authenticated custody. Borrower owes Pool 1000 Cash, with no accrued interest. Custodian holds four Collateral units owned by Borrower and encumbered to Loan1. Custodian has a bounded source grant to sell exactly those units to Buyer. The collateral is not also counted as a separate Pool NAV asset while the receivable is included.

Pool has 100 Cash, Borrower 200, Buyer 400, FeeCollector zero and Custodian zero. These five parties are the complete transfer-account universe; HolderA and HolderB own only the two enumerated share claims, and no cash transfer to them is permitted in this case. Total Cash is 700 and total Collateral four throughout. Pool has no reserve insurer or junior tranche. Its existing 1000 shares are held 600/400. Share book values allocate Pool NAV pro rata, not a promise that the whole book value can be withdrawn immediately.

The default/valuation assumption is that only 400 of the 1000 gross receivable is expected to recover at the initial assessment. A supplied executable bid offers 400 Cash for four Collateral units. A default notice is current at time 100 with due time 99. The bid is observed at 100 and valid through 105; sale occurs at 101. These are bounded assumed records, not authenticated external truth. The checker requires their selected issuer, asset/unit, duty, sequence identity, timing and version relationships, but does not verify signatures or future buyer/custodian availability.

The selected policy retains recourse for unpaid principal. Liquidation costs of 10 are borne by Pool, so the borrower receives gross 400 repayment credit although Pool's net cash receipt after its fee is 390. Later Borrower pays 150 to Pool plus a separate five-unit fee to FeeCollector. That fee is not credited against principal and does not capitalize new debt. There is no accrual or interest, no reserve absorption, no share issuance/burn/redemption, no forgiveness, no cancellation and no expected subsequent recovery guarantee.

The supplied survey's shorthand description of liquidation ending a loan is not adopted as a rule that deletes recourse. Its discussion motivates separating collateral liquidation and repayment; it does not specify this case's recourse/accounting policy. The report's named Morpho loss-socialization discussion is secondary evidence for the challenge class, not a pinned Morpho implementation oracle.

## Independent expected arithmetic

| Stage | Pool cash | Nominal duty / gross receivable | Impairment allowance | Carrying receivable | Pool NAV | HolderA / HolderB book value | Cumulative allocated net loss |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Before default | 100 | 1000 | 0 | 1000 | 1100 | 660 / 440 | 0 |
| Default and impairment | 100 | 1000 | 600 | 400 | 500 | 300 / 200 | 600 |
| Collateral sale and funded discharge | 490 | 600 | 600 | 0 | 490 | 294 / 196 | 610 |
| Later funded recovery | 640 | 450 | 450 | 0 | 640 | 384 / 256 | 460 |

The accounting identities are:

- gross receivable = nominal outstanding duty;
- carrying receivable = gross receivable − impairment allowance;
- NAV = Pool Cash + carrying receivable;
- impairment allowance = cumulative impairment expense − cumulative recovery gain;
- allocated net loss = impairment expense + Pool fee expense − recovery gain;
- initial NAV − current NAV = allocated net loss;
- share book value = NAV × held shares / 1000; loss allocation uses the same fixed weights.

All products are checked against UInt128 before exact division; this example has no rounding residue. The two holders' losses are 360/240 after impairment, 366/244 after Pool pays its fee, and 276/184 after recovery. Their final book values are 384/256, totaling 640. Share supply remains exactly 1000. Borrower's separate fee five is recorded in its cash debit and FeeCollector credit; it is not charged to Pool's share value.

Default performs two distinct events: mark Loan1 Defaulted, then recognize impairment 600. Neither event transfers any asset or reduces principal. Collateral sale is one tentative four-event operation: Custodian sends four Collateral to Buyer, Buyer sends 400 Cash to Pool, that executed transfer funds nominal repayment 400, then Pool sends fee ten. Collateral encumbrance falls to zero because the actual collateral is delivered; outstanding recourse falls only to 600 because actual cash paid only 400.

Recovery is another tentative four-event operation: Borrower transfers 150 to Pool and five to FeeCollector; that same-step 150 transfer funds a nominal discharge of 150; then the matching allocation authorizes reversal of impairment 150. The repayment lowers both gross receivable and duty to 450. Reversal lowers allowance to 450 and records gain 150. All remaining principal remains visible despite zero carrying receivable.

Exact token accounts at the final boundary: Pool Cash 640, Borrower Cash 45, Buyer Cash zero, FeeCollector Cash 15, Custodian Cash zero; Buyer owns four Collateral, all other Collateral accounts are zero. Token quantities are conserved even though lenders lost 460 of accounting value. This is the distinction that a token-conservation-only argument misses.

## Gross spending, fees and net delivery

`case.json` includes ten complete actor/asset effect rows for every step. GrossDebit sums actual outgoing transfers; GrossCredit sums incoming transfers; netChange is GrossCredit minus GrossDebit. NetCredit and NetDebit are its nonnegative parts. Refund is zero because no refund action exists; no grant can be restored by netting.

- Sale: Custodian gross-debits four Collateral, Buyer gross-debits 400 Cash and receives four Collateral. Pool gross-credits 400 Cash and gross-debits fee ten, yielding net credit 390. FeeCollector receives ten.
- Recovery: Borrower gross-debits 155 Cash, including fee five. Pool net-credits 150 and FeeCollector net-credits five. Nominal repayment is 150, not 155. Cash balances and duty movement are checked independently.
- Default/impairment: every transfer effect is zero while lender book values and impairment accounts change.

Network fees, transaction change and fees of rejected attempts are outside this design oracle and are not assumed zero. This case's explicit Cash fees are economic fees within the modeled asset universe.

## Source authority and continuation

Each grant has an issuer, holder, permitted operation, asset/payee restriction, Loan1 binding, version GenericLossV1, expiry 200, initial quantity, remaining quantity and cumulative spent quantity. All steps preserve initial = remaining + spent. The source grants are assumed authorization records; no cryptographic or institutional legitimacy is inferred from their names.

| Grant | Initial | After default | After sale | After recovery |
| --- | ---: | ---: | ---: | ---: |
| DefaultGrant (event count) | 1 | 0 | 0 | 0 |
| ImpairGrant (Cash accounting loss) | 600 | 0 | 0 | 0 |
| CollateralGrant (Collateral gross debit) | 4 | 4 | 0 | 0 |
| BuyerGrant (Cash gross debit) | 400 | 400 | 0 | 0 |
| PoolFeeGrant (Cash gross debit) | 10 | 10 | 0 | 0 |
| BorrowerGrant (Cash gross debit) | 155 | 155 | 155 | 0 |
| RecoveryGrant (nominal discharge) | 550 | 550 | 150 | 0 |
| ReverseGrant (accounting recovery) | 150 | 150 | 150 | 0 |

This separates nominal recovery authority from the transfer grants that actually pay for it. A collateral delivery is not denomination-Cash funding. A previous or absent transfer cannot be used to discharge debt. The borrower still has 45 Cash after the trace, but its source debit grant is exhausted; available cash is not renewed permission. All grants remain as spent records. The successor remains owned by Servicer with Loan1 Defaulted/outstanding 450; no work exhaustion or episode label can remove it. More recovery would require a separately reviewed successor authorization instead of copying or refreshing the exhausted grants.

There is no authorized-forgiveness path: nominal creation and forgiveness limits are zero, and a ForgiveDebt event rejects. Adding a future release would require a distinct grant, an explicit duty transition and retained economic loss accounting; recognition of an impairment is not such a grant.

## Finite work, observations and footprints

There are three ordered steps and at most four events per step. Each event consumes one ordinary unit. Work starts remaining 12/spent zero/closure reserve two. After the two default events it is 10/2/2; after the four sale events 6/6/2; after four recovery events 2/10/2. Remaining + spent = 12 and reserve = 2 at every boundary. The reserve is preserved and unavailable to these operations. There is no continuation refresh.

The complete state has ten asset accounts, one duty, two share claims and eight grant records. Capacity is fixed at those amounts. Histories grow to five transfer IDs, two allocation IDs and two observation IDs. All identity sets are checked for replay. Observations are DefaultObs and BidObs; their current timestamps, expiry, issuer, duty/asset fields and policy version are explicit. Sequential read/write conflict handling is selected; no parallel or reordering claim is made.

Each step lists concrete JSON read paths for every pre-state leaf because the complete state-invariant check reads all of them. It lists exactly the differing post-state leaf paths, including appended history entries. Maximum observed read count is 144 and write count 35, under selected limits 256 and 128. Writes are recomputed and compared, so a generic label cannot hide an omitted affected account. Source policy/observation dependencies are pinned separately from state paths.

The bounded sidecar ceiling is 131072 compact JSON UTF-8 bytes; the complete case fits. Four logical mandatory predicates and three sequential dependencies are allowed. The checker charges one logical verification unit per source hash, four invariant obligations per state boundary, and one per event: 9 + 4×4 + 10 = 35, below the declared budget 64. These are explicit design accounting units, not a measured cryptographic cost model or native verifier admission implementation. No proof allocation occurs here. Closed canonical signing/display, revocation and actual admission budgets remain owned closure work.

Each step executes against a tentative clone. A failed check throws and publishes no successor. No input case object is mutated. The recorded before state is therefore the entire financial failure residual, including debt, grants and reserve. No claim is made that failed host execution is free.

## Checker and discriminating invalid variants

Run from the worktree root:

```sh
node deliverables/rp01-loss-allocation-2026-09-09/check-case.mjs
```

The checker reads only these authored case/source files. It derives asset movement, same-step funding consumption, obligation updates, accounting values, share allocations, resource depletion and actor effects with bounded integer arithmetic, then compares every complete post-state and footprint. It does not read generated language output or use labels such as `erasure=false` as financial evidence.

The 22 variants cover debt deletion after impairment, absent sale cash, collateral substituted for cash funding, absent collateral delivery, excess nominal discharge, missing allocated loss, absent later recovery, deleted residual duty, gross fee hidden by netting, unfunded impairment reversal, unauthorized forgiveness, wrong actor, unbound version, stale bid, transfer replay, work refresh, gross grant refresh, ordinary work exhausted despite reserve, omitted write footprint observation capacity exhaustion, substituted observation kind and substituted sequence. Each must reject with its named code. The original complete positive trace must still pass; the variants do not count as independent reviews.

Initial executable check failed with ORACLE_NOT_IMPLEMENTED before implementation. After implementation, the positive trace and all 22 specified rejection codes passed. The command reports numeric residuals and acceptance scope, not a claim that .mori or ledger behavior is implemented.

## Representation and closure ownership

The existing funded-repayment kernel can express the arithmetic of same-step Cash Transfer plus Repay with identity conversion, nominal quantities and fee transfers in its collection bounds. It cannot preserve this entire richer record unchanged: Defaulted status, collateral enforcement authority, impairment accounts, loss allocation, share claims and reversal are not its accepted schema. Dropping these fields to obtain kernel success would erase required semantics. No source/Core/K/Compact translation is supplied or implied.

- **SP01:** review recourse, loss policy, fees, authority/display rules and R01 challenge inclusion. This closes no full RP01 gate without the required independent reviews and crosswalk.
- **SP02:** specify and typecheck the richer state/effect/authority syntax and canonical display; reject unsupported constructs.
- **SP03:** implement the complete transition relation and correspondence checks with the same observations and failure residuals.
- **SP07:** bind concrete credit/default/recovery mechanisms to primary source/version policies, including impairment, fees, rounding and loss allocation.
- **SP09:** establish ContractInvariant, IntentRefinement, TransitionValidity and HistoryCompliance with grant lineage, external-observation/custody assumptions, activation/revocation and acceptance evidence.

The current scopes remain design oracle and local arithmetic verification. Protocol conformance, authenticated external facts, general recursive proofs and network settlement are open.

## Pinned provenance

Source requirements and source facts are distinct from the chosen generic model assumptions. The only primary paper excerpt inspected for this task is Kotzer et al. PDF pages 4–5; it motivates collateral/default distinctions but does not define this accounting policy. Report/crosswalk statements are secondary research inputs. No network acquisition or current protocol claim was made.

| Path | SHA-256 | Inspected locator and authority |
| --- | --- | --- |
| `openspec/REPORT-RECONCILIATION-2026-09-07.md` | `a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da` | RP01 Financial and intent semantics challenge; Bad debt row; repository requirement |
| `openspec/sprints/sp01-financial-contract-and-execution-admission.md` | `bc60ffef020b282954f5543db3b00f51f06d45dd113cd68d853967ff50acc23b` | SP01.2 and SP01.3; repository requirement |
| `openspec/sprints/report-lessons.json` | `3c6aed12687209279d94af3e058193326d8361001cac92db2509ccbaeda695c8` | TX02, TX05, TX10, VX05; repository requirement |
| `evidence/moriarty-completion-program-2026-09-07/SP01/full-challenge-map-01/gpt6-review.json` | `dcae4765ff3dcab322f1c00f6780ab0b516e62db6d1b2b5f988f8b82ec0e42a2` | R01; retained rejection |
| `raw/reports/unified-2026-09-07/defi.md` | `c458c6c9e32675afe6a6aa44e6f1a6767d3963accb6d291ec24e6212ecac9975` | lines866-880 Morpho bad debt; secondary report; product-specific claim not adopted |
| `deliverables/defi-taxonomy-papers-2026-09-08/DESIGN-IMPLICATIONS.md` | `82208c569d020fbfe97acbc6d5be7fbae189b0aa593b353948536a2714e678c6` | Collateral and loss; TX10; research recommendation |
| `deliverables/erc4626-vault-report-2026-09-08/DESIGN-IMPLICATIONS.md` | `fcc1e4593ed8bfe9b8181cd2a7aed77f96df1c4504c4219d812a72673936fdfb` | four valuation roles; liquidation versus loss allocation; research recommendation |
| `.raw/captured/7165f6cbe7b86280dd5ed94dd5585d3b55dda34d77776f2d9db87321683475c8.pdf` | `7165f6cbe7b86280dd5ed94dd5585d3b55dda34d77776f2d9db87321683475c8` | PDF pages4-5 collateral liquidation discussion; primary survey; locally inspected excerpt; no recourse policy inferred |
| `experiments/moriarty-language/spec/successor/repayment-kernel.md` | `4e8a0beca7fd3998a62b111db176f74f375dd90d83839622803062e32a84cb6b` | Transfer and Repay; UInt128; finite work; no free debt discharge; existing local projection scope |

Case SHA-256: `97e4cbf714e8490c0a26b7bd1badf392f8b2246c4c2be530349758343b7287f2`.
