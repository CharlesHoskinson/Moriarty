# Parallel delivery design for doctorwatson<UTXO>

Status: advisory design only, 2026-10-17. This is a task/ownership manifest proposal for direct native agent calls, Git and existing local DoctorWatson. It creates no scheduler, dispatcher, runtime registry or execution grant. No implementation or subject check was performed by this agent. Latest user steering authorizes building the bounded adaptation, then updating all EARS and auditing the remaining roadmap. This annex remains design-only; that authorization does not automatically rearm the broader Preprod loop or change native authority, which remains zero.

## Recommendation and stopping condition

Use a rolling set of small tasks, each tied to one missing EARS predicate, with separate source, execution and acceptance dependencies. Dispatch the largest useful set with disjoint writable files and available qualified owners. One integrator owns shared interfaces and integrations. A worker count is a ceiling, never a target: up to 32 workers plus root can fit this session, but actual readiness, reviewer capacity and physical limits determine use. Three requested designers do not imply three delivery lanes.

First deliver the actual EU-NAT-028 twenty-family reporting consumer and its ordinary checks. Stop factory expansion once an exact reviewed candidate emits all twenty families with independent source/admission/compiled/native/private states, retains unobserved states, distinguishes integrity from closure and invalidates affected current evidence. A failed product attempt may justify a narrow reporting repair; a desire for another dashboard, graph or queue does not. Then make the next integrated change advance a real current-Rust compiler/artifact/consumption predicate. EU-CON-003 diagnostics can accompany that change; they must not become another reporting-only batch.

These recommendations adapt the pinned Sherlock delivery loop [S1, S2]. Its current report metadata records ten adopted original clauses and twenty-five open originals, with all twenty-five refinements open [S3]. This is a useful example of distinct evidence axes, not acceptance transferable to UTXO.

## Names and identity registry

The registry below is a convention for fields in ordinary task descriptions and public result files, not a new database. All paths in examples are proposed unless already present. Existing authoritative requirement IDs remain unchanged.

| Kind | Example | Binding rule |
| --- | --- | --- |
| Display | `doctorwatson<UTXO>` | Exact user spelling; escape angle brackets only where rendering requires it. |
| Machine slug | `doctorwatson-utxo` | Lowercase stable prefix, no provider or model embedded. |
| Design path | `docs/utxo/doctorwatson/` | Existing draft and these advisory annexes. |
| Proposed entrypoint | `scripts/doctorwatson-utxo.sh` | Integrator-owned ordinary report command; never implicit subject execution. |
| Proposed canonical evidence | `openspec/changes/utxo-doctorwatson/current-evidence.json` | Single integrator writer; workers return patches or scoped row fragments. |
| Requirement reference | `utxo-original:EU-NAT-028` | Also bind normative path, clause digest and complete scenario identities. The prefix disambiguates inventories. |
| Scenario reference | `utxo-original:EU-NAT-028:scenario-01` | Bind exact scenario text/name and source digest; ordinal is an address, not semantic identity. |
| Task | `doctorwatson-utxo/EU-NAT-028/report-consumer/001` | Stable task ID includes precise child predicate; revisions do not replace lineage. |
| Attempt | `doctorwatson-utxo/EU-NAT-028/report-consumer/001/a0001` | Unique execution/review attempt, candidate fingerprint and start/end/terminal state. Never reuse after timeout. |
| Evidence | `doctorwatson-utxo/EU-NAT-028/report-consumer/001/a0001/e0001` | Immutable public output digest, producer, evidence class and scope; different result cannot reuse ID. |
| Review | `.../a0001/r0001` | Exact full candidate, independent expectation and actual results, requested/returned identity, scope and verdict. |
| Result path | `docs/results/doctorwatson-utxo/<task-token>/a0001/` | Public allowlisted receipts only; distinct attempts append history. |
| Branch/worktree | `doctorwatson-utxo/<task-token>` | Names are convenient locators, not authority or candidate identity. |

Use the supplied full UTXO baseline `1721457439dbd6d988d00c6c86d1cf648caf2a0c` and Sherlock pin `0aecf98455dd9abc1635801166151431cba309f7`. A future task must refresh its actual base; a branch named after this design does not pin its bytes. New Rust/Groth16/Preprod clauses require explicit versioned lineage under EU-CON-018. A task naming those technologies cannot silently relabel EUC-001's /5-/6 factories, CS01's profile/5 or CS10's Preview requirements [S4]. Retain separate 119-clause/238-scenario UTXO, 11-clause/16-scenario first-contract and 18-clause/38-scenario Midnight inventories; the aggregate 292 scenarios is an inventory size, not a completion denominator. Keep all twenty family IDs verbatim.

