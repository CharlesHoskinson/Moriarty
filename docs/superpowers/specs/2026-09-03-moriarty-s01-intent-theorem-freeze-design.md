# Moriarty S01 intent-theorem freeze design

Status: approved for planning

## Decision

Freeze an architecture-neutral intent-safety interface during S01. Do not
select a Core architecture before S02 compares the four required options.

The frozen interface defines the meaning of authorization across each option.
S02 must show how each option realizes the same interface. S02 must reject an
option that cannot realize the interface without an unstated trust dependency.

Keep Moriarty Core at version `0.0.0-e00.2`. S01 adds no constructor, action,
observation, value, type, warning, error, or backend feature.

## Controlling inputs

Use these immutable inputs:

- Prompt version `1.3` at the committed repository revision.
- Semantic scope `0.0.0-e00.2`.
- Semantic-scope digest
  `9bee72cb3a71962128ce5ead07b0912a3f54b1d813d59f629852631057e36c7a`.
- The current atomic-swap trace model as the first negative-test application.
- The existing intent-standards claim ledger and report.

S01 can clarify a term from these inputs. S01 cannot change a source fact or
silently resolve a recorded contradiction.

## Alternatives

### Architecture-neutral interface

This design freezes one semantic interface before architecture selection. Each
S02 candidate must implement or refine that interface.

This design preserves the required sprint order. It also gives S02 one common
falsification target. Select this alternative.

### Agreement-Core-specific judgment

This design binds intent safety to the existing agreement-Core recommendation.
It reduces later translation work but selects option A before the S02 study.

Reject this alternative for S01. S02 can select option A after comparison.

### One theorem family for each architecture

This design creates four related correctness judgments. It exposes option-level
differences but can hide incompatible meanings behind similar theorem names.

Reject this alternative for S01. Use one judgment and explicit refinements.

## OpenSpec boundary

Create the OpenSpec change `s01-intent-theorem-freeze`. Keep it separate from
the earlier `WP02` evidence-and-taxonomy package. The older package belongs to
the prior sprint program and does not satisfy prompt version 1.3.

The change must contain these sections:

- proposal
- design
- normative requirements
- tasks
- dependencies
- immutable inputs
- exact outputs
- acceptance predicates
- negative controls
- evidence manifest
- failure outcomes
- rollback
- semantic-scope transition

The package must fail closed. Its validator must recompute each acceptance
predicate from repository files.

## Normative terminology

Create a machine-readable terminology registry. Give each term one stable
identifier and one canonical technical noun.

Each record must contain:

- the canonical noun and definition
- excluded meanings
- semantic category
- identity rule
- version rule
- mutability rule
- producer and permitted consumers
- authority boundary
- canonical representation status
- source or design basis

Define at least every W4 object from `IntentObjective` through
`CancellationReceipt`. Define the W5 evidence objects that enter authorization.
Do not equate an objective, quote, authorization, order, plan, fill, claim,
settlement, refund, or cancellation.

Classify each lifecycle object as exactly one primary category:

- command
- authorization
- carrier
- state
- observation
- evidence

A record can have secondary roles. The primary category controls its authority.

## Observation model

Define observations as typed events. An event contains its channel, actor,
domain, settlement level, time, state anchor, payload commitment, and visibility.

Define a projection `view(actor, D, L, T, O)` for these actor classes:

- user or signer
- counterparty
- resolver or solver
- prover
- Runtime or indexer
- wallet or custodian
- ledger observer
- auditor

The projection must distinguish public data, private data, commitments, approved
disclosures, metadata, failures, timing, and unavailable evidence.

An approved disclosure changes visibility. It does not authorize a financial
effect. A hidden effect remains an effect under every projection.

S01 freezes this model. A later sprint proves or tests noninterference against
the frozen model.

## Intent constraints

Separate hard predicates from optimization preferences. Only hard predicates
can establish `authorizedAt`.

Each hard-predicate record must contain:

- predicate identifier and version
- canonical parameters
- evaluation state
- required artifacts
- pass, fail, and unavailable results
- stable failure reason
- signed-binding requirement
- settlement level
- residual-intent behavior
- composition rule or explicit prohibition

The initial registry must cover these predicate families:

- allowed and forbidden effects
- spend and change destinations
- mint and burn limits
- fees
- signers and approval scope
- disclosures
- capabilities
- replay, nonce, validity, and cancellation
- refund destinations and refund conditions
- partial-fill residual conservation
- settlement level and finality

An optimization preference can rank plans only after all hard predicates pass.
A preference cannot convert an invalid or unavailable result into a valid result.

## Assumption registry

Give each assumption one identifier, version, owner, scope, and affected claim.
Record the evidence method, freshness rule, revocation rule, and failure result.

Classify an assumption as one of these kinds:

- cryptographic
- compiler
- proof-system
- ledger
- wallet or custody
- oracle or registry
- resolver or solver
- Runtime or indexer
- relay, bridge, or finality
- availability or liveness

`AssumptionsVerified(H, A.witnesses)` means that every required evidence check
passes for the selected claim and settlement level. It does not prove an
unobservable real-world fact.

An unverifiable dependency remains an explicit hypothesis in `H`. Its status
must not be `verified`. A missing, stale, revoked, contradictory, or unsupported
assumption produces `unavailable` or `invalid` as its declared rule requires.

