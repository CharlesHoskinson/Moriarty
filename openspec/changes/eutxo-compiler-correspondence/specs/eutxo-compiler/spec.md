## ADDED Requirements

### Requirement: EUC-001 Explicit factory and version routing
WHEN source compilation selects financial agreement /5 or /6, the compiler SHALL derive Core with the matching source factory and bind /5 to expression contract /4 and lifecycle-state/1, and /6 to expression contract /5 and lifecycle-state/2. IF a requested profile, contract, lifecycle schema, or optional feature lacks an approved lowering table, THEN the compiler SHALL return a named unsupported diagnostic without a blueprint.

#### Scenario: Positive Compile the approved /5 custody source
- **WHEN** the source selects /5 and the scoped T1 capability
- **THEN** the manifest records contract /4 and lifecycle-state/1

#### Scenario: Negative Prevent implicit profile migration
- **WHEN** a /6 source enters the T1-only compiler
- **THEN** compilation refuses PROFILE_MISMATCH and produces no executable artifact

### Requirement: EUC-002 Source-derived exhaustive Core traversal
WHEN elaboration succeeds, the compiler SHALL traverse every own constructor and declared expression, list, and record-field operand in source statement order. The compiler SHALL record constructor counts, types, spans, operand paths, effects, and unsupported nodes. IF a constructor or operand shape lacks a registered rule, THEN the compiler SHALL refuse before Aiken emission.

#### Scenario: Positive Walk nested Transfer payload
- **WHEN** Emit contains ConstructRecord and ConstructAmount
- **THEN** both constructors contribute to the manifest and work count

#### Scenario: Negative Reject invented or skipped nodes
- **WHEN** a walker inserts Let into the T1 source or omits ConstructRecord
- **THEN** the Core inventory comparison fails and no blueprint is published

### Requirement: EUC-003 Complete ordinary and financial expression semantics
WHERE general /5 or /6 lowering is enabled, the compiler SHALL preserve Require, Let, NextWrite, Ensure, Emit, literals, reads, projections, bounded collections, variants, conversions, scalar extraction, selection, arithmetic, comparisons, and Boolean evaluation from the selected contract table. WHEN financial post reads execute, the generated validator SHALL read the staged kernel candidate after effects and before acceptance. IF writes target protected financial fields or a post read appears outside Ensure, THEN compilation SHALL refuse with the source diagnostic.

#### Scenario: Positive Read resulting debt
- **WHEN** an Ensure reads outstanding debt after Repay
- **THEN** the predicate uses the updated obligation

#### Scenario: Negative Preserve unreachable arithmetic behavior
- **WHEN** a source short-circuit or Select avoids a zero divisor
- **THEN** the generated result matches the source evaluator without evaluating the unreachable branch

### Requirement: EUC-004 Complete staged financial kernel
WHERE a financial operation is admitted, the generated validator SHALL preserve the selected source kernel operation order, full effects, balance and allowance changes, liability and period changes, durable used identities, and work debit. The compiler SHALL distinguish Transfer, Originate, Accrue, Repay, repayment allocation, and Fee by their actual source schema and kernel routing. WHERE a separate Allocate descriptor is requested, the compiler SHALL require a demonstrated profile binding before admission. IF the selected profile does not define a requested descriptor, THEN compilation SHALL refuse instead of inventing an operation.

#### Scenario: Positive Preserve partial repayment
- **WHEN** a supported repayment leaves outstanding liability 40
- **THEN** the generated continuation retains liability 40 and the ordered repayment effects

#### Scenario: Negative Retain narrow T1 fence
- **WHEN** E12 has nonempty obligations but source evaluation prepares cancel
- **THEN** T1 admission returns NONEMPTY_OBLIGATIONS while the source result retains its own status

### Requirement: EUC-005 Closed canonical lifecycle ABI
WHEN lifecycle-state/1 is encoded, the compiler SHALL use Constr 0 with schema_version, balances, allowances, obligations, used_transfer_ids, used_allocation_ids, used_origination_ids, used_accrual_ids, and work in that order. The codec SHALL retain complete typed obligation fields and list order. IF bytes have extra fields, wrong constructors, invalid UTF-8, malformed decimal amounts, or an unknown schema, THEN admission SHALL refuse before state consumption.

#### Scenario: Positive Decode a residual obligation
- **WHEN** Lucid definite CBOR contains outstanding 40
- **THEN** Aiken decoding retains all 19 obligation fields and outstanding 40

#### Scenario: Negative Detect positional drift
- **WHEN** two same-type byte fields are exchanged
- **THEN** semantic equality fails even if typed decoding succeeds

