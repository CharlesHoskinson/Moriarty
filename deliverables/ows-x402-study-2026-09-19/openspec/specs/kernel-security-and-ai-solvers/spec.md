## ADDED Requirements

### Requirement: MPLR-019 — Consent to introduction of obligations
When a transition creates or increases a party's enforceable financial or operational obligation, it SHALL establish that party's consent to the obligation and its material terms, either directly or through an applicable prior authorization policy. A transfer to an address alone SHALL not manufacture such consent.

#### Scenario: Authorized supported transition
- **WHEN** a recipient signs the bounded debt terms under the applicable policy
- **THEN** acceptance SHALL record exactly that authorized obligation

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** a sender transfers a contract reference that makes an unconsenting recipient liable
- **THEN** acceptance SHALL reject creation of that recipient obligation

### Requirement: MPLR-020 — Certified primitive substitution
When a certified primitive substitutes for a reference expression, acceptance SHALL bind its version and establish preservation of reference semantics, preconditions, failure behavior and the relevant host/target correspondence.

#### Scenario: Authorized supported transition
- **WHEN** a jet has a verified certificate for the pinned reference relation and its call-site preconditions hold
- **THEN** acceptance SHALL preserve reference values, rejection, complete effects and the certified cost relation

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** a substituted jet drops a required carry check or assertion, or uses a certificate for another version
- **THEN** acceptance SHALL reject the substitution or its claimed certified status

### Requirement: MPLR-021 — Bounded resource certificates
Before accepting a stage under a bounded-execution claim, the verifier SHALL establish a resource bound for the concrete admitted artifact and any late-bound component under the bound cost model.

#### Scenario: Authorized supported transition
- **WHEN** a bounded loop and all invoked components fit the checked cost certificate for the emitted artifact
- **THEN** acceptance SHALL accept the stage within that declared resource bound

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** late-bound code has no checked bound or the actual artifact exceeds the claimed work limit
- **THEN** acceptance SHALL reject acceptance under that resource certificate

### Requirement: MPLR-022 — Program and evidence commitment separation
Acceptance SHALL bind program identity, semantic/profile version, signed intention, state and evidence according to their distinct commitment roles, and SHALL validate every permitted witness or program substitution against that binding.

#### Scenario: Authorized supported transition
- **WHEN** witness data opens the required commitments for the same program, intent, stage and domain
- **THEN** acceptance SHALL check the bound relation while preserving its stated privacy policy

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** evidence from another program, stage or chain is substituted despite matching data types
- **THEN** acceptance SHALL reject the mismatched evidence

### Requirement: MPLR-023 — Mandatory conditions constrain acceptance
Every declared mandatory intention predicate SHALL constrain transaction acceptance through a proved control/effect relation; computing or displaying a predicate without enforcing it SHALL not satisfy that requirement.

#### Scenario: Authorized supported transition
- **WHEN** the selected acceptance branch requires a predicate and valid evidence establishes it
- **THEN** acceptance SHALL permit that branch when its other required conditions also hold

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** a mandatory predicate is false but its computed Boolean is discarded
- **THEN** acceptance SHALL reject that branch despite successful execution of the Boolean computation

### Requirement: MPLR-024: Explicit settlement domains
When a workflow claims atomic settlement, it SHALL identify the ledger/domain and the exact effects inside that atomic boundary; relocation and independent-chain effects SHALL retain separate pending and final states.

#### Scenario: Authorized supported transition
- **WHEN** all prepared inputs and required legs execute in one declared settlement domain
- **THEN** acceptance SHALL record atomic completion only for the effects within that domain

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** unassignment has completed but assignment to the next domain remains pending
- **THEN** acceptance SHALL retain pending relocation and deny the claim of completed settlement

### Requirement: MPLR-025: Netting preserves gross economics
When gross obligations are netted, the transition SHALL preserve each authorized asset, party, fee, liability and residual duty under an explicit gross-to-net relation.

#### Scenario: Authorized supported transition
- **WHEN** a net schedule proves equivalence to the authorized gross assets, fees, liabilities and duties
- **THEN** acceptance SHALL accept the optimized schedule with the same authorized economics

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** equal final net balances hide an unauthorized fee or changed issuer liability
- **THEN** acceptance SHALL reject the net schedule

### Requirement: MPLR-026: Behavioral contracts for settlement implementations
When a workflow invokes an interchangeable settlement implementation, acceptance SHALL establish its required financial and authority postconditions; matching an interface alone SHALL not discharge these obligations.

#### Scenario: Authorized supported transition
- **WHEN** an implementation establishes every required transfer, consumption and authority postcondition
- **THEN** acceptance SHALL accept its use through the interface

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** an implementation returns Pending or omits a required delivery while claiming final settlement
- **THEN** acceptance SHALL retain the pending duty or reject the incorrect completion claim

