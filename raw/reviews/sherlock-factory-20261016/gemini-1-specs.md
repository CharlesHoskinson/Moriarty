# Moriarty V1 Independent Planning Review — OpenSpec/EARS Coherence, Roadmap Traceability, and Capability Sprint Architecture

> **Review Identification & Boundary Notice**
> - **Assigned Lens**: OpenSpec/EARS coherence, roadmap traceability, stale scope/status assertions, missing acceptance predicates, `U0–U7` / `M1–M17` / `SP01–SP12` / `MC01–MC08` gate conflicts, and reusable capability sprints preserving all requirement families.
> - **Posture**: Planning proposal only. No tests, builds, models, SDK calls, prover runs, or ledger actions were executed. No tools, shell, file writes, or web requests were invoked.
> - **Authority & Overrides**: `native_authority = 0`. No native execution authorization or resource/counter reset is created. **NO FOREMAN**: Foreman, Endstop, Pel, custom campaign schedulers, and receipt-review bureaucracy are excluded from all proposed workflows and public/internal gates. **Opus Security Domain**: All cryptographic, soundness, and security judgments (`OSR-1014`, `M7`, `M8`, `UNI-003` witness soundness, `UNI-010`/`UNI-011` trust boundaries) belong exclusively to Opus; this review preserves their interfaces and gate placement without usurping security sign-off.
> - **Explicit Inspection Limits**: This review inspects only the inline supplied texts and baseline disclosures:
>   1. Baseline state: GitHub `main` pinned at `9bcbca52bbfbc58443f6bdb287d73afb7d71f1ab` (PR #95–#101 merged).
>   2. `ROADMAP.md` (`SHA256: 0afd1af3851ac4367b453ff28573b176c99b7c8ecfb818ad291524f8b418c256`).
>   3. `docs/MORIARTY-PRODUCT-CONTRACT.md` (`SHA256: d5674c650741103284e0aa8cded4c0e8a0a1be38faa0c65aedb043710caa4396`).
>   4. `openspec/changes/consolidated-language-kernel/requirements.md` (`SHA256: aa7b2243bbb27e0420bce24977180eec24977180eec253d0f8c201c94baa1d14333ca1e4a9a`).
>   5. `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md` (`SHA256: 66f85468058830690bd8e0d21e4f76a291dd99c572f1f5224cbf4b49b8de67b1`).
>   6. `openspec/changes/consolidated-language-kernel/traceability.md` (`SHA256: d549f31a16e3b7c1ef0f1de6a58812f9084aace273939190ff10b3529c84ded0`).
>   7. `COMPLETE_HEADING_INDEX` (`original_sha256: a361071a458f1486a82bd7256e31cc8b7a94dd69d4ade7dcc90d0cf6861d248f`, 160 headings across 19 `spec.md` files; clause bodies for the 18 non-consolidated `spec.md` files, `security-reconciliation-2026-10-14.md` [`OSR-1014`], and `docs/MORIARTY-BACKEND-REQUIREMENTS.md` [`ZR01–ZR16`/`MNR01–MNR08`] are not inlined).
>   8. `ROOT_PHASE1_POST_REHASH_AND_NORMAL_OUTPUT` receipt (`capture_sha256: 4b20fae0447274a5974ba5d21a39d7eac4d6342943de944fe4b5b3708a9a4ebd`, `stdout_sha256: 1ce5a9f5155543591df062d365dbec52248160974ad0b85bb1994e732685ac7b`, observed `2026-10-16T12:54:20.679231+00:00`).

---

## Section 1: Current Evidence versus Incomplete Behavior & OpenSpec/EARS Traceability Audit

### 1.1 Baseline Accomplishments vs. Incomplete Reusable Behavior

| Subsystem / Capability | Verified Baseline Evidence (Source Text & Supplied Receipts) | Incomplete Product Behavior & Open Gaps |
|---|---|---|
| **Core Compiler & `compile_reported` (`moriarty/1`, `moriarty-core/1`)** | Pinned `main` (`9bcbca52...`, PR #95–#101 merged). Core compiler already supports stateful financial constructs: state, assets, transfers, and liabilities. `compile_reported` calls the existing Core backend compiler and appends a Sherlock projection report. Mapped-role provenance (`docs/audits/2026-10-15/core-source-provenance/README.md`) records statement/expression-to-field metadata, public export/reopen, byte-identical IR for 1 specimen, and 10 focused tests. | Production source frontend vs. `cfg(test)` draft frontend (`sherlock-draft-source`) diverge. Mapped-role mode in PR #47/#49 is explicitly unauthenticated, unproven, and uninstalled. General source-to-ZKIRv3 correspondence (`U0/U2`, `M1`, `UNI-002`, `UNI-003`) and native target execution remain open. |
| **CLI Reporting & Offline Readback (`mori test`, `STAGENET-OFFLINE-README.md`)** | `core-test-report` (2026-10-15) passes 4 CLI regressions: unchanged small-report bytes, ordered results for 256 cases (>1 MiB JSON), 3 expectation mismatches with exit 1, and 87 FeePool cases. Offline readback helper passes 16 assertions inventorying `Core/1` artifacts and comparing JSON fields without backend invocation. | Offline readback is not wired to live Midnight stagenet deployment/call or public `mori` CLI effect verification. Rust LSP and MCP servers remain stubs (`UNI-024`); Beta authoring/MCP is a separate TypeScript `Source/6 -> Core/5` local package. |
| **Core Continuations (`moriarty/2` `await` & `/4` Multi-Circuit Lowering)** | `docs/CORE-CONTINUATIONS.md` and `OBSERVATIONS-CLOSURE.json` (2026-10-15) specify typed `await`, local start/resume with captured inputs/lets/events, and `/4` lowering into $N+1$ circuits. 88 focused Rust test functions (17 run in last closure capture, 71 carried forward by call-path projection) and 11 Quint tests (without established Rust correspondence). | Not a full rerun of all 88 tests; Quint-to-Rust correspondence unestablished; `mori-ledger` native-call execution does not support `/2`; Midnight phase-partial effect retention (`UNI-006`), two-asset escrow, document predicates (`UNI-007`), and recovery authority (`UNI-008`) remain unintegrated on target. |
| **Sherlock Universal Interpreter Host & Draft Frontend (`A`-profile, `B`-helpers)** | Quint `K-A` (`sherlock-k-a/1`) has 98 + 126 passing deterministic normal tests (Quint 0.32.0, seed 1). Rust host closures pass 46 (flat image), 79 (call-helper cursor), 82 (source-origin propagation), and 86 (typed in-memory cuts) tests. Private `cfg(test)` draft source frontend parses `Factorial(4)=24` and tree fold `=7` across 11 tests (86 older tests filtered out). | Sherlock projection in `compile_reported` is scalar-only (unlike the Core compiler). Draft frontend uses cached `rlib` lexer without proven source correspondence and is test-only (`cfg(test)`). Canonical serialized checkpoints, bounded `B` budget/schema semantics, financial suspension, and circuit lowering remain open. |
| **Sherlock Machine-Circuit & Arithmetic/Fuel Models (`aux/1`, `aux/2`, `P3`, `G4`, `A102`)** | Conditional source adoption for `full39` semantic map, `aux/1` host packing (6 private tests `UNRUN`), `aux/2` UInt128 arithmetic/fuel (232 logical lanes, not measured circuit cost), `CandidateA P3` arena policy, and `G4` ledger-9 source-requirement binding. PR #95 registers `A102` (`literal_schema`, `emit_literal_step`) with 16 Rust tests `UNRUN`. `append-model` Quint passes 8 tests + seed-1 trace (9 K cases `UNRUN`). `ROOT_PHASE1` receipt (`4b20fae0...`) records 18 passing Quint arithmetic/fuel normal tests (`exit_code: 0`, post-rehash matched). | `ROOT_PHASE1` `trace_adjudication` is `PENDING` due to an `execve` regex bug rejecting ordinary whitespace in reconstructed `strace` text; `phase2` (sampled run) `NEVER_STARTED`; all 33 K checks (`K33checks`) `UNRUN`; `A102` not compiled / 16 tests `UNRUN` (`native_authority: 0`); new `G4-BYTE` draft is unadopted with no proof or stagenet claim. |
| **Native Proof & Financial Precursors (`k13`, `R3-04`, `FeePool`, `DeFiFormal`)** | One-shot `native-owner-intent-2026-10-11` produced and independently verified a local proof for the `k13` signed financial circuit (6 hostile-witness proofs and 6 statement/byte mutations rejected; existential $R/s$ second attachment passed). PR #49 `FeePool` has local/static evidence for 23 actions and 162 cases. Preview loan/swap scoped runs exist. | `k13` is a single laboratory fixture; Task 3's 16-asset/16-duty shape and 1 MiB production-preparation refusal remain unresolved. `R3-04` failed on excess-NIGHT test genesis supply. Full `M11` Capacity Exchange, `U4` native recursion (ledger-9 route without `VerifyProof`), `U5` federation/solvers, and `U6` full ACTUS (277) / DeFi (72) conformance remain open. |

---

### 1.2 Assigned Lens Deep-Dive: OpenSpec/EARS Coherence & Roadmap Traceability Audit

#### A. Stale Scope and Status Assertions Across Supplied Texts
1. **Stale Arithmetic/Fuel Execution Claim in `ROADMAP.md` vs. `ROOT_PHASE1` Receipt**:
   - *Citation*: `ROADMAP.md` ("Sherlock source status — 2026-10-16", para. 4) states that an internal Unix socket call stopped the sequence and *"all 18 tests, the sampled run and all K checks remain UNRUN."*
   - *Inconsistency*: The supplied `ROOT_PHASE1_POST_REHASH_AND_NORMAL_OUTPUT` receipt (`2026-10-16T12:54:20.679231+00:00`, `capture_sha256: 4b20fae0447274a5974ba5d21a39d7eac4d6342943de944fe4b5b3708a9a4ebd`) establishes that the 18 Quint normal tests (`arithmeticNormal1Test`–`8Test`, `fuelNormal1Test`–`10Test`) **did** execute once with `exit_code: 0`, `parent_timeout: false`, and matched `source_closure` (63 files), `package_closure` (4667 files), and `tool_proposal_P2_pins`. What actually blocks Phase 1 completion is a post-run `strace` adjudication query bug (an `execve` regex that rejected ordinary whitespace in reconstructed unfinished/resumed `strace` lines), while `phase2` is `NEVER_STARTED` and `K33checks` remain `UNRUN`.
2. **Stale Unmerged Claims vs. Pinned `main` (`9bcbca52...`, PR #95–#101 Merged)**:
   - *Citation*: `ROADMAP.md` ("Sherlock source status — 2026-10-16" and four stacked "Core continuation" sections from 2026-10-15) only cites PR #95 (`A102`) and describes the Core continuation candidates (58, 78, 86, and 88 tests) as *"local, unpublished and unmerged"*, without reflecting PR #96–#101 merged into `main` (`9bcbca52...`), notably `compile_reported` (which runs the Core backend compiler and appends a Sherlock projection report) and the unadopted `G4-BYTE` draft.
3. **Contradictory Lean/Clean Architectural Status Within `ROADMAP.md`**:
   - *Citation*: `ROADMAP.md` ("Sherlock design work — 2026-10-15", para. 3) states: *"The design includes a required Lean/Clean stage for semantic definitions, bounded representations, a transition circuit and correspondence obligations... the Clean step circuit, correspondence and Midnight exporter remain open."*
   - *Inconsistency*: `ROADMAP.md` ("Authoring tools and libraries", para. 2) states *"Lean is not a Moriarty dependency or roadmap workstream"*; `docs/MORIARTY-PRODUCT-CONTRACT.md` ("Execution target and certified basis", para. 2) states *"Lean is not a Moriarty compiler, developer-tool, proof-production or deployment dependency"*; `traceability.md` ("Conflict dispositions") marks *"Optional Lean bridge and planned Lean paths: Superseded as active delivery work"*; and the user prompt mandates *"Lean/Clean is developer QA, not public compiler/runtime/prover dependency."* Treating Lean/Clean as a required public compiler/exporter stage in `ROADMAP.md` is stale and must be reclassified strictly as internal developer QA.
4. **Stale Foreman/Pel and Model-Routing Clauses in OpenSpec (`spec.md`, `ROADMAP.md`, `COMPLETE_HEADING_INDEX`)**:
   - *Citation*: `ROADMAP.md` ("Immediate delivery brief", "Evidence and internal workflow") references *"Pel plan"* and *"Foreman/Pel governs project implementation only"*; `spec.md` `UNI-016` references *"static Pel check"*; `spec.md` `UNI-024` hardcodes model-routing bureaucracy (`GPT-6.1 Sol high` authorship and `Muse Wonder 1.0 high` review); and `COMPLETE_HEADING_INDEX` carries 8 duplicate headings named `"Internal maintainer audit and provenance"` (across `spec_paths` indices 9–16) plus `"Reviewed restart"` (index 11).
   - *Inconsistency*: Under the user override (**NO FOREMAN. Do not use or propose Foreman, Endstop, custom campaign/scheduler or receipt-review bureaucracy**), encoding model-review receipts or Pel/Foreman gates as normative EARS requirements violates the direct software-factory directive. Only objective automated verification and Opus security sign-offs remain valid gates.
5. **Blanket `specified-only` Status in `traceability.md` Violating `UNI-016` and `UNI-021`**:
   - *Citation*: `traceability.md` ("MPLR behavioral owners", "Backend requirement crosswalk", "Mina-derived refinement ownership") marks every single row (`MPLR-001–035`, `ZR01–ZR16`, `MNR01–MNR08`) as `specified-only`.
   - *Inconsistency*: `UNI-016` and `UNI-021` require separating *specified, inspected, tested, proved-under-assumptions, locally accepted, and target-finalized* evidence per child component while keeping parent exits open. A single static string `specified-only` obscures where local Rust/Quint/K child implementations exist (e.g., `MPLR-001`/`002` local continuation tests, `MPLR-012` `mori test` CLI, `MPLR-020` `G4`/`aux/2` source drafts) versus where zero code exists.

#### B. Missing Acceptance Predicates in OpenSpec / EARS
1. **Absence of Midnight Stagenet in Normative `spec.md` (`UNI-001–024`) and `traceability.md`**:
   - `ROADMAP.md` ("Sherlock design work — 2026-10-15") incorporates the October 15 stagenet requirement (`raw/assignments/moriarty-stagenet-acceptance-2026-10-15.md` and `docs/SHERLOCK-CORRECTNESS-LOOP.md#required-stagenet-end-to-end-evidence`) across `U0/U1/U2/U4/U7`.
   - Yet `spec.md` (`UNI-016`, `UNI-022`, `UNI-023`) only mentions *"Preview-finalized"* and *"Preview and Fenrir evidence"*. There is **no EARS clause in `spec.md`** defining the acceptance predicate for Midnight stagenet compilation, proof verification, deployment/call, and effect readback.
2. **Absence of EARS Predicates Governing `compile_reported` vs. Core Compiler Capabilities**:
   - Because `compile_reported` invokes the full Core backend compiler (which supports state, assets, transfers, and liabilities) and then appends a Sherlock projection report (which is currently scalar-only), OpenSpec lacks a predicate ensuring that:
     - (a) A Sherlock projection refusal/limitation in `compile_reported` does **not** block or degrade Core ZKIRv3 compilation of stateful financial constructs; and
     - (b) A scalar-only Sherlock projection report is **never** reported as full financial state/liability correspondence (`UNI-005`/`UNI-018`).
3. **Absence of Production vs. `cfg(test)` Frontend Separation Predicate**:
   - `ROADMAP.md` cites `sherlock-draft-source` (`Factorial(4)=24`, tree fold `=7`) while noting it is a private test-only envelope using a cached `rlib` lexer. `UNI-018` requires naming the executable and consumer, but lacks an explicit EARS prohibition preventing `cfg(test)`-only frontends or unverified cached `.rlib` artifacts from satisfying public `U0/U2/M1/M10` compilation gates.
4. **Omission of `MPLR-035` and `OSR-1014` from `COMPLETE_HEADING_INDEX`**:
   - `traceability.md` lists `MPLR-035` (*"Constraint-preserving solver completion"*, owner `U2/U5`, `UNI-004`). However, `COMPLETE_HEADING_INDEX` (`spec_paths[2]`: `kernel-security-and-ai-solvers/spec.md`) ends at `[2, "MPLR-034: Logical paid-request identity", ...]` and completely omits `MPLR-035`.
   - `requirements.md` notes that `OSR-1014` (`security-reconciliation-2026-10-14.md`) is outside the 160-heading index. Until `MPLR-035` and `OSR-1014` are integrated into the canonical index, automated requirement-coverage checks against `current-requirement-crosswalk.json` have a blind spot on solver constraint preservation and Opus security clauses.
5. **Unmapped Sherlock Circuit Sub-Gates (`G4`, `G4-BYTE`, `IO1–IO8`, `E1–E7`, `E-FUEL`, `A102`)**:
   - `ROADMAP.md` defines concrete acceptance blockers (`G4` Opus `B1–B13`, `G4-BYTE`, `aux/2` `E1–E7` & `E-FUEL`, `IO1–IO8`, `CandidateA P3` arena policy, `A102` `literal_schema`/`emit_literal_step`), but neither `spec.md` nor `traceability.md` maps these identifiers to `UNI-003`/`UNI-014`/`ZR01–ZR16` or defines their exact formal/native/target exit criteria.

#### C. Structural Conflicts Among `U0–U7`, `M1–M17`, `MC01–MC08`, and `SP01–SP12` Gates
1. **Cross-Milestone Ownership Deadlock in `COMPLETE_HEADING_INDEX` vs. `traceability.md`**:
   - In `COMPLETE_HEADING_INDEX`, `UNI-001` through `UNI-017` are each assigned `["U0","U1","U2","U3","U4","U5","U6","U7"]`, `MPLR-001–018` are assigned `["U0","U2","U3","U4","U5","U6","U7"]`, and `MPLR-019–034` are assigned `["U0","U1","U2","U3","U4","U5","U7"]`.
   - If `U0`, `U1`, or `U2` cannot close while any heading tagged `"U0"` remains `"open"`, then `U0` is blocked on `UNI-011` (federated adapters, `U5`), `UNI-012` (delegated AI budgets, `U5`), and `UNI-009` (native recursion, `U4`).
   - *Resolution*: Distinguish **Milestone-Closing Primary Ownership** (as defined in `traceability.md`, e.g., `UNI-011..013 -> U5`, `MPLR-001..010 -> U3`) from **Invariant Non-Violation** in earlier milestones.
2. **The `U2` / `U4` / `U6` (`MC05` / `SP09` / `SP07` / `SP08`) Dependency Loop**:
   - `ROADMAP.md` ("One implementation sequence") states `U4` acceptance depends on *"accepted SP07/SP08 semantic inputs for each claimed successor scope"*, while `traceability.md` assigns `SP07` and `SP08` exclusively to `U6`, and `U6` depends on `U2` and `U3/U4/U5`.
   - *Resolution*: Split `SP07`/`SP08` into **(a) `SP07/SP08-Spec` (Semantic & Numeric Domain Contracts)** delivered in `U0/U2` (`Sprint 1` & `Sprint 3`) so `U4` (`Sprint 7`) has frozen semantic inputs without circularity, and **(b) `SP07/SP08-Impl` (Full 277-ACTUS & 72-DeFi Library Conformance + `SP09/MC05` Requalification)** delivered in `U6` (`Sprint 9`).
3. **Unestablished Fenrir Parameters (`M2`, `M3`, `M4`, `M7`, `M11`, `M12`, `M17`) vs. `U0/U1/U2` Closure**:
   - `traceability.md` maps `M2` (qualified Fenrir devnet), `M3` (Fenrir prove/verify/deploy), `M4` (DUST/Capacity Exchange), `M7` (intent bound to network ID and Anchor), and `M12` (every-block Express Bus Registry) into `U0`, `U1`, and `U2`.
   - Because Fenrir configuration, version, `network_id`, `Anchor`, and committee identities are **not established** (`UNI-022`), whereas **Midnight stagenet is explicitly required**, coupling `U0/U1/U2` closure to live Fenrir execution would deadlock V1.
   - *Resolution*: Sprints bind `U0/U1/U2/U3/U4/U7` target execution to **Midnight stagenet** (and local/Preview), while implementing `M2/M3/M4/M7/M11/M12/M17` with parameterized target-tuple configurations (`NetworkId`, `AnchorDigest`, `RegistryAddress`, `CommitteePolicy`) verified locally and on Midnight stagenet, holding live Fenrir network execution as a target-specific qualification column under `UNI-022`/`UNI-023`.
4. **Identifier Collision Between Legacy Roadmap Aliases (`P1`, `P3`, `P4`, `C4`) and Sherlock Circuit Work**:
   - `traceability.md` defines `P1 -> U1`, `P3 -> U0–U2 authoring support`, `P4 -> U3, U4`, and `C4 -> U6, U7`. Meanwhile, `ROADMAP.md` (2026-10-16) uses `P1` for the Sherlock `full39` semantic map, `P3` for the `CandidateA` arena policy, `P4` for `aux/2` arithmetic/fuel lanes, and `C4` for `A102` (`literal_schema`/`emit_literal_step`).
   - *Resolution*: All sprint specifications below explicitly prefix Sherlock circuit sub-packages as `SH-P1 (full39)`, `SH-P3 (Arena)`, `SH-P4 (Arith/Fuel)`, and `SH-C4 (A102)` while preserving legacy `P0–P7` and `C0–C4` aliases intact.

---

## Section 2: Prioritized Capability Sprints (`S1`–`S10`)

### 2.1 Master Requirement Family Preservation Matrix

Every retained requirement family from `ROADMAP.md`, `MORIARTY-PRODUCT-CONTRACT.md`, `requirements.md`, `spec.md`, `traceability.md`, and `COMPLETE_HEADING_INDEX` is mapped to an owning capability sprint below:

| Requirement Family | Complete Retained Denominator / Scope | Primary Owning Sprints |
|---|---|---|
| **Unified Milestones (`U0–U7`)** | `U0` (`S1`), `U1` (`S2`), `U2` (`S3, S4`), `U3` (`S5, S6`), `U4` (`S7`), `U5` (`S8`), `U6` (`S9`), `U7` (`S10`) | `S1–S10` |
| **Fenrir / Direct Language (`M1–M17`)** | `M1` (`S1, S3`), `M2` (`S1, S4`), `M3` (`S4`), `M4` (`S4, S6`), `M5` (`S2, S7`), `M6` (`S1, S3, S7`), `M7` (`S3, S4`), `M8` (`S2`), `M9` (`S1, S3`), `M10` (`S1, S10`), `M11` (`S6, S9`), `M12` (`S3, S4`), `M13` (`S5, S6`), `M14` (`S6`), `M15` (`S9`), `M16` (`S8`), `M17` (`S8`) | `S1–S10` |
| **Milestone Contracts (`MC01–MC08`)** | `MC01` (`S1–S3`), `MC02` (`S3, S4, S10`), `MC03` (`S2, S7`), `MC04` (`S1–S7`), `MC05` (`S3–S7, S9`), `MC06` (`S5–S7`), `MC07` (`S8, S9`), `MC08` (`S1, S10`) | `S1–S10` |
| **Sprint Lineage (`SP01–SP12`)** | `SP01` (`S1, S2`), `SP02` (`S1, S3, S10`), `SP03` (`S1–S3`), `SP04` (`S2–S4, S7`), `SP05` (`S3, S4`), `SP06` (`S2, S7`), `SP07` (`S1` spec, `S9` impl), `SP08` (`S1` spec, `S9` impl), `SP09` (`S3, S7, S9`), `SP10` (`S5–S7`), `SP11` (`S9`), `SP12` (`S10`) | `S1–S10` |
| **Release Gates (`G01–G24`)** | All 24 release gates (`G01–G24`) preserved individually without batch reduction | `S10` (inputs from `S1–S9`) |
| **Consolidated EARS (`UNI-001–024` + `025–027`)** | `UNI-001` (`S1, S3, S10`), `UNI-002` (`S1, S3`), `UNI-003` (`S2–S4`), `UNI-004` (`S3, S5, S8`), `UNI-005` (`S3, S5, S6`), `UNI-006` (`S5`), `UNI-007` (`S5`), `UNI-008` (`S6`), `UNI-009` (`S7`), `UNI-010` (`S7`), `UNI-011` (`S8`), `UNI-012` (`S8`), `UNI-013` (`S8`), `UNI-014` (`S1, S2, S9`), `UNI-015` (`S7, S8`), `UNI-016` (`S1, S10`), `UNI-017` (`S7`), `UNI-018` (`S1, S10`), `UNI-019` (`S1, S3`), `UNI-020` (`S1, S9`), `UNI-021` (`S1, S10`), `UNI-022` (`S1, S4`), `UNI-023` (`S7, S9, S10`), `UNI-024` (`S10`, de-bureaucratized), plus new `UNI-025` (`S1` `compile_reported`), `UNI-026` (`S4` Stagenet), `UNI-027` (`S2` Sherlock circuit gates) | `S1–S10` |
| **Backend & Mina Refinements (`ZR01–ZR16`, `MNR01–MNR08`)** | `ZR01, ZR06, ZR12, ZR13` (`S1, S2, S7`); `ZR03, ZR05, ZR07, ZR10, ZR11` (`S3, S5, S7`); `ZR02, ZR04, ZR08, ZR09` (`S7`); `ZR14, ZR15` (`S7, S8`); `ZR16` (`S7, S10`). `MNR01–MNR08` (`S2, S3, S5, S7, S8, S10`) | `S1–S3, S5, S7, S8, S10` |
| **Opus Security Clauses (`OSR-1014`)** | All `OSR-1014` clauses preserved unmodified; security design/audit gates owned by Opus | Opus review across `S2–S8, S10` |
| **MPLR Theory (`MPLR-001–035`)** | `001–010` (`S5, S6`), `011, 013, 027–029` (`S7`), `012, 014, 018, 022, 023` (`S3, S4`), `015, 016` (`S1, S3, S10`), `017, 019` (`S3, S5`), `020, 021` (`S2, S6`), `024, 026, 030–035` (`S5, S8, S9`), `025` (`S9`) | `S1–S10` |
| **Core Language & Aeon (`MOR-001–012`, `AEO-001–004`, `L1–L8`)** | `MOR-001–012` (`S1–S7, S9, S10`); `AEO-001–003` (`S1, S3`), `AEO-004` (`S8`); `L1–L3, L5` (`S1, S3, S5`), `L4` (`S5, S6`), `L6` (`S7`), `L7, L8` (`S6, S8`) | `S1–S10` |
| **Prior Aliases (`P0–P7`, `C0–C4`, `K0–K5`) & Sherlock (`SH-*`)** | `P0/C0/K0` (`S1`), `P1/K1` (`S2, S7`), `P2/C1/K2` (`S3–S5, S7`), `P3` (`S1, S3`), `P4/C2/C3` (`S5–S8`), `P5` (`S8`), `P6/C4` (`S9, S10`), `P7/K5` (`S8, S10`), `K3/K4` (`S8`); `SH-P1/SH-P3/SH-P4/SH-C4(A102)/G4/G4-BYTE/aux1/aux2` (`S2, S7`) | `S1–S10` |
| **ACTUS, DeFi & Composition Denominators** | **ACTUS**: 277 fixtures, 18 executable types, 32 dispositions, `DS01–07` (`S9`). **DeFi**: 72 rows, `DA01–24`, supplements, 3 held-outs, 8 intent cases, 8 regression classes (`S9`). **Composition**: all 5 `SP10.1` operators (`S7`) | `S7, S9` |

---

### 2.2 Detailed Capability Sprint Specifications (`S1`–`S10`)

#### Sprint 1 (`S1`): Unified Production Frontend, Semantic/Numeric Contract & `compile_reported` Boundary
- **Milestones & Lineage**: `U0`, `SP01`, `SP02`, `SP03`, `SP07-Spec`, `SP08-Spec`, `MC01`, `MC04`, `MC08`, `M1`, `M2`, `M6`, `M9`, `M10`, `L1`, `L3`, `P0`, `P3`, `C0`, `K0`, `MOR-001`, `MOR-002`, `MOR-004`, `AEO-001`, `UNI-001`, `UNI-002`, `UNI-014`, `UNI-016`, `UNI-018`, `UNI-019`, `UNI-020`, `UNI-021`, `UNI-022`, plus proposed **`UNI-025`** (`compile_reported` & production frontend boundary).
- **Prerequisites**: Baseline `main` (`9bcbca52...`, PR #95–#101 merged). Zero native execution required (`native_authority = 0`).
- **Owned Source Interfaces**:
  - Production parser/lexer/typechecker crate (`moriarty-frontend` / `moriarty-core`), eliminating reliance on the unregistered `cfg(test)` draft frontend (`sherlock-draft-source`) and unverified cached `.rlib` lexer.
  - `compile_reported` public API and report schema: invokes the existing Core backend compiler (preserving state, assets, transfers, and liabilities) and attaches a strongly typed `SherlockProjectionReport` that marks constructs outside the scalar-only Sherlock subset as `ProjectedUnsupported(Reason)` without failing or restricting Core compilation.
  - Versioned `U0` Numeric & Semantic Profile module (`NumericProfileV1`: canonical quote/base price orientation, unit dimension algebra, per-primitive directed rounding toward floor/ceiling/beneficiary, dust conservation, and guaranteed vs. fallible phase failure taxonomy).
  - Canonical `current-requirement-crosswalk.json` repair: add `MPLR-035`, link `OSR-1014`, add Midnight stagenet target column, and split milestone-closing owners from invariant tags.
- **Normative EARS Clauses**:
  - **WHEN** `compile_reported` is invoked on a valid stateful `moriarty/1` or `moriarty/2` program containing assets, transfers, state transitions, or typed liabilities, **THE** toolchain **SHALL** emit the full Core backend ZKIRv3 artifact and provenance map unchanged **AND SHALL** append a `SherlockProjectionReport` explicitly distinguishing scalar-projected sub-expressions from non-projected stateful constructs (`UNI-018`, `UNI-019`, `UNI-025`).
  - **WHILE** a frontend construct exists only under `cfg(test)` or depends on an unverified precompiled `.rlib` cache, **THE** capability registry **SHALL** reject any claim of `U0`, `U2`, `M1`, or `M10` production compilation support (`UNI-016`, `UNI-018`, `UNI-025`).
  - **WHERE** financial ratios, interest accrual, or unit conversions are evaluated or lowered, **IF** a program mixes incompatible asset/price dimensions or omits an explicit directed rounding mode and beneficiary policy, **THEN THE** production typechecker **SHALL** reject the program with a deterministic static diagnostic before IR generation (`UNI-002`, `UNI-014`, `UNI-020`).
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Multi-Asset Custody Vault with Fee Split & Liability Opening)*: Production-parsed program accepting a user deposit of `1000 USDC` (`NetworkId::MidnightStagenet`, `Issuer::CircleRef`), routing `5 USDC` fee to `FeePool`, locking `995 USDC` in vault custody, and opening a typed redemption liability of `995 vUSDC` to the depositor.
  - *Program B (Dimension-Checked Fixed-Rate Coupon Calculator with Directed Rounding)*: Structurally contrasting recursive/iterative pure-and-stateful calculation (`Factorial`/`TreeFold` + coupon schedule) exercising the unified production lexer/parser (replacing `sherlock-draft-source`) with exact quote/base ratio conversion and floor rounding to the protocol reserve.
  - *Complete Effects Verified*: Authenticated domain/frame, per-asset conservation ($\Delta\text{transfer} + \Delta\text{fee} + \Delta\text{custody} = 0$), separate liability equation ($\text{opening } 0 + \text{created } 995 - \text{discharged } 0 = \text{closing } 995$), and `compile_reported` output containing both complete Core IR and the bounded Sherlock scalar projection report.
  - *Rejection Tests with Valid Positive Controls*:
    1. Mutate Program A to net the `5 USDC` fee silently into the vault balance (`UNI-005` hostile witness) $\rightarrow$ rejected; positive control is Program A.
    2. Mutate Program B to multiply `base/quote` price by `base` quantity without orientation conversion (`UNI-014` hostile witness) $\rightarrow$ rejected at typecheck; positive control is Program B.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: K/Quint reference evaluator and Rust `mori simulate` / `mori test` agree on complete observations (`UNI-020` independently frozen expectations) for Programs A and B.
  - *Native*: None claimed (`native_authority = 0`).
  - *Target*: Static target matrix (`UNI-022`) pins compiler, ZKIRv3 version, Midnight stagenet configuration schema, and explicit `Unknown` placeholders for unestablished Fenrir parameters.

---

#### Sprint 2 (`S2`): Certified Primitive Basis, Sherlock Machine-Circuit Lowering & Native Feasibility
- **Milestones & Lineage**: `U1`, `SP01`, `SP03`, `SP04`, `SP06-feasibility`, `MC01`, `MC03-feasibility`, `MC04`, `M5`, `M8` (Opus security-owned), `P1`, `K1`, `SH-P1 (full39)`, `SH-P3 (Arena)`, `SH-P4 (Arith/Fuel)`, `SH-C4 (A102)`, `G4`, `G4-BYTE`, `aux/1`, `aux/2`, `MPLR-020`, `MPLR-021`, `MOR-010`, `ZR01`, `ZR06`, `ZR12`, `ZR13`, `MNR01–MNR03`, `MNR05–MNR08`, `UNI-003`, `UNI-014`, plus proposed **`UNI-027`** (Sherlock circuit & primitive gate).
- **Prerequisites**: `S1` (`U0` numeric profile and target pins). Any actual native prover/verifier execution requires explicit user authorization (`native_authority` remains `0` during planning).
- **Owned Source Interfaces**:
  - Root Phase 1 `strace` adjudicator script fix: replace the brittle `execve` regex with a whitespace-aware parser over unfinished/resumed `strace` lines so the already-captured 18-test Quint output (`capture_sha256: 4b20fae0...`) is adjudicated without a redundant model rerun, unlocking Phase 2 sampled simulation and the 33 K checks (`K33checks`).
  - `models/sherlock/machine-circuit/`: complete adoption of `G4-BYTE` under `G4` ledger-9 field/instruction assumptions (`Opus B1–B13`), `aux/1` host packing consumer wiring (6 tests), `aux/2` UInt128 arithmetic/fuel witness placement (`Owner E1–E7`, `Muse E-FUEL`, `IO1–IO8`), `CandidateA P3` arena equality-trace compiler binding, and `A102` (`literal_schema`, `emit_literal_step`, 16 Rust tests) compiled and linked into the circuit emitter.
  - Certified Jet / Primitive Basis (`MOR-010`, `UNI-014`, `MPLR-020`): checked `UInt128` add/sub/mul/div-rem with directed rounding, range/byte decomposition (`G4-BYTE`), and in-circuit JubJub Schnorr verification splice (`M8`, subject to Opus cryptographic audit), plus `M5` static/circuit size analyzer enforcing $\le 64\text{ KB}$ tx bytes and fraud-proof circuit depth $k \le 19$.
- **Normative EARS Clauses**:
  - **WHEN** a primitive jet or Sherlock machine-circuit component (`A102`, `aux/1`, `aux/2`, `G4-BYTE`, `P3` arena, or `M8` Schnorr) is registered for `U1` certification, **THE** component **SHALL** bind its exact input preconditions, `U0` numeric rounding relation, ZKIRv3 constraint fragment, valid-witness completeness proof/test, adversarial-witness soundness constraints (`WShape` enforcement), and measured logical-to-circuit gate count (`UNI-003`, `UNI-014`, `UNI-027`).
  - **IF** an adversarial witness satisfies producer-honest wire assignments while violating `G4-BYTE` byte-width bounds, `E-FUEL` monotone fuel decrement, or `WShape` range constraints, **THEN THE** emitted ZKIRv3 constraint system **SHALL** reject the witness during verification (`UNI-003`, `Opus B1–B13`, `E1–E7`).
  - **WHILE** a compiled transaction or step-circuit exceeds the `M5` 64 KB transaction limit or $k \le 19$ circuit row bound, **THE** cost analyzer **SHALL** refuse `U1/M5` qualification and emit a structured resource-overflow diagnostic (`MPLR-021`, `ZR12`, `ZR13`).
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Boundary-Width `UInt128` Interest & Ratio Step)*: Executes checked `UInt128` mul-div with floor and ceiling rounding near $2^{128}-1$ and exact `E-FUEL` decrement across `A102` and `aux/2`.
  - *Program B (Arena-Backed Multi-Frame Call & Schnorr Intent Binding)*: Exercises `aux/1` host packing, `CandidateA P3` arena lookups, and an `M8` JubJub Schnorr signature check over a canonical intent digest within $k \le 19$.
  - *Complete Effects Verified*: Exact logical-work counter vs. actual ZKIRv3 gate/row count, zero fuel underflow, and bit-exact match with Quint (`18` normal tests) and K (`33` checks).
  - *Rejection Tests with Valid Positive Controls*:
    1. Supply a hostile witness where 16 byte lanes sum to the correct field element modulo $p$ but one byte lane holds $257 \ge 256$ (`G4-BYTE` hostile witness) $\rightarrow$ rejected by range constraints; positive control is Program A with valid bytes $\in [0, 255]$.
    2. Supply a hostile witness mutating `E-FUEL` remaining fuel without executing the charged transition (`E-FUEL` hostile witness) $\rightarrow$ rejected; positive control is Program A.
    3. Supply an `M8` Schnorr witness with a mutated intent anchor digest (`M7/M8` hostile witness) $\rightarrow$ rejected; positive control is Program B.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: Root Phase 1 trace adjudicated (`4b20fae0...`), Phase 2 sampled run and `K33checks` pass, Lean/Clean developer QA confirms `WShape` and `G4-BYTE` lemmas (strictly as internal QA, not public dependency), and Opus signs off on `B1–B13` / `M8` soundness.
  - *Native*: Local ZKIRv3 prover/verifier (when native execution is authorized) verifies valid witnesses and rejects all hostile witness mutations within $k \le 19$.
  - *Target*: Circuit/key identities and measured gate costs recorded against pinned `ledger-9` ZKIRv3 surface.

---

#### Sprint 3 (`S3`): General Single-Stage Source-to-ZKIRv3 Compiler & Signed Intent Pipeline
- **Milestones & Lineage**: `U2-Core`, `SP02`, `SP03`, `SP04`, `SP05`, `SP09-subset`, `MC01`, `MC02`, `MC04`, `MC05-subset`, `M1`, `M6`, `M7`, `M9`, `M12`, `L1`, `L2`, `L3`, `L5`, `P2`, `P3`, `C1`, `K2`, `MPLR-012`, `MPLR-014`, `MPLR-015`, `MPLR-017–019`, `MPLR-022`, `MPLR-023`, `MOR-001–006`, `MOR-009`, `MOR-012`, `AEO-001–003`, `ZR03`, `ZR05–ZR07`, `ZR10`, `ZR11`, `MNR04`, `MNR05`, `UNI-001–005`, `UNI-018–020`.
- **Prerequisites**: `S1` (Unified frontend & `U0` contract) and `S2` (`U1` primitive certificates).
- **Owned Source Interfaces**:
  - General single-stage lowering in `moriarty-compiler` (`moriarty/1 -> ZKIRv3`): full grammar coverage over admitted expressions, asset transfers, fee routing, custody/reserves, authorized supply mint/burn, and typed liabilities.
  - Authenticated Intent & Provenance Binder: upgrades PR #47/#49 mapped-role provenance (`core-source-provenance`) from unauthenticated static mode to cryptographic owner-intent binding (`M7` `network_id` + `anchor`, `M9`/`L3` `(network, issuer, reference)` asset identity, `L2` domain-qualified callee + per-call cap, `L5` signed liability cap, and `M12` Midnight Express Bus Registry anchor interface).
  - Authoring Obligation & Counterexample Engine (`AEO-001`, `AEO-002`, `AEO-003`): emits inspectable trust/obligation reports and replays any refinement counterexample through the deterministic evaluator (`M6`) so unknown/timeout never reports as success.
- **Normative EARS Clauses**:
  - **WHEN** a developer compiles any program within the admitted `moriarty/1` grammar, **THE** compiler **SHALL** emit a pinned ZKIRv3 circuit and statement relation binding the versioned program hash, signed intent (`network_id`, `anchor`, asset identities, gross debit cap, fee cap, liability cap, minimum net outcome, nonce/expiry), predecessor state, and complete effects (`UNI-002`, `UNI-003`, `UNI-004`, `M1`, `M7`, `M9`).
  - **IF** a candidate execution introduces a new liability, changes a fee asset/recipient, or substitutes an asset without satisfying the signed intent's explicit substitution and liability-consent predicates, **THEN THE** compiled acceptance relation **SHALL** reject the execution even if net value is conserved (`UNI-004`, `UNI-005`, `MPLR-019`, `L5`).
  - **WHEN** the static refinement checker (`AEO-002`) reports a violation or solver timeout, **THE** toolchain **SHALL** replay concrete counterexample inputs in the deterministic evaluator (`AEO-003`, `M6`) **AND SHALL** classify timeouts or unverified assumptions as `UnresolvedObligation` rather than passing acceptance.
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Collateralized Credit Line Origination with Signed Liability Cap)*: Novel program (distinct from existing Preview loan/swap fixtures) where a borrower locks `2000 NIGHT` collateral, draws `1000 USDX` principal, pays `2 USDX` origination fee to `FeePool`, and consents to an opening liability of `1015 USDX` (principal + fixed upfront interest) under a signed `L5` liability cap of `1020 USDX` anchored to `M12` Express Bus block root.
  - *Program B (Multi-Recipient Royalty & Reserve Revenue Splitter with Supply Burn)*: Structurally contrasting single-stage program with zero liabilities that takes `500 GOV` tokens, burns `50 GOV` (authorized supply reduction), routes `400 GOV` across two domain-qualified recipients (`L2`), and deposits `50 GOV` into a protocol reserve under an exact minimum-net signed intent.
  - *Complete Effects Verified*: Complete state domain accounting for `NIGHT`, `USDX`, and `GOV`; gross debits (`2000 NIGHT`, `500 GOV`), fees (`2 USDX`), supply change (`-50 GOV`), custody (`+2000 NIGHT`, `+50 GOV`), liability evolution ($0 + 1015 - 0 = 1015$), and consumed nonce/authority.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Unconsented Liability Creation*: Modify Program A's witness so liability accrues to `1025 USDX` (exceeding the `1020 USDX` signed `L5` cap) while rebating `5 USDX` in cash to keep net balance equal $\rightarrow$ rejected (`UNI-004`/`MPLR-019`); positive control is Program A (`1015 USDX` liability).
    2. *Cross-Network Replay (`M7`)*: Replay Program A's valid signed intent against a statement with a different `network_id` or stale `M12` `anchor` $\rightarrow$ rejected; positive control is Program A with matching `(network_id, anchor)`.
    3. *Prover-Selected Key/Relation Substitution (`UNI-002`/`MOR-003`)*: Present a valid proof generated for a weakened circuit that omits the `L2` recipient check $\rightarrow$ rejected by bound verifier key / statement hash; positive control is Program B.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: K and Rust deterministic evaluators (`M6`) produce identical step traces and complete effects against frozen independent expectations (`UNI-020`).
  - *Native*: Local ZKIRv3 proof generation and independent verification (in a separate verifier process with no shared memory) pass for Programs A and B and reject all 3 hostile controls (when native run is authorized).
  - *Target*: ZKIRv3 artifacts pass offline target schema validation (`STAGENET-OFFLINE-README.md`); live target submission is gated in `S4`.

---

#### Sprint 4 (`S4`): Midnight Stagenet & Target Qualification Loop
- **Milestones & Lineage**: `U2-Target`, `SP04`, `SP05`, `MC02`, `MC04`, `M2`, `M3`, `M4`, `M7`, `M12`, `MPLR-012`, `MPLR-014`, `MOR-009`, `UNI-003`, `UNI-016`, `UNI-021`, `UNI-022`, plus proposed **`UNI-026`** (Midnight Stagenet End-to-End Acceptance).
- **Prerequisites**: `S3` (`U2-Core` compiler and signed intent pipeline) and explicit user authorization for native/stagenet execution.
- **Owned Source Interfaces**:
  - `mori-ledger` & Stagenet Adapter (`docs/SHERLOCK-CORRECTNESS-LOOP.md#required-stagenet-end-to-end-evidence`): upgrades the offline readback helper (`models/sherlock/STAGENET-OFFLINE-README.md`) into a live Midnight stagenet deployment, transaction submission, and complete effect readback verifier (`mori verify-effects`).
  - Genesis Supply & DUST Sponsorship Validator (`R3-04` repair + `M4`): implements early pre-submission supply conservation and genesis invariant checking (preventing the `R3-04` excess-NIGHT refusal) and binds real DUST / sponsorship funding holds (elevating PR #46/#48 local accounting to target transaction fee headers).
  - Target Tuple Registry (`UNI-022`, `M2`, `M3`): binds qualified target profiles separately for `LocalVerifier`, `MidnightPreview`, `MidnightStagenet` (mandatory), and `FenrirDevnet` (parameterized slot ready for external network ID/Anchor).
- **Normative EARS Clauses**:
  - **WHEN** `U2` target acceptance is evaluated, **THE** pipeline **SHALL** compile two structurally contrasting programs (`S3` Programs A and B) from production source to pinned ZKIRv3, generate native proofs, verify them in an independent verifier process, deploy and call them on **Midnight stagenet**, and verify that read-back ledger post-states match the independently frozen complete effect expectations byte-for-byte (`UNI-003`, `UNI-020`, `UNI-026`).
  - **IF** a test or deployment environment supplies an initial state or sponsorship funding envelope whose total native asset supply exceeds the ledger genesis invariant (as in `R3-04`) or lacks valid DUST fee coverage (`M4`), **THEN THE** pre-dispatch validator **SHALL** reject the envelope with a deterministic supply/sponsorship error before proof generation (`MC02`, `M4`).
  - **WHILE** Fenrir network ID, Anchor, committee, or deployed tuple identities remain externally unestablished, **THE** capability registry **SHALL** record `MidnightStagenet` and `MidnightPreview` evidence in their own target columns **AND SHALL NOT** transfer stagenet acceptance to `FenrirDevnet` (`M2`, `M3`, `UNI-022`).
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - Executes `S3` Program A (Credit Line Origination) and `S3` Program B (Royalty/Reserve Splitter with Burn) end-to-end on Midnight stagenet, plus re-qualifies the repaired `R3-04` sorted native financial payment with supply-consistent genesis funding.
  - *Complete Effects Verified*: Stagenet transaction hash, block/anchor identity, DUST sponsorship consumption (`M4`), on-chain contract state commitment, asset balance deltas, and read-back liability commitments matching `STAGENET-OFFLINE-README.md` JSON schemas.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Excess-NIGHT Genesis Supply (`R3-04` regression)*: Submit a genesis/funding configuration exceeding max NIGHT supply $\rightarrow$ caught and rejected deterministically by the pre-dispatch supply invariant check; positive control is the supply-consistent funding configuration.
    2. *Tampered Readback Effect JSON*: Mutate 1 unit of fee or liability in the stagenet post-state readback payload $\rightarrow$ `mori verify-effects` rejects with exit 1; positive control is the authentic stagenet readback payload.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: Pre/post-state transition satisfies the `U0` semantic relation (`UNI-002`).
  - *Native*: Independent local verifier accepts produced proofs and rejects mutated statements/witnesses.
  - *Target*: Confirmed Midnight stagenet inclusion and verified complete effect readback (`docs/SHERLOCK-CORRECTNESS-LOOP.md`).

---

#### Sprint 5 (`S5`): Staged Continuations, Conditional Settlement & Programmable Escrow
- **Milestones & Lineage**: `U3-Core`, `SP10-part1`, `MC04`, `MC05-subset`, `MC06-lifecycle`, `M13`, `L1`, `L2`, `L4`, `L5`, `P2`, `P4`, `C1`, `C2`, `C3`, `K2`, `MPLR-001–004`, `MPLR-006`, `MPLR-008`, `MPLR-017`, `MPLR-019`, `MPLR-024`, `MOR-006`, `MOR-011`, `MOR-012`, `ZR03`, `ZR10`, `ZR11`, `MNR04`, `UNI-004–007`.
- **Prerequisites**: `S3` (`U2-Core`) and `S4` (`U2-Target` stagenet pipeline).
- **Owned Source Interfaces**:
  - `moriarty/2` Continuation & `/4` Multi-Circuit Target Integration: completes and merges the local continuation closure (`docs/CORE-CONTINUATIONS.md`, 88 tests) into `mori-ledger` so native-call and stagenet execution support `/2` staged programs across $N+1$ circuits for $N$ `await` points.
  - Midnight Two-Phase Effect Compiler (`UNI-006`, `MOR-011`): models Midnight's guaranteed phase vs. fallible phase explicitly in IR and signed intentions; when the fallible phase fails, guaranteed-phase fees and nonce/authority consumption persist alongside explicit residual duties without falsifying global rollback or workflow completion.
  - Conditional Settlement & Programmable Escrow Engine (`UNI-007`, `MPLR-003`, `MPLR-004`, `M13`): distinguishes request recording, funded escrow reservation, eligibility, in-flight effects, and final delivery guarded by recipient consent + authenticated document predicates (`digest`, `schema`, `issuer_key`, `freshness_window`).
- **Normative EARS Clauses**:
  - **WHEN** a `moriarty/2` workflow executes a partial fill or pauses at a typed `await` (`MPLR-001`, `MPLR-002`, `MPLR-004`), **THE** state transition **SHALL** consume only the proportional stage authority, commit a typed continuation binding captured inputs/lets/prior events and residual duties, and preserve cumulative gross debit and fee caps (`UNI-005`, `M13`).
  - **WHEN** a target transaction's fallible phase rejects after its guaranteed phase commits fees, **THE** compiled transition **SHALL** record the guaranteed fee debit, consumed stage nonce, and unchanged/newly accrued residual duties as an explicit partial-failure state **AND SHALL NOT** mark the workflow settled or erase prior liabilities (`UNI-006`, `MOR-011`).
  - **WHILE** a funded programmable escrow awaits recipient acceptance and an authenticated document predicate, **IF** a caller supplies only a raw document digest without the required issuer signature/freshness proof, a funding receipt, or a mere timeout, **THEN THE** escrow contract **SHALL** refuse release and retain the funded reservation and delivery duty (`UNI-007`, `MPLR-003`).
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Roadmap Immediate Delivery Brief Discriminator — Two-Asset Programmable Escrow with Partial Fill)*: Spend at most `11 A` (including at most `1 A` fees) to receive at least `20 B` on successful completion. Stage 1 reserves `11 A` (`10 A` principal + `1 A` max fee). Stage 2 executes a permitted 50% partial fill (`5 A` principal + `0.5 A` fee $\rightarrow$ `10 B` delivered to authorized recipient upon valid document attestation + recipient consent) and emits a persistent continuation for the remaining `5 A` principal / `0.5 A` fee / `10 B` duty. Two independent route candidates (authored via standalone local tools without `U5` infrastructure) satisfy the same signed intention (`UNI-004`).
  - *Program B (Milestone-Gated Trade Finance Letter of Credit with Fallible Phase Fee Retention)*: Contrasting 3-stage (`2 await`) workflow where Stage 2 fallible inspection fails, retaining the `0.2 A` guaranteed-phase inspection fee while keeping the `100 A` principal locked and the inspection duty open for re-presentation.
  - *Complete Effects Verified*: Per-stage authority consumption, proportional partial-fill accounting (`5.5 A` spent, `5.5 A` remaining in continuation), guaranteed-phase fee retention on fallible abort, and Quint-to-Rust trace equivalence across all 88 continuation tests (rerun in full).
  - *Rejection Tests with Valid Positive Controls*:
    1. *Timeout / Digest Substitution for Delivery (`UNI-007`)*: Attempt to release the second `5 A` in Program A using a block-height timeout or an unauthenticated hash instead of recipient consent + valid document signature $\rightarrow$ rejected; positive control is Stage 3 completion with valid recipient signature and document proof.
    2. *Uncommitted Incomplete Candidate as Completed Workflow (`UNI-006`)*: Present Stage 1's funding receipt as proof of `20 B` delivery $\rightarrow$ rejected; positive control is completed Stage 3.
    3. *Extra Fee / Wrong Domain in Partial Fill (`UNI-004`/`UNI-005`)*: Route `0.6 A` fee on a 50% fill (exceeding proportional `0.5 A` cap or leaving insufficient fee budget for remainder) $\rightarrow$ rejected; positive control is `0.5 A` fee.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: Full 88-test Rust continuation suite rerun from clean build + Quint model correspondence verified via shared JSON trace comparator.
  - *Native*: Multi-circuit `/4` proofs ($N+1$ circuits) generated and independently verified for each stage of Programs A and B.
  - *Target*: Staged escrow deployment and multi-transaction continuation execution verified on Midnight stagenet with phase-specific effect readback.

---

#### Sprint 6 (`S6`): Recovery Authority, State/Work Caps, Joins & Fast-Path Batch Clearing
- **Milestones & Lineage**: `U3-Recovery`, `SP10-part2`, `MC04`, `MC05-subset`, `MC06-lifecycle`, `M4`, `M11`, `M13`, `M14`, `L4`, `L7`, `L8`, `P4`, `C2`, `C3`, `K2`, `MPLR-005`, `MPLR-007–010`, `MPLR-021`, `MOR-006`, `MOR-008`, `MOR-012`, `ZR10–ZR12`, `MNR04`, `UNI-005`, `UNI-007`, `UNI-008`.
- **Prerequisites**: `S5` (`U3-Core` continuations and programmable escrow).
- **Owned Source Interfaces**:
  - Recovery & Reserved-Fuel Controller (`UNI-008`, `L4`, `MPLR-005`, `MPLR-008`, `MPLR-009`): enforces strict separation between ordinary execution work budgets and reserved recovery/closure budgets; ordinary steps cannot dip into recovery reserves, and only the signed recovery authority after `deadline` (`L4`) or terminal work exhaustion may invoke recovery.
  - 128-ID State-Cap & Episode Rollover Manager (`ROADMAP.md` conflict disposition, `UNI-008`): enforces the 128-ID active state cap; when an episode reaches the 128-ID boundary, requires an authenticated rollover transition that carries forward all unresolved liabilities, cumulative spent counters, and replay nonces without erasure or budget reset.
  - Join & Late-Result Race Resolver (`MPLR-007`, `MPLR-010`): deterministic resolution when a late settlement proof races a post-expiry refund/recovery action.
  - `M11` FeePool + Capacity Exchange L2 Contracts & `M14` Single-Domain Swap Fast Path: elevates PR #49's finite local FeePool (23 actions, 162 cases) to the full `M11` Capacity Exchange contract and `M14` single-domain batch swap fast path using `L7` (`FeeCapability` type) and `L8` (separate meter-counter family with zero netting against asset or ordinary work counters).
- **Normative EARS Clauses**:
  - **WHEN** ordinary execution fuel is exhausted or a workflow reaches its `L4` deadline, **THE** runtime and circuit relation **SHALL** prohibit ordinary transitions from consuming the reserved recovery budget **AND SHALL** permit only an authorized recovery transition to consume that reserve while preserving all unpaid liabilities and replay state (`UNI-008`, `MPLR-005`, `MPLR-009`).
  - **IF** an episode rollover at the 128-ID state limit attempts to reset a cumulative spending cap, drop an open liability, or net `L8` meter counters against existing asset/work counters, **THEN THE** rollover transition **SHALL** be rejected (`UNI-005`, `UNI-008`, `L8`).
  - **WHEN** a late settlement delivery and an expiry recovery transaction contend for the same `U3` escrow continuation (`MPLR-007`, `MPLR-010`), **THE** state commitment **SHALL** admit at most one winner via unique continuation nonce consumption **AND SHALL** route any late-arriving evidence to an explicit post-recovery reconciliation state rather than double-spending or silently dropping obligations.
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Escrow Expiry Recovery vs. Late Settlement Race + 128-ID Rollover)*: Extends `S5` Program A where the remaining `5 A` escrow reaches expiry (`L4`) and ordinary work exhaustion; an authorized recovery key executes refund of `5 A` principal using the reserved closure fuel while preserving an accrued `0.1 A` custodian liability across a 128-ID state rollover boundary.
  - *Program B (`M11` Capacity Exchange + `M14` Single-Domain Batch Swap Fast Path)*: Executes a 4-party single-domain swap batch cleared through the `M14` fast path, sponsored via an `M11` Capacity Exchange reservation using `L7` `FeeCapability` and tracked on `L8` separate meter counters.
  - *Complete Effects Verified*: Separate ordinary vs. reserve fuel counters, `L8` meter counters isolated from asset balances, 128-ID rollover commitment carrying exact liability/replay accumulators, and `M11`/`M14` conservation.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Ordinary Spend of Recovery Reserve (`UNI-008`)*: Attempt to use the reserved recovery fuel for an ordinary swap step before expiry $\rightarrow$ rejected; positive control is authorized post-expiry recovery in Program A.
    2. *Debt Erasure on Rollover / Expiry (`UNI-008`)*: Attempt 128-ID rollover or expiry refund that omits the `0.1 A` accrued liability $\rightarrow$ rejected; positive control is Program A preserving the `0.1 A` liability.
    3. *`L8` Counter Netting*: Attempt to offset an `L8` cost-plane meter charge with a positive rebate in an asset balance $\rightarrow$ rejected; positive control is Program B.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: K/Quint and Rust simulation verify race-freedom, reserve isolation, 128-ID rollover conservation, and all 162 + extended `M11`/`M14` cases.
  - *Native*: Native ZKIRv3 proofs verify recovery, rollover, and `M11`/`M14` transitions and reject all 3 hostile controls.
  - *Target*: Midnight stagenet execution of `U3` recovery, late-race rejection (second contender rejected by ledger state/nonce consumption), and `M11`/`M14` contracts.

