# Domain model of the Sherlock end-to-end prototype (DDD)

Sherlock is a universal, elastic proving co-processor for Midnight. This file gives the bounded contexts, the context map, the aggregates with their invariants, the domain events and the ubiquitous language of the prototype. It follows the software-factory profile `skills/specs/spec-architect/references/architecture-decisions.md`: proportionate modelling, and no obligation in this file. An invariant binds only through the EARS IDs named beside it. Terms come from `research/sherlock-final-design-20261111/TERMINOLOGY.md`; a term that file does not hold is marked (new) or explained in section 6.

## 0. Scope and proportion

- Business focus. Core: Compact Compilation, Sherlock Execution, Leaf Proving, Aggregation, Bridge, Key Management, Scheduling, Wrap, Settlement. Those hold the claims that make Sherlock worth using (any program, one key set, zero knowledge, the unchanged ledger). Supporting: Conformance, Call Flow, Box Fleet, Assurance, Governance, Network Operations, Placement. Generic or external: Native Compact (`compactc`), the Midnight ledger 9.1.0.1 verifier, the STARK engine (OpenVM stark-backend), Fenrir.
- A context is where one model and one language apply. It is not a slice, a repository, a crate or a deployable. The slices A, B and C are authoring lanes of this change. Section 7 maps contexts to capability specs. The mapping is deliberate where it is one to one and is explained there.
- A domain event below is a named fact that crosses a context boundary. It selects no event-driven implementation: today each event is a file, a runner state field or a bus line.
- The product domain and the factory that builds it are apart. DoctorWatson, the LOOP, lanes, herdr, the box bus as a lead tool and the wiki are factory terms. They appear only where the prototype still runs through them (Box Fleet, Network Operations), and EUF-011 states that the product flow must not need them.

## 1. Bounded contexts

### Slice A: language, execution, integration

