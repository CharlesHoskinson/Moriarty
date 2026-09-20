---
title: "Federated kernel evidence boundaries"
type: research
status: research-draft
created: 2026-09-19
updated: 2026-09-19
diataxis: explanation
tags: [moriarty, daml, research]
---

# Federated DeFi kernel: authority and evidence boundaries

Research proposal, 2026-09-19; configured reviewer GPT-6 Astra, medium effort. This note applies the user's federated-kernel steering to the reviewed security direction. It proposes obligations; it does not establish a deployed kernel, a PCD construction or an end-to-end NEAR guarantee. The kernel coordinates solvers, orchestration and multichain settlement using ZK, MPC and TEE mechanisms. These mechanisms establish different facts and cannot substitute for one another.

Reviewed security-direction SHA256: `3d1903e88b5bc670febc2abab1825a497694a8d0b8e4723afd9380ea5700f479`. Source basis: [Daml authorization claims](../../../deliverables/daml-study-2026-09-19/studies/authorization/claims.json), [NEAR abstraction report](../near/index.md) and its [pinned claims](../near/reference.md). This is a bounded synthesis of already inspected sources; no additional implementation or hardware-security audit was performed.

## Roles and trust

| Role or boundary | What it may supply | What acceptance must establish or explicitly assume |
|---|---|---|
| Intention principal and application counterparties | Signed constraints, delegation, amendment and recovery policies | Authentic key-to-principal mapping; consent to material effects and duties; domain/version/freshness binding. Application restrictions are legitimate; no Moriarty project admission is required. |
| Solver or planner | Candidate route, quote, program inputs, proposed partial-fill schedule | Treat as economically adversarial. Check plan admissibility before ranking, including fees, asset identities, liability changes and recovery paths. A competitive quote is not owner authority. |
| Orchestrator or relayer | Scheduling, transport, retry, proof delivery and observations | Treat as able to omit, reorder, duplicate and equivocate. It may consume only explicitly delegated authority. Correlation IDs and service acknowledgments do not prove settlement. |
| ZK prover | Witness and proof for a specific acceptance relation | May be modified or malicious. Soundness must constrain every accepted witness, bind the actual ZKIRv3 artifact and ledger effects, and preserve the declared disclosure policy. |
| MPC signing federation | Signature under a distributed key | State threshold, membership epoch and adversary assumptions. A signature establishes key authorization under those assumptions; it does not establish economic validity, remote execution or finality. |
| TEE service | Attested measurement and an authenticated result under a selected platform policy | State manufacturer/attestation roots, measurement/version policy, freshness, input/output binding and rollback assumptions. A measurement does not prove business correctness or external truth. Confidentiality and side-channel resistance remain separate claims. |
| Chain adapter and its observers | Encoded transaction, submission receipt, execution/finality evidence | Pin chain/network, contract/account, asset, nonce, semantic adapter version, evidence verifier and finality policy. Distinguish observation trust from the destination chain's own validation. |
| Kernel ledger/state service | Current resource state, replay exclusion, workflow lineage | Name the actual uniqueness and authenticated-state mechanism. A federation's signature over a stale or incomplete view is not evidence of current exclusive availability. |
| Document issuer, oracle or recipient | Signed documents, proofs, attestations, acknowledgment/action | Bind subject, content, predicates, timing and revocation policy. Authenticity does not prove the asserted real-world fact; nonresponse does not imply consent. |

“Trusted” must be a property-specific assumption, not a permanent label attached to an operator. A solver can be untrusted for safety but necessary for a particular liveness path. An MPC participant may be trusted only within a threshold assumption. A TEE host may be untrusted while a platform root and enclave implementation remain assumed. If the same organizations operate solvers, signers, attestors and state observers, the model must account for correlated compromise rather than multiply nominally independent assurances.

## A solver plan is subordinate to signed intention

The signed formal intention defines permitted transitions, including alternative valid routes, partial outcomes, fees, evidence policy, amendments and recovery. A solver supplies one candidate realization. A proof must establish that this realization satisfies the intention under the bound assumptions; the solver cannot add a recipient guarantee, widen an asset set or change an evidence threshold by placing it in a route object.