---

#### Sprint 7 (`S7`): Native Recursive History, Sherlock Step-Circuit & Private 5-Operator Composition
- **Milestones & Lineage**: `U4`, `SP04`, `SP06`, `SP09-subset`, `SP10`, `MC03`, `MC04`, `MC05`, `MC06`, `M5`, `M6`, `L6`, `P2`, `P4`, `C3`, `K1`, `K2`, `SH-P1/P3/P4/C4`, `MPLR-007`, `MPLR-011`, `MPLR-013`, `MPLR-022`, `MPLR-027–029`, `MOR-003`, `MOR-006–009`, `ZR01–ZR16`, `MNR01–MNR08`, `UNI-009`, `UNI-010`, `UNI-015`, `UNI-017`, `UNI-023`.
- **Prerequisites**: `S2` (`U1` basis & Sherlock circuit primitives), `S4` (`U2` stagenet path), `S6` (`U3` lifecycle invariants), and `S1` (`SP07/SP08-Spec` semantic inputs). Uses the demonstrated **ledger-9 recursion route** (which does not require waiting for a future `VerifyProof` instruction).
- **Owned Source Interfaces**:
  - Ledger-9 Native Recursive Prover & Verifier Schedule (`UNI-009`, `UNI-017`, `MC03`, `ZR01–ZR16`, `MNR01–MNR08`): implements transcript schedule (`MNR01`), deferred arithmetic/commitment obligations (`MNR02`), authorized verifier-key set binding (`MNR03`, `ZR05`), verified-value payload binding (`MNR04`, `ZR03`), production proving mode without test trapdoors (`MNR05`), artifact provenance (`MNR06`), batch soundness (`MNR07`), and component assurance (`MNR08`).
  - Sherlock Canonical Checkpoint & Step-Circuit Exporter (`M5`, `M6`): upgrades `sherlock-host-checkpoint-normal` (86 tests) to canonical serialized state/digest checkpoints and links `SH-P1/P3/P4/C4` into an executable ZKIRv3 step-circuit for bounded recursive execution and deterministic fraud-replay (`M5` $k \le 19$, `M6`).
  - Private Handoff & 5-Operator Composition Engine (`UNI-010`, `MC06`, `SP10.1`, `L6`): implements isolated witness handoff, authenticated private completeness domain (`MPLR-029`—preventing hidden reservations from masquerading as absence of debt), `L6` fan-in $> 1$ (multi-parent DAG join), and all **five `SP10.1` composition operators** without requiring the federated kernel:
    1. *Sequential composition* (`Seq`),
    2. *Disjoint parallel composition* (`ParDisjoint`),
    3. *Shared-state interleaving* (`Interleave`),
    4. *Atomic synchronization* (`SyncAtomic`),
    5. *Asynchronous messaging* (`AsyncMsg`).
