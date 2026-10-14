# Ledger protocol design draft

## Status and provenance

This document is specified-only. Proposed types and prospective commands are not current APIs.
The base is `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`.
Current custody files are moving Grok4.7 implementation observations, not frozen acceptance evidence.
Read-time file hashes appear in the companion `INPUT-SHA256.json`.

The immutable reviewed candidate is `next-backend-plan/revision03/CANDIDATE-SHA256.json`.
Its digest is `dcc62e21e9e351843bd04a3be537ebe260f8c1d0ec606cfcc5e15fd83ca7aff0`.
Both `revision03/design-review-a/REVIEW.md` and `revision03/design-review-b/REVIEW.md` record `APPROVE_SCOPED_T1_DESIGN`.
Their approval concerns the scoped design, not implementation or native execution.
Revision02 CHANGES_REQUIRED reviews and all earlier dissent remain preserved.

The independent source packet is `t1-independent-oracles-19/`.
Its digest is `9d71e736e3d3c159a83ff3bb21dbb3f5b2a9661dd90ed1e70414c2690b6552d6`.
`RESULTS.json` contains 46 full live source envelopes.
`ORACLES.json` contains complete sourceBindings and expected compiled-validator templates.
Its UPLC and native templates are not executed ledger evidence.
Source codes remain separate from admission codes and node rejection classes.

Qualified graph navigation uses `graphify-out/knowledge-focus.json` and `GRAPH_REPORT.md`.
The graph reports targeted semantic coverage, not full semantic review of the large corpus.
Its genesis-precondition node locates the seed/name/quantity limitation.
The retained source reports, snapshots, and current source bytes govern claims.
No graph regeneration or complete graph load occurred.

## Existing source boundaries

| Profile | Contract | Lifecycle | Source input |
| --- | --- | --- | --- |
| financial-agreement-source/5 | financial-expression-contract/4 | financial-lifecycle-state/1 | designers/02-escrow-time/escrow-custody-v5.mori |
| financial-agreement-source/6 | financial-expression-contract/5 | financial-lifecycle-state/2 | designers/01-state/head-step.mori and designers/05-authority-private/authority.mori |

Every tuple uses the full `moriarty-` prefix in artifacts.
`createFinancialAgreementSourceV5().elaborate`, `.check`, and `.evaluate` are existing source APIs.
The /6 factory supplies different authority-bearing semantics.
`elaborate()` alone does not produce complete evaluation evidence.
The prototype `moriarty-bounded-atomic/1` is a distinct backend predicate.
No tuple substitution, ordinary-only projection, or easy-prototype fallback proves the target financial relation.

## Proposed compiler and ledger interface

`AdmittedSemanticEnvelope/1` is a proposed versioned record.
It contains sourceProfile, contract, lifecycle, action, sourceArguments, exact source/Core/schema/bounds hashes, and constructor/work accounting.
It contains complete pre, financialPre, post, financialPost, effects, and workRemaining.
On source rejection, it contains the full original failure envelope, including spans and partial work where present.
The compiler exports this envelope and the admitted constructor/effect set.
The ledger checker derives its prestate from resolved authenticated inputs, rather than trusting host copies.

`HeadEnvelope/1` is a proposed generalized record, not a change to the frozen T1 ABI.
It contains GenesisCommitment, deployment identity, instance identity, lineage, transport revision/remaining, ordinary state, and complete lifecycle state.
It contains an immutable authority commitment and the nonce-authority resource identity.
The committed head includes balances, allowances, obligations, all four operation-ID histories, and remaining/spent/closure work.
For lifecycle/2 it also includes the exact admitted authority record.
The existing T1 HeadDatum fields keep their reviewed 13-field order.
Extensions receive a new ABI version and measured codec vectors.

`ResolvedLedgerContext/1` is proposed runtime input to the independent relation checker.
It contains spending-purpose input, consumed inputs, reference inputs, outputs, fee, mint, required signatories, and validity interval.
It includes all other V3-visible channels or an explicit refused-feature list.
Unsupported certificates, withdrawals, proposals, treasury changes, or other value channels do not disappear from the projection.
Full addresses include payment and declared staking credentials.
Runtime output indices and input out-references are canonical transaction indices.

