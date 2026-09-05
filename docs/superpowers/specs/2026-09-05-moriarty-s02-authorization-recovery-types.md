# S02 authorization and recovery type sketch

Superseded design: use the [delegated three-expert decision](2026-09-05-moriarty-s02-common-design-decision.md)
for implementation. This earlier sketch is retained to make the reviewed defects
and their dispositions inspectable; its inline types are not the corrected source.

Status: specified-only planning draft. Classification: recommendation derived
from the independently reviewed S02 designs. This document is not an
implementation, model result, Council decision, proof, candidate result, or S02
gate result.

## Main-review disposition

The three carrier issues identified in the initial draft are corrected below.
Transaction time is bound explicitly. Core choice identifiers remain strings.
Error and warning carriers preserve the declared frozen Python values; separate
validation predicates constrain valid observations to frozen emission rules.
This remains a proposed type sketch awaiting the user-delegated three-expert
design decision, not a validated model or a Council gate decision.

No model logic is authorized by this draft or its file presence.

## Purpose and authority

This sketch fixes the proposed common Quint types before model logic is written.
It consumes the reviewed S02 model-comparison and observation/authorization
designs and the existing effect and exact-parent bookkeeping foundations. Those
sources remain authoritative. If this sketch conflicts with them, the reviewed
source design wins.

The package is a plain Quint shared-state model. It has Alice, Bob, and Mallory
as opaque principals. It models typed external evidence dispositions; it does
not model cryptography. Resolve, pre-sign check, sign, environment change,
execution verification, and commit remain separate actions.

The proposed implementation files are:

- `specs/quint/s02/observations.qnt`: complete neutral observation carriers;
- `specs/quint/s02/authorization.qnt`: policies, evidence, authority registries,
  lifecycle records, and pure checks;
- `specs/quint/s02/authorization_harness.qnt`: a non-candidate installment and
  recovery harness for both signing profiles;
- `specs/quint/s02/authorization_test.qnt`: deterministic scenario tests.

The observation carrier and authorization/recovery model form one review unit.
Neither file contains candidate execution logic.

## Existing imports

The sketch reuses these existing definitions without changing them:

```quint
import effects.* from "./effects"
import consumption.* from "./consumption"
```

In particular, it reuses `Principal`, `Asset`, `Location`, `Transfer`, `Ledger`,
`Domain`, `AuthorityKey`, `Parent[p]`, `Entry[p]`, `canApply`,
`applyTransfers`, `policyAllows`, and the exact-parent consumption guards and
updates. Candidate-state, continuation, artifact/call, and resolved-plan data
are the only intentionally generic payloads below.

## Complete observation shapes

Core observations keep Core accounts and payments distinct from the external
wallet/escrow ledger and complete transfer effects.

```quint
type OptionalInt = NoInt | IntValue(int)

type CoreChoiceId = str

type CoreError =
  | NoCoreError
  | CoreErrorCode(str)

type CoreWarning = {
  code: str,
  requested: OptionalInt,
  paid: OptionalInt,
}

type CoreAccount = {owner: Principal, asset: Asset}

type CorePayment = {
  source: CoreAccount,
  recipient: Principal,
  asset: Asset,
  quantity: int,
}

type CoreStateObservation[continuation] = {
  accounts: CoreAccount -> int,
  choices: CoreChoiceId -> OptionalInt,
  continuation: continuation,
  minimumTime: int,
}

type CoreResultObservation[continuation] = {
  accepted: bool,
  error: CoreError,
  warnings: List[CoreWarning],
  payments: List[CorePayment],
  state: CoreStateObservation[continuation],
  reductions: int,
}

type SemanticOutcome =
  | FundingAccepted
  | Settlement
  | VoluntaryRefund
  | DeadlineRefund
  | FirstInstallment
  | SecondInstallment
  | Cancellation
  | Recovery
  | Rejected(CoreError)

type EvidenceDisposition = EvidenceValid | EvidenceUnavailable | EvidenceInvalid

type CandidateObservation[state, continuation, artifact, plan] = {
  predecessor: state,
  proposedSuccessor: state,
  artifactAndCall: artifact,
  resolvedPlan: plan,
  transactionTime: int,
  effects: List[Transfer],
  outcome: SemanticOutcome,
  coreProjection: CoreResultObservation[continuation],
  effectEvidence: EvidenceDisposition,
}
```

