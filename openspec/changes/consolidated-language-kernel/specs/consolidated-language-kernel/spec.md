## ADDED Requirements

### Requirement: UNI-001 Permissionless public pipeline
When a developer presents a supported program and valid required evidence, the public pipeline SHALL decide validity without project, registry, council, Foreman or hosted-service membership.

#### Scenario: Positive witness
- **WHEN** A clean developer authors and submits a novel supported program without project records.
- **THEN** Objective checks accept it under owner and ledger rules.

#### Scenario: Hostile witness
- **WHEN** Only a reviewer receipt is removed from an otherwise identical valid candidate.
- **THEN** Validity is unchanged; a receipt cannot substitute for a missing proof.

### Requirement: UNI-002 One semantic relation
When an accepted stage changes state, the implementation SHALL bind program, profile, signed intention, current authority, history, observations, complete effects, costs and residual duties through one versioned relation.

#### Scenario: Positive witness
- **WHEN** Two source constructs elaborate into the supported Core with checked representations.
- **THEN** Their target behaviors satisfy the same scoped relation.

#### Scenario: Hostile witness
- **WHEN** A legacy lowering path drops authority or a mandatory predicate.
- **THEN** The path cannot claim certified acceptance.

### Requirement: UNI-003 Native target and correspondence
When a program claims certified execution, the toolchain SHALL establish valid-execution completeness and adversarial-witness soundness for its pinned ZKIRv3 or explicitly qualified successor target and actual ledger interface.

#### Scenario: Positive witness
- **WHEN** A source-valid execution has a witness under the bound target relation.
- **THEN** Native verification and actual effects agree with its specified outcome.

#### Scenario: Hostile witness
- **WHEN** An adversarial witness omits a range or assertion constraint.
- **THEN** It is rejected or the missing soundness obligation prevents certification.

### Requirement: UNI-004 Intent and solver completion
When a solver completes or combines intentions, acceptance SHALL preserve every applicable signed constraint, including allowed choices, gross caps, fees, net outcomes, liabilities, disclosures and recovery.

#### Scenario: Positive witness
- **WHEN** Two independent solvers choose distinct permitted routes.
- **THEN** Both receive the same objective acceptance treatment.

#### Scenario: Hostile witness
- **WHEN** A plan changes recipient, fee asset or obligation consent while preserving net balance.
- **THEN** Acceptance rejects the unauthorized effect.

### Requirement: UNI-005 Complete financial state
When a stage commits effects, acceptance SHALL account separately for assets and authorized supply, gross debits and fees, authority consumption, liabilities and residual duties over the authenticated complete state domain.

#### Scenario: Positive witness
- **WHEN** A partial fill consumes a proportional authorized amount and retains its remainder.
- **THEN** Accounting and cumulative limits remain correct.

#### Scenario: Hostile witness
- **WHEN** Netting hides a fee or an omitted liability/reservation.
- **THEN** Acceptance rejects the incomplete or unauthorized accounting.

### Requirement: UNI-006 Phases and partiality
When a target permits retained effects after failure, the compiler SHALL enforce the signed phase-specific effects, fees, authority consumption and remaining duties without representing them as global rollback or workflow completion.

#### Scenario: Positive witness
- **WHEN** The fallible phase fails while an authorized guaranteed fee remains.
- **THEN** The result records that fee and unresolved duties.

#### Scenario: Hostile witness
- **WHEN** An uncommitted incomplete candidate is presented as a completed workflow.
- **THEN** No ledger completion or unconsented duty is inferred.

### Requirement: UNI-007 Conditional settlement
When delivery is conditional, acceptance SHALL require the authorized evidence combination and distinguish request recording, funding, eligibility, in-flight effects and actual delivery.

#### Scenario: Positive witness
- **WHEN** Recipient consent and the bound document predicate become valid for funded escrow.
- **THEN** The permitted release can execute and be recorded after actual delivery evidence.

#### Scenario: Hostile witness
- **WHEN** A document digest, funding receipt or mere timeout is substituted for delivery conditions.
- **THEN** Release or completion is rejected and unresolved duties remain.

### Requirement: UNI-008 Recovery and state limits
When a workflow reaches expiry, work or state limits, supported recovery or successor transitions SHALL preserve applicable consent, cumulative budgets, replay state and duties under their own authority and resource rules.

#### Scenario: Positive witness
- **WHEN** Ordinary work is exhausted but an authorized funded recovery path remains.
- **THEN** Only that path may consume the reserved closure work.

#### Scenario: Hostile witness
- **WHEN** Rollover resets a spent budget or expiry erases debt.
- **THEN** Acceptance rejects the reset or erasure.

### Requirement: UNI-009 Full history scope
When compliant history is claimed, evidence SHALL establish authenticated origin, compatible well-founded predecessors and unique resource use under the stated mode; ledger induction SHALL NOT count as native recursive or private split/join evidence.

#### Scenario: Positive witness
- **WHEN** Two native financial steps and a bounded private join verify under the released compatible interface.
- **THEN** Their recursive and composition claims have retained independent verification.

