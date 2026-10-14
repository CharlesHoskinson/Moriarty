# Financial pattern validation

Draft. Lifecycle S2: written proposal or design. No implementation or native result approval.

## ADDED Requirements

### Requirement: EU-FIN-001 Complete independent result oracle

WHEN a financial case is evaluated, THEN the conformance runner SHALL compare the complete result with the independently frozen expected envelope.

#### Scenario: EU-FIN-001 positive

- **WHEN** the selected versioned evaluator accepts the frozen case
- **THEN** status, all ordinary fields, complete financial state, ordered effects, and workRemaining equal the frozen expected envelope

#### Scenario: EU-FIN-001 failure

- **WHEN** one effect, row, identifier, obligation, diagnostic, or retained field differs
- **THEN** the case fails with the first differing JSON path and both values

### Requirement: EU-FIN-002 Profile and action identity

WHEN a source action is compiled, THEN admission SHALL bind its exact profile, contract, lifecycle version, action selector, Core hash, and schema hash.

#### Scenario: EU-FIN-002 positive

- **WHEN** custody release uses source /5, contract /4, lifecycle /1, and Release tag 0
- **THEN** the compiled relation evaluates release and preserves the revision03 ABI

#### Scenario: EU-FIN-002 failure

- **WHEN** Refund tag 1 is paired with release arguments, or /6 is offered to T1
- **THEN** the source cross-action case is GUARD_FAILED and T1 profile mismatch is PROFILE_MISMATCH

### Requirement: EU-FIN-003 Ordered rows and durable state

WHEN the financial kernel changes a balance, THEN lowering SHALL preserve evaluator row order, append new recipients, and retain every unrelated field.

#### Scenario: EU-FIN-003 positive

- **WHEN** the Lender balance row is absent before a custody release
- **THEN** the post appends Lender Cash 100 and deep-equals the source envelope

#### Scenario: EU-FIN-003 failure

- **WHEN** lowering sorts arrays, drops Witness Cash 5, resets allowances, or clears prior identifiers
- **THEN** complete-result comparison fails and the compiled transition rejects

### Requirement: EU-FIN-004 Exact escrow branches

WHEN a custody release, refund, or cancel satisfies its selected source guards and admitted kernel preconditions, THEN the financial relation SHALL transfer Cash 100 from Escrow to its selected recipient.

#### Scenario: EU-FIN-004 positive

- **WHEN** release selects Lender, refund selects Borrower, or cancel selects Borrower with the source-valid window
- **THEN** Escrow Cash becomes 0, selected balance increases 100, allowance consumes 100, and locked becomes 0

#### Scenario: EU-FIN-004 failure

- **WHEN** actor, recipient, locked state, amount, or branch selector violates a source guard
- **THEN** the source returns its complete GUARD_FAILED envelope and no accepted post or effects

### Requirement: EU-FIN-005 Residual duty and T1 fence

WHEN E12 cancels a cash lock with ResidualDuty outstanding 40, THEN source correspondence SHALL retain that duty and T1 admission SHALL return NONEMPTY_OBLIGATIONS.

#### Scenario: EU-FIN-005 positive

- **WHEN** E12 source /5 cancellation transfers Cash 100 to Borrower
- **THEN** FundedExpressionPrepared retains outstanding 40 and T1 performs no spend

#### Scenario: EU-FIN-005 failure

- **WHEN** a proposed continuation removes the obligation to obtain T1 admission
- **THEN** the conformance case fails and the residual-duty native positive remains future work

### Requirement: EU-FIN-006 Whole interval branch semantics

WHEN time guards use inclusive lower and upper arguments, THEN the relation SHALL require lower <= upper and match both converted finite ledger endpoints.

#### Scenario: EU-FIN-006 positive

- **WHEN** release interval is [1099,1099] or refund interval is [1100,1100] in Layer S
- **THEN** the boundary follows the selected source guard without a live inclusion claim

#### Scenario: EU-FIN-006 failure

- **WHEN** release upper is 1100, bounds reverse, or Args claim a narrow range against a wider TxInfo range
- **THEN** source GUARD_FAILED or ledger interval-binding rejection is recorded at its actual layer

### Requirement: EU-FIN-007 Authenticated oracle projection

WHEN an oracle action is admitted, THEN the backend SHALL bind every oracle argument to one authenticated CIP31 reference datum and its complete validity window.

#### Scenario: EU-FIN-007 positive

- **WHEN** USD_Cash ApprovedFeed datum has price 10, publishedAt 1000, and interval [1010,1020]
- **THEN** feed, publisher, price, and timestamp equal authenticated datum fields and whole-window freshness holds

