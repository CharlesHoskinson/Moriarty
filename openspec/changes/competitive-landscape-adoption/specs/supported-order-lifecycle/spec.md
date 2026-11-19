## Purpose

Define acceptance for the supported single-chain order lifecycle and the evidence needed to assess each report recommendation.

## ADDED Requirements

### Requirement: I.1 Complete-checkout language baseline

The maintainer SHALL reproduce both documented demos and the intended suite from a complete checkout of the assessed revision. The record SHALL preserve the original failed attempt, dependencies, commands, checkout identity and each of its 21 failed test identities. Each row SHALL cite a verified repair or justify exclusion from the supported scope. Any repair SHALL identify the receiving revision and its clean rerun.

#### Scenario: Missing fixture returns

- **WHEN** the complete checkout provides a fixture missing from the assessed extraction
- **THEN** the disposition cites the old failure and the same test's new outcome.

#### Scenario: Unresolved export

- **WHEN** the Source/6 import still fails or any failure lacks a justified disposition
- **THEN** the baseline remains open even if both demos pass.

### Requirement: I.2 Revision-bound factory checks

DoctorWatson SHALL automate build, demos and tests after I.1. Its result SHALL identify the revision, toolchain, files, coverage metric, exclusions and supported source, kernel and ledger regression checks.

#### Scenario: Deliberate regression

- **WHEN** a controlled mutation changes an expected result in a covered path
- **THEN** its registered check fails for the expected reason and passes again on the clean revision.

### Requirement: II.1 Authenticated kernel acceptance

The adapter SHALL bind each owner's key to the account, order ID, assets, recipient, program, chain and signed constraints. It SHALL verify the generated kernel against current authenticated state. It SHALL consume authority atomically with funded settlement. Its contract SHALL identify trusted components and upgrade authority.

#### Scenario: Valid authority

- **WHEN** a funded permitted plan refers to the current accepted state
- **THEN** ledger readback shows one authority consumption and exactly the authorized effects.

#### Scenario: Invalid or reused authority

- **WHEN** an owner is forged, a domain changes, the predecessor is stale or the order is replayed
- **THEN** native acceptance refuses the order and readback shows no new order transfer or authority consumption.

### Requirement: II.2 Canonical funded O1

One frozen implementation SHALL spend exactly 100 A from Alice in one fill before 1400. Bob SHALL receive at least 200 B. Application fees SHALL be zero. The evidence SHALL reconcile all assets, residual custody, authority and separate network fees against accepted state.

#### Scenario: Exact and improved delivery

- **WHEN** independently initialized orders deliver 200 B and 201 B to Bob at time 1000
- **THEN** both fills complete and each consumes exactly 100 A once.

#### Scenario: Invalid fill

- **WHEN** a fill uses 99 A, pays 199 B, changes Bob, charges an application fee, forges authority, reaches expiry or repeats
- **THEN** the order rejects with its unauthorized effects absent from the ledger.

### Requirement: II.3 Owner cancellation and recovery

The custody interface SHALL isolate each order's assets and define surplus deposit handling. It SHALL preserve completed transfers and recover only the owner's remainder. It SHALL exclude fills at expiry and preserve owner recovery beyond the execution horizon. It SHALL document outstanding rights, revoked authority and trust across verifier upgrades.

#### Scenario: Cancel before or after a permitted fill

- **WHEN** an owner cancels an unfilled order or a partially executed order that permits partial fills
- **THEN** the owner can spend the returned remainder and all completed transfers persist.
- **AND** both fill/cancel orderings preserve the winning accepted effects.

#### Scenario: Surplus, expiry and renewed funding

- **WHEN** unrelated deposits enter custody or execution expires
- **THEN** recovery remains available without taking another order's assets.
- **AND** fresh funding cannot reactivate revoked authority under the documented upgrade policy.

### Requirement: II.4 Durable continuation consumption

Native acceptance SHALL bind continuation identity, predecessor, order custody, event identity and work or resource limits. The storage contract SHALL state durability and trust assumptions. The accepted ledger SHALL serialize resume and refund across client restarts.

#### Scenario: Concurrent terminal actions

- **WHEN** two clients race resume and refund in either ordering, including after restart
- **THEN** exactly one permitted terminal outcome commits and replay of the consumed continuation fails.

