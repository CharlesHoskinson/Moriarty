# Independent developer arithmetic result review

## Scope and method

Repository and integer-arithmetic observations, 2026-09-30. Restored AGENTS.md and the checked-in moriarty-dev:develop skill; refreshed guarded status in the assigned worktree. SP01.6 remains blocked by stale campaign inputs, missing accounting/live state and unresolved operational history. Pending transactions are empty. No campaign dispatch occurred.

Read original S1/S2/S3/G1/G2 programs, explicit scenarios, case manifests and each parent-reproduction-v3.json. G3 arrived during final verification; its18 original cases, starter, parent reproduction and two successful local fixture probes were then inspected separately. No implementation audits or peer verdicts were read. A separate Python calculation transcribed amounts/nonces from operative source terms and derived ordered effects and complete candidate post records from the fixtures, using integer arithmetic and AccrualFirst. No language/profile implementation was imported or executed. Only this report was written; no code/configuration/commit changes occurred.

## Counts and arithmetic results

| Seat | Original cases | Successful money cases |
| --- | ---: | ---: |
| S1 | 4 | 1 |
| S2 | 13 | 5 |
| S3 | 9 | 1 |
| G1 | 7 | 2 |
| G2 | 14 | 3 |
| G3 | 18 | 0 |

The65 original cases contain12 PreparedUnqualified,17 CoreRejected,12 AuthoringRejected,21 Unsupported and3 FormationRejected expectations. G3 contributes14 Unsupported and4 AuthoringRejected, with no original executable money case. Of the12 successful cases,11 assert complete ordered effects and10 assert full post records. S2 round10 asserts effects but omits post; round0 asserts status only. Independent calculations agree with every supplied successful money expectation. Six archived transfer starters were checked separately: these are copies of one example, not five original programs. The current shipped repayment tutorial fixture was also checked separately.

G3's two local probes are outside its18-case original assignment manifest and are not additional original passed tests.

All amounts in the next table are economic asset atoms. Balance columns retain fixture account order.

| Original case | Gross debit | Post balances | Post work remaining/spent | Post principal/accrued/outstanding/status |
| --- | ---: | --- | --- | --- |
| S1 merchant | 126625 | 373375 / 125375 / 1250 | 9/1 | none |
| S2 partial100.00 | 10000 | 70000 / 15000 | 9/1 | 41250/0/41250/Outstanding |
| S2 interest-only5.00 | 500 | 79500 / 5500 | 9/1 | 50000/750/50750/Outstanding |
| S2 full512.50 | 51250 | 28750 / 56250 | 9/1 | 0/0/0/Settled |
| S2 partial at rounds10 and0, independently reset | 10000 each | 70000 / 15000 each | 9/1 each | 41250/0/41250/Outstanding each |
| S3 fee settlement transfer | 25075 | 74925 / 30000 / 75 | 3/1 | none |
| G1 Alice payroll | 246300 | 4753700 / 260075 / 1475 | 7/3 | none |
| G1 Bob payroll | 188490 | 3011510 / 187550 / 9740 | 4/2 | none |
| G2 disburse | 126250 | 0 / 125000 / 1250 | 7/1 | none |
| G2 installment | 21875 | 0 / 21875 | 2/5 | 80000/0/80000/Outstanding |
| G2 close-note | 80625 | 0 / 80625 | 0/10 | 0/0/0/Settled |

S1 scale3:125.375+1.250=126.625 EURC. S3 scale2:3×0.25=0.75 EUR, gross250.75. Its successful operation is a transfer to a vault account, not an executed AMM swap. G1 Alice2450.75+12.25=2463.00; Bob1875.50+9.40=1884.90. S2 original principal500/accrued12.50: payment100 clears12.50 accrued and87.50 principal; payment5 leaves7.50 accrued; payment512.50 settles. G2 installment218.75 clears18.75 accrued and200 principal; later payoff806.25 uses independently stipulated new accrual6.25.

Transfers with nonzero fees derive Debit(gross), Credit(value), Credit(fee), UseAllowance(gross), UseReplay(domain/owner/nonce tuple), AdvanceHead, in that order. Repayments derive Debit, Credit, SetObligation, UseAllowance, UseReplay, AdvanceHead. Complete expected vectors match these derivations. Full post comparisons include economic IDs, ordered balances, obligation identity/accounting/status, allowance remaining/spent, work remaining/spent, consumed tuple replay key and head. Allowance decreases/increases by gross, work decreases/increases by1, and observed round is unchanged. The successor is stipulated, not authenticated.

