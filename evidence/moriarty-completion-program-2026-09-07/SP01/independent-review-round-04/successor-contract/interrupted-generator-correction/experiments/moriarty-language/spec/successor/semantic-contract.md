# SP01.3 successor semantic and signing contract

Status: **PROPOSAL**. Not a semantic freeze. Not registered. Not accepted.
Pending SP01.2 challenge-map reconciliation and user-selected cross-provider
majority for consequential choices. Independent GPT-6 reviews this output.
Full source grammar is SP02. Executable K Core is SP03. This file defines the
interfaces those sprints consume.

Author: Grok 4.6 high. Worktree only. No runtime, proof, native, or old-profile
edits. Old `moriarty-bounded-atomic/1` bytes remain a distinct experimental
profile. Their SHA domains are not successor crypto.

Companion JSON (`signing-display-schema.json` `$id`
`https://moriarty.local/spec/successor/signing-display-schema/0.2.0-proposal`,
`x-schemaVersion` `moriarty-succ-sign/0`) is the closed wire. Prose here must
not invent constructors absent from `$defs`. Circular "see the other file"
without a named def is a defect.

## 0. Classification and pins

Every sentence below is one of: source fact, repository observation, inherited
constraint, proposal, open check. Unmarked normative tables in sections A-I
are **proposal**.

Pinned inputs (SHA-256 of exact worktree bytes):

| Path | SHA-256 |
| --- | --- |
| `deliverables/defi-language-design-2026-09-07/LANGUAGE-DESIGN.md` | `bd9a7af78e400da7d8ba4d57abc9693665579ea78e7fdb98004f231ecbfd0aa8` |
| `deliverables/defi-language-design-2026-09-07/action-targets.csv` | `7b9b245b97a4d63f6cc946c9dd7e7f6c3773df4cf47b2540d82b5ffc6cfe3d2f` |
| `openspec/REPORT-RECONCILIATION-2026-09-07.md` | `a1a1661fc0d554d8e469651fdbd42c41e7c9e2115c8550f369742dcee94bd5da` |
| `openspec/sprints/sp01-financial-contract-and-execution-admission.md` | `fdb586d9bf05f2fbf7e9c2294be49854528db20e575faf9aaa1144b033908172` |
| `openspec/sprints/sp02-complete-mori-authoring-frontend.md` | `d5a0bfca8edf795e7724b1a61b0ab959851fdba9cfe0c3b3f84ac289b2dfee28` |
| `openspec/sprints/sp03-executable-bounded-semantics-in-k.md` | `1bbe0401f74cf236c2150486bfec719db027648d7aad52ea97952e6b56919ab9` |
| `evidence/moriarty-design-sprint-2026-09-06/worked-examples.json` | `46aee15de568ce1bab7372588866aac5f3e62ca7ecac73ee1ee23de0610ecf2b` |
| `evidence/moriarty-completion-program-2026-09-07/SP01/loan-swap-subset-01/design.md` | `4ec089205fc69b94b80cd68b94a82b17c16d98e0240355b9977506983da9d7ca` |
| `experiments/moriarty-language/spec/bounds.json` (old profile, immutable) | `b548641a1a9d74bab68ba699ffb1e2350fa0889d61b8704e98216f9d4a6c3664` |
| `experiments/moriarty-language/src/types.ts` (old profile, immutable) | `5cc7356e9a38a74f1a202c7c6c8d7ac6d1ef6eac8fe0d8013beb242e3de604d0` |
| `experiments/moriarty-language/src/codec.ts` (old profile, immutable) | `8575d98e630fed5b6e68b23c574aec450fc2bbce1b83b057ce6154e43610b7e8` |
| `experiments/moriarty-language/spec/numeric-profile.json` | `6d88f694bf8af8c5b7dd75fce76f58fe0d14fc68c2782dd0d3fb885ca4a7eb15` |

Inherited constraints (RP01, LANGUAGE-DESIGN, program rules): finite registered
bounds; four mandatory acceptance claims; Preview sole public target; no host
boolean as acceptance; `Prepared` is not acceptance; no unbounded
strings/arrays/maps/recursion/callbacks; refunds cannot restore gross debit;
fees count against net goals; residual duties survive partial progress; unknown
extensions and revoked keys reject. These are not reopened by this proposal.

SP01.2 live work is not an input. No row here implies that map is approved.

Narrow correction this pass (actual contradiction, not a generator rewrite):
`ResidualCapability.allowedEffectSet` items are `CoreOp`, not `EffectKind`.
The interrupted accrue example listed `Accrual`/`DueCreated`. Those strings are
effect constructors. The document, successor, loan state, post-state and
prepared residual now list `DebtAccrue`/`DebtCreate`/`RequestCancel`. Schema
was not widened. Paths and before/after hashes are in FOREMAN_REPORT.json.

## A. Named finite domains

Proposal: every value inhabits a named finite domain. Profile admission names
the simultaneous maxima. Cartesian products of maxima are not encodable.
`ProfileBody` in the example registry (`profileName` `moriarty-successor/0`):
`sourceUtf8Bytes` 65536, `signingUtf8Bytes` 65536, `maxNodes` 4096,
`maxSidecarBytes` 16384, `maxTotalVerificationWork` 65536, `predecessorFanIn`
8, `collectionCapacity` 128.

