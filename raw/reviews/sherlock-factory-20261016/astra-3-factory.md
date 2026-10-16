# Astra A3: a practical delivery factory for Moriarty V1

Planning recommendation, 2026-10-16. Inspected source HEAD: `9bcbca52bbfbc58443f6bdb287d73afb7d71f1ab`. No product, SDK, model, build, test or ledger execution was performed. The sole repository CLI invocation was the mandated read-only `moriarty_dev ... status --json`; it reported an old SP01 loan subset with stale inputs and missing accounting. It was not used to select work. This report is a proposal, not an accepted control, native admission, security verdict or milestone closure. Requested reviewer role: Astra high A3; no provider-returned canonical model identity was exposed to this reviewer, so identity equivalence is not asserted.

**Recommendation:** use existing Git branches/worktrees, GitHub issues and PRs, the existing test consumers and requirement records. Run a small number of independent source lanes behind one integrator. Each lane must improve a named public behavior or supply a necessary semantic/circuit component with an explicit consumer. Review the full candidate and actual observations once at its integration boundary; do not review approvals of approvals. Keep source readiness, measured behavior and milestone acceptance separate. Keep all existing execution/resource/C4 boundaries; native authority remains zero. No replacement orchestrator is needed.

## What is causing churn

These are source observations followed by process inferences; they are not fresh runtime findings.

| Observation | Consequence and direct remedy |
|---|---|
| The September 8 post-mortem records a second recurrence: packet revision 04 held 201 hashes before the decisive production-path check; the resulting ledger driver returned `admitted-not-executed` and the decoder still threw. Its independent composition review found 50 of 56 corrupted candidates accepted despite 84 supplied passing tests. | Identity and local suite success displaced behavior. Before widening a task, trace one input through the real entrypoint and specify a complete independent expected result. The first repair must reach the missing operation or return the expected receipt; more packet coverage cannot satisfy that predicate. These numbers are historical, not reproduced here. |
| `docs/PARALLEL-DEVELOPMENT.md` already defines disjoint paths, shared ownership, exact-candidate review, two-cycle failure and administration limits. The initial six-lane manifest still pins an old base and historical roster. | The missing ingredient is consistent use, not another framework. Reuse the ownership pattern, refresh concrete tasks at current HEAD, and apply current user routing. Never automatically launch the historical manifest. |
| `ROADMAP.md` records real local components, conditional source adoption, UNRUN tests and unregistered/test-only Sherlock consumers; all full U exits remain open. | Build the missing consumer seam and complete required observations. A component source PR may merge honestly, but earns zero working-behavior credit until its scoped callable predicate runs; it earns zero milestone credit until all retained exits pass. |
| The October 14 crosswalk contains 160 requirement headings and explicitly denies that this is an atomic acceptance denominator. The audit plan has 69 top-level discovery IDs, likewise not a denominator. | Attach task predicates to existing clauses and scenarios; do not create percentages from those inventories, tests or PRs. Flatten the next milestone's mandatory predicates just in time, without first reauthoring the whole archive. |
| `crates/mori/src/commands/test.rs` is a real public manifest consumer that compares complete snapshots and ordered effects; `compile.rs` chooses distinct Core/1 and staged Core/2 paths. Node beta is another executable named `mori` and a separate profile. | Task evidence must name resolved executable, profile and consumer. Use the existing manifest format for independent finance cases. Do not transfer Node/Core/5 results to Rust/Core/1 or Core/2. |
| `.github/workflows/site.yml` is the only tracked workflow found. It builds/tests the website and browser journeys; it does not gate language changes. | Add a small ordinary source-check workflow as a future task after classifying actual calls. Preserve site CI; do not introduce a large native matrix. |
| Ordinary-looking tests include blocked calls: `mori-backend/tests/compile.rs` uses `ProofPreimage::check`, and `tests/proving.rs` includes a normal test calling `ir.model()`, `ir.k()` and model rows. Even Core has Midnight dev-dependencies. | Neither `cargo test -p ...` nor a filename containing “compile”, “normal” or “offline” establishes an ordinary-only check. Classify selected test bodies and dependency/build effects first. Preserve the Opus-owned boundary rather than recategorizing SDK work to make CI green. |

