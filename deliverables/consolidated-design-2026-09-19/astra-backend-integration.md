# Supplemental proposal: next ZKIR and recursion integration contract

2026-09-19. Advisory supplement to `astra-kernel.md`. **User planning assumption:** comprehensive Midnight recursion becomes available in approximately six months, around March 2027. Use that horizon to plan interfaces and dependent work; it is not a claim of current support, a verified upstream commitment or a guaranteed date. All requirements/tests below are proposed and unexecuted.

Recommendation: design Moriarty's full native-history relation now, using the assumed upcoming native capability, while keeping delivery acceptance conditional on the exact shipped ZKIR/compiler/prover/ledger tuple. Do not preserve the older blanket rejection of DAG/per-transaction history as a permanent scope restriction. Equally, do not reinterpret a recursion instruction as automatic history compliance, unique consumption, privacy or settlement.

## Source evidence and its limits

Local retained intake: `evidence/moriarty-completion-program-2026-09-07/execution-intake-20260907/verifier-interface-intake.md`. It reports incompatible native-IVC and ledger-call statement/transcript/decider formats under historical pins, not current upstream impossibility. The September 11 architecture decision in the evidence packet supplies later historical recursion and ledger observations; its dates and pins remain attached.

Fresh read-only inspection of the pinned git object `midnight-zk@695351f1cdb3909affd1c89fef0a5eb3e9fa3ab7`, `aggregation/src/ivc/verifier.rs`, confirms that this verifier checks canonical `vk_repr`, the application `T::decider`, Poseidon-transcript proof preparation, full transcript consumption, accumulation with the instance accumulator and final pairing validation. Its context/key/parameters are crate-private. This is a source observation, not a reproduced proof or a claim about a future API.

Fresh read-only inspection of `midnight-ledger@a8ab82ba2124c36f92795c683e70bd888bc1d1fb`, `ledger/src/structure.rs` lines 410–490, confirms a historical compile-time branch that returns success when `proof-verifying` is disabled, operation-key selection, proof-version matching and an optional mock verification route. This does not allege that deployed Preview disables proof verification; it motivates exact binary configuration evidence.

The current local `deliverables/aeon-study-2026-09-19/PR17-APPLICABILITY.md` distinguishes canonical witness faithfulness from arbitrary shaped-witness statement soundness. It records witness-side WShape premises, concrete chip/encoding assumptions and an older instruction surface. The future backend must close or expose these obligations for its actual emitted instruction set. No Lean dependency follows.

Required status was refreshed: no pending transactions; internal implementation dispatch remains blocked by stale inputs and unavailable operational accounting. No builds, proofs, network requests or canonical edits were made for this supplement.

## Ownership: capabilities to request without assigning them to the wrong layer

| Owner | Required responsibility | Outside that owner's guarantee |
|---|---|---|
| ZKIR specification and constraint implementation | Typed statement/commitment encodings; exact instruction semantics; sound proof-verification relation, guard semantics, bounds and declared residual verification obligations | Truth of arbitrary external inputs, owner consent not encoded in constraints, finalized ledger effects |
| Moriarty/Compact compiler route | Preserve the supported source/Core relation, mandatory predicates, exact arithmetic, authority, effects, failure behavior and phase layout; emit artifact and dependency bindings | Ledger uniqueness or native verifier soundness assumed without evidence |
| Native prover and verifier libraries | Canonical serialization, supported proof/transcript/key formats, recursion and retained certificate verification, accumulator/decider lifecycle, resource reporting | Financial statement completeness or current ledger state merely from proof generation |
| Midnight ledger/node | Authoritative execution context, operation/verifier identity, mandatory final cryptographic checks, current reads, replay/resource uniqueness, phase execution, fee assessment and authenticated receipts | Foreign-chain finality, witness availability, global rollback |
| Moriarty application/library | Formal intention, legitimate origins, permitted lifecycle, liability/accounting invariants, recovery and signed evolution policy | An unstated business intention or external real-world truth |
| Federated kernel/adapter | Exact external bytes/effects, policy-bound signing, observations, retries and durable reservation/reconciliation under declared trust | Public deployment authority or ability to replace native mandatory proof checks |

## Required next-release interfaces and discriminating tests

### B1. Versioned statement ABI and authoritative context

Provide a documented typed statement ABI, canonical encoding and semantic domain separators. Bind network/chain genesis identity, contract/address and operation, program/property/Core profile, intent and authority domain, predecessor state/history, complete effects, phase boundaries, bounds and applicable time/expiry policy. Expose a precise map of fields supplied by the ledger versus fields committed by the application, and prove/check the link.