`CoreChoiceId` preserves the frozen Core string identifier. Candidate fixtures
supply finite literal identifier sets; the shared carrier assigns no
candidate-specific constructors. The bounded map is initialized over each
fixture's complete identifier set. `NoInt` represents an absent Core choice,
while `IntValue(0)` represents a present zero. Its abstraction map must preserve
that distinction and reject out-of-fixture identifiers, not silently omit them.

`CoreError` and `CoreWarning` preserve the declared Python carriers without
normalizing unknown values. `NoCoreError` represents `None`; `CoreErrorCode`
preserves the exact string. Warning code and both optional integers remain
independent. `validCoreObservation` must reject values outside the frozen
emission rules rather than making the carrier lossy. Frozen Core declares
these fields in `moriarty/core.py:187` and `moriarty/core.py:203`.
Its current errors are `time_before_state`, `contract_closed`, `input_required`,
`no_matching_input`, `choice_out_of_bounds`, and `non_positive_deposit`.
Its warning emitters are `non_positive_payment` and `partial_payment`.
These source observations specify validation work; they are not correspondence
evidence.

`continuation` must be a complete continuation payload. For Candidate A, a
node index alone is insufficient: the instantiation must carry the complete
constructor table together with the current index. `artifactAndCall` likewise
contains the complete candidate artifact identity and call payload, not a phase
label or a two-valued stand-in.

Core warnings and payments are ordered lists. A successful deposit adds a
wallet-to-escrow `Transfer` to `effects` but adds no `CorePayment`. A rejected
observation preserves the complete predecessor Core state, has empty warnings,
payments, and effects, and records zero reductions.

## Environment and condition shapes

```quint
type StateAnchor = Anchor0 | Anchor1 | Anchor2
type ImplementationVersion = ImplementationV1 | ImplementationV2
type EnforcementMechanism = LocalCheckerV1 | LocalCheckerV2

type Environment = {
  physicalTime: int,
  stateAnchor: StateAnchor,
  implementationVersion: ImplementationVersion,
  enforcementMechanism: EnforcementMechanism,
}

pure val TRANSACTION_TIMES: Set[int] = Set(1, 2, 100, 101)

type Capability =
  | Fund
  | Dispose
  | FillSlot1
  | FillSlot2
  | CancelParent
  | RecoverEscrow

type Disclosure = PublicSummary | CounterpartyTerms | PrivateEffectEvidence

type PolicyCondition =
  | RequiresIncoming(List[Transfer])
  | RequiresOutcome(SemanticOutcome)
  | RequiresParentCancelled(AuthorityKey)
  | BeforeDeadline(int)
  | AtOrAfterDeadline(int)

type Validity = {notBefore: int, notAfter: int}

type RefundRule =
  | NoRefund
  | RefundTo({
      destination: Location,
      asset: Asset,
      maximumQuantity: int,
      condition: PolicyCondition,
    })

type CancellationRule = NotCancellable | CancellableParent(AuthorityKey)
```

Environment transitions may advance `physicalTime`, `stateAnchor`, or the
implementation/mechanism version. They never move backward or wrap to an
earlier value. Proposed transaction time is adversarial input and may be less
than the current minimum time; physical environment time never moves backward.

## Policy, evidence, and authority shapes

