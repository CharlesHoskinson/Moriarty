# Ledger state and authority draft

Status: DRAFT, specified-only amendments. No implementation or native approval follows from this document.

## ADDED Requirements

### Requirement: EU-LDG-001 Source version and complete semantic boundary

WHEN admitting a transition, the adapter SHALL bind the exact source, contract, lifecycle, Core, schema, and bounds versions.

Trace: S01, S02, escrow-time. Acceptance evidence: compiler admitted Core and oracle19 complete envelopes.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 001

- **WHEN** A /5 custody envelope matches contract/4 and lifecycle/1
- **THEN** the adapter admits its exact version tuple

#### Scenario: Negative 001

- **WHEN** A /6 source is supplied to /5 custody admission
- **THEN** the adapter refuses PROFILE_MISMATCH without publishing artifacts

### Requirement: EU-LDG-002 Authenticated genesis commitment

WHEN minting a genesis identity, the policy SHALL enforce the complete GenesisCommitment and authenticated initial state.

Trace: S01. Acceptance evidence: designer01 S01 and constrained mint-policy UPLC/native readback.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 002

- **WHEN** The configured seed is spent and every GenesisCommitment field matches
- **THEN** mint evaluation accepts exactly the constrained initial head

#### Scenario: Negative 002

- **WHEN** The initial revision, program, network, authority, seed, bounds, or obligation history differs
- **THEN** mint evaluation refuses GENESIS_COMMITMENT_MISMATCH

### Requirement: EU-LDG-003 One-shot identity and placement

WHEN initializing an instance, the policy SHALL mint exactly one configured identity at the committed complete address.

Trace: S01. Acceptance evidence: designer01 S01 identity mutations and actual mint/input/output readback.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 003

- **WHEN** One seed-consuming transaction mints one configured token at the committed address
- **THEN** one authenticated head exists with token quantity one

#### Scenario: Negative 003

- **WHEN** The seed is referenced, token quantity is two, address differs, or additional identity assets are minted
- **THEN** the policy refuses GENESIS_IDENTITY_REFUSED

### Requirement: EU-LDG-004 Counterfeit and unauthorized mint refusal

WHEN resolving a head, the validator SHALL reject counterfeit identities and unauthorized identity minting or burning.

Trace: S01, S02. Acceptance evidence: actual policy hashing, mint-policy templates and stage00 genesis limitation.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 004

- **WHEN** A genuine identity and the admitted zero-mint continuation are supplied
- **THEN** the validator accepts identity authentication

#### Scenario: Negative 004

- **WHEN** A same-name wrong-policy token or fake same-address datum is supplied
- **THEN** the validator refuses HEAD_IDENTITY_REFUSED without recognizing the counterfeit as an instance

### Requirement: EU-LDG-005 Trusted genesis scope

WHERE initialization uses a trusted lock fixture, the reporter SHALL label constrained genesis unverified.

Trace: S01, escrow-time. Acceptance evidence: stage00 ASSESSMENT and revised T1 scoped reviews.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 005

- **WHEN** T1 starts from the externally configured trusted lock
- **THEN** the result reports trusted-lock-fixture-not-proved and S01 unverified

#### Scenario: Negative 005

- **WHEN** A one-shot seed/name/quantity receipt is offered as full genesis approval
- **THEN** the coverage classifier refuses GENESIS_EVIDENCE_INSUFFICIENT

### Requirement: EU-LDG-006 Runtime pin generation order

WHEN binding native pins, the builder SHALL execute the acyclic generation protocol and verify actual artifact identities.

Trace: S01, S02, escrow-time. Acceptance evidence: design proposed generation protocol and runtime compile/hash parity.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 006

- **WHEN** Actual script, policy, asset, role, and domain identities pass the generation protocol
- **THEN** a native manifest is emitted with reproducible identity derivations

#### Scenario: Negative 006

- **WHEN** A future-unknown value, shape sample, all-zero identifier, or circular parameterization is submitted as native
- **THEN** the builder refuses NATIVE_PINS_UNBOUND without a native artifact claim

### Requirement: EU-LDG-007 Script-own custody identity

WHEN checking Escrow custody, the validator SHALL derive its credential from the consumed spending-purpose input.