| Context | Purpose | Kind | Capability specs | Code today |
|---|---|---|---|---|
| Compact Compilation | Turn the stock `compactc` output of a contract into Sherlock program images, or refuse with a named cause | core | `sherlock-e2e-compact-compiler` (ECC), reuses MYC (frontend, values, lowering, witness-tape, ledger-ops, builtins, refusals), CF-001 to CF-014 | `mycroft/compiler/` (worktree `mycroft-factory`, d1e8eed8d plus uncommitted files), `mycroft/lean/` |
| Sherlock Execution | Give one call of one program its meaning: the rule families, the kits, the call record and the statement that proving and settlement consume | core | `sherlock-e2e-universal-interpreter` (EUI), `sherlock-e2e-rsl-kits` (EKT), `sherlock-e2e-rsl-statement` (ERS), reuses RSL, GP-001, CORE, RFB | openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/src/rsl/`, `src/dec.rs`; `mycroft/compiler/src/sem.rs` |
| Conformance | Show, call by call, that Sherlock gives what native Compact gives on a pinned corpus | supporting | `sherlock-e2e-compact-suite` (ECS), reuses STL-005, STL-006, MYC-ACC, MYC-LOW-018 | `mycroft/tests/compact-suite/`, `tools/t23-suite/t23.py` (`t23-suite` 16836929b) |
| Call Flow | The ordered path of one call across every other context; it owns no state of its own | supporting (a process, section 5) | `sherlock-e2e-user-flow` (EUF) | runbooks and runners: `t17.sh`, `t23.py`, `t7_chain` |

Compact Compilation and Sherlock Execution are two contexts in two repositories (Moriarty and the openvm tree). The word "program" differs: in Compact Compilation a program is one compiled circuit of a contract; in Sherlock Execution a program is any image with a `prog_d`, of either rule family.

### Slice B: proving

| Context | Purpose | Kind | Capability specs | Code today |
|---|---|---|---|---|
| Leaf Proving | Prove the shards of one run: app proof under the per-program chip set, leaf proof under the universal leaf key; fixed shard heights; `prog_d` and `input_d` in the leaf statement | core | `sherlock-e2e-leaf-proving` (LFP), reuses RSL-001 to RSL-008, CPR-002, SPL-006 | openvm `rekey-chips` dcf1b4c `crates/sherlock-leaf/` (`leaf.rs`, `core.rs`, `rsl/`) |
| Aggregation | Merge leaf proofs into one node root: N1, N2, N3, MERGE-WORK units, the GP-2, GP-3 and GP-4 hooks, continuation | core | `sherlock-e2e-node-aggregation` (NAG), `sherlock-e2e-gp-hooks` (GPH), reuses NODE, SHP, CLM, CONT, FREG, FVR, GP-001 to GP-004 | openvm `t10mm` 5b7ada8 `crates/sherlock-lfr/src/stmt.rs`, `gp.rs`; `crates/sherlock-wrap-export/examples/t8_gp.rs` |
| Bridge | `L` and `L_Fr`: verify the node root, expose the statement, the call vector and the field log; bind a call to the ledger (FP-3, anchor slot, lock, chunk count); hand one `L_Fr` proof to the wrap | core | `sherlock-e2e-l-transcript` (LTR), `sherlock-e2e-call-binding` (CLB), reuses LW, FLG, HDM, ANC | openvm `crates/sherlock-lfr/src/` (`machine.rs`, `tape.rs`, `iface.rs`, `lverify.rs`, `transcript.rs`; `fp3.rs` untracked on `fp3` d2abc38) |
| Key Management | The universal key set, `key_root`, `keys_d`, the single rekey, the key manifest | core | `sherlock-e2e-universal-keys` (UKS), reuses RKY-001, RKY-002, RKY-005, CPR-003, SPL-004, SPL-005 | `crates/sherlock-wrap-export/examples/t7_chain.rs` (`keys()`), `crates/sherlock-lfr/src/frozen.rs`; `~/.cache/rekey-t11/LIST.md` |
| Scheduling | Setup mode, the box profile `sherlock-profile/1`, the per-run schedule; the elastic property | core | `sherlock-e2e-setup-mode` (STM), `sherlock-e2e-elastic-proving` (ELA), reuses SPL-011, LPC-006, LPC-007 | openvm `t16-elastic` 1baf605 `crates/sherlock-leaf/src/profile.rs`, `sched.rs`; `examples/common/t16.rs` |
| Box Fleet | The owned boxes muninn, huginn, odin: admission (heavy-slot), GPU lock, box bus, file moves, durable runs; the GPU paths; the resident daemon | supporting | `sherlock-e2e-box-fleet` (BXF), `sherlock-e2e-gpu-proving` (GPV), `sherlock-e2e-prover-daemon` (PDM), reuses PRD, GPW-007, LPC-005 | `~/.local/bin/heavy-slot`, `box-relay` (no repository); Moriarty `t14b` 5e54263da `tools/sherlock-wrap-gpu/` |
| Assurance | Zero knowledge at the proving layer; 100-bit security, D3, the T12 gate; Quint lanes and the counterexample record | supporting | `sherlock-e2e-zero-knowledge` (ZKE), `sherlock-e2e-stark-security` (SSG), reuses ZK, SQG, DG, FPR, LPC-004, LPC-016 | openvm `t11-zk` cce8c1d (`blind.rs` untracked); `examples/fp8_d3.rs`; `tests/quint-regressions/COUNTEREXAMPLES.md` |

One context is not one crate. Leaf Proving, Aggregation and Bridge share the crates `sherlock-leaf` and `sherlock-lfr`; the chain driver `t7_chain` is the only orchestrator today (decision D-045).

### Slice C: wrap, settlement, operations

| Context | Purpose | Kind | Capability specs | Code today |
|---|---|---|---|---|
| Wrap | Turn one `L_Fr` proof into the one BLS12-381 proof that the ledger reads; hold the wrap key and its limits (k <= 21, VK <= 50,000 bytes, 1,026 public inputs) | core | `sherlock-e2e-wrap` (WRP), reuses LW, GPW, RKY-003, RKY-004 | `tools/sherlock-wrap/` (`t10q-ntfix@95990a6d3`) |
| Ledger Verification | The stock ledger 9.1.0.1 verifier as an external system, and the local check that speaks its language | generic (external), with an anti-corruption layer | `sherlock-e2e-ledger-verifier` (LVR), reuses MID | `tools/sherlock-wrap/ledger-check/`, `validate_call` |
| Settlement | SherlockLedger contracts: run claims, collection roots, read entries; the argument map from a statement to `acceptRun` | core | `sherlock-e2e-ledger-contract` (SLC), `sherlock-e2e-batched-settlement` (BAT), reuses LDG, STL, ANC-012 to ANC-014 | `tools/sherlock-sdk/sherlock-ledger.compact` (`t20-batched@5dc1be2ee`), `tools/t23-suite/accept_args.py` (staged on `t23-args`) |
| Governance | Eras, the era table, the Fenrir operator roster and its verbs; sealing | supporting | `sherlock-e2e-governance` (EGV), reuses GOV, ERT, ELD, CRR | sketches only (`t22-governance@fe7207819`) |
| Network Operations | Preview deployment, the wallet, fees, the proof server, receipts, recovery | supporting | `sherlock-e2e-preview-deployment` (PVW), `sherlock-e2e-wallet` (WAL), `sherlock-e2e-sdk-cli` (SCL), `sherlock-e2e-observability` (OBS), `sherlock-e2e-recovery` (FRC) | `tools/sherlock-sdk/t10a-ledger-run.mjs`, `t17.sh`, `tools/t23-suite/t23.py` |
| Placement (settlement edge) | Which machine may hold what: case A only, the muninn check of box wraps, the wallet on muninn | supporting | `sherlock-e2e-helper-machines` (HLP), reuses ZK-001, ZK-009, ZK-018, PRD-002, PRD-003 | loop rules, BOXES.md, no code guard |

One context is not one deployable. Network Operations is four tools and one runbook today (decision D-085). The names muninn and OP-PV-WALLET belong to the factory rehearsal; the product rule is "the one machine that holds the wallet" and "the wallet pinned in the run configuration" (SCL-010).

## 2. Context map

Relations use the DDD names: customer-supplier, conformist, anti-corruption layer, shared kernel, published language, open host service, partnership. Upstream and downstream state influence; request direction is stated where it differs.

### Slice A: language, execution, integration

| Upstream | Downstream | Relation | Exchanged meaning | Translation owner |
|---|---|---|---|---|
| Native Compact (`compactc` 0.36.0, ZKIR 3.1, external) | Compact Compilation | anti-corruption layer: `zkir.rs` reads ZKIR into GIR and refuses what it has no rule for | circuits, types, the native transcript as the reference | Compact Compilation |
| Compact Compilation | Sherlock Execution | customer-supplier; published language `rsl-call/1` (program image, ABI fields, tape) | one call ready to execute | Compact Compilation writes, Sherlock Execution validates (ERS-013, ERS-014) |
| Compact Compilation | Sherlock Execution | shared kernel, kept small: the kit interfaces (`SECP-INTERFACE.md`; the Jubjub interface of EKT-010), the 13 instruction shapes and the builtin ids | which chip pairs exist and their word layouts | both; a change is a new machine version |
| Sherlock Execution | Leaf Proving (slice B) | customer-supplier; published language `RslStatement`, `ExecutionTrace` | what a run did, as data to prove | Leaf Proving (leaf input, FP-1 words) |
| Key Management, Leaf Proving (slice B) | Sherlock Execution | published language `LeafKeyRef`, `ProgramAdmission` | `key_root`, chip set id, AIR bucket, authenticated `prog_d` | Key Management |
| Sherlock Execution | Settlement (slice C) | published language: the private record `RslStatement` goes to the argument builder on the proving box; only its projection `StatementPublics` leaves the box | the four public words of one run | Settlement |
| Settlement, Network Operations (slice C) | Conformance | published language `SettlementReceipt`, `NativeCallReference` | what settled; what native did on Preview | Conformance |
| Sherlock Execution, Leaf Proving, Aggregation, Bridge, Wrap, Settlement | Conformance | conformist: Conformance reads results and has no power to change a protocol | per-call evidence | Conformance |

Upstream and downstream describe influence. Request direction differs for Conformance: its runner calls every other context, and it is downstream of all of them.

### Slice B: proving

| Upstream | Downstream | Relation | Exchanged meaning | Translation owner |
|---|---|---|---|---|
| Language (slice A, ERS) | Leaf Proving | customer-supplier; published language `RslStatement` and `ExecutionTrace` (call record `rsl-call/1`) | one R_SL run to prove: program, tape, W, `comm` | Leaf Proving (`rsl/compile.rs`) |
| Language (slice A, EKT) | Leaf Proving | shared kernel, kept small: the R_SL AIR set (WORD, ALU, PUBLIC, RANGE, SHA_IN and the SHA or Keccak AIRs) and the 6-or-9 AIR bucket | kit AIRs and their buses 200 to 206 | both; a change is a row of the rekey list |
| Key Management | Leaf Proving, Aggregation, Bridge, Scheduling, Box Fleet | published language `LeafKeyRef {key_root, chip_set_id, air_count}` (new) and the key manifest (new) | which keys a proof must verify under | Key Management |
| Leaf Proving | Aggregation | customer-supplier; `LeafProofsReady` (new) | leaf proofs, their public values, sha256 | Aggregation |
| Aggregation | Bridge | customer-supplier; node root in `ProofBundle` (new) | one node root and its statement | Bridge |
| Bridge | Wrap (slice C) | customer-supplier; published language `LfrProofReady {l_fr_proof, l_fr_vk_prehash, public words, claim_pub}` | one `L_Fr` proof that is ready to wrap | Wrap (`wrapin.rs`) |
| Bridge | Wrap and Settlement (slice C) | shared kernel, kept small: the interface layout (12,166 public words, 5 lane pairs), the anchor slot constant 15, the lock form `x_j` | the call binding | both; a change is a rekey and a redeploy (CLB-008) |
| Scheduling | Leaf Proving, Aggregation | customer-supplier: the schedule (shape, device, concurrency, cut); it enters no proof | how to run, never what is proved | Scheduling |
| Box Fleet | every proving context | open host service: slots, GPU lock, bus, file moves | `BoxResult {path, sha256, box}` (new) | Box Fleet |
| Placement (slice C, HLP) | Box Fleet | partnership: C issues `WrapVerdict` on muninn; B owns boxes, daemon and STARK-side zero knowledge | keep or drop a box wrap | both |
| Assurance | Key Management | customer-supplier: the gate record moves a key set from `prototype` to `qualified` | security status of a key set | Assurance |
| stark-backend and OpenVM (external) | Leaf Proving, Aggregation, Bridge | conformist: stock verifier, stock transcript order; local patches are exported series (LOOP T0) | proof and key formats | each proving context |

Influence and request direction differ in one place: Scheduling calls the provers, but Key Management is upstream of Scheduling, because a profile is valid only for one key set (STM-008).

### Slice C: wrap, settlement, operations

| Upstream | Downstream | Relation | Exchanged meaning | Translation owner |
|---|---|---|---|---|
| Proving (slice B, LTR) | Wrap | customer-supplier; published language `LfrProofReady {l_fr_proof, l_fr_vk_prehash, public words, claim_pub}` | one `L_Fr` proof that is ready to wrap | Wrap (`wrapin.rs`) |
| Language (slice A, ERS) | Settlement | published language `StatementPublics {c_p, input_pub, outcome_pub, claim_pub}`; it is the result of the argument builder, not its input | the public face of one run statement | Settlement (the argument builder, SLC-006 to SLC-009). The builder input is the private `sherlock-accept-bundle/1` (preimages of program, input and outcome); it stays on the proving box (HLP-003, HLP-004). `acceptRun` takes 3 of the 4 words; the runner file adds `anchor` |
| Wrap | Ledger Verification | conformist: the wrap conforms to `MidnightVK::read` and `VerifierKey::verify`; nothing in the ledger changes | `WrapOutput` (3 files) | Wrap |
| Ledger Verification (Midnight ledger 9.1.0.1, external) | Settlement, Network Operations | anti-corruption layer: `sherlock-wrap-ledger-check`, `validate_call`, node error 1010/115 mapped to `refused-by-node` | accept or refuse | Network Operations |
| Wrap | Settlement | shared kernel, kept small: the call vector layout (1,026 lanes; lane 10 anchor, lanes 20 and 34 `claim_pub`, lane 39 `outcome_pub`; anchor slot 15) | lane map and anchor slot | both; a change is a rekey and a redeploy |
| Governance | Settlement and consumer contracts | customer-supplier, Governance upstream: the era table says which era accepts. Primary path: each wrap-keyed ledger entry calls `active(era_id)` at inclusion (ELD-003), once the wrap binds the era words (EGV-005). Fallback, only on a recorded failure of premise A2 or A3: the consumer checks at first use (EGV-006) | `era_id`, `active(era_id)`, `end_eff` (BAT-010, BAT-011) | Governance |
| Settlement | Consumer contracts (external) | open host service with published language: read entries `rootAccepted`, `runAccepted`; claims tree depth 20 | membership of a claim | Settlement |
| Settlement | Language (slice A, EUF) and Proving (slice B) | published language `SettlementReceipt`, `WrapVerdict` | what settled, and whether a box wrap is kept | Network Operations |
| Placement | Proving (slice B: BXF, PDM, ZKE) | partnership: B owns boxes, daemon and STARK-side zero knowledge; C owns the settlement edge (HLP) | `BoxResult {path, sha256, box}` in, `WrapVerdict` out | both |
| Fenrir (external) | Governance | conformist for the roster (rotated Fenrir operator keys); no Fenrir contract is called | operator keys | Governance |

Request direction and influence differ in one place: the ledger entry calls the era table (request from Settlement to Governance), and Governance is upstream because it decides which era accepts.

### Whole-system view

```text
 Native Compact (external) --ACL--> Compact Compilation --rsl-call/1--> Sherlock Execution
                                                                            | RslStatement, ExecutionTrace
                                                                            v
 Scheduling, Key Management, Box Fleet, Assurance  <-->  Leaf Proving -> Aggregation -> Bridge (L, L_Fr)
                                                                            | LfrProofReady
                                                                            v
 Midnight ledger 9.1.0.1 (external) <--conformist-- Wrap --WrapOutput--> Ledger Verification (ACL)
                                                                            | WrapVerdict
                                                                            v
 Governance --era--> Settlement (SherlockLedger) <--StatementPublics-- Sherlock Execution
                          | SettlementReceipt
                          v
                     Conformance (reads every context; changes none)