```quint
type SigningProfile = SignAfterResolve | SignBeforeResolve

type BoundedIntent = {
  domain: Domain,
  principal: Principal,
  key: AuthorityKey,
  allowedEffects: List[Transfer],
  conditions: List[PolicyCondition],
  capabilities: Set[Capability],
  disclosures: Set[Disclosure],
  cancellation: CancellationRule,
  enforcementMechanism: EnforcementMechanism,
}

type PlanBinding[plan] =
  | CompleteResolvedPlan(plan)
  | CompleteBoundedIntent(BoundedIntent)

type Policy[plan] = {
  principal: Principal,
  key: AuthorityKey,
  profile: SigningProfile,
  requiredEffects: List[Transfer],
  allowedEffects: List[Transfer],
  debitLocations: Set[Location],
  conditions: List[PolicyCondition],
  refundRule: RefundRule,
  validity: Validity,
  capabilities: Set[Capability],
  disclosures: Set[Disclosure],
  cancellation: CancellationRule,
  enforcementMechanism: EnforcementMechanism,
  implementationVersion: ImplementationVersion,
  binding: PlanBinding[plan],
}

type SignatureToken = {
  principal: Principal,
  key: AuthorityKey,
  policyRevision: int,
}

type SignedPolicy[plan] = {
  policy: Policy[plan],
  signatureToken: SignatureToken,
}

type SignatureEvidence[plan] = {
  signedPolicy: SignedPolicy[plan],
  boundEnvironment: Environment,
  disposition: EvidenceDisposition,
}

type StateEvidence[state] = {
  completeState: state,
  boundEnvironment: Environment,
  disposition: EvidenceDisposition,
}

type EffectEvidence = {
  completeEffects: List[Transfer],
  boundEnvironment: Environment,
  disposition: EvidenceDisposition,
}

type AssumptionEvidence = {
  boundEnvironment: Environment,
  cryptographic: EvidenceDisposition,
  authenticatedState: EvidenceDisposition,
  completeEffectExtraction: EvidenceDisposition,
  settlement: EvidenceDisposition,
}

type EvidenceLevel = AbstractModelEvidence | CoreResultEvidence

type SettlementEvidence[continuation] = {
  coreResult: CoreResultObservation[continuation],
  level: EvidenceLevel,
  boundEnvironment: Environment,
  disposition: EvidenceDisposition,
}

type AuthorityRecord[plan] =
  | AuthorityVacant
  | AuthorityRegistered(SignedPolicy[plan])
  | AuthorityConsumed({policy: SignedPolicy[plan], consumptionRevision: int})

type AuthorityRegistry[plan] = AuthorityKey -> AuthorityRecord[plan]

type ParentRecord[plan] =
  | NoParent
  | ActiveParent({
      parent: Parent[Policy[plan]],
      entry: Entry[Policy[plan]],
    })

type ParentRegistry[plan] = AuthorityKey -> ParentRecord[plan]
```

`SignatureToken` records that a signing action occurred. It is not proof that a
real signature is authentic. Only `SignatureEvidence.disposition` records the
model's typed external premise, and unavailable or invalid evidence fails
closed. The four assumption fields remain separate so one accepted premise
cannot hide another unavailable premise.

Every registry is initialized over the complete finite set of modeled keys.
Records retain the complete policy or parent; changing plan identity does not
create a fresh nonce namespace. The installment parent uses Alice nonce `0`.
The recovery policy is a distinct Alice nonce `1` record and never reuses or
reactivates the cancelled parent.

## Lifecycle records

