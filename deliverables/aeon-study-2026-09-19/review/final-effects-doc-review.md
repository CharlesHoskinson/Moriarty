# Final effects documentation review

Status: **NO BLOCKERS** after rechecking F1–F3 and the revised roadmap, design, EARS traceability and implementation packages. Read-only review of the files below in `/home/charl/Moriarty-aeon-study`; no production files were changed by this audit. This is a documentation review, not evidence of implemented, proved or deployed behavior.

## Resolved findings (history retained)

### F1 — Distinguish success predicates from allowed failure transitions

**Resolved; formerly medium and blocking.** `openspec/changes/mc05-mandatory-claim-acceptance/specs/mc05-mandatory-claim-acceptance/spec.md`, lines 17–25, especially the “Authorized route choice” scenario.

The previous text required every accepted route to satisfy net goals and treated fees violating a net goal as unconditionally intent-invalid. The new product contract and MOR-011 intentionally permit explicitly signed partial/failure transitions where fees or other guaranteed effects survive without financial fulfillment. Before correction, an implementation could follow either reading and produce opposite verdicts for the same permitted fee-only outcome.

Repair requested and now applied: require every accepted outcome to satisfy all-outcome authority, complete-effect and residual-obligation predicates; require successful fulfillment to satisfy the net goal; require a non-fulfillment transition to satisfy the separately signed partial/failure policy. A prohibited failure charge must still reject. Do not weaken the success floor or reinterpret arbitrary failure as authorized.

### F2 — Qualify the no-partial-effect verification boilerplate

**Resolved; formerly low, related consistency correction.** SP09 “Verification entry points” previously said invalid source/data must reject without a partial effect. This is appropriate for frontend/preflight failures and unsupported constructions; it was ambiguous for state-dependent fallible ledger execution. The requested correction was to state that ledger outcomes use the pinned phase mapping and retain every authorized surviving guaranteed effect/fee. SP02/SP08 can retain source/evaluator-local atomicity, explicitly scoped to those layers.

### F3 — Restore MOR-012 traceability table rendering

**Resolved; formerly low.** `openspec/changes/permissionless-provable-intention/requirements.md` had a blank line between the MOR-011 and MOR-012 rows (lines 27–29 in reviewed bytes). MOR-012 therefore rendered outside the table. That blank line is now removed; the requirement itself exists and is substantively sound.

The final MC05 wording now separates successful net goals from all-outcome signed failure bounds and explicitly forbids labelling partial/failure outcomes successful fulfillment. SP09 now distinguishes effect-free pre-submission rejection from submitted transactions retaining authorized guaranteed effects and fees. MOR-012 now occupies a continuous table row. These resolve F1, F2 and F3 without weakening authority or successful fulfillment.

## Expanded roadmap/design review

No blockers found. The roadmap prioritizes general supported-program source/Core-to-ZKIRv3-to-Midnight execution and mandatory native proof verification, followed by compositional financial/authority semantics and private composition. Fixed examples remain regression evidence rather than a program allowlist. Existing MC/SP objective obligations remain open until independently discharged; historical roadmap content is preserved by a linked snapshot.

The new design and EARS traceability retain explicit observation assumptions, exact assets, resource bounds, residual liabilities, phase outcomes and arbitrary-witness certificate obligations. P0–P7 supply bounded technical objectives, inputs, owned outputs, acceptance evidence and stop conditions. P0 must establish actual toolchain commands and correspondence boundaries; P1 cannot equate host tests with target certification; P2 requires a novel financial program and exact phase-aware native acceptance; P3 keeps evaluator replay and vacuity checks; P4 preserves affine authority and persistent liabilities; P5 cannot weaken specifications. Missing implementation paths and behavioral harnesses are explicitly planned, not represented as existing.

The Pel sample verifies before independent review, limits correction to one round and returns needs-action if unresolved. Its approval token and roles apply to internal implementation artifacts only; public compile/prove/deploy acceptance does not require them. This review reads its control flow; it does not claim independent Pel execution or host-binding validation. No active campaign or deployment is established by the plan.

The final P6/P7 additions and full roadmap crosswalk were reviewed after the author reported stable files. P6 retains original fixture identities and fields, independent expected traces, separate semantic/proof/local/Preview evidence and explicit reduction obligations; pending claims and typed debt cannot disappear. P7 retains all legacy release predicates, pilots, baseline, licenses and two-builder reproducibility, while an external developer needs no project metadata. No blocker was found in the added MC01/02/03/06/07 and remaining SP01–SP12 scope corrections: internal campaign review remains distinct from proof/ledger validity, and signed phase failure bounds replace blanket no-partial-effect wording. Review of these additions covered their diffs and crosswalk rather than every nested execution record or implementation.

