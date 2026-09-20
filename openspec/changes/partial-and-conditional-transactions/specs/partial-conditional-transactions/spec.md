## ADDED Requirements

### Requirement: MPLR-001 Staged and partial transaction semantics
When a supported transaction spans several settlement steps, Moriarty SHALL represent each committed step and the outstanding transaction state explicitly.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Prove each accepted prefix preserves authority, accounting and outstanding duties; do not label a successful prefix complete.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: An accepted debit followed by an unresolved withdrawal is reported as fully settled.
- **THEN** The report and proof relation SHALL retain the unresolved stage and SHALL NOT claim full settlement.

### Requirement: MPLR-002 Typed persistent continuations
When execution suspends for a later event, Moriarty SHALL persist a typed continuation bound to the program, predecessor, awaited events, authority and outstanding duties.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Resume a valid continuation once with correctly typed evidence and the signed stage policy.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: Replay or attach an unrelated callback to consume reserved authority twice.
- **THEN** Continuation validation SHALL reject the replay or mismatched callback without consuming authority again.

### Requirement: MPLR-003 Conditional settlement with composable evidence
When a submitted transaction specifies settlement conditions, Moriarty SHALL withhold the specified delivery until the required combination of authenticated signatures, documentary evidence, proofs, recipient actions and other supported predicates is satisfied.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A funded request remains pending until its bound recipient signature, document predicate and proof all validate at the policy-defined settlement point.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A document hash alone, an unrelated signature, or an expired proof releases the funds.
- **THEN** Settlement validation SHALL reject release until the exact bound evidence combination is valid under its evaluation policy.

### Requirement: MPLR-004 Partial fulfillment and residual duties
When a signed policy permits partial fulfillment, each accepted fill SHALL satisfy its fill rules and preserve cumulative bounds, remaining authority and residual obligations.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Two valid partial payments reduce the liability by exactly their authorized discharges.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: Splitting a fill bypasses a cumulative fee bound or erases remaining debt.
- **THEN** The verifier SHALL reject the fill or completion claim that violates cumulative bounds or drops the residual liability.

### Requirement: MPLR-005 Refunds and compensation
When a settlement step fails or remains unresolved, recovery SHALL distinguish a refund of still-controlled assets from a new compensating action after effects have committed.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A recovery transition spends only assets or claims it actually controls and retains unrecovered duties.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: Compensation rewrites history or issues a second refund after external delivery.
- **THEN** Recovery validation SHALL reject the nonexistent inverse or duplicate refund and retain the actual committed history and outstanding duty.

### Requirement: MPLR-006 Authenticated asynchronous outcomes
When evidence resumes a stage, Moriarty SHALL authenticate its origin, correlation and declared finality, and enforce the specified consumption rule.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A duplicated delivery is harmless or rejected without duplicate financial effects.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: An attacker substitutes a success flag or callback from another transaction.
- **THEN** Evidence validation SHALL reject the unauthenticated or mismatched result and SHALL NOT use it to authorize financial effects.

### Requirement: MPLR-007 Joins and late results
When a program joins asynchronous branches, it SHALL specify availability and success policies separately and retain duties for unselected or late branches.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A quorum branch proceeds under its signed rule while late branches remain accounted for.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: An any-success join drops the debit or refund duty of a losing branch.
- **THEN** Join validation SHALL reject the missing obligation accounting; the losing or late branch SHALL remain accounted for.

### Requirement: MPLR-008 Stage-specific authority and consent
When a stage executes, it SHALL use only the authority and consent granted for that stage under the signed lifecycle, revocation and expiry policy.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Recipient consent is bound to the exact request, amount, asset, conditions and allowed amendment version.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A callback gains the full original signer authority or changes the recipient silently.
- **THEN** Authority validation SHALL reject the amplified permission or unauthorized destination amendment.

### Requirement: MPLR-009 Cumulative work fees and reserves
When execution spans stages, Moriarty SHALL account for cumulative costs, reserved resources and permitted retained fees under the signed bounds and target phase rules.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A failed stage records retained fees and remaining reserves without exceeding its approved envelope.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: Retrying or splitting stages resets the total budget.
- **THEN** Resource validation SHALL reject the budget reset and preserve costs already incurred.

