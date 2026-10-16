# A1 source and evidence audit for V1 planning

Date: 2026-10-16. Reviewer assignment: Astra high A1, independent nonsecurity source/evidence review. Checkout: `/home/charl/Moriarty/.worktrees/v1-factory-plan-20261016`; observed HEAD `9bcbca52bbfbc58443f6bdb287d73afb7d71f1ab`. This report is planning-only. It changes no production, requirement, frozen worktree, canonical wiki, source receipt, counter or acceptance record. The implementation goal remains paused. No tests, builds, models, SDK, native or target operations were run. C4, native authority zero and historical limits remain unchanged. No Foreman tool or runtime was invoked; none is proposed.

**Finding:** Moriarty has substantial callable local Core tooling and source compilers, plus narrower model and native results at historical pins. The missing product is the integrated, general supported-source → correctly bound proof → actual Midnight effects path, followed by persistent financial lifecycle and composition. The current source does not justify any full U0–U7 or M1–M17 closure. The useful next work is to connect existing consumers and close explicit semantic seams. Another scheduling or receipt platform would not supply those behaviors.

## Scope and falsifiable predicates

The audit asked whether (1) a claimed source feature reaches the real public command consumer, (2) a claimed pass belongs to the inspected tree/profile/target, and (3) a sprint can name an observable product result and its blocking dependency. Each claim below is tagged by its evidence type. `INPUTS.json` records actual file hashes and reading ranges; selected-line inspection is not whole-file review. No graph was used: freshness was not qualified, so navigation used direct files. The requested `crates/mori-cli/src/main.rs` and root `tools/` do not exist here; the actual executable is `crates/mori/src/main.rs`.

Startup loaded the installed `moriarty-dev:develop` skill, read AGENTS/product contract, and ran its required guarded read-only status. Status returned historical SP01.6 with stale binding/candidate inputs, missing accounting and unavailable live resource state; that queue did not select this task. Only normal filesystem/source reads and report writing followed.

## What the real consumers do

| Consumer | Source observation at this HEAD | Evidence limit / first missing boundary |
|---|---|---|
| Rust `mori check` | `crates/mori/src/commands/check.rs` routes a `language`-headed program into Rust Core; `/2` uses staged loading. Other source goes through `mori_parser`/`mori_checker`. | A successful authoring check is not admission by either compiler. No fresh command result here. |
| Rust Core `/1` | `crates/mori-core/src/ir.rs:4` declares `moriarty/1` and `moriarty-core/1`. Values are UInt128 and Bool; operators are finite Add/Sub/Mul, comparisons, And/Or and Not. Actions carry state, transfers, fees, liabilities, discharge and assertions. | No general division, source function, recursive datatype or branch constructor is in this inspected Core IR. These may exist in other profiles without being available here. |
| Core semantics | `runtime.rs:451` evaluates both binary operands; `runtime.rs:721` executes `requires` and `ensures` at their statement position. Checked UInt128 arithmetic is explicit. | An oracle or source adapter must preserve these semantics. A trailing-postcondition or short-circuit source cannot silently share this meaning. |
| Rust `mori compile` | `commands/compile.rs` invokes `mori_backend::source_projection::compile_reported`; `/2` selects `compile_staged`. `/1` uses explicit signed/unsigned target mode. | Target descriptor and ZKIR text exist; the descriptor is not a qualified deployed tuple. `mori-backend/src/lib.rs:363` explicitly leaves the authenticated predecessor/outside-frame ledger adapter open. |
| Sherlock metadata in public compile | `mori-backend/src/source_projection.rs:31` compiles the existing target artifact, then appends separate scalar projection metadata. `mori-core/src/sherlock/source_adapter.rs:135` labels it `source-static-scalar-projection-only`. | It does not execute Sherlock or change the target artifact. It admits stateless `/1` actions with ordered lets and one final emit value. It explicitly sets source functions/branches/recursion/data support false and does not execute the emit effect. |
| Rich Sherlock draft frontend | `mori-core/src/lib.rs:10–26` registers lexer-dependent draft source, typechecking and image bridge under `cfg(test)`. | Historical factorial/treefold results are private draft host observations; they do not expose that grammar through the public compiler. |
| `compile-call` | `commands/compile_call.rs:49` routes legacy one-action compilation or all-action source-static export/reopen. `mori-ledger/src/relation/mod.rs:86` creates ledger-call relations; these differ from the transition backend. | Static export has `source-static-unproven-uninstalled`; mapped mode additionally says `unauthenticated` (`static_compile.rs:18–22`). The existing native-policy test source expects `one-public-payer` refusal for mapped FeePool debits. Reopen is deterministic recompilation, not financial execution. |
| Local `/2` continuation | `commands/simulate.rs` dispatches staged start/resume; `mori-core/src/stage/profile.rs` isolates `/1` and `/2`. `mori-backend/src/stage/mod.rs` describes N+1 segment circuits for N awaits. | `/4` artifacts deliberately cannot convert to `/3`; proving, signed and native consumers cannot accept them. All ten stage gates remain unperformed according to the source. Local continuation is not ledger lifecycle. |
| Manifest tests | `commands/test.rs:37–82` reads a bounded `moriarty-tests/1` manifest and compares complete post/effect values with Core parse/simulate. Aggregate reporting no longer applies the single-document cap to the whole report. | It still directly calls `/1` parse/simulate, unlike staged `simulate`; no staged manifest journey was found in this consumer. Independent expectations are required; emitted actual results are not an oracle. |
| Rust servers | `commands/lsp.rs:9` and `commands/mcp.rs:9` are `todo!`. | These advertised Rust commands have no server implementation. The Node servers do not fill those command bodies. |
| Node beta | `packages/moriarty-beta/package.json:9` installs another executable named `mori`; `src/bridge.ts:5–6` imports experimental Source/6 and TypeScript Core/5 preparation. | Explicit local-stipulation-only/PreparedUnqualified scope. This is a distinct install and consumer closure, not Rust Core or a native compiler. |