A commitment should connect intention identity, program/property and semantic version, plan or allowed plan parameters, stage/attempt, authenticated predecessors, chain-specific effects, residual duties and target artifact/verifier. Some fields may be hidden, but equality and policy checks must remain enforced. Do not require a new signature for each descendant when an applicable standing policy already grants scoped authority. Conversely, an outer signature cannot be treated as ambient authority in every nested helper. Daml DA01–DA06 supply this distinction; DA12 shows why a privately revealed subtree still needs its real root authority.

NEAR's account keys, MultiPayload intent signatures, 1Click quote signatures and service credentials represent distinct authority layers. Its SDK admissibility checks before quote ranking are a useful reference, not Moriarty's general refinement proof. Chain Signatures requests bind supplied bytes to a derived key; the immediate predecessor/path matters for principal selection. NEAR A-mpcsign/A-mpcwire and AA-quotecheck support these limited comparisons.

## MPC, TEE and ZK must meet at the same statement

A safe candidate architecture makes the authorization predicate explicit before releasing an external signature. The MPC request must be bound to the exact transaction digest, execution domain, key derivation, workflow stage, allowed attempt and proven policy result. Nodes must not infer authority merely because a request arrived from a known coordinator or an attested enclave. If an external account accepts the bare threshold signature, a compromised signing threshold can bypass an off-chain proof-checking policy unless the destination enforces an additional check. That is a material trust boundary, not a defect repaired by calling the surrounding service ZK-enabled.

TEE attestation can support a claim about the measured component participating in a protocol. It does not establish that the measured program implements the formal intention, that its inputs represent current chain state, or that its output was consumed by the transaction ultimately signed. Fresh challenge/session binding, input/output commitments, version policy and rollback/fork handling need their own obligations. A correctly attested enclave can execute a wrong policy perfectly. These are proposed trust requirements, not claims about a particular NEAR TEE implementation.

ZK proves the selected relation over its bound statement. PCD recursively proves legitimate genesis and compliance of predecessor transitions, with compatible versions, authority scopes and conserved duties. Neither proves an unverified external observation merely because the observation was signed or produced inside a TEE. Neither independently prevents two valid branches from spending the same external resource.

Define the composition rule explicitly: are proof verification, attestation and threshold authorization all required, or are some alternative evidence paths? An OR fallback through a weaker authority inherits that path's assumptions. An AND rule still needs common statement binding; a valid proof for transaction X plus an attestation/signature for Y is insufficient. Key rotation, federation membership changes and verifier/enclave upgrades must preserve the signed evolution policy and existing obligations.

## Federation and chain progress

For each federation epoch specify n participants, the signing/decision threshold, tolerated corrupt and unavailable sets, quorum-intersection requirements where applicable, and recovery/rotation authority. Do not infer Byzantine safety from the word federation or from an unspecified majority. A threshold signature is not automatically a consensus certificate for one globally ordered workflow state. State rollback, equivocation and double-signing need explicit exclusion or detection/compensation assumptions.

Unavailable solvers, signers, enclaves, bridges, chain validators or evidence providers can stop progress without violating safety. Recovery must be authorized in advance or by valid amendment, and may itself need unavailable participants. The specification should state when funds can remain locked indefinitely under failed availability premises. Permissionless Moriarty development does not imply that every optional external signing service must serve every request; service refusal cannot become a hidden language-wide admission rule.

Each adapter defines an evidence progression such as proposed, submitted, observed execution, final under policy, rejected, or unresolved. These are chain-specific judgments, not universal receipt labels. Bind confirmation/finality parameters and reorganization handling to the signed policy. A source-chain debit and a destination receipt may be separately final; a cross-chain atomicity claim requires an additional protocol and stated assumptions.

NEAR provides concrete negative witnesses: detached withdrawals survive the synchronous verifier boundary; failed `ft_transfer_call` resolution may follow a real downstream transfer; one MPC request key can represent several queued invocations. These justify distinct semantic request, attempt and accepted-effect identities and adapter-specific evidence interpretation (AA-detached, AA-ftresolve, AA-fanout). They do not establish global rollback or kernel-wide exactly-once execution.

