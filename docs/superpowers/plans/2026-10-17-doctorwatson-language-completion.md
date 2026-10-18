# DoctorWatson language completion implementation plan

> **For agentic workers:** Execute this plan through the existing goal continuation and direct disjoint assignments with one integrator. Use the applicable development skills for each implementation task. Do not install or invoke Foreman, fm-session, Endstop or a replacement scheduler. The user has authorized concrete work and merging fixed PRs without another permission round.

**Goal:** Deliver useful supported Moriarty behavior through real consumers, and eventually satisfy every retained language exit with its actual scoped evidence.

**Architecture:** Follow [the delivery loop](../../utxo/doctorwatson/DELIVERY-LOOP.md) and the repository-native DoctorWatson commands. Separate independent oracle preparation, implementation, local observations, source/result review and integration. The prior goal was cleared and the new language goal is active; its existing continuation mechanism drives work. DoctorWatson is explicit local QA and reporting.

**Tech stack:** Existing Rust/TypeScript source profiles, Git/GitHub, Python local reporting, and separately admitted K, finite Quint, Lean/Clean, circuit/native and target checks. No new runtime framework.

## Global constraints

- Load `moriarty-dev:develop`, checkout `AGENTS.md` and plugin `status --json` in every fresh agent context and checkout.
- GPT-6.1 Sol high authors nonsecurity work; fresh non-author AGY `gemini-4-argon-high` reviews it. Security authoring/tests/judgments use Opus 5.5 or 4.8 high, with the existing observed-outage Gemini then Muse fallback. Record requested and returned identities.
- Native authority is zero. Preserve C4, k17 exhaustion, counters, terminal attempts and native resource limits. The prior broader Preprod goal's paused state is historical and that goal was cleared; the new language goal is active, with proof and target execution separately unadmitted. Inspect transitive command effects; no broad Cargo/native suite or compiler/SDK invocation is implicitly ordinary.
- Local CI only. `scripts/doctorwatson.sh` and `scripts/doctorwatson-utxo.sh` are explicit commands. `--host` and `--design` are distinct additional lanes requiring their applicable authority.
- Preserve all 119 UTXO, 11 first-contract and 18 Midnight PCD requirements/scenarios; all 69 U/M/MC/SP/G exits; ACTUS 277, DeFi 72 and full composition/lifecycle/private/recursive scope. Do not introduce a completion percentage or replace original predicates with operational issues.
- Shared interfaces/Cargo/module exports have one integrator. Current capacity is 33 including root; no six-lane cap. Parallel source slots do not grant execution concurrency.
- No private capture traversal, key/witness/provider read, wallet/network funding, proof, VM or ledger work belongs to the initial tranche.

## Task 1: Continue the existing goal and integrate the fixed PRs

**Owner:** Root/integrator. **Deliverable:** Reviewed actual integrated source, explicit source-only merge qualification and accurate existing publication records.

**Files and records:** Read `AGENTS.md`, `docs/DOCTORWATSON.md`, `ROADMAP.md`, `docs/utxo/doctorwatson/github/README.md`, `GITHUB-PUBLICATION.json`, `PR74-PUBLICATION.json`, `PR75-PUBLICATION.json` and the exact candidates they identify. Update existing publication/progress records only after observed actions. Own shared interfaces in the actual PR composition; do not assign their writes to parallel workers.

**Interface:** Consume current remote heads, integration base, full candidate diff, original review scopes and actual local outputs. Produce the composed commit/tree, affected dependency list, current merge-scope reviews and eventual merge identities. Historical author-reported macOS tests do not become current local checks.

- [x] Observe the new active language goal and its existing continuation mechanism; retain its identity. The prior goal was cleared; do not relabel its paused history as the current goal's state or falsely complete unfinished work. Record actual runtime status separately from local QA.
- [x] Inspect current Git heads and working state using `git rev-parse HEAD`, `git status --short`, `git diff --check`, and structured GitHub PR metadata. Existing receipt heads at plan time are PR74 `bd8bbfa2` and PR75 documentation source `26ec28af`; newer actual heads take precedence.
- [x] Compose PR74 and PR75 against the current base in the integrator's isolated branch. Preserve ordinary and five scoped security-source draft reviews already completed on PR74 tree `850405bf`; identify exactly what requires fresh merge/composition review. Preserve PR75's fourteen open gaps and runtime UNRUN status.
- [x] Run inspected eligible local checks for changed closure. Give fresh non-author reviewers the exact combined tree, requirements, independent expectations and actual outputs. Route substantive security deltas and merge judgments to Opus under current rules.
- [x] Integrate the fixed PRs when their actual source-merge predicates pass. If higher layers remain unrun, state `source-only integration; Rust/native/target qualification unrun` with the exact applicable scope. A failed current ordinary check is a defect to resolve, not a permissible inferred pass.
- [x] Record actual merge commits and readback in existing progress records. Keep historical intake snapshots and failed results unchanged; leave requirement gates open unless their exact predicates were observed.