```quint
type Context[state, plan] = {
  candidateState: state,
  ledger: Ledger,
  environment: Environment,
  authorityRegistry: AuthorityRegistry[plan],
  parentRegistry: ParentRegistry[plan],
}

type AfterResolveCheckedContent[state, continuation, artifact, plan] = {
  completePlan: plan,
  checkedContext: Context[state, plan],
  checkedArtifactAndCall: artifact,
  checkedCoreExpectation: CoreResultObservation[continuation],
}

type BeforeResolveCheckedContent[state, plan] = {
  boundedIntent: BoundedIntent,
  checkedContext: Context[state, plan],
  checkedEnforcementMechanism: EnforcementMechanism,
}

type AfterResolvePreSignCheck[state, continuation, artifact, plan] = {
  policy: Policy[plan],
  checked: AfterResolveCheckedContent[state, continuation, artifact, plan],
  disposition: EvidenceDisposition,
}

type BeforeResolvePreSignCheck[state, plan] = {
  policy: Policy[plan],
  checked: BeforeResolveCheckedContent[state, plan],
  disposition: EvidenceDisposition,
}

type PreSignCheckRecord[state, continuation, artifact, plan] =
  | AfterResolveCheck(AfterResolvePreSignCheck[state, continuation, artifact, plan])
  | BeforeResolveCheck(BeforeResolvePreSignCheck[state, plan])

type OptionalPreSignCheck[state, continuation, artifact, plan] =
  | NoPreSignCheck
  | SomePreSignCheck(PreSignCheckRecord[state, continuation, artifact, plan])

type VerificationRecord[state, continuation, artifact, plan] = {
  policies: Principal -> AuthorityRecord[plan],
  observation: CandidateObservation[state, continuation, artifact, plan],
  transactionTime: int,
  predecessorContext: Context[state, plan],
  currentEnvironment: Environment,
  signatureEvidence: Principal -> SignatureEvidence[plan],
  stateEvidence: StateEvidence[state],
  effectEvidence: EffectEvidence,
  assumptionEvidence: AssumptionEvidence,
  settlementEvidence: SettlementEvidence[continuation],
  disposition: EvidenceDisposition,
}

type OptionalVerification[state, continuation, artifact, plan] =
  | NoVerification
  | SomeVerification(VerificationRecord[state, continuation, artifact, plan])

type OptionalPreparedCancellation[plan] =
  | NoPreparedCancellation
  | SomePreparedCancellation(Entry[Policy[plan]])

type LifecyclePhase =
  | Drafted
  | Resolved
  | PreSignChecked
  | Signed
  | ExecutionVerified
  | Committed
  | RejectedPhase
```

For `SignAfterResolve`, `AfterResolveCheck.checked.completePlan` must equal the
complete plan later signed and verified. For `SignBeforeResolve`, only the
`BeforeResolveCheck` variant is valid; it checks the complete bounded intent and
enforcement identity and has no concrete plan, artifact/call, or expected Core
result fields. Both profiles require full execution verification before
financial or authority commitment.

## Pure-function interface

The functions below are signatures, not implementations. Functions named
`compute` or `apply` return candidate records or state. They never authorize an
action by themselves.