The dominant process error is fragmented delivery without an owner for the complete path. Independent review caught important defects; removing it would conceal the problem. The remedy is earlier behavioral checks, stable author ownership through repair, narrow integrations and fewer approval boundaries.

## Working unit and task template

Use one existing GitHub issue per independently testable behavior. Keep the following as ordinary Markdown in its body; no new schema, service or generated packet is required.

```text
Behavior: A developer using [resolved executable/profile] can [observable action].
Parents: existing U/M/MC/SP/G and exact requirement/scenario links.
Predicate: Given [input and premises], [public consumer] returns [complete result].
Owner / non-author oracle owner / fresh reviewer / integrator: actual identities.
Base: full commit; candidate: full commit and tree when ready.
Write set: exact files; read-only inputs: interfaces and source closure.
Interface: input/output types, semantic version, errors/refusals, caller premises.
Independent expectation: frozen fixture/derivation; no imports from audited evaluator.
Checks: inspected exact commands, test functions, expected exits and resource class.
Evidence needed: source / ordinary / model / proof / native / target, each explicit.
Dependencies: source prerequisites versus acceptance prerequisites.
Blocker: first missing operation, accountable owner, next action, exit evidence.
Done: local behavior and applicable review; parent exits remaining open listed.
```

A concrete next task can be **“complete public two-program Core/1 source-static correspondence walkthrough”**, linked to U0/U2/M1/M6/M10. An oracle author first freezes two structurally contrasting *admitted* source programs, initial states and complete observations; the implementation owner then drives existing parse/simulate/test/static-compile consumers. Division and staged continuation must not be smuggled into a Core/1 specimen. The invoice/reserve candidates in the existing audit plan are designs, not ready fixtures. If the required behavior is outside the grammar, create the named general grammar/lowering task; do not add application-specific host logic. Native U2 acceptance stays blocked. Security controls and signed-intent judgments belong to Opus.

**Ready:** named developer or component-consumer predicate; verified current production path; exact write set; independent expected values; stable interface or an integrator-owned prerequisite; exact ordinary check surface inspected; blocked tiers explicit; current author/reviewer routing known. A native-dependent acceptance claim does not stop independent source preparation. An unclassified execution path is not ready to run.

**Done for a source task:** implementation is called by the stated consumer (or a necessary component has an explicit downstream integration task), scoped applicable ordinary observations pass, independent expectations agree, full candidate and results have fresh required non-author review, integrated compatibility checks pass, and documentation states remaining limits. A source-only merge with UNRUN tests is “source merged, behavior unverified”, not task-behavior done. Documentation explicitly requested by the user can be complete as documentation.

**Done for a milestone:** all linked original acceptance predicates have exact current evidence and required reviews at the relevant source/profile/target. No source-task definition of done can weaken U0–U7, M1–M17 or MC/SP/G requirements.

## Ownership, interfaces and cadence

Scale independent source lanes to actual write-set separation, execution capacity and review/integration bandwidth. The user has removed artificial slot limits; the runtime supports 32 subagents. Three implementation lanes plus an independent oracle lane can be an optional initial work-in-progress experiment, but neither three nor the historical six-lane roster is a fixed policy or permission ceiling. Queue age and integration congestion determine whether to split or combine work. Source concurrency does not grant execution concurrency.

| Lane | Paths and interface contract |
|---|---|
| Core/consumer | Assign exact leaf command/test files and dedicated examples, following S1/S2 ownership patterns. Shared dispatch and Core representation stay integrator-owned. Document source profile, complete result shape and refusal codes. |
| Sherlock/model-to-component | Exact leaf files beneath `models/sherlock/` and `crates/mori-backend/src/sherlock_machine/`; no two authors edit the same machine module. Freeze state encoding, transition/fuel meaning, frame ownership and helper preconditions. Required Lean/Clean work stays in the existing Fenrir fork and must deliver its actual correspondence boundary. |
| Financial/lifecycle | Exact example/fixture and leaf test files, using S3/G2/G3 patterns. State units, rounding, gross/fees/net, ordered effects and residual duties; supplied time/oracle/finality remain named premises. |
| Independent oracle/conformance | Dedicated expectations and adapters in task-owned test support; author cannot import the system under audit. Validate frozen ACTUS/DeFi membership and complete fields. Security test design remains Opus-owned. |
| Integrator | Existing manifest `shared_files`: Core IR/parser/runtime/intent/JSON/replay and exports, backend/ledger shared relation and ABI, CLI routing/IO, Cargo manifests and lock, shared fixtures, current roadmap/spec interfaces. Reconcile proposed changes in small dependency PRs; no worker independently changes these contracts. |