## Preserved obligations and sound corrections

- MC04 retains exact native proof/VK/public-input verification, source/Core/proof/ledger correspondence, full effect projection, durable one-time consumption, conflict/restart cases and complete final verification. Administrative records are explicitly outside public program validity.
- MC05 retains all four mandatory claim types, canonical non-circular bindings, rejection of mock/signature-only substitutes, bounded verification, policy lifecycle and liabilities distinct from token budgets. Contract/protocol/participant policy has replaced a project developer allowlist without allowing an unrelated prover-selected verifier.
- MC08 preserves actual finalized-effect reporting, G01–G24 reconciliation and unavailable-proof/witness handling. Maintainer release reviews do not become a developer prerequisite.
- SP02/SP08/SP09 retain finite typing, mixed-asset rejection, the 72 DeFi rows and DA01–DA24 qualification obligations, durable partial fills, refund-not-reset semantics, all-path mandatory proofs and real Preview acceptance. Examples qualify the language; they do not license programs.
- The public contract and MOR-011 correctly include per-phase fees and replay/authority consumption. MOR-012 correctly separates per-asset conservation, including custody/reserves and authorized supply changes, from typed liability evolution. No debt-to-money-supply equation is introduced.
- Source/target/circuit verification, attested observations, privacy and liveness are distinguished. The public contract explicitly marks end-to-end capability unfinished and semantic correspondence unestablished where not supported by evidence.

## Deployed-behavior wording

No new deployment completion claim was found in the reviewed scope. The corrected packages say their requirements describe intended behavior, and the product contract is labelled target architecture. Its statement about Midnight phase behavior matches the freshly captured official documentation, not a newly reproduced Moriarty transaction. Preserve that distinction and pin the actual backend/version during implementation. Do not count this review or the literature bank as deployed proof evidence.

The retained sentence “actual acceptance checks ... before applying effects” is acceptable as a normative requirement for invalid mandatory proof/authorization, which must fail before ledger acceptance. It must not be extended to imply global rollback of every business-operation failure.

## Verification

Strict noninteractive OpenSpec validation passed for `permissionless-provable-intention`, `mc04-ledger-correspondence-and-consumption`, `mc05-mandatory-claim-acceptance` and `mc08-release-evidence-and-developer-flow`. This verifies document structure, not semantic consistency or runtime behavior. Initial target file content and diffs were inspected. On re-review, the three corrected locations and expanded roadmap/design/plan files were inspected; strict validation was rerun successfully for permissionless-provable-intention and MC05, and repository git diff --check passed. The remaining MC01/02/03/06/07 strict validators also passed independently during expanded review. Code tests and network campaigns were outside this review.

## Reviewed file hashes

Hashes bind this final re-review to the stable bytes read after the P6/P7 and full-crosswalk additions.

