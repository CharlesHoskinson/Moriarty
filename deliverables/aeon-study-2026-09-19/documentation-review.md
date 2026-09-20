# Current documentation audit — Astra A2

Date: 2026-09-19. Repository: `/home/charl/Moriarty-aeon-study`, HEAD `1896217a28553e0254b0ba319422ac4c0b39ac0d`, with uncommitted documentation changes. This is an advisory review of the working tree, not product acceptance. Findings use repository-relative paths and observed line numbers; concurrent edits can change them.

**Verdict: changes requested before describing documentation reconciliation as complete.** The new product contract and roadmap establish the correct permissionless language boundary and retain mandatory proofs. Remaining issues concern surviving assurance claims, an active contradictory summary, and traceability. No product implementation or deployment was tested.

## Reviewed inventory and independence

Read the new product contract, roadmap, OpenSpec index and permissionless-provable-intention proposal, design, specification, requirements table, tasks, workflow and Pel sample. Inspected git diff for README, SDK design/reconciliation, completion program and wiki navigation; examined the corrected MC04/05/08 and SP02/08/09 requirements, targeted AGENTS/plugin instructions, sprint navigation and machine-register wording. Checked local Markdown path existence across changed and untracked Markdown files. Inspected SDK absolute-guarantee statements separately from its new scope preface.

This reviewer authored the earlier six MC04/05/08 and SP02/08/09 corrections. Their assessment here is self-review, not independent approval. Other sections were authored by others. No other reviewers' outputs were consulted for this audit.

## Blocking documentation findings

### B1 — Active SDK summary still presents superseded product semantics

`wiki/defi-kernel-sdk-interface.md:5` is active; `:27–29` calls the interface normative; `:40–55` says assets/chains are hidden routing details and only strength/finality reach callers; `:79–82` hides capability assumptions. These conflict with the corrected SDK and product contract. The old commit locator also presents a historic version as current normative text.

Repair through the authorized vault transaction: scope this page to optional managed-routing research, expose exact asset/domain/substitution and trust assumptions, qualify guarantees, and point to the current language product contract. Keep historical commit evidence explicitly historical. A planned later transaction does not yet resolve this finding.

### B2 — Instant universal revocation survives the SDK correction

`docs/superpowers/specs/2026-09-11-defi-kernel-sdk-interface-design.md:88` promises revocation “at once” on “every network.” Its final caveat about already dispatched work does not address propagation, independently exercisable outstanding authority, or unavailable networks. A specified-only label avoids an implementation claim but does not make this cross-domain requirement feasible as stated.

Repair: distinguish immediate local refusal of new service work from per-domain effective revocation; record submission, acknowledgment, finality and residual exposure. Routes lacking the required revocation property must be refused or use signed bounds/expiry with disclosed limitations. Preserve owner authorization and refuse falsely claiming revocation complete.

### B3 — Funding a reserve is still presented as guaranteed completion

Same SDK `:131` says in-flight work is completed from a reserve and stopping “never strands” work. The decision table `:1085` reinforces unconditional satisfaction. Funds do not establish inclusion, network availability, evidence availability or recoverability. This conflicts with the corrected opening and the product contract's explicit separation of safety from liveness.

Repair: reserve enough bounded cost for the defined continuation/recovery policy, then condition progress on stated environment assumptions. Preserve pending/unknown state and residual duties when progress is unavailable. Do not report a funding reservation as a liveness proof.

### B4 — ZKIRv3, jets and phase semantics lack traceability rows

`openspec/changes/permissionless-provable-intention/requirements.md:5–17` lists MOR-001 through MOR-008, AEO-001 through AEO-004 and DEV-001, but omits MOR-009/010/011, which are defined in `specs/permissionless-intention/spec.md:146–176`. These are explicit user priorities rather than optional additions.

Repair: add compiler/target, primitive-certificate/composition, and ledger-phase owners with specific planned evidence. Bind MOR-009 to task3.7, MOR-010 to3.8, MOR-011 to3.9 and relevant main proof tasks. Replace generic repeated “planned behavioral harness” text with discriminating evidence: target pin/artifact, arbitrary-witness constraint soundness and valid-input completeness, and phase-specific effects/fees controls. Do not invent implemented commands.