A shared-interface change is an integrator issue with the affected consumers named. Prefer the smallest compatible extension and land it before dependent lane candidates. If semantics or cost changes, make the explicit profile/version decision required by the product contract. Do not force backend and ledger lowerings into byte equality: compare an explicitly documented common subset and its complete observations.

Proposed cadence: inspect ready work and blockers once at the start of the working day; integrate ready candidates at two short daily windows, one candidate at a time. A blocked lane switches to another eligible source task rather than producing another readiness packet. The integrator reviews queue age and interface collisions; use two waiting candidates per lane as a provisional congestion signal, not a hard cap. Adjust cadence and work in progress after measured throughput.

Use ordinary PR branches from a full base commit. Before merge, compare changed paths with the write set, inspect the actual full candidate, refresh affected consumers against the current integration base, and freeze the resulting tree for fresh applicable review. A conflict-free merge is insufficient. Combine source and available actual-result assessment in one substantive review request with separate verdict scopes. Component review never certifies composition. If an integrator changes candidate bytes or composition, that exact candidate gets fresh applicable review. No reviewer is assigned merely to endorse another review. Consequential design/resource decisions retain the existing two substantive agreeing votes; routine repair of approved behavior does not reopen that decision.

The present nine-reviewer planning exercise is not a new standing nine-reviewer implementation gate. Existing current implementation routing remains Sol high, non-author Muse high, and Opus for security unless the user changes it. Failed/unavailable reviewers do not approve or silently substitute. Preserve frozen native review rosters.

## One current board and the first two implementation PRs

Use one existing GitHub Project board or an equivalent issue list, with Backlog, Ready, In Progress, Review, Integrated and Blocked views. Store U0–U7 owner, exact parent IDs, behavior predicate and blocker owner on the issue. “Integrated” means source integration; the linked canonical roadmap retains acceptance status. Do not create another milestone database, automated scheduler or receipt workflow. Archive historical queue links without rewriting their evidence.

**PR 1 — make Core/2 source inspection report its actual qualification consistently.** The current `crates/mori/src/commands/inspect.rs` Core/1 branch emits a versioned `format`, `qualification`, and `open_premises`; its early Core/2 branch emits `status: accepted`, source structures and `stage_plan` without that explicit qualification envelope. Implement a documented source-only Core/2 report contract at this actual public consumer, preserving the existing Core/1 bytes and Core/2 target refusal. Assign only `inspect.rs`, focused cases in `crates/mori/tests/core_continuations.rs` or a new dedicated inspect test, and the relevant public guide. The integrator owns any shared report version/type changes. A new ordinary subprocess regression invokes the built Rust `mori inspect` on an independently authored staged program and asserts explicit profile, stage plan and source-only qualification, with no target or SDK call. Existing Core/1 compatibility and the Core/2 target-refusal path need the appropriate scoped checks. This is a concrete U0/M10 user-visible reporting fix, not a claim of new proving capability. Opus reviews any security-facing interpretation; this is a consistency observation, not an independently assigned vulnerability finding.

**PR 2 — deliver two independent public Core/1 programs and their end-to-end ordinary conformance consumer.** Add exact task-owned files under `examples/core/independent-walkthroughs/` and `crates/mori/tests/independent_walkthroughs.rs`, using the actual `mori test` snapshot/effects format and actual public source-static `compile` artifacts. The independent oracle author freezes one single-asset state/duty program and one two-asset, differently ordered effect program within the admitted grammar, including preserved state, fees and residual duties. The test resolves the Rust executable, runs the public consumers and checks complete output against those frozen expectations plus emitted action/ABI mappings; it must not compute expected effects through `mori_core::simulate`. Inspect the selected source-static call closure before authorizing these commands; native stages remain UNRUN. If an integration defect appears, fix only its production seam in this PR or an integrator-owned prerequisite, without adding a second compiler. Files in this PR are disjoint from PR 1. Done means useful runnable examples plus a real integration regression and current scoped observations, not merely a README of planned commands. This advances U0/U2/M1/M6/M10 child evidence; two ordinary examples alone close no U2 exit.