- **Normative EARS Clauses**:
  - **WHEN** compliant history or `MC03` recursion is claimed (`UNI-009`, `UNI-017`), **THE** proof artifact **SHALL** establish an authenticated genesis base, at least two native recursive financial steps, and a bounded multi-parent join (`L6` fan-in $\ge 2$) satisfying `ZR01–ZR16` and `MNR01–MNR08` **AND SHALL NOT** substitute ledger-only induction or outer-only wrapper checks for recursive step verification.
  - **WHERE** a transaction transitions private state across separate parties (`UNI-010`, `MPLR-013`, `MPLR-029`), **THE** acceptance relation **SHALL** bind the authenticated completeness domain (proving no hidden reservation or omitted liability exists outside the disclosed/committed frame), enforce noninterference for hidden fields, and verify encrypted/out-of-band witness handoff availability.
  - **WHEN** programs are composed via any of the five `SP10.1` operators (`Seq`, `ParDisjoint`, `Interleave`, `SyncAtomic`, `AsyncMsg`), **THE** composed relation **SHALL** preserve cumulative resource caps, unique parent consumption (`ZR08`), and residual duties without requiring `U5` federated coordinator infrastructure (`MC06`, `UNI-023`).
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Two-Step Native Recursive Credit Line + Private 2-Parent Join [`L6`, `Seq`, `SyncAtomic`])*: Party 1 originates a private credit position (Step 1) and accrues/partially repays (Step 2); Party 2 independently originates a private liquidity note (Parent B); Party 3 executes a private multi-parent join (`L6` fan-in $= 2$, `SyncAtomic`) combining both histories to net-settle part of the debt while preserving residual private liability and hiding individual trade sizes.
  - *Program B (5-Operator Composition Suite Without Federation)*: Exercises `ParDisjoint` (two independent asset vaults), `Interleave` (shared fee-pool accumulator), and `AsyncMsg` (cross-contract committed message envelope with replay nonce) alongside `Seq` and `SyncAtomic`, plus a 2-cut Sherlock checkpointed computation verified via the ledger-9 recursion route.
  - *Complete Effects Verified*: Authentic genesis binding, deferred accumulator discharge (`MNR02`), zero double-consumption across `L6` DAG join, private completeness proof commitment, and public disclosure limited strictly to declared statement outputs.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Fabricated Genesis / Unauthorized Predecessor Key (`UNI-009`, `ZR04`, `MNR03`)*: Supply a valid recursive step proof whose parent proof uses an unapproved verifier key or unanchored genesis state $\rightarrow$ rejected; positive control is Program A.
    2. *Diamond Double-Spend of Common Ancestor (`ZR08`, `UNI-009`)*: Join two branches in `L6` that both consume the same predecessor note/authority $\rightarrow$ rejected by unique nullifier/consumption lineage; positive control is Program A with disjoint parent notes.
    3. *Hidden Reservation / Incomplete Private Domain (`UNI-010`, `MPLR-029`)*: Omit an active private encumbrance from the witness handoff completeness merkle/commitment proof $\rightarrow$ rejected; positive control is Program A.
    4. *Undischarged Deferred Limb (`MNR02`)*: Mutate the deferred accumulator commitment at the final recursive boundary $\rightarrow$ rejected; positive control is Program A.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: K/Quint and Lean/Clean (developer QA) verify compositional invariants across all 5 `SP10.1` operators and `MNR01–MNR08` accumulator soundness; Opus reviews `OSR-1014` privacy/recursion boundaries.
  - *Native*: Two-step recursive proof + `L6` 2-parent join proof generated and independently verified on the pinned ledger-9 native stack.
  - *Target*: Recursive and private handoff transactions deployed and verified on Midnight stagenet with complete effect readback.

