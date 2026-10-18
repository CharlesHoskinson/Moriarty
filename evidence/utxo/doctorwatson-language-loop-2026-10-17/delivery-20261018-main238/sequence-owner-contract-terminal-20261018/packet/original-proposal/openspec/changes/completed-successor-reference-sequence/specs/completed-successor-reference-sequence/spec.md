## ADDED Requirements — proposed, not adopted

These ordinary success-contract clauses apply only after adoption and the applicable designated-owner contracts. They do not define new source/admission/identity/refusal/error/security rules. Existing original clauses and owner obligations remain verbatim in the pinned basis. All scenarios below are ordinary positive observations, not hostile/security tests or native acceptance evidence.

### Requirement: CSQ-001 Owned completed predecessor
WHEN a planned action completes and its successor is requested, the production Sequence SHALL start the successor from its actual retained complete predecessor head, preserving all earlier original outcomes and without accepting caller-supplied successor financial state or prior accounting.

#### Scenario: Two distinct successful programs
- **WHEN** FirstPayment completes with phase1, invoice6 and its original transfer/fee/discharge history, and SecondPayment is requested
- **THEN** SecondPayment's projected opening SHALL use those actual values, its source initializer SHALL NOT reset the mapped state, and the report SHALL retain the complete FirstPayment prefix.

#### Scenario: More than two invocations
- **WHEN** an additional supported program is requested after the second completes
- **THEN** the same production operation SHALL use the current retained head and prefix without an application-specific branch or a new initial budget.

### Requirement: CSQ-002 Complete framed state
WHEN a real stage invocation observes a selected local view, the production adapter SHALL preserve its complete global outside frame and retain both the original local snapshots and every lifted global Step and boundary snapshot.

#### Scenario: Different state schemas and unrelated rows
- **WHEN** P.phase and Q.progress map to one global payment.phase, Q has another declared state, and the opening also contains unrelated state, another exact asset, observer balances and an unselected duty
- **THEN** the projected values SHALL follow their explicit maps and every untouched global row SHALL remain present and equal at each whole Step/head observation.

### Requirement: CSQ-003 Sequential shared financial domain
WHEN sequential supported actions use the same exact asset/account/duty domain, the ordinary sequence SHALL carry that domain along its declared serial order through actual stage calls rather than applying the existing Pair's disjoint operator rule or treating two independent starts as succession.

#### Scenario: One asset, two source programs
- **WHEN** two different programs pay the same recipient from the same payer and explicitly discharge the same mapped invoice
- **THEN** the second program SHALL consume the first program's retained balance/duty head and the complete report SHALL name both original source programs and original call histories.

### Requirement: CSQ-004 Typed duty evolution and residuals
WHEN an observed source action creates, increases or explicitly discharges a liability, the sequence observation SHALL retain the full identity, opening/closing presence and amount, original source/effect references and complete residual set, including outside duties.

#### Scenario: Successful partial payments leave debt
- **WHEN** P creates12 and discharges6, then Q discharges3, while another duty5 is untouched
- **THEN** the complete final head SHALL retain invoice3 and outside duty5, the evolution SHALL be 0+12−6−3=3 with opening absence separately recorded, and local list completion SHALL NOT mean settlement.

#### Scenario: Present zero is observed
- **WHEN** an admitted source action explicitly discharges a present liability to zero
- **THEN** the original zero-valued row and present:true observation SHALL remain distinct from an absent opening/closing row; no effect or row SHALL be deleted by the composition reporter.

### Requirement: CSQ-005 Original complete history
WHEN the sequence reports a prefix, it SHALL retain every original invocation envelope and every segment/Execution/Step/statement/ExprId/value/snapshot/frame/capture/continuation/effect/residual/work field without replacing earlier segments by the final segment.

#### Scenario: An awaiting predecessor completes
- **WHEN** a supported predecessor has start and resume observations before completion
- **THEN** Q's later report SHALL retain all those original observations, their original local labels and work, and the exact whole-head prefix after each call.

### Requirement: CSQ-006 Ordered complete effects
WHEN a call appends source effects, the sequence SHALL preserve invocation-then-source order, original Step/Execution attachment, repetitions, zero/self effects and the explicit global name image.

#### Scenario: Repeated financial effects
- **WHEN** two programs emit similar Transfer/Fee/Emit effects under their own original statements
- **THEN** the global table SHALL retain every separate effect in order and link it to the complete original call/Step/effect rather than deduplicating or netting it.

### Requirement: CSQ-007 Gross debit and fee observations
WHEN an ordinary prefix is reported, its complete financial summary SHALL separately account for each exact asset/account's transfers, fees, gross debit/credit, opening/closing balance and net-change direction, derived from the complete observed effect prefix.

#### Scenario: Later credit follows spending
- **WHEN** two programs incur alice gross debit12 including fee3 and a later supported program transfers1 back to alice
- **THEN** alice gross debit SHALL remain12 and fees paid3 while gross credits become1 and net decrease becomes11; the other payer's debit and all original effects SHALL remain separately visible.

### Requirement: CSQ-008 Original local and cumulative work
WHEN the sequence appends a normal stage observation, it SHALL retain its original attempted/local committed/cap fields and separately report sums of attempted work per call and latest committed work per action instance without resetting earlier consumption on successor start.