#### Scenario: Changed continuation

- **WHEN** a client changes custody, event identity or a resource cap
- **THEN** native acceptance rejects the altered continuation before releasing custody.

### Requirement: II.5 Authenticated observations

The feed adapter SHALL authenticate the named feed and verify freshness and the current condition at accepted execution. Its trust contract SHALL name the controller, update policy, clock and outage behavior.

#### Scenario: Observation changes before execution

- **WHEN** a valid trigger becomes false before the order executes
- **THEN** the current check refuses delivery.

#### Scenario: Feed and time boundaries

- **WHEN** the feed identity and signature match and the sample meets the condition and timestamp bounds
- **THEN** acceptance admits the observation under the stated feed assumptions.
- **AND** separate tests refuse substitution, stale or future times, threshold equality and missing required data.

### Requirement: II.6 Scoped financial guarantees

The implementation SHALL supply independently checkable accounting, budget and authority results for one named source profile and generated kernel. Each result SHALL state the domain, target, assumptions and relation to accepted ledger effects. It SHALL distinguish source termination, static checks, finite tests, proof or analysis results and ledger observations.

#### Scenario: Conditional invariant

- **WHEN** a checker establishes conservation, a cumulative debit bound or authorized recipient preservation under stated premises
- **THEN** the record names the checked statement, implementation binding and replay command.

#### Scenario: Missing correspondence

- **WHEN** compiler, custody or acceptance correspondence lacks a checked result
- **THEN** the obligation remains open even if the example traces pass.

### Requirement: III.1 Complete operation cost

The measurement SHALL use the pinned operation profile, independent expected effects and serial scored runs. Each sample SHALL bind revision, environment, operation, completion boundary, fee scenario, exclusions and all required transactions. It SHALL report setup, operation, separate recovery and totals including setup. Each isolated sample SHALL retain its execution ID, signed bytes, ordered receipts and observed state checks. A zero phase SHALL have an explanation.

#### Scenario: Both workloads complete

- **WHEN** the runner starts with separately funded, authorized and fillable orders
- **THEN** the fill covers final delivery and reconciled assets at 100 A for at least 200 B.
- **AND** cancellation before expiry covers finalized revocation and spendable return of all 100 A with no application fee.
- **AND** the cancellation controls reject unauthorized cancellation and any later fill, including the cancel-first race.

#### Scenario: Incomplete or incompatible sample

- **WHEN** a receipt, required transaction, resource, fee or completion check is missing
- **THEN** the runner retains the incomplete sample and withholds the operation result.

For each transaction the runner SHALL retain native blockUsage, bytesChurned, bytesWritten, readTime and computeTime with their units. It SHALL record mandatory native fees, payer, DUST denomination, integer SPECK and applicable fee policy. It SHALL retain protocol-version-specific resources and additional fees. The native-local cost check SHALL reconcile processed DUST fees against balances at the same ledger time. It SHALL separate prover effort from Midnight resources and compare only matching environments, scenarios and boundaries. Assessed figures SHALL require independent review through the assessment cost workflow.

### Requirement: III.5 Optional executor participation

The application SHALL publish participation, selection, open-entry conditions where applicable, fees, exclusivity, fallback and censorship assumptions. Two separately authorized clients SHALL exercise funded fill and fallback. The owner SHALL retain an exit when all operators stop.

#### Scenario: First client stops

- **WHEN** the first authorized client becomes unavailable
- **THEN** the second client can execute the permitted fallback and the owner can recover on a separate stopped-operator trace.
- **AND** a client outside the selected policy cannot execute a restricted action.

Service policy SHALL apply only to that application. Public language use SHALL remain permissionless. The policy SHALL state that participation supplies no fill guarantee.

### Requirement: III.6 Versioned lifecycle release

The maintainer SHALL release a versioned implementation and replay package with source identity, supported operations, trusted components, setup instructions and known gaps. A maintainer SHALL replay the lifecycle from a clean setup using only released documentation and declared public inputs.

#### Scenario: Maintainer reproduces the release

- **WHEN** a clean installation completes funded fill, cancellation and owner recovery
- **THEN** the replay records revision, package version and digest, commands, environment, setup time, receipts and outcome.
- **AND** the index records outside-developer reproduction as deferred and the report recommendation as partial.