`SettlementAllocation/1` contains entries `(instanceId, headOutRef, effectIndex, outputIndex, asset, amount, recipient)`.
Each claimed payout output belongs to exactly one effect allocation.
Each required effect has one full exclusive payout output in the initial profile.
Change and successor outputs cannot simultaneously satisfy payouts.
Future shared-output aggregation needs an independently reviewed bounded allocation rule.
The checker evaluates all participating heads as one transaction relation.

`RelationDecision/1` reports accepted/refused, requirement ID, layer, original source code, adapter code, and native class.
It identifies the measured context and expected envelope hashes.
It never turns a failure-class alias into equivalent evidence.

## GenesisCommitment and trusted limits

`GenesisCommitment/1` is a proposed closed canonical record.
Its fields are protocol/domain version, chain genesis discriminator, network label, deployment ID, and source profile tuple.
It includes source/program/schema/seed/bounds commitments, immutable role credentials, assets/quantum, initial ordinary state, and full initial lifecycle state.
It includes lifetime/nonce/fan-in/collection bounds, seed out-reference, initial lineage, and allowed lifecycle transitions/programs.
It includes the declared staking credential and sponsorship/closure funding policy.
Genesis commitment excludes later scriptHash, threadPolicyId, and instanceId to prevent self-reference.
Its own versioned domain hash binds every listed field.

The constrained thread policy consumes the exact configured seed out-reference, not a reference input.
The mint branch enforces quantity one, exact asset name, exact destination, revision zero, and the full constrained seed.
Mint entries under that policy contain only the authorized identity asset.
Unrelated minting requires an explicit separate admitted effect and policy.
T1 transfer steps require zero mint of thread and Cash.
A terminal branch burns exactly the declared identity and checks closure authority.
Counterfeit outputs can exist at the same script address.
They cannot authenticate an instance without the genuine policy identity.

One-shot gift-card evidence constrains only seed, name, and quantity.
T1's trusted lock remains usable with the explicit trusted-genesis label.
It does not close S01.
Constrained genesis proves consistency with registered commitments, not truthful oracle observations or legitimate real-world economics.
The genesis signer and deployment registration remain explicit trust assumptions.

## Proposed acyclic RuntimePins amendment

The frozen revision03 prose asks for script identities before compilation and derives them after compilation.
This draft replaces that circular operational wording with the following proposed dependency protocol.
The amendment does not reinterpret the old approval as approval of this protocol.

1. Freeze source, Core, schema, seed, bounds, and GenesisCommitment bytes.
2. Bind actual Cash-policy identity and public role hashes from independently verified public artifacts.
3. Compile a spending validator without its own scriptHash, threadPolicyId, or instanceId as static parameters.
4. Derive the Cardano spending credential from the emitted Plutus version and canonical script bytes.
5. Apply the pinned thread-policy template to that credential, seed out-reference, and GenesisCommitment.
6. Derive the policy ID with the exact Cardano script-hash domain and serialized applied-script bytes.
7. Compute instanceId using the declared instance domain and actual policy, asset, and deployment fields.
8. Recompute every artifact identity independently before emitting `RuntimePins/1`.
9. Compare supplied native IDs with those recomputed identities before native admission.

These sentences are prospective procedures, not actions performed by this draft.
The spender identifies script-own Escrow from its spending-purpose input credential.
It verifies expected thread-policy identity using a pinned bounded `PolicyHashWitness/1` reconstruction relation.
The witness contains canonical applied-policy bytes and the exact template/parameter decomposition.
The checker accepts only that fixed template applied to own credential, seed out-reference, and GenesisCommitment.
A caller-chosen policy byte string is insufficient.
The relation requires measured byte-for-byte Cardano hash-domain and canonical parameter application parity.
The on-chain hash algorithm and byte-size cost require actual UPLC/cost-model measurements.
No implementation feasibility is claimed here.
If the exact reconstruction exceeds backend bounds, its implementation task remains open.
An immutable authenticated deployment registry would be a different design amendment with explicit trust and lineage obligations.
Neither arbitrary policy data nor an unchecked registry substitutes for this relation.

