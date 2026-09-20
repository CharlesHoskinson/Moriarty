# Product-scope audit: permissionless Moriarty and provable intent

Date: 2026-09-19. Read-only source/document audit of `/home/charl/Moriarty-aeon-study`, HEAD `1896217a28553e0254b0ba319422ac4c0b39ac0d`. All locators below are relative to that checkout unless stated otherwise. This report changes no product code, roadmap or vault page. Earlier Aeon consensus v2 is historical and superseded by the user's explicit clarification.

## Controlling interpretation

**User requirement:** Moriarty is a permissionless programming language for all Midnight DeFi developers. Anyone can author, compile, prove and deploy supported programs without a Moriarty administrator, reviewer panel, Foreman account or campaign permit. Programs and transactions carry actual correctness evidence connecting execution to declared developer properties and authenticated user intent.

Permissionless deployment does not authorize spending another participant's assets, weakening the proof relation, choosing a fake verifier, exceeding finite bounds or erasing liabilities. Those are semantic/protocol failures. A developer's application can implement participant permissions or governance; that does not grant the Moriarty project universal power to approve that application's deployment.

**Principal finding:** the source language and earlier intent research contain useful foundations, but an active SDK specification turns a language into a centrally admitted multichain routing service. Roadmap/OpenSpec then mix product predicates with agent campaign and release-review prerequisites. The new direction requires correcting this architecture and its normative documents, not merely changing “gate” to another word.

## Scope and observed status

Loaded `moriarty-dev:develop` and read `AGENTS.md`. The required read-only `status --json` reports stale SP01 bindings, missing accounting and unresolved campaign history; no pending transactions. These are internal campaign conditions, not evidence that developers lack permission to compile or deploy programs. The latest user direction overrides instructions to preserve every existing gate indiscriminately.

Inspected README, ROADMAP, completion program, relevant MC04/MC05/MC08 acceptance specifications, SP02/SP08/SP09 task contracts, current financial source /5 specification and implementation, outcome-intent prototype and research, and the 2026-09-11 SDK interface plus its reconciliation. There is no `deliverables/defi-kernel-sdk-interface*` path in this checkout; the actual maintained design is `docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md`, linked as normative by `wiki/defi-kernel-sdk-interface.md:27`. Multichain research resides in `deliverables/defi-kernel-multichain-2026-09-11/` and category studies in `deliverables/defi-interface-research-2026-09-12/`.

This is a cross-layer design audit, not exhaustive execution or a proof of absence of every hidden admission check. No suite or network experiment was run. Planned `experiments/moriarty-acceptance` and `experiments/moriarty-ledger-adapter` directories referenced by SP09 do not exist at this HEAD; their commands cannot be presented as implemented product interfaces.

## Useful design and actual implementation

**Existing research worth retaining.** `docs/research/2026-09-06-intents-report-integration.md:32` separates intent, permission, plan, execution and receipt. Lines 34–40 retain prefix safety, hard constraints versus ranking, affine authority, residual duties, anti-vacuity, domain-qualified assets, canonical signing display and mandatory proof classes. Lines 49–80 define distinct `Complete` and `Progress` judgments and connect predecessor compliance to authority consumption. This is substantially closer to the user's goal than the managed-service SDK framing.

**Existing implemented prototype, not product proof.** `experiments/moriarty-developer-mock/src/language/outcome.ts:7` defines signed intent independently of plan; :13 names the four required claims; :142–180 checks complete balance effects, prefix solvency, recipients, gross debit, fees and net credits. `outcome-runtime.ts:180–186` jointly commits local state and nonce usage. Critically, :188–190 explicitly returns unavailable for all four real proof claims. This is honest local execution evidence, not deployed proof enforcement.

**Existing source authoring is broader than hardcoded demo selection.** `experiments/moriarty-language/src/successor/financial-agreement-source-v5.ts:43–96` accepts source, elaborates, checks and evaluates a selected source action; schema is derived from source (:55). No panel approval, campaign ID or Foreman account is requested at that API. `spec/successor/financial-agreement-source-v5.md:3–11` documents multiple actions and protected Transfer/Repay/Originate/Accrue; :49–62 documents atomic postcondition evaluation and work accounting. Line 64 explicitly excludes proof construction and public submission. This is an actual reusable local language slice with substantial missing end-to-end work, not a universal DeFi implementation.

