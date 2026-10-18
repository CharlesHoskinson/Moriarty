# Proposed developer experience

Status: PROPOSED journey and interface design, r2 with terminal Opus errata applied. Names below describe future tools unless explicitly identified as source-documented commands. This is not a shipping CLI, SDK, language extension, or wallet integration. It uses the glossary in CONCEPT.md. V2 descriptions are attributed Opus readings of unadopted, uncompiled source. The review did not receive the full four-piece settle implementation or byte/type helpers. No supplied record pins the observed S07/S08 artifacts to those exact bytes. [A03 §0]

## Start with the target and the financial action

The proposed documentation starts with three choices: application role, target/profile, and action. A dapp author chooses the invoice walkthrough. An issuer chooses a lifecycle policy worksheet. An infrastructure developer chooses the application-to-consumer handoff. Each journey identifies the executable/profile, consumer, supported stages, and first gap.

Keep the current Rust experiment separate from the older Source/6 JavaScript path and the required Midnight ZKIRv3 target. The Rust package does not supply the Rust `mori` CLI, LSP, or MCP. [S04 “Package and provenance”] A future wizard may present the same artifacts, but cannot conceal a missing producer, unsupported operation, or unavailable target.

A shared guide explains financial meaning. Separate Cardano experimental and Midnight target guides explain actual artifacts and ledger behavior. A guide never asks for campaign, reviewer, council, or registry records as public developer prerequisites. [S03 “Verification versus project process”]

## Journey A: build a first dapp action

**1. Read and edit a supported source action.** Start with S12 `partial(amount)`. The capability card identifies `moriarty/1`, `moriarty-core/1`, selected action, supported constructs, and refused boundaries. Keep literal source semantics visible: `ensures` observes state at its position, and present-zero liability differs from absence. `await` is not lowered in the documented compiler. [S04 “Supported subset and refusals”]

**2. Derive independent expected effects.** Use the invoice worksheet before invoking the implementation being evaluated. Use two labelled policies. Source behavior: open(90,10) funds escrow 110 and charges buyer 3. Payment 35 plus program fee 2 leaves escrow 73, invoice duty 65, and reserve duty 10. The source requires non-negative custody, not liability backing. PROPOSED full backing: fund 112 for one partial, leaving 75; each further partial needs another fee allowance. No extra transfer or reserve-duty-return action exists in S12. From the 110-funded path, finish leaves 8 against reserve duty 10, blocking Close under reviewed V2 source. Record unaffected state and separate network requirements. [S12 actions; ERR-22]

**3. Inspect source conditions and local outcomes.** The proposed report separates preconditions, positional postconditions, proof obligations, transfers, fees, and financial duties. Show supplied premises and unsupported frame fields. The documented circuit profiles `TouchedOnly` and `NotBound` do not supply a complete authenticated frame or cap slot. The reviewed V2 consumer source nevertheless has signed gross/fee/net/liability limits, duty chaining, an exact thread custody snapshot, and zero-duty/zero-balance Close. A general multi-UTxO frame remains PROPOSED. Keep circuit and consumer responsibilities distinct. [S04 “Supported subset and refusals”; ERR-10]

**4. Prepare authorization and the exact statement.** A future supported route derives consumer requirements and the canonical owner constraints under its target/profile. The user reviews exact assets, recipients, gross-debit and fee limits, minimum net receipts, validity, nonce, and permitted partial completion. Required intent authorization must precede any statement/proof that commits to that intent. Final transaction signing is a later, distinct event. The concrete dependency order depends on the chosen consumer. S09's skeleton-to-intent-to-statement order is an unadopted candidate, not a generic API promise. [S03 “What provable intention means”; S09 §2.2]

**5. Produce and inspect proof instances.** A proof instance identifies relation, statement, verifier/key, setup class, and context provenance. A Strand includes that proof and its metadata. Before proof production, the UI says “requirement” or “planned proof.” It does not create a Strand from an arbitrary claim. A whole-execution instance may cover several specifications. A separate fee or liability proof requires an actual relation. The reviewed header has constant tag/family/predicate/shape/body-length fields and free hashed context bits; T3 and the BLAKE2b link remain OPEN. Claimant bytes bind at consumption in the source, while Issue does not compare them with the statement. Do not report this as compiled behavior or authenticated prover identity. [ERR-02/ERR-03/ERR-16]