These are immediately actionable source tasks once implementation resumes. They do not reactivate the cleared goal or authorize execution during this planning review. The large native/Sherlock/Clean obligations remain on the critical path; these small PRs establish a reliable public baseline while those owners resolve their explicit prerequisites.

## QA tiers and minimal artifacts

| Tier | Evidence and timing | Limit |
|---|---|---|
| Source inspection and specification checks | Entry path, exact diff, requirements, types/interfaces and current closure. Existing `openspec/sprints/verify.py` and specification validators are candidate future ordinary checks after inspection. | Static observations only. This review ran none of those validators. |
| Ordinary local behavior | Named pure/reference/frontend tests and real CLI journeys against independent expected results. Apply exact test selection, locked dependencies, one build/test process initially, two Cargo jobs and one Rust test thread. At most two ordinary processes only after the existing capacity check. | No broad workspace suites, ignored tests, parameter environments, SDK checks/queries or native operations hidden in test helpers. Cheap work uses ordinary limits; it does not acquire proof budgets. |
| Semantic/model/correspondence | Explicit profile/field adapters; complete-state and prefix comparisons; K/Quint evidence and Lean/Clean obligations where required. | Scheduled separately under their actual execution classification; this planning task authorizes no model execution. Finite samples do not prove refinement. |
| Native and proof | Exact primitive completeness/soundness, relevant proofs, current independent verification and measured cost. | Existing admission only; authority now zero. Preserve all resource ceilings, counters, C4 and original failures. Opus owns security judgments. |
| Target and release | Full independently read effects for the claimed target, required stagenet path, separate Preview and Fenrir qualification, clean developer installation and two-builder evidence. | Target facts must be authentic and known. Source descriptors and local proof success do not establish deployment or another target's qualification. |

Keep the Git issue/PR as the work record, repository fixtures/tests as the executable contract, and existing audit dossier/evidence locations for results. A result links exact commit/tree, executable/build identity, profile, public input hashes, command and exit, expected/actual complete output, raw logs, target identity, scope and first blocked stage. Use the audit plan's existing implementation/evidence/acceptance axes. Preserve old outputs as historical; affected closures invalidate claims that depend on changed bytes. Do not regenerate unrelated passing historical prefixes merely to refresh a dashboard.

The evidence bundle need not duplicate every source file: Git owns source identity; input manifests and immutable actual output bytes establish the observed run. Native records continue using their existing required artifact forms. A GitHub check or PR approval links to evidence; it is not itself proof or a financial result.

## Failure and escalation rules

Apply the already adopted stop rules across the capability, even when task IDs or authors change. After two failures of the same defect class, stop the broad repair loop. Preserve the smallest input and complete expected/actual difference, exact source identity, failing public call and first divergence. The same author reduces the failure and changes the implementation strategy or task size before another broad run. A blocked reproducer remains specified-only; use source diagnosis and continue eligible work rather than crossing execution admission.

After two process-only cycles or thirty minutes of administration, the integrator selects eligible product diagnosis/implementation or reports the exact blocking dependency and owner. A new receipt, vote, resource allocation or longer timeout does not reset the trigger. Running necessary tests/proofs is not administration, and failures/costs survive branch or task changes. Return all substantive findings together to reduce serial one-comment repair loops. Distinguish a code defect, wrong oracle, unsupported construct, unavailable provider and external target dependency before choosing the remedy.

## Dependency graph and accountable blockers

```text
U0 semantic/profile/target contract -> required U1 basis -> U2 public source-to-target
U2 -> U3 lifecycle -> U4 history/private composition -> applicable U5 federation
SP07/SP08 accepted semantics -----------------------> claimed U4 successor scope
U2 + applicable U3/U4/U5 family interfaces ----------> U6 full conformance
U2..U6 + retained release predicates ---------------> U7 / V1
```