### Requirement: EUC-006 Head and action ABI identity
WHEN T1 HeadDatum or Redeemer is encoded, the compiler SHALL use the revision03 ABI field order and Release 0, Refund 1, Cancel 2 tags. The validator SHALL select the action from the tag and decode actor, recipient, transfer_id, lower, upper in that order. IF field order, tag, arity, or copied content identity differs, THEN the validator SHALL reject the spend.

#### Scenario: Positive Select refund explicitly
- **WHEN** Refund tag and valid refund Args are decoded
- **THEN** the refund Core action executes

#### Scenario: Negative Reject action guessing
- **WHEN** Release Args are paired with Refund tag
- **THEN** the source action guard rejects and the spending validator cannot succeed

### Requirement: EUC-007 Identifier text arithmetic and quantum policy
WHEN values enter the compiler or generated validator, the boundary SHALL enforce source identifier syntax, scalar UTF-8, declared text byte limits, decimal-string canonicality, numeric widths, intermediate bounds, collection bounds, quantum conversion, and rounding provenance. IF any value exceeds its pinned bound, a divisor is zero, or rounding-none consumes a rounded conversion, THEN execution SHALL refuse atomically. WHILE T1 is selected, quantum other than 1 SHALL remain unsupported.

#### Scenario: Positive Distinguish rounding
- **WHEN** principal 101 accrues at rate 1/10 under floor or ceiling
- **THEN** the result is 10 or 11 respectively under the admitted financial policy

#### Scenario: Negative Constrain unused arguments
- **WHEN** an unused Text argument is invalid UTF-8 or exceeds its schema bound
- **THEN** the validator rejects rather than admitting unconstrained ByteArray

### Requirement: EUC-008 Domain-separated immutable digests
WHEN compilation binds content, the compiler SHALL recompute source, program, schema, seed, bounds, work-domain, deployment-domain, and instance digests from the actual admitted bytes. The compiler SHALL use codec.ts domain-NUL-canonical-value or domain-NUL-raw-byte preimages. The compiler SHALL keep workBudget inside the bounds record and exclude work_hash from T1 HeadDatum. IF a pin differs or canonical data includes a JSON number, THEN compilation SHALL refuse.

#### Scenario: Positive Calibrate source explicitly
- **WHEN** a reviewed POSIX calibration changes guard literals only
- **THEN** source and program hashes change and unchanged bounds retain their hash

#### Scenario: Negative Reject atomic-domain substitution
- **WHEN** a financial source uses MORIARTY-PROGRAM-bounded-atomic/1
- **THEN** the financial pin check fails

### Requirement: EUC-009 Actual operation work and distinct counters
WHEN an action executes, the generated validator SHALL debit the visited expression work and the selected kernel operation cost under the pinned work model. The manifest SHALL compare counted static Core work with staticWorkBound. The validator SHALL keep transport_revision, transport_remaining, ordinary revision, and lifecycle work as distinct fields. IF work or closure reserve cannot satisfy source acceptance, THEN the spend SHALL reject without committing candidate state.

#### Scenario: Positive Use variable admitted work
- **WHEN** release-work-additive is evaluated
- **THEN** the continuation has remaining 151 and spent 59 rather than the control post

#### Scenario: Negative Preserve early refusal accounting
- **WHEN** release-work-one-short or an early guard fails
- **THEN** the report retains source workUsed and sourceCode separately from validator refusal

### Requirement: EUC-010 Parameter roles guards effects and intent
WHEN a source action is lowered, the compiler SHALL retain every parameter dependency, source guard, ordinary invariant, effect descriptor, and intent predicate. WHERE ledger context is required, the compiler SHALL expose typed bindings for authenticated actors, joint roles, observations, validity intervals, and spending inputs. IF a required binding is absent, unauthenticated, or inconsistent with source Args, THEN the generated validator SHALL refuse.

#### Scenario: Positive Bind joint authority
- **WHEN** a source action requires two roles
- **THEN** both independent authenticated role bindings are checked

#### Scenario: Negative Keep observations explicit
- **WHEN** ReadObs has only caller-supplied text without an authenticated reference binding
- **THEN** compilation or execution refuses the missing trust binding

### Requirement: EUC-011 Runtime pins and self identity dependencies
WHEN native compilation is requested, the compiler SHALL require a recomputable runtime manifest with real distinct policy ids, role key hashes, network, deployment, asset names, genesis reference, and compiler dependencies. IF any hash is zero, malformed, equal where distinctness is required, or a shape sample, THEN compilation SHALL refuse. IF a spending-script identity creates a cyclic compile dependency, THEN compilation SHALL refuse until a reviewed acyclic identity recipe exists.