## Candidate correctness judgment

Quantify the candidate theorem over signing state, execution state, intent,
plan, trace, outcome, artifacts, assumptions, domain, nonce, and settlement
level.

Use this architecture-neutral statement:

```text
for all S_sign, S_exec, I, P, T, O, A, H, D, N, L:
  wellTyped(I)
  and I.domain = D
  and I.nonce = N
  and AuthorizationValid(I.authorization, S_exec, D, N)
  and SnapshotFresh(A.snapshot, A.querySet, A.mutability, S_exec)
  and ResolverBound(
        A.resolverId,
        A.codeHash,
        A.implementation,
        A.upgradeState)
  and AssumptionsVerified(H, A.witnesses)
  and RefinementChainBound(I, A, P)
  and verifyPlan(I, S_sign, S_exec, A, P)
        = VerificationCertificate.valid
  and executes(P, S_exec, A.asyncBoundary, T, O)
  and SettlementPredicate(L, I.finalityPolicy, T, O)
  implies authorizedAt(I, H, L, view(I.authorizer, D, L, T, O))
```

`authorizedAt` means that all applicable hard predicates pass at level `L`.
It also requires no undeclared effect in the complete trace and outcome.

The theorem does not imply destination finality from an internal-ledger result.
It does not imply liveness, economic optimality, or legal enforceability.

The theorem must bind these identities through its objects or premises:

- network and ledger
- verifying contract and upgrade state
- Core and serializer
- compiler and circuit
- proof parameters
- resolver implementation and code hash
- signing and execution state
- state sequence and nonce domain
- validity interval
- signers and approval scope
- assets, effects, fees, and disclosures
- capabilities, assumptions, and failure outcomes

Freeze subsidiary claim statements for no-extra-spend, no-diverted-change,
no-extra-mint-or-burn, fee compliance, signer compliance, disclosure compliance,
capability non-escalation, replay rejection, cancel-or-fill exclusivity, refund
authorization, residual correctness, settlement correspondence, and composition.

S01 does not mechanize these claims. S04 selects the normative proof environment
and starts the theorem spine.

## Atomic-swap negative vector

Use the current atomic swap as the first executable falsifier. Keep its Core
contract and generated Compact artifact unchanged.

The valid plan transfers only the two authorized swap assets between the named
parties. The mutant adds one payment to an unauthorized third party.

The vector must satisfy these conditions:

- The baseline plan passes every available S01 check.
- The mutant preserves the baseline authorized effects.
- The mutant adds one representable payment effect.
- The mutation changes no signed hard predicate.
- The verifier rejects the mutant before signing.
- The result uses the stable reason `UNAUTHORIZED_EXTRA_EFFECT`.
- Removing the extra effect restores the baseline result.

The vector tests the plan verifier and effect projection. It does not claim
proof-system, Compact, ZKIR, or ledger correspondence.

## Evidence and validation

Place S01 evidence under `evidence/s01-intent-theorem-freeze/`. Add schemas under
`schemas/intent/`. Add focused tests under `tests/`.

Produce at least these evidence files:

- `terminology.json`
- `lifecycle-objects.json`
- `observation-model.json`
- `hard-predicates.json`
- `optimization-preferences.json`
- `assumption-registry.json`
- `intent-safety-judgment.json`
- `atomic-swap-extra-effect.json`
- `validation-report.json`
- `evidence-manifest.json`

The validator must check these acceptance predicates:

1. Every required term has one definition and no unresolved alias.
2. Every lifecycle object has one primary category and one authority boundary.
3. Every hard predicate has a signed binding or an explicit external premise.
4. No optimization preference enters `authorizedAt`.
5. The theorem contains every required variable, premise, binding, and claim.
6. Every assumption has a scope, evidence rule, and failure result.
7. Every observation field has a visibility and declassification rule.
8. The atomic-swap baseline passes.
9. The extra-effect mutant fails with the required reason.
10. The active semantic-scope version and digest remain unchanged.

Record input and output SHA-256 digests in the evidence manifest. Mark the S01
result `specified-only` until the validator runs successfully.

## Failure and rollback

Stop S01 when a normative term has two incompatible meanings. Stop S01 when an
effect is invisible to the verifier's complete effect projection.

Stop S01 when the theorem requires an architecture-specific Core operation.
Move that operation to the S02 comparison instead.

Reject the package when the extra-effect mutant passes. Do not widen the signed
intent to make the mutant valid.

Rollback removes only S01 artifacts and their manifest entry. Rollback does not
change the frozen Core snapshot or any immutable prior evidence.

## Semantic-scope transition

Record S01 as an evidence-only transition. Retain version `0.0.0-e00.2` and its
current digest.

The journal entry must state that S01 freezes an intent interface. It must also
state that S02 still owns the architecture decision.

## Design self-review

- The design contains no placeholder.
- The design selects no S02 architecture.
- The design adds no Core feature.
- The theorem quantifies every prompt-required variable.
- Hard predicates remain separate from preferences.
- Disclosure remains separate from effect authorization.
- Assumptions cannot become facts through naming alone.
- The negative vector uses an effect supported by the current Core model.
- The gate fails closed before signing.
- The result makes no proof, backend, ledger, or ACTUS compatibility claim.
