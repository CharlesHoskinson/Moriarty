# doctorwatson<UTXO>: Groth16 and PCD evidence design

Advisory design draft. Machine slug: `doctorwatson-utxo`. This contribution proposes reporting interfaces and ordinary QA only. It is not a cryptographic design, security approval, execution admission or product acceptance. The broader Preprod goal remains paused; native authority remains zero. No implementation or subject test was executed. The companion `groth16-design.json` carries the structured proposal and inspected-source pins.

The useful increment is a small evidence reader that prevents a real but limited proof result from being mistaken for a property certificate, authenticated ledger claim or completed settlement. After the bounded reporting bootstrap, work should advance the actual Rust → Cardano artifact → claim issuance → consumption → composition → settlement path. DoctorWatson reports dependencies and first blocked boundaries; it does not schedule, launch, retry or grant permission for that work.

## Evidence already available

The baseline is Moriarty `1721457439dbd6d988d00c6c86d1cf648caf2a0c`. Sherlock is inspected at `0aecf98455dd9abc1635801166151431cba309f7`, `models/sherlock/qa/check.py:682–914`. Its reader separates implementation, validation, acceptance and currentness and expressly does not authenticate review or decide semantics. Its catalogue paths and 35/25/68 membership are Sherlock-specific; its permissive absolute-path handling must not be inherited by this public-only UTXO reader. [S1]

The root design retains separate inventories: 119 legacy UTXO clauses and 238 scenarios across 20 families; 11 first-contract clauses and 16 scenarios; 18 Midnight PCD clauses and 38 scenarios. All 292 named scenarios remain attached to their original clauses. Current Rust/Preprod applicability needs explicit reconciliation. A source/5, source/6, Preview or Midnight receipt cannot acquire a different profile by changing its label. The baseline is an extraction aid, not semantic review or current acceptance. [S2–S4]

| Available artifact | Reuse in the report | Boundary retained |
| --- | --- | --- |
| Reviewed K6 invoice roundtrip and root adoption | Captured local observation: one honest compiled Rust invoice proof, source-recompiled and export-only verification, eleven specifically classified refusals | Class-0 `single-party-test-setup/1`; fabricated caller context; no universal certificate, authenticated ledger truth or Aiken C2 acceptance. K6 originally lacked public VK bytes; the later K7 public export resolves that input-publication gap without a new pairing run or consumer-established VK-to-matrix link. [S5–S7, S20–S21] |
| K7 public verifier-record adoption | Byte-exact public 3,926-byte setup-record export, including the 488-byte VK; public file SHA-256 matches the adopted record in this inspection | Export prerequisite closed; no new cryptographic validation or public pairing re-execution. Class-0 setup, fabricated context and unproved VK/matrix link retained. [S20–S21] |
| Four-piece actual adoption | 147/147 frozen expected unit outcomes, applied sizes and tool-derived parameter DAG for the exact historical candidate | No property tests, C2 pairing, VI signatures, compiled-wrapper execution, independent script-hash check, ledger ExUnits or Preprod settlement. Preserve first 129/147 result and its 18 failures. [S8–S9] |
| R4 renderer attribution | Exact renderer occurrence and reviewed call-site attribution only | Full builder is an unrun, unadopted draft; upstream preconditions and full-builder contract await owner review. [S10] |
| D7 result and checker correction checkpoint | Observed missing-implementation baseline: 14 methods, 1 provenance/control/premise pass, 13 exact failures, 0 errors; independent actual review pending | No checker executed and no hash behavior established. Corrected uppercase fixture is unexecuted; preserve original 19-test/18-pass/1-failure history reported in the task handoff, whose original payload was not inspected here. [S11; handoff] |
| Midnight PCD clauses | Exact retained obligations, named scenarios, owner questions and profile-adapter dependencies | No translation of `VerifyProof`, `InnerProof`, ledger generations or deployed-operation-key rules into Cardano acceptance. Recursion, private handoff and multi-parent obligations survive. [S12–S15] |

These are scoped source observations from published records. This design did not reproduce the runs, audit every payload, validate a proof, authenticate a provider identity or establish that an older capsule describes all later work. K7 is the specific later adopted public export that resolves K6’s input-publication gap. The earlier K6 capsule remains unchanged; no global public-VK blocker is reopened from its dated prose.

## Proposed interface and record types

Use the root proposal's `./scripts/doctorwatson-utxo.sh` and canonical evidence record. Add no second executable framework. A pure reader consumes explicit public repository-relative references, inventory/profile mappings and declared owner decisions, and emits a report. `--requirement ID`, `--next` and `--gate ID` remain read-only selectors. Commands in receipts are inert text. Default integrity success means the declarations are well formed; a separate closure result reports only current, applicable owner declarations.