**6. Assemble and construct.** A future Web assembler matches Strands to the identified consumer's requirements. Assembly is a local preparation stage. Today's documented package stops before a supported general Web/transaction path. On the reviewed issuance route, C2 structurally permits one claim per Issue transaction; settlement later consumes claims, requires every claimant’s signature, and burns their tokens. Counts are source readings, not measurements. Already-issued outputs lock min-ADA even if the Web is incomplete. A future builder names these stages, outputs, funding, and continuation. [ERR-04/ERR-15/ERR-25] Transaction edits invalidate affected preparation according to the specified binding contract. No synthetic transaction reference fills an unsupported stage.

**7. Sign, submit, and reconcile.** Recheck the candidate against canonical authority, fees, effects, and current state. Keep transaction signing and submission explicit. Claimant settlement signatures are distinct from the owner intents signed before proving. Observe inclusion, required finality, and complete effect readback separately. [ERR-25] Show the remaining invoice and reserve duties after success. Unknown settlement preserves the last observation and submission reference. Observation does not authorize a new spend. [S03 “What provable intention means”]

The proposed user summary reads: “Seller receives 35 CASH. Escrow pays 2 CASH application fee. Invoice duty remaining: 65 CASH. Buyer reserve duty: 10 CASH.” Expand CASH to its exact identifier. Show network fees, service fees, collateral exposure, locked funding, and authority separately when the target requires them. Unknown values remain preparation gaps.

## Journey B: specify an asset lifecycle

An issuer starts with exact asset identity and operations: issue, distribute, service, redeem, migrate, terminate. A proposed policy worksheet separates unissued capacity, actual supply, reserves, and financial duties. For an illustrative 300-unit issue under cap 1,000, the 700 units of capacity are not redemption debt.

Each operation names its source support, claim specification, actual relation, owner authority, target policy, and expected complete effects. The documented transfer/fee/liability surface does not establish a mint/burn lifecycle. Stop at specification work until those producers and consumers exist. A later issuer demonstration must inspect actual supply delta and recipient outputs, then reconcile servicing and carried duties. [S03 “What provable intention means”; Study 04 USE-02, PROPOSED]

The reserve lesson distinguishes non-negative custody of 23 A / 4 B from PROPOSED full backing of 25 A / 10 B. The units input is an arithmetic check, not a debit. S13 cannot discharge allocation_reserve, so that nonzero reserve duty blocks V2 Close. Its Boolean is not a separate governance decision. In reviewed V2 source, required signers consent to action inputs through the scenario; a distinct approver must be a required signer, advancer, or authenticated input. Retarget the proposed lint to missing approver coverage, not free prover choice. [ERR-09/ERR-21/ERR-24]

## Journey C: consume proofs in an application

An integrator starts with the actual consumer and target. The proposed handoff exposes required claim specifications, their authenticated policy source, supported relation mappings, and each concrete statement/proof identity. Unknown mappings remain visible. Concrete execution proofs, universal certificates, and authenticated ledger claims are distinct meanings, not interchangeable supported variants.

The builder can use an assembly report for diagnostics, but actual validator invocation and effects remain authoritative. The integration contract names which actor supplies, authenticates, checks, and consumes each artifact. Settlement recomputes the statement compared with the digest verified at issuance; it does not verify the proof again. Actual output allocation is the validator’s job. A profile chooses strict distinct outputs or an amount partition admitting coalescing. The reviewed strict rule covers one S2 script hash only. Cross-script/version coverage remains proposed. [ERR-08/ERR-12/ERR-23] A public evidence bundle records source/profile pins, descriptor, statement, independent expectations, scoped receipts, consumer/target references, and omissions. It excludes witness, private-key, and setup-secret material. [Study 07 DX07-P03/P06, PROPOSED]

## Provenance, lifecycle, and naming

Reference inputs are a PROPOSED path: the validator must read the datum and authenticate its origin. Ledger presence at application is a PREMISE, giving no exclusivity or historical truth. A validity interval does not date or authenticate the datum. [SEC06-P06 PROPOSED; ERR-11]

