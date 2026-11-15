# Design: the Sherlock end-to-end prototype

## Context

Sherlock is a universal, elastic proving co-processor for Midnight. A developer writes a Compact contract. A user runs a call on machines the user owns, and Midnight accepts the result by checking one blinded proof with the unchanged ledger 9.1.0.1 verifier. The parts exist on lane branches and in several OpenSpec changes. This change is the one place that shows the whole path, its domain model, its decisions and its coverage.

This file explains. Obligations are the EARS requirements in `specs/` and the requirements this change cites (proposal.md, "Reuse rule"). One table of this file is bound by a requirement: the stage dependency table below, which EUF-002 names. Nothing else here is an obligation. The domain model is `architecture/DDD.md`, the decisions are `architecture/decisions/` (`INDEX.md`), and the map from every component and LOOP row to requirements is `architecture/COVERAGE.md`.

## Goals and non-goals

Goals: one requirement home for every component of the prototype; every LOOP row mapped; every decision with its authority; the build order to a working prototype (`tasks.md`).

Non-goals: no code, key, proof, deploy or push; no change to a hard rule; no edit of another change or worktree; no restatement of a requirement that exists (D-026).

## Method (the drwatt architecture method, as applied)

The owner named the local drwatt work as the architecture method (`~/Projects/software-factory/tools/drwatt/`, and the profile `skills/specs/spec-architect/references/architecture-decisions.md`). This change applies it as follows.

| drwatt rule | Applied here |
|---|---|
| One authoritative requirement inventory; never copy statements (`docs/requirements-organization-guidelines.md`) | Each requirement has one home. This change cites existing IDs and adds only new obligations (D-026) |
| Capability sets answer "what capability does this belong to"; a shallow hierarchy around enduring responsibilities, not modules or teams | One OpenSpec capability per responsibility (30 specs). The three slices are authoring lanes, not sets. Contexts group capabilities (DDD.md section 8) |
| Categories answer cross-cutting concerns (`docs/requirements-categories.md`) | The hard rules are the cross-cutting concerns of Sherlock. COVERAGE.md section 4 lists, per hard rule, the requirements that state it |
| A requirement owns its check | Each requirement has a `Verification:` line that names one test, model, theorem or run, and says if it exists or must be built |
| Evidence stays distinct from authority; green is not acceptance | The status of a requirement is in its scenario "Status of <ID>", with the words MEASURED, BUILT, OPEN or DERIVED. A decision status of accepted states authority only |
| Formal views of one requirement: Run, Prove, Build (`proposed/formal-methods-workflow.md`) | Run: the reference executors (`sem::sl_run`, the Lean oracle, the Quint models). Prove: the Lean theorems (Mycroft spine, ZK-L1, ZK-L2, G4). Build: the Rust provers and the refinement gates (KRS suite, counterexample replay) |
| Proportionate DDD; MADR records bind only through mapped EARS IDs and checks | DDD.md keeps to contexts, map, aggregates, events and language. Each decision record has a Confirmation section with EARS IDs; INDEX.md lists them |
| Decision IDs reserved by one lead before parallel work (`decision-allocation.md`) | Fable-A reserved D-001 to D-029, D-030 to D-069 and D-070 to D-109 on the bus before the slices wrote records; cancelled IDs are not reused |

`dw validate` and `dw ears` do not apply: the repository keeps no drwatt `requirements.json` for Sherlock, and OpenSpec needs upper-case `SHALL`, which the drwatt grammar rejects (AUDIT.md of the spec audit). The checks of this change are `openspec validate --strict` and the EARS linter with `--require-ids --strict`.

## The end-to-end path