### A.1 Primitive domains (proposed widths, not registered)

| Domain | Width | Inclusive range | Wire |
| --- | --- | --- | --- |
| `UInt64` | 64 | `0 .. 2^64-1` | `UInt64Text` |
| `UInt128` | 128 | `0 .. 2^128-1` | `UInt128Text` |
| `SInt128` | 128 | `-2^127 .. 2^127-1` | `SInt128Text` |
| `Scale` | 8 | `0 .. 18` | `ScaleText` |
| `Bool` | 1 | `{false,true}` | JSON boolean |
| `Identifier` | 1..64 | `[A-Za-z][A-Za-z0-9_]*` | ASCII |
| `Party` | Identifier | instance roster | Identifier |
| `AssetId` | Identifier | instance roster | Identifier |
| `NetworkId` / `DeploymentId` | Identifier | roster | Identifier |
| `Time` | UInt64 | UTC epoch seconds | unsigned decimal |
| `EventOrder` | UInt64 | instance-local ordinal | unsigned decimal |
| `Digest` | 256 bits | SHA-256 | 64 lowercase hex |
| `AsciiLabel` | 1..128 | `[A-Za-z0-9][A-Za-z0-9 _./:-]{0,127}` | ASCII |

Canonical integer spelling: `0` or `-?[1-9][0-9]*`. Reject leading zeros, `+`,
`-0`, separators, exponents, JSON numbers. `UInt64Text` pattern
`^(0|[1-9][0-9]{0,19})(?![\s\S])` (maxLength 20). `UInt128Text`
`^(0|[1-9][0-9]{0,38})(?![\s\S])` (maxLength 39). `SInt128Text`
`^(0|-?[1-9][0-9]{0,38})(?![\s\S])`. `ScaleText`
`^(0|[1-9]|1[0-8])(?![\s\S])`. Identifier/Party/AssetId/NetworkId/DeploymentId
use `^[A-Za-z][A-Za-z0-9_]{0,63}(?![\s\S])`. Digest
`^[0-9a-f]{64}(?![\s\S])`. The true-end group `(?![\s\S])` rejects a final
newline, CR, or U+2028. ECMA-262 `$` is not used.

`2^64` is twenty canonical digits, so `UInt64Text` may schema-pass.
`2^128` is thirty-nine digits, so `UInt128Text` may schema-pass.
`DomainValidation` (pre-hash) still rejects any parse outside the inclusive
range. `overflowDeferredToProof` is false. Intermediates use the result width
unless a named `checked_wide` form is admitted. Overflow, underflow, and
division by zero reject the whole action.

### A.2 Indexed financial domains

`Amount`: `{tag:"Amount", asset:AssetId, value:UInt128}`. Nonnegative tokens.
Asset is a ledger identity, not a unit name.

`Debt`: required fields `tag, debtId, controller, debtor, creditor,
denomination, principal, accrued, outstanding, startDate, nextPaymentDate,
accrualPeriodSeconds, allocationRule, negativeRateDisposition, liabilityCap,
settlementAsset, conversionMantissa, conversionScale, conversionRounding,
status`. `principal`, `accrued`, `outstanding`, `liabilityCap` are UInt128
denomination quanta. `status` is `Outstanding|Settled|WrittenOff|Capitalized`.
`allocationRule` is `AccrualFirst|PrincipalFirst|ProRata`.
`negativeRateDisposition` is `Reject|FloorZero|CreditAccrued`.
`conversionRounding` is `floor|ceil|none`. Transfer authority never implies
`DebtCreate`. `SInt128` is for P&L, funding, and signed rate mantissas, not a
hidden debt sign.

`Shares`: `{tag:"Shares", vault:Identifier, value:UInt128}`.

`Quantity`: `{tag:"Quantity", mantissa:SInt128, scale:Scale, units:UnitTerm[0..8]}`.
Each `UnitTerm` is `{symbol:Identifier, exponent:SInt}` with unique symbols.
Unit-vector length at most 8. Mul adds exponents. FloorDiv/CeilDiv subtract
exponents. The result is `Quantity`. Zero vector is dimensionless.

`Rate`: `{tag:"Rate", mantissa:SInt128, scale:Scale}` meaning
`mantissa * 10^(-scale)`.

`Price`: `{tag:"Price", base:AssetId, quote:AssetId, mantissa:UInt128,
scale:Scale, direction:"basePerQuote"}`. `direction` is a closed const.
Meaning: `mantissa * 10^(-scale)` units of `base` per one unit of `quote`.
The swap example has `base=AssetB`, `quote=AssetA`, `mantissa=19743`,
`scale=4`, so 1.9743 AssetB per AssetA. Display unit is "AssetB per AssetA
at scale 4". The old reversed display is forbidden.

`AmountPredicate`: `{tag:"Min"|"Exact", amount:Amount}`. Min allows
over-delivery. Exact rejects over-delivery.

`Dust`: `{asset:AssetId, value:UInt128}` output-asset threshold on Exchange
only.

`Remainder`: `{tag:"Remainder", numerator:UInt128, denominator:UInt128,
unit:AssetId, location:"SourceReserve"|"NamedParty"|"DiscardRecorded"}`.
Remainder is not a Transfer of the input asset.