The reviewed network byte is catalogue-declared, not compared with the executing ledger. Do not show it as a network-authentication badge. Source claimant binding remains subject to T3 and the forgeable setup. Intent revocation (Revoke) advances seq and can orphan claims; competing settlement can do likewise. Existing reclaim is claimant-signed after expiry, not immediate. Key migration and earlier reclaim are PROPOSED. V2 catalogues are immutable. Distinguish thread reserve lovelace from a reserve duty liability, and both from program fees, network fees, collateral, and claim-output min-ADA. [ERR-03/ERR-06/ERR-07/ERR-14/ERR-28]

## Current source commands and future ergonomics

SOURCE-DOCUMENTED interfaces: `mori-k2 compile --source FILE --selector FILE` and `mori-k2 inspect --source FILE --request FILE --vk-digest HEX`. The README separately documents `mori-k2 setup`, `mori-k2 prove`, and `mori-k2 verify` with their required files. These are literal source interfaces, not commands executed for this package. [S04 “Build and ordinary reproduction,” “Proof API boundary”]

The documented flags use `--flag value` pairs. There is no separate help handler. `inspect` exit zero reports command completion and can accompany violated rows. Same-binary descriptor comparison is not an independent anchor. [S04 “Build and ordinary reproduction,” “Supported subset and refusals”]

| PROPOSED future CLI family | Corresponding PROPOSED SDK operation | Result |
| --- | --- | --- |
| `mori obligations inspect` | `inspectObligations(source, profile)` | Source-linked conditions, effects, duties, and gaps |
| `mori scenario compare` | `compareScenario(observed, independentExpected)` | Scoped comparison with provenance |
| `mori claim describe` | `describeClaim(specification)` | Reusable definition and relation mappings |
| `mori strand inspect` | `inspectStrand(proofInstance)` | Concrete statement, proof identity, scoped checks |
| `mori web requirements` | `readRequirements(consumer, context)` | Requirements plus provenance, otherwise gap |
| `mori web assemble` | `assembleWeb(requirements, strands)` | Local match report, never settlement |
| `mori transaction build` | `buildTransaction(assembly, effects)` | Candidate with explicit invoked validators |
| `mori transaction submit` / `observe` | `submitTransaction(candidate)` / `observeTransaction(reference)` | Distinct submission and ledger observations |

These names are interface proposals, not runnable examples or accepted signatures. Reports should share versioned human-readable and JSON views. Importing or inspecting an artifact must not silently trigger setup, proving, signing, or submission. A future convenience operation must expose those boundaries rather than hide them.

PROPOSED language ergonomics prioritize source spans, named conditions, explicit liability identity, positional-postcondition explanations, and typed unknowns in reports. Any new annotation or construct needs its own semantics and lowering. Do not invent supported syntax for claim decomposition or silently replace unsupported source behavior with host logic.

## Diagnostics, support, and evaluation

The proposed summary shows financial meaning, required proof status, and the first blocked boundary. Expand to provenance and exact identities. SEC06-P03/P08 remain PROPOSED presentation inputs, subject to their assumptions and evidence listed in DECISIONS-AND-DISSENT.md. The report provides no security verdict. Its V2 card shows catalogue program/source identities as visible and signed, the link to predicate/VK as CATALOGUE-AUTHOR ASSERTION, and VK-to-matrix as OPEN. Exact predicate-digest field coverage depends on identity.rs, unread by the security reviewer. [ERR-13]

Use distinct outcomes: unknown fact, unsupported operation, stale context, missing requirement, mismatched identity, local check rejection, proof-check failure, and settlement unknown. Each names its stage, public artifact, expected/actual value or missing premise, and next supported action. Do not invent internal guard attribution from a broad runtime failure. [S08 `/cases/1/validation/guardAttribution`]

Provide failure tutorials for wrong executable/profile, refused construct, violated-row receipt, missing evidence, unsettled submission, and duties remaining after payment. A PROPOSED DEMO ONLY evidence report uses ERR-18’s qualified layers: H ordinary host tests captured (K4H, S04, fresh actual-result review pending), N one local roundtrip, V one local C2 wrapper/seven cases, P none. Whole C2 Issue evaluation cost is not isolated verification or ledger ExUnits. [ERR-05/ERR-18]

Support attachments are user-selected public diagnostics. They do not require witness or key uploads. Track source admission, local result, proof production/check, assembly, authorization, and ledger observation independently.

Evaluate all three journeys with external developers and two contrasting new supported programs. Record comprehension, time, failures, and gaps. Clean installation, proof, authorization, target acceptance, and complete readback need separate evidence. No user study or execution occurred here.
