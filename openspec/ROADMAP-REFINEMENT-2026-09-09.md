# Report-informed Moriarty roadmap refinement

Status: specified-only planning revision. Execution date: 2026-09-09 UTC; research cutoff: 2026-09-08. This refines the existing SP01–SP12 program under the user's latest planning request. It grants no proof resources, network dispatch or product acceptance. [ROADMAP.md](../ROADMAP.md) is the reading entry point; the original eight MC packages retain acceptance ownership.

## Recommended roadmap

Build a usable bounded financial language, test the complete native boundary early, then join them at actual Midnight acceptance. The financial taxonomy organizes requirements; the linked standards atlas supplies version-scoped interface and behavioral constraints. Neither automatically determines the Core's constructors.

| Sprint | Concrete output | First demonstration and exit condition |
| --- | --- | --- |
| [SP01](sprints/sp01-financial-contract-and-execution-admission.md) | Complete financial/authority contract and early native decision | Reuse accepted atomic evidence; freeze independent positive/negative behavior traces; record bounded F0 go/no-go and path ownership. |
| [SP02](sprints/sp02-complete-mori-authoring-frontend.md) | Complete `.mori` lexical, EBNF, static and canonical specification; check/format tools | A real source agreement checks and formats; wrong units, unknown forms and excessive bounds reject. Complete matched syntax study and all grammar coverage. |
| [SP03](sprints/sp03-executable-bounded-semantics-in-k.md) | Executable K, reference evaluator, simulation and base-domain proofs | Partial payment preserves residual debt; K and evaluator agree on all admitted observations. Required metatheorems and elaboration/evaluator correspondence are discharged. |
| [SP04](sprints/sp04-complete-native-verifier-component-feasibility.md) | Complete native/outer verifier feasibility | Independent non-loan fixtures pass all P1/P2/P3 controls, including carried accumulator and actual final pairing; decisive failure stops the affected route. |
| [SP05](sprints/sp05-financial-integration-on-preview.md) | Real loan and swap financial integration | Both operations finalize on Preview with authenticated roles, actual custody, fees/change and exact state/effect comparison. Report transaction IDs. |
| [SP06](sprints/sp06-real-recursive-financial-history.md) | Real two-step recursive financial proof | Retained serialized artifacts verify in another process; altered state, context, keys and proof bytes fail. |
| [SP07](sprints/sp07-actus-obligations-and-lifecycle-semantics.md) | Complete ACTUS semantic implementation | NAM19 capitalization and authorized refinance first; then all 277 fixtures/all present fields, 18 executable types and 32 dispositions through shared source/Core/backend paths. |
| [SP08](sprints/sp08-defi-actions-and-outcome-intents.md) | DeFi action library, outcome authority and request lifecycle | All 72 historical rows, DA24, intent cases and adopted TX/VX regressions; pending rights, gross budgets, valuation purpose and complete effects survive adversarial traces. |
| [SP09](sprints/sp09-mandatory-pcd-and-ledger-correspondence.md) | Mandatory proof acceptance and compiler/ledger correspondence | Atomic F3 first; later general successor relation with all four claims, durable consumption and verification-enabled financial Preview acceptance. |
| [SP10](sprints/sp10-private-handoff-and-bounded-composition.md) | Real private continuation and five composition operators | Isolated participants prove split/branches/join without predecessor secrets; shared collateral, residual duties, authority and work survive conflicts and recovery through the accepted lineage. |
| [SP11](sprints/sp11-full-financial-and-formal-conformance.md) | Complete financial and formal qualification | Every required behavior maps to semantic, profile-proof, local acceptance and required Preview evidence; proved reductions are explicit and mutations cannot disappear behind samples. |
| [SP12](sprints/sp12-developer-release-and-reproducible-evidence.md) | Usable, reproducible developer release | End-to-end workflow, two independent clean builders, two non-toy pilots including ACTUS, audit closure, licensing and individual G01–G24 dispositions. |

These are evidence boundaries, not twelve promised calendar durations. No runtime cost estimate has been established by this planning exercise.

## What is already accepted, and what is not

The register records `atomic-prepare`, `atomic-accept` and `rp01-mc02` complete. Atomic acceptance binds commit `c30aaf5c0d091b1efea7e042d0fcbd539ca6cc51`, candidate digest `2f1fb86407537720174bbf73cb50426499744986e97ce8a5dde9f792577e0b5e`, and the [retained reconciliation](../evidence/moriarty-completion-program-2026-09-07/report-reconciliation/atomic-reconciliation.json). This revision corrects stale roadmap text; it does not rerun or broaden that acceptance.

The fixed financial comparator and skip-zk custody wrappers are useful scoped implementation. Actual payer/asset/UTXO ownership, fee/finality decoding and finalized financial settlement remain SP05. The retained Preview hello-world transaction demonstrates network access only. The full successor grammar, K semantics/proofs, native recursion, mandatory financial acceptance and private composition remain unfinished.