---

#### Sprint 8 (`S8`): Federated DeFi Kernel, External Byte Adapter & Constrained AI Solvers
- **Milestones & Lineage**: `U5`, `MC07-kernel`, `M16`, `M17`, `L7`, `L8`, `P4`, `P5`, `C3`, `K3`, `K4`, `K5`, `MPLR-024`, `MPLR-026`, `MPLR-030–035`, `AEO-004`, `ZR14`, `ZR15`, `MNR03`, `MNR06`, `UNI-004`, `UNI-011–013`, `UNI-015`.
- **Prerequisites**: `S6` (`U3` escrow, recovery, and `L7/L8` meters) and `S7` (`U4` history and `ZR14/ZR15` interfaces). Note: Direct Midnight use never requires `U5` (`UNI-001`, `ROADMAP.md`).
- **Owned Source Interfaces**:
  - Exact-Byte External Adapter & Common Evidence Statement (`UNI-011`, `MPLR-026`, `MPLR-030`): binds exact decoded target bytes and effects to the common `(Intention, Stage, Domain, Epoch, EvidencePolicy)` statement, explicitly separating ZK, MPC, TEE, and external finality assumptions.
  - Federation Policy & Epoch-Migration Engine (`M17`, `UNI-015`, `ZR15`): codifies membership, threshold, ordering, equivocation slashing/refusal, availability, and epoch change rules (parameterized for the Fenrir committee identity), with an epoch-migration transition that preserves all open duties, liabilities, and consumed authority.
  - OWS Delegated AI Spending & Durable Concurrent Budget Manager (`UNI-012`, `MPLR-031`, `MPLR-032`): enforces attenuated owner authority and atomic, durable `spent + pending + reserved` budget accounting across concurrent solver tasks, retries, and failovers.
  - `x402` Service Lifecycle & Two Independent Constraint-Preserving Solvers (`UNI-004`, `UNI-013`, `MPLR-033–035`, `AEO-004`, `M16`): binds logical paid-request identity separating payment finality, TEE/solver result availability, and actual recipient delivery; integrates `M16` kernel cost-plane meters (`L7/L8`); and runs two independent solvers (plus optional `AEO-004` bounded synthesis) on the `U3` escrow program.