#### Scenario: EU-FIN-007 failure

- **WHEN** publisher label is forged, reference NFT is wrong, publishedAt exceeds lower, or upper is 1021
- **THEN** context authentication or freshness fails without publishing a financial transition

### Requirement: EU-FIN-008 Joint role semantic authority

WHEN joint-role or AP03 executes, THEN acceptance SHALL require the admitted distinct roles and the ledger package authentication receipt for every required role.

#### Scenario: EU-FIN-008 positive

- **WHEN** Borrower and Lender roles select Lender payout with both required native authorities
- **THEN** the source role guards and actual ledger witness predicates hold together

#### Scenario: EU-FIN-008 failure

- **WHEN** only actor strings, consent booleans, funding witnesses, or reference-only bearer tokens are supplied
- **THEN** native authority rejects even if source labels evaluate successfully

### Requirement: EU-FIN-009 Bilateral atomic settlement

WHEN SB01 exchange is accepted, THEN settlement SHALL allocate A10 to trader and B25 to provider in one atomic transition.

#### Scenario: EU-FIN-009 positive

- **WHEN** both admitted authorities consent and both escrow assets are prefunded
- **THEN** both ordered settlement effects match the atomic source oracle and closed changes 0 to 1

#### Scenario: EU-FIN-009 failure

- **WHEN** one leg is absent, B is 24, recipient changes, or Bob consent is absent
- **THEN** the corresponding source or native case rejects without a partial business settlement

### Requirement: EU-FIN-010 Constant product exact arithmetic

WHEN SB02 swaps input d against authenticated reserves x and y, THEN arithmetic SHALL calculate floor(d*997*y/(x*1000+d*997)) with checked intermediate bounds.

#### Scenario: EU-FIN-010 positive

- **WHEN** x is 1000000, y is 2000000, and d is 10000
- **THEN** output is 19743, remainder is 162290000, reserves become 1010000 and 1980257, and product is 2000059570000

#### Scenario: EU-FIN-010 failure

- **WHEN** denominator is zero, intermediate overflows, or output is replaced with 19744
- **THEN** arithmetic or full-envelope comparison fails without accepted effects

### Requirement: EU-FIN-011 Slippage and serialized successor

WHEN SB02 uses a minimum output, THEN acceptance SHALL require computed output >= minimum and the authenticated current pool predecessor.

#### Scenario: EU-FIN-011 positive

- **WHEN** first minimum is 19743 and second swap uses independently specified first successor
- **THEN** first output is 19743 and second output is 19356 with reserves 1020000 and 1960901

#### Scenario: EU-FIN-011 failure

- **WHEN** minimum is 19744 or two writers reuse the same spent pool head
- **THEN** source GUARD_FAILED or actual ledger conflicting-input rejection is recorded

### Requirement: EU-FIN-012 Order execution and timeout cancellation

WHEN SB03 closes a funded order, THEN settlement SHALL dispose of all Cash 100 under its selected execute or cancel policy.

#### Scenario: EU-FIN-012 positive

- **WHEN** owner executes at 999 or cancels at 1000 with phase 0
- **THEN** phase becomes 1 or 2, payout is 100, allowance becomes 0, and complete post matches the source

#### Scenario: EU-FIN-012 failure

- **WHEN** execute occurs at 1000, cancel at 999, owner changes, phase is 2, or payout is 70
- **THEN** source GUARD_FAILED publishes no accepted post or effects

### Requirement: EU-FIN-013 Injective batch satisfaction

WHEN SB04 settles two orders, THEN the ledger relation SHALL cover each participating sorted input exactly once and assign distinct tagged payout outputs.

#### Scenario: EU-FIN-013 positive

- **WHEN** indices 1 and 2 fund ORDER1 Cash 30 and ORDER2 Cash 70 at slots 0 and 1
- **THEN** paid increases 100 and both ordered effects allocate once even when recipients share a credential

#### Scenario: EU-FIN-013 failure

- **WHEN** input 1 repeats, output 0 repeats, second payout is 69, or an extra participating order is omitted
- **THEN** source guard or native allocation rejection prevents both orders claiming one payout

### Requirement: EU-FIN-014 Funded loan origination

WHEN F01 originates Loan1 with principal 100, THEN the financial relation SHALL retain the matching Transfer and Origination effects and complete liability.

#### Scenario: EU-FIN-014 positive