Task1 source integration was observed on October18UTC: PR74 merge `7a25a75e6074ac6c9728864dd9b874bd00ef564e`, PR75 merge `baf61ebd0cb49f2c4441cb7fa8de53b343e056dc`. Full receipts and review scopes are in `docs/utxo/doctorwatson/github/merge-20261017/` and `evidence/utxo/doctorwatson-language-loop-2026-10-17/`. These checked delivery steps do not close language, Rust-runtime, native or target exits.

## Task 2: Prepare and implement the next actual publication/consumption slice

**Owner:** Nonsecurity consumer author plus integrator; independent oracle owner; Opus for substantive identity/authentication/security decisions.

**Requirement:** RCU-001–004 as additive source proposal, linked original EUC-001/002/008/011/012/013/014/016 and relevant EU-CON/EU-FIN clauses. Original generated-Aiken/spending exits remain separate.

**Existing consumer and owned source:** `crates/mori/src/commands/compile.rs::run`, `publish_artifact` and `publish_staged_artifact`; `crates/mori/src/commands/proof_io.rs::publish_new`; the existing `Artifact` decoder in `crates/mori-backend/src/lib.rs`; and publication tests in `crates/mori/src/commands/compile.rs` and `crates/mori/tests/check_compile_io.rs`. The production compiler already publishes circuit leaves and a final `artifact.json` completion marker. Preserve that actual call path and its no-clobber/order behavior; do not create a replacement exporter/helper and call it delivery.

**Requirement sources:** `openspec/changes/utxo-doctorwatson/route-reconciliation.md`, `docs/utxo/doctorwatson/audit/compiler-finance.md`, `compiler-finance.json`, `audit/ledger-native.json` and `docs/audits/2026-10-14/AUDIT-PLAN.md`. Inspect `crates/mori/src/commands/inspect.rs` and `crates/mori-ledger/src/relation/compiler.rs` to retain the separate compile/synthesis and Cardano boundaries. Locate legacy `compileCustody` from the audit's pinned public locator before editing; unavailable external source is a blocked dependent task, never a reason to traverse private captures.

**Interface:** Consume a declared source/semantics/target profile and explicit published artifact identities. Produce an isolated complete success manifest only on successful publication; downstream consumers refuse incomplete/mismatched manifests before claiming a usable artifact. The designated security owner determines relation/VK/statement/anchor adequacy; digest equality alone does not establish it.

- [ ] Trace `compile.rs::run` → existing publication functions → `proof_io::publish_new`, and the `mori-backend` artifact decoding contract. Record that `compile_reported`, `compile_staged` and existing unit-test `artifact()` fixture calls to `mori_backend::compile` reach held work. Do not invoke them or their current tests.
- [ ] Prepare the first source-only patch on those existing publication consumers/tests: replace the publication unit tests' compile-dependent artifact construction with independently specified frozen artifact values where valid, and retain checks for every leaf failure, original refusal, no-clobber behavior and final completion-marker ordering. Keep the actual production publication calls intact. Inspect decoder side effects before proposing any bounded runtime check; no Cargo, compiler or decoder execution is admitted by this task. This is test-source/consumer preparation, not an observed behavior pass or a newly discovered production bug.
- [ ] Have a non-author freeze two contrasting program inputs and complete financial/work envelopes before candidate execution. Start from the audit's proposed installment invoice and conditional reserve allocation; check admitted grammar first. If unsupported, preserve the refusal and implement the missing ordinary language support as a separately bounded task; do not replace it with host logic.
- [ ] Freeze positive publication/consumption expectations and refusal cases for partial publication, stale dependencies, wrong statement encoding, wrong target and unsupported staged profiles. Preserve original diagnostic objects and explicit absence of a success artifact. The independent owner derives expected values without importing implementation code.
- [ ] Add narrowly scoped failure tests at the actual consumed publication seam. Specifically preserve the legacy `compiled=true/checkOk=false` case for reproduction under its original scope; source inspection is not its reproduction. Do not rerun terminal R4 RED attempts or claim their 17 missing-implementation failures are resolved by a new wrapper.
- [ ] Make any requirement-supported publication/decoder changes in those existing consumers, with independent expected artifacts and exact profile/refusal semantics. Keep relation synthesis, SDK, VM and proof calls outside any proposed bounded check. The initial deliverable remains source-static; if a check lacks applicable admission, report the exact first blocked boundary and continue another ready source increment.
- [ ] Run only admitted ordinary checks; retain first differing path, complete original output and attempt identity. Review exact source and actual results independently. Integrate only after current candidate/base review; retain higher-tier UNRUN gaps.