Identities unique in the instance lifetime, including tombstones:
`PositionId`, `ObligationId` (`debtId`), `RequestId`, `MessageId`,
`EventClaimId`, `AccountId`, `DutyId`. Reuse rejects.

### A.3 Stored values and bounded records

`StoredValue` is the closed union: `StoredUInt128`, `StoredSInt128`,
`StoredBool`, `StoredText`, `Amount`, `Debt`, `Shares`, `Quantity`,
`StoredEnum`, `StoredOptionNone`, `StoredOptionSome`, `StoredRecord`,
`StoredCollection`, plus `Rate`/`Price` only where a write field's schema
permits them through those tagged records. No free JSON. Collection capacity
is `ProfileBody.collectionCapacity` including tombstones.

`Position`: `tag, positionId, owner, vault, lowerTick, upperTick, liquidity,
direction, status`.

`Request`: `tag, requestId, controller, asset, committed, filled, claimed,
status` with status `Pending|Claimable|Claimed|Cancelled|Expired`. Claimed
cannot cancel. Expiry does not erase a Claimed payment duty.

`Message`: `tag, messageId, sender, recipient, payloadBound, status,
finalityAssumption`.

`EventClaim` (DA13): `tag, claimId, eventId, outcomeIndex, claimant, stake,
status, evidenceAnchor, resolutionRule`. Supported Core ops:
`EventClaimOpen`, `EventClaimResolve`, `EventClaimSettle`. Unsupported:
unbounded off-ledger evidence payloads (bound by `payloadBound`/`stake` only).

`RewardAccount` (DA14): `tag, accountId, holder, vault, shareUnits,
entitlementRate, lastAccrual, nominalBalance, pendingEntitlement`. Slash
removes `shareUnits` and records `nominalBalanceUnchanged=true`. Slash does
not erase Debt. Rebase may change share units. It does not refresh work or
authority.

`Duty`: `tag, dutyId, kind, creditor, debtor, status, nominalOrAsset`.
Residual duties survive partial progress.

`LockRight`: `locker, asset, amount`.

`Balance`: `party, asset, value`.

## B. Lexical, source, Core layers

Three layers, never collapsed:

1. Source sugar (SP02). Action names on `OutcomeIntentDocument.allowedActions`
   (`swap`, `accrue`, `settle`, `genesis`, ...). File encoding well-formed
   UTF-8 without BOM. Identifiers as A.1. No float tokens. Comments counted in
   `sourceUtf8Bytes`. `sourceHash = SHA256(raw utf-8)` with empty domain and
   no NUL frame.
2. Core operations (`CoreOp` enum, 38 constructors). Residual permission.
3. Observable effects (`EffectKind` enum and `ExactEffect` union).

`allowedEffectSet` is layer 2. `exactEffects[].kind` is layer 3. Putting
`Accrual` in `allowedEffectSet` is a type error. Mapping:

| CoreOp | EffectKind image |
| --- | --- |
| `Transfer` | `Transfer` |
| `Fee` | `Fee` |
| `Exchange` | `Transfer` (and optional `Fee`). No `Exchange` effect tag |
| `Mint` / `Burn` | `Mint` / `Burn` |
| `Lock` / `Unlock` | `Lock` / `Unlock` |
| `DebtCreate` | `DueCreated` |
| `DebtAccrue` | `Accrual` |
| `DebtRepay` | `DueSettled` |
| `DebtWriteOff` | `WriteOff` |
| `DebtCapitalize` | `Capitalize` |
| `ShareMint` / `ShareBurn` / `ShareConvert` | same names |
| `PositionOpen` / `PositionAdjust` / `PositionClose` | same names |
| `RequestOpen` / `RequestFulfill` / `RequestClaim` / `RequestCancel` | same names |
| `MessageSend` / `MessageReceive` / `MessageRefund` | same names |
| `EventClaimOpen` / `Resolve` / `Settle` | `EventClaimOpened` / `Resolved` / `Settled` |
| `RewardAccrue` / `RewardClaim` / `Slash` / `Rebase` | `RewardAccrued` / `RewardClaimed` / `Slashed` / `Rebased` |
| `Observe` | `Observe` |
| `Genesis` / `Activate` / `Pause` / `Revoke` / `Migrate` | same names |

Authorization: the source action elaborates to one CoreOp. That CoreOp must
be in the current `residualAuthority.allowedEffectSet`. Each emitted
`ExactEffect.kind` must be in the image of that CoreOp. Assets must be in
`allowedAssets`. Recovery rights may invoke only
`RequestCancel|MessageRefund|Unlock|DebtRepay|Migrate`.

An ExactPlan `executionBody.action.name` of `accrue` elaborates to
`DebtAccrue`. `genesis` elaborates to `Genesis`. `swap`/`exchange` elaborate
to `Exchange` and require `dust`, `rateOrPrice`, `outPredicate`, `remainder`.
Non-exchange actions forbid those four fields (schema `allOf` if/then).
`exactEffects` minItems is 0. Genesis may emit a single `Genesis` effect with
no transfers.

### B.1 Core expression constructors (SP03 AST, not signing wire)