| # | Stage | Input artifact | Output artifact | Context (slice) | Requirements | State on 2026-11-15 |
|---|---|---|---|---|---|---|
| 1 | compile (native) | Compact source | ZKIR 3.1 files, `contract-info.json` | Native Compact (external) | ECC-001; MYC-FE-001 to MYC-FE-005 | BUILT: 33 of 36 suite programs compile with stock `compactc` 0.36.0 |
| 2 | compile (Mycroft) | ZKIR build | program image and ABI per circuit, or a refusal code | Compact Compilation (A) | ECC-002 to ECC-005, ECC-007, ECC-009 to ECC-014 | BUILT for 106 circuits; 6 calls wait on kits; no `sherlock compile` command |
| 3 | setup | the box | `sherlock-profile/1` | Scheduling (B) | STM; EUF-007 | MEASURED on huginn and odin |
| 4 | plan and admit | image, universal keys | `prog_d`, `key_root`, app key | Key Management, Leaf Proving (B) | ERS-006, ERS-015; UKS; LFP-006, LFP-011 | BUILT for chip sets 0 to 2; OPEN for R_SL |
| 5 | deploy | `C_P = H(SHK_PROGC, r_P, prog_d)`, wrap VK | SherlockLedger address, anchor in slot 15 | Network Operations, Settlement (C) | SLC; PVW; EGV-001 | BUILT on the T10m keys (T17 (a)); redeploy after T12 |
| 6 | tape | preimage, ABI | `rsl-call/1` call record (private) | Compact Compilation host (A) | ERS-001, ERS-002, ERS-013; MYC-TAPE | BUILT, two formats |
| 7 | execute | call record | `RslStatement` (private), trace; or a reject code | Sherlock Execution (A) | ERS-003, ERS-005 to ERS-011; EUI; EKT | MEASURED: 142 of 142 executed vectors equal to native on odin |
| 8 | prove | statement, trace, profile | app, leaf, N1 to N3, `L`, `L_Fr` proofs; `LfrProofReady` | Leaf Proving, Aggregation, Bridge (B) | LFP, NAG, LTR, CLB, ZKE, GPV, PDM, ELA | BUILT for the interpreted subset on the T10m keys; OPEN for R_SL (ERS-012, LFP-010) |
| 9 | wrap | `LfrProofReady` | `WrapOutput` (one blinded proof, k = 21, 1,026 public inputs) | Wrap (C) | WRP | MEASURED: 1,685,845 rows, VK 1,325 bytes on the T10m keys |
| 10 | ledger check | `WrapOutput` | `WrapVerdict` (on muninn) | Ledger Verification (C) | LVR; HLP-001 | BUILT |
| 11 | submit | `StatementPublics`, wrap proof | transaction | Network Operations (C) | SLC-006 to SLC-010; PVW; WAL; FRC | BUILT on Preview (T17 (a)) |
| 12 | accept and read back | transaction | `SettlementReceipt`, `runs[claim_pub] = outcome_pub` | Settlement (C) | STL-004; OBS-001; EUF-010 | MEASURED at block 115,187 (T17 (a)) |
| 13 | compare | receipt, native reference | suite table row | Conformance (A) | ECS; STL-005 | dry run only: 0 mismatch of 0 compared |

The run form applies to Compact calls. A natively refused Compact call stops at stage 7 (ERS-009, EUF-006). A general Sherlock run keeps its provable Rejected and Exhausted outcomes (EUI-005).

### Stage dependency table (EUF-002)

A stage reads only the inputs named in its row. "Private" inputs stay on boxes the user owns (ERS-002, HLP-004).

| Scope | Stage | Reads | Writes |
|---|---|---|---|
| per program | compile | Compact source; the `compactc` 0.36.0 build; compiler and ZKIR pins | program image, ABI, or a refusal code |
| per program | plan and admit | image; machine version; universal key-set identity (key manifest, UKS-004) | `ProgramAdmission`: `prog_d`, `key_root`, app key commitment |
| per program | deploy | `prog_d`; a fresh private `r_P`; wrap VK; chain pins; wallet (muninn) | deployment record: address, block, `c_p`, anchor slot, VK hash, key-set identity. `r_P` stays private |
| per box | setup | the box; the installed universal key set | `sherlock-profile/1` |
| per call | tape | ABI; the preimage (private); a fresh salt and `rand` (private) | `rsl-call/1` call record (private) |
| per call | execute | call record; image | `RslStatement` (private) and the trace, or a reject code |
| per call | prove | `RslStatement`; trace; `ProgramAdmission`; profile; universal keys | `ProofBundle`; `LfrProofReady` |
| per call | wrap | `LfrProofReady`; wrap key; fresh blinders | `WrapOutput` |
| per call | ledger check | `WrapOutput`; the stock ledger 9.1.0.1 verifier (muninn) | `WrapVerdict` |
| per call | build arguments | `RslStatement` (private); `r_P` (private); deployment record | the private accept bundle on the box; `StatementPublics` (the only part that leaves the box) |
| per call | submit | `StatementPublics`; wrap proof; deployment record; `WrapVerdict`; wallet | transaction hash, or a node refusal with its error |
| per call | accept and read back | transaction hash; contract address | `SettlementReceipt` |
| per call | compare | `SettlementReceipt`; `NativeCallReference`; call identity | one suite table row with its evidence set (ECS-014) |

A second call of a deployed program starts at the tape stage. A new universal key set repeats plan, deploy and setup. After submit, a call with no read-back is reconciled by its transaction hash and is never sent again (EUF-014).

No run joins stages 1 to 13 today (EUF-001). Stages 9 to 12 ran on Preview with a general run on the T10m keys. Stages 1, 2, 6 and 7 ran on the whole suite. The joint is stages 4 and 8 for R_SL programs.

## The joint that is open

The R_SL chip set proves that a program ran. It does not yet say which program, on which input, with which result, to the layers above it:

