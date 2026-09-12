## ADDED Requirements

### Requirement: Per-item specification approval before execution
For each item, the orchestrator SHALL write its concrete missing predicate, current affected producer/consumer paths, applicable constraints, independently checkable positive/rejection outcomes and completion evidence in OpenSpec before implementation or actionful execution. Fresh non-author Opus and Astra medium SHALL independently approve the exact full applicable specification bytes, with substantive verdicts retained in existing session/deliverable records. Both approvals are required; specification approval SHALL NOT replace source approval, resource authority, live admission or actual-result approval. Routine read-only inspection and specification authoring may precede this gate.

#### Scenario: Approved current item
- **WHEN** both reviewers approve the same exact specification revision and the item-specific prerequisites hold
- **THEN** Grok may implement that item, or the orchestrator may execute its authorized action through the applicable guarded path, and the result remains pending actual verification.

#### Scenario: Missing or changed specification
- **WHEN** an item lacks requirements, either auditor is unavailable or has a blocking finding, or relevant requirement bytes change after review
- **THEN** implementation/execution for that item does not start until both fresh current specification approvals exist; old verdicts remain history.

### Requirement: Item 1 — Current binding identity with preserved history
The repair SHALL trace the real binding/candidate producer and `records.py` consumers and create a reviewed bounded current I2 binding/action/admission using the existing record family. It SHALL inspect changed SP01 prerequisite inputs and determine their actual applicability without rewriting immutable historical bindings, hashes, source approvals or failure receipts. Historical SP01 rehashing SHALL NOT substitute for a current I2 action. The selected action, campaign, stage, profile, worktree, authority, command and full executable/source closure SHALL refer to the same applicable current candidate. Historical design approval SHALL remain design-scoped.

#### Scenario: Legitimate current binding
- **WHEN** a reviewed current record binds all applicable current inputs and owned files, with valid stage/profile lineage and current source reviews
- **THEN** the existing reader accepts binding/candidate currentness for that scope, while historical records retain their original bytes and wider acceptance remains unchanged.

#### Scenario: Stale or mismatched identity
- **WHEN** a bound input changes, a candidate/command/worktree/campaign/profile is mismatched, or an old approval is presented for changed bytes
- **THEN** the dependent action is rejected before launch; copying a current hash into a historical receipt does not repair admission.

### Requirement: Item 1 — Genuine cumulative accounting and live observations
The repair SHALL identify the existing authoritative accounting producer or missing invocation and connect its real observations to the path consumed by `currentAccounting`. A manually fabricated shape-valid snapshot SHALL NOT satisfy this requirement. Any narrowly necessary producer/consumer repair SHALL include its actual caller and evidence of the defect. The current accounting family SHALL remain closed and preserve authority/package identity, all applicable historical charges, reservations, amendments, worker counts, and externally charged package use. The actual master budget SHALL be reconciled as well as the package/allocation budget; a new package SHALL NOT conceal master exhaustion. Any master increase SHALL require an explicit reviewed bounded amendment under standing authority, never a guessed replacement limit. The existing runner SHALL require the exact single debit for the admitted action/candidate/runner before launch; a refresh SHALL NOT duplicate a debit or reset consumed attempts.

Existing allocation/admission evidence SHALL bind the public Preview network, wallet/role identity, current observation time and source, applicable contract state, available resources, attempt counters and ceilings to the proposed action. The admission check SHALL refresh these observations immediately before dispatch and reject a changed identity or resources that no longer cover the admitted maximum. Do not put secrets in accounting or add unrecognized wallet fields to `ACCOUNTING_KEYS`. Missing/ambiguous observations SHALL remain unavailable. Reservations, native fee reservations, actual paid fees and available balances SHALL be distinguished, including unresolved unit conversions. Refunds SHALL NOT erase gross debits; fees SHALL count against net goals.

The actual producer/admission path SHALL verify the reviewed amendment chain and enforce aggregate master and per-package dispatch ceilings even where the current reader omits those checks. It SHALL charge each allocation envelope once, preserve package ownership, serialize budget updates under the existing lock and atomically persist pre-launch debits/counters. Repeated reconciliation SHALL NOT create another charge for the same recorded attempt. Preserve the original authority identity; extend limits only by the explicit reviewed amendment. Mutable accounting SHALL remain outside immutable candidate/binding input hashes, with the exact debit still bound to the action/candidate/runner.

