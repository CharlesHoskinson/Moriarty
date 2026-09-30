# MIL/2 research findings — draft

Date: 2026-09-29. Repository baseline: `983a4bb49e3ccae399ee2514da2f6fa3f03593fd`. This is a source-grounded research draft. It neither freezes MIL/2 nor closes a U0 predicate. The 15 public primary-source captures and their SHA-256 values are in [sources.json](sources.json). The exact retrieved text is in `source-text/`; source links below point to publishers.

## Findings that change the next sprint

### R1. The escrow authoring check has a narrower decision procedure than its grammar

**Repository observation.** [MIL/2 §4.4](../../concepts/intent-language/DESIGN-MIL2.md) distinguishes difference logic from QF-LIA, yet §7 and §15 call the general `after(deadline) ⇒ release_when ∨ refund_when` authoring check “difference logic” and “no solver.” The guards are arbitrary Φ₀ predicates, including sums, literal multiplication, `min`/`max`, disjunction and `k_of_n`.

**Source fact.** [SMT-LIB QF_IDL](https://smt-lib.org/logics-all.shtml) admits atoms comparing variables or differences against constants. Its QF_LIA definition permits linear arithmetic with concrete coefficients. The two fragments have different admissible terms. Capture: `source-text/smt-lib-logics.md`, QF_IDL and QF_LIA sections, lines 722–792.

**Inference.** The negative-cycle algorithm applies only after a sound restriction or reduction to difference constraints. The full Φ₀ authoring check needs a separate decision procedure, explicit finite enumeration, or a narrower guard grammar. The sprint should construct a legal Φ₀ guard with a three-variable sum and show that the proposed difference-logic checker cannot directly represent it. The eventual acceptance result must fail closed on unknown or timeout.

### R2. The showcase footprint does not contain its effects

**Repository observation.** [MIL/2 §9](../../concepts/intent-language/DESIGN-MIL2.md) requires `declared ⊇ derived`. In §13, `on_release` transfers A from escrow E to `counterparty`, and `on_refund` transfers A from E to `owner`. The declared write footprint lists `escrow(E)` and `balance(owner,B)`, but omits `balance(counterparty,A)` and `balance(owner,A)`; the asset A debit cell must also be represented under the selected cell model. This is a direct counterexample to the example's implied admissibility unless `escrow(E)` is formally defined to encompass all those balances, which §9 does not say.

**Source fact.** [Liquid Effects](https://goto.ucsd.edu/~rjhala/papers/deterministic_parallelism_via_liquid_effects.html) describes read and write effects as predicates used to reason about deterministic parallel execution. [Separation-logic frame-rule work](https://arxiv.org/abs/cs/0610081) treats local reasoning as dependent on a sound account of the state a command can affect. These are method references, not proofs about MIL/2.

**Inference.** Define the derivation rules for transfer, guards, dynamic cells, and aliases before asserting a checked footprint. Run the derivation on every showcase branch; it should reject the current declaration or produce a justified corrected one.

### R3. Acceptance refinement needs a denotation for the unfilled intent

**Repository observation.** [MIL/2 §8](../../concepts/intent-language/DESIGN-MIL2.md) states `Accepted(I[σ]) ⊆ Accepted(I)` and says the circuit checks it per concrete filling. `Accepted(I)` is not defined for a template with holes. Merely checking that `σ` lies within declared bounds does not by itself show that the completed program preserves every fixed condition, evidence requirement, or resulting effect.

**Source fact.** [The formal synthesis paper](https://arxiv.org/abs/1505.03953) distinguishes a formal specification from the candidate program learned through oracle queries. It does not supply MIL/2's completion semantics; that relation must be defined here.

**Inference.** Define `Accepted(I)` as the union of the traces accepted for all admissible fillings, or choose another explicit denotation. Define a projection that forgets completion-private choices while preserving signed fields, obligations, effects and evidence classes. Then state refinement over that projection and prove it by cases over each hole type. Include a completion that respects a numeric bound but changes a recipient or evidence policy as a negative case.

### R4. Evidence labels need a binding from source identity to checked facts

**Repository observation.** [MIL/2 §5](../../concepts/intent-language/DESIGN-MIL2.md) propagates source classes such as `anchored@d`, `imported(policy)@d`, and `attested(issuer,k,n)@d`. It does not yet define a per-observation identity in the source set or a complete checked relation from an observation's claimed value, issuer, time and status to a particular ledger read or verification result. The design correctly marks non-laundering as an obligation.

**Source facts.** [The decentralized-label paper](https://www.cs.cornell.edu/andru/papers/sp98/paper.html) models labels and their joins for information flow; [W3C PROV-O](https://www.w3.org/TR/prov-o/) represents derivation through entities and activities. [W3C VC 2.0](https://www.w3.org/TR/vc-data-model/) separates issuer statements, evidence, status and verifier judgment; the [Bitstring Status List](https://www.w3.org/TR/vc-bitstring-status-list/) supplies one status mechanism. None makes a raw label into proof that a financial assertion is true.

**Inference.** Research a typed evidence record with observation ID, origin, policy, domain, observed-at, expiry/status reference, and the exact authenticated value commitment. Prove source-set preservation through every Φ operator, including Boolean short-circuit and `pre`/`post`. Distinguish syntactic dependency, causal derivation and cryptographic authenticity.

### R5. Recovery must split local safety from external progress

**Repository observation.** [MIL/2 §7](../../concepts/intent-language/DESIGN-MIL2.md) correctly separates a deadline from refund entitlement and lists recovery viability as an obligation. It has not specified the exact acceptance predicate or the evidence needed when the foreign outcome is unknown.

**Source fact.** [IBC ICS-004](https://github.com/cosmos/ibc/blob/main/spec/core/ics-004-channel-and-packet-semantics/README.md) requires a destination-chain timeout proof; absence of a response on the sending chain is insufficient. [IBC v2 packet semantics](https://github.com/cosmos/ibc/blob/main/spec/IBC_V2/core/ics-004-packet-semantics/PACKET_HANDLER.md) uses receipt and non-membership proofs to prevent both receipt and timeout for one packet. These are comparative protocols, not a ready-made Midnight adapter.

**Inference.** Model `pending`, `received`, `unknown`, `refundable`, `released`, and `refunded` separately. Require an authenticated negative outcome or an explicit trust policy before allowing a cross-domain refund. State conditional liveness under chain availability, relay, witness access and authorized actor assumptions; prove mutual exclusion as a safety property independently.

### R6. The encumbrance invariant needs reservation and release semantics

**Repository observation.** [MIL/2 §3.4](../../concepts/intent-language/DESIGN-MIL2.md) states `Σ active locks ≤ balance` per owner and asset. It does not define whether pending, challenged, released-but-unfinalized, or cross-domain locks are “active,” nor how multiple solvers reserve the same collateral during partial progress.

**Source fact.** [Nomos](https://arxiv.org/abs/1902.06056) uses linear types to prevent asset duplication or deletion and resource-aware types to bound consumption. Its abstract supports the design direction, not a proof of this specific invariant.

**Inference.** Define lock lifecycle and an invariant over committed plus reserved encumbrances. Show that lock release is tied to a unique authenticated transition and does not erase a debt. Explore one collateral position backing several obligations with priorities, and two concurrent candidates that each fit individually but exceed the shared balance together.

### R7. ZKIR formal results have explicit caller and witness premises

**Source fact.** The pinned [ZKIR v3 specification](https://github.com/midnightntwrk/midnight-zkir/blob/47793c8ab042aa5a91d1a4672c6b82de6bdf9dd8/zkir-spec/docs/zkir-v3-spec.md) describes the typed IR and its trust base. Its [Agda formalization README](https://github.com/midnightntwrk/midnight-zkir/blob/47793c8ab042aa5a91d1a4672c6b82de6bdf9dd8/zkir-spec/src/zkir-v3/README.md) states statement soundness for satisfying witnesses with `WShape` and producer well-typedness premises. It also says exact equality with every witness cell is false because some transcript input cells are unconstrained.

**Inference.** For each MIL/2 stage field, specify which circuit constraint, public input, ledger check, or authenticated read enforces it; show the caller discharges the ZKIR premises. A proof of the generic IR relation is not yet a proof of MIL/2 source-to-ledger correspondence. The sprint should prioritize intent digest, observation binding, actual effects, and terminal tombstone.

## Boundary and source quality

The ACTUS page captured here is a documentation index, not an ACTUS rule; it cannot validate the MIL/2 obligation formula. The arXiv and Liquid Effects captures are abstracts or landing pages, so they guide the next reading pass and do not support theorem-level claims. The two IBC pages are from the same standards repository and share an independence key. No compiler, prover, K differential, TypeScript evaluation, or Preview transaction was executed for this draft.