1. No `prog_d` for an R_SL program (ERS-006, LFP-011). `acceptRun` needs `c_p`, which commits to `prog_d`.
2. No INIT for R_SL, so `input_d` is not bound to the tape (ERS-007, LFP-012).
3. The statement words `comm` and W are trace rows of the PUBLIC AIR, not public values. Under the leaf layer only AIR 0 (FP-1) has public values, and the FP-1 probe carries zeros (openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/air.rs:198-222`). No recursion proof authenticates a Compact outcome. FP-1 has 54 assigned words, so the binding is a commitment to `comm` and the ordered W, not the raw words (ERS-012, LFP-010, NAG-008).
4. The per-program app key is not yet under `key_root` (ERS-015).

These four items are inputs of the single rekey (T25): each changes a keyed constraint. COVERAGE.md marks the component "Run statement and its binding" as a HIGH gap with owner lane rekey-chips.

## Run form and sealed form

The Mycroft design seals a Compact call as the native call vector, so the contract entry itself holds the universal wrap key and the ledger applies the effect words W (MYC-SEAL, MYC-CHIP-004). The prototype does not build that. It settles each call as a general run through SherlockLedger `acceptRun`, with outcome `Halted(vd([comm] ++ W))` (D-022, D-071). So "the contract works on Preview" means, for the prototype: the run claim is accepted by the stock verifier, `runs[claim_pub] = outcome_pub`, and W equals the native transcript words off chain. It does not mean that the contract's own ledger state changed. The suite report states this boundary (ECS-010, EUF-010). The sealed form stays the target after T15.

## Rule families

The Sherlock machine has two rule families under one universal key set (EUI-001, GPH-001). The interpreted family (120 step rules) serves general programs (GP-1). Its built part is 5 subset rules and 2 value rules (EUI-010). The straight-line family R_SL (16 rules, chip set 3) serves Compact Stage 1 and is lowered to micro-op AIRs inside a 6-or-9 AIR bucket, so every app key fits the universal leaf (D-017). The prototype demonstrates both families under the T25 keys (EUI-002). A program that calls SHA-256 and keccak256 has no R_SL proof today; the compiler gives a temporary named refusal, and the capability is incomplete until a path proves both under the universal key rule (EKT-004, D-024). The interpreted family has no such limit.

## Decisions

`architecture/decisions/INDEX.md` lists every record with its authority. Hard rules: D-001 to D-015 and D-018 (OWNER). The owner accepted the 21 records that were LEAD PICK for T15 on 2026-11-15 ("go with the recommended answers for the 21 decisions"); they are OWNER records now. Params, keys and key handling stay open until the end-to-end prototype (D-015). Conflicts between older texts that this change resolves:

| Conflict | Resolution | Record |
|---|---|---|
| CF-001 taps `Lflattened`; MYC-FE-001 reads stock ZKIR | Stage 1 reads stock ZKIR; CF-001 describes Stage 2 | D-016 |
| Three lists of R_SL rules (ARCHITECTURE.md 1.7, MYC-CHIP-001, code) | the code table of 16 rules governs | D-017 |
| LOOP.md row T23 and `t23.py` defer secp256k1 and keccak256; the owner said "yes to both" | both kits in the T25 rekey | D-019 |
| MYC-BLT-007 routes `keccak256` to a tier 3 kit | `keccak256` lowers to `R_SL_KECCAK256` | D-019, ECC-007 |
| MYC-REF-052 refuses a per-program preprocessing key; R_SL app keys are per program | per-program keys sit under `key_root`; universal means node, `L`, `L_Fr`, wrap | D-004, ERS-015 |
| MYC-CHIP-004 public values of `2 + L_TX` words; built R_SL has `comm` then W as rows | run form for the prototype | D-022, ERS-003, ERS-012 |
| Anchor cell (0) on the T10m keys; slot 15 confirmed | slot 15, a key constant | D-021 |
| `sherlock-universal-interpreter-design` (35 UIREQ) and processor-final | processor-final governs a demonstrated conflict; every other UIREQ stays in force until the retention map rules on it | D-025, EUI-009 |

## Capabilities of slice A (language, execution, integration)

| Capability | Prefix | Requirements | Scope | Reused, not restated |
|---|---|---|---|---|
| `sherlock-e2e-compact-compiler` | ECC | 12 | Stock `compactc` output to program images and ABIs; the compile command; suite coverage; equality with the Lean oracle | MYC (frontend, values, lowering, witness-tape, ledger-ops, builtins, refusals), CF-001 to CF-014, ANC-015, LPC-013, LPC-019 |
| `sherlock-e2e-rsl-statement` | ERS | 14 | The call record `rsl-call/1`, the private record `RslStatement`, `prog_d`, `input_d`, the outcome, and the binding of the statement into recursion | MYC-TAPE, MYC-TRN, MYC-CHIP-004, MYC-KEY, MYC-CLM-023, STL-001, INIT |
| `sherlock-e2e-rsl-kits` | EKT | 10 | The rule table of chip set 3 and the kits SHA-256, keccak256, Jubjub, secp256k1, Poseidon, ALU: interfaces, references, refusal of invalid traces | RSL-001 to RSL-008, MYC-BLT, MYC-CHIP, CF-005 |
| `sherlock-e2e-universal-interpreter` | EUI | 10 | The Sherlock machine: two rule families, `machine_version`, outcome kinds, executor agreement, rule coverage | GP-001, CORE, RFB, CPR-003, CPR-005, UIREQ (retained by reference) |
| `sherlock-e2e-compact-suite` | ECS | 14 | The Compact test suite and T23 Part B: manifest, native references, stages, result classes, evidence completeness | STL-002 to STL-006, MYC-ACC, MYC-LOW-018 |
| `sherlock-e2e-user-flow` | EUF | 12 | The joined call flow across every context: stage order, stage inputs, stop and reconcile rules, pins, completion | ZK-001, ZK-002, GP-ZK, LPC-008, LPC-014, CF-011 |

## Capabilities of slice B (proving)

Per capability: scope, the reused requirement IDs that the spec does not restate, and the hard rules it states. In the slice B specs each requirement body is the EARS sentence and its Verification line (ending 'exists', 'to build' or 'partly to build'); the evidence (file:line or commit) and the status are in the scenario 'Status of <ID>'. Reused IDs are cited by ID; the spec audit commits (0f859938e, 95643d85f) are in this branch. An If-then requirement of slice B that is the refusal side of a reused binding (CLB-002 for ANC-018, CLB-003 for ANC-019, CLB-005 for ANC-017, CLB-006 for ANC-016) leaves the reused ID normative for the binding and adds the refusal and its tests.

### sherlock-e2e-box-fleet

Scope: the three owned proving boxes and their tooling: muninn (lead, aarch64, GB10, ledger-check authority), huginn (RTX 5080), odin (RTX 5090). heavy-slot admission, `gpu.lock`, the box bus, file moves, durable runs. The muninn check of box wraps and the helper rule are HLP (slice C); the wallet is WAL.

Reused requirements, not restated here: ZK-018 (case B refused), LPC-005 (timing per box baseline), LPC-020 (rows named on the box queues).

Sources outside a repository: `~/.local/bin/heavy-slot`, `~/.local/bin/heavy-run`, `~/.local/bin/box-relay`, the `heavy.slice` unit, `~/.cache/box-bus/all.log`. They have no commit; the gap list names this.

Hard rule stated here: case A only, owned boxes (BXF-001).

### sherlock-e2e-call-binding

Scope: how `L` and `L_Fr` bind a call to the ledger: FP-3 (call lanes bound to guest effects, forward chain), the anchor slot 15 on the interface side, the `x_j` lock form, the chunk count `k` and the chunk cap. The wrap decoder (`decode.rs`) is WRP, the SherlockLedger contract is SLC, the Mycroft refusal of slot 15 is slice A.

Reused requirements, not restated here: ANC-001 to ANC-011 (change `sherlock-processor-final-20261111`; ANC-001 and ANC-002 as fixed by the spec audit), ANC-012 (branch `t20-batched` 5dc1be2ee), ANC-013 to ANC-019 (change `sherlock-completion-20261115`), HDM-001 and HDM-007 (as fixed on this branch at 95643d85f). FP-3 lane text: FP3-001 to FP3-014 in `~/Projects/Moriarty-wt/fp3/research/sherlock-tested-design-20261113/fp3/SPEC.md:45` (untracked).

Hard rules stated here: the `x_j` lock form is canon (CLB-005); K_MAX = 2 (CLB-007).

### sherlock-e2e-elastic-proving

Scope: row T16, the elastic property: the same run under different schedules, memory budgets, devices and boxes gives one `claim_pub` under one key set; leaves from two boxes join under one node.

Reused requirements, not restated here: SPL-011 (as fixed on this branch at 95643d85f), LPC-006 (two run lengths), LPC-007 (1x to 8x run length), SC-011, SC-012, SC-014 of `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`) (carried here as ELA-001, ELA-002, ELA-006), LFP-008, GPH-006.

Hard rules stated here: one universal key set for every schedule (ELA-002); case A, owned boxes only (ELA-006).

### sherlock-e2e-gp-hooks

Scope: GP-1 to GP-4 at the proving layer. GP-1 at the execution layer is EUI (slice A). The requirement text of GP-001 to GP-004 is in change `sherlock-vm-consensus-20261107`, spec `sherlock-general-purpose` (`spec.md:3`, `:17`, `:31`, `:41`).

Reused requirements, not restated here: GP-001 to GP-004 (above), CPR-005 and DG-010 (never narrowed; change `sherlock-processor-final-20261111`), CLM, CONT, FREG, FVR (change `sherlock-modular-leaf-20261111`), LPC-001, LPC-002, LPC-003 (change `sherlock-completion-20261115`).

Hard rule stated here: GP-1 to GP-4 never narrowed (GPH-007).

### sherlock-e2e-gpu-proving

Scope: the GPU paths of the STARK stages: the BabyBear CUDA backend for app, leaf, nodes and `L`, and the GPU hashing of `L_Fr`. The GPU wrap is WRP (GPW-001 to GPW-008).

Reused requirements, not restated here: SPL-007 (STARK engine and production settings), SQG-014 (the GPU prover leg of G3), DG-006 (seconds per proved step), LPC-005, ZK-017.

Code paths: openvm `t14` and `e4` clones under `~/.cache/m7-wrap/src/` (`openvm-t14`, `openvm-e4`), openvm `t16-elastic` 1baf605.

### sherlock-e2e-l-transcript

Scope: the bridge layer `L`, the Fr-native layer `L_Fr`, the `L_Fr` tape and the transcripts. Output: the published record `LfrProofReady`, the input of the wrap (slice C, WRP).

Reused requirements, not restated here: LW-001 to LW-019 (change `sherlock-processor-final-20261111`; LW-018 as fixed by the spec audit), LW-020, LW-021 (typed MEM_BUS, ill-typed tapes; change `sherlock-completion-20261115`, this branch at 95643d85f), FLG-001 to FLG-009 (deferred field log), HDM-001 to HDM-010 (hash domains, use codes; HDM-001 and HDM-007 as fixed by the spec audit), PRD-004 (path C). The G3 transcript legs are SQG-014; the gate record is SSG-006.

Hard rules stated here: `L` and `L_Fr` are STARKs and no BLS12-381 circuit (LTR-001, LTR-002); one universal key set (LTR-007).

### sherlock-e2e-leaf-proving

Scope: the leaf stage. An R_SL run arrives from slice A as `RslStatement` and `ExecutionTrace`. This capability proves the app shards and the leaf layer. App and leaf-layer keys are per program under `key_root`; "universal leaf key" (LOOP T24, T25) means that every leaf-layer key gives the universal N1 pre-hash. The R_SL statement and the kits stay with slice A (ERS, EKT).

Citation keys: openvm `rekey-chips` dcf1b4c = openvm branch `rekey-chips` at dcf1b4c (`~/.cache/m7-wrap/src/openvm-rekey-chips`), paths under `crates/`. Moriarty 3dc9001a9 = this worktree at 3dc9001a9. LOOP Tn = row Tn of `~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md`. this branch at 95643d85f = this branch at 95643d85f (the spec-audit commits 0f859938e and 95643d85f are merged).

Reused requirements, not restated here: RSL-001 to RSL-008 (change `sherlock-completion-20261115`): universal leaf key for every R_SL app, the 6-or-9 AIR bucket, app proofs of the 142 suite calls, the keccak256, secp256k1, jubjub and SHA-256 kits, canonical input range checks. CPR-002 (change `sherlock-processor-final-20261111`): every proof is a fixed-size shard. SPL-001, SPL-006, SPL-008 (same change, as fixed by the spec audit): layer stack, fixed shapes, machine version.

Hard rules stated here: leaves are small-field STARKs (LFP-001); fixed-size shards (LFP-005); one universal key set (LFP-008).

### sherlock-e2e-node-aggregation

Scope: the node stage N1, N2, N3 and the MERGE-WORK unit: every AIR required, `prog_d` and `key_root` opened in N1, mixed chip sets, the 5,400 column bound, the T10q counterexample fixes, the merge of leaves from more than one host.

Reused requirements, not restated here: NODE-001 to NODE-015 and SHP-001 to SHP-009 (change `sherlock-modular-leaf-20261111`; SHP-008 as fixed by the spec audit), SQG-013 (the `merge_work` set, as fixed by the spec audit), LPC-002 (STACKS and VALUE DIGEST inside 5,400 columns), LPC-018 (T3 parent constraints fire), SC-013 and SC-014 of `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`).

Hard rules stated here: nodes are small-field STARKs (NAG-001); MERGE-WORK has at most 5,400 columns (NAG-005).

### sherlock-e2e-prover-daemon

Scope: the resident prover daemon and the warm path (rows T14, T14b): `wrapd serve`, the STARK worker, what stays loaded, the socket, the blinders of a served proof. Slice C uses the daemon through the SDK and CLI (SCL) and owns recovery (FRC).

Reused requirements, not restated here: PRD-001 to PRD-005 (change `sherlock-processor-final-20261111`), ZK-016 (blinding slots are one-use), LPC-005 (final timing on the new keys), GPW-007.

Code paths: Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/`; openvm `t14b` clone `~/.cache/m7-wrap/src/openvm-t14b`.

### sherlock-e2e-setup-mode

Scope: the setup mode that profiles a box and the scheduler that derives the per-run schedule from the profile `sherlock-profile/1`. Owner ruling 2026-11-14: F-3 is replaced by the setup mode ("We want to be completely elastic, Sherlock should have a setup mode where it profiles the machine and optimizes for it"; LOOP loop rules).

Source text: `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`) holds the EARS ST-001 to ST-008, PF-001 to PF-008 and SC-001 to SC-014 (no SC-009) in a research file. This spec is their OpenSpec home: each STM requirement names the ST, PF or SC ids it carries, with one obligation per requirement. SC-011 and SC-012 are in ELA; SC-013 and SC-014 are in NAG and UKS; ST-007 is UKS-008.