Startup currently reports missing current accounting and unresolved historical resource/operational state for `sp01-loan-swap-grok-01`. Its dependent dispatch remains blocked. This planning revision does not repair that state, reset charges, revoke accepted atomic evidence or turn it into a dependency for unrelated authorized work.

## Order the work by actual prerequisites

The existing 21-stage graph in [sprints.json](sprints/sprints.json) and the [program register](moriarty-completion-program.json) is unchanged:

1. **Language track:** full RP01 behavior/authority design → successor frontend → base K/evaluator semantics → ACTUS and DeFi extensions. Define financial expectations during SP01, before implementing their constructors. SP07/SP08 can prepare separate fixtures in parallel; serialize changes to shared Core/K files.
2. **Native track:** F0 go → native-path-freeze → F0a → independently generated non-loan fixtures → all F1 controls → corrected financial F2. Preserve source/deployment pins, actual outer final decision and the prohibition on retrying the failed k17 encoding without a reviewed changed hypothesis.
3. **Network track:** accepted atomic profile + accepted RP01-MC02 subset → I2 financial loan/swap integration. Full RP01 is not a prerequisite of this fixed subset.
4. **Early join:** atomic acceptance + I2 + F2 + F1 → SP09.1 atomic F3 with its scoped mechanized correspondence. Full SP07/SP08 completion is not an atomic F3 prerequisite.
5. **General join:** F3 + full RP01 + accepted successor, ACTUS and DeFi semantics → mandatory successor acceptance → private composition → complete conformance → release.

Whole-sprint completion dependencies describe final closure, not task-entry barriers. Preparation records cannot waive native controls. Full MC04 acceptance cannot become a prerequisite for generating the MC03 proof that MC04 consumes. A native no-go blocks its route while eligible language work continues.

## Convert reports into explicit acceptance obligations

[report-lessons.json](sprints/report-lessons.json) is the machine-readable additive crosswalk. It records source digests, existing task owners, disposition, positive evidence and distinguishing failures. All entries remain planned. The research atlas's financial hierarchy remains a proposed organizing reference mapped to existing F1–F6/P and DA identifiers; this revision does not declare ERC implementation or add an EVM backend.

The eighteen lesson groups cover classification dimensions; source/version evidence; nominal quantities; four valuation purposes; conversion/preview/limit/authorization distinctions; requests and residual rights; debt/accrual/loss; complete effects; signed authority; economic assumptions; bounded composition; interface revisions; complete native verification; nonvacuous correspondence; actual Preview evidence; usable reproducible release; capability-focused work; and preservation of the full financial denominator.

Specific report dispositions are:

- **TX01–TX12 and VX01–VX06:** adopt as required bounded modeled positive/negative regressions. They are hypothetical models, not reproduced incidents or certification of named deployments. Source-defined product behavior still needs a pinned primary lifecycle source. Existing fixtures may satisfy overlapping cases only with a reviewed equivalence mapping that retains each case's distinguishing failure.
- **The 24 modern component cases:** require a component/version-scoped crosswalk to existing financial/action rows and an explicit additional/shared/comparative disposition. They are evidence for design coverage, not 24 protocol ports or a replacement for the 72 historical rows. Non-EVM examples test the taxonomy's generality only.
- **Three uncovered leaves:** conditional crowdfunding (`FIN-CAP.2`), managed liquidity (`FIN-MGT.3`) and default backstops (`FIN-RSK.3`) require representative primary behavior and independent fixture resolution in SP08.1/SP11.1. Coverage remains incomplete for those leaves until that work is done. Other unmeasured deployment/adoption claims remain research gaps, not invented implementations.
- **Vault semantics:** adopt asset/share distinctions, all four preview directions, actual limits, explicit fee/rounding policies and separate accounting/redemption/market/stressed values. ERC-7540 pending/claimable/pull-claim behavior and affected preview rejection are explicit; ERC-7575 entry points and external shares are distinct. NAV publication is not exit liquidity.
- **Evolving standards:** bind revision and actual behavior. The May 2026 ERC-7683 resolver/payment/assumption design is separate from old order/settler interfaces. ERC-7887 cancellation wording remains a source gap; do not derive a general cancellation rule from it. Transfer restrictions do not establish legal enforceability.
- **Security and economics:** preserve Werner's atomicity-based definitions, Zhou's system/dependency layers and protocol-versus-strategy distinctions. Authentication, interface compatibility, finite execution, solvency, oracle truth, liquidity, liveness, legal claims and optimality remain separate propositions with named assumptions.

The first new language demonstration should be the partial-payment slice: `.mori` → typed Core → K and evaluator → complete observation with surviving debt. The plain due100/pay30 model leaves70; the separate interest-first TX02 model P100/I10/pay7 leaves P100/I3. Neither authorizes rewriting ACTUS fields. This slice checks the central obligation distinction before broad library expansion.

## Preserve every existing obligation