The `simulate --trace --target` branch and signed `verify-intent --profile` branch reach backend consumers (`commands/simulate.rs:75`, `verify_intent.rs:140`). Their benign command names are not evidence that execution is an ordinary reference-only check. This audit did not invoke them and supplies no security interpretation.

## Evidence ledger: delivered source, retained tests and target claims

**Source-inspected:** All consumer statements above describe present source. No current-tree runtime pass was produced by this review. Source registration, test bodies and compileable-looking APIs remain source facts until observed.

**Historically tested / captured local:** The October 14 audit describes PR49 FeePool's 23 actions/162 finite financial cases and static export, not full M11. The current `core-test-report/README.md` records a later six-CLI/four-unit capture and final scoped reviews, so the old October 14 aggregate-overflow task is resolved within that reporting scope. The same README preserves failures and exact binaries. `core-source-provenance/README.md` records ten selected tests and, in its final section, completed successor reviews; its introductory “audits pending” sentence is historical/stale. Do not reschedule those fixes from older summaries. Their full raw execution closures were not independently replayed or rehashed in this audit.

The ROADMAP carries other captured host cohorts (Sherlock normal programs/prefixes, image/helper/checkpoint work and staged Core tests), historical local native signed-payment/financial proofs, and scoped Preview loan/swap results. I inspected those roadmap reports, not every underlying raw result. They remain **reported historical evidence**, not fresh verification of this HEAD. In particular, the staged-Core narrative's “local/unmerged” wording cannot negate the staged code now directly observed in this pinned tree; source presence and the historical capture/review status need separate fields.

**Latest local observation outside the pinned tree:** The supplied Root receipt `P4-QUINT-ROOT-PHASE1-POSTREHASH-OUTPUT-r1.json` reports exit 0, no parent timeout, 18 named passing arithmetic/fuel tests, matching source/package/tool checks, and stdout digest `1ce5a9f5155543591df062d365dbec52248160974ad0b85bb1994e732685ac7b`. It explicitly marks zero failed/ignored as inferred rather than printed. Full trace adjudication is PENDING; phase2 NEVER_STARTED. This supersedes “tests never started” only as a later captured output observation. It is not accepted complete QA, current checkout execution, K correspondence or circuit validation. The original capture/trace was not inspected here; the receipt's closure statements are not my independent rehash.