```quint
pure def validCoreObservation[c](before: CoreStateObservation[c],
                                 result: CoreResultObservation[c],
                                 effects: List[Transfer]): bool
pure def validFrozenCoreError(error: CoreError): bool
pure def validFrozenCoreWarning(warning: CoreWarning): bool
pure def rejectionPreserved[c](before: CoreStateObservation[c],
                               result: CoreResultObservation[c],
                               effects: List[Transfer]): bool

pure def validEnvironment(environment: Environment): bool
pure def environmentFresh(expected: Environment, current: Environment): bool

pure def conditionsHold[p](policy: Policy[p],
                           actualEffects: List[Transfer],
                           outcome: SemanticOutcome,
                           parentRegistry: ParentRegistry[p],
                           transactionTime: int): bool
pure def principalAllows[s, c, a, p](policy: SignedPolicy[p],
                                    observation: CandidateObservation[s, c, a, p],
                                    context: Context[s, p]): bool
pure def affectedPrincipals(effects: List[Transfer]): Set[Principal]
pure def allAffectedPrincipalsAllow[s, c, a, p](
  policies: Principal -> AuthorityRecord[p],
  observation: CandidateObservation[s, c, a, p],
  context: Context[s, p]): bool

pure def computeAfterResolvePreSignCheck[s, c, a, p](
  policy: Policy[p], plan: p, context: Context[s, p], artifactAndCall: a,
  expectedCore: CoreResultObservation[c]): AfterResolvePreSignCheck[s, c, a, p]
pure def computeBeforeResolvePreSignCheck[s, p](
  policy: Policy[p], context: Context[s, p]): BeforeResolvePreSignCheck[s, p]
pure def canSignAfterResolve[s, c, a, p](
  check: AfterResolvePreSignCheck[s, c, a, p], plan: p,
  current: Context[s, p]): bool
pure def canSignBeforeResolve[s, p](
  check: BeforeResolvePreSignCheck[s, p], current: Context[s, p]): bool

pure def computeVerification[s, c, a, p](
  policies: Principal -> AuthorityRecord[p],
  observation: CandidateObservation[s, c, a, p],
  predecessor: Context[s, p], current: Context[s, p],
  signatures: Principal -> SignatureEvidence[p],
  stateEvidence: StateEvidence[s], effectEvidence: EffectEvidence,
  assumptions: AssumptionEvidence,
  settlement: SettlementEvidence[c]): VerificationRecord[s, c, a, p]
pure def verificationValid[s, c, a, p](
  record: VerificationRecord[s, c, a, p],
  current: Context[s, p]): bool
pure def canCommit[s, c, a, p](record: VerificationRecord[s, c, a, p],
                               actual: CandidateObservation[s, c, a, p],
                               current: Context[s, p]): bool

pure def canRegisterAuthority[p](registry: AuthorityRegistry[p],
                                 policy: SignedPolicy[p]): bool
pure def applyRegisterAuthority[p](registry: AuthorityRegistry[p],
                                   policy: SignedPolicy[p]): AuthorityRegistry[p]
pure def canConsumeAuthority[p](registry: AuthorityRegistry[p],
                                policy: SignedPolicy[p], revision: int): bool
pure def applyConsumeAuthority[p](registry: AuthorityRegistry[p],
                                  policy: SignedPolicy[p], revision: int): AuthorityRegistry[p]

pure def recoveryTransfer[p](parent: Parent[Policy[p]], entry: Entry[Policy[p]]): Transfer
pure def canRecover[p](recoveryPolicy: SignedPolicy[p],
                       parent: Parent[Policy[p]], entry: Entry[Policy[p]],
                       registry: AuthorityRegistry[p], ledger: Ledger,
                       actual: Transfer): bool
pure def applyRecoveryLedger(ledger: Ledger, actual: Transfer): Ledger
pure def applyRecoveryAuthority[p](registry: AuthorityRegistry[p],
                                   recoveryPolicy: SignedPolicy[p],
                                   revision: int): AuthorityRegistry[p]
```

The intended `allAffectedPrincipalsAllow` semantics are conjunction, not an
implicit joint signer. Every wallet or escrow debit occurrence requires the
applicable principal's policy. Bob's choice cannot authorize Alice's assets.
Each policy also checks its complete incoming consideration and conditional
outcome; debit coverage alone is insufficient. Effect comparison preserves
ordered occurrences and multiplicity.

`computeVerification` copies `observation.transactionTime` into the verification
record. `verificationValid` requires those values to agree and the time to be
in the fixture's declared `TRANSACTION_TIMES` domain. `principalAllows` and
`allAffectedPrincipalsAllow` pass only `observation.transactionTime` to
`conditionsHold`; no independent time argument can disagree with the proposal.

`canCommit` requires `actual == record.observation`,
`record.transactionTime == actual.transactionTime`, and exact freshness of the
predecessor context, environment, implementation version, and consumption
state. A substituted observation or time requires fresh verification before
any ledger or authority update.

## Harness state and action boundary

The non-candidate harness is parameterized only by signing profile and the four
complete candidate payload types:

```quint
type HarnessState[state, continuation, artifact, plan] = {
  lifecycle: LifecyclePhase,
  context: Context[state, plan],
  preparedCancellation: OptionalPreparedCancellation[plan],
  preSignCheck: OptionalPreSignCheck[state, continuation, artifact, plan],
  verification: OptionalVerification[state, continuation, artifact, plan],
  lastDisposition: EvidenceDisposition,
}
```