For the user's conditional-settlement workflow, submit/fund at the destination, then wait for the bound combination of signatures, documents, proofs, recipient action and other supported conditions. Record destination funding separately from final delivery. Release only under the complete predicate; unresolved evidence preserves the asset reservation and residual duty. If one leg is final and another fails, retain the final effect and prove only the authorized residual transition or separately funded compensation. A timeout is not proof of no effect.

## Three requirement refinements and hostile traces

### Candidate A: common-statement authority across solver, prover and signer

**EARS:** When a candidate plan requests any kernel-mediated external effect, the kernel shall establish that the exact effect and its material terms satisfy the applicable signed formal intention, and bind any proof, attestation and signing authorization used for acceptance to the same domain, stage, artifact/version and effect identity.

Refines existing authority/intention requirements, especially MPLR-008/014/016/018/019; allocate no new ID here. Motivating patterns: NEAR MPC supplied-byte signing and predecessor-derived principal; quote admissibility before ranking; Daml's immediate-parent authority and missing root context.

Hostile trace: a solver obtains proof for transfer X, then sends digest Y to an attested coordinator and threshold signer. Y changes beneficiary or creates a recipient duty. Acceptance must reject the mismatch even if each component independently reports success. Positive witness: a different solver proposes a different admissible route under the same policy and succeeds without project registration.

Open PL questions: How should effect commitments and proof-indexed signing capabilities express exact binding while permitting signed classes of plans? Which checks are static, proved, or enforced by the destination? If the destination accepts only a bare signature, exactly which threshold-corruption assumption is unavoidable?

### Candidate B: federation epoch and trust-policy continuity

**EARS:** When membership, keys, attestation policy, verifier relation or adapter semantics changes, acceptance shall require the applicable signed evolution policy or amendment authority and shall preserve outstanding duties, recovery rights and replay scope across the change.

Refines MPLR-008/014/018 and the existing evolution candidate. Motivating patterns: Daml compatible upgrades may change controllers/bodies; NEAR signing authority depends on derived principal/domain, independently of economic correctness.

Hostile trace: an operator rotates to a quorum it controls or enables an attestation-only fallback, then signs release of a still-pending escrow under the old workflow identifier. It must fail absent authorized evolution. Positive witness: an authorized key rotation with unchanged obligations and authenticated state continuity remains usable.

Open PL questions: Should trust policy be part of a continuation's type, a refinement on the state transition, or a versioned external assumption object? How can migration prove attenuation or equivalent authority without freezing all operational recovery? How are correlated failures and emergency transitions represented without importing deployment approval?

### Candidate C: evidence-indexed partial settlement and recovery

**EARS:** While any destination leg is unresolved or conditionally funded, the kernel shall retain its reservation and residual obligations, and shall authorize delivery, retry, refund or compensation only from authenticated adapter evidence and the signed stage policy, accounting for every already-final effect and applicable resource-uniqueness rule.

Refines MPLR-002/005/006/011/014/017 and conditional-evidence requirements. Motivating patterns: NEAR detached effects and failed transfer-call ambiguity; Daml `Settle = pure ()` consuming a record without delivery.

Hostile trace: destination funding succeeds, recipient action is withheld, a callback times out, and the orchestrator requests both refund and retry. A late final delivery arrives after the refund request. The model must preserve the unresolved duty, reject duplicate unauthorized spend and resolve the race under the signed recovery policy; a callback timeout alone cannot justify refund. Positive witness: final funding plus all required evidence releases once, with a proof of the remaining accounting state.

Open PL questions: Which evidence refinements and continuation/join rules can preserve unresolved outcomes without dead-end overrestriction? How is finality weakening or reorganization represented? What authenticated completeness and uniqueness premises are needed when obligations span private state and multiple independent ledgers?

## Bounded conclusion

The kernel is an evidence-and-authority composition boundary, not an automatic source of trust. Its strongest defensible research goal is a conditional theorem connecting signed intention, recursively justified state, exact target execution and per-chain evidence under explicit cryptographic, federation, hardware and ledger premises. The current security proposal supports this direction; the unresolved work is to select and prove those relations and demonstrate non-vacuous positive and adversarial traces on the actual Midnight ZKIRv3 path.


[Study index](index.md)