Reused requirements, not restated here: SPL-002 and SPL-011 (change `sherlock-processor-final-20261111`, as fixed by the spec audit).

Code paths: openvm `t16-elastic` 1baf605, crate `crates/sherlock-leaf/src/` (`profile.rs`, `sched.rs`) and `crates/sherlock-wrap-export/examples/common/t16.rs`.

Hard rule stated here: F-3 replaced by setup mode, no schedule menu in `key_root` (STM-013, STM-014).

### sherlock-e2e-stark-security

Scope: proven 100-bit security of the STARK layers, the D3 calculator on every key, the T12 gate (G1 to G4, FP-8) on the T25 keys, and the Quint lane and counterexample record (T13).

Reused requirements, not restated here: SQG-001 to SQG-013 (change `sherlock-processor-final-20261111`; SQG-013 as fixed by the spec audit), SQG-014 to SQG-016 and RKY-005 (change `sherlock-completion-20261115`), DG-001 to DG-011, FPR-001 to FPR-019, LPC-004, LPC-016.

Hard rules stated here: 100-bit security (SSG-001, SSG-006); leaves and nodes are small-field STARKs and the wrap is the only BLS12-381 circuit (SSG-002); never k = 22 (SSG-002).

### sherlock-e2e-universal-keys

Scope: the universal key set (internal-recursive, `L`, `L_Fr`, wrap), `key_root`, `keys_d`, and the single rekey of row T25. The wrap rows and the wrap VK bytes stay with slice C (WRP, RKY-003, RKY-004).