The model has separate guarded actions for `resolve`, `preSignCheck`, `sign`,
`changeEnvironment`, `executionVerify`, `commitFirstFill`, `prepareCancel`,
`rejectStaleCancel`, `commitFreshCancel`, `verifyRecovery`, and
`commitRecovery`. Each action assigns the complete harness state. There is no
blanket stutter.

The installment fixture starts with ten TokenA units in Alice's escrow. Its two
five-unit parent slots use Alice nonce `0`. The recovery policy uses Alice nonce
`1`, permits only the exact remaining escrow-to-Alice-wallet transfer, and
requires the parent to be cancelled.

The harness must reach both paths under both signing profiles:

- `cancel-wins/recovery-before-any-fill`: cancel the unused parent, verify the
  separate recovery policy, and refund ten units;
- `fill-wins/recovery-after-first-fill`: prepare cancellation, commit slot 1,
  reject the stale prepared cancellation, accept a fresh cancellation, and
  refund the remaining five units. Slot 2 remains unauthorized.

Both terminal states satisfy:

```text
escrowBalance + paidToBob + refundedToAlice = 10
```

The recovery action reduces escrow, increases Alice's wallet balance, and
consumes the recovery nonce. It does not consume a parent installment slot or
reactivate cancelled allowance.

## E00 correspondence boundary

This package only fixes the carrier that later candidate models and the
independent correspondence checker consume. It does not implement a Core
interpreter or compare a generated result with Python Core.

The later Candidate A instantiation must supply a complete continuation payload
and preserve the exact Python result fields: accepted status, error, ordered
warnings with requested/paid values, ordered payments, accounts, choices,
continuation, minimum time, and reductions. The independent checker remains
outside candidate extraction and must detect a corrupted abstraction map.

The paired deadline cases remain later Candidate A/E00 work:

- no supplied input at the deadline commits timeout refunds;
- supplied input after timeout reduction reaches `Close`, returns
  `contract_closed`, and restores the complete original state and refund
  effects.

This sketch does not change Core, introduce a Core constructor, or treat the
recovery refinements as accepted Core semantic motions.

## Exclusions and evidence language

This design does not provide candidate execution, candidate-specific state
transitions, a Core correspondence result, an abstraction-map test, Apalache
model checking, cryptographic authenticity, ledger execution, Compact
correspondence, proof, architecture selection, or an S02 gate result.

Any future sampled run is simulation evidence only. Every external evidence
disposition remains a modeled premise, even when its value is `EvidenceValid`.
No model value may be described as a real signature or proof.

## Delegated design signoff

The user subsequently delegated this design decision to GPT-6 Astra, Fable 5.1,
and Grok 4.6 acting as formal-methods experts, and instructed the workstream to
implement their agreed design while the user is AFK. This supersedes the earlier
request for direct human approval. Do not ask for that approval again.

The three proposals must still be received and reconciled before model logic is
written. Record the agreed changes and preserve any unresolved dissent. Their
decision supplies design signoff only: it does not establish model-checking
results, architecture selection, S02 acceptance, or a requested Council gate.

The proposed implementation review boundaries for that decision are:

1. **Recommended:** approve `observations.qnt`, `authorization.qnt`, the two
   profile harnesses, and recovery scenarios as one common-foundation package;
2. split `observations.qnt` into a separately reviewed prerequisite package,
   then implement authorization/recovery against the approved carrier.

Candidate-specific state and constructor tables, artifact/call shapes, and
resolved-plan shapes are supplied by later candidate plans through the four
generic parameters. Frozen Core choice identifiers remain exact strings in the
Core projection. Each bounded fixture supplies its finite identifier set and
documents its independent abstraction map. These are not choices required to
approve this common type sketch.