| PR | Source delivery at the pinned history | Result boundary retained |
|---|---|---|
| #95, `5cf27d64…` | Registers `literal_schema` / `emit_literal_step` in the actual backend machine module. | 16 registered fetch/identifier/enabled-fetch/literal tests remain UNRUN; A102 compile not performed. |
| #96, `bd62ed70…` | Full39 conditional semantic source map. | Source adoption, not P1 completion or a full emitter. P2–P6/G4/F4 conditions remain. |
| #97, `e0740ced…` | Complete CandidateA P3 arena source proposal/adoption record. | Design evidence; complete machine realization/correspondence remains open. |
| #98, `1a68573d…` | Append model sources and captured finite QA publication. | Quint typecheck/eight tests and bounded simulation are captured evidence; 32 samples inferred. Nine K cases UNRUN; no executed K/Quint comparison. |
| #99, `56f973ff…` | Aux/1 private packing source plus aux/2 arithmetic/fuel design. | Only six private normal-test callers; 35 host codec/packing tests including prior 29 remain UNRUN. No complete `packStepWitnessA` or production caller. |
| #100, `70220c8d…` | Full-width arithmetic/fuel Quint and K source projections. | Published stop record is historical; later 18-pass receipt above is separate. K33 UNRUN, phase2 unstarted; no complete-state/circuit agreement. |
| #101, `9bcbca52…` | Conditional G4 field/instruction source requirements. | Source argument only; no G4 runtime certification or emitter acceptance. Security meaning remains Opus-owned. |

**Target-qualified:** This review establishes none. Historical Preview results qualify their own Preview scope only. No new A102 compilation, native execution, proof, independent native verification, deployment or Midnight stagenet result is present in the supplied current observation. Fenrir identities and deployed tuple remain unestablished in the reviewed requirement records. Preview, Fenrir and stagenet must retain separate identities and claims.

## U0–U7: accomplishment versus the still-open exit

| U | Accomplished precursor | Critical missing behavior/evidence |
|---|---|---|
| U0 | Versioned Core profiles, real commands, scalar projection/provenance and target descriptors. | One implemented consumer-facing admission/semantic map; source/target field correspondence and actual target tuple. |
| U1 | Arithmetic/constraint source, narrow old native cases, Sherlock partial source/model work. | Certified required primitive basis, call-site preconditions, complete preservation/completeness and measured cost for actual admitted native scope. |
| U2 | Bounded Core compile/evaluate/intents, static call export/reopen. | Two positive contrasting novel programs through the same genuine source-to-target pipeline, grammar-level generality and complete authoritative effects. |
| U3 | Local typed await/start/resume, duties and policy/sponsor/recovery precursors. | Signed/native persistent stages, actual phase effects, conditional delivery, joins, unknown/late outcome handling, reserved recovery and complete duty preservation. |
| U4 | Host/reference/model preparation and recursive design work. | Two-step native recursive financial history, bounded multi-parent compatibility, separate-party private handoff and all five composition operators. |
| U5 | Local policy and metering source. | Exact external adapter, durable concurrent service budgets, epoch migration and independently operating solvers under explicit federation policy. Direct use must remain available. |
| U6 | Finite FeePool and other source examples plus retained fixture corpora. | Full frozen family membership and field-level semantics, exact rounding, source/native/target conformance and extension requalification. |
| U7 | Real local Rust tools plus separate beta/editor distribution. | Clean external authoring-to-own-wallet submission, reproducible builders, honest supported scope, pilots and all retained release predicates. |

Every full exit stays open. The 160-heading current OpenSpec index and 69 top-level discovery IDs are not an atomic completion denominator. No percentage is assigned.

## M1–M17 reconciliation