Signing documents carry `StoredValue` and `FieldBinding`, not a full AST.
SP03 must implement this finite expression set. Unsupported forms reject.

Literals: `LitUInt`, `LitSInt`, `LitBool`, `LitText`, `LitAmount`,
`LitQuantity`, `LitShares`, `LitRate`, `LitPrice`.
Reads: `ReadLocal`, `ReadPre`, `ReadArg`, `ReadObs`.
Projection/access: `ProjectField`, `ProjectIndex`, `AccessField`, `AccessIndex`.
Construction: `ConstructRecord`, `ConstructEnum`, `ConstructSome`,
`ConstructNone`, `ConstructCollection`.
Arithmetic: `Add`, `Sub`, `Mul`, `FloorDiv`, `CeilDiv`, `Eq`, `Lt`, `Lte`,
`Gt`, `Gte`, `Not`, `And`, `Or`.
Staging: `Require`, `Let`, `NextWrite`, `Ensure`, `Emit`.

No plain `/`. FloorDiv/CeilDiv require positive denominators. First failed
clause in lexical order is the diagnostic. Financial rollback discards
tentative writes and effects. Submission fees are outside that rollback.

### B.2 Keyword categories and precedence

Declaration: `agreement`, `profile`, `lifetime`, `horizon`, `unit`, `const`,
`state`, `observation`, `settlement`, `policy`, `status`, `effect`, `action`,
`reserve`, `recovery`, `enum`, `option`, `record`, `finite`.
Type: `UInt64`, `UInt128`, `SInt128`, `Bool`, `Amount`, `Debt`, `Shares`,
`Quantity`, `Rate`, `Price`, `Time`, `Party`, `AssetId`, `Text`.
Statement: `requires`, `let`, `next`, `emit`, `ensures`.
Composition (not Boolean): `seq`, `par`, `interleave`, `atomic`, `message`.
Boolean: `not`, `and`, `or`, `true`, `false`.
Rounding: `floor_div`, `ceil_div`.
Admin: `genesis`, `activate`, `revoke`, `migrate`, `pause`.
Reserved binders: `pre`, `post`, `remaining`.

Precedence, tightest to loosest: parentheses/calls/projection; Mul;
Add/Sub; one non-chained comparison; `not`; `and`; `or`. So
`not a == b` means `not (a == b)`. Complete EBNF is SP02.

### B.3 Staged evaluation

```text
Eval(profile, program, pre, action, observations, authority, predecessors)
  = Reject(code, span)
  | Prepared(post, effects, duties, residualAuthority, successors,
             remainingWork, claims)
```

`Prepared` is a candidate. K tests and host flags are not acceptance.
`PreparedBody` required:
`postStateHash, effects, duties, residualCapability, successors,
remainingWork, claims`.

Closed diagnostic families: `LEX_*`/`PARSE_*`, `TYPE_*`, `BOUND_*`,
`GUARD_FAILED`, `ENSURES_FAILED`, `ARITH_*`, `AUTH_*`, `INTENT_*`,
`OBLIGATION_*`, `REQUEST_*`/`MESSAGE_*`, `WORK_*`, `OBS_*`, `SPEC_*`,
`HISTORY_*`, `ACCEPT_*`, `LEDGER_*`.

## C. Signing documents and claims

Schema `oneOf`: `ExactPlanDocument` | `OutcomeIntentDocument`.
`additionalProperties` is false on every record.

Shared required envelope: `kind, signingDomain, schemaVersion, network,
deployment, principal, nonce, validity, spec, policy, profileBinding,
instanceId, genesisHash, requiredClaims, requiredClaimRoot,
observationPolicy, residualAuthority, remainingWork, recoveryRights, locks,
assumptions`.

`requiredClaims` minItems 4 maxItems 16. Schema `allOf` four `contains`
clauses with `minContains=1` and `maxContains=1` for kinds
`ContractInvariant`, `IntentRefinement`, `TransitionValidity`,
`HistoryCompliance`. Four copies of `ContractInvariant` fail schema.
No source option disables a kind. Capability-proof correspondence remains
abstract pending native history (open check, not a missing claim).

ExactPlan extra required: `beforeStateHash, predecessors, executionBody,
executionBodyHash`. If `executionBody.action.name=="genesis"` then
`predecessors` minItems 0 maxItems 0. Else minItems 1 maxItems 8.

OutcomeIntent extra required: `allowedActions, grossDebitCaps, feeCaps,
netGoals, debtCaps, permittedRecipients, permittedCalls, partialFill,
predecessorConstraint`. No execution body. Selected plan is supplied at
fill time and bound in `AcceptanceBody.selectedPlanHash`.

`ExecutionBody` required: `action, exactWrites, exactEffects`. Optional
exchange-only: `dust, rateOrPrice, outPredicate, remainder`.
`exactWrites` maxItems 64. `exactEffects` maxItems 16.
`ActionCall`: `name, actor, arguments` with `FieldBinding` `{name,value}`.

`PartialFill`: `{tag:"FillOrKill"|"Partial", minFill:Amount,
cumulativeCap:Amount}`. `minFill.value <= cumulativeCap.value` is
contextual.

## D. Hash DAG and typed bodies

Framed digest (proposal, host SHA-256, native compatibility unresolved):