#### Scenario: Reconciled admissible resources
- **WHEN** genuine observations and complete applicable history establish remaining master/package/allocation resources, wallet funding and unconsumed explicitly authorized attempts sufficient for the exact bounded action
- **THEN** the producer supplies current accounting, independent reviewers can recompute the totals from retained source records, and existing admission may accept the resource predicate without inventing credit.

#### Scenario: Invalid or exhausted resources
- **WHEN** observations are missing/stale, wallet/network/campaign identity differs, history is partial, an allocation is consumed, master/package dispatches are exhausted, an amendment is unreviewed, an envelope is double-counted, funds/time are insufficient, or the runner debit is absent, duplicated or mismatched
- **THEN** live admission fails before launch, retains the specific cause and consumed totals, and no replacement identity or retry silently restores credit.

#### Scenario: Clock authority mistaken for spend authority
- **WHEN** the eight-hour block or an old reservation is offered as permission for another loan submission
- **THEN** the allocation remains consumed until a separate explicit bounded amendment receives the required substantive votes and the current admission checks pass.

### Requirement: Item 1 — Operational history is evidence
The repair SHALL resolve history from retained real terminal outcomes and containment evidence. A software parser/producer defect SHALL be reproduced before repair. Unknown outcomes SHALL NOT become successful exits or verified zero-failure counts. Two failures of the same defect class SHALL require reproduction and a changed approach before another broad retry.

#### Scenario: Recoverable terminal record
- **WHEN** retained process identity, terminal outcome and containment establish an outcome that a reproduced parser defect had missed
- **THEN** the repaired consumer reports that outcome and keeps the original record and reproducer.

#### Scenario: Unavailable raw exit
- **WHEN** the historical loan has financial output but no retained raw main exit before unit unload
- **THEN** the raw-exit predicate stays unmet; neither output text nor an administrative history reset establishes success.

### Requirement: Item 2 — Complete callable loan lifecycle
Grok SHALL connect the real bounded executor to `retain_loan_main_exit`, its durable writer, exit selection and explicit cleanup owner. The callable path SHALL preserve startup invocation identity, poll running-to-terminal within the original deadline, and durably retain the real main-process terminal observation before deliberate stop. Success SHALL require raw main exit zero observed before stop and separate verified outer containment. The process SHALL NOT be stopped merely to manufacture terminal success. A stopped incomplete draft or helper-only packet SHALL NOT be installed as the complete path.

#### Scenario: Successful callable invocation
- **WHEN** the same launched invocation transitions from running to exited with raw exit zero, terminal evidence is fsynced before stop, and independent outer containment succeeds
- **THEN** the caller retains those ordered facts and returns a scoped successful process result pending financial/result audits.

#### Scenario: Lifecycle fault matrix
- **WHEN** offline injections through the actual caller exercise malformed startup/process identity, invocation mismatch, preexisting stop evidence, nonzero main exit, terminal write/fsync failure, stop nonzero/exception/timeout, deadline crossing or invalid containment ordering
- **THEN** every case has an independently specified non-success outcome and an explicit cleanup owner; terminal persistence failure does not authorize an evidence-driven premature stop, and the original outer timeout/containment procedure remains responsible for cleanup.

#### Scenario: Stop succeeded but its receipt cannot persist
- **WHEN** an actual successful stop is followed by stop-receipt write/fsync failure
- **THEN** the result preserves actual stop success and persistence failure as separate observations, does not label the stop itself failed, and does not grant full successful evidence retention.

### Requirement: Item 3 — Exact live admission and one bounded invocation
Before dispatch, the orchestrator SHALL freeze the full executable/runtime closure and obtain both fresh concurrent non-author source audits of the exact candidate, then refresh wallet/resource observations and demonstrate applicable guarded admission. The registered live action SHALL bind its actual command, candidate, reviewed explicit allocation, debit and limits. SP01 syntax or SP05 local-runtime approval SHALL NOT admit public execution. Registered campaign dispatch SHALL use the existing plugin `run --action` path; absent mapping or unsupported admission must be repaired within this item, never bypassed. Source changes after review SHALL require both new source audits and renewed admission.

#### Scenario: Admitted loan run fits the deadline
- **WHEN** all item 1 and item 2 predicates hold, both source audits pass, one new bounded loan invocation is explicitly admitted, and its maximum run, cleanup and both result reviews fit before `2026-09-11T14:46:18Z`
- **THEN** the guarded action may run once, retaining raw process evidence, before-state, native transaction bytes, complete effects and public submission/finality observations; all attempts and costs accrue to existing history.