## Findings and precise repairs

### F01 — Critical: governed primitive deployment becomes a language access restriction

**Observation:** SDK design :470 requires every foreign argument record's adapter to be admitted; :489 says new primitives require “a governed deployment change”; :552 forbids runtime registration; reconciliation `2026-09-12-defi-kernel-sdk-interface-reconciliation.md:12` repeats governance as the resolution. The false dichotomy is that accepting new checked code is equivalent to an unrestricted foreign call.

**Repair:** separate immutable/versioned language semantics from permissionless program deployment. A new user program composed of supported constructs needs technical proof validation, not registry approval. New semantics do require implementation and a sound proof/lowering definition, but no project committee shall approve each deployed program. If an optional adapter service maintains its own catalog, explicitly scope its admission to that service; users remain free to deploy other correctly proved applications. Define a proof-checked extension/module path separately if proposed; do not promise arbitrary FFI safety.

### F02 — Critical: product identity is narrowed to a managed multichain outcome service

**Observation:** SDK :12/:20 hides chains and signing schemes from application developers; :24 promises the kernel reaches outcomes on whatever chains are needed; :36 excludes concrete asset resolution from signed intent; :51 hides mechanisms. `wiki/defi-kernel-sdk-interface.md:27–55` presents this as the normative Moriarty interface. Meanwhile README :5/:15 describes a Midnight language and :24 says applications do not connect to the DeFi study as a runtime service. These are incompatible primary product descriptions.

**Repair:** establish the language/compiler/prover/ledger SDK as the product contract. Reclassify the multichain account/router design as an optional application or research proposal written using Moriarty. Preserve useful constraints and failure models, but remove its identity service, committee, tariff, royalties and routing mandate as prerequisites for using the language. Developers need exact semantic/proof interfaces; ordinary application UI may offer simpler views without hiding signed meaning.

### F03 — High: a symbolic-asset UI conflicts with canonical intent fidelity

**Observation:** SDK :36 says an intent names symbolic assets and concrete chain/asset resolution is not signed; R2b design :31 and existing `outcome.ts:5` bind structured asset identity. SDK :38 promises conversions have evidence but does not make that a substitute for the user's agreement on permitted substitutions.

**Repair:** preserve exact asset identity or an explicit signed equivalence/conversion predicate and its assumptions. Symbolic display is optional. The proof must establish that selected assets and conversions satisfy those signed constraints. Display derives from the canonical signed object; the router cannot silently choose economically different same-name assets.

### F04 — High: “fixed trusted policy” lacks an authority boundary

**Observation:** MC05 `specs/mc05-mandatory-claim-acceptance/spec.md:4` requires a fixed trusted policy; :11 rejects arbitrary verifiers; :87–89 revokes specifications/keys. SP09 :46 requires certificate “admission into trusted deployment policy.” ROADMAP :123 and :136 similarly refer to trusted policy and verifier activation without distinguishing per-contract commitments from global administrative approval.

**Repair:** state who binds each policy: deployed contract/program semantics, immutable protocol verification rules and authorized participants. A prover cannot choose a verifier that accepts an unrelated statement. A developer can deploy a new contract without project approval. Any key/version lifecycle must be explicitly scoped to the relevant contract/protocol, mechanically enforceable, and unable to revive consumed authority. Do not replace arbitrary-verifier rejection with arbitrary administrator discretion.

### F05 — High: normative product requirements embed named reviewer availability

**Observation:** MC05 spec :40–49 binds package acceptance to Fable/GPT identities and prohibits dependent acceptance when unavailable; MC04 spec :40–41 and MC08 spec :4/:40–41 repeat this. These are package/release requirements, but they share the same normative surface and overloaded “acceptance” vocabulary as transaction rules.

**Repair:** move reviewer/provider constraints into internal development-delivery contracts. Product verification must not read reviewer receipts, campaign IDs or provider identity. Preserve engineering evidence records as historical release evidence; revise their status and applicability instead of preserving their authority over end-user programs.

