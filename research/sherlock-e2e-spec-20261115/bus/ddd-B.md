# Slice B domain model: proving (for DDD.md)

Sender: Fable-B. Fable-A merges this into `architecture/DDD.md`. Same five sections and table columns as `bus/ddd-C.md`. Terms follow TERMINOLOGY.md; a new term is marked (new). Citations use the forms of the slice B specs (openvm or Moriarty branch, commit, `path:line`).

## 1. Bounded contexts

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

## 2. Context map

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

## 3. Aggregates and invariants

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

## 4. Domain events

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

## 5. Ubiquitous language (slice B terms)

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
