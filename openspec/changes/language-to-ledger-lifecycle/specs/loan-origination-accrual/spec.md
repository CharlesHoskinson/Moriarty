## ADDED Requirements

### Requirement: Explicit funded origination and authority boundary
The supported successor profile SHALL create obligations through a protected Originate operation, not through ordinary next writes or edited input fixtures. Origination SHALL bind a unique obligation ID, debtor, creditor, nominal denomination, settlement asset, principal, conversion, allocation policy and bounded accrual terms. Disbursement SHALL consume one matching actual Transfer funding record. It SHALL not reuse funding or originate unsigned additional liability at the authenticated acceptance boundary. Local supplied terms SHALL be labeled unauthenticated until that boundary is implemented.

#### Scenario: Funded principal creation
- **WHEN** a lender transfers100 Cash to the borrower and authorized origination binds principal100
- **THEN** one obligation SHALL exist with principal100, accrued0, outstanding100 and retained funding identity.

#### Scenario: Unbacked or duplicate origination
- **WHEN** funding is missing, mismatched, already consumed, or the obligation ID already exists
- **THEN** origination SHALL reject with no balance, allowance, obligation or identity mutation.

### Requirement: Bounded integer interest with explicit rounding
Accrual SHALL use finite nonnegative integer/rational terms and an explicit floor or ceiling rounding mode. The first supported policy SHALL use simple interest on current principal for one exact declared period. The formula SHALL be floor or ceiling of principal times numerator divided by denominator. Denominator SHALL be positive. Intermediate and output bounds SHALL reject overflow before truncation. Compounding, day-count conventions and generalized schedules SHALL remain unsupported until separately specified.

#### Scenario: Distinguishing rounding
- **WHEN** principal101 accrues at rate1/10 for one period
- **THEN** floor SHALL accrue10 and ceiling SHALL accrue11, with no cash movement.

#### Scenario: Invalid arithmetic
- **WHEN** denominator is zero, terms are negative, intermediate arithmetic exceeds bounds, or output cannot be represented
- **THEN** the action SHALL reject atomically.

### Requirement: Monotone period and duplicate protection
Each obligation SHALL retain its last accepted period and next admissible boundary. Accrue SHALL name the exact obligation and period. Only the next eligible period SHALL succeed. Repeated, skipped, reversed or unauthorized periods SHALL reject. Accepted period identity and cumulative work SHALL survive repayment, continuation and serialization. Authenticated execution SHALL bind time eligibility to accepted ledger context; local timestamp arguments alone SHALL not establish that authority.

#### Scenario: Repeated period after repayment
- **WHEN** period1 accrues, a partial payment succeeds, and period1 is submitted again
- **THEN** the second accrual SHALL reject without changing debt or work state.

### Requirement: Whole-stack operation support
Originate and Accrue SHALL have source declarations, checked Core descriptors, executable kernel transitions, complete effects, CLI cases and independent expected results. Existing operation schemas SHALL remain protected. Profile and state-schema extensions SHALL be explicit. Financial POST reads SHALL inspect their resulting candidate state.

#### Scenario: Hidden state forgery
- **WHEN** ordinary next assignments attempt to create principal, reset an accrual cursor or delete an obligation
- **THEN** they SHALL not alter the financial projection or satisfy a financial postcondition.