- **Normative EARS Clauses**:
  - **WHEN** the Federated Kernel dispatches an external adapter action (`UNI-011`), **THE** enforcing signer or adapter **SHALL** verify that the exact decoded target payload bytes match the proved `(Intention, Stage, Domain, Epoch)` statement **AND SHALL** reject any cross-intent proof reuse (`Proof(X)` used to sign `Y`), partial decoding, or policy downgrade.
  - **WHILE** concurrent delegated AI solver tasks share an attenuated owner budget $B$ (`UNI-012`, `MPLR-032`), **IF** concurrent requests attempt to reserve amounts $r_1, r_2, \dots, r_k$ such that $\text{spent} + \text{pending} + \sum r_i > B$ (including all `M16` fees), **THEN THE** budget manager **SHALL** atomically reject or defer the excess request even under copied parent capability tokens or failover retries.
  - **WHEN** a paid `x402` service request is retried after payment finality (`UNI-013`, `MPLR-033`, `MPLR-034`), **THE** workflow **SHALL** reconcile the existing payment via its authenticated logical request ID without a second charge **AND SHALL NOT** mark the recipient delivery duty discharged until actual recipient delivery evidence (not merely payment finality or TEE-local output) is recorded.
  - **WHEN** the federation epoch, verifier key, or committee policy rotates (`UNI-015`, `ZR15`, `M17`), **THE** migration transition **SHALL** preserve every active liability, residual duty, and consumed nonce/authority commitment, rejecting any upgrade that resurrects spent authority or alters a signed beneficiary.
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Live Two-Solver Coordinated `U3` Escrow + Exact-Byte External Adapter + `x402` Delivery)*: Runs the `U3` discriminator (spend $\le 11\text{ A}$ incl. $\le 1\text{ A}$ fee to receive $\ge 20\text{ B}$) through two independent live solvers proposing distinct valid routes (`UNI-004`, `MPLR-035`), paid via `x402` logical request IDs (`UNI-013`) under a shared `10 USDC` OWS delegated AI budget (`6 USDC` + `4 USDC` concurrent reservations, `UNI-012`), with the external leg verified by the exact-byte adapter (`UNI-011`).
  - *Program B (`ZR15`/`UNI-015` Committee Epoch Migration Under Active Duties & `M16` Cost-Plane Metering)*: Transitions an active federated clearing state from `Epoch 1` to `Epoch 2` (rotating threshold keys while preserving an open `50 B` settlement duty, consumed nonces, and `M16`/`L8` kernel cost-plane meters).
  - *Complete Effects Verified*: Exact-byte adapter payload match, durable concurrent reservation (`6 + 4 = 10 <= 10`), idempotent `x402` retry (`1` charge for `2` retrieval attempts), and intact duty/nonce commitments across `Epoch 1 -> Epoch 2`.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Copied Parent Budget Over-Reservation (`UNI-012`)*: Two concurrent delegated tasks with copied parent tokens request `6` and `5` units against a `10`-unit cap $\rightarrow$ second reservation rejected/deferred; positive control is `6` and `4` units.
    2. *Proof-for-X Used to Sign Y (`UNI-011`)*: Present a valid proof for recipient `Alice` with adapter bytes decoded for `Bob` $\rightarrow$ adapter rejects; positive control is matching `Alice` bytes.
    3. *TEE-Local Result Claimed as Delivery (`UNI-013`)*: Present finalized `x402` payment + TEE enclave attestation without recipient delivery receipt $\rightarrow$ delivery duty remains open; positive control is Program A with recipient delivery receipt.
    4. *Epoch Migration Resurrecting Authority or Changing Beneficiary (`UNI-015`/`ZR15`)*: Submit an `Epoch 2` migration state that clears the consumed-nonce set or alters the escrow beneficiary $\rightarrow$ rejected; positive control is Program B.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: Quint/Rust state-machine verification of concurrent reservation serializability, `x402` idempotence, and `ZR15` migration invariants; Opus security review of `UNI-011`/`UNI-012`/`OSR-1014`.
  - *Native*: Native proofs for `ZR15` epoch migration and solver intent refinement (`UNI-004`) verified independently.
  - *Target*: Stagenet settlement of the Midnight leg + verified exact-byte adapter execution and direct-Midnight independence check (confirming `S5` Program A still runs on stagenet with all `U5` services shut down).