Reused requirements, not restated here: CPR-003 (one universal key set), SPL-004, SPL-005, SPL-009 (per-program and universal keys, `key_root` checks, what freezes), SQG-002, SQG-011 (key bounds, no freeze before the gate), all in change `sherlock-processor-final-20261111`; RKY-001, RKY-002, RKY-005 (change `sherlock-completion-20261115`); ST-007, PF-006, SC-011, SC-013 of `SETUP-SPEC.md` (Moriarty `t16-elastic` a7a7b8867, `research/sherlock-t16-elastic-20261114/`).

Hard rules stated here: one universal key set (UKS-001); design parameters and keys may change until the end-to-end prototype (UKS-007).

### sherlock-e2e-zero-knowledge

Scope: zero knowledge at the proving layer for case A. The wrap is the only hiding layer; every STARK layer stays on the owned boxes. This spec adds the end-to-end gaps that the T11 work and the ZK requirements leave open.

Reused requirements, not restated here: ZK-001 to ZK-011 and GP-ZK (change `sherlock-processor-final-20261111`; ZK-001, ZK-003, ZK-008, ZK-009, ZK-010 as fixed by the spec audit), ZK-012 to ZK-019 (change `sherlock-completion-20261115`), PRD-002, PRD-003. Decision T11-3 (no PCS masking in case A) is a decision record, not a requirement.