| File | SHA-256 |
|---|---|
| `openspec/changes/mc04-ledger-correspondence-and-consumption/specs/mc04-ledger-correspondence-and-consumption/spec.md` | `09ccaac973a58fe0e26b2795197757a43bcf6a4749103a4e0499be9aefe37fcd` |
| `openspec/changes/mc05-mandatory-claim-acceptance/specs/mc05-mandatory-claim-acceptance/spec.md` | `959e902e241c55179b31fbfb115c450de9197372c6a3684d72031c32e00fa36a` |
| `openspec/changes/mc08-release-evidence-and-developer-flow/specs/mc08-release-evidence-and-developer-flow/spec.md` | `1fc033b9b3fe0e070c43a889eb8c53719f17d437ca844abf47f6d37a1116a672` |
| `openspec/sprints/sp02-complete-mori-authoring-frontend.md` | `cdd42ddb432f9b15dd4cedada2962d1f4a68f314b32fd653e1269c6a4e159666` |
| `openspec/sprints/sp08-defi-actions-and-outcome-intents.md` | `c045a966b302aea167ab67f97332ea86c37d3b5782aeecd515a551b8ed947141` |
| `openspec/sprints/sp09-mandatory-pcd-and-ledger-correspondence.md` | `fe75769ef7760fa7c8caab5a20305dd0e22658f3dd17be088272cffe85c15395` |
| `docs/MORIARTY-PRODUCT-CONTRACT.md` | `4fc10f35df06c2bf3784176a13e048530c98003f2a3b04948512d6c6bcfc32c7` |
| `openspec/changes/permissionless-provable-intention/requirements.md` | `e38b6930aba348feea6932f390ac65d20dadc7af215fb7b610047c998373b1a8` |
| `openspec/changes/permissionless-provable-intention/specs/permissionless-intention/spec.md` | `d582dc807f654a4c39c1aa216cc83c9da6991eebd6b959ae28401a43a39f3e99` |
| `ROADMAP.md` | `564e6555a734ffc1881a72239271198c63ae67d4d56dd4548ebba8d5ebb34516` |
| `openspec/README.md` | `58ccdb84b245e95529a7e62b11db36ac201254019b0ce887a8ef91bd32e1f742` |
| `openspec/changes/permissionless-provable-intention/design.md` | `817882e7dcaebe9b06efb61ea70aa4444cfe2d16f7fdf1c34ed5b52615ffb060` |
| `openspec/changes/permissionless-provable-intention/tasks.md` | `bb4fda6df59f80f3f1c60eefc4591cb9719518c567955929a4e9a663ba8c036f` |
| `openspec/changes/permissionless-provable-intention/implementation-plan.md` | `163fba7540890e42d33bad961092dc7f0c716f07e8b8382427f71eb552b742a3` |
| `openspec/changes/permissionless-provable-intention/implementation.pel` | `fc8a5562e088524e8113961ad06f6851edabdc388cffdea13a531cb137c388f9` |
| `openspec/changes/mc01-bounded-language/specs/mc01-bounded-language/spec.md` | `e3fa66dc06523ab3b0a73e1cad8c48eba7ee01a7e652cd44ad627f5eb038f1b7` |
| `openspec/changes/mc02-preview-financial-operation/specs/mc02-preview-financial-operation/spec.md` | `6ef648b52b0f974c4af61cfaf7e366db309c80667c68de7a81aefbc2e53d35f4` |
| `openspec/changes/mc03-native-recursive-proof/specs/mc03-native-recursive-proof/spec.md` | `06548717ab2202fee13c7642fdcec183bdb63b21164018d82a5d5e28d330f053` |
| `openspec/changes/mc06-private-handoff-and-composition/specs/mc06-private-handoff-and-composition/spec.md` | `58121600ae1da0c05d2747fc19b01d4c5cd47ced4980987258fb81ec0c44fcad` |
| `openspec/changes/mc07-complete-financial-conformance/specs/mc07-complete-financial-conformance/spec.md` | `4ee4e177261beb140ff096589c82e2209c34beb7c68a115ad846fa25bd7cf44a` |
| `openspec/sprints/sp01-financial-contract-and-execution-admission.md` | `e5effe7b7490228d1aa71808219f6ab02fcdf909e0801f52072c00f493eeeea8` |
| `openspec/sprints/sp03-executable-bounded-semantics-in-k.md` | `2c8fe30a97e0347aec00ca5cfd022abda65ecffbf0f26e96cb4f3f281931bc89` |
| `openspec/sprints/sp04-complete-native-verifier-component-feasibility.md` | `3639ca22886b25dd25b73f7133f522ffdd2a1799a38333a2ed6f72c898d6618a` |
| `openspec/sprints/sp05-financial-integration-on-preview.md` | `850647920680df57c6603ef4ab740bf868df560148863c68adb6402cd0dcdc56` |
| `openspec/sprints/sp06-real-recursive-financial-history.md` | `3af3f3e350018f48715275274dff97c80a7436a1a30b4dd831dac26760ccca8d` |
| `openspec/sprints/sp07-actus-obligations-and-lifecycle-semantics.md` | `1d92c9565e39fa3ac5a9c0ed5c5ee8fe69a073d8988a8873ff05be246332b5ee` |
| `openspec/sprints/sp10-private-handoff-and-bounded-composition.md` | `f4a2b54deaf074f2784f96df0b76b1538b74cf180452175602c72674e8742005` |
| `openspec/sprints/sp11-full-financial-and-formal-conformance.md` | `960f3fb314ef7d42fd96445ad3cc40646a46127123a5040b47d8e9f964f6f569` |
| `openspec/sprints/sp12-developer-release-and-reproducible-evidence.md` | `fda3c1a4d20c470710bb96fc9e1d03812c8635172e791d9f5135c38486949a2a` |
| `docs/ROADMAP-RECONCILIATION-2026-09-19.md` | `dd1a58882f7ffbc46d84ded29f21be6a7df5fd94dfdcdc3ead749adccef9240a` |
| `openspec/sprints/README.md` | `3b9ca89e1aba8fddff37f1a8193f78a9a19b01e5f40d4005e1e7cc8bf7afa294` |
| `openspec/changes/permissionless-provable-intention/workflow.md` | `ecec98588f6c47cf2492553e5cf262a8b78e0a6b55c6b34a438a52d86455cca0` |