One `EvidenceItem` contains the following fields. The companion JSON specifies field names and structured reference types. This is an interface proposal, not an adopted schema.

| Field | Required meaning |
| --- | --- |
| `id`, `recordType`, `claimScope` | Stable observation identity; property certificate, concrete execution proof, claim issuance, claim consumption or settlement observation; exact predicate and admitted domain |
| `requirementRefs`, `scenarioRefs`, `ownerContractRef` | Original inventory, canonical clause/scenario membership and the reviewed owner contract defining the claim; additive refinements retain parent lineage |
| `identities` | Source language/semantics, program/source/compiler/relation/statement, verifier and script identities, anchor, target and bounds; unknown facts are explicit gaps |
| `trustProfile` | Named setup class, public provenance record, VK identity and public availability, owner-reviewed assumptions and unresolved links; no PK, SRS, witness or recovery material |
| `observation` | Actual consumer, original command text, attempt, timestamp, input/output public references, exit, observation class, exact failure layer and first divergence |
| `expectation` | Independently frozen outcome reference, oracle author/consumer independence and complete relevant comparison; an evaluator under test is not its own independent oracle |
| `dependencies` | Typed edges to every declared consumed artifact, profile, bounds, oracle, review and prior claim; each edge records why it matters |
| `implementation`, `validation`, `acceptance`, `currentness` | Four distinct axes; owner acceptance and review/adoption references are declarations, while currentness is computed from declared dependencies |
| `gaps`, `nextDiscriminatingAction` | Missing fact/behavior, responsible owner, observable completion condition and authority state; no command execution |

The identity tuple records both declared and measured identities where the existing contract distinguishes them. Use typed digest labels (`sha256`, `blake2b-256`, owner-defined script hash) and encoding/profile references; matching text in unrelated digest fields does not establish equality. Record source identity separately from compiled bytes and running binary identity. For Cardano artifacts, retain each applied script and its parameters, unapplied source/build identity, observed hash and independent hash-check evidence. Preserve the reviewed DAG `C2 → VI(c2) → VS3(c2, vi) → S2′(c2, vi, vs1, vs2, vs3, vs4)`. [S8–S9, S16]

Anchor fields describe the actual acceptance locus: chain/network/genesis identity, contract or policy identity, reference-script output when applicable, verification-key binding source, predecessor output reference and observed block/readback identity. A caller-declared context uses `authentication: caller-declared`; it cannot fill `target-observed`. Unknown target facts remain unknown. The report neither fetches them nor infers them from an address or network label.

`TrustProfile` records `classification`, `ownerContractRef`, `publicProvenanceRefs`, `verificationRecordRef`, `verificationKeyDigest`, `publicAvailability`, `relationLinkEvidenceRefs`, `assumptions` and `unresolved`. For the captured K6 specimen it must preserve the owner's Class-0 setup qualification and absent consumer-checked VK/matrix linkage. Public provenance may later reference an owner-reviewed ceremony transcript or public verification record, but DoctorWatson does not decide ceremony sufficiency or setup trust. Public digest availability alone is not public verification reproducibility. [S5–S7]

`Bounds` records the supported numeric/source domain, input/output/effect/asset/obligation counts, history/segment/fan-in/fan-out limits where applicable, and measured resource limits with units, environment and evidence. Separate logical work, constraint count, proof CPU/memory, script size, ledger CPU/memory, transaction size, fees, minimum ADA and collateral. Do not relabel K6 nominal charge as measured transitive CPU, nor the four-piece 14,336 B working target as a protocol limit. No numeric bound is invented here. Existing Midnight bounds remain Midnight-specific; a Cardano bound requires its own owner contract and evidence. [S5, S8, S12, S14, S18]

## Five evidence types and their seams

