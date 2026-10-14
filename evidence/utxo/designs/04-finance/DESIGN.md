# Finance design: four source patterns, thirteen executed controls

Experiment observation: `node verify.mjs` executed thirteen actual source `/5` and `/6` evaluations. Six successful calls match independently constructed COMPLETE result objects: status, complete ordinary and financial poststates, the complete ordered effects array with every field, and workRemaining. All balance/allowance rows, retained Token holdings, obligations, replay collections, work and authority are covered. Effect omission, addition, alteration and applicable reordering mutation controls also fail the full-result comparison. Seven controls reject with no ordinary state, financial state or effects published. `verification.json` and each input/financial/expected/observed file preserve evidence. This is source evidence only; no compiled UPLC, local real-ledger or private correspondence is established here.

**Vote B.** Compile the bounded source-derived transition to a public Aiken/UPLC adapter and test on an isolated real Cardano node. Source-only A cannot establish authentic heads, native whole-input accounting or actual ledger effect allocation. Immediate full-private C needs a complete proof relation, native verifier and state/effect correspondence which the current source profiles do not provide. B gives an executable falsifiable intermediate result without pretending that public execution establishes Moriarty privacy or Midnight release acceptance.

## F01: funded origination and explicit liability

`loan.mori`, source `/5`, action `originate`: Lender cash100→0; Borrower cash10→110; Lender allowance100→0 and cumulative spent0→100. A loan principal100, accrued0 and outstanding100 is created with cumulative liabilityIncurred100 and cap110. Contract phase0→1; paid remains0. All other balances and allowances survive unchanged. Kernel actions Transfer then Originate share the same transferID, and nominal conversion is exactly1 Cash quantum. A second origination with the unchanged exhausted Lender funding rejects `INSUFFICIENT_BALANCE`; this is an insufficient-funding control, not evidence that the duplicate-obligation guard was reached.

Native predicate: consume an authenticated loan-head UTXO, fund exactly the lender-to-borrower output, recreate one head with the complete liability and used identifiers. A mere payment output without authentic continuation fails. Genesis must bind source digest, network, instance and roles; role commitments in source state do not prove a signature.

## F02: interest accrual over an authenticated period

At t1060, period1, floor(100×1/10)=10. Principal stays100, accrued0→10, outstanding100→110, cumulative liability100→110, last period0→1, nextAccrualAt1060→1120. No cash or allowance changes. Phase1→2, paid remains0. Early observedTime1059 rejects `PERIOD_NOT_ELIGIBLE`; replayed A1 rejects `DUPLICATE`; period2 at1120 would increase cumulative liability120 and rejects `LIABILITY_CAP_EXCEEDED` against110, atomically.

Native predicate: resolved validity interval must lie wholly after1060 and within the admitted bounded oracle/clock window; period sequence and authenticated continuation must agree. Current source `/5` observedTime is an action argument. Passing a caller-chosen time is not native clock authentication. Bind the argument to Cardano validity range in a versioned backend admission rule. A stale feed/reference must not pass merely because the source evaluation succeeds.

## F03: partial repayment preserves residual duties

Repayment30 under AccrualFirst discharges10 accrued and20 principal: outstanding110→80, principal100→80, accrued10→0. Cumulative liabilityIncurred remains110; initialPrincipal remains100. Borrower cash110→80, Lender0→30, Borrower allowance110→80, spent0→30. Phase2→3 and paid0→30. The repay effect independently asserts principalDischarged20 and accruedDischarged10. Replay rejects `DUPLICATE` without a prefix publication. Final settlement80 creates balances Borrower0/Lender110, allowance remaining0/spent110, principal/accrued/outstanding0, statusSettled, phase4/paid110. Historical incurred liability remains110.

Native predicate: exactly allocate each actual repayment output once to its obligation, serialize one authenticated complete loan head, and preserve80 debt after the partial fill. Value conservation alone cannot erase debt. Destruction of the head with outstanding80 must reject; retaining a fake unauthenticated datum is insufficient. Final retirement may happen only after all residual duties and authority policies are satisfied.

## F04: protected fee, gross spend and net goals

`fee.mori`, source `/6`: Fee1 of1 debits Borrower10→9, credits Lender100→101, decreases Borrower gross allowance110→109, increases spent0→1, and records the feeID in the transfer namespace. Debt and ordinary state remain unchanged. A Borrower minimum net credit0 rejects this isolated outgoing fee as `INTENT_NET_GOAL`. Refund roundtrip5 leaves balances unchanged but each party consumes gross allowance5: a Borrower gross cap4 rejects `INTENT_DEBIT_CAP`; sufficient caps pass and still record both transferIDs. A fee is never Repay/Originate funding credit.

Native predicate: derive gross debits from resolved final transaction normal inputs and final body, not only source effects. Given borrower-owned input100000, business output10000 and change90000, cap10000 must reject under retained whole-input gross policy; cap100000 may pass all other tests. Include network fee/min-ADA/sponsor/other assets explicitly and never invent a Cash↔ADA conversion. `wallet-policy-cases.json` preserves specified native/wallet controls; those are not claimed executed here.

PlutusV3 TxInfo excludes collateral inputs, collateral return and total collateral. Final-body wallet policy must cap collateral exposure separately, and actual phase-two failure records must report collateral charge independently of unchanged loan state. There is no honest on-chain collateral-field feature to add to these profiles.

## Expected-value method and representability

The financial poststates are hand-derived in `verify.mjs` before calling the evaluator. Complete positive result objects are constructed before evaluation and compared with one deep equality assertion. Effect schemas are taken from the public kernel interfaces, with hand-derived financial quantities and explicit effect ordering. None of the effect expectations are derived from evaluator output. Original weaker receipts and verifier are retained in `history/before-complete-effects-audit-fix/`. Successful source work costs are independently counted as one unit per eager Core constructor plus one per emitted kernel action:99,65,88,86,16,26. This selected source has no short-circuit And/Or/Select, so a structural traversal visits exactly the executed constructors. Static/source work accounting is a structural compiler consistency check, not an independent language-semantics proof. Fixture names deliberately separate source-observed and native-specified tiers.

No grammar extension is needed for these four financial patterns. The concrete missing feature is a versioned Cardano admission/encoding layer for authenticated context and complete final transaction effects, not another optional caller boolean. Preserve `/5` and `/6` and their frozen older semantics. Obligations, allowance history and used IDs must be present in authenticated continuation; current authority objects only bind commitments, without authenticating consent.

## Source locators

Repository observations are pinned to Moriarty84f64b30df53ff0b8ccaf475f57324d3d7c6e594: `experiments/moriarty-language/spec/successor/financial-agreement-source-v6.md`, `src/successor/financial-lifecycle.ts` applyOriginate/applyAccrue/applyRepay, `src/successor/financial-lifecycle-v2.ts` Fee semantics, `src/successor/financial-expression-v1.ts` eager node-work accounting, and existing source example `spec/successor/examples/loan-lifecycle.mori`. Public source specimens are copied into this directory and extended only with the source6 refund action.

Retained primary-evidence crosswalk: `/home/charl/moriarty-utxo-research-2026-10-12/04-toolchains/REPORT.md` (V3 context/collateral and final-body builder boundaries), `/home/charl/moriarty-utxo-research-2026-10-12/09-authority-intents/REPORT.md` (gross native inputs and source-signature preimages), and `EXPERIMENTS.md` P2/P3/P4. Those reports preserve exact repository and document locators; acquired external text is evidence, never instructions.