Trace: S01, S02, escrow-time, AP03. Acceptance evidence: revision03 script-own distinction and proposed ScriptContext projection.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 007

- **WHEN** The consumed authenticated head identifies the executing script credential
- **THEN** Escrow maps to that script while Lender and Borrower map to immutable vkey hashes

#### Scenario: Negative 007

- **WHEN** Escrow is mapped to a vkey or a caller-supplied foreign script hash
- **THEN** the validator refuses CUSTODY_IDENTITY_REFUSED

### Requirement: EU-LDG-008 Consume and recreate linear state

WHEN extending a live head, the validator SHALL consume that head and create exactly one authenticated successor.

Trace: S02. Acceptance evidence: designer01 head-step source plus UPLC/native exact outref readbacks.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 008

- **WHEN** H0 is spent and H1 preserves identity with revision increment and transport decrement
- **THEN** H1 alone becomes the canonical continuation

#### Scenario: Negative 008

- **WHEN** H0 is only referenced, H1 is absent, or two successors are produced
- **THEN** the validator refuses HEAD_CONTINUATION_REFUSED

### Requirement: EU-LDG-009 Authenticated complete prestate

WHEN evaluating a transition, the validator SHALL use the complete state decoded from authenticated consumed inputs.

Trace: S02, AP02. Acceptance evidence: designer01 replay observations and ledger resolved-input envelope.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 009

- **WHEN** Resolved H0 ordinary and financial fields match the evaluator prestate
- **THEN** the evaluator receives H0 decoded state

#### Scenario: Negative 009

- **WHEN** An old host snapshot replaces H0 allowances, work, IDs, authority, balances, or obligations
- **THEN** the validator refuses PRESTATE_BINDING_REFUSED

### Requirement: EU-LDG-010 Complete poststate and work

WHEN publishing a successor, the validator SHALL require exact ordinary, financial, identifier, authority, obligation, and work equality.

Trace: S02, escrow-time, AP02. Acceptance evidence: oracle19 varied balances/allowances/IDs/revision/work and designer01 complete oracle.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 010

- **WHEN** The successor deep-equals the complete accepted source envelope and admitted transport updates
- **THEN** the successor and all ordered effects are accepted

#### Scenario: Negative 010

- **WHEN** A retained unrelated balance, allowance, obligation, identifier, or work field differs
- **THEN** the validator refuses POSTSTATE_CORRESPONDENCE_REFUSED

### Requirement: EU-LDG-011 Immutable deployment and program identity

WHILE extending an instance, the validator SHALL preserve every immutable deployment, genesis, program, asset, and authority binding.

Trace: S01, S02, AP01, AP03. Acceptance evidence: designer05 well-formed marker substitutions and exact commitments.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 011

- **WHEN** All immutable fields are copied exactly from H0 to H1
- **THEN** the continuation preserves the registered identity

#### Scenario: Negative 011

- **WHEN** A well-formed network, program, role, schema, seed, or bounds digest is replaced
- **THEN** the validator refuses IMMUTABLE_BINDING_REFUSED

### Requirement: EU-LDG-012 CIP31 reference observations

WHEN reading reference inputs, the adapter SHALL exclude their value, nonce, identity, and role authority from consumption.

Trace: S02, oracle-observation, AP03. Acceptance evidence: CIP0031 Reference inputs and actual reference/consumed input projections.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 012

- **WHEN** A permitted read-only observation uses an authenticated reference input
- **THEN** the read leaves the reference UTXO unspent

#### Scenario: Negative 012

- **WHEN** A state transition, role claim, or gross funding claim relies only on that reference
- **THEN** the adapter refuses REFERENCE_AUTHORITY_REFUSED

### Requirement: EU-LDG-013 Script custody value frame

WHEN transferring custody value, the validator SHALL reconcile head Cash, thread identity, payouts, and unrelated assets exactly.

Trace: S02, escrow-time. Acceptance evidence: revision03 correspondence and oracle19 E13/E11/cancel native template.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 013

- **WHEN** Head Cash100 becomes Cash0 and an exclusive payee Cash100 output appears
- **THEN** thread quantity one and admitted ADA sponsorship persist with zero Cash/thread mint

#### Scenario: Negative 013

