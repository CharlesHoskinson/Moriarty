## ADDED Requirements

### Requirement: EU-NAT-001 Exact candidate admission

WHEN a native run starts, the runner SHALL verify current implementation, design, helper, source, and tool hashes against NativeRunManifest/1.

#### Scenario: EU-NAT-001 positive
- **WHEN** matching frozen files and two current implementation reviews
- **THEN** the runner shall admit the exact candidate

#### Scenario: EU-NAT-001 negative
- **WHEN** one changed compiler file or an absent implementation review
- **THEN** the runner shall stop before resource creation

### Requirement: EU-NAT-002 Task resource isolation

WHILE a native run owns resources, the runner SHALL use only its fresh local network, wallets, policies, processes, and free ADA.

#### Scenario: EU-NAT-002 positive
- **WHEN** exclusive ownership and available reserved ports
- **THEN** the runner shall create only the task namespace

#### Scenario: EU-NAT-002 negative
- **WHEN** an occupied port, foreign process, public endpoint, or user wallet
- **THEN** the runner shall stop without reset or submission

### Requirement: EU-NAT-003 Secret isolation

WHEN task wallets are generated, the runner SHALL store keys in mode-0700 private storage and expose only public identities.

#### Scenario: EU-NAT-003 positive
- **WHEN** fresh payment keys and redacted process output
- **THEN** the runner shall publish verification keys and key hashes only

#### Scenario: EU-NAT-003 negative
- **WHEN** a command would print a seed or private key
- **THEN** the runner shall refuse the command and preserve a redacted failure

### Requirement: EU-NAT-004 Authentic runtime pins

WHEN policies and keys exist, the runner SHALL derive RuntimePins/1 from actual scripts, vkeys, genesis, and ledger outputs.

#### Scenario: EU-NAT-004 positive
- **WHEN** compiled distinct policies and actual vkeys
- **THEN** the runner shall recompute instance identity and accept authentic pins

#### Scenario: EU-NAT-004 negative
- **WHEN** zero ids, shape samples, equal policies, stale genesis, or unmeasured ids
- **THEN** the runner shall refuse native compilation

### Requirement: EU-NAT-005 Policy hash dependency

IF pin construction contains a compile-policy hash cycle, THEN the runner SHALL stop with PIN_DEPENDENCY_CYCLE and retain the dependency graph.

#### Scenario: EU-NAT-005 positive
- **WHEN** an acyclic reviewed policy plan
- **THEN** the runner shall compile in dependency order

#### Scenario: EU-NAT-005 negative
- **WHEN** spending and minting scripts require each other’s final hash
- **THEN** the runner shall refuse to replace hashes with labels or samples

### Requirement: EU-NAT-006 Genesis and POSIX parity

WHEN time calibration starts, the runner SHALL compare genesis-derived CLI, SDK, and V3 ScriptContext POSIX values before substitution.

#### Scenario: EU-NAT-006 positive
- **WHEN** slotLengthMs equals 1000 and all three measured POSIX values match
- **THEN** the runner shall persist NativeTimeWitness/1

#### Scenario: EU-NAT-006 negative
- **WHEN** slot length differs, a value is negative, or parity differs
- **THEN** the runner shall refuse calibration

### Requirement: EU-NAT-007 Finite exclusive interval

WHEN a native validity window is planned, the runner SHALL use sixty slots and an exclusive upper bound with last-millisecond Args.

#### Scenario: EU-NAT-007 positive
- **WHEN** a 30-slot inclusion margin and 150-second lookahead within 1800 seconds
- **THEN** the runner shall use [L,P) and Args L/P-1

#### Scenario: EU-NAT-007 negative
- **WHEN** overflow, skew, reversed endpoints, infinity, or an unrepresentable slot boundary
- **THEN** the runner shall refuse the window before signing

### Requirement: EU-NAT-008 Guard-only calibrated sibling

WHEN native parity passes, the runner SHALL derive a distinct source sibling by replacing only the three approved guard literals.

#### Scenario: EU-NAT-008 positive
- **WHEN** a guard-only AST difference and recomputed source/program hashes
- **THEN** the runner shall retain original evidence and calibrated evidence separately

#### Scenario: EU-NAT-008 negative
- **WHEN** a changed amount, role, work budget, ABI, or unchanged source hash
- **THEN** the runner shall refuse the sibling

### Requirement: EU-NAT-009 Source logical correspondence

WHEN calibrated source is checked, the runner SHALL compare full evaluator envelopes under the original-to-calibrated time mapping.

#### Scenario: EU-NAT-009 positive
- **WHEN** identical ordinary posts, financial posts, effects, work, and failure classes
- **THEN** the runner shall record logical equivalence within the mapped interval domain

#### Scenario: EU-NAT-009 negative
- **WHEN** a hardcoded post or a changed non-time result
- **THEN** the runner shall stop native construction

### Requirement: EU-NAT-010 Full body and context binding

WHEN a signed transaction is ready, the runner SHALL compare decoded body fields with the V3 context and independent case inputs.