The retained denominator is unchanged: 277 ACTUS fixtures, 18 executable types, 32 taxonomy dispositions, 72 historical DeFi rows, 24 actions, three held-outs, eight intent cases, eight regression classes, five composition operators and twelve additional report products. Existing DS-01–DS-07 source discrepancies remain blocking in their required scope.

[coverage.json](sprints/coverage.json) includes every normative OpenSpec requirement, including the sprint-program specification previously omitted by the validator. [package-task-map.json](sprints/package-task-map.json) preserves all 239 source-qualified numbered MC tasks and maps them to the existing 45 sprint tasks. Mappings assign closure responsibility; they do not alter original task status or add scheduler prerequisites. Required contributing scopes must close before claiming the full original obligation. Existing narrow atomic acceptance cannot discharge broader successor work.

[legacy-release-gates.json](sprints/legacy-release-gates.json) retains all G01–G24 predicates verbatim from the [original prompt](../deliverables/moriarty-semantics-intent-compiler-sdk-deep-research-prompt-2026-09-03.xml), checked against the historical program record. In particular:

- G12 requires byte-for-byte builds on **two independent clean builders**.
- G16 requires a named trusted computing base and **a separate leakage theorem for confidential profiles**; process isolation alone is insufficient.
- G17 requires **two non-toy pilots**, including the complete ACTUS benchmark, against the audited Compact-library baseline. A model's opinion does not establish user preference.
- G19–G24 retain the complete source lock, lossless all-field execution, general compiler path, 32 dispositions, attribution/licenses and honest reference-vector compatibility claims.
- G18's historical reduce-scope-or-stop predicate is preserved. Current Midnight, mandatory-PCD and full-coverage instructions prohibit autonomous scope reduction: stop the affected release path and retain the blocker; a changed product scope requires the user's decision.

## Network evidence by milestone

| Milestone | Required evidence | Scope limit |
| --- | --- | --- |
| SP02/SP03 | Local source, Core, K, evaluator and mechanized claim artifacts | No invented transaction for a syntax or theorem milestone. |
| SP04/SP06 | Native/outer controls and independently verified retained proof bytes | Local proof feasibility is not financial settlement. |
| SP05 | Actual loan **and** swap Preview IDs, canonical finalized blocks, assets/roles/fees/change and complete independent effect comparison | I2 remains an uncertified integration probe. |
| SP09 | Actual verification-enabled Preview financial acceptance under the exact atomic and then mandatory lineages; conflict/replay/currentness controls | Earlier narrower receipts cannot inherit new semantics. |
| SP10 | Accepted private composition/conflict/recovery evidence bound to Preview and the same mandatory lineage | A shared-process or purely linear demonstration cannot close private split/join. |
| SP11 | Per-required-behavior traceability across semantic/proof/local/required Preview columns, with explicit proved reductions and qualified representative episodes where valid | A sample is not a universal theorem; no omitted row or required Preview condition. |
| SP12 | Cold-start checks, retained proof/receipt verification and a usable accepted developer demonstration | Fresh transactions are separate funded actions, not reruns disguised as receipt verification. |

For every actual submission, report transaction ID and observed state in the conversation and retain the exact deployment/profile/evidence path. Failed and pending submissions remain visible. This planning sprint submits no transactions.

## Apply the orchestration lessons during implementation

Use the already loaded development plugin and existing task contracts. Keep one primary capability, a concrete next command or observable result, and one owner per shared file. A routine authorized edit, regression or local review needs no new campaign, packet, dashboard, vote or loop. Resource-controlled K/native/prover/network runs still require their exact existing admission, candidate review and limits. This distinction supersedes older blanket “packet before every edit” wording.

Each implementation slice ends with a demonstrated behavior, decisive bounded probe or concrete resolved blocker. Report its scope and the next useful demonstration. Apply the [footgun stop rules](../docs/FOOTGUNS.md): after two same-defect cycles, reproduce the defect and change the approach before another broad correction; after two process-only cycles or thirty minutes of administration, stop adding orchestration and work on the callable product path or an eligible independent task. A planning document is the product of this user-requested planning turn; it does not count as language implementation.

These instructions preserve mandatory tests, independent audits and hard resource stops. They do not claim the plugin can force technical success or that orchestration failures are solved merely because the rules are written down.

## Review and remaining limits

The requested reviewer roles are Fable 5.1 medium as report advocate and a fresh GPT-6 Astra high as independent roadmap auditor. The [review record](../deliverables/roadmap-refinement-2026-09-09/README.md) records actual provider participation, candidate digests, findings and dispositions. A failed provider call has no vote and cannot be labeled advocacy. Planning review never constitutes execution admission or result acceptance.

Open technical blockers remain explicit: native outer-verifier fit/finalization, full grammar and K claims, real financial custody/finality integration, source gaps, full conformance, actual private composition and release/pilot evidence. Resolve them at their existing owning tasks. Do not make a new orchestration project the prerequisite for progress.

Validation uses the existing `python3 openspec/sprints/verify.py` and `openspec validate --all --strict`, plus focused negative controls for dropped ownership, stale sources and altered legacy gates. These check planning integrity only.