- **WHEN** Payout is missing, Cash is minted, thread moves, or an unrelated asset is diverted
- **THEN** the validator refuses VALUE_FRAME_REFUSED

### Requirement: EU-LDG-014 Exclusive settlement allocation

WHEN allocating settlement outputs, the validator SHALL enforce exclusive head-and-effect attribution for every claimed output.

Trace: S02, bilateral atomic escrow swap, two-order batch exact input coverage and injective output allocation / same-output double satisfaction. Acceptance evidence: designer03 cases and proposed output-index allocation table.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 014

- **WHEN** Two heads consume distinct inputs and allocate disjoint complete payout outputs
- **THEN** both source effects receive exactly one exclusive settlement allocation

#### Scenario: Negative 014

- **WHEN** Both heads claim the same payee output or one output also serves as change
- **THEN** the validator refuses DOUBLE_SATISFACTION

### Requirement: EU-LDG-015 Multi-head transaction atomicity

WHEN settling multiple heads, the validator SHALL enforce all admitted predecessor, successor, value, authority, and effect relations atomically.

Trace: S02, bilateral atomic escrow swap, two-order batch exact input coverage and injective output allocation / same-output double satisfaction. Acceptance evidence: native input/output/effect joint readback and compiler composition boundary.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 015

- **WHEN** All selected heads and effects satisfy their complete relation in one transaction
- **THEN** all selected successors become visible together

#### Scenario: Negative 015

- **WHEN** One required head is referenced, one effect is omitted, or an unrelated head replaces a predecessor
- **THEN** the transaction refuses MULTI_HEAD_RELATION_REFUSED without partial financial settlement

### Requirement: EU-LDG-016 Durable instance outcome nonce

WHEN accepting outcome authority, the validator SHALL consume its authenticated nonce resource and preserve consumed-or-cancelled history.

Trace: AP02, S03, S04. Acceptance evidence: designer05 AP02 and proposed AuthorityState with lifetime bound.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 016

- **WHEN** Nonce N7 is unused and the transition consumes the registered authority state
- **THEN** N7 becomes durably consumed in the recreated authority state

#### Scenario: Negative 016

- **WHEN** N7 is reused, erased, reset, or restored during extension, split, join, or migration
- **THEN** the validator refuses NONCE_REPLAY

### Requirement: EU-LDG-017 Cancellation and competing authority

WHEN cancellation and filling compete, the ledger classifier SHALL require one shared consumed authority and at most one canonical outcome.

Trace: AP02, S02. Acceptance evidence: actual concurrent pair receipts and state readbacks distinct from sequential replay.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 017

- **WHEN** Cancel and fill have independent fee inputs and compete for the same authority UTXO
- **THEN** exactly one may settle and the loser records input conflict after a winner

#### Scenario: Negative 017

- **WHEN** Cancellation only deletes host data or both outcomes consume disjoint nonce copies
- **THEN** the protocol refuses NONCE_AUTHORITY_REFUSED

### Requirement: EU-LDG-018 Nonce scope and lifetime bounds

WHERE authority is per-instance, the reporter SHALL limit nonce uniqueness claims to that instance and admitted lifetime bounds.

Trace: AP02, S03, S04. Acceptance evidence: designer05 scope limitation and bounded registry size policy.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 018

- **WHEN** Two independent instances use the same textual outcome ID under separate signatures
- **THEN** the report distinguishes both permitted per-instance consumptions

#### Scenario: Negative 018

- **WHEN** A multi-instance globally-once signature lacks jointly consumed shared authority
- **THEN** admission refuses GLOBAL_NONCE_AUTHORITY_ABSENT

### Requirement: EU-LDG-019 Required signer consent

WHEN authorizing a role action, the validator SHALL require the immutable role hash in TxInfoV3 signatories.

Trace: AP03, joint-role, escrow-time. Acceptance evidence: cardano-ledger Conway TxInfo and native signatory/witness negative.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 019

- **WHEN** The required role hash appears and the ledger accepts its matching body witness
- **THEN** native authorization accepts that role

#### Scenario: Negative 019

- **WHEN** A funding witness exists but its role hash is absent from required signatories
- **THEN** the validator refuses ROLE_SIGNATORY_MISSING

### Requirement: EU-LDG-020 Multisig and role token consumption