---

#### Sprint 9 (`S9`): Composable Financial Libraries, Full ACTUS/DeFi Conformance & Extension Requalification
- **Milestones & Lineage**: `U6`, `SP07-Impl`, `SP08-Impl`, `SP09-full`, `SP11`, `MC05-full`, `MC07`, `M11-full`, `M15`, `P6`, `C4`, `MPLR-025`, `MPLR-026`, `MOR-003–006`, `MOR-010`, `MOR-012`, `UNI-014`, `UNI-020`, `UNI-021`, `UNI-023`.
- **Prerequisites**: `S1` (`U0` numeric profile & `SP07/SP08-Spec`), `S2` (`U1` certified primitives), `S4` (`U2` stagenet path), `S6` (`U3` lifecycle), `S7` (`U4` composition), and `S8` (`U5` solver/batch interfaces).
- **Owned Source Interfaces**:
  - Reusable Composable Financial Standard Libraries (`moriarty-stdlib`):
    1. **ACTUS Library**: reusable schedule generator, state transition engine, day-count/rounding conventions, and payoff evaluator covering all **18 executable ACTUS contract types**, **32 taxonomy dispositions**, **`DS01–07`**, and **277 ACTUS fixtures** with all expected fields (`UNI-023`).
    2. **DeFi & Clearing Libraries**: loans/claims, payments/escrow, constant-product/concentrated AMMs, vaults, gross-preserving netting (`MPLR-025`—preserving gross debits, fees, and individual counterparty duties while settling net transfers), redemption/loss allocation, margin, and **`M15` batch clearing** (uniform clearing price, solver bounds, oracle freshness/bounds, and liquidation) covering all **72 original DeFi rows**, **`DA01–24`**, adopted supplements, **3 held-out behaviors**, **8 intent cases**, and **8 regression classes** (`UNI-023`).
  - Independent Oracle & Conformance Harness (`UNI-020`, `UNI-023`): compares every library execution against independently derived and frozen arithmetic/effect expectations (declaring zero shared evaluator code) across separate columns for `Semantic`, `Local`, `Native`, `MidnightPreview`, `MidnightStagenet`, and `FenrirDevnet`.
  - `SP09` / `MC05` Extension Requalification Suite: requalifies source/Core, native proof, unique consumption, and mandatory-acceptance predicates across all `U6` financial extensions.
- **Normative EARS Clauses**:
  - **WHEN** any `U6` financial library primitive or contract (`ACTUS`, `DeFi`, `M15` batch clearing, or `MPLR-025` netting) is compiled and evaluated (`UNI-014`, `UNI-020`), **THE** implementation **SHALL** convert exact quote/base price orientations before applying directed rounding under the bound `U0` numeric profile **AND SHALL** match the independently frozen expected values for ordered effects, gross debits, fees, net receipts, dust, liability changes, residual duties, and logical work.
  - **WHEN** `MPLR-025` multi-party netting or `M15` uniform-price batch clearing executes, **THE** settlement relation **SHALL** preserve every participant's signed gross debit cap, fee cap, price orientation bound, and un-discharged liability **AND SHALL** reject any clearing solution where net balance conservation masks an unauthorized fee or violated individual limit (`UNI-004`, `UNI-005`, `M15`).
  - **WHILE** any single row of the frozen `UNI-023` manifest (277 ACTUS fixtures, 18 executable types, 32 dispositions, `DS01–07`, 72 DeFi rows, `DA01–24`, supplements, 3 held-outs, 8 intent cases, 8 regression classes) lacks verified evidence in a target column, **THE** conformance registry **SHALL** keep that cell and `U6`/`MC07` open without denominator reduction (`UNI-021`, `UNI-023`).
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Composable Multi-Leg Portfolio: ACTUS `PAM`/`ANN` Loan + Vault Collateral + `MPLR-025` Gross-Preserving Netting)*: Composes an ACTUS annuity/principal-at-maturity schedule with a collateral vault and a 3-party netting ring, demonstrating that library modules compose into a novel user program rather than running only isolated fixture runners.
  - *Program B (`M15` Uniform-Price Batch Clearing with Oracle Bounds & Margin Liquidation)*: Clears 6 bi-directional limit orders at a uniform quote/base clearing price with directed rounding to the `M11` FeePool reserve, while executing a bounded partial liquidation of an under-collateralized margin account under an authenticated price-oracle observation.
  - *Complete Effects Verified*: All 277 ACTUS fixtures and 72 DeFi rows + `DA01–24` + held-outs + intent/regression suites verified against the independent frozen oracle (`UNI-020`), plus full `SP09`/`MC05` mandatory-acceptance requalification.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Inverted Price Orientation / Premature Reciprocal Rounding (`UNI-014`)*: Compute `1 / round(p)` instead of directed rounding of the exact ratio `num / den` in `M15` batch clearing $\rightarrow$ rejected by numeric profile assertion; positive control is Program B.
    2. *Netting Hiding Gross Cap or Fee Violation (`MPLR-025`/`UNI-005`)*: Net two opposing trades so net transfer is `0` while charging one party a fee exceeding their signed fee cap $\rightarrow$ rejected; positive control is Program A.
    3. *Shared-Evaluator Oracle Substitution (`UNI-020`)*: Attempt to score conformance using `mori simulate` output as its own expected oracle $\rightarrow$ rejected by `UNI-020` provenance check; positive control uses the frozen independent expectation manifest.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: 100% of retained `UNI-023` ACTUS and DeFi denominators pass against the independent frozen oracle (`UNI-020`).
  - *Native*: Native ZKIRv3 proofs generated and independently verified for representative and held-out circuits across every executable ACTUS type (18) and DeFi family (`DA01–24`, `M15`), plus full `SP09`/`MC05` requalification.
  - *Target*: Composed financial programs (Programs A and B) and held-out conformance slices deployed and verified with full effect readback on Midnight stagenet.