## Minimum task record

The companion JSON gives twelve concrete task examples and a proposed field contract. These are design records, not dispatch registrations. Each real assignment needs:

- Requirement references, complete scenario membership, exact missing predicate, real consumer and falsifiable expected observation. State what this child can close and which parent predicates remain open.
- Source/semantics/backend/target profiles and explicit applicability disposition; input paths and digests; exact allowed write files; base commit/tree; dependency closure and its known limits. No broad directory write grant.
- Author, independent oracle author, fresh result reviewer and integrator, with actual participant identities and requested/returned model/effort. The implementation author must not derive its oracle from the implementation output or review its own candidate.
- Separate source, check, review and acceptance prerequisites. A source-preparation task can be ready while its dependent execution and acceptance remain blocked.
- Immutable oracle version and independently derived complete normal envelope, frozen before subject output is observed. Financial expectations include affected/preserved state, ordered effects, gross/fee/net, work and remaining duties. If semantics are incomplete, record the ambiguity; never manufacture an expected value from the subject.
- Actual affected checks and their inspected side effects, tool/input pins, process/resource bounds, check attempt and raw result identities, failure layer and first divergence. Until inspected and authorized, `executionReady=false` and no runnable command is claimed.
- Full-candidate review scope, remaining gaps, adoption/currentness, first blocked boundary, responsible owner and observable unblock event. Separate statuses are necessary: valid JSON or a green report cannot prove a compiler, cryptographic or ledger predicate.
- Capability and defect-class lineage, inherited resource/counter references, attempts and terminal history. Splitting a task, changing provider or renaming a batch never resets stops, caps, failures or exhausted attempts.

Task records can live in a bounded JSON list beside the existing evidence record or in PR/task descriptions. Root makes explicit assignments with native agent calls and records ownership there. There is no watcher or automatic scheduler.

## Readiness, ownership and currentness

| State/axis | Meaning and allowed next step |
| --- | --- |
| `proposed` | Predicate or concrete file ownership not yet adopted; useful plan, no active write ownership. |
| `ready_source` | Current source inputs and semantics permit the scoped preparation, exact files are free, qualified owner is available and assignment is authorized. No check or acceptance permission follows. |
| `active_source` | Root has assigned one writer to each declared path at a pinned base. Other agents read these inputs but do not edit them. |
| `ready_check` | Frozen independent oracle, inspected exact command, current candidate, required prerequisites and bounded resource capacity all exist. This state is not reachable for native work under current authority. |
| `ready_review` | Full candidate and actual result bundle are frozen; independent applicable reviewer is available. Source-only reviews explicitly retain unrun behavior. |
| `ready_integrate` | Required source/result reviews apply to the exact integration candidate, and affected checks are current. Source-only integration is separately labelled and closes no unobserved behavior. |
| `blocked` | Named missing premise, owner and observable event; release active write ownership after preserving candidate. Unrelated ready work continues. |
| `integrated` | Source merged by the integrator; acceptance still depends on its exact evidence. |
| `cancelled` / `failed` | Terminal attempt; preserve outputs and counters, release writes. Later dependency edits do not reactivate it. |
| `stale` / `missing` / `current` | Independent evidence axis. Stale closure is excluded from current accepted counts; historical acceptance is retained at its original cut. |

An ownership reservation is a declared file set maintained by the root, not a lease service. Ownership transfer requires explicit handoff, clean output boundary and new pins. No two active writers share a file, including JSON evidence ledgers, generated indices, Cargo manifests/locks, IR, ABI, module exports, CLI dispatch or common result schemas. Workers propose shared changes; the integrator applies them. New leaf files can be written concurrently once their signatures are frozen. Test/oracle authors get separate paths and no implementation ownership. A worker that touches undeclared files stops integration until the owner resolves the overlap.

Security work and judgments go to Opus 5.5 or Opus 4.8 high under current checkout AGENTS.md, with Gemini then Muse only under the authorized observed-outage fallback. Do not solicit a new provider merely to erase a refusal. Actual identities and fallback reasons persist. Current ordinary authors are Sol high; current independent nonsecurity reviewers are Gemini 4 Argon high. Historical skill and pinned Sherlock reviewer tables have different rosters; they remain history. An unavailable reviewer gives no vote, and design consensus is separate from exact-result review. A security blocker applies to its consuming change, not every independent ordinary reporting, oracle or language task [S5]. This Astra annex supplies coordination design, not security acceptance.