WHERE authority requires multiple roles, the validator SHALL enforce every role credential and its declared consumption rule.

Trace: AP03, joint-role. Acceptance evidence: designer05 AP03 and designer02 joint-role native controls.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 020

- **WHEN** Both immutable required keys sign or declared unique role tokens are consumed
- **THEN** the native role relation accepts all required consent

#### Scenario: Negative 020

- **WHEN** One key is missing, a token is only referenced, or a wrong-policy same-name token substitutes
- **THEN** the validator refuses ROLE_AUTHORITY_REFUSED

### Requirement: EU-LDG-021 Final-body wallet collateral boundary

WHEN assessing collateral exposure, the wallet policy SHALL inspect the final transaction body independently of TxInfoV3.

Trace: AP01, AP03, F04-fee-gross-net-policy. Acceptance evidence: pinned V3 Contexts and whole-body wallet policy measurements.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 021

- **WHEN** Two bodies share V3-visible effects but differ in collateral risk
- **THEN** the wallet policy accepts only the body within signed exposure limits

#### Scenario: Negative 021

- **WHEN** A V3-only checker claims collateral-input or collateral-return enforcement
- **THEN** the evidence classifier refuses COLLATERAL_CONTEXT_ABSENT

### Requirement: EU-LDG-022 Split conserved lineage

WHEN splitting a head, the validator SHALL enforce the declared identity transformation and exact partitioned state conservation.

Trace: S03. Acceptance evidence: designer01 S03 partition oracle and proposed LifecycleDescriptor.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 022

- **WHEN** Parent H and nonce authority A are spent into child0, child1, and recreated A
- **THEN** the split satisfies the proposed numeric oracle and unique obligation ownership

#### Scenario: Negative 022

- **WHEN** Allowances, work, duties, identities, reserves, or nonce history are copied or dropped
- **THEN** the validator refuses SPLIT_CONSERVATION_REFUSED

### Requirement: EU-LDG-023 Join complete lineage

WHEN joining children, the validator SHALL consume every declared child and preserve all obligations, identifiers, and nonce authority.

Trace: S03. Acceptance evidence: designer01 S03 join oracle plus bounded fan-in coverage.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 023

- **WHEN** Both children and authority A are consumed into one successor under the proposed join oracle
- **THEN** the join preserves state sums minus the declared work debit

#### Scenario: Negative 023

- **WHEN** A child is missing or referenced, a duty duplicates, or spent IDs disappear
- **THEN** the validator refuses JOIN_LINEAGE_REFUSED

### Requirement: EU-LDG-024 Migration allowlist and invariants

WHEN migrating an instance, the validator SHALL enforce the signed successor-program allowlist and complete inherited state invariants.

Trace: S04, AP02, AP03. Acceptance evidence: designer01 S04 oracle and proposed migration descriptor.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 024

- **WHEN** Revision5 migrates to allowed revision6 with work99/spent61 and preserved O1 outstanding80
- **THEN** one target head preserves instance, duties, roles, nonce history, value, and reserves

#### Scenario: Negative 024

- **WHEN** Target program is unlisted or liabilities, roles, nonce history, assets, or closure funds change
- **THEN** the validator refuses MIGRATION_RELATION_REFUSED

### Requirement: EU-LDG-025 Debt-free termination and retirement

WHERE termination uses the debt-free profile, the validator SHALL require zero residual duties and declared identity retirement.

Trace: S04, AP02, F03-partial-repayment-residual-liability. Acceptance evidence: designer01 S04 termination and E12 outstanding40 nontermination.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 025

- **WHEN** All liabilities are zero with final payouts and authorized identity burn minus one
- **THEN** no live successor remains and the closed authority history persists

#### Scenario: Negative 025

- **WHEN** Outstanding debt remains or retirement resets nonce authority or retains an unauthorized live head
- **THEN** the validator refuses TERMINATION_RESIDUAL_DUTY

### Requirement: EU-LDG-026 Lifecycle language gap refusal

IF a lifecycle selector lacks an admitted descriptor, THEN the compiler SHALL refuse UNSUPPORTED_LIFECYCLE without emitted validators.

Trace: S03, S04. Acceptance evidence: unsupported-split source and SOURCE_ACTION_UNKNOWN controls.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 026