**Falsifiable result to observe once separately admitted:** The existing publication consumer never emits success after a failed step, preserves existing destination bytes and writes its final completion marker only after complete leaves; the actual artifact decoder preserves its defined identity/refusal contract. Initially deliver only the source patch and independent test fixtures, with runtime UNRUN. Two actual program walkthroughs and complete artifact-to-Cardano consumption remain open until run through the same claimed pipeline. This task cannot itself close general U2 or EUC-012/013.

## Task 3: Advance ready language/tooling work in parallel

**Source:** `docs/utxo/doctorwatson/github/work-items.json` retains all original issue predicates and links. Each author consumes the full original issue and applicable requirements, not just its title. Assign disjoint exact paths after reading current files; the integrator alone changes shared APIs and Cargo files.

| Work package | Concrete consumed result | Dependency boundary |
| --- | --- | --- |
| #76 profile decision | Explicit Core/beta/Sherlock grammar/representation/consumer choice, with reasons and separate unsupported surfaces | No implicit frontend unification; decision blocks its migrations, not independent fixtures/tooling |
| #80/#82 CLI/library and diagnostics | Existing public inspect/test/init/call logic consumed through libraries; source spans/help and non-source cause chains survive CLI output | Coordinate PR75 and aliases/output schema; preserve security error contracts |
| #84/#86 fixtures and oracles | Portable canonical original fixture bytes; actual Sherlock tests deserialize independent JSON rather than generated expected Rust literals | Agree fixture location with #79; never regenerate expected data from actual output |
| #85/#87 tests and robustness | Library consumers retain report/exit/I/O coverage; original frontend consumers get source-bearing diagnostics and nonpanic properties | Final diagnostics depend on #82; inventories and independent properties proceed now; security hostile-test design stays Opus-owned |
| #79/#81/#83 source boundaries | Cargo-owned adopted Sherlock, explicit native-feature surface and profile-aware Core simplification | Current exact behavior and refusal envelopes preserved; shared exports integrated once |
| #77/#78/#88 identity/lowering/evidence | Exact artifact identity and migration proposal, lowering/admission map and env-hook consumer inventory | Opus decides substantive security/control changes; no source-hash or lowering shortcut |
| #23 finance usability | Paid/owed/rate/date/rounding vocabulary; dated chained scenarios/CSV; trust/invariant display; deterministic agreement; checked templates; observed finance-user study | Prepare existing public-output consumers now; U6 semantics and actual 3–5-person observation remain separate dependencies |

- [ ] Select only ready disjoint consumer work; record one callable predicate and exact path owner per assignment. Use available session capacity, reviewer capacity and host resources rather than a fixed lane roster.
- [ ] Freeze independent financial/work/refusal expectations for each applicable change before executing it. Treat formatting-only fixture relocation as preservation of original bytes/provenance, not fresh semantic certification.
- [ ] Implement through the actual consumer, run inspected authorized checks and obtain fresh exact-candidate/result audits. A source-only task reports its unobserved behavior explicitly.
- [ ] Integrate one candidate at a time, refresh affected dependency results and review after shared changes, and retain original issue scope in #89. Do not close #89 until every #76–88 child meets its complete original predicate.

## Task 4: Validate the workflow design without confusing it with product evidence

**Files:** `models/doctorwatson/README.md`, `models/doctorwatson/doctorwatson.qnt`, `doctorwatson_test.qnt`, `doctorwatson_coverage.qnt`, `doctorwatson_coverage_test.qnt`, `openspec/changes/doctorwatson/requirements.md`, `scripts/doctorwatson.sh`. Consult quint-llm-kit for any model review/change.