#### Scenario: A successor begins at local zero
- **WHEN** P commits12 and Q's actual successful action commits14
- **THEN** original local work SHALL remain12 and14 and sequence committed/attempted work SHALL be26 for this all-advancing example, rather than14 or38 from summing repeated cumulative fields.

#### Scenario: Nonadvancing observation
- **WHEN** the current API returns successful Waiting(advanced:false) for a supported false guard
- **THEN** the sequence SHALL retain that original segment:null outcome, add its actual attempted observation, and preserve the exact continuation/global head/frame/effect prefix and successful committed prefix.

### Requirement: CSQ-009 Explicit ordinary/reserve observation
WHERE a successful ordinary trace fits its declared logical envelope, the sequence SHALL retain its declared ordinary allocation and separate closure-reserve amount, show consumed/remaining ordinary work, and leave the reserve unchanged by ordinary calls.

#### Scenario: A visible unspent reserve survives succession
- **WHEN** an ordinary allocation40 and closure reserve7 accompany the two successful actions totaling26 logical attempted work
- **THEN** the ordinary remaining observation SHALL be14, the reserve observation7, and no successor SHALL reset the cumulative consumed observation to its own local work.

This clause specifies success-trace arithmetic only. Global enforcement, failure charges, reserve authority, numeric limits and target resource adequacy remain designated-owner contracts and cannot be inferred from this scenario.

### Requirement: CSQ-010 Actual pending continuation
WHEN a sequence action waits and later resumes, its production path SHALL use the exact original local continuation with the real existing stage API while preserving the complete outside frame; it SHALL begin no successor until that action actually completes.

#### Scenario: Waiting is visible before succession
- **WHEN** a supported P waits for an event, then advances under the declared normal event input
- **THEN** its original waiting/resume frames SHALL remain observable, the outside frame SHALL be preserved throughout, and Q SHALL first start from the subsequent actual completed P head.

### Requirement: CSQ-011 One real public production path
WHEN an external developer invokes the proposed ordinary compose command or its test-manifest adapter, both SHALL use the same production Sequence/loader/driver over actual loaded programs and stage operations; expected observations SHALL never supply runtime state or accounting.

#### Scenario: Public compose without expected files
- **WHEN** a new finite plan of supported sources, complete opening, inputs, bindings and operations is supplied without any expected-output path
- **THEN** the actual public compose command SHALL execute its local source sequence and print the complete run report through the production path.

### Requirement: CSQ-012 Complete observations and public bytes
WHEN a normal sequence report is serialized or compared, it SHALL preserve every complete root/catalog/plan/call/effect/prefix/head/frame/financial/logical/residual field defined by WIRE-CONTRACT.md, with exact run-local prefix attachments and a final LF in public output.

#### Scenario: Full independent comparison
- **WHEN** complete expectations and consumer-specific byte goldens were independently frozen before the real public run
- **THEN** the binary regression SHALL compare the full report/bytes and every complete expanded prefix, including all preserved rows and original nested reports; matching only final balances SHALL be insufficient evidence for this predicate.

### Requirement: CSQ-013 Local status and evidence boundary
WHEN local planned programs finish, wait or remain between invocations, the report SHALL expose that exact local progress and keep it separate from external payment/delivery/finality, native proof, private handoff and ledger acceptance claims.

#### Scenario: Positive residual after local list completion
- **WHEN** both selected source programs complete while invoice3 and outside duty5 remain
- **THEN** local progress SHALL be complete with the full positive residuals still present and no whole-obligation settlement claim.

### Requirement: CSQ-014 Other operators remain separate
WHEN this ordinary sequential capability is reported, it SHALL retain separate dispositions for sequential, disjoint parallel, shared-state interleaving, atomic synchronization and asynchronous messaging, including full native/history/global-resource/private/target requirements.

#### Scenario: Sequential local observation
- **WHEN** the proposed public two-program sequence later has its named positive predicates observed
- **THEN** only those named ordinary child predicates SHALL receive evidence; Pair requirements, SI1–SI4, atomic Join/domain semantics, asynchronous outcomes/late duties and parent MC06/SP10/U4 exits SHALL remain independently required.

### Requirement: CSQ-015 Independent expectation provenance
WHEN a source or result review uses financial/work or public-wire expectations, it SHALL retain their independent pre-run derivation, exact source/semantic/codec premises and prior source exposure; authored arithmetic or codec agreement SHALL NOT be relabeled an independent compiler-wire oracle.

#### Scenario: This proposal's worked example
- **WHEN** a custodian receives the adopted plan/source inputs and this authored contract
- **THEN** it SHALL derive the complete values/Program/Plan/prefixes/bytes independently of production evaluation, declare exposure and authored-codec conditions, and freeze the exact expected artifacts before any consumer comparison.

### Requirement: CSQ-016 Additive compatibility
WHEN the new ordinary sequence path is integrated, existing Core/1/Core/2 stage APIs, original stage DTOs/digests/admission/costs, disjoint Pair and all original fixture/oracle/result records SHALL retain their exact selected behavior and qualifications unless a separately scoped reviewed extension explicitly changes them.

#### Scenario: New public profile beside existing consumers
- **WHEN** a sequential-proposed manifest is selected
- **THEN** it SHALL use the new production operation without relabeling old disjoint schedules as sequential, changing old profiles or weakening whole-program disjoint requirements.