#### Scenario: Hostile witness
- **WHEN** A fabricated base, arbitrary verifier, repeated parent resource or finalization omission is supplied.
- **THEN** Verification rejects the invalid history.

### Requirement: UNI-010 Privacy and completeness
Where private continuation is claimed, acceptance SHALL bind the authenticated completeness domain and permitted observations, while the implementation supplies the stated witness-handoff and availability mechanism.

#### Scenario: Positive witness
- **WHEN** A separately isolated successor gets authorized private inputs.
- **THEN** It continues under the declared privacy and history relation.

#### Scenario: Hostile witness
- **WHEN** A hidden reservation is treated as absence or unauthorized information is disclosed.
- **THEN** The claimed completeness or privacy property is not accepted.

### Requirement: UNI-011 Federation and adapter boundary
When a kernel requests an external effect, its enforcing signer or destination SHALL bind exact decoded bytes and effects to the same intention, stage, domain, epoch and evidence policy, with ZK, MPC, TEE and finality assumptions separately stated.

#### Scenario: Positive witness
- **WHEN** Proof and threshold authorization bind the same permitted transfer.
- **THEN** The adapter signs or executes only those exact effects under its declared assumptions.

#### Scenario: Hostile witness
- **WHEN** Proof for X is used to sign Y, decoding is incomplete, or a fallback weakens the policy.
- **THEN** The request is rejected; compromised bare-threshold enforcement remains an explicit trust boundary.

### Requirement: UNI-012 Delegated AI spending
When concurrent delegated tasks commit funds, acceptance SHALL enforce attenuated owner authority and durable aggregate spent/pending/reserved budgets across retry and failover.

#### Scenario: Positive witness
- **WHEN** Requests for six and four units including fees share a ten-unit budget.
- **THEN** Both can reserve without exceeding ten.

#### Scenario: Hostile witness
- **WHEN** Concurrent requests for six and five units use copied parent budgets.
- **THEN** At least one is rejected or deferred.

### Requirement: UNI-013 Service lifecycle
When payment or result retrieval is retried, the workflow SHALL preserve authenticated logical request identity and distinguish payment finality, result availability and recipient delivery.

#### Scenario: Positive witness
- **WHEN** A paid request retries result retrieval.
- **THEN** Existing payment is reconciled without an unauthorized second charge.

#### Scenario: Hostile witness
- **WHEN** A finalized payment or TEE-local result is claimed as recipient delivery.
- **THEN** The delivery duty remains unresolved.

### Requirement: UNI-014 Certified libraries
When an optimized primitive or financial library is substituted, correspondence SHALL preserve values, refusal, complete effects, declared costs and discharged preconditions, including exact units, price orientation, rounding direction and beneficiary rules bound to the U0 numeric profile.

#### Scenario: Positive witness
- **WHEN** A native port converts an exact positive quote/base ratio before directed rounding under the bound U0 numeric profile.
- **THEN** The declared economic predicate and target constraints hold.

#### Scenario: Hostile witness
- **WHEN** A port reuses opposite price type spelling, reciprocal of an already rounded value without the required exactness/rounding relation, or a certificate for another implementation.
- **THEN** Qualification rejects the mismatch.

### Requirement: UNI-015 Semantic evolution
When program, verifier, policy, federation epoch or persistent state evolves, acceptance SHALL preserve signed semantics, consumption, recovery, privacy and obligations or require applicable amendment consent.

#### Scenario: Positive witness
- **WHEN** An authorized migration retains duty and replay commitments.
- **THEN** The successor continues under the bound version.

#### Scenario: Hostile witness
- **WHEN** An interface-compatible upgrade changes beneficiary or resurrects consumed authority.
- **THEN** Migration is rejected.

### Requirement: UNI-016 Assurance and release
When tools or release reports claim capability, they SHALL distinguish specified, inspected, tested, proved-under-assumptions, locally accepted and Preview-finalized evidence for the exact scope, retaining all required conformance and release rows.

#### Scenario: Positive witness
- **WHEN** All required rows carry exact artifacts, commands and supported predicates.
- **THEN** Only demonstrated capabilities are reported complete.

#### Scenario: Hostile witness
- **WHEN** Six advisor votes, finite samples or a static Pel check are supplied as proof of implementation.
- **THEN** Implementation and full-release status remain open.

### Requirement: UNI-017 Next native backend contract
When the next ZKIR/recursion interface is qualified for Moriarty, the integration SHALL satisfy ZR01–ZR16 and MNR01–MNR08 in the backend requirements contract for its claimed scope, preserving full native recursive financial history and private bounded multi-parent composition.

#### Scenario: Complete native recursive path
- **WHEN** A compatible released tuple supports the specified financial base, two native steps and a private join with all final checks.
- **THEN** Retained independent verification and actual ledger controls qualify only the evidenced scope.

#### Scenario: Incomplete cryptographic or release evidence
- **WHEN** A proposed instruction, outer-only check, six-month forecast or different proof format is offered as completed backend support.
- **THEN** The affected requirements remain open and no weaker mechanism is relabeled as full recursive compliance.