### Requirement: MPLR-027: Authenticated origins and recursive compliance
When an accepted transition relies on recursive history, its proof SHALL establish legitimate initial state, compatible predecessor statements and well-founded composition, including every consumed resource and residual obligation.

#### Scenario: Authorized supported transition
- **WHEN** authenticated genesis and compatible well-founded predecessors establish the current state and unused resources
- **THEN** acceptance SHALL accept a preserving transition subject to actual ledger uniqueness

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** a prover invents already-funded genesis or uses a cyclic or wrong-relation predecessor
- **THEN** acceptance SHALL reject the recursive compliance claim

### Requirement: MPLR-028: Consent-preserving semantic evolution
When code, verifier, policy or persistent continuation evolves, acceptance SHALL preserve the signed semantics, residual duties, recovery rights, privacy and resource constraints, or obtain applicable amendment authorization.

#### Scenario: Authorized supported transition
- **WHEN** authorized migration proves preservation of residual duties, recovery, privacy and resource constraints
- **THEN** acceptance SHALL permit the continuation under the bound new version

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** an interface-compatible update changes the beneficiary without applicable amendment authority
- **THEN** acceptance SHALL reject that migration

### Requirement: MPLR-029: Authenticated completeness of private state
When a transition relies on absence, uniqueness or completeness, its proof SHALL bind an authenticated state domain and establish the required nonmembership or completeness property under explicit ledger assumptions.

#### Scenario: Authorized supported transition
- **WHEN** a proof establishes nonmembership in the authenticated current domain required by the policy
- **THEN** acceptance SHALL use that absence fact within the declared completeness assumptions

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** a private lookup returns None only because an existing reservation is not visible
- **THEN** acceptance SHALL reject the global absence claim

### Requirement: MPLR-030: Common evidence statement and federation trust
When the kernel combines proofs, attestations and signing authority, it SHALL bind them to the same intention, domain, stage, epoch, artifact and exact effects, and expose the threshold, hardware, observation and recovery assumptions.

#### Scenario: Authorized supported transition
- **WHEN** proof, required attestation and signing authorization bind the same effect, domain, stage and authorized epoch
- **THEN** acceptance SHALL accept their conjunction under the declared threshold and hardware assumptions

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** a proof authorizes transfer X but the signer is asked to sign transfer Y
- **THEN** acceptance SHALL reject the mismatched effect before signature release at the declared enforcement boundary

### Requirement: MPLR-031: Delegated solver capabilities
When a solver requests a financial effect, acceptance SHALL enforce the owner's bounded delegation across wallet, agent and signer boundaries.

#### Scenario: Authorized supported transition
- **WHEN** an unregistered solver presents valid delegated authority for the exact permitted effect
- **THEN** acceptance SHALL permit the supported action without project registration

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** an agent token is used to request an effect outside the owner delegation
- **THEN** acceptance SHALL reject the effect at the stated enforcement boundary; do not claim local policy alone prevents key extraction

### Requirement: MPLR-032: Concurrent service budgets
When concurrent tasks or retries commit funds, acceptance SHALL reserve and account for spent, pending and residual amounts, including fees and sponsored execution.

#### Scenario: Authorized supported transition
- **WHEN** a ten-unit budget already reserves six units and a second request commits four including fees
- **THEN** acceptance SHALL reserve the second request without exceeding ten units

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** the same budget receives concurrent six-unit and five-unit commitments
- **THEN** acceptance SHALL reject or defer at least one commitment so total exposure cannot exceed ten

### Requirement: MPLR-033: Payment and service completion
When a service is purchased, the program SHALL distinguish payment finality, result availability and delivery to the authorized recipient, retaining each unresolved obligation.

#### Scenario: Authorized supported transition
- **WHEN** payment finality and recipient delivery each have the evidence required by the signed policy
- **THEN** acceptance SHALL record both completed obligations

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** payment is final but the result is available only inside a TEE vault
- **THEN** acceptance SHALL retain the recipient-delivery duty and deny whole-purchase completion

### Requirement: MPLR-034: Logical paid-request identity
When a request is retried or recovered, the workflow SHALL preserve one authenticated logical obligation and distinguish result retrieval from new billable work.

#### Scenario: Authorized supported transition
- **WHEN** a retry names the same authenticated request whose payment is already final
- **THEN** acceptance SHALL reconcile that payment and apply the permitted result-retrieval policy without a second charge

#### Scenario: Invalid evidence or omitted obligation
- **WHEN** replayed transport data requests a second payment or a new billable service under the old authorization
- **THEN** acceptance SHALL reject the extra effect while preserving authorized recovery