- [ ] Review the existing finite workflow model against exact candidate/attempt freshness, transitive dependency invalidation, disjoint ownership, fail-stop local CI, source-only integration and retained terminal dispositions in the delivery design.
- [ ] Preserve previously observed model results as historical results. Record any uncovered workflow predicate and prepare source changes separately from execution. Do not execute `--design`, Quint/model commands or other held lanes during the initial source-only tranche.
- [ ] When a separately permitted workflow check is actually run, retain exact source/attempt, command, limits, trace/output and independent review. Mark only its finite workflow predicate observed; it establishes no interpreter, circuit, native or ledger result.

## Task 5: Continue from useful slices to the original full exits

**Files and evidence:** `ROADMAP.md`, original EARS/scenarios, `openspec/current-requirement-crosswalk.json`, UTXO `current-evidence.json`, `docs/utxo/doctorwatson/audit/roadmap.md`, existing correctness-loop matrices and original ACTUS/DeFi membership records.

- [ ] Use U0 profile/consumer/semantic/target reconciliation, the required U1 proof/intent interface and two contrasting useful U2 program consumers as the dependency spine. Select ready independent U3–U7 source preparation concurrently instead of waiting for whole milestones.
- [ ] Carry complete state/prefix/effect observations across source/Core, K, the declared finite Quint projection, Lean/Clean and circuits at their exact common domains. Keep witness generation and constraint enforcement separate; never count an absent layer as agreement.
- [ ] Preserve all five composition operators, successor split/join/migration/termination/conflict behavior, private handoff, recursive/multi-parent history, universal certificates, cumulative fees/work and residual duties. Give each an original requirement/scenario and actual consumer.
- [ ] Complete ACTUS and DeFi against frozen original membership and field coverage, including held-outs and regressions. Keep the twenty original UTXO family mappings separate from invoice examples and aggregate finance scores.
- [ ] Prepare exact target contracts and public nonsecret interfaces while execution is held. Native proving/verification, genuine authenticated claim issuance/consumption, C2 spend/release, VI/VS1–VS4 shard path, whole-transaction fit and actual settlement require their existing separate admission and actual observations. Do not manufacture Fenrir identities or merge Preview/Preprod/Midnight results.
- [ ] Demonstrate the original external developer installation/authoring/compile/prove/deploy journey on Midnight ZKIRv3, with independent valid alternative plans and required tamper refusals, before closing release exits. Maintainer metadata never becomes a public developer prerequisite.

## Per-cycle evidence and stop conditions

Every cycle records requirement/scenario, consumer, candidate commit/tree, integration base, consumed digests, current attempt, independent oracle, command/tool/bounds, exact outputs/exit, review identities/scope, first divergence, implementation/evidence/acceptance status and next consumer action in existing records. No new registry is needed.

- [ ] After two failures of one defect class, reduce a reproducer and change approach before retrying broadly; preserve failures and original expected values.
- [ ] After two process-only cycles or thirty administration minutes, return to eligible consumer diagnosis/implementation. Apply the limit across renamed tasks, delegated work and integrated delivery.
- [ ] If all eligible executions are held, continue ready source/fixture/consumer preparation and name the exact held predicate and owner. A source-only merge remains source-only.
- [ ] Mark a requirement gate closed only on its actual original scoped evidence; mark the goal complete only when all retained full exits pass. A plan, report, workflow-model pass, merge or inventory check cannot finish the language.
- [ ] If the same external blocker repeats for at least three consecutive goal turns and no meaningful independent progress remains, follow the host rule to mark the goal blocked, retaining the unfinished obligation. A resumed blocked goal starts a fresh three-turn audit. Do not mark the goal complete because work is blocked, time is short or ready tasks are temporarily absent.

## Initial verification and unresolved decisions

For the two design documents themselves, use `git diff --check`, inspect references and compare all retained inventories and routing against the named current sources. These checks establish document consistency only. The plan intentionally does not run Cargo, compiler, model, SDK, native, wallet or ledger commands.

The Core frontend decision, artifact identity/migration contract, lowering common subset, fixture ownership and authenticated claim adequacy remain owned decisions. Consequential design/resource choices still need two substantive agreeing votes; independent authorized source tasks continue while those votes or dependent execution are missing. Exact merge readiness and observed goal arming belong to the root integrator's current evidence, not this document's existence.
