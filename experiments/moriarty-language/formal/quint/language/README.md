# Moriarty language Quint QA

## Unsigned ledger preparation custody QA

`ledger_call_custody.qnt` is a plain finite, single-actor model of semantic
Transfer/Fee groups, exact native output multiplicity, separate payer change,
all mapped balances, retained inputs and present-zero duties. Its nineteen
deterministic tests cover payment70, fee-first43, repeated recipients, self
payments, equal outputs, zero/empty offers, above-u64 amounts and hostile custody
mutations. A 100-sample, 10-step TypeScript run at seed1011 found no
`preparation_invariants` violation; payment and collection were witnessed in
58 and72 traces respectively. This is bounded QA, not exhaustive verification.

Numeric owner/token identities abstract distinct native mappings and native
ordering. Request-derived nextValue/events/frozen and duty updates are premises;
the model does not evaluate source expressions or execute VM instructions.
Native tagged bytes, FAB alignment, hashing, VM correctness, supplied state
trust, signatures, proofs, funding and source-to-VM correspondence are external
premises. Rust and actual process tests separately execute the pinned VM and
native offer/transcript APIs. Existing signed models do not establish custody
for this unsigned preparation profile.

```sh
quint typecheck ledger_call_custody_test.qnt
quint test ledger_call_custody_test.qnt --main ledger_call_custody_test --backend typescript
quint run ledger_call_custody.qnt --main ledger_call_custody --backend typescript --invariant preparation_invariants --witnesses paymentWitness collectionWitness --seed 1011 --max-samples 100 --max-steps 10 --verbosity 1
```

The TypeScript evaluator supports the actual UInt128/i128 constants; the local
Rust evaluator rejected the wider-than-i64 literals. No Quint verify, Apalache,
native proving, key generation, SRS, or MockProver job was used for this slice.

Experiment observation, 2026-10-10: Quint 0.32.0 typechecks all three models;
59 deterministic tests pass; 10,000 sampled traces per model at 12 steps found
no violation of the named `safety` predicates. This is bounded sampled QA,
not exhaustive verification, cryptographic soundness, ledger acceptance or
completion of M1–M17. Raw commands, output and model digests are in
[results/qa.log](results/qa.log).

The approved scope is Moriarty language/library support. These models use
generic target identities and shared authenticated ledger state. They do not
implement Fenrir, external consensus, remote messaging or a fraud game.

## State and assumptions

`acceptance.qnt` has one cohesive `State` record, with a `Ledger` containing
balances and gross/fee/net/liability counters for two distinct asset identities,
nonce/head consumption, and separate user and sponsor DUST accounts. Immutable
`intent` binds source/program/semantic/relation/key/ledger/network/Anchor/address/
deployed-circuit-set/target/action identities and asset network/issuer/reference/
scale. The fixed signer and domain mean the nonce set represents replay keys
for that one `(network, Anchor, signer)` scope. Testing multiple owners or target
instances requires a separately approved extension.

`signatureVerified`, `nativeVerified`, `resourceVerified`, `sponsorVerified` and
oracle receipt booleans are symbolic premises. A true value assumes an authentic
verifier result bound to the whole candidate, not a caller-supplied assertion.
The model cannot establish the receipt's authentication or cryptographic truth.
Resource byte/work values are small model units (64 and 5), not measurements of
real protocol limits. Scalar values are bounded to 100, abstracting checked u128
rather than testing every u128 boundary.

All transitions assign the whole state record. Guards (`can*`) and updates
(`apply*`/`record*`) are separate pure functions. Tests live in separate files.
Observable acceptance/batch refusals have their own guarded transitions and
preserve financial state. Lifecycle terminal states stop when no action is
enabled; a blanket stuttering transition does not create apparent progress.

`lifecycle.qnt` models two exclusive reservations against one allowance. Timeout
marks exposure unknown while retaining funds and duties. A verified success may
arrive late. A verified unused receipt must be authentic, request-bound,
irreversible and incompatible with success for that same request; this is an
external premise, not a theorem of this model. Recovery releases funds only
following that receipt. Partial success preserves the remaining duty. It does
not model cross-chain finality, compensating obligations, cancellation protocols,
recovery-work budgets or eventual delivery.

