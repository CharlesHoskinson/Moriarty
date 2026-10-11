# Moriarty language Quint QA

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

`native_transition.qnt` and `native_transition_test.qnt` model the new public
local proof verifier separately from ledger acceptance. Five deterministic
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