Fees need their own explicit binding design. Bind maximum authorized fee and fee asset in intent, then enforce assessed fees against that cap at the actual debit boundary. If exact fees are decided after proof generation, expose a verifiable ledger check rather than requiring a fictitious pre-known exact fee. Bind each guaranteed/fallible phase's permitted authority consumption and retained-effect policy. Network identity and deadlines cannot be trusted prover inputs merely because the proof commits to them.

**Tests:** reuse a valid proof on another network, contract, operation, intent domain, recipient, asset or program version; alter fee recipient/amount/asset; move an operation across a phase boundary. Each invalid context must reject at its named enforcement boundary. A legitimate fee change still within a signed permitted policy should remain possible when the chosen design allows it. Record which changes invalidate proofs and which trigger ledger checks.

Historical grounding: September 11 ledger-8 observation reports network ID, fees, TTL and state root absent from the call's binding input. This is a coverage question for the future ABI, not a demand that every item necessarily become a raw public input or a claim that transaction-level checks are absent.

### B2. Exact rejection and retained-effect contract

Expose distinct results for pre-inclusion rejection, guaranteed-phase failure, accepted partial success and accepted full success. Each accepted result must allow reconstruction of the complete actual effect frame: transfers, custody, minted/burned supply, fees, nonce/authority consumption and continuation changes. Moriarty must separately project those facts to liabilities and residual duties; a ledger receipt does not know business obligations automatically.

**Tests:** deliberately fail each mandatory condition before effects; fail a fallible step after an authorized guaranteed fee; fail after a permitted guaranteed state change; race stale reads. Assert exact effects and unchanged regions, not only a status code. Every accepted partial outcome must map to a source-permitted transition. A rejected local simulation must never be recorded as proof of zero effects from an already-submitted transaction. Verify crash recovery using retained receipts without resubmission.

### B3. Current reads, genesis and durable uniqueness

Offer authenticated read/absence semantics and compare-and-consume behavior sufficient to enforce one current workflow head or explicit resource identities. Bind the relevant read set, complete state domain and effect frame. Specify nullifier domain, uniqueness lifetime and migration behavior. Absence checks must prevent a write-only creation path from inventing a funded/approved base.

**Tests:** two individually valid successors spending one predecessor; fabricated genesis; stale authenticated root; private omitted debt/reservation; double-consumed join parent; replay across version/network; concurrent create at the same logical identity. Exactly the allowed successor set may commit. A recursively verified parent proof that remains reusable as evidence must not become reusable spending authority.

ZKIR needs authenticated membership/nonmembership and equality primitives or sound compositions thereof; the ledger supplies currentness and consumption. Neither one alone establishes the combined property.

### B4. Recursive certificates with complete finalization

Define `Certificate` separately from `ContractCallProof`. Each format identifies relation/VK, transcript, canonical public statement, proof bytes, accumulator data if any, application decider semantics, proof/version and parameters. Expose verification levels explicitly: decoded, recursive relation processed, accumulated, fully cryptographically verified and ledger accepted. Only a complete accepted verification path may justify effects.

The native API must support bounded predecessor sets, legitimate base cases, well-founded ancestry and application state continuity for the required IVC/DAG profile. State which of these are native mechanics and which must be proved by Moriarty's local relation. If recursive verification defers pairings or decisions, require propagation of every residual verification obligation and unconditional ledger finalization before accepting protected effects. A guard disabling an optional check must not disable a mandatory predecessor check. Final cryptographic verification and network consensus finality are different meanings of “final.”

**Tests:** valid one-parent continuation and two-parent join; invalid but parseable inner proof; wrong inner public instance; changed relation key; malformed accumulator; omitted application decider; valid outer proof with invalid deferred pairing; zero/false guard bypass; cyclic/fabricated origin; extra trailing bytes. Test all final checks through the real ledger path with proof-verifying enabled, not only a host API. Observe exact rejected/retained effects under the documented ordering.

For ordinary predecessor contract-call proofs, require either a compatible native recursive verifier or an explicitly specified certificate bridge proving the intended ledger acceptance statement. A byte/tag conversion between formats does not establish that bridge. Comprehensive recursion support is the planning assumption; the exact supported format remains an integration requirement.

### B5. Retained-proof export, resume and resource costs