`batch.qnt` models two same-domain orders at one price, conservative fixed
seller/buyer bounds, permitted solver fees, exact token/cash identities, and an
authorized oracle observation within a fixed age/price band. It does not model
order discovery, liquidation, changing committees, oracle construction, or
cross-domain atomicity. Seller net is cash credit less solver fee.

## Named predicates and expected outcomes

| Requirements | Model predicates | Positive/negative controls |
|---|---|---|
| M1/M2/M7/M8/M9 | `bindingSafety`, `canAccept` | Source/program/semantics/key/relation/target/domain/Anchor/address/circuit/action mismatch; every asset identity dimension; false verification premises |
| M3/M6/M10 | `completeEffectConservation`, `refusalAtomic`, `nonceHeadUnique` | Valid transfer; stale predecessor; duplicate nonce; wrong successor; missing effects; conservation/balance refusal |
| M4/M11/M16 | `sponsorCap`, `reservationExclusive`, `heldMatchesReservations` | Sponsored DUST; grant scope/nonce/cap refusal; exclusive concurrent reservations; no user fee fallback |
| M9/M13/M14 | `cumulativeAuthority`, `persistentDuties`, `noRefundPlusSuccess`, `timeoutRetainsExposure` | Refund retains gross debit; fee reduces net; liabilities persist; timeout cannot refund; partial/late success; unused recovery |
| M5 | `canAccept` resource/circuit clauses | Missing replay circuit, encoded byte/work refusal; symbolic resource-verification refusal |
| M14/M15 | `uniformPrice`, `conservation`, `bounds`, `oracleSafety` | Uniform-price fill; unequal price, solver/quantity/limit, stale/future/unauthorized oracle and cross-domain refusal |

Gross counts all owner debit, including fees, and a refund never subtracts from
gross. An acceptance effect's debit splits into refund, recipient credit and
application fee. Net goal accounting is recipient credit minus application fee,
matching `check_effects` for this fixed beneficiary scope; net is distinct from
the actual recipient balance. Liability counters accumulate across accepted
steps, and ordinary transfers never erase them. The lifecycle partial-success
witness preserves a duty of 7 after delivering 3 of 10.

## Concrete correspondence and gaps

Repository observations refer to the current candidate; they are not formal
refinement proofs. The models describe acceptance expectations beyond local
simulation. Their witness traces have not been automatically replayed through
Rust or an external ledger.

| Model field/action | Actual Rust field/function | Correspondence limit |
|---|---|---|
| Binding source/program/semantics/action | `OwnerIntent.source_hash/program_hash/semantics/action`, `prepare_intent`, `verify_intent` in [intent.rs](../../../../../crates/mori-core/src/intent.rs) | Canonical intent/signature checks exist; relation/key/deployed circuit set and authenticated target admission remain outside this verifier |
| Binding network/Anchor/address/target | `Authorization.network/anchor/program_address/target_hash` | Bound into signed bytes; `verify_intent` does not authenticate current network/Anchor state |
| Exact asset identities | `AssetId`, `Program.assets`, `Balance.asset`, `Effect` asset fields in [ir.rs](../../../../../crates/mori-core/src/ir.rs) | Actual network/issuer/reference/scale identity; no real ledger asset readback |
| Candidate financial effects/poststate | `Execution.effects/post/steps`, `Snapshot.balances/liabilities`, `simulate`, `replay` in [runtime.rs](../../../../../crates/mori-core/src/runtime.rs) | Clone-and-commit local atomicity and deterministic replay; no cryptographic receipt or ledger finality |
| Gross/fee/net/liability authority | `AssetLimit.gross_cap/fee_cap/net_floor/liability_cap`, `check_effects` | Actual per-execution complete-effect checks; persistent cross-execution cumulative allowance storage remains a gap |
| Nonce/head/validity | `Authorization.nonce/predecessor/not_before/not_after` | Signed and interval checked; nonce/head consumption against authenticated mutable state remains a gap |
| DUST reservation/timeout/late settlement | `FeeCapability.reserve/available/mark_unknown/settle` in [policy.rs](../../../../../crates/mori-core/src/policy.rs) | Local exclusive accounting; caller must authenticate capability and settlement observations; actual DUST charging is external |
| Proved-unused recovery from Unknown | `FeeCapability.release_unused` | Intentionally unsupported: release only covers never-submitted Reserved. No authenticated no-execution receipt API exists for Unknown |
| Cost streams | `Meter`, `Cost`, `CostPlane` | Runtime independently accounts logical/DUST/service/external counters. Model separates application fees from DUST but does not encode all four meters |
| Batch, oracle, persistent application duties | Source-authored `Requires/Ensures/Next/Transfer/Fee/Liability/Discharge` | Language can express bounded predicates/effects; no dedicated runtime batch/oracle/lifecycle protocol is claimed |
| Signature premise | `verify_intent` with BIP340 or ECDSA verification | Actual native signature checks; Quint excludes crypto; strict JubJub/native/in-circuit correspondence remains open |
| Resource/native proof premise | Compiler/target artifacts outside this model | No proof generation, authenticated measurement, external deployment, or qualified immutable circuit-set admission established here |

