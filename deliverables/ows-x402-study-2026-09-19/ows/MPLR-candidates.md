# OWS-derived MPLR candidates

Research refinements only; no stable IDs allocated. These supplement the existing authority, evidence, upgrade, partial-settlement and PCD requirements. Exact evidence is indexed in [claims.json](claims.json). They preserve permissionless development by every Midnight DeFi developer compiling to the supported ZKIRv3 profile; application-specific counterparty policy remains allowed.

## 1. Delegation must state its actual enforcement boundary

**Behavioral EARS:** When a program relies on delegated signing authority, the acceptance claim shall identify the components able to reconstruct or use the signing secret, the scope enforced at each boundary, and the revocation conditions that remain effective against the declared adversary.

**Motivation:** OWS token-plus-file access can decrypt a complete secret; owner passphrase bypasses policy; deleting the local key file does not erase copied material (OWS02–OWS05). NEAR MPC also separates signature generation from transaction economics.

**Adversarial acceptance:** An agent retains its encrypted secret copy and token, then the owner revokes the ordinary lookup record. The model must not conclude destination spending authority disappeared. If the supported safety profile requires such revocation, it must rely on an actual non-bypassable destination/key-isolation mechanism and state its assumptions. Positive case: a distinct solver uses a valid attenuated grant without any project gate.

**PL questions:** Is enforcement location part of an authority type or proof assumption? How can software mediation, hardware isolation, threshold custody and destination validation share an interface without erasing their different adversaries? What is the semantics of revoking future signing versus invalidating already-issued actions?

## 2. Typed signing operations and complete policy evidence

**Behavioral EARS:** When a policy authorizes an operation, the verifier shall bind the operation kind, domain, complete material effects and required evidence to the actual signed payload; unavailable or unsupported analysis shall not be interpreted as zero effect or unrestricted permission.

**Motivation:** Typed-data allowlist passes other operation kinds; raw hashes have no automatic semantic domain binding; empty effects on EVM do not mean zero transfer; optional-field migration can make a policy fail open; daily_total is a zero placeholder in the inspected code (OWS08–OWS12).

**Adversarial acceptance:** Replace a typed-data request with an opaque digest or omit the old transaction field while retaining the same chain label. A spend cap must reject or demand certified decoding, not authorize by empty effects. Change the claimed daily total from an authenticated counter to a constant; the cumulative-budget claim must fail. Positive case: supported decoded effects with authenticated budget state pass.

**PL questions:** Which sum/refinement/effect types prevent implicit operation-kind coercion? How do versioned policy contexts expose unsupported, absent, unknown and known-empty evidence? How can an authorized class of plans be proved without requiring one fixed transaction before solving?

## 3. Failure effects and persistent settlement accounting

**Behavioral EARS:** While any permitted execution branch can consume value or create a duty, the accepted transition shall account for that branch's complete effects and preserve the successor's residual obligations; broadcast success or missing evidence shall not discharge a delivery duty.

**Motivation:** Cardano reports contingent collateral separately from success effects; the minimal policy guide omits it. OWS returns broadcast identifiers and delegates usage-based accounting, nonce coordination and reconciliation upward (OWS15–OWS18/OWS22).

**Adversarial acceptance:** A cap approves successful-path outflow while failure consumes collateral beyond the cap. Or a remote leg times out after funding and the orchestrator refunds while delivery later completes. The proof must cover the actual allowed failure branch and preserve accounting through the race. Positive case: destination is funded, the required signatures/documents/proofs/recipient action are verified under policy, and release occurs once with bounded remaining duties.

**PL questions:** How should phase-indexed effects and continuation joins express mutually exclusive outcomes, unresolved observations and collateral? Which authenticated state and ledger uniqueness assumptions prevent conflicting retries? How does the lowerer prove correspondence to actual Midnight ZKIRv3 guaranteed/fallible phases rather than borrowing Cardano or NEAR semantics?

## 4. Policy evolution and PCD cannot be an advisory allow log

**Behavioral EARS:** When a policy schema, interpreter, signing adapter or verifier changes, acceptance shall preserve the signed policy's authority, disclosure, cost and residual-duty constraints or obtain its required amendment authorization; a proof claim shall recursively validate the bound compliance relation rather than merely authenticate historical allow results.

**Motivation:** OWS explicitly documents deny-to-allow migration despite normal executable failure handling; ordinary append-only audit records and subprocess execution provide no PCD theorem (OWS11/OWS13/OWS23). The current optional isolation profile is not hardware attestation (OWS05).

**Adversarial acceptance:** A measured executable or signed audit entry says allow under a newer schema whose omitted field drops a condition. The kernel must reject a proof against the wrong relation even if the attestation or signature is authentic. Positive case: a semantics-preserving certified migration retains recovery rights and private lineage.

**PL questions:** What total policy core and extension discipline make these preservation obligations tractable? What statement binds verifier keys, policy semantics and actual target artifact? How are private predecessor context and completeness of liabilities proved without exposing unnecessary data? Turing incompleteness and PCD reduce selected risks but do not create external truth or liveness.