Provide stable, canonical export/import of keys, public instances, commitments, accumulators, parameters and proof bytes with complete length/EOF checks. Resume must specify necessary private witness/state, not imply that a public proof reconstructs it. Bound parents, statement size and recursive work; distinguish host time/memory, circuit rows, proof size, ledger verification work and fee charges, including deferred checks.

**Tests:** independent fresh-process verification of retained proof; resumed versus uninterrupted continuation with matching declared semantics; truncation/appended bytes; version/parameter mismatch; maximum supported parent count; just-over-bound rejection; accurate accounting of extra pairings/inputs. Resource limits are measured properties of the pinned artifact, not inferred from “constant-size proof.”

### B6. Verifier-key and semantic migration

Bind operation keys and relation identity to the intent's permitted evolution policy. Expose a migration path with authenticated old/new state, replay/nullifier continuity, continuation schema, residual liabilities, recovery rights and privacy commitments. Distinguish key rotation preserving semantics from changing semantics; neither interface compatibility nor new operator approval proves preservation.

**Tests:** unauthorized weaker key, malicious epoch rotation, valid new key with altered beneficiary/policy, migration resetting consumed nonces, concurrent old/new acceptance, stranded pending escrow. Accept an authorized preserving migration and reject the incompatible cases. If migration cannot preserve consumption state, use an explicitly authorized fresh domain that rejects old grants while retaining old outstanding duties through a defined recovery path; do not erase them.

### B7. Foreign evidence and proof adapters

Define typed imported evidence by the claim it proves: computation relation, threshold authorization, TEE execution attestation, authenticated foreign state, application success or policy-final settlement. Require domain, subject, effect, stage, epoch, verifier and freshness binding. Ordinary signed observations remain assertions under named trust assumptions even when verified inside a recursive native certificate.

A foreign cryptographic proof needs a selected verification or certified translation relation plus its source-chain consensus/finality and data-availability premises. Do not promise arbitrary proof-system interoperability merely because native recursion is comprehensive. Start with explicit supported profiles; unsupported evidence fails rather than becoming an unchecked host Boolean. ZKIR may supply required cryptographic operations; adapter semantics determine what they mean economically.

**Tests:** proof for X paired with signed transaction Y; valid proof at wrong chain/root/epoch; stale or revoked attestation; source inclusion without required finality; transport acknowledgment without application success; weaker signed fallback absent; foreign finalized delivery followed by timeout refund. A valid alternative supported evidence path should succeed if the owner's policy permits it.

### B8. Private continuation and multi-party proving

Publish the observation model and handoff interface: public inputs, private witness fields, permitted recipients, access/prove/spend/recover authorities, encryption/commitment identities, witness availability and recovery. Private predecessor data needed by the next transition must be supplied through authorized transfer, joint proving/MPC or a different statement avoiding that dependency. Recursion alone cannot reveal an unavailable private witness safely or make it unnecessary.

**Tests:** separate-principal private successor and join under real access isolation; denied access to unauthorized predecessor fields; corrupt/withheld handoff; available proof but unavailable required witness; public metadata/error/cost leakage against declared policy; unauthorized witness disclosure; privacy-preserving state-completeness proof. Report conditional progress when a necessary witness holder disappears. No spending authority should arise solely from possession of proving or viewing material.

## Planning sequence and evidence handoff

Before March 2027, specify B1–B8 and implement only currently supported bounded paths: source/Core relation, phase and authority contracts, exact encodings, failure oracle, source-level history invariants and native compatibility harnesses. Keep MC03/04/05/06 obligations open. Use the assumed future capability to choose extensible interfaces instead of permanently truncating product scope.

At availability, freeze a compatible release tuple: ZKIR semantic revision and concrete instruction set, compiler/lowerer, native proof library, circuits/keys/SRS parameters, ledger/node build/features and deployed verifier identity. Run B1–B8 against that tuple, starting with one native certificate and two-parent composition before expensive financial campaigns. Requalify previous correspondence claims affected by changed constraints or phases.

The acceptance dossier should contain each exact requirement, owner, source/API pin, statement schema, source-to-target mapping, assumptions, positive example, single-property negative controls, complete observed effects, cost data, retained independently verifiable artifacts and unresolved limitations. Record native final proof verification separately from local ledger acceptance, Preview acceptance and consensus finality.

Traceability: B1/B2 → MPLR-014/017/018/022/023/024; B3 → 011/027/029/032; B4/B5 → 009/020/021/027; B6 → 008/028; B7 → 006/010/026/030/033; B8 → 013/027/029/030. All preserve public permissionless deployment and the four mandatory claims. The new release enables implementation; only evidence for the actual acceptance relation completes it.