Local native controls in [intents.rs](../../../../../crates/mori-core/tests/intents.rs)
exercise a genuine signature, changed Anchor, and a forged network binding.
[authority.rs](../../../../../crates/mori-core/tests/authority.rs) checks refund/
fee authority, isolated cost planes and timeout retention. Their execution is
recorded separately in `results/native-controls.log`. They are independent
native tests, not a model-derived trace refinement check.

## Reproduction

From this directory, typecheck each `*_test.qnt` file and run the tests with
`--match 'Test$'`. A broad `--match '.*'` also selects imported helper definitions
in Quint 0.32.0 and is unsuitable here.

```bash
quint typecheck acceptance_test.qnt
quint typecheck lifecycle_test.qnt
quint typecheck batch_test.qnt
quint test acceptance_test.qnt --main acceptance_test --match 'Test$' --seed 1010
quint test lifecycle_test.qnt --main lifecycle_test --match 'Test$' --seed 1010
quint test batch_test.qnt --main batch_test --match 'Test$' --seed 1010
quint run acceptance.qnt --main acceptance --invariant safety --witnesses acceptedWitness rejectedWitness sponsoredWitness competingWitness --seed 1010 --max-samples 10000 --max-steps 12 --verbosity 1
quint run lifecycle.qnt --main lifecycle --invariant safety --witnesses reservedWitness unknownWitness lateSuccessWitness recoveredWitness residualDutyWitness --seed 1010 --max-samples 10000 --max-steps 12 --verbosity 1
quint run batch.qnt --main batch --invariant safety --witnesses clearedWitness rejectedWitness partialFillWitness --seed 1010 --max-samples 10000 --max-steps 12 --verbosity 1
```

The initial one-transition version typechecked, passed its transfer test, and
reached its acceptance witness before the model was extended. Current controls
are deterministic; identity tests enumerate all alternatives rather than rely
on random negative-case sampling. Simulation uses Quint's default init/step.
No `quint verify` command was run.

## Native local proof binding QA

`native_transition.qnt` and `native_transition_test.qnt` preserve the historical
revision 1 public local proof verifier model separately from ledger acceptance. Five deterministic
tests cover every claim field, repaired metadata with a reused proof, derived
relation identity, native refusal, pinned parameters, canonical proof framing,
residual liability refusal and explicit k caps. Seeded simulation checks
receipt binding with positive verification and refusal witnesses.

`Claim` corresponds to `proving.rs::Claim`; expected claim reconstruction and
metadata comparison correspond to `prepare` and `verify`. `proofClaim` and
`nativeValid` are symbolic cryptographic premises, exercised separately by
genuine Rust/CLI proof controls. `pinnedParameters` corresponds to the offline
snapshot SHA-256 check before upstream decoding. The model assumes collision
resistance and source-to-constraint correspondence; it does not establish
them. Complete execution includes host-reconstructed untouched framing.
There is no ledger, signature, nonce consumption or history acceptance in
this local verifier model.

```sh
quint typecheck native_transition_test.qnt
quint test native_transition_test.qnt --main native_transition_test --match 'Test$' --seed 1010
quint run native_transition.qnt --main native_transition --invariant safety --witnesses verifiedWitness refusedWitness --seed 1010 --max-samples 10000 --max-steps 12 --verbosity 1
```