1. **Universal property certificate.** Reference the exact quantified property, program/semantics identity, domain, assumptions, certificate bytes, checking consumer and owner-reviewed correspondence. A collection of positive executions, R1CS satisfaction, a proof of one selected action or an unverified certificate hash cannot populate this type as validated. A current Rust/Cardano certificate contract and its discharge locus remain owner-definition gaps. Retained Midnight deployment-audit wording cannot introduce a public developer approval requirement. [S15, S19]
2. **Concrete execution proof.** Reference the exact statement, proof export, public inputs, relation descriptor, verifier/VK and trust profile. Record source-recompiled and export-only verification as distinct observations. Keep declared input context distinct from authenticated state; one local proof supports only its exact specimen and reviewed domain. No automatic edge converts this item into a universal certificate or ledger claim. [S5–S7]
3. **Authenticated claim-output issuance.** Reference the owner-defined C2 issue operation, exact applied script/DAG, concrete proof observation, claim payload identity, issued output/token identity and public target readback. Separate proof verification, issuance execution and observed ledger issuance. The substantive issue/burn/release semantics belong to the C2/security owner. The report requires that contract by reference and never substitutes a proposed cryptographic check. [S16]
4. **Claim-output consumption.** Name the exact issued claim output and consuming transaction, actual consumed/reference inputs, owner-defined release/burn action, selected settlement instance and resulting outputs. Reading or referencing a claim is distinct from consuming it. A proof receipt alone supplies no consumed UTXO observation. Preserve the independently specified complete effects and remaining duties. [S17–S18]
5. **Settlement observation.** Link issuance and consumption evidence to the actual source transition and full financial comparison. Distinguish submission, provider evaluation, phase-one/phase-two refusal, inclusion, readback and finality. A timeout means unknown settlement unless later separately qualified observations resolve it; callback success is not native acceptance. [S18]

Sequential composition uses separate ordered settlement observations connected by the actual successor/predecessor identity. A claim awaiting consumption remains an intermediate state. Multiple sequentially successful transactions cannot establish one atomic multi-input settlement. An atomic record names one transaction, every required consumed input, all successors/effects and the joint observed outcome under the reviewed composition contract. Preserve FV-02: shared VS1/VS2/VS4 hashes restrict atomic composition across distinct S2′ deployments. Do not design around that restriction in the evidence schema. Unsupported composition remains open and identifies its owner. [S8–S9, S17]

Recursion, private handoff and multi-parent history have independent coverage rows even if a scoped public ledger-induced path is delivered. Record the mode (`ledger-induced`, `recursive-certificate`, `private-handoff`, `multi-parent`), exact applicable contract, predecessor evidence identities, bounded domain and observation references. Public reports contain only authorized public attestations of private handoff; they do not read predecessor secrets or private openings. Missing callable interfaces or disclosure/compatibility definitions become named owner gaps. No mode aliases another and no “PCD complete” boolean replaces the retained obligations. [S12–S15, S17]

## Phase-specific reporting gates

All gates below are proposed reporting predicates. They do not authorize the associated subject work. A gate can remain blocked while unrelated ordinary work continues.

| Gate | Observable evidence needed to report the scoped owner declaration | Current disposition |
| --- | --- | --- |
| G0 Inventory and applicability | Exact clause/scenario membership and explicit original-profile → claimed-profile mapping, with owner disposition | Inventories extracted; current Rust/Preprod inheritance unestablished |
| G1 Artifact identity | Actual consumed source/dependencies, applied scripts, parameter DAG, independent hash result, current source/result reviews | Four-piece tool-derived historical measurements reusable; R4 full builder and corrected hash checker unadopted/unexecuted |
| G2 Local proof | Public statement/proof identities, trust profile, actual verification observations and exact reviewed scope | K6 narrow captured roundtrip adopted; K7 public inputs exported; fresh public pairing re-execution is not observed |
| G3 Compiled Cardano claim issuance | Existing C2 contract, actual applied-wrapper evaluation including real proof consumer, identity correspondence and scoped result review | Unestablished in inspected evidence |
| G4 Claim consumption | Existing release/burn/settlement contract, actual compiled spend consumer, complete input/output/effect comparison and applicable authority observation | Unestablished in inspected evidence |
| G5 Concrete transaction fit | Same intended body and scripts, genuine authorized intent evidence, pinned target parameters, measured size/fees/ExUnits/collateral and all complete financial outcomes | Unestablished; no qualified target tuple here |
| G6 Target settlement | Actual target identity, persisted exact body/transaction identity, ledger observations, consumed inputs, full readback and named confirmation/finality evidence | No Preprod completion; broader goal paused |
| G7 Composition and broader coverage | Separate sequential and atomic multi-input results, lineage/complete effects; separately retained recursion/private/multi-parent/property-certificate rows | Open; FV-02 restriction retained |

`G2` does not satisfy `G3`; `G3` does not satisfy `G6`; successful `G6` for one program does not close `G7` or the whole language. Required universal-property and claim-family obligations cannot be skipped to describe G6 as full product acceptance. A bounded testnet experiment, if later authorized, must be labelled by its actual remaining gaps. [S17, S19]

## Minimal real path to a scoped testnet result