| M | Observable precursor | Remaining qualifying result |
|---|---|---|
| M1 | Rust transition compile and call-static compile. | General admitted-source → ledger-9 relation/correspondence and target execution. |
| M2 | Source pins and target DTOs. | Actual matching Fenrir toolchain/network/Anchor/deployed tuple. |
| M3 | Existing local APIs and historical narrow proof receipts. | Complete required-target prove/independent verify/deploy/call/effects chain. |
| M4 | Local sponsor accounting. | Actual DUST and Capacity Exchange settlement/sponsorship. |
| M5 | Static byte/instruction fields; backend sets `circuit_k`, `fraud_max_k` to None and `cost_measured=false`. | 64 KB transaction, fraud k≤19 and replay-budget qualification. Static ZKIR size is not transaction size. |
| M6 | Deterministic reference replay and scalar semantics. | Full supported semantics, independent complete traces and fraud-game correspondence. |
| M7 | Existing domain/context intent paths. | Authentic network/Anchor binding and required target acceptance; security adjudication belongs to Opus. |
| M8 | Scoped Schnorr/native precursors as reported. | Required primitive/splice audit and correspondence; not reviewed by A1. |
| M9 | Core `AssetId {network, issuer, reference, scale}`. | Consistent actual target identity binding and full effects readback. |
| M10 | Real simulate/test/verify-intent local paths; repaired aggregate reports. | Consistent profile journey, staged testing, complete developer and target qualification. |
| M11 | Finite FeePool static/local source delivery. | Capacity Exchange and full L2 contract behavior on target. |
| M12 | Requirement retained. | Every-block-anchored Express Bus Registry interface and authoritative anchoring result. No implementation certification from this bounded review. |
| M13 | Local staged continuation, policy and duties. | Full persistent coordinator, pending/recovery/liability/scoped-call behavior with real phase effects. |
| M14 | Bounded batch examples reported. | Complete single-domain swap fast path with actual batch-clear conformance. |
| M15 | Partial financial library examples. | Batch/uniform price/solver bounds/oracle/liquidation and retained full conformance. |
| M16 | Logical meters and fee-policy precursors. | Complete distinct cost planes and qualified fee capability. |
| M17 | Local policy precursors reported. | Named actual Fenrir committee and qualified clearing/proving/relaying services. |

All M exits remain open. M12/M14–17 implementation families were not exhaustively inspected; the rows preserve required acceptance gaps and bounded observations, not findings of total code absence.

## Ten critical missing product behaviors

1. **One reproducible public source-to-proof-to-ledger journey.** Current commands meet different relations, and `mori-ledger` explicitly lists final proof/settlement open predicates. Deliver a novel program with complete effects under UNI-001/003/018/019 before claiming the factory produces deployable language programs.
2. **Consumer-specific admission that a developer can inspect before work.** `check`, `/1 compile`, `/2 compile`, mapped export and Node beta disagree in scope. Expose actual profiles and supported constructs in the existing tools; do not infer downstream success from a checker (UNI-018/024).
3. **Independent semantic correspondence across the admitted grammar.** Current source projection/reopen guarantees identity/metadata, not values, effects or logical work. Freeze full observations and cover rules/fields, including eager booleans, positional ensures and present-zero duties (UNI-019/020).
4. **A complete Sherlock step realization connected to program/source inputs.** Current A102 and fetch pieces are not full39 dispatch/StepInv; public scalar metadata is not a source compiler into a machine proof. Preserve C4 before any dependent circuit edits (UNI-002/003/014/017).
5. **Signed staged continuation accepted by the target.** `/2` local continuation and `/4` source-static artifacts have no supported proving/native consumer. Preserve actual committed prefixes, late results, fees and duties instead of representing a pause as completion (UNI-005/006/007/008).
6. **Separate logical, circuit and fee resource semantics with real qualification.** `cost_measured=false`, symbolic lanes and supplied sponsor observations cannot establish DUST, transaction/fraud/replay bounds or reserved recovery (UNI-008/014, M4/M5/M16).
7. **History and private composition for financial states.** Current pure machine work does not implement financial suspension or private multi-parent handoff. Two native steps and all five retained composition operators remain mandatory (UNI-009/010/017).
8. **Complete financial libraries rather than fixture-only success.** FeePool does not supply Capacity Exchange, broad batch clearing, liquidation, or full ACTUS/DeFi conformance; finite arithmetic cannot silently stand for general division/rounding (UNI-014/020/023).
9. **Usable public developer tooling.** Rust server commands are stubs, the same executable name resolves to separate products, and manifest tests do not follow staged simulation. Deliver honest commands with a self-contained install/authorship path (UNI-018/024).
10. **Real external integration where claimed.** Qualified registry/committee/Anchor, exact external adapter and durable multi-solver service behavior remain absent from the demonstrated chain. Keep direct Midnight use permissionless and avoid imposing a service dependency (UNI-001/011/012/013/015/022).

These are nonsecurity product gaps. Any authorization, hostile-witness, privacy or cryptographic acceptance assessment and the associated negative tests must come from the designated Opus review; this list is not a substitute verdict.

## Seven implementation sprint candidates

These are proposed cuts within existing U owners, not new milestones or an authorized execution plan. No sprint below starts while the implementation goal is paused. Use direct repository tools and direct author/reviewer calls after resumption. One integrator owns shared IR/ABI/result interfaces; reserve build/model/native actions to their existing scope. Each candidate has a concrete observable result rather than a new management system.