## Current revision 2 liability stage QA

`liability_transition.qnt` is a separate current local-stage model. It does not
replace the earlier revision 1 `native_transition` model or its recorded results.
The approved requirements and independent outcomes select one shared local record
and one atomic stage; there are no messages, time, concurrent owners or ledger
phases. Immutable source names are represented by integer labels, and the full
asset/party tuple by exact symbolic strings and scale. These labels abstract source
names; serialization, lexical sorting and hashes are outside this model.

The state groups the complete immutable registry and presence/amount map with
ordered published duty effects, prior-frame evidence and acceptance/refusal
counters. Source admission allows up to sixteen unique names with identical repeats;
scenario rows must have exactly one canonical slot for every name. The default
simulation has two slots and samples deltas zero through four; deterministic tests
also use zero and sixteen slots. Amount arithmetic uses the exact mathematical
UInt128 maximum, not a smaller ceiling. Quint 0.32.0's TypeScript evaluator is
required because its Rust evaluator rejects integer literals beyond i64.

| Requirement | Model guard/property | Actual consumer correspondence and limit |
|---|---|---|
| Immutable exact source registry, cap16 | `sourceAdmitted`, `contextAdmitted`, `exactClosure` | `liabilities::registry`, compiler descriptors/resources; source closure does not authenticate actual duties |
| Canonical absence and persistent present-zero | `canonicalFrame`, `persistence` | Bool/UInt128 opening/closing constraints and full `LiabilityDescriptor` positions; no deletion or implied payment |
| Ordered checked accrual/discharge | `canOperation`, `canStage`, `orderedEffects` | Compiler sequential wires and `proving::public_values` reconstruction; honest preimage checks are separate Rust evidence |
| Complete slots and unchanged unrelated duties | `untouched`, `exactClosure` | Every action's actual-width `liability_frame`; no hidden-name omission is admitted |
| Atomic observable refusal | `refuse`, `refusalAtomic` | Local clone-and-commit simulation and preparation refusal; no ledger finality claim |
| Exact effect-bearing source steps | `descriptorSteps`, `orderedEffects` | Task 2 source descriptor sequence check; swapped identical zero effects refuse by source step identity |

Pure guards and updates are separate; thin `accept`/`refuse` actions publish the
whole state only after every ordered operation is checked. Intermediate scratch
updates never publish on refusal, which increments observable refusal accounting
and preserves duty state and prior published effects. No native-valid or
cryptographic truth flag is used. An empty duty-operation list represents an
action without duty effects, including transfers; this model does not check that
transfer's balances, fees, state or event values.

Experiment observation, 2026-10-11: 25 deterministic tests pass. They cover exact
zero lifecycle, partial/full discharge, untouched present-zero, absent discharge0,
canonical/malformed frames, duplicate/hidden/missing names, all six tuple identity
dimensions, source/operation substitution, cap16/17, ordered identical zero effects,
actual UInt128 maximum and intermediate overflow/underflow. The seeded run explored
10,000 traces at 12 steps with no `safety` violation. Witnesses require actual
accepted/refused actions: accrual 9,927; discharge 9,569; partial discharge 8,890;
full discharge 1,565; absent-zero creation 1,778; refusal 9,985 traces. These are
sampled reachability counts, not exhaustive proofs. The four historical models'
64 deterministic tests and typechecks were rerun unchanged.

```sh
quint typecheck liability_transition_test.qnt
quint test liability_transition_test.qnt --main liability_transition_test --backend typescript --match 'Test$' --seed 1010
quint run liability_transition.qnt --main liability_transition --backend typescript --invariant safety --witnesses accruedWitness dischargedWitness partialWitness fullWitness zeroCreationWitness refusedWitness --seed 1010 --max-samples 10000 --max-steps 12 --verbosity 1
```

Logs, failures and exact model digests are in the runtime evidence named by
[the Task 3 report](../../../../../.superpowers/sdd/native-liability-task-3-report.md).
Simulation uses default `init`/`step`. No `quint verify` or Apalache was used.
Source-to-constraint correspondence, public input publication, hash collision
resistance, native cryptography, signed authority, authenticated predecessor/full
real duty completeness, ledger phases/unique consumption/history, recovery and
private recursion remain external premises or open work. Historical artifact/proof2
revisions and scoped circuit measurements are documented in
[NATIVE-CORE](../../../../../docs/NATIVE-CORE.md); neither this model nor ordinary
checks establish native proof2 roundtrip or any full roadmap exit.