The minting validator obtains its own policy credential from its minting purpose.
It derives instanceId after policy application, avoiding instanceId in GenesisCommitment.
It checks destination against its statically pinned spending credential.
The spender verifies the same reconstructed policy and derives the same instanceId.
No recompilation of the spender after policy generation occurs.
The /5 instance digest retains the revision03 domain and exact six-field preimage.
The generalized instance domain also commits seed/genesis/authority provenance through the policy and explicit GenesisCommitment.

Future values may be absent in a draft or compile-plan record.
They cannot be native IDs or emitted executable constants.
All-zero 28-byte/32-byte identifiers and labeled shape samples fail native admission.
Cash and Head names remain `43617368` and `48656164` for the retained T1 profile.
A native policy ID is 28 bytes, not a source program SHA-256.
Actor payment hashes are Blake2b-224 verification-key hashes.
Escrow is the script credential and never a payment verification-key hash.
Runtime key generation is outside this documentation task.

## Complete T1 correspondence foundation

The three actions are release, refund, and cancel with explicit redeemer tags 0, 1, and 2.
The tags select action identity, not actor/window inference.
The original /5 sources use quantum1 and one Cash100 Transfer.
The source work count is 48 constructors plus one lifecycle debit.
Initial work1024/spent0/reserve16 becomes975/spent49/reserve16 for the control seed.
Ordinary locked1/revision0 becomes locked0/revision1.
Transport revision0/remaining1 becomes revision1/remaining0 independently.
Thread stays one, head Cash becomes zero, and recipient receives Cash100 in an exclusive output.
ADA sponsorship, fees, and collateral remain distinct from kernel Cash balances.
Unrelated assets stay with their declared owner or sponsorship frame.

The complete oracle includes varied lender/borrower/witness balances, allowance history, prior IDs, revision4, and additive work.
The missing-lender-row positive creates the row in the source-defined order.
No hardcoded control post substitutes for those envelopes.
`release-work-one-short` is source WORK_EXHAUSTED, not a renamed ABI error.
E12 prepares at the source layer with outstanding40 but the T1 admission fence refuses NONEMPTY_OBLIGATIONS.
Profile /6 at the /5 boundary refuses PROFILE_MISMATCH.
Original millisecond windows are source/unit vectors, not live inclusion windows.
The runtime package owns actual POSIX parity, calibrated sibling hashes, and slot-valid inclusion measurements.

## Durable authority and consent

`AuthorityState/1` is proposed authenticated per-instance consumed state.
It stores next nonce, consumed/cancelled outcome IDs, immutable role map, program allowlist, and bounded remaining authority.
Single-head outcomes can retain this state in the head.
Split instances use one separately authenticated authority UTXO A, consumed and recreated for every nonce-changing step.
References to A provide observations only.
This deliberately serializes authority updates and makes no cross-head parallelism promise.
Each used/cancelled nonce remains durable across split, join, migration, and retirement.
Registry growth is bounded by the committed lifetime capacity.
Capacity exhaustion refuses new outcomes without resetting history.
An authenticated tombstone preserves the terminal consumed/cancelled history or its committed immutable archive root.
A root without an enforced membership/nonmembership relation is insufficient replay protection.

Exact-head authority binds predecessor out-reference, revision, commitment, action, arguments, deployment, program, and validity.
Consumption prevents repeating that exact predecessor.
Outcome authority separately binds constraints, fees, gross caps, net goals, program allowlist, expiry, and durable nonce scope.
A globally-once signature across instances needs a shared consumed authority resource.
Per-instance records do not provide global uniqueness.
Cancellation consumes or updates authenticated authority, not a host request file.

Immutable declared role hashes appear in required signatories.
The ledger checks corresponding transaction-body witnesses.
Funding-input witnesses alone do not satisfy script signatories.
Multiple roles require every declared credential.
Role-token mode requires exact policy/name/quantity/instance and actual consumption according to its declared return/burn policy.
A reference-only role token and host boolean fail consent.