This dependency sequence is a proposal for later work, not a resume instruction. Each step requires the applicable existing authority and owner review; no command is prescribed here.

1. Reconcile the current Rust/Cardano profile and exact bounded target claim. Produce a public evidence mapping from retained clauses to current refinements. Reuse the already published K7 public verification record and adoption for input availability; retain its trust and relation-link limits. Do not rerun K6 or repeat the K7 export solely to refresh a report.
2. Complete current review of the actual hash-checker correction and complete builder candidate; then obtain separately admitted results from the real generated-artifact consumer. D7's missing-implementation RED and renderer attribution remain limited inputs to this work. Freeze the concrete expected artifact and script/DAG identities.
3. Establish the unchanged intended C2 verification consumer and compiled applied wrappers with genuine hashes, exact statement encoding and real proof input. Require source/result review against the existing owner contracts. A unit predicate result cannot replace wrapper evidence.
4. Demonstrate actual claim issuance and release/consumption through the intended compiled spend path; integrate genuine intent/signature evidence (FX1) and complete financial outcomes. Compare the same intended transaction shape across proof, script and settlement consumers.
5. Measure a concrete Conway transaction against pinned target parameters, including whole-transaction size, fees, ExUnits, minimum ADA and collateral/failure effects. Per-script size and host CEK observations alone cannot establish fit.
6. Only after the paused goal is resumed and target execution is separately admitted, observe actual Preprod issuance, consumption and settlement with exact input/output readback and confirmation evidence. Then extend to the separately defined sequential and atomic multi-input scenarios. Keep unsupported cross-deployment composition and all wider PCD/private/general-language obligations visible.

The useful parallel preparation is disjoint: a nonsecurity implementer can prepare the reader and profile mappings; compiler/artifact owners can prepare current source-to-artifact correspondence; designated security owners can resolve setup, C2, signatures and composition contracts; ledger/finance owners can freeze independently expected complete outcomes and target receipt fields. The integrator joins their exact published interfaces. No new dispatcher, campaign, provider polling or runtime is needed. Shared schema changes go through the single integrator.

## Ordinary positive and negative QA proposals

These are specified-only tests of reporting behavior using synthetic declarations or already public frozen records. They are not new hostile cryptographic tests and do not execute proof, VM, wallet or ledger consumers. Subject security controls remain the designated owner's existing scenarios.

| ID | Positive reporting fixture | Negative reporting fixture and expected report |
| --- | --- | --- |
| Q01 | Separate current certificate, proof and ledger rows retain their own owners and scopes | Label one K6 proof as universal or ledger-authenticated: incompatible evidence class; requested closure remains open |
| Q02 | All original scenario names survive in their proper inventory | Remove a required scenario but preserve total count with a duplicate: inventory integrity failure |
| Q03 | Legacy Preview result stays visible as historical legacy evidence | Relabel it Rust/Preprod without approved mapping: applicability unestablished |
| Q04 | Public VK reference, its digest and owner-qualified provenance are distinct | Digest-only capsule claims public pairing reproducibility or Class-0 becomes ceremony-backed: unsupported declaration |
| Q05 | Unchanged consumed dependencies preserve an owner declaration's computed currentness | Change semantics, script parameter, VK, bounds or oracle pin: all transitively affected declarations become non-current; history retained |
| Q06 | Adopted result references the exact candidate and actual-result review | Source review or renderer attribution offered as full-builder/result acceptance: missing scoped review |
| Q07 | Issued, referenced and consumed claim outputs are distinct events | Claim reference input labelled consumption, or proof verification labelled issuance: missing observation of required event |
| Q08 | Sequential settlements have ordered references; atomic settlement has one joint body and complete input set | Combine separate transaction receipts into atomic closure or omit one required input: incomplete atomic evidence |
| Q09 | Proof, C2, VI and S2′ identities match the declared reviewed DAG | Swap a script identity or omit FV-02 restriction: inconsistent dependency/unsupported composition declaration |
| Q10 | Preserve D7 1/13/0 result and review-pending state | Convert expected missing-implementation failures into checker passes: invalid scope promotion; keep original result |
| Q11 | Separate nominal charge, observed wall and unknown transitive CPU; separate script bytes and transaction costs | Present K6 nominal CPU or 14,336 B as measured proof CPU or a protocol limit: mismatched metric/provenance |
| Q12 | Submission, refusal, inclusion, readback and finality remain distinct | Callback success, timeout or provider evaluation labelled settled: missing ledger/finality evidence |
| Q13 | Full financial comparison includes gross debit, fees, net receipts, frame, liabilities and residual duties | Omit a required effect or duty field while retaining a pass summary: incomplete comparison |
| Q14 | Recursion, private handoff and multi-parent rows stay open beside a public-only success | Collapse those rows into a single PCD pass or delete an unsupported family: inventory/claim-scope failure |
| Q15 | Complete all-open inventory yields integrity success and selected closure failure | Infer product success from integrity exit 0 or a synthetic reporting fixture: reject closure declaration |
| Q16 | Explicit allowlisted public relative file refs resolve without executing content | Absolute/traversal/symlink-escape/private refs or command strings offered as inputs: report invalid reference and do not follow/execute |