---

#### Sprint 10 (`S10`): Permissionless Developer Release, Real LSP/MCP Tooling & Reproducible Qualification
- **Milestones & Lineage**: `U7`, `SP02`, `SP12`, `MC02`, `MC08`, `G01–G24` (all 24 gates), `M10`, `P7`, `C4`, `K5`, `MPLR-015`, `MPLR-016`, `MOR-001`, `MOR-002`, `MOR-005`, `MOR-007`, `ZR16`, `MNR05`, `MNR06`, `MNR08`, `UNI-001`, `UNI-016`, `UNI-018`, `UNI-021`, `UNI-023`, `UNI-024` (reconciled for NO FOREMAN).
- **Prerequisites**: `S1`–`S9` (`U0`–`U6` capabilities and evidence).
- **Owned Source Interfaces**:
  - Public `mori` CLI & Developer Toolchain (`M10`, `MC08`, `UNI-024`): production `mori compile`, `mori simulate`, `mori test` (retaining the >1 MiB 256-case streaming/aggregate contract from `core-test-report`), `mori verify-intent`, `mori prove`, `mori verify`, and `mori verify-effects`.
  - Real Rust `mori lsp` and `mori mcp` Servers (`UNI-024`): replaces the current Rust LSP/MCP stubs with working production servers backed directly by the `S1`/`S3` Rust compiler, obligation reporter (`AEO-001`), and counterexample replayer (`AEO-003`), superseding the disconnected TypeScript `Source/6 -> Core/5` beta preparation stub.
  - Permissionless & Two-Builder Reproducible Release Package (`UNI-001`, `MNR06`, `G01–G24`): strips all internal project/Foreman/Pel/reviewer metadata from public tool paths; verifies bit-identical compiler/circuit/verifier-key outputs across two independent clean build environments; and validates the complete `G01–G24` release matrix.
- **Normative EARS Clauses**:
  - **WHEN** an external developer installs the public `mori` toolchain in a clean environment without project metadata, reviewer receipts, Foreman/Pel artifacts, council keys, or hosted solver credentials (`UNI-001`, `MOR-001`, `MOR-002`), **THE** developer **SHALL** be able to author a novel non-fixture program, inspect obligations via `mori lsp`/`mori mcp`, run `mori simulate`, `mori test`, and `mori verify-intent` (`M10`), generate native ZKIRv3 proofs, submit via their own Midnight wallet to Midnight stagenet, and verify complete effects (`U7`, `MC08`).
  - **WHEN** two independent builders compile the same source repository and target pins from clean checkouts (`U7`, `MNR06`), **THE** toolchain **SHALL** produce byte-identical ZKIRv3 circuits, proving/verifying keys, and provenance manifests.
  - **WHEN** the `U7` release report is generated (`UNI-016`, `UNI-021`, `UNI-023`, `G01–G24`), **THE** report **SHALL** attach reproducible command transcripts and artifact SHA256 digests for every `U0–U7`, `M1–M17`, `MC01–MC08`, `SP01–SP12`, and `G01–G24` predicate, explicitly stating any external target fields (such as unestablished live Fenrir network parameters) that remain scoped outside stagenet qualification.
- **Positive Contrasting New Programs, Complete Effects & Rejection Controls**:
  - *Program A (Clean-Room External Developer Program — Perpetual Royalty Streaming & Conditional Buyout Contract)*: Authored from scratch in a clean external directory (outside all repo fixture trees), inspected via `mori lsp` / `mori mcp`, tested via `mori test`, proved, and submitted via an independent wallet on Midnight stagenet.
  - *Program B (Two-Builder Reproducibility & Permissionless Equivalence Specimen)*: Compiled and proved across two isolated Linux container builders, confirming identical artifact hashes and identical acceptance when all project/review metadata files are absent (`UNI-001`).
  - *Complete Effects Verified*: Identical SHA256 digests across two builders, working LSP diagnostics/hover/obligations and MCP tool calls, clean-room wallet submission on Midnight stagenet, and all `G01–G24` gates verified.
  - *Rejection Tests with Valid Positive Controls*:
    1. *Hostile Witness in Clean-Room Workflow (`UNI-001`)*: Remove a non-existent/mock reviewer receipt from Program A $\rightarrow$ still accepted (proving zero administrative gate); remove the valid ZKIRv3 proof or owner signature while supplying a mock receipt $\rightarrow$ rejected (`UNI-001` hostile witness); positive control is Program A with valid proof and owner signature.
    2. *Tampered Intent / Fee / Predecessor in `mori verify-intent` & `mori verify-effects` (`M10`)*: Mutate intent fee cap or predecessor state hash $\rightarrow$ `mori verify-intent` and native verifier reject with exit 1; positive control is Program A.
- **Observable Acceptance (Formal / Native / Target Distinction)**:
  - *Formal*: All `UNI-001–027`, `MPLR-001–035`, `MOR-001–012`, `AEO-001–004` scenarios linked to reproducible test/proof artifacts.
  - *Native*: Clean-room native proof generation, independent verification, and two-builder bit-for-bit reproducibility verified.
  - *Target*: Clean-room developer wallet deployment and effect readback on Midnight stagenet; all `G01–G24` release predicates satisfied; Opus final security sign-off completed.

---

## Section 3: QA Strategy and Fixture/Requirement Coverage

### 3.1 Four-Tier Verification Architecture (Formal / Local / Native / Target)
To enforce `UNI-016`, `UNI-020`, `UNI-021`, and `UNI-023` without conflating model tests with target execution, QA is structured into four distinct, non-substitutable tiers:

1. **Tier 1 — Independent Semantic & Formal Developer QA (Zero Native Authority Required)**:
   - **Independent Expectation Oracle (`UNI-020`)**: A standalone arithmetic and state-transition reference generator frozen *before* test execution, with zero shared code from `moriarty-core` or `moriarty-compiler`. Computes exact `U0` directed rounding, gross debits, fee splits, dust, custody deltas, typed liability evolution ($\text{opening} + \text{accrual} - \text{discharge} = \text{closing}$), consumed authority, residual duties, and logical work counters.
   - **K & Quint Reference Models (`M6`, `SP03`)**: Differential trace comparison between the Rust deterministic evaluator, the K reference semantics (`K33checks` + lifecycle suites), and the Quint models (`sherlock-k-a/1` 98 + 126 tests, `append-model` 8 tests, `arithmetic/fuel` 18 tests + Phase 2 sampled traces, continuation 11 tests). Every Quint/K suite must be bound to Rust execution via a shared JSON trace schema so model QA establishes actual Rust correspondence (`traceability.md` gap closure).
   - **Lean/Clean Developer QA (`ROADMAP.md` boundary)**: Used strictly as internal developer QA to check bounded machine-circuit lemmas (`WShape`, `G4-BYTE` decomposition, `E-FUEL` monotonicity, `CandidateA P3` arena invariants). Never required by or shipped in the public `mori` compiler, prover, or runtime.
2. **Tier 2 — Local Compiler, Static Provenance & Hostile-Envelope QA (Zero Native Authority Required)**:
   - Verifies production frontend parsing/typechecking (`S1`), `compile_reported` IR + Sherlock projection reporting (`UNI-025`), mapped-role field-by-field provenance (`core-source-provenance`), `M5` static size ($\le 64\text{ KB}$) and circuit row ($k \le 19$) bounds, and `mori test` (>1 MiB 256-case JSON output contract).
3. **Tier 3 — Native ZKIRv3 Proving & Adversarial-Witness QA (Gated by Explicit User Native Authorization)**:
   - Executes real Halo2/PLONK/KZG proof generation (`midnight-zk`) and **independent-process verification** (separate binary invocation reading serialized proof + public statement bytes only).
   - **Mandatory Valid Positive Control Rule (`ROADMAP.md` Immediate Delivery Brief)**: Every hostile-witness or negative proof test *must* be paired with a valid, feasible positive control proof over the exact same circuit and envelope schema, mutating *only* the targeted semantic/witness invariant (e.g., `G4-BYTE` limb $\ge 256$, `E-FUEL` skip, unconsented `L5` liability, `M7` wrong `network_id`, `ZR08` diamond double-spend, `MNR02` undischarged limb). Rejections caused by malformed envelopes or serialization errors do not count as semantic rejection evidence.
4. **Tier 4 — Target Ledger Qualification (`MidnightPreview`, `MidnightStagenet`, `FenrirDevnet`)**:
   - Executes real contract deployment, transaction submission (including guaranteed-phase vs. fallible-phase failure transitions `UNI-006` and DUST sponsorship `M4`), and complete effect readback via `mori verify-effects` (`docs/SHERLOCK-CORRECTNESS-LOOP.md`).
   - Maintains strict column separation (`UNI-022`, `UNI-023`): `MidnightPreview` historical runs, `MidnightStagenet` required V1 runs, and `FenrirDevnet` runs (held open as `BlockedOnExternalConfig` until Fenrir `network_id`/`Anchor`/`Committee` are supplied) never overwrite one another.

### 3.2 Complete Denominator Accounting & Anti-Fixture-Overfitting Rules
- **Preserved Conformance Denominators (`UNI-023`)**:
  - **ACTUS**: 277 fixtures and all expected fields, 18 executable contract types, 32 taxonomy dispositions, and `DS01–07`.
  - **DeFi**: 72 original action rows, `DA01–24`, adopted supplementary cases, 3 held-out behaviors, 8 signed-intent cases, and 8 regression classes.
  - **Composition (`SP10.1`)**: All 5 operators (`Seq`, `ParDisjoint`, `Interleave`, `SyncAtomic`, `AsyncMsg`) plus `L6` fan-in $> 1$.
  - **Release & Crosswalk Gates**: `U0–U7`, `M1–M17`, `MC01–MC08`, `SP01–SP12`, `G01–G24`, `ZR01–ZR16`, `MNR01–MNR08`, `MPLR-001–035`, `MOR-001–012`, `AEO-001–004`, `L1–L8`, and Opus-owned `OSR-1014`.
- **Anti-Fixture-Overfitting Rule**: No sprint may pass by hardcoding fixture IDs or special-casing the Preview loan/swap specimens. Every sprint (`S1–S10`) requires **two newly authored, structurally contrasting programs** plus held-out parametric property tests exercising the general compiler grammar and reusable library modules.

---

## Section 4: Direct Software Factory Workflow, Parallel Work, and Bottlenecks

### 4.1 Direct Engineering Workflow (NO FOREMAN / Zero Bureaucracy)
In accordance with the user's override (**NO FOREMAN. Do not use or propose Foreman, Endstop, custom campaign/scheduler or receipt-review bureaucracy**):
1. **Standard Git + Deterministic CI Only**: All work happens in standard feature branches merged into `main` (`9bcbca52...` baseline) via deterministic, scriptable CLI checks (`cargo test`, `mori test`, Quint/K differential scripts, and `current-requirement-crosswalk.json` schema validation).
2. **Elimination of Receipt/Campaign Bureaucracy**: No Foreman packets, no Pel plans, no Endstop daemons, no custom orchestration software, and no LLM-identity receipt checks (`UNI-024` is amended so acceptance depends solely on reproducible test/artifact hashes, plus Opus review on security-scoped changes).
3. **Hard Native/Resource Guard (`native_authority = 0` Default)**: Ordinary CI runs only Tier 1 (Formal/Model QA) and Tier 2 (Compiler/Static/Hostile-Envelope QA) with zero native proving or network calls. Tier 3 (Native ZKIRv3 proving) and Tier 4 (Midnight stagenet deployment) are invoked only as explicit, bounded command targets when the user grants native execution authority.