Six starters: debit1010, post balances8990/1000/10, allowance8990 remaining/1010 spent, work9/1, h0→h1 and replay tuple [Midnight,Owner,n1]. Current repayment tutorial:3000 pays1000 accrued then2000 principal; principal/outstanding98000, accrued0, balances197000/3000, allowance197000/3000, work9/1 and replay tuple [Midnight,Payer,n1]. Both complete expectations agree independently.

## Reproduction and controls

Each inspected parent-reproduction-v3.json records original manifests and its starter as TestsPassed; G3's18 original cases pass through unsupported/type-rejected outcomes. S1's separate negative-control manifest is a4-case copy: it intentionally expects the wrong rejection code for insufficient allowance. Actual S0_AUTH_SCOPE makes exactly that case fail, yielding TestsFailed/exit1; the other3 pass. This is an expected control failure, not an arithmetic defect or4 additional original cases. A successful expected rejection is distinct from a prepared payment. Archived test summaries corroborate fixture comparison only; this review did not rerun them or establish authenticated execution.

S2 partial, interest-only and full cases restart the same open fixture. Round0/10 probes likewise reset it. G1 payroll has different heads/scenarios and does not demonstrate atomic or chained payroll. G2 disbursement is a transfer with no obligation creation; installment and close-note supply independent later liabilities/heads/rounds/work. Matching residual800 does not establish the intervening accrual or history.

## Tutorial factual check and all discrepancies

Inspected GETTING-STARTED.md and TUTORIAL-SITE-PLAN.md against current packages/moriarty-beta/src/cli.ts, src/starter.ts, package.json and examples/local/repay. Verified Node>=24 metadata; default transfer and explicit repay init templates; existing-directory refusal; readable check/JSON option; complete ordered effects/post comparisons when supplied; bounded mismatch pointers; and local-stipulation-only results. Tutorial transfer and repayment literals agree. This is factual inspection, not code authorization, fresh installation or editor/provider activation evidence. No site deployment is claimed.

1. **No original G3 financial execution:** the archive arrived during this review and all six seats are now inspectable. Its18 original cases exercise structural refusal/unsupported behavior. Its successful local transfer/repayment probes are discovery fixtures, not bridge/staking execution or original manifest money tests. Tutorial six-seat provenance is now supported by the available archives.
2. **Two weaker S2 assertions:** round10 omits post; round0 omits effects/post. Their values are independently derivable, but these cases do not assert full financial observations. The other10 original successes assert full vectors/posts.
3. **Stipulated history boundary:** no archived case establishes authenticated chaining, accrual, atomic multi-action execution or settlement. Unused source constants do not bind fixture liabilities; the tutorial correctly states this.

G3 probe arithmetic, separately checked against explicit inputs and archived simulator candidates: local-rail moves25000 atoms with fee0, leaving975000/25000/50000 balances; allowance975000/25000, work7/2, round12 unchanged, rail-head-0→rail-head-1 and tuple [HomeRail,StakerPrincipal,rail-n1]. Its ordered vector omits zero fee Credit. Local-repay pays125 atoms against principal5000/accrued125: principal5000/accrued0/outstanding5000, balances9875/125, allowance9875/125, work3/1, round4 unchanged, note-head-0→note-head-1 and tuple [HomeRail,Borrower,note-n1]. Archived ordered effects and full post records agree. The one-atom foreign transfer probe has holder balance0 and rejects S0_EFFECT_RANGE with no post; it is not a received bridge claim.

No incorrect supplied successful money amount, ordered effect or asserted candidate post was found. No optional broad testing was performed.

## Qualification boundary

PreparedUnqualified remains a local candidate. This report does not establish native payload correctness, cryptographic authority, authenticated snapshots/providers/heads, proof soundness, atomic ledger consumption, financial settlement, full horizon execution or formal correspondence. Artificial-user trials and matching literals do not establish population usability. The independent result is limited to the inspected fixture arithmetic and supplied expectation equality.