- **WHEN** Existing finite fee and transfer actions use admitted source operations
- **THEN** their economics require no new grammar

#### Scenario: Negative 026

- **WHEN** Split, Join, or Migrate is requested without versioned source semantics and a ledger descriptor
- **THEN** compilation refuses UNSUPPORTED_LIFECYCLE and records an implementation gap

### Requirement: EU-LDG-027 Evidence stage and failure classification

WHEN reporting acceptance, the reporter SHALL distinguish source, UPLC, node, inclusion, readback, and proof evidence.

Trace: S01, S02, S03, S04, AP02, AP03, escrow-time. Acceptance evidence: oracle19 templates and stage00 source/node/readback distinctions.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 027

- **WHEN** A signed positive is included and independently decoded head/payouts match the full oracle
- **THEN** only the measured native relation receives native acceptance

#### Scenario: Negative 027

- **WHEN** A source pass, Ogmios preflight, stale witness failure, or sequential conflict is called native semantic approval
- **THEN** the report refuses EVIDENCE_LAYER_MISMATCH

### Requirement: EU-LDG-028 Private and PCD acceptance boundary

WHERE the public adapter lacks complete proof verification, the admission path SHALL reject required private or PCD modes.

Trace: AP04, S01, S02, AP02. Acceptance evidence: existing pcd-ledger-anchored spec and designer05 failclosed certificate controls.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 028

- **WHEN** An explicitly public-only admission requests no private or PCD claim
- **THEN** the report retains private, PCD, and Complete as unverified

#### Scenario: Negative 028

- **WHEN** A requiredClaimRoot or private transition is accepted from host verified=true or public settlement alone
- **THEN** admission refuses REQUIRED_PROOF_VERIFIER_ABSENT

### Requirement: EU-LDG-029 Canonical hostile controls and real races

WHEN measuring hostile transitions, the runner SHALL retain canonical bytes, valid witnesses, resolved contexts, and exact refusal classes.

Trace: S02, AP02, escrow-time. Acceptance evidence: oracle19 templates and stage00 InvalidWitnessesUTXOW versus PlutusFailure versus BadInputsUTxO.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 029

- **WHEN** Wrong poststate is re-signed and reaches phase2 while the authenticated head remains unchanged
- **THEN** the receipt records script refusal and preserved financial state

#### Scenario: Negative 029

- **WHEN** Stale witnesses or sequential replay replace the required semantic mutation or concurrent race
- **THEN** coverage remains incomplete with the observed distinct failure class

### Requirement: EU-LDG-030 Ledger-anchored source lifecycle invariants

WHEN implementing ledger-anchored lifecycle actions, the adapter SHALL preserve initialization, migration signatures, predecessor imports, and terminal disposition invariants.

Trace: S01, S04, AP02, AP03. Acceptance evidence: existing pcd-ledger-anchored-acceptance specification and native lifecycle controls.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 030

- **WHEN** Uninit initialization has all-principal signatures and migration satisfies the declared or unanimous branch with one predecessor import
- **THEN** the native projection preserves the existing source lifecycle invariants and explicit cross-domain commitment map

#### Scenario: Negative 030

- **WHEN** initialization omits a principal, migration substitutes a key, or a consumed predecessor commitment is imported twice
- **THEN** the adapter refuses SOURCE_LIFECYCLE_INVARIANT_REFUSED without rewriting existing approved semantics

### Requirement: EU-LDG-031 Mandatory claim discharge boundaries

WHEN claiming ledger-anchored history compliance, the reporter SHALL require immutable operation keys, constrained genesis, and measured head discipline.

Trace: S01, S02, S04, AP01, AP04. Acceptance evidence: existing claim-discharge map and complete verifier/native measurements.
Definitions and exact interfaces: `design.md`.

#### Scenario: Positive 031

- **WHEN** all induction assumptions and the separately required contract-property, intent, transition, and proof predicates have evidence
- **THEN** the reporter discharges only the established claims at their declared loci

#### Scenario: Negative 031

- **WHEN** public head settlement lacks a checked property certificate or any required complete-relation proof
- **THEN** the reporter refuses MANDATORY_CLAIM_NOT_DISCHARGED and leaves private, PCD, and Complete unverified