```

## 3. Aggregates and invariants

### Slice A: language, execution, integration

| Aggregate (root) | Context | Invariants (each bound by EARS) | Atomic scope |
|---|---|---|---|
| CompiledEntry (root: contract, circuit, selector) | Compact Compilation | one image and one ABI per accepted circuit (ECC-003); image and ABI bytes do not depend on a witness (ECC-004); only the 13 instruction shapes (ECC-005); kits inside the machine version (MYC-BLT-008); anchor slot 15 in a deployable profile (ECC-009); equal to the Lean oracle on the golden vectors, under the open premises of LPC-019 (ECC-011) | one compile |
| MachineVersion (root: version name, `rules_d`) | Sherlock Execution | one rule family per program (EUI-001); the R_SL table has exactly 16 rules (EKT-001); every app has 6 or 9 AIRs (EKT-002, RSL-002); a kit adds no universal key (EUI-003, RSL-001) | one rekey |
| CallRecord (root: program, circuit, label, index) | Sherlock Execution | schema `rsl-call/1` (ERS-001, ERS-013); a malformed record is a named refusal (ERS-014); stays on boxes the user owns (ERS-002); a bad tape starts no proof (EUI-007); static step count (EUI-008) | one call |
| RslStatement (root: `prog_d`, `input_d`) | Sherlock Execution | `comm` and W equal native (ERS-005; MYC-LOW-009); the outcome kind is Halted with payload `vd([comm] ++ W)` (ERS-008); no statement for a natively refused Compact call (ERS-009); exactly five fields, private to boxes the user owns (ERS-010, ERS-002); `comm` and W equal on every box (ERS-011); bound into the recursion statement (ERS-012) | one execution |
| SuiteRun (root: suite manifest hash, universal key-set identity) | Conformance | one identity per call (ECS-013); one result class per call (ECS-004); a blocked call names its reason (ECS-005); the mismatch count states its denominator (ECS-012); completion needs the whole evidence of each call (ECS-014); no live action without its switch (ECS-007, ECS-008) | one suite run; each call resumable |

Open consistency questions: `input_d` commits to the whole tape (D-071 pick 2), and the tape salt and the `input_d` salt are distinct fields whose randomness policy slice B owns (ZK-013). The R_SL `prog_d` does not exist yet (ERS-006), so RslStatement has no root in code today.

### Slice B: proving

| Aggregate (root) | Context | Invariants (each bound by EARS) | Atomic scope |
|---|---|---|---|
| UniversalKeySet (key manifest) | Key Management | one set for every program (UKS-001, CPR-003); made once from one list (RKY-001), from a tree that holds every lane (UKS-009), with a universal key manifest that holds no per-program key (UKS-004); `keys_d` has 7 node rows (UKS-002); R_SL key leaves do not collide with the DAG leaf (UKS-003); every gate reruns on it (UKS-006); `prototype` until G1 to G5 and FP-8 pass (UKS-007, SSG-006); 100 bits on every key (RKY-005, SSG-001) | one keygen |
| ProgramKeys (`key_root` of one program) | Key Management | per-program app, leaf-layer and internal-for-leaf keys under `key_root` (SPL-004); 6 or 9 AIRs with FP-1 (RSL-002, LFP-003); at most 30,000 app columns (LFP-004); no schedule menu (STM-014) | one `sherlock plan` |
| Run (the `ProofBundle` of one program and input) | Leaf Proving, Aggregation, Bridge | every shard has the ordered AIR height vector of its shape (LFP-005); each leaf carries the run's `prog_d` (LFP-006, LFP-007, LFP-011) and `input_d` (LFP-012); the app statement is bound into FP-1 (LFP-010); children link by step interval and state digest (NAG-009); every required AIR is present (NAG-002); the outcome is bound in `L` (NAG-008); one `claim_pub` for every schedule of one prepared run (ELA-001) | one run; shards are proved independently and joined by the node constraints |
| WorkUnit (MERGE-WORK) | Aggregation | at most 5,400 columns (NAG-005); the T10q forgeries are refused (NAG-006, NAG-007) and the honest T10q inputs are accepted (NAG-011); any tree shape gives one root (GPH-005) | one unit proof |
| CallBinding (anchor statement of one call) | Bridge | effect lanes equal guest effects (CLB-002, ANC-018); `k` is the slice count (CLB-003, ANC-019); anchor slot 15, read by every call and written by chunk calls only (CLB-004, CLB-006, CLB-008); lock operand `x_j` (CLB-005, ANC-017); at most 2 chunks (CLB-007); 12,166 public words (CLB-001) | one `L` proof |
| LfrProof | Bridge | Fr base field, no extension (LTR-002); the tape replays the `L` transcript (LTR-005); typed MEM_BUS (LW-020, LW-021); stock-verified before hand-off (LTR-008) | one prove |
| BoxProfile (`sherlock-profile/1`) | Scheduling | written by setup only (STM-006); refused when malformed (STM-007); valid for one key set (STM-008); changes no key (UKS-008) | one setup run on one box |
| Schedule | Scheduling | derived from the profile, the run length and the limits (STM-009); limits only lower (STM-010); passes `check_free` (STM-013); least estimated time with a fixed tie order (STM-017); never faster at a lower budget (STM-018); stays on the user's boxes (SPL-011) | one run |
| Box (a machine in the owned-box roster of the user; the factory fixtures are muninn, huginn, odin) | Box Fleet | in the roster, which the user writes (BXF-001); holds the same key manifest (UKS-005); one GPU job at a time under `gpu.lock` (BXF-004); files leave with a sha256 (BXF-007) and, in the factory fleet, through muninn (BXF-014); secrets, witnesses and STARK proofs move only to boxes of the roster (ZKE-004) | one box |
| DeploymentSecret (`deploy.secret`, `run.secret`) | Assurance | `r_P` once per deployment (ZK-012); fresh salt per run (ZK-013); no constant `r_P` outside a test build (ZKE-001); a missing secret stops the call (ZKE-005); never printed (ZKE-006) | one deployment, one run |
| GateRecord (T12) | Assurance | G1 to G5 and FP-8 on the T25 keys (SQG-014, SQG-015, SSG-006); no UNVERIFIED step (SQG-016); no CONFIRMED defect without a fix (SSG-003); names the key manifest (SSG-006) | one gate run |

Open consistency question: the Run aggregate has no single owner in code. The run record of `t7_chain`, the T16 records and the T23 runner each write their own JSON. `ProofBundle` is the proposed root (decision D-045).

### Slice C: wrap, settlement, operations

| Aggregate (root) | Context | Invariants (each bound by EARS) | Atomic scope |
|---|---|---|---|
| WrapKey (the one wrap VK of an era) | Wrap | k <= 21 (WRP-002); rows <= 1,782,573 (WRP-003, RKY-003); VK <= 50,000 bytes (WRP-004, RKY-004); 1,026 public inputs (LW-003); the LW checks pass on the final keys (WRP-012); anchor slot 15 is a key constant for the read key and the chunk write key (WRP-007, ANC-013, ANC-016); one key for every program and mode (WRP-010); GPU key = CPU key (WRP-011) | one keygen |
| WrapProof | Wrap | one proof for each ledger call (WRP-001); fresh blinders (WRP-006); verified by the stock verifier with tamper controls before use (LVR-001, LVR-004, HLP-001) | one prove |
| ProgramLedger (one SherlockLedger for each program; root = contract address) | Settlement | anchor = `c_p` of its program, sealed in slot 15 (SLC-002, SLC-005); a `claim_pub` settles at most once (LDG-002, STL-004); no live settle of an unbound statement (SLC-012); write entries hold the wrap key, read entries native keys (SLC-003, SLC-004); 4 entries only (SLC-001); sealed for life, with authority and entry keys read back (EGV-001, EGV-002) | one ledger transaction |
| ClaimEpoch (one collection of an operator; root = `claims_root`) | Settlement | at most one ledger write for each epoch (LDG-006, BAT-001, BAT-002); a claim is used by membership with no ledger write (LDG-004, BAT-007); the leaf list is available off chain (BAT-008); refused while the `L` collection form has no key (BAT-009); closed before an announced era end (BAT-010) | one epoch close |
| ChunkPlan | Settlement | `K_MAX` = 2 at every TTL (BAT-005); TTL 3,600 s (BAT-006) | plan time |
| EraTable (root = table address) | Governance | append-only; one era for each key set; one era accepts at a time; an end is set once (ERT-003 to ERT-007, EGV-004); verbs need the quorum of the roster in force (GOV-001); a key change is a new era (EGV-007) | one governance transaction |
| SettlementRun (one call from proof to receipt; root = `claim_pub` + state folder) | Network Operations | pinned chain and wallet (PVW-001, PVW-002, WAL-003); one sender at a time (WAL-004); a deploy is sent once (FRC-002); an unknown inclusion state is resolved before a new submission (FRC-013); saved artifacts are checked before reuse (FRC-014); a refusal needs a node error (PVW-007); secrets never in a message or receipt (WAL-002, WAL-005) | one runner step (each step resumable, FRC-001) |
| BoxResult acceptance | Placement | a box result is bound to its job by sha256 and call vector (HLP-008); a box wrap is kept only after the muninn check (HLP-001, HLP-002); private parts stay on the box (HLP-003, HLP-004); no host outside the user's roster (HLP-005, ZK-018) | one result |

Open consistency questions (carried as gaps): how ERT-003 "one ledger for each era" reads under one ledger for each program (T22 O8); whether a retry of one logical run keeps its salt, which CRR-005 needs across eras.

## 4. Domain events

### Slice A: language, execution, integration

| Event | Producer | Consumers | Carries |
|---|---|---|---|
| `EntryCompiled` | Compact Compilation | Sherlock Execution, Conformance | image, ABI, kits, compiler and ZKIR version |
| `EntryRefused` | Compact Compilation | Conformance, the developer | refusal code (`MYC-REF-*`), circuit, cause (ECC-013) |
| `CallTaped` | Compact Compilation (host) | Sherlock Execution | `rsl-call/1` record; private |
| `CallExecuted` | Sherlock Execution | Leaf Proving (B), Conformance | `RslStatement`, micro-op counts, kits used |
| `CallRejected` | Sherlock Execution | Conformance, Call Flow | reject code; for a Compact call no statement (ERS-009, EUF-006); a general run keeps its provable Rejected outcome (EUI-005) |
| `CallCompared` | Conformance | the lead, Observability (C) | call identity, class, evidence kind (MEASURED or DERIVED) |
| `SuiteReported` | Conformance | the owner at T15 | the suite table of ECS-010 |

### Slice B: proving

| Event | Producer | Consumers | Carries |
|---|---|---|---|
| `KeySetGenerated` (new) | Key Management (T25 runner) | every box, Wrap (C), Scheduling | the universal key manifest: sha256 of each universal key file, `keys_d`, list sha256, tree commits, status `prototype` (UKS-004, UKS-007) |
| `BoxProfiled` (new) | Scheduling (setup mode) | Scheduling | `sherlock-profile/1` (STM-006) |
| `RunScheduled` (new) | Scheduling | Leaf Proving, Aggregation | shape, device, concurrency, cut; never leaves the box (STM-009) |
| `LeafProofsReady` (new) | Leaf Proving | Aggregation | leaf proof paths, sha256, public values, `LeafKeyRef` (LFP-009) |
| `NodeRootProved` (new) | Aggregation | Bridge, slice A (EUF) | node proofs added to `ProofBundle` (NAG-010) |
| `LfrProofReady` | Bridge | Wrap (C) | `L_Fr` proof path and sha256, `L_Fr` key pre-hash, public words, `claim_pub` (LTR-006); not emitted when the stock verifier refuses (LTR-008) |
| `BoxResultPosted` (new) | Box Fleet | muninn (lead), Placement (C) | `BoxResult {path, sha256, box}` on the box bus (BXF-007, BXF-008) |
| `ProvingRefused` (new) | any proving context | Recovery (C, FRC), slice A (EUF) | the reason: bucket (LFP-003), bounds (LFP-004, LTR-003, NAG-005), key manifest (UKS-005, PDM-006), schedule (STM-012, STM-013), roster (BXF-001), seed guard (ZKE-001), device fault (GPV-006) |
| `GatePassed` (new) | Assurance | Key Management, Preview deployment (C, PVW) | the gate record; the manifest status becomes `qualified` (SSG-006) |
| `WrapVerdictIssued` | Ledger Verification on muninn (C) | Box Fleet | `WrapVerdict`; B consumes it to keep or drop a box result |

### Slice C: wrap, settlement, operations

| Event | Producer | Consumers | Carries |
|---|---|---|---|
| `LfrProofReady` | Proving (B) | Wrap | `L_Fr` proof, key pre-hash, public words, `claim_pub` |
| `WrapProved` | Wrap | Ledger Verification, Placement | `WrapOutput` = `wrap.vk`, `wrap.proof`, `wrap.public.json` (WRP-009) |
| `WrapVerdictIssued` | Ledger Verification on muninn | Proving (B), Settlement | `WrapVerdict {ok, key bytes, proof bytes, controls refused}` (LVR-004, HLP-001) |
| `AcceptArgsBuilt` | Settlement (builder, on the box) | Network Operations | the 4 public words (HLP-003) |
| `LedgerDeployed` | Network Operations | Settlement, Governance | address, block, installed keys, maintenance authority readback (SLC-003, EGV-002) |
| `RunSettled` | ProgramLedger | Language (A), Observability | `SettlementReceipt {contract, tx hash, block, status, bytes, fee, runs[claim_pub] = outcome_pub}` (OBS-001) |
| `CallRefused` | node, through the runner | Observability, Recovery | tx sha256, node JSON-RPC error (OBS-002, PVW-007) |
| `EpochClosed` | epoch operator | Settlement, submitters | `claims_root`, slot and path for each job, leaf list location (BAT-001, BAT-007, BAT-008) |
| `CollectionAccepted` | ProgramLedger | consumers | `claims_root` in `claim_roots` (LDG-003) |
| `EraAnnounced`, `EraActivated`, `SoundnessEnded`, `RosterRotated` | EraTable | Settlement, consumers, provers | era record (GOV-002, GOV-003, GOV-005, GOV-006) |
| `RunResumed`, `IncidentLogged` | Recovery | lead loop, wiki | step, state folder, cause (FRC-001, FRC-008, OBS-011) |

An event here is a named fact for the context map. It does not select an event-driven implementation: today each is a file or a runner state field.

## 5. Cross-boundary process: one call

The Call Flow context owns one process and no aggregate. It is specified by `sherlock-e2e-user-flow` (EUF).

| Concern | Record |
|---|---|
| Trigger | A user calls one entry of a deployed program with a preimage |
| Required outcome | `runs[claim_pub] = outcome_pub` on Preview, with W equal to the native transcript words (EUF-010); or a named stop before submit with no transaction; or a node refusal of a submitted call, recorded with the node error |
| Participants, in order | Compact Compilation (compile), Scheduling (setup), Key Management and Leaf Proving (plan), Network Operations (deploy), Compact Compilation host (tape), Sherlock Execution (execute), Leaf Proving, Aggregation and Bridge (prove), Wrap (wrap), Ledger Verification (check), Network Operations (submit), Settlement (accept), Conformance (read-back and compare) (EUF-001) |
| Ordering | Stage order of EUF-001; each stage reads the inputs that the stage dependency table of `design.md` names: per program (compile, plan, deploy), per box (setup), per call (tape to read-back) (EUF-002) |
| Duplicates | A `claim_pub` settles at most once (LDG-002); an included call is never resubmitted (FRC-006); a deploy is sent once (FRC-002) |
| Partial failure | A failed stage before submit sends no transaction (EUF-005); a natively refused Compact call is not proved or settled (EUF-006); a box wrap that fails the muninn check is not kept (HLP-001); after submit the call is reconciled by transaction hash and never sent again (EUF-014, FRC-006) |
| Recovery | Each runner step is resumable from its state folder (FRC-001); a new `key_root` forces setup again (EUF-007) |
| Privacy boundary | Everything before submit stays on boxes the user owns; the published view is the call vector and one blinded wrap proof (ZK-001, ZK-002, GP-ZK, ERS-002, HLP-004, ZKE-003) |

## 6. Published language

The types that cross a slice boundary. A type marked to build has no schema file today.

| Type | Producer -> consumer | Fields | State |
|---|---|---|---|
| `rsl-call/1` | Compact Compilation -> Sherlock Execution | `schema`, `id`, `circuit`, `label`, `index`, `kits`, `selector`, `n_out`, `l_e`, `n_e`, `tape_len`, `program`, `tape`, `native_words`, `native_comm` | built by fields, no schema file (ERS-001, ERS-013, ERS-014) |
| `RslStatement` (private, owned boxes only) | Sherlock Execution -> Leaf Proving, the argument builder on the box | `prog_d`, `input_d`, `outcome_d`, `comm`, `W` | to build (ERS-010, ERS-002) |
| `StatementPublics` (the only projection that leaves the box) | Settlement builder (on the box) -> Network Operations | `c_p`, `input_pub`, `outcome_pub`, `claim_pub` | built: `accept_args.py`, 16 tests (STL-001, SLC) |
| `LeafKeyRef`, `ProgramAdmission` | Key Management, Leaf Proving -> Sherlock Execution | `key_root`, chip set id, AIR count 6 or 9, authenticated `prog_d` | to build for R_SL (ERS-006, ERS-015) |
| `ProofBundle` | Leaf Proving, Aggregation, Bridge -> Call Flow | leaf, N1 to N3, `L`, `L_Fr` proofs with public words | built as files of `t7_chain` |
| `LfrProofReady` | Bridge -> Wrap | `L_Fr` proof, key pre-hash, public words, `claim_pub` | built (`wrapin.rs`) |
| `sherlock-profile/1` | Scheduling -> Leaf Proving, Call Flow | `format`, host, created, `key_root`, `keys_d`, gpus, cpu, memory, disk, calibration, `setup_s` | built (PF-001 to PF-008) |
| `WrapOutput` | Wrap -> Ledger Verification | `wrap.vk`, `wrap.proof`, `wrap.public.json` | built (WRP-009) |
| `WrapVerdict` | Ledger Verification on muninn -> Box Fleet, Settlement | ok, key bytes, proof bytes, controls refused | built as a runner record (LVR-004, HLP-001) |
| `BoxResult` | Box Fleet -> Placement | path, sha256, box | built as bus lines |
| `SettlementReceipt` | Settlement -> Call Flow, Conformance | contract, tx hash, block, status, bytes, fee, `runs[claim_pub] = outcome_pub` | built (OBS-001) |
| `NativeCallReference` | Network Operations (T23 Part A) -> Conformance | program, circuit, call index, disposition, transcript words, Preview receipt | built: `mycroft-native-ref/1`, `PREVIEW-20261115.md` (ECS-002, STL-006) |

Shared kernels are kept small and named: the kit interfaces and instruction shapes (Compact Compilation with Sherlock Execution), the call vector layout and anchor slot (Wrap with Settlement), and the value and statement encodings of FP-1 to FP-3 (Sherlock Execution with Leaf Proving and Bridge). A JSON schema that two contexts exchange is published language, not a shared kernel.

## 7. Ubiquitous language

The base vocabulary is TERMINOLOGY.md and is not repeated here. The tables below add the terms of each slice and the words that have two meanings.

### Slice A: language, execution, integration

| Term | Meaning in slice A | Note |
|---|---|---|
| Mycroft | the Compact to Sherlock compiler | `mycroft/README.md` |
| Stage 1, Stage 2 | Stage 1 compiles the stock ZKIR file; Stage 2 is a pass inside `compactc`, accepted only by equality with Stage 1 | D-016 |
| GIR | the guarded straight-line IR of 17 instruction shapes that the frontend makes from ZKIR | `mycroft/compiler/src/gir.rs:1-3` |
| program image | selector, `n_out` and code of one compiled circuit | not a contract |
| ABI | selector, tape length, slots, `l_e`, `n_e`, kits of one image | CF-002 calls the file `abi.json` |
| kit | a named group of builtins with one interface and one reference library: FieldEmu, PoseidonFr, Sha256, Bytes, Jubjub, Secp256k1, Tier3 | "kit" in ARCHITECTURE.md section 2 means a module kit of the design; context decides |
| rule family | the interpreted family (120 step rules) or the straight-line family `R_SL` (16 rules) | TERMINOLOGY.md "Sherlock machine" |
| universal interpreter | the Sherlock machine seen from the user: any program, of either rule family, runs and proves under one universal key set | no code object has this name; EUI |
| chip set 3 | the R_SL chip set: micro-op AIRs WORD, ALU, PUBLIC, RANGE, SHA_IN and one hash group | D-017 |
| tape | the private input words of one call: salt, selector, `rand`, arguments, witness, read results | MYC-TAPE |
| `comm` | the communication commitment of a call | equals native `lane1_comm` |
| W | the effect words of a call, equal to the native transcript words | CF-003 |
| call record | one `rsl-call/1` file | private |
| run form, sealed form | run form: the call settles as a general run through `acceptRun`; sealed form: the Compact contract entry holds the wrap key and the ledger applies W | D-022 |
| refused-as-native | a Compact call that native Compact rejects and the T23 route does not settle; not the Rejected outcome of a general run | ECS-004, ERS-009 |
| blocked | a call with no result for a named reason; not a pass and not a mismatch | ECS-005 |

### Slice B: proving

| Term | Meaning in slice B | Note |
|---|---|---|
| shard | one fixed-size proof of the tree; work grows by adding shards | TERMINOLOGY.md |
| leaf, node, `L`, `L_Fr` | the STARK stages; leaf and nodes over BabyBear, `L` over BabyBear with Fr hashing, `L_Fr` over Fr | TERMINOLOGY.md |
| app proof, leaf proof | the per-program proof of the steps (chip set AIRs), and the OpenVM leaf-layer proof that verifies it under the universal leaf key | ARCHITECTURE.md 1.6 |
| chip set | the AIR set of one program's app proof; set 3 is R_SL | D-017 |
| bucket | the AIR counts (6 or 9 with FP-1) that keep the leaf pre-hash universal | RSL-002 |
| N1, N2, N3 | first node (opens `prog_d`, checks keys against `key_root`), inner node, root node | `stmt.rs` `enum Kind` |
| MERGE-WORK | the work unit of GP-4; NW and NW2 are its node modes | bound 5,400 columns, D-012 |
| path C | one leaf, one leaf layer at internal parameters, no internal node, then `L` | TERMINOLOGY.md |
| universal key set | internal-recursive, `L`, `L_Fr` and wrap keys, the same for every program | TERMINOLOGY.md |
| universal leaf key | LOOP wording (T24, T25). App and leaf-layer keys are per program under `key_root`; the term means that every leaf-layer key gives the universal N1 pre-hash | LFP-002, RSL-001, SPL-004 |
| prepared run (new) | one program, input, `r_P` and input salt; the unit over which `claim_pub` is equal | ELA-001 |
| `key_root`, `keys_d` | Merkle root of the per-program key commitments; digest of the universal key record (7 node rows and the foreign rows) | UKS-002 |
| the single rekey | row T25: one keygen from one list | RKY-001 |
| universal key manifest (new) | the record of one universal key set: file digests, `keys_d`, list digest, tree commits, status; no per-program key | UKS-004 |
| tape | the program of the `L_Fr` machine: typed operands on MEM_BUS | LW-020 |
| interface | the public-word layout of `L` (`iface.rs`): statement, call vector, lane map | CLB-001 |
| anchor, lock, chunk | the cell that holds `C_P` (slot 15); the value `A_j` of a chunked call; one of at most 2 parts of a call | ANC, D-011, D-013, D-021 |
| setup mode, profile | the run that measures a box; its record `sherlock-profile/1` | D-014 |
| schedule | shape, device, leaf concurrency and cut of one run; outside `plan_d` and outside every proof | SPL-011 |
| box, roster | a machine the user owns that proves; the roster is the user's list of them. muninn, huginn and odin are the factory fixtures | D-010, BXF-001 |
| slot | in slice B a heavy-slot admission slot. "Blinding slot" is the PRD-002 term and names no code object today (D-040) | two meanings, kept apart |
| gate | a named pass/fail run (V.9, V.10, gp2, x7, d3work, sg6); "the gate" alone is the T12 security gate G1 to G4 with FP-8 | SSG |
| prototype, qualified | status of a key set before and after the T12 gate | UKS-007 |

### Slice C: wrap, settlement, operations

| Term | Meaning in slice C | Note |
|---|---|---|
| wrap | the one BLS12-381 PLONK proof of a ledger call, and the circuit that makes it | TERMINOLOGY.md |
| call vector | the 1,026 public inputs of the wrap: `[binding_input, comm]`, anchor prologue, effect words | TERMINOLOGY.md |
| anchor | the sealed ledger cell that holds `c_p` of the ledger's program (slot 15 after T25) | "anchor" in Governance never means this |
| settle | include an `acceptRun` or `acceptCollection` call on chain | not "verify": the node verifies, the contract settles |
| refusal | a node JSON-RPC error on a submitted call; a timeout or a transport error is not a refusal | PVW-007, PVW-008 |
| epoch (claim epoch) | the batching window of one operator: 3,600 s or 2^20 claims | differs from a registry epoch (LDG-008) and from an era |
| era | one sealed set of ledgers with its own universal key set, listed in the era table | proposed name of T22, not yet in TERMINOLOGY.md (new) |
| sealed | deployed with an empty maintenance committee and threshold 1; no key change is possible | differs from Compact `sealed ledger` (a field only the constructor writes); both are used, always with the noun |
| receipt | the public record of a settled or refused call | never holds a private word |
| box | a machine the user owns that proves (muninn, huginn, odin) | case A |
| helper | a host the user does not own | case B, refused |
| KEEP | the lead's verdict that a box result enters the record after the muninn check | loop word |

### Words with more than one meaning

| Word | Meanings | Rule |
|---|---|---|
| program | a compiled circuit of a contract (Compact Compilation); any image with a `prog_d` (Sherlock Execution, Proving, Settlement) | say "circuit" or "entry" for the Compact meaning |
| kit | a builtin group with one interface (slice A); a module kit of ARCHITECTURE.md section 2 (design) | say "R_SL kit" or "module kit" |
| key | universal key (node, `L`, `L_Fr`, wrap); per-program key under `key_root`; wallet key; Fenrir operator key | always with its adjective |
| anchor | the ledger cell that holds `C_P`; never a governance term | D-021 |
| epoch, era | claim epoch (batching window); registry epoch; era (one sealed key set in the era table) | always with its adjective |
| sealed | a contract with no maintenance authority; a Compact `sealed ledger` field; the Mycroft sealed form of a call | always with its noun |
| refusal, rejection | compiler refusal (no image); execution rejection (no statement); node refusal (JSON-RPC error on a submitted call) | name the stage |
| helper, box | box: a machine the user owns (case A); helper: a host the user does not own (case B, refused) | D-010 |
| verify, settle | the node or a verifier verifies; a contract settles | not exchanged |

## 8. Contexts, capabilities and slices

| Context | Kind | Capability specs | Slice |
|---|---|---|---|
| Compact Compilation | core | `sherlock-e2e-compact-compiler` (ECC), reuses MYC (frontend, values, lowering, witness-tape, ledger-ops, builtins, refusals), CF-001 to CF-014 | A |
| Sherlock Execution | core | `sherlock-e2e-universal-interpreter` (EUI), `sherlock-e2e-rsl-kits` (EKT), `sherlock-e2e-rsl-statement` (ERS), reuses RSL, GP-001, CORE, RFB | A |
| Conformance | supporting | `sherlock-e2e-compact-suite` (ECS), reuses STL-005, STL-006, MYC-ACC, MYC-LOW-018 | A |
| Call Flow | supporting (a process, section 5) | `sherlock-e2e-user-flow` (EUF) | A |
| Leaf Proving | core | `sherlock-e2e-leaf-proving` (LFP), reuses RSL-001 to RSL-008, CPR-002, SPL-006 | B |
| Aggregation | core | `sherlock-e2e-node-aggregation` (NAG), `sherlock-e2e-gp-hooks` (GPH), reuses NODE, SHP, CLM, CONT, FREG, FVR, GP-001 to GP-004 | B |
| Bridge | core | `sherlock-e2e-l-transcript` (LTR), `sherlock-e2e-call-binding` (CLB), reuses LW, FLG, HDM, ANC | B |
| Key Management | core | `sherlock-e2e-universal-keys` (UKS), reuses RKY-001, RKY-002, RKY-005, CPR-003, SPL-004, SPL-005 | B |
| Scheduling | core | `sherlock-e2e-setup-mode` (STM), `sherlock-e2e-elastic-proving` (ELA), reuses SPL-011, LPC-006, LPC-007 | B |
| Box Fleet | supporting | `sherlock-e2e-box-fleet` (BXF), `sherlock-e2e-gpu-proving` (GPV), `sherlock-e2e-prover-daemon` (PDM), reuses PRD, GPW-007, LPC-005 | B |
| Assurance | supporting | `sherlock-e2e-zero-knowledge` (ZKE), `sherlock-e2e-stark-security` (SSG), reuses ZK, SQG, DG, FPR, LPC-004, LPC-016 | B |
| Wrap | core | `sherlock-e2e-wrap` (WRP), reuses LW, GPW, RKY-003, RKY-004 | C |
| Ledger Verification | generic (external), with an anti-corruption layer | `sherlock-e2e-ledger-verifier` (LVR), reuses MID | C |
| Settlement | core | `sherlock-e2e-ledger-contract` (SLC), `sherlock-e2e-batched-settlement` (BAT), reuses LDG, STL, ANC-012 to ANC-014 | C |
| Governance | supporting | `sherlock-e2e-governance` (EGV), reuses GOV, ERT, ELD, CRR | C |
| Network Operations | supporting | `sherlock-e2e-preview-deployment` (PVW), `sherlock-e2e-wallet` (WAL), `sherlock-e2e-sdk-cli` (SCL), `sherlock-e2e-observability` (OBS), `sherlock-e2e-recovery` (FRC) | C |
| Placement (settlement edge) | supporting | `sherlock-e2e-helper-machines` (HLP), reuses ZK-001, ZK-009, ZK-018, PRD-002, PRD-003 | C |

Mapping rationale. Compact Compilation maps to one capability (ECC) because the Mycroft change already splits the compiler into 17 modules; this change adds one integration spec on top. Sherlock Execution maps to three capabilities (EUI, EKT, ERS) because the machine, the kits and the statement change at different times and have different consumers. Conformance is its own context: it must stay able to report a mismatch against every other context. The slice boundary A to B cuts through one shared kernel (FP-1 to FP-3 encodings); the gaps on that cut are listed in `COVERAGE.md` (ERS-006, ERS-007, ERS-012, ERS-015).