### S1 — Public consumer discovery and staged test journey

**Dependencies:** current Core/CLI only; no native prerequisites for source preparation. **Deliver:** extend existing inspect/check output with resolved executable, source/semantics, consumer and unsupported boundary; add explicit staged cases to the existing test consumer (or a clearly separate versioned staged manifest) so start/resume results can be tested through the public command. Implement useful Rust LSP diagnostics using the same parser/checker once profile reporting is stable; report MCP as unsupported until actually implemented. Do not build a second server framework.

**Exact EARS:** UNI-018 “Profile and consumer admission,” both scenarios; UNI-024 “Incomplete public journey” and “Complete aggregate CLI test report”; preserve UNI-001 “Permissionless public pipeline.” **Demonstration:** one `/1` program, one `/2` pause/resume and one beta-only input show their real scope, with manifest-order complete results. **Needed evidence:** narrowly selected public CLI tests, unchanged `/1` report bytes, explicit stage profile isolation, and a real editor diagnostic exchange if LSP is included. Preserve the already-passing aggregate tests instead of reopening their implementation.

### S2 — Two ordinary compiler correspondence specimens

**Dependencies:** explicit consumer mapping from S1; independent expected results prepared before implementation output. **Deliver:** installment-invoice and conditional-reserve specimens in the same actual supported `/1` pipeline, using existing compile and compile-call paths with a declared common domain. Keep division outside the claim unless a finite quotient/remainder contract is explicitly authored. Attach complete source/ABI statement/effect mapping to existing output.

**Exact EARS:** UNI-019 “Common construct domain” / “Different admitted statements”; UNI-020 “Complete finite observation” / “Shared evaluator oracle”; UNI-003 “Native target and correspondence”; UNI-005 “Complete financial state.” **Demonstration:** positive source programs with contrasting arithmetic/state/assets/duties and complete independently predicted effects. **Needed evidence:** independent literal/exact-arithmetic expectations, selected parser/evaluator/compiler/static-call checks and rule/field coverage. Static stages may be delivered first. Real proof/verification/readback is a later separately admitted stage; no U2 closure is claimed from this sprint's local output.

### S3 — Public bounded Sherlock source/image route

**Dependencies:** select and document the supported source subset; preserve Core `/1` semantics and the separate draft grammar. **Deliver:** turn the existing source/image work into one versioned public route for a bounded function/call/list/tree slice, with deterministic source origins and stable image/checkpoint serialization. Keep unsupported constructs explicit; do not replace the financial Core compiler with the draft frontend.

**Exact EARS:** UNI-002 “One semantic relation” / “Two source constructs”; UNI-018 “Profile and consumer admission”; UNI-020 “Complete finite observation”; UNI-016 “Assurance and release.” **Demonstration:** independent weighted-total and recursive/tree-fold programs through the chosen public route, with full machine-state/prefix observations rather than final-value-only comparisons. **Needed evidence:** exact source-to-image mappings, independent expected intermediate states/fuel, serialization/reopen identity and existing reference-model adapter comparisons. These establish host/source behavior only; circuit and target columns stay open.

### S4 — One complete machine transition through an admitted circuit consumer

**Dependencies:** C4's admitted A102 compilation gate must be resolved before dependent circuit edits; complete owner-specified P2–P6/G4/F4 inputs, aux witness placements and exact physical semantics for the selected slice. The later 18-pass arithmetic receipt needs trace disposition; phase2/K remain separate. **Deliver:** connect actual source/image cells, witness construction and one complete selected transition's preservation/outputs to the real emitter, then extend along concrete rules rather than more standalone source maps.

**Exact EARS:** UNI-003 “Native target and correspondence”; UNI-014 “Certified libraries”; UNI-017 “Complete native recursive path” / “Incomplete cryptographic or release evidence.” **Demonstration:** one literal step with complete unchanged state and exact fuel, then the first arithmetic step using the same real consumer. **Needed evidence:** source-bound compile result, independent complete-state expectations, rule/field wiring and, only under admitted authority, constraint/native observations and Opus-authored controls. A source construction test cannot replace native correspondence. Do not run or implicitly authorize C4 now.

### S5 — Conditional invoice lifecycle with real residual duties