#### Scenario: EU-NAT-010 positive
- **WHEN** matching inputs, references, outputs, datum, redeemer, signatories, mint, fee, and validity
- **THEN** the runner shall persist body-context checks

#### Scenario: EU-NAT-010 negative
- **WHEN** body-derived values differ from the script context or source arguments
- **THEN** the runner shall refuse submission

### Requirement: EU-NAT-011 Disjoint fee and collateral

WHEN native transactions are built, the runner SHALL allocate separate fee and collateral UTXOs to each transaction.

#### Scenario: EU-NAT-011 positive
- **WHEN** two concurrent transactions share only the head
- **THEN** the runner shall record pairwise-disjoint auxiliary input sets

#### Scenario: EU-NAT-011 negative
- **WHEN** the pair shares fees, collateral, or a fee equals collateral
- **THEN** the runner shall refuse both submissions

### Requirement: EU-NAT-012 Receipt preimage binding

WHEN submission is requested, the runner SHALL persist signed CBOR and a valid CLI-derived 64-hex TxId before provider invocation.

#### Scenario: EU-NAT-012 positive
- **WHEN** CLI11 decodes the CBOR and persistence succeeds
- **THEN** the runner shall invoke the pinned revision02 helper once

#### Scenario: EU-NAT-012 negative
- **WHEN** invalid CLI hash, decode failure, namespace failure, collision, or write failure
- **THEN** the runner shall invoke no provider callback

### Requirement: EU-NAT-013 Returned identifier binding

WHEN a provider returns an identifier, the runner SHALL compare it with the persisted CLI identifier.

#### Scenario: EU-NAT-013 positive
- **WHEN** identical normalized identifiers
- **THEN** the runner shall enter confirmation polling

#### Scenario: EU-NAT-013 negative
- **WHEN** a returned identifier differs or is malformed
- **THEN** the runner shall record transport-binding-failure without native-invalidity inference

### Requirement: EU-NAT-014 Owned confirmation deadline

WHILE confirmation is pending, the runner SHALL race each poll against its remaining owned deadline.

#### Scenario: EU-NAT-014 positive
- **WHEN** a confirmation observation arrives before the deadline
- **THEN** the runner shall record the observation and its provenance

#### Scenario: EU-NAT-014 negative
- **WHEN** a late response, hanging poll, or deadline exhaustion
- **THEN** the runner shall persist confirmation-timeout without later mutation

### Requirement: EU-NAT-015 Ledger outcome classification

WHEN a transaction observation is recorded, the runner SHALL separate transport status, evaluation, phase-one rejection, phase-two rejection, and inclusion.

#### Scenario: EU-NAT-015 positive
- **WHEN** node rejection or block/readback evidence supports the category
- **THEN** the runner shall retain the observed class and raw public evidence

#### Scenario: EU-NAT-015 negative
- **WHEN** only callback success, provider evaluation, or no observation exists
- **THEN** the runner shall leave native acceptance false and finality unknown

### Requirement: EU-NAT-016 Correctly signed semantic negatives

WHEN a phase-two negative is tested, the runner SHALL remove stale witnesses and sign the mutated body with task keys.

#### Scenario: EU-NAT-016 positive
- **WHEN** valid witnesses and an isValid-true semantic failure
- **THEN** the runner shall require node phase-two rejection and unchanged head/fee/collateral

#### Scenario: EU-NAT-016 negative
- **WHEN** only InvalidWitnessesUTXOW or evaluation failure occurs
- **THEN** the runner shall record control-inconclusive instead of semantic rejection

### Requirement: EU-NAT-017 Invalid-tag collateral behavior

WHEN an isValid-false failing script is included, the runner SHALL measure collateral consumption and collateral return independently.

#### Scenario: EU-NAT-017 positive
- **WHEN** a failing script is included under isValid false
- **THEN** the runner shall require unchanged head and measured totalCollateral/return

#### Scenario: EU-NAT-017 negative
- **WHEN** an isValid-true submission rejects before inclusion
- **THEN** the runner shall require unspent collateral and reject any consumption claim

### Requirement: EU-NAT-018 Complete financial readback

WHEN a native case concludes, the checker SHALL compare complete ledger state and effects with the independent evaluator envelope.

#### Scenario: EU-NAT-018 positive
- **WHEN** all state fields, authority, nonces, work, balances, allocations, and output quantities match
- **THEN** the runner shall record native case acceptance

#### Scenario: EU-NAT-018 negative
- **WHEN** one omitted field, missing payout, reused allocation, or partial send differs
- **THEN** the runner shall record failure without settlement acceptance

### Requirement: EU-NAT-019 Authenticated state families

WHERE state-family compilation is supported, the runner SHALL execute S01 through S04 positive and counterfeit-state controls.

#### Scenario: EU-NAT-019 positive
- **WHEN** genesis, continuation, split/join, migration, and termination authenticate
- **THEN** the runner shall retain source-specific native evidence

#### Scenario: EU-NAT-019 negative
- **WHEN** forged genesis, consumed reference, duplicated authority, or discarded duty
- **THEN** the runner shall require rejection and unchanged protected state