TxInfoV3 has no collateral input, total-collateral, or collateral-return fields.
`FinalBodyWalletPolicy/1` is a proposed separate wallet interface over canonical final-body bytes and resolved collateral inputs.
It measures complete collateral exposure and signed limits before wallet consent.
Altered body bytes require renewed witness binding.
Normal head settlement and phase2 collateral-only inclusion remain separate outcomes.

## Proposed split, join, migration, and retirement descriptors

`LifecycleDescriptor/1` is proposed source/backend boundary data.
It identifies action, input lineage set, output lineage set, identity mint/burn rule, state partition, work debit, and required roles.
It identifies the authority resource, obligation ownership mapping, ID namespaces, value frame, and exact allowed program changes.
Only missing lifecycle primitives justify a new source profile.
Existing finite fee, transfer, accrual, origination, and repayment economics remain current-source targets.

Split numeric oracle from designer01:

| Field | Parent | Child0 | Child1 |
| --- | --- | --- | --- |
| Gross remaining/spent |100/20|60/12|40/8|
| Cash |100|60|40|
| Work remaining/spent/reserve |200/30/20|119/19/12|80/12/8|
| Duties |O1 principal80, O2 principal20|whole O1|whole O2|

The proposed Split debit is one unit, not an existing runtime charge.
Child identities derive from parent out-reference and distinct child index under the authorized mint transformation.
Parent identity burns and exactly declared child identities mint.
Authority A preserves nonce N7.
Operation-ID histories remain globally available through A or disjoint committed namespaces with an enforced uniqueness relation.
No duty has two owners or no owner.

Join consumes both children and A.
The proposed Join debit produces work198/spent32/reserve20, Cash100, gross100/20, O1/O2, and nonce N7.
Fan-in and output counts are explicitly bounded by GenesisCommitment.
Missing children, reference-only children, duplicate indices, copied reserves, and discarded IDs refuse.

Migration oracle preserves revision5 O1 outstanding80, gross80/30, work100/60/reserve16, nonce N7, roles, and complete assets.
The allowed target receives revision6 work99/spent61 with all other inherited fields unchanged.
Only declared migration work and sponsorship costs may differ.
Both current and successor program allowlists and identity-policy target rules authorize the move.
A field rename does not discharge inherited liabilities.
Termination requires zero principal/accrual/outstanding residual duties and exact final payouts.
A duty-output transfer is a different explicitly declared liability-preserving action.
Retirement burns the authorized identity, leaves no live successor, and preserves terminal nonce history.

## Measurement and refusal interfaces

| Layer | Required artifact | Acceptance | Negative control |
| --- | --- | --- | --- |
| Source | complete factory envelope and independent economics oracle | exact pre/post/effects/work | stale revision, duplicate ID, obligation/work mutation |
| Codec | canonical JSON and full CBOR vectors | both decoders reconstruct exact typed fields | field swap, malformed UTF-8, extra/truncated fields |
| UPLC | actual compiled spender and constrained mint policy with pinned contexts/cost model | both predicates execute for positives | counterfeit mint, wrong role, poststate, output reuse |
| Native | canonical signed CBOR, independent TxId, node class, inclusion and resolved UTXOs | complete oracle and native effects read back | re-signed wrong state, reference-only authority, conflicts |
| Concurrent | two distinct valid bodies sharing only declared head/authority | at most one canonical transition with determinate evidence | loser or indeterminate remains classified |
| Proof/private | complete-relation actual verifier and handoff | separate required proof gate | missing proof, changed root, opening/key/program/effects |

Native goldens map every retained source case to actual script/context hashes and expected complete decoded projections.
Readbacks include inputs consumed, outputs, mint, addresses, signatories, fees, full datums, and exclusively attributed effects.
An isValid=false collateral inclusion never supplies a successful financial poststate.
A stale-witness InvalidWitnessesUTXOW does not test wrong-state semantic rejection.
Ogmios evaluation remains preflight evidence.
Sequential BadInputsUTxO remains sequential conflict evidence.
Concurrent pairs submit without awaiting first confirmation, with independent fee and collateral resources.
Rollback or unknown finality leaves native acceptance partial or unverified.
Exact transaction IDs and status lines belong to the authorized future runtime task.
No transaction was created or submitted by this draft task.