### F06 — High: blanket proof/public-command admission leaks campaign policy into SDK use

**Observation:** SP02 :70, SP08 :92 and SP09 :94 state that proof and public commands require live RP03 admission. ROADMAP :74/:198–202 and the skill define RP03 with exact candidate, reviews, accounting and resource reservations. This is understandable for project agents spending shared resources, but is not a language deployment rule.

**Repair:** qualify every occurrence as project-operated campaign execution using project credentials/resources. Remove this dependency from distributable compiler, prover and SDK contracts. Add a regression scenario: a clean external developer installation with no `.moriarty-dev`, Foreman or reviewer records can compile, produce a valid proof and submit through its own Midnight environment. Missing proof still rejects; missing project records must not.

### F07 — High: proof-carrying general programs remain unimplemented, despite useful prototype checks

**Observation:** source /5 spec :64 excludes proof and submission; outcome runtime :188–190 refuses real acceptance. SP09 :44–48 correctly demands replacing the fixed-instance relation with a versioned Core transition relation and constructor coverage. The referenced product acceptance/adapter directories are absent.

**Repair:** prioritize a genuine general supported-domain source → Core → proof statement → Compact/Midnight verifier path. Bind program, property specification, signed intent, predecessor state/history, observations, complete outputs/effects and bounds. Include a newly authored contract outside loan/swap fixtures; constructor coverage and correspondence remain required beyond two examples. Maintain honest feature-state labels until implemented and demonstrated.

### F08 — High: attestation “strength grades” risk substituting trust for correctness proofs

**Observation:** SDK :48 offers `Proven` versus `Attested`; :724 admits adjudication as another trusted import. That distinction is useful for external fact assumptions, but cannot downgrade Moriarty's own mandatory contract/refinement/transition/history claims. `site/src/data/intents.ts:129` explicitly rejects operator/bridge attestations as discharge of a proof obligation.

**Repair:** use separate axes: verified program/transition/history correctness, and provenance/trust of observations. A theorem conditional on an attested input is not a proof that the external event happened. Never let `Attested` satisfy the mandatory execution proof. A signed assumption must be visible and correctly linked to the proved conditional statement.

### F09 — High: library conformance can be mistaken for an exhaustive program allowlist

**Observation:** README :19–27 makes financial behavior define the language; SP08 :36/:44 covers fixed DeFi rows and categories; mock runtime :88 accepts only `swap|loan`, and :147 rejects unregistered programs. These are valuable coverage and prototype boundaries, not an appropriate restriction on all future DeFi programs.

**Repair:** explicitly designate ACTUS/DeFi corpora as required qualification libraries and regression targets, not the exhaustive set of deployable contracts. Replace demo registry selection in the future product path with ordinary checked source/code commitments. A user's signed permitted-program set remains legitimate protection of that user's authority; a centrally curated set of developers or programs does not.

### F10 — High: correctness intent must be formalized without claiming mind-reading

**Observation:** the good intent relation is concrete, while the product goal could be read as proving arbitrary natural-language intention. Existing `IntentIR` supports finite constraints; nominal liability authorization is still a planned requirement in MC05 :80–81 and SDK :419.

**Repair:** expose developer-specified invariants/pre/postconditions and user-signed authority/outcome constraints in canonical form. Prove actual execution refines them, including newly created liabilities separately from debit caps. Provide examples/counterexamples and feasibility checks to help users validate specification fidelity. Do not claim a proof establishes unstated intent or oracle truth.

### F11 — Medium: Preview qualification is written as a possible per-program administrative prerequisite

**Observation:** README :9 says each supported financial capability needs Preview execution; ROADMAP :7 defines the product release gate. This is sound release qualification but ambiguous if read as requiring project review of every user program's deployment.

**Repair:** distinguish proving a user's transaction from qualifying advertised toolchain capabilities. Keep actual Preview financial and mandatory-PCD evidence as engineering deliverables. A permissionless deployment is governed by objective target-network/proof rules, not by whether the project has previously demonstrated that exact program.

### F12 — Medium: stale authoritative summaries encourage demo-first scope regression