### Requirement: EU-NAT-020 Escrow and observation families

WHERE escrow-family compilation is supported, the runner SHALL execute escrow, interval, joint-role, and authenticated observation controls.

#### Scenario: EU-NAT-020 positive
- **WHEN** release/refund/cancel and full-window observation checks succeed
- **THEN** the runner shall retain complete custody readbacks

#### Scenario: EU-NAT-020 negative
- **WHEN** missing role, wrong beneficiary, stale feed, future update, or crossed interval
- **THEN** the runner shall require correctly signed rejection

### Requirement: EU-NAT-021 Swap and batch families

WHERE swap-family compilation is supported, the runner SHALL execute bilateral, pool, order, and two-order allocation controls.

#### Scenario: EU-NAT-021 positive
- **WHEN** complete paired payouts and serialized pool continuation match source
- **THEN** the runner shall record complete economic effects

#### Scenario: EU-NAT-021 negative
- **WHEN** slippage, rounding mismatch, shared payout, unmatched input, or timeout misuse
- **THEN** the runner shall require rejection without partial settlement

### Requirement: EU-NAT-022 Financial operation families

WHERE financial compilation is supported, the runner SHALL execute F01 through F04 and AP01 with full operation policies.

#### Scenario: EU-NAT-022 positive
- **WHEN** origination, accrual, partial repayment, settlement, and fee-inclusive bounds match
- **THEN** the runner shall record liabilities, residual duties, gross debit, and net delivery

#### Scenario: EU-NAT-022 negative
- **WHEN** duplicate accrual, repayment replay, cap violation, or fee-hidden net deficit
- **THEN** the runner shall require rejection and preserve obligations

### Requirement: EU-NAT-023 Nonce and authority families

WHERE authority compilation is supported, the runner SHALL execute AP02 and AP03 with authenticated instance nonce and actual signatures.

#### Scenario: EU-NAT-023 positive
- **WHEN** valid native role signatures and unused durable nonce
- **THEN** the runner shall record authority consumption and the continuation nonce

#### Scenario: EU-NAT-023 negative
- **WHEN** wrong signer, signature stripping, reused nonce, or another instance identity
- **THEN** the runner shall require rejection without authority transfer

### Requirement: EU-NAT-024 Unsupported private refusal

WHEN AP04 or an unsupported native family is requested, the runner SHALL retain the exact admission refusal without native submission.

#### Scenario: EU-NAT-024 positive
- **WHEN** private certificate, missing key, incompatible schema, or unsupported profile
- **THEN** the runner shall record refusal with nativeAcceptance false

#### Scenario: EU-NAT-024 negative
- **WHEN** a host flag or synthetic proof bypasses the refusal
- **THEN** the runner shall fail the case and preserve the missing dependency

### Requirement: EU-NAT-025 Conflict and reference execution

WHEN competing spends or reference reads are tested, the runner SHALL distinguish concurrent submission, sequential replay, and reference-only access.

#### Scenario: EU-NAT-025 positive
- **WHEN** both competing transactions submit before confirmation and share only the head
- **THEN** the runner shall observe one included winner and one nonincluded branch

#### Scenario: EU-NAT-025 negative
- **WHEN** winner confirmation precedes loser submission or loser status is unknown
- **THEN** the runner shall label sequential or partial and prohibit concurrent-pass

### Requirement: EU-NAT-026 Measured protocol limits

WHEN a supported native case is measured, the runner SHALL retain actual protocol parameters, CBOR sizes, fees, collateral, and execution units.

#### Scenario: EU-NAT-026 positive
- **WHEN** measured values fit pinned limits with recorded margins
- **THEN** the runner shall record native measurement and deterministic reproduction inputs

#### Scenario: EU-NAT-026 negative
- **WHEN** a size, CPU, memory, min-ADA, fee, or collateral bound fails
- **THEN** the runner shall stop with measured failure instead of increasing limits

### Requirement: EU-NAT-027 Immutable reproduction and stop

IF two cycles repeat the same defect class, THEN the runner SHALL stop retries and retain one minimal reproducer.

#### Scenario: EU-NAT-027 positive
- **WHEN** a changed hypothesis and reviewed bounded repair resolves the defect
- **THEN** the runner shall resume only the affected case under existing authority

#### Scenario: EU-NAT-027 negative
- **WHEN** the next retry only widens timeouts or relabels evidence
- **THEN** the runner shall refuse the retry and preserve failures

### Requirement: EU-NAT-028 Twenty-family honest result matrix

WHEN a run report is produced, the checker SHALL emit all twenty inventory rows with independent source, admission, compiled, native, and private status.

#### Scenario: EU-NAT-028 positive
- **WHEN** each asserted native pass has exact source and ledger readbacks
- **THEN** the runner shall report only supported demonstrated predicates

#### Scenario: EU-NAT-028 negative
- **WHEN** a prototype, template, partial send, or unit fixture substitutes for a family
- **THEN** the runner shall leave that family unverified and reject all20-native claims