## Twelve example lanes

The JSON lists exact requirement/scenario bindings, proposed exclusive files, dependency stages and success predicates. The table summarizes its intended concurrency. All lanes are proposed assignments; implementation authority comes from the latest user instruction, not this annex. “Ready preparation” means root can assign the scoped source task without waiting for unrelated security/native work once its exact file contract is adopted.

| Lane | Exact EARS and consumer | Parallel output and dependency |
| --- | --- | --- |
| L01 integrator | EU-NAT-028; EU-CON-001 | Freeze report interface, inventory/profile bindings and shared entrypoint/evidence wiring. Own all shared report files. |
| L02 reporting consumer | EU-NAT-028 positive/negative | Implement real twenty-family report from declarative public evidence. Ready after L01's small interface freeze, not after every product/profile issue closes. |
| L03 independent reporting oracle | EU-NAT-028; EU-CON-005 | Independently freeze expected family identities, statuses and normal complete report. Ready preparation in parallel with L01; no output-derived expectations. |
| L04 ordinary reporting regressions | EU-NAT-028; EU-CON-001 | Target real report with normal/all-open, missing/duplicate-family, changed-input and stale-child cases. Ready after L01 interface; execution needs L02 and frozen L03. Security interpretations stay outside this ordinary integrity test task. |
| L05 current-profile reconciliation | EUC-001; EU-CON-018; retained CS01/CS10 | Map legacy and current Rust consumers; propose explicit amendments without changing original clauses. Ready source work; security-dependent dispositions stay pending. |
| L06 real compiler/artifact increment | EUC-012 | Inspect existing Rust driver/export and actual artifact consumer, select one missing applicable artifact/publication predicate, then implement it in exact adopted files. Depends on L05 and file classification; protected relation/crypto edits belong to security owner. No helper-only closure. |
| L07 generated-consumer correspondence | EUC-013; EUC-014 | Prepare complete source-envelope/representation cases and the real spending-consumer path. Execution depends on current-profile mapping, L06 and the independently frozen oracle; generated-spend/VM/native work remains blocked. |
| L08 normal financial oracle | EU-FIN-001/010/019; EU-CON-005 | Independently derive complete two-program normal results and arithmetic boundaries from frozen semantics. Ready preparation; claims about authenticated gross/net or ledger state remain downstream. |
| L09 semantic/formal preparation | EU-CON-002/006; EU-FIN-024 | Map complete state/effect/work observations for K, finite Quint and Lean/Clean against the same declared common domain. Disjoint source maps may proceed; formal correspondence and actual model results are separate and unrun. |
| L10 ledger observation adapter preparation | EU-NAT-018; EU-LDG-027 | Specify public decoded-record adapter and independent complete readback mapping. Ready for source mapping; authenticity rules require designated security owner and actual settlement remains blocked. No wallet/config reads. |
| L11 security/claim preparation | EU-LDG-030/031; EU-CON-012/015 | Assigned security owner identifies applicable Rust/Groth16 claim/verifier/anchor obligations and blocked boundaries. Owner unavailable means this lane waits; other independent lanes continue. No setup/prover/VM/ledger execution. |
| L12 failure diagnostics and integration audit | EU-CON-003/010 | Preserve full failure objects in actual selected product consumer; fresh reviewer audits full integrated candidate/results. Implementation and review are separate actors. A diagnostic change rides L06/L07's product batch rather than spawning a documentation cycle. |

Twelve examples are not a ceiling. Split L08 by independently specified families, L09 by K/Quint/Lean preparation, and L10 by independent public record types when files and semantic interfaces genuinely separate. Keep hostile/security expectations under the designated security owner. Do not divide a single owned file or invent busywork to reach 32. Reserve review capacity before a burst of candidates; otherwise completed patches accumulate without usable product progress.

## First two real batches under the scoped build instruction