- **WHEN** Lender Cash 100 and allowance 100 fund Borrower initial Cash 10
- **THEN** balances become 0 and 110, Loan1 outstanding is 100, liabilityIncurred is 100, and nominal cap is 110

#### Scenario: EU-FIN-014 failure

- **WHEN** origination repeats with exhausted funding or matching funding transfer is missing
- **THEN** the actual first failure is recorded, including INSUFFICIENT_BALANCE for repeat-unfunded

### Requirement: EU-FIN-015 Period accrual and cumulative cap

WHEN F02 accrues the next eligible period, THEN the kernel SHALL retain principal and update interest, outstanding, incurred liability, period, and next eligibility atomically.

#### Scenario: EU-FIN-015 positive

- **WHEN** period 1 at 1060 uses principal 100, numerator 1, denominator 10, floor, and periodSeconds 60
- **THEN** accrued becomes 10, outstanding and incurred become 110, last period becomes 1, and nextAccrualAt becomes 1120

#### Scenario: EU-FIN-015 failure

- **WHEN** time is 1059, A1 repeats, or period 2 would incur 120 against cap 110
- **THEN** PERIOD_NOT_ELIGIBLE, DUPLICATE, or LIABILITY_CAP_EXCEEDED matches the complete source rejection

### Requirement: EU-FIN-016 Repayment allocation and final settlement

WHEN F03 applies AccrualFirst repayment, THEN the kernel SHALL discharge accrued debt before principal and retain initialPrincipal and cumulative liability.

#### Scenario: EU-FIN-016 positive

- **WHEN** repayment 30 applies to principal 100 and accrued 10
- **THEN** principal becomes 80, accrued becomes 0, outstanding becomes 80, and later repayment 80 settles outstanding to 0

#### Scenario: EU-FIN-016 failure

- **WHEN** an implementation erases debt 80, resets incurred 110, reverses Transfer/Repayment effects, or reuses P1/R1
- **THEN** full comparison fails or DUPLICATE rejects with no accepted financial publication

### Requirement: EU-FIN-017 Allocation policy arithmetic

WHERE AccrualFirst, PrincipalFirst, or ProRata is admitted, WHEN repayment n is allocated, THEN the kernel SHALL apply the pinned allocation policy exactly.

#### Scenario: EU-FIN-017 positive

- **WHEN** n is positive and does not exceed outstanding
- **THEN** AccrualFirst uses dA=min(n,A), PrincipalFirst uses dP=min(n,P), and ProRata uses dP=floor(n*P/(P+A)) with dA=n-dP

#### Scenario: EU-FIN-017 failure

- **WHEN** P+A is zero, checked product overflows, or a discharge exceeds its component
- **THEN** the actual kernel returns INVARIANT, OVERFLOW, or ALLOCATION_COMPONENT without accepted post

### Requirement: EU-FIN-018 Protected fee effect semantics

WHEN F04 emits Fee, THEN the full kernel SHALL debit its payer allowance and transfer namespace without treating Fee as origination or repayment funding.

#### Scenario: EU-FIN-018 positive

- **WHEN** Borrower pays Fee1 Cash 1 from balance 10 to Lender balance 100
- **THEN** balances become 9 and 101, Borrower allowance becomes 109/spent 1, and debt remains unchanged

#### Scenario: EU-FIN-018 failure

- **WHEN** Fee is counted as repayment funding or an outgoing fee violates Borrower minimum net credit 0
- **THEN** full comparison fails or source INTENT_NET_GOAL rejects

### Requirement: EU-FIN-019 Gross inputs and net wallet outcomes

WHEN native outcome authority is checked, THEN its gross projection SHALL count complete resolved normal inputs and its net projection SHALL include all authorized wallet outputs and fees.

#### Scenario: EU-FIN-019 positive

- **WHEN** Borrower input is 100000, business payment is 10000, and change is 90000
- **THEN** gross cap 10000 rejects and cap 100000 remains subject to every other authority predicate

#### Scenario: EU-FIN-019 failure

- **WHEN** a roundtrip refund hides gross debits or fees reduce a promised minimum net credit
- **THEN** INTENT_DEBIT_CAP or native outcome-policy rejection is retained at its actual layer

### Requirement: EU-FIN-020 Exact authority envelope identity

WHEN AP01 outcome authority is verified, THEN its signed envelope SHALL bind network, deployment, instance, program, action, arguments, predecessor, nonce, asset policy, and complete monetary projection.

#### Scenario: EU-FIN-020 positive

- **WHEN** route transfers 100 and charges Fee 5 with gross caps 100/5 and minimum credit 95
- **THEN** source complete result has net Borrower credit 95 and native acceptance additionally verifies all identity bindings