#### Scenario: Released instructions fail

- **WHEN** replay requires an undocumented dependency or cannot reach the claimed ledger outcome
- **THEN** the failed record remains available and the reproducibility claim stays open.

### Requirement: IV.2 Funded O5

The O5 policy SHALL bind feed-AB and the authorized provider to a funded O1-style fill. The execution-time price SHALL be below 3 and sample age SHALL be between 0 and 30 inclusive. Feed, clock and inclusion assumptions SHALL accompany the result.

#### Scenario: Fresh permitted delivery

- **WHEN** an authenticated sample meets the bounds at accepted execution
- **THEN** readback reconciles 100 A and at least 200 B to Bob before expiry, including the age-30 boundary.

#### Scenario: Changed or substituted trigger

- **WHEN** the feed is substituted, time is stale or future, price equals 3, or an earlier valid trigger becomes false
- **THEN** the native custody path refuses the fill.

### Requirement: IV.3 Atomic funded O8

The O8 policy SHALL bind each owner's assets, amount and recipient to the same complete plan. A single settlement SHALL transfer 100 A from Alice to Bob, 200 B from Bob to Carol and 300 C from Carol to Alice.

#### Scenario: Three separate authorizations

- **WHEN** all three owners independently authorize their legs and all assets are funded
- **THEN** all three transfers commit together and the record identifies funding, fee payer and resource limits.

#### Scenario: One leg cannot commit

- **WHEN** authority is absent or changed, a recipient changes, or one leg lacks funds
- **THEN** accepted state contains zero order transfers from the plan.
- **AND** any backend limitation on multiple payers remains explicit.

### Requirement: IV.4 Funded staged O9

The first trade SHALL exchange 100 A for at least 200 B before 1200 into Alice's isolated custody. Only its resulting B SHALL fund the authorized conditional second trade for at least 300 C before 1400. After the second deadline the owner SHALL recover residual B without reversing the first trade.

#### Scenario: Continue or recover

- **WHEN** funded trials exercise both continuation/refund orderings, including a process restart
- **THEN** exactly one terminal action commits per trial and the first trade remains settled.
- **AND** a follow-up spend demonstrates owner control of the recovered residual assets.

#### Scenario: Ineligible second trade

- **WHEN** the condition is false, the second trade reaches expiry or the terminal action repeats
- **THEN** the rejected action moves no order assets.

### Requirement: IV.6 Cross-chain decision

After II.2 and II.3 the integration maintainer SHALL record the concrete need for O10 or justify deferral. A needed design SHALL define custody on both chains and source verification of authenticated destination inclusion. It SHALL specify proof, finality, timeout, reorganization and late-completion assumptions. Source release and refund SHALL exclude each other.

#### Scenario: No concrete integration needs O10

- **WHEN** the decision records its evidence and reasons for deferral
- **THEN** IV.6 can complete with O10 coverage and cost unestablished.

#### Scenario: Integration needs destination delivery

- **WHEN** a concrete integration requires O10
- **THEN** design checks cover nonfinal evidence, reorganizations, premature refunds and late delivery racing refund.
- **AND** coverage and cost remain open until implementation, execution and independent review.

### Requirement: Evidence review and external scope

A project result SHALL retain provenance, deterministic replay instructions, observed effects, limitations and review status. Independent evidence review SHALL precede each new assessed result, including O1 coverage and assumptions, O5 coverage and complete-operation costs. Review SHALL apply to the exact evidence and revision. A review request or timeout SHALL leave the assessment claim open.

#### Scenario: Scoped project closure

- **WHEN** the in-scope work passes and the maintainer replay succeeds
- **THEN** the change can close with outside reproduction under III.6 and the entire IV.5 pilot explicitly deferred.
- **AND** external use, live liquidity, independent reproduction and parent acceptance keep their own evidence requirements.

#### Scenario: External pilot becomes possible later

- **WHEN** an external operator commits to a supported order after III.5 and the original III.6 prerequisites
- **THEN** IV.5 requires dated deployment, operator, route, authorization, asset, funded completion and failure/recovery evidence.
- **AND** downstream adoption requires corroboration and test-network outcomes retain their network label.