**Batch 1: EU-NAT-028 reporting consumer.** Root pins the actual base and assigns L01, L03 and bounded L05 source work concurrently; L08 normal oracle and L09/L10 public source maps can also proceed independently. L11 may prepare only through the designated owner. Freeze the small report interface promptly, then overlap L02 and L04. Root/integrator selects inspected local reporting checks, captures the real all-open report plus discriminating integrity cases, and gives a fresh reviewer the full candidate, frozen normal expectation and actual outputs. Integrate serially, refresh affected results/currentness, then adopt only the complete reporting predicate if warranted. Source, admission, compiled, native and private statuses remain independently open/unobserved where appropriate. The report must expose the next real compiler/product task, not select another factory feature by default.

**Batch 2: one actual current-Rust compiler/artifact acceptance increment.** Use L05 to adopt an explicit applicable child predicate or versioned refinement for EUC-012. Select its actual public consumer after source inspection; the present README says `mori-k2 compile` performs constraint synthesis, so its name does not authorize execution [S7]. L06 prepares the minimal consumer change while L07 prepares the downstream complete-observation adapter, L08 supplies frozen normal expectations, L09 semantic maps and L10 public readback maps. L12 handles exact diagnostic preservation inside that consumer and a separate fresh reviewer assesses results. Select only actually authorized checks. If the decisive check requires blocked subject execution, retain source-only integration and name that exact blocker; use remaining slots on independent product source/oracle work. Do not close EUC-012 from R4/hash-only tests or EUC-013 from helper comparison. L11's work can unblock only its explicit downstream dependencies. The batch cannot resume Preprod, manufacture native authority or claim settlement.

Root should spend the large majority of active slots on product source, independent expectations and discriminating checks after Batch 1. One short public row refresh per integrated candidate is sufficient. No requirement to rerun unchanged green cohorts or obtain repeated reviews of unchanged evidence solely to populate a dashboard.

## Full EARS and roadmap audit after the bounded build

The user now requests the build, then an update of all EARS and an audit of the remaining roadmap to completion. Complete this audit by covering every source member, mapping each remaining predicate to evidence or an explicit gap, and preserving parent exits. Do not interpret completed audit coverage as completed product capabilities. Source audit preparation can overlap the first build; publish reconciled current rows after the actual report consumer is usable.

Use the existing inventories as separate coverage ledgers. Root reports that it inspected `openspec/current-requirement-crosswalk.json` with 160 indexed headings and `docs/audits/2026-10-14/reconciliation/ears-clause-baseline.json` with 85 files and 388 original requirement lines. Those counts are root-supplied observations, not reverified by this advisory agent. Bind the actual current selected-tree files and hashes before audit, recompute exact membership and ordered scenario text, and show drift explicitly. Do not sum headings, original requirement lines, scenarios, families and exits into a single denominator.

| Audit shard | Exact retained inventory | Writer boundary and completion condition |
| --- | --- | --- |
| A-compiler | EUC-001–016, 16 clauses/32 scenarios | One scoped row fragment per clause in the compiler shard; all applicability, source/evidence/review/currentness and missing-action fields assessed. |
| A-ledger | EU-LDG-001–031, 31 clauses/62 scenarios | Separate ledger shard, with security judgments assigned to designated owner; no copied native passes. |
| A-finance | EU-FIN-001–026, 26 clauses/52 scenarios | Separate financial shard; all independent complete-oracle gaps remain explicit. |
| A-native | EU-NAT-001–028, 28 clauses/56 scenarios | Separate native shard; report public historical evidence only at exact target/candidate. No execution implied. |
| A-conformance | EU-CON-001–018, 18 clauses/36 scenarios | Separate conformance shard; retain formal, private and claim boundaries. |
| A-first-contract | CS01–CS11, 11 clauses/16 scenarios | Separate retained scope, including /5 and Preview; current Rust/Preprod needs explicit adaptation. |
| A-Midnight-PCD | 18 titled clauses/38 scenarios | Identify by normative path + heading + span digest when original ID is absent; never invent IDs that imply adoption. |
| A-general-EARS | 160-heading current crosswalk; baseline 85 files/388 original requirement lines | Split by explicit nonoverlapping source-path sets, not arbitrary word counts; reconcile line/heading memberships and documented amendments before declaring audit coverage. |
| A-roadmap | U0–U7, M1–M17, MC01–08, SP01–12, G01–24 | Root/integrator owns parent map; each parent links retained children, exact exit, present evidence and next remaining discriminator. No orphan original exit. |
| A-families | Twenty UTXO families plus retained formal/financial/composition families | Preserve existing membership: ACTUS 277 aggregate distinction, DeFi inventories, formal layers and composition operators; verify actual source membership before exact family-count claims. |