## Dependencies and remaining gaps

Compiler correspondence supplies exact admitted semantics, action tagging, closed serialization, work, and full effects.
Native runtime supplies actual contexts, canonical script-hash parity, clock calibration, inclusion, negative controls, and readbacks.
Financial patterns supplies complete duty/economic invariants and multi-head allocation fixtures.
Conformance/private supplies required proof-verifier and terminal 20-family reconciliation.
S01 constrained genesis, S03/S04 descriptors, durable authority, and policy-witness reconstruction are implementation gaps.
They are not external blockers or reasons to reduce required coverage.
Missing actual source freeze and ledger measurements block evidence claims, not draft delivery.
No new RuntimePins values, proof backend, private/PCD acceptance, or Complete status is claimed.

## Later frozen implementation18 observation

Implementation18 is now terminal and frozen at candidate `aed4090af8f1e94e3904140ac2b23be9aa79e85141806841f1066115e5386680`.
`RESULT.md` reports24 Node tests and25 Aiken checks on synthetic Transaction contexts.
`RESIDUAL-LIMITS.md` records nativePins=false, no calibrated source, no node, and no spend.
The blueprint credential differs from the repeated-e5 unit script identifier.
The prior moving-source observations remain read-time observations.
This frozen packet remains implementation-author evidence pending both independent Grok4.7 reviews.
It neither implements nor approves the proposed acyclic policy-hash-witness amendment.

## Existing ledger-anchored source invariants

`openspec/changes/pcd-ledger-anchored-acceptance/specs/pcd-mandatory-acceptance/spec.md` remains authoritative for revised Moriarty claims.
Its initialization starts from `Uninit(Π_P, netTag)`, requires all-principal signatures, and occurs once.
Its instance formula is `H(A || H(G))`.
The /5 experimental six-field instance digest is distinct and cannot silently replace that formula.
A generalized adapter carries both source instance commitment and native identity with an explicit injective commitment map.
The map preserves full address A and canonical genesis G bytes and domains.
One source instance cannot authenticate two native live identities.

Migration preserves immutable operation keys.
Declared migration binds Π_new in Π_old's compiled program-digest allowlist and uses Π_old's declared threshold.
The unanimous branch uses all principals over `MORIARTY-MIGRATE-v2`.
Both branches sign netTag, A_old, A_new, Π_new, revision, and state.
Successor `Uninit(Π_new, netTag, A_old)` records predecessor address.
`ImportFrom` requires that predecessor and a previously unimported commitment.
Downgrades, repeated imports, key substitution, and maintenance bypasses fail.
The experiment's proposed S04 descriptor does not erase either branch.
A backend lacking the complete migration branch refuses the action as an implementation gap.

The proposed zero-duty termination requirement is a debt-free native profile restriction.
The broader source permits a terminal condition or declared threshold with a disposition for every residual obligation.
A supported liability-transfer disposition needs its own conserved-duty relation and evidence.
A debt-bearing source terminal action cannot be mislabeled accepted by the debt-free profile.

ContractInvariant needs the governed checked deploy-time property certificate and per-step invariant.
IntentRefinement and TransitionValidity need the fused step relation.
HistoryCompliance requires ledger applicability plus justified induction or an actual off-ledger certificate.
Public Cardano scripts alone do not establish complete Moriarty private/PCD predicates or Preview qualification.
Missing certificates, requiredClaimRoot binding, or complete verification cannot be waived by a valid native spend.

## Shared finance interface alignment

The finance peer proposes `EffectAllocationReceipt/1`, compatible with `SettlementAllocation/1`.
Its participatingInputs, effect ordinal/id, full recipient address, payout index, and continuation supplement the allocation record.
The peer's `FinancialWalletProjection/1` supplies complete normal-input gross, all-output net, fees, sponsorship, change, and bodyHash.
Collateral remains in the separate final-body policy.
`ObservationBinding/1` supplies authenticated CIP31 fields and whole-window freshness.
These remain proposed cross-package contracts, not present APIs.