Evidence state of the reused requirements on 2026-11-15: `blind.rs` (openvm `t11-zk` cce8c1d `crates/sherlock-leaf/src/blind.rs`) and `zkguard.rs` (Moriarty `t11-zk` 71c4c001b `tools/sherlock-wrap/src/zkguard.rs`) are untracked; the tests `eq_zk`, `slot_reuse` and `seed_refused_without_feature` (Moriarty `t11-zk` 71c4c001b `tools/sherlock-wrap-gpu/tests/t11_zk.rs:36`, `:62`, `:92`, untracked) have no recorded run; ZK-L2 is committed (Moriarty `t11-zk` 71c4c001b `clean/fork/Clean/Sherlock/ZkPadding.lean`, theorems `pv2_inj` at `:183` and `pv2_count_det` at `:198`, 0 sorry).

Hard rules stated here: zero knowledge end to end (ZKE-003); seeded proofs test-only (ZKE-001, ZKE-002); no plain witness to helpers (ZKE-004).

## Capabilities of slice C (wrap, settlement, operations)

Per capability: scope, the reused IDs that the spec does not restate, the hard rules it states, and the code paths. Each requirement body is the EARS sentence and its Verification line; evidence (`branch@commit:path:line`) and status are in the scenario "Status of <ID>". The full adopted-ID table with check states is `bus/adopted-C.md`. 121 EARS: WRP 11 (WRP-005 withdrawn, number not reused), LVR 8, SLC 12, BAT 12, EGV 10, PVW 13, WAL 10, SCL 10, OBS 12, FRC 15, HLP 8. Status words on 2026-11-15: 24 BUILT, 50 PARTIAL, 47 TO BUILD (counted from the "Status of" scenarios).
Target state of the prototype settlement edge: the T20 batched SherlockLedger (4 entries) with the anchor sealed in slot 15, one ledger for each program, sealed at deploy with a readback gate, write entries on the one wrap key and read entries on native keys; settlement of a user's own run by `acceptRun`; the era check in the ledger if it enters the single rekey, else the consumer fallback on a recorded premise failure (D-078). The three contract forms on unmerged branches are named in SLC-001: T10 (`t17-preview`, with `openClaim`, the T17 (a) record), T20 (`t20-batched@5dc1be2ee`), T22 era ledger (sketch, `t22-governance@fe7207819`).

### sherlock-e2e-wrap (WRP)