### B5 — Completion mutation criterion rejects legitimate alternatives literally

`docs/MORIARTY-PRODUCT-CONTRACT.md:62` says mutations of fees/effects must reject without qualification. Different fees or effects can satisfy the same signed bounds with newly valid evidence. Mutation testing must establish binding, not require one unique admissible execution.

Repair: require rejection of tampering without the corresponding valid proof/signature binding, or of changes violating the bound semantics/intent; include acceptance of independently valid alternative plans within unchanged hard constraints.

## Nonblocking repairs and limitations

1. **Archived roadmap navigation:** `deliverables/aeon-study-2026-09-19/ROADMAP-before-audit.md` preserves old relative links after moving two directories deeper. The path check identified101 broken targets in this archive, beginning at`:9`. Keep the byte-exact historical archive if desired, but document repository-root link interpretation and supply a navigable wrapper or separate rebased rendering. Do not silently edit historical evidence while claiming byte identity.
2. **Agent entry-point wording:** `AGENTS.md:47` says preserve all product gates; plugin `plugins/moriarty-dev/skills/develop/SKILL.md:22` groups financial settlement and PCD as open. The new contract overrides contradictory product scope, and plugin`:30` already separates independent edits from campaign admission. Nevertheless add an explicit product-contract pointer and internal-only scope at these active entry points so a fresh agent does not reconstruct administrative deployment requirements. Clarify scoped financial results versus general proof capability. Preserve legitimate project-resource controls.
3. **README target clarity:** `README.md:7` still describes Compact as the compilation goal while the new contract requires emitted/pinned ZKIRv3 and Midnight execution. Compact can remain the intermediate route, but the overview should name the actual terminal target. This is ambiguity, not evidence that the implementation now supports ZKIRv3.
4. **Traceability remains a plan:** repeated generic evidence rows do not yet identify test ownership or coverage of every subclause. For example formal intent includes fee/liability/lifetime constraints, while MOR-004's negative example tests only asset substitution. A full implementation package needs separate boundary controls; this does not require manufacturing tests for a documentation change.
5. **APSS literature delivery separate:** tasks2.1–2.3 remain unchecked; this audit did not validate completed vault ingestion or all six current receipts. The architecture distinguishes application formalization, owner authority, proposal sources and settlement without imposing a managed router. Do not equate that conceptual coverage with a completed literature bank.

## Technical completeness assessment

The product contract correctly requires actual ZKIRv3 artifacts, compiler/circuit/key/ledger pins, invalid-witness exclusion and valid-execution completeness. It explicitly rejects claiming certified jets from hashes/tests/shared invariants, preserves preconditions/framing and separates physical speed from logical work/circuit cost. It retains history, residual liabilities and private-composition obligations. MOR-011 correctly prevents local rollback from being promoted to universal ledger rollback. These are substantial improvements; none is a completed theorem.

The Pel sample visibly verifies before reviewing and bounds correction to one attempt. Its accompanying text correctly marks host gate/roles/schema registration as prerequisites and disclaims static checking as execution authority. I did not execute Pel, validate its pinned interpreter semantics, or audit host candidate immutability. Treat executable workflow acceptance as open until separately verified.

## Verification and uncovered surfaces

`git diff --check` passed. `openspec validate permissionless-provable-intention --strict --no-interactive` passed. Plugin read-only status ran; output saved at `/tmp/astra-doc-status.json`. These results establish formatting/schema checks, not semantic reconciliation or product correctness.

Local Markdown scan examined367 relative file links, excluding URLs and ignoring fragments. It found the101 archive failures and10 failures in pre-existing portions of wiki/index.md and wiki/log.md; those ten are not newly introduced by the observed diff. Wikilinks, anchors, remote links and Obsidian resolution were not exhaustively validated.

This was a broad bounded audit, not an exhaustive repository audit. Unreviewed or sampled surfaces include every legacy OpenSpec requirement/register field, all public language specs, installed plugin copies versus checked-in copies, generated graph/index content, all historical deliverables, every SDK assurance sentence, all APSS vault material, actual proof circuits, runtime/CLI acceptance and deployed Midnight behavior. Existing claims should not be called globally reconciled on the strength of this report.