## Revision 3 integrated signed stage QA

`signed_transition.qnt` models the adopted native-owner contract as one cohesive
shared local `State`: immutable source AssetIds and duty identities, complete
balance and presence/amount maps, independently supplied policy/authority,
ordered financial effects, previous published state and an identified refusal.
There are no messages or distributed actors. Pure admission, arithmetic, cap,
guard, update and property functions feed separate thin `accept` and `refuse`
actions. Refusal preserves the previous ledger/effects and records its reason.
A trace ends after 12 attempts; no blanket stutter hides a refusal or deadlock.

Assets compare all network/issuer/reference/scale fields. Reference/name integers
are opaque symbolic labels; maps abstract the admitted canonical source registry
and unique exact asset-limit membership. This model does not implement JSON,
lexical ordering, source-expression evaluation, hashes or signature arithmetic.
An authentic external `signatureVerified` premise is bound by exact equality of
`signedStatement` and the complete reconstructed `Statement` (source, full
opening frame, operations and all authority/domain/cap fields). It is never a
claim that a caller boolean supplies verification authority. The current logical
`at` is independently checked and deliberately excluded from the signed message.
Account/key association, source-to-circuit correspondence, authenticated real
duty completeness, predecessor history, ledger time/acceptance and cryptographic
soundness remain external obligations. This model does not qualify M8.

All financial values and every gross/fee/credit/debit/net/closing-duty accumulator
use the actual UInt128 maximum 340282366920938463463374607431768211455. Simulation
uses Quint 0.32.0's TypeScript evaluator because its Rust evaluator cannot represent
this bound. Gross retains owner outgoing transfers and fees; beneficiary net
subtracts transfer debits and owner fees once, including the self-beneficiary
case. Complete present, absent and present-zero duty membership remains distinct;
partial/full discharge retains presence and conveys no payment or consent.

Experiment observation, 2026-10-11: **44 deterministic tests pass**. They reproduce
independent payment alice 28 / bob 80 / treasury 2 / paid 70 / duties 30, 5, present-zero and exact
Transfer70/Fee2/Liability50→30/State0→70/Emit70 order. Other controls cover each
fresh cap refusal, state-only/zero finance, selftransfer, self-beneficiary fee
accounting, partial/full/zero/absent duties, full candidate signature mismatch,
every domain dimension, policy/key/nonce/predecessor/time mismatch, source-wide
hidden payer/debtor, exact asset mismatch,16/17 admission, atomic intermediate
failure, exact UInt128 maximum and individual accumulator overflow.

The final seeded run sampled 10,000 traces at 12 steps with no `safety` violation.
Actual witness counts: accepted payment 4,339; state-only 6,178; selftransfer 6,123;
partial discharge 4,319; full discharge 6,242; present-zero accrual 6,046; refusal 10,000.
Each witness requires an actual accepted/refused action. These are sampled
reachability observations, not exhaustive verification.

| Requirement | Pure guard/property | Runtime correspondence and limitation |
|---|---|---|
| Source-wide exact owner/assets/duties | `admitted`, `complete` | Signed compiler admission and full frames; symbolic source labels, no parser proof |
| Separate policy/domain/key/interval | `reason` policy/domain/authority/time clauses | Trusted Target/ExpectedAuthority/at reconstruction; no ledger identity/time authentication |
| Whole candidate signature binding | `signedStatement == statement` | Strict reconstructed MOI1/FENS reference path; authentic verifier result is an external premise |
| Checked sums and each cap | `totals`, `checked`, `authorityCaps` | Actual effect/closing-duty compiler wires and host cap checks; no compiler correspondence proof |
| Atomic effects and refusal | `orderedEffects`, `conservation`, `persistence`, `refusalAtomic` | Ordered reference replay and local clone-and-commit; no ledger settlement or nonce consumption |

