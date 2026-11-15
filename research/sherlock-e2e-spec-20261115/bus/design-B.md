# Slice B design notes for design.md (Fable-B)

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