#### Scenario: Positive Use real policy dependencies
- **WHEN** compiled policy bytes and actual role verification keys are supplied
- **THEN** the manifest recomputes policy credentials, role hashes, and instance identity

#### Scenario: Negative Stop recursive hash pinning
- **WHEN** Escrow identity is required as a literal equal to the final validator hash while compiling that hash
- **THEN** the compiler reports the identity dependency and does not insert a placeholder

### Requirement: EUC-012 Artifact publication and failure isolation
WHEN compilation succeeds, the compiler SHALL publish the generated Aiken, checked blueprint, UPLC bytes, Cardano credential, source-file digest, and dependency manifest from one isolated output directory. IF admission, Aiken check, Aiken build, required tests, or blueprint parsing fails, THEN the compiler SHALL return nonzero and leave no success blueprint. IF artifacts or dependencies drift after verification, THEN consumption SHALL refuse the stale manifest.

#### Scenario: Positive Publish a checked result
- **WHEN** check, build, and required named tests succeed on pinned files
- **THEN** the blueprint references exactly the compiled source manifest

#### Scenario: Negative Refuse stale success
- **WHEN** a prior plutus.json exists and the next build fails
- **THEN** the old blueprint cannot be returned as the new compilation result

### Requirement: EUC-013 Executed generated spending correspondence
WHEN compiler correspondence is measured, the test consumer SHALL run the actual generated spending entrypoint on reconstructed transaction context and source-envelope inputs. Accepted cases SHALL compare complete ordinary post, canonical admitted financialPost, ordered effects, and workRemaining against the independent source envelope. Rejected cases SHALL require spending refusal and preserve sourceCode separately.

#### Scenario: Positive Challenge fixed control posts
- **WHEN** release-lender-balance-7, prior used ids, and revision-4 are supplied
- **THEN** generated outputs retain 107, PriorSpend plus Release1, and revision 5 respectively

#### Scenario: Negative Reject helper-only evidence
- **WHEN** only a lifecycle codec or pure helper has run
- **THEN** the report leaves generated-spending correspondence unperformed

### Requirement: EUC-014 Real cross-language representation tests
WHEN ABI compatibility is claimed, the test consumer SHALL decode Lucid canonical definite CBOR with the actual pinned Aiken decoder and execute the generated validator using decoded values. The consumer SHALL distinguish representation round trips, Aiken unit execution, compiled UPLC evaluation, and native ledger results. IF malformed or swapped-field fixtures decode differently, THEN the measured compatibility claim SHALL fail.

#### Scenario: Positive Exercise definite head bytes
- **WHEN** a complete Lucid HeadDatum is decoded in Aiken
- **THEN** all copied identity and lifecycle fields equal the admitted record

#### Scenario: Negative Reject serializer equivalence shortcut
- **WHEN** only Aiken serialise matches five small vectors
- **THEN** full HeadDatum decoding remains unperformed

### Requirement: EUC-015 Twenty-family coverage and bounded extensions
WHEN capability is reported, the manifest SHALL map every PATTERN-INVENTORY family and T1 through T18 to requirements, tasks, dependencies, and separate source, representation, UPLC, native, and private results. WHERE Split, Join, Migrate, or Terminate is proposed, the amendment SHALL include a demonstrated S03 or S04 source limitation and independent duty-conservation oracle. IF any family lacks executed native evidence, THEN the report SHALL retain that family as incomplete.

#### Scenario: Positive Keep staged roadmap visible
- **WHEN** T1 custody passes while F01 and S03 remain unrun
- **THEN** only the scoped T1 predicate advances

#### Scenario: Negative Prevent synthetic expansion
- **WHEN** 20 inventory rows exist with source checks only
- **THEN** the report cannot claim 20 native families

### Requirement: EUC-016 Unsupported and private classification
IF source transport, parser, profile routing, Core lowering, representation, UPLC execution, ledger acceptance, or required proof verification fails, THEN the consumer SHALL report the actual failure layer and preserve the original diagnostic. WHILE private, certificate, PCD, and Complete remain unverified, the compiler SHALL refuse those capabilities and retain the global Midnight acceptance gates.

#### Scenario: Positive Retain explicit unsupported seam
- **WHEN** unsupported-private.mori or unsupported-certificate.mori is compiled
- **THEN** the source diagnostic remains UNKNOWN_DECLARATION or OPERATION_BINDING

#### Scenario: Negative Prevent compatibility release substitution
- **WHEN** a public Cardano compatibility test passes
- **THEN** Midnight financial settlement, mandatory PCD, private handoff, and Complete remain open