## Owner decisions and unresolved definitions

The nonsecurity reader owner implements membership, type checking, public-path confinement, pin comparison, dependency propagation and honest summaries. That owner does not certify mathematical claims or authenticate reviews from file hashes. The integrator owns shared schema/profile mappings and resolves source conflict without rewriting historical receipts.

The designated cryptographic/security owner must supply or confirm exact current contracts for: universal-property certification and discharge; setup trust and public provenance; VK/relation/statement/anchor correspondence; C2 claim issue/burn/release; genuine authority/signature integration; and permitted sequential/atomic/recursive/private/multi-parent composition. The applicable AGENTS routing permits Opus 5.5 or Opus 4.8 high for that work. This Astra contribution supplies no substitute security judgment.

Compiler/semantics owners define supported grammar, numeric semantics, complete effect projection and exact consumer correspondence. Ledger owners define the current target tuple, confirmation/finality contract and phase observations. Finance owners freeze independent complete outcomes. Existing limits, failed runs and counters stay with their original attempts. Internal reviews are maintainer evidence controls, never public developer compile/prove/deploy licenses. [S19]

Missing definitions are explicit blockers, not invented defaults: current Rust/Cardano universal-certificate contract; current-profile mapping for retained mandatory claims; owner disposition for setup trust and VK/relation linkage beyond the already adopted K7 public export; complete authenticated frame/context bridge; qualified Preprod identity and parameter observation; exact finality rule; current claim-output lifecycle and its target evidence contract; and permitted cross-deployment atomic composition under FV-02. Some existing contracts may already define parts of these; the next owner task is to name and pin them, not assume they are absent from the entire repository.

## EARS update and remaining-roadmap audit after the tool exists

The user's later steering requires the built tool to support an update of all EARS and an audit of the remaining roadmap to completion. Keep the first implementation bounded. Once its actual ordinary behavior and review exist, reconcile every retained clause and scenario against exact current evidence, preserving the three inventories and additive current-profile refinements. The audit itself is required work; it is not a declaration that the roadmap has already completed.

For each atomic roadmap exit, enumerate its profile, required proof classes, setup assumptions, ledger/target observations, owner contracts, complete scenario set and dependency closure. Map U0–U7, M1–M17 and every retained MC/SP/G exit as separate obligations where applicable; identify unmapped or uninspected exits instead of publishing a whole-language percentage. Retain all 20 UTXO families, ACTUS 277 fixtures/18 executable types/32 taxonomy dispositions/DS01–07, DeFi 72 rows/DA01–24/held-outs/intents/regressions, and recursion/private/multi-parent/property-certificate obligations. This contribution does not claim to have read all those source predicates.

Adopt K6 and K7 only at their exact scopes; show the obsolete K6 publication gap as resolved by K7 and the remaining test-setup/context/linkage gaps as open. Preserve old failures and explicit supersession edges. A missing source or owner definition is an audit finding, not an assumed refusal or implied completion. Emit the next real consumer check and owner for every open exit; classify unavailable runtime/resource/target authority separately. Fresh security judgments go to the designated owner. The audit creates no scheduler, grants no native authority and does not silently resume the paused Preprod goal.

## Source and read limits

`groth16-design.json` lists S1–S21 with commit, path, SHA-256 and inspected line spans. S2–S4 are mutable root design inputs inspected during this task, not newly committed normative requirements. Pinned baseline repository documents supply normative clauses; selected evidence README/adoption/review files supply recorded observations. Historical absolute pointers were read only as inert text and were not followed. No private captures, auth/config/env files, wallet material, PK/SRS/cache data or prohibited historical directories were read. No web retrieval was needed.

The four retained Midnight PCD specs were read fully. The large baseline was parsed for structure and selected clauses/scenarios, not semantically reviewed end to end. Only Sherlock's evidence-reader function was inspected. Evidence capsule payloads, validator implementation, circuit code, full security contracts and archived execution scripts were not audited. Product/security acceptance therefore remains with the existing owners and exact future evidence.