**Dependencies:** S2 financial observation contract and an accepted stage interface; U2/native basis for target acceptance. **Deliver:** extend the existing `/2` route toward persistent signed stages: partial discharge, event-gated continuation, separate recovery and explicitly represented unknown/late outcome. Bind the source's phase policy instead of inventing a charged failure branch in lowering. Use the existing local start/resume as the integration point.

**Exact EARS:** UNI-005 “Partial fill”; UNI-006 “Phases and partiality”; UNI-007 “Conditional settlement”; UNI-008 “Recovery and state limits.” **Demonstration:** two-asset escrow bounded by spend≤11 A including fee≤1 A and receive≥20 B, with recipient/document predicates and a surviving partial duty; resume after a process restart. **Needed evidence:** independent complete prefix effects/work/duties, ordinary serialization/start/resume tests, explicit false-condition waiting, and eventual actual phase readback after native admission. Opus supplies the late-success/refund and authorization controls. This is not a federation project.

### S6 — Financial library slice and retained conformance inventory

**Dependencies:** S2 numeric/profile contract; S5 only for families requiring lifecycle. **Deliver:** first close one missing reusable Capacity Exchange or batch-clear behavior through the same language primitives, then map the resulting construct needs back into Core. Maintain the existing frozen coverage manifest with each original row visible; avoid a new coverage service.

**Exact EARS:** UNI-014 “Certified libraries”; UNI-020 “Complete finite observation”; UNI-023 “Full family accounting” / “Finite library sample.” **Demonstration:** independently specified allocation/price/remainder outcomes with gross fees, reserve dust and preserved liabilities. **Needed evidence:** accepted source tests for the selected family, independent arithmetic and exact field comparison, explicit unsupported cells, and separate future native/Preview/Fenrir observations. Preserve 277 ACTUS fixtures/fields, 18 executable types, 32 dispositions, DS01–07; 72 DeFi rows, DA01–24, supplements, three held-outs, eight intent cases and eight regressions. A finite slice never closes U6.

### S7 — Clean developer target walkthrough, then history/composition slice

**Dependencies:** useful S1/S2, actual target tuple and separately granted native authority, the required U1 basis; S5 lifecycle invariants before full U4 composition. **Deliver:** a clean external developer can install the actual CLI, author a novel supported program, inspect intent, prove/independently verify, submit from their own environment and read all effects. Reuse that concrete chain for a second financial step and a bounded private handoff rather than another infrastructure harness.

**Exact EARS:** UNI-001 “Positive witness”; UNI-003 “Positive witness”; UNI-009 “Full history scope”; UNI-010 “Private continuation”; UNI-017 “Complete native recursive path”; UNI-022 both target scenarios; UNI-024 “Incomplete public journey.” **Demonstration:** same source/statement/artifacts followed across real target boundaries, then separate-party continuation. **Needed evidence:** independently recorded target identity, proof/key/circuit/ABI identities, complete effects, two-builder reproduction, separately authored Opus controls and actual receipts for each claimed target. This sprint is currently blocked for native execution; source packaging may be prepared independently. Full U7 still waits for U2–U6 and all retained release predicates.

## Delivery simplification and limits

Prioritize S1 and S2 as immediate product cuts after resumption, with S3 source work only on an explicitly chosen subset. S4 is gated, not a reason to stop unrelated CLI and financial correspondence work. S5 uses the actual staged consumer already present; S6 supplies missing financial behaviors; S7 produces the eventual public journey. Avoid repeated publication of proof-adjacent prose as a substitute for these callable results.

Keep existing requirements and source manifests. Add only exact source/result evidence needed for a changed product predicate. Do not build a scheduler, new campaign hierarchy, Endstop, approval registry or extra public toolchain prerequisite. Reviews assess concrete code/results; they do not license user programs. Unknown target facts stay unknown; no guessed Fenrir version or committee identity is needed for eligible source implementation.

Uninspected mandatory families remain explicit: full inherited MC/SP/G/L/ZR/MNR/MPLR/MOR/AEO bodies; frozen financial membership/manifests and raw fixtures; full independent financial oracles; complete K/Quint/Clean/Lean/SDK/dependency implementations; native source/proof bytes and original trace captures; actual wallets, network configuration, private Fenrir fork and external services; full Node import closure; complete production ownership/custody/signature/security paths; full language parser/checker and all backend statement arms. No security certification, exhaustive repository absence claim, full end-to-end audit or decision-grade release approval follows from this bounded report.
