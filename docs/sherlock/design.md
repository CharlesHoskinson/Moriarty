# Sherlock completion design — revision 03

Current state: see [Implemented design as of 2026-10-25](#2026-10-25-implemented-design-by-group) at the end of this file for what has landed per group; the sections between are the 2026-10-17 reconciled design and stay as history where later sections change them.

Superseded 2026-10-24: review routing, adoption votes, consumed-attempt and no-retry rules, permanent lanes and campaign queues in this file are retired. The process is the factory loop in [AGENTS.md](../../AGENTS.md); technical descriptions below stand as history of the legacy tooling.

Date: 2026-10-17. Status: reconciled design after ten Gemini Argon HIGH research reports and three Astra MAX reconciliation reviews; finite Quint execution and five Astra HIGH final readiness passes are separately recorded prerequisites to the newly requested implementation loop. Baseline public source: `628615a94d8a74837a1af510293927bb74d5c2b5`. The old automatic goal remains historical and CLEARED. The user now requests a new completion loop after this design, EARS/OpenSpec and readiness checks. Root arms that new goal explicitly; this document changes no native/security authority.

## Outcome and retained contract

Finish Sherlock as Moriarty's general interpreter and its handwritten Midnight realization: a developer supplies a new supported program through the public language tools, observes deterministic complete execution, produces the required native evidence, and deploys/calls on the qualified target. A typed-library literal demonstration is an intermediate result. Retain the canonical Sherlock OpenSpec, all 35 UIREQ requirements, full39 events, B helper/ADT/financial requirements, K/Quint/private Lean-Clean stages, U0–U7 and M/MC/SP/G exits. The roadmap traceability file maps these existing requirements; the following contracts do not replace them.

The target is Midnight ledger-9 for stagenet. Preview and Fenrir have separate qualification. No new `verifyProof` primitive or alternative proving backend is assumed. Actual supported native recursion and ledger behavior govern implementation. Lean/Clean remains a required project implementation/correctness stage; it is not a public Moriarty developer dependency. Private fork code stays private.

## What the audit establishes

The A host has 36 charged and three terminal semantic events, recursion/calls/frames and structural values. B owner/admission/row-family code and a Core scalar consumer exist. These are substantial reusable implementations. Their public source-language CLI connection, complete generic helpers/ADTs and financial/lifecycle lowering remain incomplete. `project_a` exists as a private test observation helper; a production/native canonical projection is not established.

A102 Literal, A80 Return, A82 Error and A84 fuel exhaustion have handwritten source. A82/A84 are standalone; the composed positive-fuel literal path does not add general39 dispatch or zero-fuel startup. PR168 merged the composed public-statement and typed ledger consumer source into sherlock at 011284561884346412a7c7eaa36a195310924878; SDK compilation remains unrun. Separately, the integrated DoctorWatson batch passed 241 Python tests and actual Rust stream compilation, eight tests and the independent216-run comparison. The actual SDK-free composed capture matched all333 noncontext values in named and ordered forms. Its initial lexical dep-info check failed after successful compilation; the preserved continuation normalized the paths to the exact ten-member set and reused that artifact without recompilation. Independent ordinary result review approved this scoped publication. Both context limbs remain unbound; the checked335 bridge/circuit/native path was not executed.

K has one actual N01/fuel1 engine case, not zero and not216 cases. Quint has actual earlier bounded runs and separate current frontend-only terminal-source checks. Pure Lean observations, Clean components, handwritten constraints, native verification and target settlement need separate evidence. Existing financial host comparisons are not Sherlock financial lowering. Full-B N01/full96, complete PassT expectations and omitted exhaustive cases remain open.

## Alternatives and recommendation

1. Keep adding isolated rule leaves. This reuses code but leaves public integration, shared-state chaining and evidence convergence late.
2. Replace the interpreter/backend with another zkVM architecture. This adds migration and correspondence work and does not establish Midnight acceptance.
3. Converge existing A/B components around explicit profile adapters, the existing profile-specific state/record contracts and explicit comparison adapters, a reusable bounded machine relation, and one public developer pipeline. Recommended: it retains tested work while making missing consumers and evidence concrete.

Convergence means shared interchange and verified adapters. A-clock semantic events and B helper microsteps remain distinct until their refinement/prefix relation is established. A source hash, common JSON shape or a matching final value cannot unify those profiles.

## Component contracts

**Program and admission.** Reuse existing ProgramImage, declaration/type and bounded-image machinery. The public parser/checker must feed the same admitted program representation consumed by the interpreter and witness path. Specify supported grammar rule by rule. Preparation refusal, runtime rejection and unsupported lowering have distinct results. Source-specific host branches are not a general compiler. Program/profile identity must be stable under the existing specified canonical encoding; any missing identity contract is an explicit prerequisite, not a guessed new hash algorithm.

**Deterministic execution.** Keep a reference realization with explicit Program + complete State -> StepRecord/successor/no-next. Events retain source rule, profile, work charge, fuel, frames/control and all preserved arenas. `visit_steps` must expose the same ordered records as existing execution on their admitted domain, including the free terminal publication and terminal/no-next behavior. Streaming must not change evaluation order or work.

**Representations and adapters.** Every adapter declares its numeric domain, table capacities, padding, optional-value convention, clock and projection. Current Step-mode Constant-space Done and canonical Data-space observation remain different representations. Complete projectA/F3/F4/F5/StepInv/G4 obligations remain mandatory before canonical/native claims. No host normalization silently supplies a missing circuit condition.

**Reusable handwritten realization.** Build on existing fetch, scalar, representation and terminal fragments; supply actual dispatch/helper implementations for every admitted full39 event. Module/call ownership is disjoint; shared ABI and dispatcher integration has one owner. Normal witness agreement is one check; designated security/constraint obligations remain separately required. B helpers need complete declared preparation/guard/admission/cost and runtime application, not just row-effect function bodies.

**Bounded runs and continuations.** A proof chunk records a contiguous prefix with exact input/output state, program/profile identity, work/fuel and residual duties. Resume must consume the exact prior output on that same profile. Cutting a proof is not an application Yield/Await or financial commit. Terminal publication remains separate from charged work; error and exhaustion priority must match the normative profile, including initial zero fuel. Never infer a complete run from a prefix or an exhausted resource bound.

**Finance and composition.** Preserve ordered effects, affected/preserved state, gross debit/fees/net receipts, reserves/dust, liability evolution and pending duties. Financial commit/await/recovery boundaries are explicit language/ledger transitions. All five retained composition operators and genuine native/private multi-parent requirements stay in scope. The missing independent PassT result is either supplied completely or the comparison is explicitly B/F-only.

**Midnight adapter and public tools.** Isolate pinned SDK key/proof/deploy/call/ContractState interfaces from the SDK-free interpreter. A public command must reach the actual source/admission/execution/ledger consumers. Raw witness JSON and a typed literal-only constructor are different interfaces; neither becomes a general source compiler by relabeling it. The first target result binds actual compiler/relation/key/statement/network/transaction identities and complete supported state/effect/fee readback. Chain authenticity/freshness and funding remain explicit established prerequisites.

## Scoped development sprint completed before the implementation loop

S1: add a DoctorWatson SDK-free stream realization using real `Machine::visit_steps` and the existing independent normal corpus. Compare full ordered records with the existing batch realization; retain all prefixes, terminal records and no-next. Publish measured results only after inspected ordinary execution.

S2: review and execute the already-authored minimal real A102->A80 host capture, then compare all333 noncontext carriers against the independently frozen expectations. The capture exports the existing adapter's actual images; it is not a second independently encoded semantic post. Compare both named and ordered payloads via explicit extraction. It does not test the checked335-lane bridge or bind context.

S3: improve DoctorWatson's actual local QA and source-reference reporting using its existing runner. Add useful diagnostics and tests, not another scheduler or approval framework. Research-backed fixes must identify their real consumer and actual test coverage. Preserve all historical failures and private/native boundaries.

S1 and S2 have the exact observed scoped results above. S3 source reporting and helper closure are integrated and passed the241-test ordinary run; the final three-document repair changes no executable behavior. Do not requeue these unchanged passing closures. These improvements do not close any downstream native or full-interpreter requirement. A discovered implementation defect is fixed within its actual owner/design scope or recorded for its designated owner.

## Completion sequence and parallelism

The finishing roadmap is the dated companion and retains every parent exit. First-demo serial path: reconcile reviewed source and actual ordinary observation -> admitted SDK build/tests -> native proof + independent verification -> qualified stagenet deploy/call -> complete readback. General39 dispatch, B integration, public compiler, independent expectations, K/Quint/private Clean correspondence and complete financial implementation proceed in disjoint lanes and converge before full v1 release. Shared schemas, canonical adapters and final integration are serial decisions.

Current native authority is0, C4 OPEN, R2 UNSATISFIED and60460 HOLD. Native compilation has not been attempted. The existing source-author context contract and incomplete Amendment10 are recorded blockers; this proposal neither retries/reroutes that refusal nor relaxes controls. A source-only admission issue must not be described as a demonstrated compiler defect. Work possible without that dependency continues under existing scope.

## Research-informed next-version ideas

Alive2's per-transformation translation validation suggests checking each optimized residual realization against its declared baseline and bounds, rather than treating a source/IR digest as equivalence. This is an architectural analogy, not an LLVM tool directly applicable to Sherlock. Its boundedness must remain visible. [Alive2 PLDI2021](https://users.cs.utah.edu/~regehr/alive2-pldi21.pdf).

Informal Systems' model-based testing suggests carrying concrete Quint traces into real implementation replay. Reuse the existing StepRecord adapters and independently expected fixtures; model-generated traces exercise correspondence without becoming an independent proof oracle. [Model-based techniques](https://mbt.informal.systems/).

RISC Zero's execution/segment/receipt layering is useful vocabulary for separating chunk execution from whole-run evidence and final target acceptance. Borrow the separation, not its STARK/Groth16 backend or proof-verification API. [Proof-system overview](https://dev.risczero.com/proof-system/).

Current v1 already requires B-T07–09/UIREQ-031–035: separate constraint/witness plans and export report, earliest-stop slices at widths1/2/4/8, bounded residualization with generic fallback, declaration-driven matching, and deterministic uncached witness dataflow. Existing owners retain these obligations alongside the mandatory private Clean stage. Future performance experiments may vary last-use storage or specialization strategy only after the required baseline exists. Build provenance and semantic identities remain distinct concepts; their concrete existing-or-missing encodings are explicit implementation tasks. Typed user diagnostics remain part of the developer tooling contract. Benchmark complete-path time/memory and preserve exact work/prefix semantics before adoption. Required ADTs, helpers, recursion and finance are current scope, not deferred future features.

## Review and validation disposition

Ten actual AGY Gemini4ArgonHIGH research reports were independently reconciled by the three AstraMAX architecture, correctness and delivery reviewers. Their accepted/rejected/deferred decisions and exact refinements are retained in the three review annexes. Root read all three full design-addition packets before integration. Research proposals and foreign-system analogies do not establish Midnight features or measured costs. Full source/citation limitations remain in the decision registers.

The finite Quint contract below validates selected interpreter/observer/helper/boundary/financial lifecycle cases after reconciliation. Its actual commands, independent literal expectations, source membership, directed coverage, negative functional controls and seeds/bounds are reported separately. Missing model components are explicit gaps, not passes. A finite sampled pass cannot establish full39/full96, UInt128/field encodings, parser lowering, K/Clean/circuit correspondence, cryptography or deployment. Five additional AstraHIGH reviewers audit this exact design, EARS/OpenSpec and completion-loop contract before rearm; conditional verdicts do not promise absence of future blockers.

## Mandatory architecture refinements from the three Astra reviews

This section is normative for candidate02 where earlier general prose is less specific. It incorporates A1–A6, C1–C4 and AD1–AD5; original reviews remain immutable and their conditional votes are not rewritten as unconditional implementation acceptance.

### Profile and observation adapter matrix

Keep `sherlock-k-a/1`, frozen `sherlock-quint-a/1` and `sherlock-helper-microstep/1` separate. Each adapter must name numeric/symbol domain, frame direction, raw control/outcome/option tags, origin metadata availability, capacities, work clock and actual producer/consumer. A Snapshot currently stores frames top-first; proposed common ReferenceState stores them bottom-to-top and is not a production shared ABI. B's StepEventC sink is not a complete before/after snapshot; complete comparisons need its actual owner-bound image materialization. Missing provenance remains absent. Free terminal publication is an A/B property, not a property imported into frozen Quint-A. Pending/None is outside the A observation domain. A completed B helper interval erases to its corresponding A event only on the declared common domain; incomplete helper prefixes erase to the saved A origin. The conditional completed-interval work relation `W_B = W_A + H_B` assumes the retained common-domain/no-capacity-failure premises and implies no equal-fuel behavior.

### Concrete public compiler increment

Integrator I owns `crates/mori/src/cli.rs`, `commands/compile.rs` and current backend/source/stage dispatch. The public `mori compile` path currently calls `source_projection::compile_reported`; its JSON is scalar correspondence metadata attached to the ordinary Core artifact. The existing B `core_action_consumer::run_scalar_action`/bounded scalar source adapters are reusable host consumers, not public generic CLI admission.

The proposed increment extends that same compile command with an explicitly selected Sherlock execution profile (option spelling/schema to be frozen in its OpenSpec before implementation), action/entry selection and a rule-by-rule checked-source-to-program-table map. Preserve the public Rust grammar/version and separately identify source/Core/execution profiles. The path must produce a real admitted program plus separately identified witness and constraint/export artifacts, then pass those to actual Sherlock/backend/ledger consumers. Artifact shape and unsupported dispositions are part of that increment, not guessed existing APIs. Promote the recursive source/type draft only after reconciling its grammar with that public version. Two independently expected, contrasting newly authored programs use this same path; no fixture-specific branch or evaluating a source action into a manufactured literal proof. Source preparation can proceed; generic caller/CLI execution still needs the existing designated-owner H1 disposition. Public Moriarty use remains permissionless; these internal development controls never become developer approval requirements.

### Separate A/B native implementation maps

For A, map each39 event and L:106 precedence/no-next to selector, ordered inputs/outputs, read/write/preserve sets, witness producer and actual handwritten/Clean consumer. Missing rows return explicit unsupported capability; they never fall through to the literal emitter. For B, separately map owner selection/family plans, entry/iteration/exit guards, cursors, raw counters, capacity behavior and actual runtime application. Its HostOnly carrier is not a bounded native encoding. Generic TypeDecl/CtorLayout/MatchPlan admission, identity, first matching arm, ordered construction and actual runtime application remain current work; Schedule and mutually recursive declarations are discriminators. Keep the existing Phase0 owner requirements and C4 dependency for dependent circuit edits. Independent inventories/oracles/ordinary source preparation remain possible.

### Trace continuity, observer stops and persistent boundaries

Freeze independent initial states and complete expected records first. Compare stream and batch separately to those expectations. Check `initial == first.before`, every `previous.after == next.before`, `last.after == returned state`, full program/state fields, charge sums and terminal/no-next. Existing first_difference/materialize_trace_a do not supply continuity automatically. A present empty record list is a zero-step observation, not absent evidence.

In A, Pending Return with empty frames publishes A80/0; immediate Error publishes A82/0; other pending controls at semantic fuel0 publish A84/0. A frame-cap failing push with available fuel is a charged event to Error4, followed by free A82. Observer budget0 returns Budget without advancing or callback even on an already terminal machine. After an event, Terminal outranks Observer break, then Budget; free publication still occupies an observation-event slot. Quint terminal stutters are not StepRecords and add no work. New observation calls do not refill semantic fuel.

Within one Step representation, adjacent records share the exact prior output. A persistent boundary retains original Step image, canonical projection result and explicit domain/capacity status. Import decodes the stored canonical boundary and preserves full semantic state, saved environments/frames/helper cursors/pending result, origins where supported, work/fuel/logical allocation counters, program/profile/layout/bounds and financial duties. Physical references/rows may differ across projection while decoded meaning is preserved. Projection refusal is tooling failure, not application exhaustion. A cut before publication retains Pending publication; terminal outcome has no semantic resume. Repeated calls on the same in-memory Machine are not persistent export/import evidence. Production persistent boundary storage/import is still missing.

### Complete financial continuation map

Owner H maps actual Core/2 load/start/resume behavior before choosing universal Commit/Yield/Await opcodes. Map committed financial snapshot and tentative segment separately; ordered effects including supply changes; exact assets/domain; gross debit, fees, net receipts, reserves/dust; liability identity/value and absent versus present-zero; pending site/captures/observations; residual duties; attempted versus committed work. Preserve earlier committed prefixes on later local precommit failure. Map guaranteed/fallible ledger phases separately from local semantics. Pure return, technical proof cut and source financial suspension remain separate boundaries.

### Measured sprint exits and parallel ownership

S1 exit needs actual stream Rust build, all8 selected Rust tests and the complete independent N01–N08/216-run comparison; Python wiring alone is source preparation. S2 exit needs actual333 named and ordered comparisons from the real capture; context/checked335/native remain open. S3 exit needs the integrated checker facts and meaningful ordinary regressions. No performance claim follows from the stream QA sink: it retains callback records. A later matched-workload experiment must declare sink behavior, exact workloads, complete-path timing and peak memory, with compile/execute/serialize/compare phase breakdown and unmeasured values explicitly absent.

DW2 owns stream producer/local QA leaves and proposes the fixed realization data. DW3 owns source-read diagnostics/helper/tests. Root integrates the shared run_host/check helper membership: adding a new checker dependency must also put it in every selected snapshot closure. The separate composed capture is another writer only after its disjoint fixed path is frozen. Per-slice representation/frame/arithmetic/B/financial/oracle/formal preparation can overlap; shared ABI/dispatcher/canonical adapters and final integration have one owner. Physical concurrency/reviewer throughput limits are unmeasured; no invented slot ceiling or runtime estimate is a gate.

### Bounded Quint contract

After ten Gemini reviews and reconciliation, instantiate a sequential interpreter/observer/chunk model with explicit complete modeled fields, not only a factory queue. Bind concrete normal literal/call/return/environment and terminal fixtures; semantic fuel, observer budget and outcome precedence; contiguous saved-state resume; scalar Step-to-canonical projection success/refusal; carried committed/tentative financial fixture fields and duties. Any B helper component modeled uses its own named clock, cursor and pending result, with the erasure premises explicit; unsupported B scope stays omitted rather than normalized to A fuel. Freeze requirement/field/action/invariant/case coverage and independently written literal expectations before execution. Check directed action reachability, functional wrong-output sensitivity and bounded sampled runs with actual seeds/bounds/imports. Model-only financial fixtures adopt no opcode, lowerer or target fee policy. No result supplies full39/full96 refinement, UInt128/field/canonical-byte coverage, Clean/handwritten constraints, native recursion/proof or target settlement.

## Integrated research refinements and precedence

The architecture, correctness and delivery annexes implement the accepted findings from the ten Gemini reports. They are normative design refinements for their declared proposed scope; source facts retain the baseline and locator stated by their author. Rejected/deferred proposals are not selected. This revision and its concrete OpenSpec contracts take precedence over earlier general prose. Existing owner-controlled physical/native contracts remain in force.

- Elaborate closed nominal recursive declaration groups in two stages, retain source-ordered declarations/matching, and distinguish test-only recursive drafts from the public Core parser. Generic constructor fields require an explicit continuation carrying identity, saved environment, evaluated prefix and remaining fields; recursive field evaluation cannot disappear into a Build helper expecting already evaluated values.
- Keep cross-profile observation erasure separate from same-profile persistent resume. Preserve pending B helper/task/result state at technical cuts. No quiet-helper-only restriction, origin invention or raw-StepJoin replacement of the selected canonical boundary is adopted.
- Reuse shared range/fetch/preservation infrastructure only with actual rule-specific inputs; keep all39 authoritative rule IDs and free terminal charges. Operation, witness and constraint plans are distinct typed artifacts. The current B EffectPlan is not already a declarative ConstraintPlan.
- Keep finite residualization, immutable specialization keys, generic fallback, widths1/2/4/8, declaration-driven matching and uncached witness DAG in v1. Runtime values/control/helpers/finance remain dynamic; a residual size limit never becomes application exhaustion. No memory or k reduction is claimed without matched measurement.
- Independent complete observations retain initial/before/after/returned-state continuity, exact work, empty-vs-absent fields and raw profile-specific records. B erasure is an observation map and never rolls back live arenas/counters. Canonical data remains child-before-parent and may unfold sharing with capacity refusal; it is distinct from raw Step equality.
- Actual Core/2 run_segment and stage publication/false-await/local-failure behavior own the financial mapping. F01 has committed17, false attempt2, true segment18 and total35. F02 segment costs15/15/28, committed-before[0,15,30], false attempts6/2, total58, duties absent→30→18→20→present0. These real source witnesses remain separate from the illustrative reserve100/liability40 fixture. Whole PassT expectations and Sherlock finance lowering remain open.
- The composition interface inventory separates execution step, persistent checkpoint, financial source boundary, recursive compliance statement and ledger settlement envelope. Preserve all five operators, required bounded multi-parent cases and compatible ancestry; no guessed digest/fan-in/backend or VerifyProof prerequisite is selected.

### First production compiler batch

The accompanying FIRST-COMPILER-BATCH contract freezes an explicit selection on existing mori compile, reusing the real registered B scalar source/owner consumer. Omitted selection preserves ordinary Core compilation. The first admitted subset is parameter-free Core/1 scalar-value actions with ordered lets/operators; it is explicitly HostOnly. Config, prepared/admitted tables and observation dispositions are typed and complete for the declared scope. Witness/constraint/native export omissions are explicit, never placeholder proofs or evaluated-literal substitution. Two independent new positive programs retain their original arithmetic/boolean structure. Generic recursion/ADTs, source effects and other profiles remain required subsequent increments.

The compiler owner must inspect the actual field inventory before implementing its wire adapter; stale comments claiming registered modules are test-only do not create duplicate registrations. Existing production types are Admitted/MachineImage/ProgramView; no BoundedProgramImage type is assumed. Compilation/preparation and explicit observation have separate predicates and existing owner dependencies. This first source task starts as soon as the final ordinary readiness conditions pass; native blockage cannot postpone it into another factory rebuild.

### Scope of the first finite validation

The first concrete model uses unchanged pure A/B definitions and independently frozen literal expectations. Its B helperWork8 instance calls pure initial/advance explicitly; old initCase would require16. The approved nonempty model-only finance continuation uses unrelated balance77, tentative pay5/fee1 and reserve delta-6 while the committed reserve68/duty10 remains preserved. This is an alternate discriminator fixture to the annex's illustrative tentative pay3; neither sets source fee policy. Extra shared-tree physical and complete F01/F02 correspondence cases, constructor/composition/residual cases are reported individually if omitted, then remain follow-up correctness tasks. They do not become silently validated by the smaller model. Every major action actually claimed needs a directed reached witness and functional defect sensitivity; zero sampled hits and early dead ends remain visible.

### Completion loop

Use the existing DoctorWatson commands, task/traceability records and correctness-loop matrix. Each batch names an actual missing developer capability, real consumer, independently expected observations, disjoint owned paths, affected checks, documentation changes and first blocked dependency. A changed transition slice retains conjunctive K/finite-Quint/pure-Lean/bounded-Clean/handwritten correspondence obligations; source-only completion does not assert that closure. Native and target jobs remain separately admitted and observed.

After two failures of the same defect class, preserve the smallest permitted reproducer and change approach before another broad attempt. After two process-only cycles or thirty minutes of administration, perform eligible source implementation or decisive diagnosis. Do not requeue unchanged blockers or passing source closures. If only blocked work remains, report the exact missing owner/input and stop dependent activity rather than build infrastructure. Public use remains permissionless; these maintainer controls are never developer compile/prove/deploy permissions.

Each implementation batch updates affected Diátaxis tutorial/how-to/reference/explanation pages, or records a concrete no-impact disposition for a category. Validate examples, relative links and actual API references; preserve source/result/acceptance distinctions and earlier receipts. Regular reviewed GitHub integration and local CI continue. The graph and DoctorWatson/circuit design logs are updated after material changes.

Full completion is the conjunction of all retained atomic UIREQ/B/U0–U7/M/MC/SP/G, formal, financial, composition, native, target, conformance and release exits. A literal transaction, source merge, model pass or scoped host success cannot complete Sherlock. Unknown implementation feasibility, external targets and owner availability are explicit risks, not a guarantee of no blockers.

## 2026-10-25: U021/U026 executable comparison packages

Under the owner's Sherlock design authority, `models/sherlock/qa/model_package.py`
exposes two maintainer interfaces. `projection` compares a selected complete K/Quint
trace pair with the independent frozen oracle. Its default re-compares the retained
actual F03 artifacts; producer executions stay historical. An explicit supplied pair
requires both artifacts, case and fuel. Every ordered pre/post state, rule, charge and
publication participates; the projection names bounds, field maps, hidden steps,
representation differences and unmatched finance/await behavior. Agreement supplies
no refinement, constraint, theorem, native or ledger evidence.
Public divergence diagnostics identify the field and hash the differing values;
raw states and arbitrary provenance remain in the digest-identified input artifacts.

`stage` emits the required recursive Lean semantic/fuel tasks separately from bounded
Clean encoding, fixed-capacity transition circuits and correspondence/export tasks.
It carries the six-file pinned public upstream reuse inventory, complete-prefix
comparison matrix, first divergence and missing layer/component membership. The eight
normal Lean examples plus exact-remaining-zero publication remain historical narrow
evidence. An optional comparison report is supplied data, not authenticated execution.
A missing comparison is explicitly not run, even with a null divergence.
`stage --retained-comparison` instead re-compares the pinned retained F03 pair
in-process (producers not rerun), so the canonical package needs no local files;
it and `--comparison` are mutually exclusive.

`bind-private` reads an explicitly selected integration tree without modifying it.
Source-byte pins, Git HEAD/dirty membership and integration details stay in an ignored
private record; public output contains only its opaque digest, source count and
qualification. No private source or paths enter the shareable package. A missing
record is explicit, and source membership establishes no proof/native correspondence.
These are maintainer checks, with no public compile/prove/deploy prerequisite. Ledger 9,
stock-verifier compatibility and the existing proof format stay fixed.

## 2026-10-25: implemented design by group

This section states what the pipeline is now and which lines of work carry it. It is a snapshot of the
owner's finish push (`qa/sherlock/predicates.yaml` groups A, B, C, F, U). The ledger (major 9), node,
Midnight SDK and proof server are unchanged. Everything else in the design is open to change, and the
changes below are the ones that have landed. Counts are from `scripts/sherlock-next` on each line at
the time of writing. The counts are per branch and will change when the lines are consolidated (see "Lines" below).

### Pipeline as built

1. **Source and compile.** `moriarty-sherlock-draft/1` text is parsed, typed and compiled by the
   public source compiler into a `ProgramImage`. Nominal ADTs, typed word arithmetic and generic
   whole-program types are admitted through the declaration and preparation APIs (U001-U012, U022,
   U027, U034, U035).
2. **Reference machine.** The fixed 39-rule `sherlock-k-a/1` transition table runs the image one step at
   a time and produces `StepRecord`s. Profile `sherlock-helper-microstep/1` is the original machine.
   Profile `/2` adds the `Await` control (C6 tag 11), the `Waiting{site}` outcome (tag 4) and the
   `LogicalCap` exhaust reason (tag 7); `/1` refuses all three, and the owner engine pins the selected
   profile (SI-01).
3. **Chunks and checkpoints.** A technical chunk cut is not a wait and not a financial commit. `mori
   simulate` chunks a run, resumes from a checkpoint, groups K-A `ReferenceSlice`s and materializes
   chunk boundaries through the `WitnessPlan` DAG (U013, U016, U029, U032). The `Waiting` checkpoint,
   its single observation and the atomic resume relation are specified in
   [suspension-and-finance](design/suspension-and-finance.md) and landed on the reference machine for
   profile `/2` as the control, outcome and exhaust-reason slice (tags 11, 4, 7); the resume relation, the
   `moriarty-sherlock-draft/2` syntax and the U014/U015 and U017-U020 acceptance checks are not yet in code.
4. **Finance.** Core/2 financial actions run on the Sherlock financial route. Await maps to the same
   `Waiting` outcome. `XFinAwaitEnter` exists in the fin selection code; `XFinCommit`,
   `XFinResumeAccept` and the pure `BAwaitEnter`/`BResumeAccept` are specified in the design note and
   not yet in code.
   Commit stays implicit at an await and at completion; there is no surface commit statement.
5. **Native inventory and C6 fragments.** The hand-authored native relation inventory is reported for
   every draft run, and receipted compiled-and-run evidence is accepted for U023 fragments. C6
   fragments landed for the Eval schedule, CallBind, Find/EnvGet, Append, Bind, Eval CallArguments and
   the legacy control rows, behind one dispatch owner that constrains Core rule selection (U023, U028,
   U030). The native constraints for the new `/2` tags remain later work.
6. **Model correspondence.** `models/sherlock/qa/model_package.py` provides `projection` (U021, K/Quint
   against the frozen oracle) and `stage` (U026, recursive Lean semantics and fuel tasks separate from
   bounded Clean encoding) plus `bind-private`; details in the previous section.
7. **Recursive proof chain.** Two linked child proofs (A102 and A80) are verified by a k20 parent, which
   a handwritten terminal decider D checks against the stagenet verifier contract. The exact artifacts
   were verified independently, accepted by local ledger-9, and the five rejection cases (tampered inner
   proof, mismatched VK, wrong predecessor root or height, reordered batch, replayed statement) were
   refused locally and on stagenet. A finalized contract call and its public receipts and complete
   effects readback exist (A1-A10, all passing).
8. **Rollup.** The B line builds the stagenet recursive rollup: a deterministic Fenrir-derived dataset
   with public seed (B1), independent replay (B2), a 16-transaction batch as one `sherlock-v3-batch/1`
   leaf, one verifier parent and the terminal decider (B3), per-layer k, rows and columns (B6) and
   time, RSS and bytes in separate scopes under decimal 32 GB with swap zero (B7; the cap was raised
   from 16 GB by the owner on 2026-10-25, original 16 GB failures kept). The fixed-key
   `FSYNB002` prefix extension commits to a 67-state trace so one sealed anchor serves every batch of a
   prefix. B4 (stagenet acceptance of the batch), B5 (two-, four- and six-batch prefixes) and B9 (the
   feasibility report) remain open; B8 follows B4.
9. **Settlement.** Every settle goes through an accept/settle anchor of the shape used by A9 and B4.
   The planned `sherlock-run-anchor` generalizes it for any Sherlock run: a Waiting checkpoint digest is
   an ordinary root, the resume observation is disclosed and recomputed by the anchor, and the /2
   verifier-key set is sealed as `keys_id`. It is a new contract, not a ledger change.

### Groups

| Group | Meaning | State (2026-10-25) |
| --- | --- | --- |
| A | Native backend, linked children, k20 parent, decider, local and stagenet acceptance | 10 of 10 |
| B | Synthetic rollup data, replay, batch proof, resources, feasibility | 7 of 9 on the factory-b line; B5 and B9 open |
| C | Rollups of U items (C1 core; C2 chunks; C3 compiler; C4 native/optimizations; C5 finance; C6 evidence) | 1 of 6; C2-C6 follow their U items |
| F | Factory: charter, schema, hooks, preflight, holdout, bus | 12 of 13 on factory-b; F4 needs only a quiet heavy lock |
| U | The 35 UIREQ scenarios | 23 of 35 on factory-u; open: U014, U015, U017-U021, U023, U024, U026, U031, U033 |

### Lines

The work is on parallel lines that have not been consolidated: `factory-a` (A), `factory-b` (B and F,
carrying A 10/10 by merge), `factory-u` and `factory-u-c4` (U, at the same head for now),
`factory-u-kc`. Consolidation into one line and a full re-verification of all groups is the last step
before publication. Pages and the visualization are updated after that.

### What did not change

The 39-rule table, the ledger-9 call shape, the stock verifier compatibility, the existing proof format
and the public-use rule (no compile, prove or deploy permission is required of a developer) stand.
Claims stay conjunctive: a host pass, a model pass or a settled root alone does not close an exit.