```sh
quint typecheck signed_transition_test.qnt
quint test signed_transition_test.qnt --main signed_transition_test --backend typescript --match 'Test$' --seed 1011
quint run signed_transition.qnt --main signed_transition --backend typescript --invariant safety --witnesses acceptedPaymentWitness stateOnlyWitness selfTransferWitness partialWitness fullWitness presentZeroWitness refusalWitness --seed 1011 --max-samples 10000 --max-steps 12 --verbosity 1
```

The [signed Task3 report](../../../../../.superpowers/sdd/native-owner-task-3-report.md)
names exact model/source/log digests, current ordinary checks, integrated shape
measurements and the retained max-shape report-size refusal. Those Task3 checks
ran no Quint verify or Apalache, real SRS, key generation, proving, native
verification, MockProver or ignored tests. Historical beta native signature adapters remain a separate
compatibility scope. The signed compiler/CLI is implemented. The separately
retained [October11 selected payment](../../../../../deliverables/native-owner-intent-2026-10-11/RESULT.md)
produced and independently verified one genuine local integrated signed native
case; broader native/U1 and ledger qualification, full U0–U7 and M1–M17 remain
open. The [target-bound reference replay slice](../../../../../docs/evidence/2026-10-11-bound-replay.md)
adds an actual CLI consumer without changing this model or certifying its
cryptographic premises. Update this
model against the approved semantic contract when that contract changes, and
rerun the matching Rust/CLI controls before extending its correspondence claims.

## Native call relation projection QA

`ledger_call_relation.qnt` and `_test.qnt` model one local checker, with atomic
accepted/refused inspection. Source-derived zero/first/repeated claims, optional
duty old reads, exclusive UInt event branches and selected-zero narrow ranges
feed per-Op activity. The checker requires exact raw widths and complete skip
vectors, then preserves field count while merging adjacent Noops. Handcrafted
malformed vectors cover missing/extra/reordered Ops, wrong widths, zero/wrong
inactive skips and missing/extra activity. Bool0/1 vs2, UInt128 aggregate overflow
and independent single/two-event Noop/PI counts have deterministic controls.

Source map: arithmetic/event guards correspond to
`crates/mori-ledger/src/relation/{arithmetic,event}.rs`; optional duty/claim guards
to `compiler.rs`; full consumption/Noop merge to `process.rs` and `prepare.rs`.
Native widths are premises: header/event literals use pinned SDK widths, while
claim/duty records use abstract width1. Duty amount models only a bounded,
selected optional opening; it does not model accrual/discharge arithmetic or
duty history. All17 UInt branches are assumed native-emittable for this short-name
fixture. FAB bytes, bounded node-list event admission, cryptography, funding,
context authentication, deployed keys, global balances and history are excluded.

Observed local QA: typecheck passed,15 deterministic tests passed, and100 traces
at10 steps, seed1012, TypeScript BigInt had no projection invariant violation.
Witnesses: positive claims65, zero claims74, present duties62, absent duties63,
long events20, actual refusals100. These are sampled observations, not exhaustive
verification or a native proof. Update the model and matching Rust controls when
these source guards or the processing protocol change.

```sh
quint typecheck ledger_call_relation_test.qnt
quint test ledger_call_relation_test.qnt --main ledger_call_relation_test --backend typescript
quint run ledger_call_relation.qnt --main ledger_call_relation --backend typescript --invariant relation_projection_invariants --witnesses positiveClaimWitness zeroClaimWitness presentDutyWitness absentDutyWitness longEventWitness refusedWitness --seed 1012 --max-samples 100 --max-steps 10 --verbosity 1
```

## Installed local call consumer QA

`installed_call_consumer.qnt` and `_test.qnt` model one local caller with
separate keygen, cold proving and cold public verification purposes. Metadata
preflight precedes external degree discovery and source derivation; exact opaque
key comparison precedes complete installed-map admission. Every source action,
including unused freeze/reset, must have the derived current VK and exact IR.
Keygen admits the complete keyless seed before native work without evaluating a
selected request. Cold command preflight parses/reads inputs and checks source
metadata; full actual predecessor/frame/map admission occurs after rederivation
and opaque export comparison. Malformed cold state can therefore incur native
discovery/derivation cost before full frame refusal.
The snapshot replaces only operations and recomputes acyclic commitments.
Cold public import ignores optional PK leaves. Preparation retains the actual
selected stored operation; cold proving expands its raw SDK owner once.
Full final intent/state/effects/events/gas/PI comparison precedes consumption of
an external ProofMarker/Real verification result. Verification reports its
local result without publishing files. Keygen/proving publication writes the
completion marker last; collisions and partial writes cannot complete.