```text
SHA256( UTF8(domain) || 0x00 || canonicalJSON(object) )
canonicalJSON = json.dumps(sort_keys=True, separators=(',', ':'),
                           ensure_ascii=False)
```

`sourceHash` is the exception: SHA256 of raw UTF-8, empty domain, no NUL.

Proposed domains (`signing-examples.json` `domainTagsProposed`). Atomic
domains `MORIARTY-SIGN-bounded-atomic/1` and
`MORIARTY-OUTCOME-bounded-atomic/1` are not reused.

| Digest | Domain | Body def | Depends on |
| --- | --- | --- | --- |
| `sourceHash` | (raw) | source UTF-8 | none |
| `profileHash` | `MORIARTY-SUCC-BOUNDS/0` | `ProfileBody` | none |
| `programHash` | `MORIARTY-SUCC-PROGRAM/0` | `ProgramBody` | `sourceHash` |
| `claimRoot` | `MORIARTY-SUCC-CLAIMS/0` | `requiredClaims` array | none |
| `policyHash` | (policy body) | `PolicyBinding` bytes | none |
| `genesisHash` | `MORIARTY-SUCC-GENESIS/0` | `GenesisBody` | profile, program, claims |
| `stateHash` | `MORIARTY-SUCC-STATE/0` | `StateBody` | genesis |
| `observationsHash` | `MORIARTY-SUCC-OBS/0` | `ObservationSet` | none |
| `executionBodyHash` | `MORIARTY-SUCC-EXEC-BODY/0` | `ExecutionBody` | none |
| `exactPlanDigest` / outcome digest | `MORIARTY-SUCC-EXACT-PLAN/0` or `...-OUTCOME-INTENT/0` | the signed document | body hashes it embeds |
| `authorityDigest` | `MORIARTY-SUCC-AUTHORITY/0` | `AuthorityWrapper` | intent digest |
| `consumptionId` | `MORIARTY-SUCC-CONSUMPTION/0` | `ConsumptionIdBody` | none |
| `preparedHash` | `MORIARTY-SUCC-TRACE/0` | `PreparedBody` | state, effects |
| `proofContextHash` | `MORIARTY-SUCC-PROOF-CTX/0` | `ProofContext` | authority, obs, prepared |
| `acceptanceBind` | `MORIARTY-SUCC-ACCEPTANCE/0` | `AcceptanceBody` | intent, plan, obs, prepared, successor |
| successor digest | `MORIARTY-SUCC-SUCCESSOR/0` | `SuccessorRecord` | intent, ledgers |

DAG is acyclic by the depends-on column. Hashing a document requires
structural plus domain validation first. Unknown keys reject before hash.

Closed bodies:

`GenesisBody`: `instanceId, profileHash, programHash, claimRoot, lifetime,
horizon, ordinaryWork, recoveryWork, initialStateHash, network, deployment,
principalBindings, observationBindings`.

`StateBody`: `instanceId, eventOrder, balances, debts, shares, positions,
requests, messages, eventClaims, rewardAccounts, locks, duties, status,
residualCapability, remainingWork, successorRecordHash`.

`ObservationSet`: `{observations:[Observation,...]}`. `Observation`:
`field, value, anchor, freshness, role, authenticity, truthAssumption`.

`AuthorityWrapper` (not residual capability): `tag:"AuthorityWrapper",
intentDigest, signingDomain, principal, nonce, signatureAbsent`. Examples
are unsigned (`signatureAbsent=true`). No signature verification is claimed.

`ProofContext`: `programHash, specVersion, policyHash, authorizationDigest,
selectedPlanHash, predecessorIds, observationsHash, postStateHash,
effectsHash, residualCapabilityHash, remainingWork, claims`.

`AcceptanceBody`: `intentDigest, selectedPlanHash, observationsHash,
effects, liabilities, remainingWork, predecessorIds, successorRecord,
preparedHash`.

`ConsumptionIdBody`: `network, deployment, principal, nonce`. Durable
consumption identity.

`ProgramBody`: `sourceHash, entryActions, coreVersion`.

Every advertised digest in a valid example has a retained preimage in
`preimageRegistry` or is the framed signed document itself. Independent
recompute uses Python hashlib. Native sponge/Poseidon correspondence is
unresolved.

## E. Display (FIELD_META)

Deterministic projection (examples `displayProjectionRule`):

1. Parse canonical UTF-8 to a JSON value. Re-encode. Byte identity must hold.
2. Walk the object. Object keys in increasing UTF-8 byte order. Arrays in
   index order.
3. Emit one entry per leaf. Empty array emits value `[]`. Empty object emits
   `{}`. That is the empty-prohibition display: the signer sees the empty
   `debtCaps`, `permittedCalls`, or `predecessors` list rather than an omitted
   field.
4. Path is dotted with decimal indices (`grossDebitCaps.1.maximum`).
5. Each entry is `{path, label, unit, role, value}`.

FIELD_META is the closed catalog of schema `x-display` objects. Lookup:

- Exact property path suffix (`maximum` under `DebitCap` -> label "Gross
  debit cap", unit `asset-quantum`, role `gross-debit-limit`).
- If unit is `from asset` or `from amount.asset`, inherit the nearest ancestor
  `asset` or `denomination`.
- Price mantissa unit is "`base` per `quote` at scale N".
- Identity fields use `Party`, `AssetId`, `ObligationId`, `InstanceId`.
- Indexed entries are first-class. Omitting `netGoals.1` while showing
  `netGoals.0` is a completeness failure.
- Missing path, extra path, or value unequal to the parsed leaf is a display
  reject (`inv-display-missing-entry`, `inv-display-extra-entry`,
  `inv-display-changed-entry`, `inv-display-mismatch-gross-fee-debt`).

Completeness is on paths and values. Some example rows still copy the last
path segment as label with unit `signed-field`. That is diagnostic fallback.
The catalog above is the contract. A later formatter must emit FIELD_META
labels. Value mismatch already rejects.

## F. Debt, accrual, price, dust

Let `P` be `principal`, `A` be `accrued`. Invariant while `status` is
`Outstanding`: `outstanding = P + A`. Both nonnegative. `outstanding <=
liabilityCap`. Creating or increasing `P` or `A` requires a CoreOp in
`{DebtCreate, DebtAccrue, DebtCapitalize}` and a matching `debtCaps` row
(`debtor, creditor, denomination, maximumOutstanding`). Transfer never
satisfies that.

### F.1 Accrual

`DebtAccrue` emits `AccrualEffect`:
`kind, ordinal, debtId, rate, periodStart, periodEnd, principalBefore,
accruedDelta, outstandingAfter, capApplied`.

`period = periodEnd - periodStart` (UInt64 seconds). Require
`periodEnd > periodStart`. Proposed year length `secondsPerYear = 31536000`
(365 * 86400). ACTUS day-count is SP07 mapping, not this freeze.

Let `r = rate.mantissa / 10^rate.scale` as a signed rational.

If `r < 0` apply `negativeRateDisposition`:

- `Reject`: `ARITH_*` / `OBLIGATION_*`, no write.
- `FloorZero`: `accruedDelta = 0`, `capApplied = false`.
- `CreditAccrued`: decrease `A` then `P` by the floor magnitude, never below
  0, never mint `Amount`. `accruedDelta` is the unsigned decrease.

If `r >= 0`:

```text
raw = floor( principalBefore * rate.mantissa * period
             / (10^rate.scale * secondsPerYear) )
```

using result-width UInt128. Overflow rejects. The admitted first-period
sample (`exact-plan-accrue-nonexchange` argument `qty` 533972602
`USD_micro`) must equal `raw` (one-final-floor). Tentative
`outstanding* = principalBefore + A + raw`. If `outstanding* > liabilityCap`
then `outstandingAfter = liabilityCap`, `capApplied = true`,
`accruedDelta = liabilityCap - principalBefore - A`. Else
`outstandingAfter = outstanding*`, `capApplied = false`,
`accruedDelta = raw`. `P` is unchanged. `A' = outstandingAfter - P`.

### F.2 Capitalize, repay, write-off

`DebtCapitalize`: `amountMovedToPrincipal <= A`.
`P' = P + amountMovedToPrincipal`. `A' = A - amountMovedToPrincipal`.
`outstanding` unchanged. Status may become `Capitalized` only when `A'=0`
and a named policy says so. Default: remain `Outstanding`.

`DebtRepay` emits `DueSettledEffect`:
`kind, ordinal, debtId, nominalDischarged, settlementAsset,
settlementAmount, conversionMantissa, conversionScale`.

Settlement conversion:

```text
settlementAmount = round( nominalDischarged * conversionMantissa
                          / 10^conversionScale , conversionRounding )
```

`none` requires exact divisibility else reject. `settlementAsset` must equal
`Debt.settlementAsset`. `settlementAmount` is an `Amount` quantum of that
asset. It is not a denomination unit.

Allocate `nominalDischarged` N against `(P,A)`:

- `AccrualFirst`: `dA = min(N,A)`, `dP = N - dA`.
- `PrincipalFirst`: `dP = min(N,P)`, `dA = N - dP`.
- `ProRata`: if `P+A = 0` reject; else `dP = floor(N*P/(P+A))`,
  `dA = N - dP` (last unit to accrued).

Require `dP <= P` and `dA <= A`. `P' = P - dP`. `A' = A - dA`.
`outstanding' = P' + A'`. If both zero, `status = Settled`. Refunds of
settlement assets do not restore `cumulativeGross` or `cumulativeCreated`.

`DebtWriteOff` emits `WriteOffEffect`:
`kind, ordinal, debtId, amount, authority, residualOutstanding`.
`authority` must be `Debt.controller`. `amount <= outstanding`.
`residualOutstanding = outstanding - amount`. This is authorized residual
reduction, not an accounting-loss hide and not a Transfer. Status
`WrittenOff` when residual is 0.

`DebtCreate` emits `DueCreatedEffect` carrying the full `Debt` record.
`cumulativeCreated' = cumulativeCreated + principal`. Must not exceed
`originalDebtCap`.

### F.3 Exchange price and dust

Exchange only. `direction = basePerQuote`. Quote units in, base units out:

```text
out_raw = in_quote * mantissa / 10^scale
out = floor(out_raw)                 # admitted swap: one-final-floor
remainder.numerator = (in_quote * mantissa) mod 10^scale
remainder.denominator = 10^scale
remainder.unit = base
remainder.location = SourceReserve   # unless NamedParty is authorized
```

If `remainder.numerator / remainder.denominator * in_scale < dust.value`
the remainder stays in the source reserve. It is recorded as `Remainder`,
not as a Transfer of quote dust. Example: in 10000 AssetA, mantissa 19743,
scale 4, out 19743 AssetB, numerator 0, dust AssetB 1.

`outPredicate` Min: `out >= amount`. Exact: `out == amount`. Exact 19743
with delivered 19744 rejects (`inv-exact-output-overdelivery`).

## G. Authority, history, successor

Every protected dimension of the originating intent remains bound:

gross debit caps, fee caps, net goals, debt caps, permitted recipients,
permitted calls, validity, partial-fill rule, work, recovery, assumptions,
allowed CoreOps, allowed assets, original intent digest.

`SuccessorRecord` required:
`tag (GenesisInitial|ActiveSuccessor), originalIntentDigest, consumptionId,
partialRule, originalValidity, originalRecipients, originalCalls,
originalAssumptionsHash, originalWork, actorLedgers, debtLedgers,
residualCapability, remainingWork, predecessorMode`.

`ActorLedger` (per actor, asset):
`originalGrossCap, originalFeeCap, originalNetGoal, cumulativeGross,
cumulativeFees, cumulativeNet, remainingGross, remainingFee`.

```text
remainingGross = originalGrossCap - cumulativeGross
remainingFee   = originalFeeCap   - cumulativeFees
cumulativeNet  = inflow_to_actor  - cumulativeFees     # same asset
```

Refunds and reverse Transfers do not decrease `cumulativeGross`. Fees count
in `cumulativeFees` and therefore against `cumulativeNet` and `netGoals`.
`remainingGross` never increases. Child residual caps are componentwise
`<=` parent remaining, not parent original.

`DebtLedger`:
`originalDebtCap, cumulativeCreated, cumulativeRepaid, cumulativeWrittenOff,
outstanding`.
`outstanding = cumulativeCreated - cumulativeRepaid - cumulativeWrittenOff`
as UInt128 (no wrap). Write-off does not create tokens.

Net-goal timing:

- Each prefix (Partial fill): `cumulativeNet + remainingGross_of_counterparties`
  must still be able to reach `originalNetGoal`. A prefix may sit below the
  goal if later authorized fills can close it.
- Completion (FillOrKill success, or Partial when residual continuation is
  `none` or remaining fill is zero): `cumulativeNet >= originalNetGoal`.
- Cancel: unsatisfied `originalNetGoal` becomes a residual `Duty`. Cancel
  does not erase it. `inv-net-after-fees-shortfall` selects a plan with Fee
  30 AssetB so trader net 19713 < goal 19743.

`predecessorMode`:

- `Pinned`: ExactPlan `predecessors` is an exact list. Each
  `consumptionId` must be current and unconsumed. `stateHash` must equal
  the ledger state. Duplicate `consumptionId` is schema-legal and
  `HistoryCompliance` illegal.
- `AdmittedContext`: OutcomeIntent `predecessorConstraint.tag="AdmittedContext"`
  with `instanceId, requiredPolicyHash, minEventOrder, requireUnconsumed`.
  Any matching unconsumed predecessor of that instance with
  `eventOrder >= minEventOrder` and `policyHash` may be selected. The
  selected ExactPlan then pins the concrete list.
- `GenesisNone`: genesis ExactPlan, empty predecessors, still four claims.

No work refresh. No authority refresh. Migrate conserves residual caps,
ledgers, duties, and remaining work. It does not mint budget.
`inv-replay-migration` recomputes `executionBodyHash` so hash stages pass
and isolates `HISTORY_*` on a consumed predecessor.

## H. Work and composition

`Work`: `ordinaryRemaining, recoveryRemaining, lifetimeOrdinary,
lifetimeRecovery` (UInt128). Lifetimes freeze at genesis. They never
increase. Ordinary exhaustion never erases debt.

Work unit: one ordinary (or recovery) unit per charged CoreOp. Proposed
charges (majority before freeze):

| CoreOp | ordinary | recovery if invoked via RecoveryRight |
| --- | --- | --- |
| every CoreOp except as below | 1 | 1 (recovery ops only) |
| `Exchange` | 1 | n/a |
| `Genesis` | 1 | n/a |
| `Migrate` | 1 | 1 if recovery-migrating |

Examples: swap successor `ordinaryRemaining` 7 from 8. Accrue successor 7
from 8. Charge is applied when forming the successor, not by rewriting the
signed pre-charge `remainingWork` on the ExactPlan. Signed `remainingWork`
is the inherited pre-charge residual. Successor `remainingWork` is
post-charge.

`RecoveryRight`: `{controller, allowedEffectSet, alias}`.
`alias = {tag:"AliasReserve", which:"recovery"}`. This aliases
`remainingWork.recoveryRemaining`. It does not embed a second `Work` and
does not partition unless a later `PartitionReserve` tag is admitted
(currently not in schema; unknown tag rejects). Two children that both
alias the same recovery reserve and copy the parent's remaining numbers
clone work (`inv-cloned-residual-work`: two children 8+2 each versus
parent 8+2).

Conservation:

```text
split: sum(child.ordinaryRemaining) = parent.ordinaryRemaining - charge
       sum(child.recoveryRemaining) = parent.recoveryRemaining - rec_charge
       children alias XOR partition; alias => at most one child holds recovery
join:  sum(children remaining) <= parent remaining; no mint
cancel: remaining after charge returns to the residual successor; duties stay
migrate: remaining' = remaining - charge(Migrate); caps do not increase
```

Admission maxima from `ProfileBody`: `maxNodes` 4096, `maxSidecarBytes`
16384, `maxTotalVerificationWork` 65536, `predecessorFanIn` 8. Checks run
before allocation or expensive proof.

`CompositionPolicy`: `operator` in `seq|par|interleave|atomic|message`,
`unknownRejects=true` const.

- `seq`: ordered. Prefix observations concatenate. Write-set of i is
  visible to i+1 after commit of i.
- `par`: `parFrame = disjoint_keys_concat_obs`. Write paths, identity
  keys, and `(actor,asset)` / `(debtor,creditor,denomination)` cap keys
  of branches are pairwise disjoint. Observations concatenate in
  increasing key order. Combined prefix is the concatenation. Financial
  framing is the disjoint union of actor/debt ledgers.
- `interleave`: `interleaveConflict = write_path_or_identity_or_cap_key`.
  Two steps conflict if they write the same state path, the same identity
  (`debtId`, `requestId`, `positionId`, `messageId`, `claimId`,
  `accountId`), or the same cap key. Conflict rejects. Ordering is the
  admitted interleaving with that filter.
- `atomic`: one effect set, all-or-nothing, one consumption.
- `message`: request/message lifecycle, not proof-join.

Unknown operator rejects. Proof split/join is not a composition operator.
Boolean `and` is not composition. Child `allowedEffectSet` is a subset of
parent.

## I. Validation stages and examples

Stages (`x-validationStages` and examples lists):

1. Structural (schema): Draft 2020-12 types, required keys,
   `additionalProperties` false, true-end patterns, tagged unions,
   required-claims contains-exactly-once, genesis predecessor emptiness,
   exchange-only dust/price/predicate/remainder.
2. Domain (pre-hash): every UInt64/UInt128/SInt128/Scale inclusive range,
   recovery alias (no nested Work), Quantity vector length and unique
   symbols, canonical re-encode byte identity.
3. Hash: recompute every advertised digest from retained preimages.
4. Contextual (specified-only until an evaluator exists): authorization
   network/deployment/revocation/validity vs `obs.now`, net-after-fees
   including actual Fee effects, clone residual work, cancellation vs
   winning fill currentness, migration replay, display completeness,
   cap sums versus movements, unique consumption.

Do not describe a contextual check as executed. Schema and host-hash
checks in FOREMAN_REPORT were run this pass.

Positive documents (four):

| id | kind | notes |
| --- | --- | --- |
| `exact-plan-swap-min-receive` | ExactPlan | Exchange, Min 19743 AssetB, one predecessor |
| `outcome-intent-swap-and-loan-caps` | OutcomeIntent | actor-indexed gross/fee/net/debt, Admitted/Pinned constraint |
| `exact-plan-genesis-no-predecessor` | ExactPlan | empty predecessors, `Genesis` effect, four claims |
| `exact-plan-accrue-nonexchange` | ExactPlan | `DebtAccrue` -> `Accrual`, no dust/price |

Reconstruction of invalid fixtures (`reconstructInvalid`): start from
`valid[id=base]` parsed `canonicalUtf8`. Apply `sets` in order. If
`recomputeExecutionBodyHash`, set `executionBodyHash` to the framed
execution body. If `recomputeDependentHashes`, recompute listed names
from retained preimages. Isolated contextual fixtures:

- `inv-net-after-fees-shortfall`: context `selectedPlan` emits Fee 30
  AssetB plus the two Transfers. Trader net 19713 < 19743. Schema on the
  OutcomeIntent (fee cap raised to 30) passes. Evaluation rejects.
- `inv-cloned-residual-work`: two children each inherit 8+2. Sum exceeds
  parent 8+2. Both alias recovery.
- `inv-cancellation-race`: `winningFill` consumed
  `consumptionId`/`NonceIntent01`. Stale `RequestCancel` reuses that id.
  Ledger currentness rejects.
- `inv-replay-migration`: execution body replaced with `Migrate`,
  `executionBodyHash` recomputed `3ac0552c...`. Consumed predecessor
  `eb9cdedf...` remains. History rejects.

`executionStatus` on every invalid row is `specified-only`.

## J. SP02 / SP03 interfaces and freeze gates

SP02: EBNF for every DA01-DA24 construct, elaboration to CoreOp, successor
`bounds.json` as a new file. Atomic `bounds.json` bytes stay identical.
SP03: K rules for B.1 constructors, C.3/F equations, composition H.
SP09: native hash correspondence.
SP01.7/SP09: native statement for the four claims.

Still required before freeze: SP01.2 challenge-map reconciliation;
user-selected majority on `majorityRequired` decisions; native hash and
domain correspondence; formative usability evaluation; formal proofs of
named claims. Tests are not proofs. Merge status is not product
acceptance. This design remains proposed.