### 4.2 Parallel Development Tracks vs. Serialization Bottlenecks

```
Track A (Frontend & Core Compiler) : [S1: U0/Frontend/compile_reported] ──> [S3: U2-Core Compiler & Intents] ──> [S5: U3 Continuations & Escrow] ──> [S6: U3 Recovery/M11/M14]
                                                      │                                         │                                                           │
Track B (Sherlock & Native Basis)  :                  ├──> [S2: U1 Basis, G4-BYTE, A102, Aux] ──┼──> [S4: U2 Midnight Stagenet Loop] ───────────────────────┼──> [S7: U4 Recursion & 5-Op Comp]
                                                      │                                                                                                     │               │
Track C (DeFi Kernel & Libraries)  :                  └──> [S9-Prep: Independent Oracle & ACTUS/DeFi Source] ───────────────────────────────────────────────┴──> [S8: U5] & [S9: U6] ──> [S10: U7 Release]
```

- **Disjoint Parallel Work (Immediate Zero-Native-Authority Execution)**:
  - **Track 1 (`S1`)**: Unify production frontend (replacing `cfg(test)` `sherlock-draft-source`), codify `U0` numeric profile, and enforce `compile_reported` Core-vs-Sherlock boundary.
  - **Track 2 (`S2` Source & Model QA)**: Fix the Root Phase 1 `strace` regex whitespace bug to adjudicate `capture_sha256: 4b20fae0...`, adopt `G4-BYTE` source, wire `aux/1`, `aux/2`, `CandidateA P3`, and `A102` (`16` Rust tests) in local source/unit tests.
  - **Track 3 (`S5` Local Preparation)**: Reconcile the 88 Core continuation tests (`OBSERVATIONS-CLOSURE.json`) with `mori-ledger` interfaces and build the shared Quint-to-Rust trace comparator.
  - **Track 4 (`S9` Independent Oracle Preparation)**: Freeze the `UNI-020` independent arithmetic/effect expectations for all 277 ACTUS fixtures and 72 DeFi rows.
  - **Track 5 (`S10` Tooling Preparation)**: Replace Rust `mori lsp` and `mori mcp` stubs with real compiler-backed implementations.
- **Key Bottlenecks & Mitigations**:
  1. **Root Phase 1 Trace Adjudication & `native_authority = 0` Gate**: Phase 2 Quint sampling, `K33checks`, `A102` 16 Rust tests, and `aux/1` 6 tests are blocked by the Phase 1 `strace` regex whitespace bug and execution gates. *Mitigation*: Fix the offline `strace` parser first (reading the existing captured log without re-executing the model) so Phase 1 closes cleanly.
  2. **`mori-ledger` `/2` Continuation & Phase-Partial Lowering Gap**: `mori-ledger` currently supports `/1` single-stage calls and lacks `/2` multi-circuit (`N+1`) dispatch and Midnight guaranteed-vs-fallible phase accounting. *Mitigation*: Prioritize `S5`'s `mori-ledger` `/2` adapter immediately after `S3`'s single-stage `U2` compiler path.
  3. **Task 3 `1 MiB` Production-Preparation Refusal (`16-asset/16-duty` shape) & `M5` ($k \le 19$) Circuit Size**: Large multi-asset/multi-duty frames hit the 1 MiB preparation cap or $k \le 19$ row bound. *Mitigation*: Use `S2`'s `CandidateA P3` arena commitment and `S6`'s bounded episode decomposition so per-step circuit state stays strictly within $k \le 19$ and $< 64\text{ KB}$.
  4. **Opus Security Review Queue**: `M7`, `M8`, `G4` (`B1–B13`), `OSR-1014`, `UNI-003` soundness, and `U4/U5` privacy/adapter boundaries require Opus sign-off. *Mitigation*: Package self-contained specification + witness-constraint diffs for Opus review at the end of `S2`, `S3`, `S7`, and `S8`.

---

## Section 5: Risks, Disagreements, Unknown Facts, and Next Three Concrete Deliverables

### 5.1 Technical Risks & Inter-Document Disagreements

| ID | Risk or Document Disagreement | Impact on V1 Delivery | Proposed Resolution |
|---|---|---|---|
| **R1** | **Frontend Divergence (`cfg(test)` vs. Production)**: `sherlock-draft-source` (`Factorial(4)=24`, tree fold `=7`) exists only as an unregistered test-only frontend with an unverified cached `.rlib` lexer, separate from the production `moriarty/1` frontend and the TypeScript `Source/6 -> Core/5` beta package. | Developers cannot compile rich source functions through the public `mori` CLI, and test claims on the draft frontend do not transfer to `U2/M1/M10`. | Merge the lexer/parser/AST lowering into the single production Rust frontend in `S1` from source (no unverified cached `.rlib`) and retire `cfg(test)`-only frontend envelopes. |
| **R2** | **Misinterpreting `compile_reported` as Limiting Core to Scalars**: Because `compile_reported` appends a scalar-only Sherlock projection report, contributors may mistakenly block `U2/U3` stateful compilation on Sherlock circuit completion. | Artificial serialization of `U2/U3` behind full Sherlock stateful VM circuits. | Enforce `UNI-025` (`S1`): `compile_reported` uses the existing Core backend compiler for ZKIRv3 generation (which already supports state, assets, transfers, and liabilities) while emitting the Sherlock projection report as an advisory/scoped diagnostic. |
| **R3** | **Lean/Clean Role Contradiction in `ROADMAP.md`**: `ROADMAP.md` (p. 3) calls Lean/Clean and a "Clean Midnight exporter" a *required* stage, whereas `ROADMAP.md` (p. 6), `MORIARTY-PRODUCT-CONTRACT.md`, `traceability.md`, and the user prompt forbid Lean/Clean as a compiler, prover, or runtime dependency. | Wasted engineering effort building a second proving/export pipeline in Lean/Clean instead of targeting Midnight ZKIRv3 directly from Rust. | Amend `ROADMAP.md` in `S1` to state uniformly that Lean/Clean is optional internal developer QA for circuit/semantic lemmas only; the Rust compiler emits Midnight ZKIRv3 directly. |
| **R4** | **Incomplete Test Reruns & Unlinked Quint/Rust Traces**: Continuation observations (58, 78, 86, 88 tests) carried forward 29–71 tests by static call-path projection rather than fresh execution, and their 11 Quint tests have no automated Rust correspondence check. | Regressions in carried-forward call paths or semantic drift between Quint models and Rust implementation. | Require full test-suite execution (when test runs are authorized) and automated JSON trace equivalence between Quint/K and Rust in `S3`/`S5`. |
| **R5** | **Task 3 `16-asset/16-duty` 1 MiB Refusal & `R3-04` Genesis Supply Failure**: Both failures remain open in `ROADMAP.md` (2026-10-11 and 2026-10-01 observations). | Native financial transactions fail either at preparation (>1 MiB) or ledger application (excess-NIGHT genesis). | Repair genesis supply invariant checking in `S4` and bind `M5`/envelope size bounds in `S2`/`S3`. |

### 5.2 Explicitly Unknown External Facts
1. **Fenrir Target Configuration (`UNI-022`, `M2`, `M3`, `M7`, `M12`, `M17`)**: Fenrir version, `network_id`, `Anchor` genesis/block digest, Express Bus Registry contract address (`M12`), and Fenrir federation committee public keys (`M17`) are **unknown and unestablished**. They must be represented as explicit configuration parameters and marked `UnknownExternalFact` in `UNI-022` tables—never invented or faked.
2. **Midnight Stagenet Live Environment Parameters (`UNI-026`)**: Exact stagenet RPC endpoint, deployed `ledger-9` verifier version tuple, and live DUST sponsorship faucet/funding parameters must be pinned in `S1`/`S4` before live `U2` stagenet execution.
3. **Full Bodies of Non-Inlined Spec Files**: The exact clause text of `security-reconciliation-2026-10-14.md` (`OSR-1014`), `docs/MORIARTY-BACKEND-REQUIREMENTS.md` (`ZR01–ZR16`, `MNR01–MNR08`), and the 18 non-inlined `spec.md` files in `COMPLETE_HEADING_INDEX` were not inlined in this prompt and must be cross-checked directly in-repo during `S1`.
4. **Measured Circuit Gate Costs for `aux/2` (232 Logical Lanes) and `G4-BYTE`**: Logical lane counts exist in source drafts, but actual Halo2/PLONK row counts ($k$) on `ledger-9` remain unmeasured until `S2` native compilation is authorized and run.

---

### 5.3 The Next Three Concrete Engineering Deliverables

1. **Deliverable 1: OpenSpec & Traceability Reconciliation Patch + Root Phase 1 `strace` Adjudicator Fix (`S1` / `S2-Prep`, Zero Native Authority)**
   - **Scope**:
     - Update `openspec/changes/consolidated-language-kernel/specs/consolidated-language-kernel/spec.md`, `traceability.md`, `current-requirement-crosswalk.json`, and `ROADMAP.md` to:
       1. Add **`UNI-025`** (`compile_reported` Core-backend vs. Sherlock-projection boundary & production-frontend-only rule), **`UNI-026`** (Midnight stagenet end-to-end acceptance predicate), and **`UNI-027`** (Sherlock circuit sub-gates `G4`, `G4-BYTE`, `aux/1`, `aux/2`, `P3`, `A102`).
       2. Restore missing **`MPLR-035`** and **`OSR-1014`** entries in `current-requirement-crosswalk.json`, disambiguate `SH-P1/P3/P4/C4` from legacy `P0–P7`/`C0–C4`, split `SP07/SP08-Spec` (`U0`) from `SP07/SP08-Impl` (`U6`), and remove all Foreman/Pel/model-routing bureaucracy (`UNI-024`).
       3. Update `ROADMAP.md` to reflect the 18 passing Quint arithmetic/fuel normal tests in `ROOT_PHASE1` (`4b20fae0...`), PR #95–#101 merges, and the strict developer-QA-only status of Lean/Clean.
     - Fix the whitespace-rejecting `execve` regex in the Root Phase 1 trace adjudication script and run it strictly offline against the already-captured Phase 1 `strace` data (`capture_sha256: 4b20fae0...`) to produce the completed Phase 1 trace adjudication receipt without re-running Quint.

2. **Deliverable 2: Unified Production Frontend & `U0` Numeric/Effect Contract with `compile_reported` Separation (`S1` Core Deliverable)**
   - **Scope**:
     - Consolidate the lexer/parser/typechecker capabilities from the `cfg(test)` draft frontend (`sherlock-draft-source`) into the production Rust compiler frontend (built from source without cached `.rlib` dependencies).
     - Implement `NumericProfileV1` (enforcing quote/base price orientation, unit dimensions, directed rounding, and beneficiary rules) and verify that `compile_reported` compiles two contrasting stateful financial programs (Program A: Multi-Asset Custody Vault with Fee Split & Liability Opening; Program B: Dimension-Checked Coupon Calculator) to full ZKIRv3 via the Core backend compiler while appending an accurate `SherlockProjectionReport`.

3. **Deliverable 3: Sherlock `G4-BYTE` + `A102` + `aux/1` + `aux/2` ZKIRv3 Emitter Integration & Stagenet Readback Wiring (`S2` / `S4` Bridge)**
   - **Scope**:
     - Adopt the `G4-BYTE` source module under `Opus B1–B13` constraints and wire `A102` (`literal_schema`, `emit_literal_step`), `aux/1` host packing, `aux/2` checked `UInt128` arithmetic/fuel (`E1–E7`, `E-FUEL`, `IO1–IO8`), and `CandidateA P3` arena witness placement into a unified Rust circuit-emitter module ready for Opus security review and authorized test execution (16 `A102` tests, 6 `aux/1` tests, 33 K checks).
     - Wire the offline readback verifier (`models/sherlock/STAGENET-OFFLINE-README.md`) and the `R3-04` genesis supply invariant check into the public `mori verify-effects` and `mori verify-intent` CLI commands so the `U2` Midnight stagenet loop (`docs/SHERLOCK-CORRECTNESS-LOOP.md`) is turnkey-ready as soon as execution authorization is granted.