### Requirement: UNI-018 Profile and consumer admission
When a tool reports checking, compilation or simulation, it SHALL identify the source/Core profile, resolved executable, consumer and admitted subset, and SHALL report unsupported consumer constructs explicitly.

#### Scenario: Supported Rust Core program
- **WHEN** A `moriarty/1` program is checked and compiled by its supported Rust consumers.
- **THEN** The record names `moriarty-core/1`, both consumers and the construct coverage actually exercised.

#### Scenario: Different authoring profile
- **WHEN** A beta or Source/6 program passes its local checker but is outside a Rust compiler's admitted input.
- **THEN** The record retains the local checking result and reports the compiler boundary as unsupported; compilation receives no inferred pass.

### Requirement: UNI-019 Scoped lowering comparison
When two lowering consumers are compared, the comparison SHALL state their statements, admitted constructs and common domain, and SHALL compare independently specified complete observations only within that domain.

#### Scenario: Common construct domain
- **WHEN** Both lowerings admit a case with declared numeric and effect representations.
- **THEN** The comparison records each representation and the complete expected observations under the stated mapping.

#### Scenario: Different admitted statements
- **WHEN** One consumer intentionally refuses a construct admitted by another.
- **THEN** The admission matrix records that refusal and its reason; neither global equivalence nor identical artifact bytes is inferred.

### Requirement: UNI-020 Independent financial and work expectations
When a semantic or conformance result is scored, its expected values SHALL come from independently derived arithmetic and observations frozen before the run, with shared-code dependencies and numeric-profile scope declared.

#### Scenario: Complete finite observation
- **WHEN** A finite financial example is evaluated under a versioned numeric and logical-work schedule.
- **THEN** The comparator checks ordered effects, changed and preserved state, gross debit, fees, net receipts, dust, liability changes, duty presence and logical work against the independent expectations.

#### Scenario: Shared evaluator oracle
- **WHEN** Expected results come only from the same evaluator, static reopen or a wrapper around the implementation under review.
- **THEN** The record describes internal agreement and leaves independent semantic-oracle coverage open.

### Requirement: UNI-021 Capability evidence and parent acceptance
When a capability report is updated, it SHALL record implementation, evidence and acceptance separately for the exact predicate, tree, profile, consumer and target, while retaining every unmet parent exit.

#### Scenario: Delivered finite library child
- **WHEN** The report cites the PR49 FeePool dossier's 23 actions and 162 captured finite financial cases.
- **THEN** It records captured local/reference/static evidence and leaves full M11, Capacity Exchange, U6 and target qualification open.

#### Scenario: Incomplete predicate denominator
- **WHEN** A discovery index contains 69 top-level IDs but mandatory atomic clauses are not fully reconciled.
- **THEN** The report supplies no whole-language completion percentage and identifies the unresolved coverage.

### Requirement: UNI-022 Target-separated requirement reconciliation
When M1–M17 or existing Preview requirements are reconciled, the register SHALL retain the user's direct M requirements and preserve each result's actual target, qualified tuple and unresolved external facts.

#### Scenario: Scoped Preview result
- **WHEN** A historical Preview loan or swap result is cited alongside an open Fenrir requirement.
- **THEN** The record retains the Preview observation and identifies the missing Fenrir result without transferring acceptance.

#### Scenario: Unknown Fenrir configuration
- **WHEN** Fenrir versioning, network ID, Anchor, committee, registry or deployed tuple is unavailable.
- **THEN** The corresponding fields remain unknown and their dependent qualification remains open; independent language preparation can continue without another M1–M17 adoption decision.

### Requirement: UNI-023 Complete retained membership
When conformance coverage is reported, it SHALL validate membership against the frozen manifests and retain all ACTUS, DeFi, composition and release obligations, with semantic, local, native, Preview and Fenrir evidence recorded separately.

#### Scenario: Full family accounting
- **WHEN** A conformance inventory is prepared.
- **THEN** It retains 277 ACTUS fixtures and expected fields, 18 executable types, 32 dispositions, DS01–07, 72 DeFi rows, DA01–24, adopted supplements, three held-outs, eight intent cases, eight regression classes, all five composition operators and G01–G24.

#### Scenario: Finite library sample
- **WHEN** A new finite FeePool or batch example passes its local cases.
- **THEN** Only its named cells gain that scoped evidence; missing original rows and broader parent exits remain open.

### Requirement: UNI-024 Developer journey and current maintainer routing
When a developer journey or maintainer task is documented, the documentation SHALL identify implemented local commands, stubs and missing stages, and SHALL apply the current scoped routing without rewriting historical receipts or terminal task contracts.

#### Scenario: Current nonsecurity task
- **WHEN** New nonsecurity language work is implemented under the October 14 assignment.
- **THEN** The record names GPT-6.1 Sol high authorship and fresh non-author Muse Wonder 1.0 high review of the exact candidate and applicable results, with requested and returned identities retained.

#### Scenario: Incomplete public journey
- **WHEN** Local Rust commands and a separate Node beta package exist while Rust LSP/MCP are stubs or a required compile/target stage is unavailable.
- **THEN** The journey reports those exact boundaries, leaves U7/M10 open and does not label all CLI commands as stubs or claim an independently qualified release.