Scope: the one BLS12-381 wrap at the settlement edge: the tool guards (k, rows, VK bytes), fresh blinders, the slot-15 pin in the decoder, key equality with the deployed key, the three output files, one key for all programs, GPU and CPU key equality, the final-key qualification gate. Input event `LfrProofReady` (slice B, LTR).
Reused, not restated: DG-001 to DG-003, DG-010, CPR-004, LW-001 to LW-019, LW-022, GPW-001 to GPW-008, DCD-001 to DCD-007, RKY-003, RKY-004, ANC-013, ANC-014, ANC-016.
Hard rules stated: never k = 22 (WRP-002); VK at most 50,000 bytes (WRP-004); one BLS12-381 wrap (WRP-001); one universal key set, wrap side (WRP-010); zero knowledge, wrap blinders (WRP-006).
Code: `tools/sherlock-wrap/` (`t10q-ntfix@95990a6d3`): `src/bin/wrap.rs`, `src/util.rs`, `src/decode.rs`.

### sherlock-e2e-ledger-verifier (LVR)

Scope: the stock ledger 9.1.0.1 verifier as an external system; the local check tool, its tamper controls, the native check before submit, chain identity in the run state.
Reused: MID-001 to MID-007, LW-017, STL-006.
Hard rule stated: ledger 9.1.0.1 verifier unchanged (LVR-001, LVR-002).
Code: `tools/sherlock-wrap/ledger-check/` (rev `a2b30128`), `validate_call` (`~/.cache/xc9`, no commit in Moriarty).

### sherlock-e2e-ledger-contract (SLC)

Scope: the SherlockLedger contract form, slot 15, entry keys, one ledger for each program, the argument builder from the private bundle to `StatementPublics` and the `acceptRun` words, Mycroft outcome and input forms, refused calls, instance reuse, the readiness gate before live settle.
Reused: LDG-001, LDG-002, STL-001 to STL-006, ANC-014, ANC-015, ZK-006, ZK-011, ZK-012; ERS-006 to ERS-008, ERS-010, ERS-012 (slice A).
Hard rules stated: none new; SLC-002 carries the slot-15 pin (D-021).
Code: `tools/sherlock-sdk/sherlock-ledger.compact` (`t20-batched@5dc1be2ee`; T10 form on `t17-preview@9e1e7f398`), `tools/t23-suite/accept_args.py` (staged on `t23-args`), `tools/t23-suite/parta.py` (`t23-suite@16836929b`).

### sherlock-e2e-batched-settlement (BAT)

Scope: epoch close, operator jobs through one collection, write counts in every report (per request, per epoch, per day), T23 suite counts (145 `acceptRun`, at most 106 deploys), `K_MAX` = 2 at every TTL, default TTL 3,600 s, membership data and the off-chain leaf list, the refusal of a collection submit while the `L` collection form has no key, era-aware close and admission, the T15 line for collection settlement.
Reused: LDG-003 to LDG-008, ANC-012, LW-009, LW-011, DCD-004, LPC-010; CLM and FREG stay in slice B for the in-circuit side.
Hard rules stated: Midnight writes batched, one root per checkpoint (BAT-001, BAT-002); transaction counts at scale (BAT-003, BAT-004); `K_MAX` = 2 (BAT-005).
Code: contract and tests on `t20-batched@5dc1be2ee` (`tools/sherlock-sdk/`, `models/sherlock/quint/t20-batched/`); no operator code.

### sherlock-e2e-governance (EGV)

Scope: sealing at deploy and the readback gate, unsealed ledgers as path evidence only, one era for each key set, the era check and the single rekey, the fallback, no key change on a deployed contract, open parameters at T15, the premise test, proof-version migration.
Reused: GOV-001 to GOV-011, ERT-001 to ERT-009, ELD-001 to ELD-007, CRR-001 to CRR-007, LPC-012, RKY-001, RKY-002.
Hard rules stated: one universal key set for each era (EGV-004); verifier unchanged, no upgradable contract (EGV-007).
Code: none deployed; compile-only sketches and `quint/eraTable.qnt` on `t22-governance@fe7207819`.

### sherlock-e2e-preview-deployment (PVW)

Scope: chain pin, no Mainnet, fees from chain parameters, own proof server, one command per step, the four tamper controls and their exclusive verdicts, final deploy after the gate with the gate record of the exact key, the final call on a GPU box of the user, the validium ledger refusals.
Reused: MID-004, MID-006, STL-003, LPC-008, LPC-009, RKY-003 to RKY-005, SQG-014 to SQG-016, ZK-012 to ZK-017.
Hard rules stated: Mainnet is owner-gated (PVW-002).
Code: `tools/sherlock-sdk/t10a-ledger-run.mjs`, `research/sherlock-t17-preview-20261114/t17.sh` (`t17-preview@9e1e7f398`).

### sherlock-e2e-wallet (WAL)