**Observation:** completion program :25–29 retains hello-world-only wording for its old evidence boundary, while README :57 and ROADMAP :9 report later financial Preview success. AGENTS current constraints describe financial settlement as open without sufficient scoped distinction. Successor README :3 describes syntax-only support although later profiles implement much more. These can be locally true historical statements but are misleading as unqualified current summaries.

**Repair:** distinguish immutable historical receipt, current capability index, active product specification and internal execution plan. Root documents should lead with permissionless language scope and present capability status, with exact links to older evidence. Historical records stay intact; active normative documents must receive explicit supersession notices or corrections.

### F13 — Medium: optional service economics become compulsory language machinery

**Observation:** SDK :613 prices quote solicitation, verification and every service; :622 requires benchmarked tariffs for profile admission; :694 and :712 discuss protocol/author revenue and admission eligibility. These may be choices for an operator marketplace, not requirements of a language.

**Repair:** keep static/runtime work bounds and explicit transaction fee constraints in the language/protocol model. Move paid quote service, operator bonds, market tariffs and royalties to optional applications. Offline verification and self-hosted proving must not require a paid coordinator.

## Proposed separation and acceptance tests

| Layer | Legitimate predicate | Must not depend on |
|---|---|---|
| Language | Defined syntax/types, bounded execution, effect semantics, expressible contract properties | Approved developer identity or central program catalog |
| Program proof | Correct statement, actual proof, program/spec commitments, supported semantics | Model-provider approval or self-declared certificate labels |
| User authorization | Signed authority, recipients/assets, liabilities, expiry, remaining budgets | Project permission to publish code |
| Ledger acceptance | Verified transition/history, correct effects, currentness, unique consumption, network resource rules | RP03, Foreman run, internal audit receipt |
| Optional application | Its disclosed user-selected policy, oracle assumptions, service terms | Universal control of Moriarty deployment |
| Internal development | Review, test, budget and credential controls for project work | Inclusion in language acceptance relation |

Add explicit negative controls for administrative leakage: delete project control files; replace provider names; leave a reviewer offline; deploy a novel bounded program not in demo registries. These changes must not alter product validity. Conversely, deleting correctness evidence, changing signed recipient/asset/program commitments, exceeding work, forging predecessors or substituting a verifier must reject. Thus permissionlessness and mandatory correctness are tested together.

## Suggested EARS amendments

- **MOR-PERM-001:** When any developer supplies a supported well-formed program and the required correctness evidence, the public toolchain shall permit compilation/proof/deployment without project-administrator approval, subject to the target ledger's objective rules.
- **MOR-PERM-002:** If project workflow metadata or reviewer receipts are absent, the public toolchain shall not reject solely for that absence.
- **MOR-PROOF-001:** When a transaction applies program effects, the acceptance relation shall verify the required contract, intent-refinement, transition and predecessor-history predicates for the exact bound artifacts.
- **MOR-INTENT-001:** When a concrete plan is selected, its proof shall establish the signed authority, asset/recipient, fee, liability, lifecycle and outcome constraints without enlarging authority.
- **MOR-EXT-001:** When a developer deploys a new program using supported language constructs, validation shall depend on those constructs' semantic/proof rules rather than membership in a project program registry.
- **MOR-ASSUME-001:** Where correctness depends on an external observation, the artifact shall identify that assumption and shall not label an attestation as proof of the external fact.
- **MOR-DEV-001:** While Foreman/Pel executes project development work, its review/resource controls shall remain internal workflow conditions and shall not become required inputs to user program acceptance.

These are proposed amendments, not new immutable gate identities. Crosswalk them against existing requirements, delete or rescope contradictory administrative requirements, and retain objective financial/proof obligations on their merits. Do not mechanically preserve all MC/SP gates.

## Revised consensus conditions

I would not endorse v2 as the current plan. Its instruction to preserve every old gate and its architecture-preservation premise were too conservative after this user correction. A revised plan must explicitly name permissionless Midnight language deployment, establish the proof-versus-process separation above, demote the managed multichain SDK to optional research/application scope, and add a genuine arbitrary-supported-program proof path rather than only Aeon authoring conveniences. Trust reporting and exact VC/synthesis tooling remain useful subordinate enhancements. Final consensus awaits the revised common brief.