Preparation edges are weaker than acceptance edges: independent library examples, oracle derivations, tooling repairs and bounded Sherlock components can start before parent acceptance. U4 need not wait for every U6 family; full successor acceptance still needs its applicable accepted financial semantics and extension requalification.

| Blocker | Accountable owner and concrete exit |
|---|---|
| No complete current profile/grammar/consumer map | Integration owner with Core lead: reconcile actual callable consumers, exact grammar and refusal cells under U0; never reuse a historical roster as dispatch. |
| No general lowering/correspondence chain | Compiler lead: two positive contrasting programs through the same supported public pipeline plus rule/field coverage; required U1 evidence supplied by primitive owner. |
| Sherlock components lack full production consumer/correspondence | Sherlock lead with Fenrir Clean owner: complete one independently specified transition through reference, models, representation, emitter and declared consumer; retain every gap and unrun stage. |
| Native authority zero; R3 and later resource history retained | Existing execution/admission owner, with Opus for security: satisfy the existing exact-candidate admission, accounting and review prerequisites. No date or automatic escalation is assumed. |
| Unknown Fenrir/committee/registry/target tuple | Target integration maintainer: obtain authoritative current identities and qualified target evidence; M2/M7/M12/M17 remain explicitly blocked where needed. This is external evidence work, not a request to invent configuration. |
| Missing complete financial families | Conformance owner: preserve all 277 ACTUS fixtures/fields, 18 executable types, 32 taxonomy dispositions, DS01–07; all 72 DeFi rows, DA01–24, supplements, three held-outs, eight intent cases and eight regression classes. |
| No language PR CI gate; executable/profile confusion | Tooling owner: implement the minimal classified ordinary check job and explicit developer journey in existing tools; independent release owner later supplies U7 evidence. |

These are proposed accountable roles; the integration owner must assign a real person/agent on each issue. An unfilled owner is an explicit scheduling blocker, not implicit delegation to the user.

## Suggested sprints and useful metrics

Use bounded sprint commitments of one or two integrated public behaviors plus scoped supporting source tasks. Start with short iterations (proposed one working week) and revise from observed lead time. No fixed V1 delivery date is defensible while native admission and target facts are blocked.

1. **Current path and first contrast:** reconcile U0 profiles/consumers and independent complete expectations; fix one actual public CLI/compiler seam; assign target/admission owners. M2/M6/M10, with M1 groundwork. Add only the ordinary CI slice justified below.
2. **Basis and single-stage path:** implement required U1 primitive/caller obligations and complete U2's two-program source path. Pursue native certification/target chain only after existing admission. M1/M3/M5/M7/M8/M9, with M4/M11/M12 dependencies explicit. Sherlock provides necessary components and its required proof/circuit path rather than a parallel milestone system.
3. **Lifecycle discriminator:** U3's spend ≤11 A, fees ≤1 A, net ≥20 B; partial progress, recipient/document conditions, continuing duties, recovery and complete phase outcomes. M11/M13/M14; Opus supplies security requirements and tests. Local progress is separate from target exit.
4. **History and composition:** U4 two-step native history, private separate-party handoff and all retained sequential, disjoint-parallel, shared-state interleaving, atomic-sync and async operators; accepted SP07/SP08 scope inputs. Continue independent U6 source/conformance work in parallel.
5. **Federation and financial completion:** U5 exact adapter and two solvers, named policy/committee and applicable cost planes (M16/M17); U6 complete M15 and retained financial requirements, including full FeePool/Capacity Exchange and registry obligations from M4/M11/M12. Multiple iterations may be necessary; none silently drops scope.
6. **Independent release:** U7 clean public install, independently authored non-fixture program, own-wallet submission, two-builder reproducibility and every retained release predicate. Preview, Fenrir and requested stagenet evidence remain separately qualified. Full V1 requires the actual retained exits, not the arrival of this sprint number.