Scope: the phrase read in process, scrubbing, the pinned address, one sender at a time, public-only receipts and balances, one machine holds the wallet, no key material in the repository, no overwrite, permissions before the read. Written from code and the runbook only; no wallet file was opened.
Reused: ZK-012.
Hard rule stated: wallet seeds and keys are never logged, printed or copied (WAL-001, WAL-002, WAL-008).
Code: `t10a-ledger-run.mjs:114-116,487-545` (`t17-preview@9e1e7f398`), `preview-session.mjs` (`preview-deploy@78592bf59`).

### sherlock-e2e-sdk-cli (SCL)

Scope: the seven settlement commands, exit codes, dry run, state folder bound to one key, `E5JSON` records, build from committed source, TTL range, the suite driver, the SDK root, use outside the factory.
Reused: ANC-012 (`sherlock plan`), LPC-018, CF-002.
Hard rules stated: none.
Code: openvm `rekey-chips@dcf1b4c:crates/sherlock-lfr/examples/sherlock_plan.rs`; the tools named above; `tools/t23-suite/t23.py`.

### sherlock-e2e-observability (OBS)

Scope: receipts of included and refused calls (`SettlementReceipt`), privacy scan and its self-test, step log, input manifest, times per call and box, wallet failure causes, MEASURED / DERIVED / ESTIMATE labels, structural validation of the record, the wiki log, the prover GUI page.
Reused: GPW-007, LPC-005, LPC-011, LPC-016, LPC-017, ZK-002, ZK-007; PGUI IDs on branch `prover-gui`.
Hard rule stated: zero knowledge at the published view (OBS-003, OBS-004).
Code: `t10a-ledger-run.mjs:397-403`, `sherlock-ledger-scan.mjs`, `t17.sh:25,117-125`.

### sherlock-e2e-recovery (FRC)

Scope: step markers, one deploy, reuse of a proven call, bounded retries, a refused call proved again, the refusals step, durable units, reboot recovery, a gate that fails twice, durability in git, pushes, suite resume, unknown inclusion state, artifact checks, integration records.
Reused: RKY-002, LPC-010, LPC-014, LPC-015.
Hard rules stated: no push before T15 except `t10q` (FRC-011).
Code: `t17.sh`, `t10a-ledger-run.mjs:556-573,675-726,763-776`; loop rules in LOOP.md.

### sherlock-e2e-helper-machines (HLP)

Scope: the muninn check of every box wrap (`WrapVerdict`), dropped results, the bundle and private parts stay on the box, case A only, case B as its own change, no wallet on a proving box, a box result bound to its job.
Reused: ZK-001, ZK-009, ZK-018, PRD-002, PRD-003; BXF (slice B) for the box bus.
Hard rules stated: case A only, case B through Veritee later (HLP-005, HLP-006); no plain witness to helpers (HLP-004).
Code: none as a guard; loop rules (LOOP.md:96) and `check_prove_result` in `accept_args.py`.

## Risks and open items

The gap list of `architecture/COVERAGE.md` (section 5) is the full list. The items that can stop the prototype:

1. The open joint: `prog_d`, INIT and the statement binding for R_SL (ERS-006, ERS-007, ERS-012, ERS-015; LFP-010 to LFP-012; NAG-008; SLC-012). They change keyed constraints, so they land before the single rekey (T25): lane rsl-bind, openvm branch `rsl-bind` from `rekey-int` 971bc67 (lead ruling 2026-11-15).
1a. The era check (EGV-005, D-078) enters the single rekey, or a recorded failure of premise A2 or A3 selects the ELD-007 fallback. T22 tasks 2 and 3 are on the critical path before T25 (lead ruling 2026-11-15).
2. The interpreted family has 7 built rules of 120 (EUI-010). GP-1 at the proving layer is shown on a subset only.
3. T8c has no LOOP row: the MERGE-WORK width with STACKS and VALUE DIGEST (4,988 columns MEASURED without them, 5,121 DERIVED with them, bound 5,400; not MEASURED), the X8 in-circuit verifier, `L` over SET (LPC-002, LPC-003, GPH-008, BAT-009). The `L` collection form is a recorded second rekey after T25 and before T15 (D-049).
4. A program with SHA-256 and keccak256 has no R_SL proof (EKT-004, D-024). The owner has not seen this limit.
4a. OD-13 one-use slots: the owner ruled on 2026-11-15 that the per-proof draw of the daemon satisfies them; no slot pool is built (D-040).
5. The zero mismatch of the T23 dry run is vacuous (ECS-012); completion needs the evidence set of every call (ECS-014).
6. Much of the evidence is uncommitted: the spec-audit change, the Mycroft factory work, T11, FP-3, slot 15, MAP.md. A status in this change that cites a worktree says so.
7. The 21 former LEAD PICK for T15 records are OWNER since 2026-11-15; D-022 (run form) sets what "works on Preview" means.

## Migration and archive

This change adds requirements only. When the cited changes are archived into `openspec/specs/`, the citations of this change resolve there and D-026 is reviewed. The decision records and DDD.md stay with the change and are referenced from the archive, so the reasoning stays discoverable.