The 51 deterministic `Test$` cases include six independent complete literal
statements: Metered A1/A2 have eight ordered effects and balances84/19/3 and
92/11/3; Toggle B1/B2 have four ordered effects and count32/256; freeze/reset
exercise the other stored operations. Tests also distinguish selected-only and
candidate-hash-only checks from complete source-derived policy. These literals
are separate from the model's expected-statement functions. Asset and typed
value identities are compared fully; no literal is a native cryptographic result.

Observed QA: Quint0.32.0 typecheck and all51 tests passed. The TypeScript run
sampled100 traces at10 steps with requested seed1012 and found no `safety`
counterexample. Completion was reached in47 traces, refusal in53, keygen in17,
cold proving in11 and PK-free cold verification in19. Lazy-init, unused-action,
purpose and weak-key refusals were reached in6,6,14 and6 traces. Three negated
reachability controls deliberately violated their predicates and yielded concrete
accepted/refused/PK-free verifier ITF traces. Initial syntax/test-framing failures
and an initial run with zero refusal coverage remain with all logs and traces in
`.moriarty-dev/runtime/installed-call-20261012/quint-qa/`.

| Model transition | Concrete source contract |
|---|---|
| Request-free preflight | `local_deployment::admit_local_seed`, `layout::admit_frame` with Keyless, `relation::compile_bindings` |
| External discovery/derivation | `local_deployment::derive_all`, `proving::derive_public_artifact`/`derive_artifact_material`, `parameters::PinnedCallParameters` |
| Opaque import equality/purpose | `call_artifacts::snapshot_public_keyset`/`snapshot_proving_keyset`, `local_deployment::preflight`/`compare_public`/`reopen_local_proving_deployment` |
| Complete map and frame | `build_local_installed_snapshot`, `layout::admit_frame` with Installed and `PublicLocalDeployment::admit_operation` |
| Stored operation/raw owner | `prepare_installed_native_call`, `PreparedInstalledCall`, `prove_installed_call` |
| Complete final comparison/external result | `verify_installed_call`, `verify_proven_call`, `validate_final_intent`, ProofMarker Real verifier |
| Marker-last publication | `export_local_deployment`/`export_proven_call` and CLI `call_io` publication |

This is bounded consumer-boundary QA, not exhaustive verification or a refinement
proof. Source expression evaluation, concrete serialization/ordinal ordering,
file-system races, measured byte/resource work and complete native frame decoding
are separate Rust checks. Frame/key/hash/PI/gas/output-ID symbols abbreviate
external expectations; structural commitment tuples do not implement hashes.
Compiler/SDK/VM/codec soundness, pinned SRS/setup, deterministic key derivation,
native degree/fit, actual crypto and embedded verifier compatibility remain
external premises. Caller `max_degree` can refuse a discovered result; it cannot
contain SDK `optimal_k` scans9..=25 or repeated keygen/keygen_vk `k()` work.
Authenticated state, signatures, history, funding, nonce consumption, fees,
application, settlement and recursion remain open. No native/SRS/keygen/proof/
cryptoverification/MockProver/ignored test/network action was run by this QA.
Update the model and corresponding source checks when this contract changes;
later source changes need refreshed alignment and current independent audits.

```sh
quint typecheck installed_call_consumer_test.qnt
quint test installed_call_consumer_test.qnt --main installed_call_consumer_test --backend typescript --match 'Test$' --seed 1012
quint run installed_call_consumer.qnt --main installed_call_consumer --backend typescript --invariant safety --witnesses preflightWitness discoveryWitness regenerationWitness comparisonWitness installedWitness preparedWitness finalWitness externalResultWitness leavesWitness acceptedWitness refusedWitness keygenWitness coldProveWitness pkFreeVerifierWitness lazyInitRefusalWitness unusedActionRefusalWitness purposeRefusalWitness weakKeyRefusalWitness --seed 1012 --max-samples 100 --max-steps 10 --verbosity 1
```