### Requirement: MPLR-010 Time finality and unresolved outcomes
When a deadline or observation boundary is reached, Moriarty SHALL classify what is known and apply the signed expiry policy without treating missing evidence as proof of nonexecution.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** An expired request retains unresolved external duties until reliable evidence establishes the relevant delivery, cancellation, nonexecution or recovery outcome; any other residual duties remain.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A late successful transfer is followed by a refund justified only by timeout.
- **THEN** Recovery validation SHALL reject a timeout-only claim of nonexecution and classify the relevant outcome according to available authenticated evidence, retaining unresolved duties where the evidence is insufficient.

### Requirement: MPLR-011 Causality concurrency and interference
When a suspended workflow resumes amid other activity, its proof SHALL preserve predecessor causality and revalidate the relevant state and authority under its signed policy.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Concurrent valid workflows cannot consume the same reserve or invalidate each other silently.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A simulation snapshot is reused after its relevant balance or authority changed.
- **THEN** The resuming stage SHALL reject stale evidence for changed relevant state or re-establish the required judgment under the signed policy.

### Requirement: MPLR-012 Simulation and assurance status
When a tool simulates or reports a workflow, it SHALL state which stages, calls and proof judgments were covered and keep prediction separate from accepted execution.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A local preview visibly reports unresolved external stages and does not claim settlement.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: An omitted external call is displayed as successfully executed or proved.
- **THEN** The report SHALL expose the omitted execution or unproved judgment and SHALL NOT claim successful execution or proof.

### Requirement: MPLR-013 Private evidence and continuations
Where evidence or workflow state is private, Moriarty SHALL define what each participant can observe and how later authorized stages obtain the witness needed to continue.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Prove a required document property without disclosing fields outside the authorized view.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A public status or continuation identifier leaks a property claimed confidential.
- **THEN** The privacy claim SHALL fail qualification when an observable output violates the declared disclosure relation.

### Requirement: MPLR-014 Certified lowering of staged effects
When a staged program is compiled, correspondence SHALL preserve its permitted traces, conditions, rejection behavior and costs in the pinned Midnight ZKIRv3 execution model.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A target trace is related to a permitted source trace with every retained effect accounted for.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A correct witness generator is presented as proof excluding invalid satisfying witnesses.
- **THEN** The compiler assurance report SHALL identify missing arbitrary-witness soundness; successful witness generation alone SHALL NOT certify the lowering.

### Requirement: MPLR-015 Open programmable financial workflows
When a developer composes supported constructs into a new workflow, validation SHALL use the versioned language and proof rules rather than a catalog of approved example programs.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A newly authored conditional workflow passes the ordinary public pipeline.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A valid program is rejected because it is absent from a project registry.
- **THEN** The public toolchain SHALL NOT reject solely for missing project-registry membership when all required language, authorization, proof and ledger predicates hold.

### Requirement: MPLR-016 Application authority versus project access
When an application restricts participants or evidence issuers, Moriarty SHALL enforce its authenticated policy without imposing project approval on public program deployment.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** A user can require a particular counterparty signature while independent developers deploy supported programs freely.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: A hosted service credential or reviewer receipt becomes mandatory proof evidence.
- **THEN** The public validity relation SHALL ignore missing project or hosted-service receipts while still enforcing the signed application and participant policy.

### Requirement: MPLR-017 Complete staged financial accounting
When a stage is accepted, its authenticated effect frame SHALL account for exact assets and custody, authorized supply changes, fees and separately typed liability evolution.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** Every asset transfer and liability change has its own well-typed conservation/evolution equation.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: Authority units are added to refund amounts, or internal zero-sum balance is called external settlement.
- **THEN** The verifier SHALL reject dimensionally invalid equations or a settlement claim unsupported by the complete effect frame and external evidence.

### Requirement: MPLR-018 Inspectable conditional intent
When a participant authorizes a conditional workflow, the signing and reporting interface SHALL expose its canonical conditions, destination, evidence policy, allowed partial outcomes and recovery rules.

#### Scenario: Supported behavior
- **WHEN** a supported program exercises this behavior with the required bound evidence
- **THEN** The displayed request and proved canonical object bind the same documents, recipient consent, deadline and remedies.

#### Scenario: Invalid or unsupported behavior
- **WHEN** a candidate attempts the following invalid case: The UI promises automatic refund while the signed policy permits an unresolved or compensating outcome.
- **THEN** The signing/reporting conformance check SHALL reject the mismatch and expose the actual canonical recovery policy before authorization.