Track weekly: (a) distinct developer behaviors with independent expected/actual results on integrated source, (b) mandatory atomic exits newly verified, split by ordinary/model/native/target, (c) median ready-to-integrated lead time, (d) review wait and blocked age by owner, (e) repeat-defect/reopen rate and escaped integration regressions, (f) actual resource consumption under existing accounting. Report source-only increments separately. Start with a target of one integrated behavior per short sprint and calibrate after two; this is a planning hypothesis, not promised throughput. Zero native/target exits is the honest metric while admission remains absent. Never substitute agent count, PR count, test count or tokens for capability progress.

## Minimal future CI tasks

**CI-1, tooling owner:** add one ordinary language workflow using GitHub Actions and the existing scripts/tests. Scope initial paths to the actual affected frontend/pure reference closure. Pin the repository toolchain (`rust-toolchain.toml` requests 1.99.0) and locked dependencies; inspect dependency build scripts and selected test helpers before naming exact runnable commands. Capture logs and failing fixture IDs. Do not execute a default backend/ledger/workspace suite. If no existing suite separates ordinary behavior cleanly, make the smallest normal test-target separation as an explicit product QA task, subject to Opus's boundary classification. No such workflow or test change is implemented here.

**CI-2, integrator:** require the concrete ordinary workflow on affected PRs once its behavior is verified; keep the existing site workflow for site changes. Check the final integrated candidate, not only the lane base. Branch protection, GitHub remote state and Actions runner capacity were not inspected, so this is a proposed repository setting/task, not an observed control. Native/target QA stays separately admitted rather than an automatic PR job.

## Proposed EARS workflow requirements

1. **WF-01:** When a source task enters Ready, the integration owner shall record its observable predicate, existing requirement IDs, exact write set, consumer/profile, independent expectation owner and first acceptance blocker in its issue.
2. **WF-02:** While independent source tasks are active, authors shall modify only their assigned paths, and the integrator shall own changes to shared IR, ABI, result interfaces and build manifests.
3. **WF-03:** When a candidate requests integration, a fresh required non-author reviewer shall inspect its exact full tree, applicable requirements, independent expectations and actual available results, and shall state source, result and acceptance scopes separately.
4. **WF-04:** When composition changes a reviewed candidate, the integrator shall run the applicable admitted compatibility checks and obtain fresh applicable review for the composed candidate before merging it.
5. **WF-05:** When an operation reaches a native, SDK or target boundary, the execution owner shall require existing exact admission and preserve resource limits, counters and C4; absent admission shall block that operation while independent eligible source work remains available.
6. **WF-06:** When two cycles repeat the same defect class, the capability owner shall preserve a minimal reproducer and change the approach before another broad correction; when two process-only cycles or thirty administration minutes occur, the integrator shall switch to eligible product work or record the concrete blocker and owner.
7. **WF-07:** When evidence is reported, the reporter shall bind it to source, profile, consumer, target and observation scope, and shall keep implementation, evidence and acceptance statuses separate.
8. **WF-08:** When a U/M/MC/SP/G exit or V1 release is proposed as complete, the acceptance owner shall require every retained applicable predicate and required independent review; source merges, receipt ingestion and planned commands shall confer no downstream acceptance.

These EARS clauses are proposals for a bounded workflow amendment, not amendments already applied to canonical OpenSpec. They add no public developer permissions or new security policy.

## Inspection limits

`INPUTS.json` records inspected paths and hashes. The review used local files and Git metadata only; no web/GitHub API, private Fenrir checkout, actual target, build, test, model, proof or ledger observation was obtained. Large documents were read in focused sections as recorded. Historical evidence summaries were read, but the original September mutation suite and all retained native artifacts were not independently replayed. Graphify guidance was read; no graph was present at this checkout's standard location and no graph was created, in accordance with the two-output planning scope. No security finding is independently adopted here; Opus owns security interpretation and changes. Numerical throughput, runner capacity and end-to-end cost remain unmeasured. Existing C4/resource controls are preserved without reinterpretation; this report does not establish their current satisfaction.

Final coordination note: while this independent report was being written, the shared source worktree acquired an AGENTS.md change and a new planning assignment file from concurrent parent work. This reviewer did not write either. The evidence baseline remains the initial HEAD above; the input record identifies the original inspected AGENTS.md blob.