These are ownership shards, not a new scheduler. Use proposed public fragments `docs/utxo/doctorwatson/audit/<shard-id>.json` with one writer each. The integrator alone edits canonical evidence/crosswalk/roadmap summary files. Scope A-general-EARS so clauses already covered by an explicit UTXO/CS/PCD shard are linked by identity rather than independently rewritten. Within a shard, source extraction can split into disjoint fragments while semantic/security assessments keep their designated owners. Audit workers must report original IDs/text, exact current source scope, evidence pins, partial/unobserved/stale distinctions, accepted amendments and their next product action. They do not rewrite normative clauses merely to fit existing implementation.

Phase A freezes exact manifests and records mismatches. Phase B audits shards in parallel with reserved nonauthor review capacity. Phase C checks cross-links, duplicates, missing original lines/scenarios/families and transitive currentness, then integrates one coherent candidate at a time. Phase D runs the local report and reviews the complete audit, producing a finite ordered product queue: eligible ordinary predicates first, security/formal/native predicates carrying exact dependencies and responsible owners. Future subject execution remains governed by existing authority.

The audit is complete only when every source member has a disposition, every retained scenario is accounted for, all parent exits and families have mapped obligations, declared evidence has been checked at its actual scope, and every remaining requirement has either an actionable product step or a concrete blocker/unblock event. Unavailable required review means the dependent assessment remains pending; it cannot be counted as audited acceptance. Independent ordinary shard work continues. Do not repeatedly reopen a finished audit shard without changed source, evidence or a concrete discovered omission. After the audit, product work dominates; the next report update follows actual product progress.

## Integration protocol and invalidation

1. Pin actual integration base and exact file ownership before assignment. Record input closure, profile, family/scenario membership and oracle owner. Keep preexisting unrelated user changes outside the candidate.
2. Freeze the independent expected values before observing subject output. An ambiguity returns to the semantics owner, with prior oracle and failed result retained.
3. Author in an isolated worktree when useful; return a patch/commit and exact changed files. No worker commits or merges another worker's files. Review source and results at the complete candidate tree, not an excerpt or final diff alone.
4. Run only the affected inspected checks under existing limits, retain raw stdout/stderr/exit and first divergence, and bind each capture to candidate and attempt. Start ordinary checks with one process, two Cargo jobs and one Rust test thread where applicable; any move to two ordinary processes requires current capacity evidence under the existing rule [S6]. No source worker count multiplies those process limits.
5. Obtain the current applicable nonauthor source/result review. For mixed changes, security review covers its actual security dependency and ordinary review covers ordinary behavior; neither missing result is substituted with a design vote. A local report validates references, not reviewer authenticity or semantic correctness.
6. Integrator constructs one candidate against the latest integration base. After any merge/base/interface change, recompute affected transitive dependencies, rerun affected checks and get applicable review of resulting bytes and actual results before the next integration. Unchanged checked dependencies can retain evidence; a conflict-free merge alone proves nothing.
7. Publish through ordinary Git/GitHub with local checks. Integrator alone updates current evidence rows and exact pins. Feature-level currentness changes before any closure subtotal is reported. Shared schema/ABI/oracle/consumer/membership changes invalidate every consuming child, transitively; unrelated declared closures remain current. Unknown dependency scope is an explicit unestablished-closure limitation, not an empty dependency set.
8. Keep terminal attempts, failed/timeout outputs, historical acceptance and stop/cap lineage. A result arriving from a cancelled attempt cannot satisfy its replacement even if source bytes match. Reopen a stopped task only by explicit owner decision within unchanged authority, never by new name/provider.

The complete path remains requirement → missing predicate → disjoint owner + independent oracle → actual affected check → fresh full-candidate/result review → integration → row refresh. Neither a source-only merge nor an all-open report closes unobserved product behavior.

## Utilization and closure metrics without gaming

Measure per integrated feature, with original and added inventories reported separately:

- Useful active slots / eligible available slots, where eligible means authorized ready work with exact ownership and reviewer capacity. Show why slots are idle; never count blocked security/native tasks as ready or encourage filler tasks.
- Time from predicate selection to first discriminating result and to reviewed integration. Report review wait and integration wait separately from author/check time.
- Number of distinct complete clauses currently adopted, partial predicates advanced, stale clauses removed, and reopened defects. Report counts by evidence tier and inventory, never a blended percentage across 119/11/18 inventories or twenty families.
- Useful product increments per batch, repeated defect classes, first divergence resolution, wasted reruns of unchanged cohorts and administrative time. Preserve actual failed attempts in denominators; cancellation/renaming/provider changes do not improve success rate.
- Current scenario coverage with frozen expectations and actual relevant observations; positive and refusal cases both retained. A split scenario, duplicate fixture, test count, file count, model vote or worker-hour is not additional product closure.
- Check resource use when actually observed, plus unknowns; distinguish logical work, host memory/time, proof/VM costs and ledger fees. A source-only task cannot report zero proof cost.

Apply the existing stop rule to capability lineage and the integrated batch: after two failures of the same defect class, reduce the reproducer and change approach. After two process-only cycles or thirty minutes of administration, return to an eligible product action or publish the exact blocker. No administrative phase resets merely because another agent or batch owns it. Record a factory defect only when a concrete product acceptance attempt demonstrates it.

## Alternatives and unresolved choices

A wide rolling task set is recommended because normal oracles, source maps, leaf implementation and targeted tests can overlap while their acceptance dependencies remain explicit. Its cost is integrator/reviewer pressure; control that by smaller coherent candidates and held review capacity. A fixed three-lane wave is simpler but leaves independent useful work idle and confuses designer count with delivery capacity. A 32-author burst maximizes activity but risks shared-file conflicts and a review backlog; use 32 only if there really are 32 eligible roles, including independent reviewers and oracles. A new scheduler is excluded by the user's workflow.

Before implementation, root resolves the smallest concrete choices: exact leaf consumer paths after source inspection; L01's report interface; current-Rust applicability/refinement of EUC-012/013/014; actual available reviewer identities; and which inspected ordinary checks are permitted. These do not justify a global planning pause. The current draft's dedicated UTXO entrypoint is the default; reuse helper code only when the actual first consumer warrants it. Generalization, CI service, new task database and runtime model dispatch are out of scope.

## Sources and read limits

- **S1:** Sherlock `docs/DOCTORWATSON.md` at `0aecf98455dd9abc1635801166151431cba309f7`, read through `git show`; relevant full delivery/freshness/parallel sections. The source's roster is historical relative to current checkout AGENTS.md.
- **S2:** Sherlock `docs/sherlock/implementation-loop.md` at the same pin, selected delivery contract and next-action sections via `git show`. Output was bounded; no claim of exhaustive review of all linked artifacts.
- **S3:** Same-pin `openspec/changes/sherlock-completion-convergence/current-evidence.json` top-level/row-key/count metadata, `NEXT-ACTIONS.json` action IDs and selected action metadata, and `docs/results/sherlock-public-draft-compiler-20261018/current-evidence/RESULT.json` inventory/summary/assessment fields. Full evidence catalogue and underlying 639 KB report were not semantically audited.
- **S4:** [Existing draft](../DESIGN.md), [assessment](../ASSESSMENT.json), [source inventory](../SOURCE-INVENTORY.json), [EARS baseline](../EARS-BASELINE.json). Selected exact clauses/scenario names read for the twelve lanes; baseline metadata is not a new proof of each clause's semantics. Parent refreshed verbatim scenario spans during this design; final references bind normative source digests, not transient draft line offsets.
- **S5:** Checkout `AGENTS.md`, current October 17/16 routing and no-Foreman rules; installed `moriarty-dev:develop` skill and its execution-focus reference. Read-only status returned historical SP01 blocked dependencies; it was not followed or resumed.
- **S6:** Checkout `docs/PARALLEL-DEVELOPMENT.md`, selected ownership/resource/integration sections and bounded historical task manifest output. Current AGENTS routing overrides historical lane roster. `ROADMAP.md`, OpenSpec and wiki-llm entrypoints and product-contract excerpts establish retained scope; full roadmap acceptance was not audited.
- **S7:** `experiments/moriarty-cardano-rust/README.md`, first 95 lines, and bounded repository file-name listings of public Rust/Cardano source consumers. Implementation bodies, crypto requirements, existing tests and imported dependency graph were not audited; proposed code paths need exact inspection before assignment or execution.

No public web acquisition, model-provider call, subject test, compiler/prover/VM/ledger operation, wallet/key/PK/SRS/cache action or private provider/config read occurred. The forbidden external validation tree was not traversed. This advisory source design supplies no independent security judgment, current acceptance or runtime authority.