#### Scenario: EU-FIN-020 failure

- **WHEN** minimum is 96, a well-formed source networkTag changes, or the signed body/head is substituted
- **THEN** source INTENT_NET_GOAL rejects underfill and native identity rejects substitution despite source shape acceptance

### Requirement: EU-FIN-021 Durable per-instance nonce and identifiers

WHEN AP02 consumes an outcome, THEN its authenticated successor SHALL increment nonce exactly once and retain all used and cancelled identifiers.

#### Scenario: EU-FIN-021 positive

- **WHEN** consume Nonce1 on a fresh instance has nonce 0
- **THEN** nonce becomes 1 and the identifier remains in subsequent authenticated state

#### Scenario: EU-FIN-021 failure

- **WHEN** old snapshots are restored, the same signed outcome reaches the next head, or migration erases history
- **THEN** native replay rejects while pure old-snapshot source acceptance remains recorded as a source limitation

### Requirement: EU-FIN-022 Lifetime duties across lifecycle operations

WHEN split, join, migrate, or terminate processes financial state, THEN its approved relation SHALL conserve duties, cumulative allowances, identifier history, and lifetime work bounds.

#### Scenario: EU-FIN-022 positive

- **WHEN** a future approved lifecycle operation retains debt and authority across authenticated successors
- **THEN** sum of residual liabilities and remaining authority matches its admitted policy without resetting spent work

#### Scenario: EU-FIN-022 failure

- **WHEN** termination removes debt 80 or split duplicates authority and join omits a predecessor
- **THEN** admission rejects unsupported operation or the conservation predicate rejects the proposed transition

### Requirement: EU-FIN-023 Arithmetic and quantum refusal

WHEN numeric values cross source or settlement boundaries, THEN the backend SHALL enforce the pinned type ranges, rounding policy, quantum divisibility, and denominator rules.

#### Scenario: EU-FIN-023 positive

- **WHEN** conversion has mantissa 1, scale 0, rounding none, and quantum 1
- **THEN** nominal-to-ledger conversion is exact and no undeclared Cash-to-ADA conversion occurs

#### Scenario: EU-FIN-023 failure

- **WHEN** conversion remainder is nonzero under none, quantum differs from T1 1, or FloorDiv appears in T1 Core
- **THEN** source INEXACT_CONVERSION or current-layer unsupported admission rejects without integer wrapping

### Requirement: EU-FIN-024 Work counters and early failure

WHEN an action consumes semantic work, THEN correspondence SHALL preserve actual traversed work and keep transport counters separate from ordinary revision and kernel work.

#### Scenario: EU-FIN-024 positive

- **WHEN** custody release starts work 1024/spent 0/reserve 16 and ordinary revision 0
- **THEN** semantic work becomes 975/spent 49/reserve 16 while transport 0/1 becomes 1/0 and ordinary revision becomes 1

#### Scenario: EU-FIN-024 failure

- **WHEN** work is one short or spent total exceeds its admitted range
- **THEN** source WORK_EXHAUSTED at workUsed 48 or INVARIANT is retained instead of replacing it with full debit 49

### Requirement: EU-FIN-025 Separate network fee and collateral policy

WHEN a transaction body is authorized, THEN wallet checks SHALL cap collateral separately and distinguish source Fee, ADA fee, sponsorship, change, and other assets.

#### Scenario: EU-FIN-025 positive

- **WHEN** ADA fee and min-ADA sponsorship come from declared funding outside kernel Cash
- **THEN** Cash effects remain exact and final-body collateral exposure fits the authorized cap

#### Scenario: EU-FIN-025 failure

- **WHEN** collateral exposure is 5000000 against cap 2000000 or a phase-two invalid transaction loses collateral
- **THEN** wallet authorization rejects excess and any actual collateral charge is reported separately from unchanged business state

### Requirement: EU-FIN-026 Evidence tier and private boundary

WHEN a family result is reported, THEN the matrix SHALL retain distinct source, admission, compiled UPLC, native, and private predicates with immutable provenance.

#### Scenario: EU-FIN-026 positive

- **WHEN** Stage00 public atomic wallet-transfer has preserved native receipts
- **THEN** only its exact predicate is credited and all twenty financial family native gaps remain explicit

#### Scenario: EU-FIN-026 failure

- **WHEN** OpenSpec validation, oracle templates, a mutable custody module, or public Cardano success is offered as private or Midnight release proof
- **THEN** the promotion is refused and private, PCD, Complete, and mandatory Midnight release gates remain unverified