#### Scenario: Denied or late launch
- **WHEN** admission is missing/stale, a reviewer is unavailable, only a historical design action is eligible, the block has reached T0 + 6.5 hours, or the complete run/cleanup/review bound cannot fit
- **THEN** no new live launch occurs; retain the concrete blocker and mark the execution objective incomplete. A continuation does not reset the deadline or allocation.

#### Scenario: Public transaction notification
- **WHEN** an actual public transaction submission or later status is observed
- **THEN** record the observation through the existing notify procedure, emit its transaction ID and observed status in the conversation before a network summary, and use deliver only after the host-visible message exists; unresolved finality remains explicit.

### Requirement: Item 4 — Actual financial result and original MC02 acceptance
Both fresh selected auditors SHALL independently review the full exact actual-result evidence after independently recomputable native-byte decoding, complete-effect comparison, indexed success, canonical node finality and exact state readback. Source votes SHALL NOT count as result votes. Compare the frozen loan fixture's token/domain, roles, dues, recipients, transfers, all debits/credits, fees, approvals and writes, preserving residual duties and inner-incomplete flags. Preserve the original MC02 manifest provenance and reconcile each original positive/rejection criterion once in existing records. Reuse accepted swap evidence only after checking scope, bytes and lineage; excluded distinct-counterparty funding/signatures SHALL remain a gap until applicable evidence resolves it.

#### Scenario: Newly accepted loan predicate
- **WHEN** the new invocation has retained raw exit zero, independent containment, complete decoded financial effects matching independent local expectations, required finality/readback and both current substantive result approvals
- **THEN** record precisely the newly accepted financial predicate, with actual costs and mandatory proof claims unavailable until MC05; aggregate I2 remains open unless every original criterion is covered.

#### Scenario: Incomplete or invalid effects
- **WHEN** comparison finds a wrong recipient/domain, excess fee, missing debit, extra approval, undeclared write, missing role/funding evidence, or a required rejection that mutates state or lacks evidence
- **THEN** result acceptance fails for that predicate and aggregate promotion stays blocked, even if network submission or process exit succeeded.

#### Scenario: Unsupported promotion
- **WHEN** old loan success output, a narrower swap, local tests, source approval or network success is offered as raw-exit evidence, full I2/SP05 completion, compiler correspondence, history certification or mandatory PCD
- **THEN** reject that claim while retaining any independently approved narrower financial result.

### Requirement: Item 5 — Reviewed publication and honest handoff
The orchestrator SHALL publish only reviewed owned changes under standing GitHub authority, preserve unrelated dirty work and immutable history, and exclude credentials/private witness material. Final records SHALL report actual resource use, completed predicates, unresolved gaps, process containment and one next executable action in the existing session/acceptance/checkpoint locations. No new scheduler or acceptance register SHALL be added. A loop SHALL be reported armed only after current live runtime confirms that state.

#### Scenario: Publish and close the interval
- **WHEN** reviewed owned bytes and scoped results are ready before the deadline
- **THEN** publish those bytes, retain review/result references and the six existing session measurements, verify cleanup, and leave a precise continuation without marking the full-roadmap goal complete.

#### Scenario: Incomplete execution or unsafe publication set
- **WHEN** live execution did not occur or failed, finality/audits/containment remain unresolved, or the publication set contains unreviewed/unrelated/private material
- **THEN** retain reviewed repairs and exact failures, mark the block's execution objective incomplete, and exclude the affected publication material until resolved.

### Requirement: Item 6 — Optional bounded F0 diagnosis
Only after binding/accounting readiness and with noncompeting independent capacity, the orchestrator MAY perform at most thirty minutes of read-only F0 diagnosis. Its specification SHALL name the source/interface question and required observations before work. The finding SHALL identify the actual outer verifier, transcript, VK, carried accumulator, constrained finalizer, SRS/export owner and deployment provenance, or state the exact missing interface/fit blocker. It SHALL NOT authorize larger k, a proof campaign or a new backend. Consequential GO/resource decisions retain existing independent votes and admission.

#### Scenario: Scoped source finding
- **WHEN** the bounded inspection establishes a source route or locates a missing required interface
- **THEN** record that source-supported finding in the existing F0 record with its limitations; no native/PCD acceptance follows from source inspection.

#### Scenario: Competing or inconclusive lane
- **WHEN** F0 would delay the loan path, the thirty-minute limit is reached, or essential fit/interface evidence remains absent
- **THEN** drop or stop the lane, keep dependent proving stopped and retain the precise blocker without filling the remaining clock with new packets.
