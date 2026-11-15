# Coverage of the Sherlock end-to-end prototype

This matrix maps every system component and every LOOP row to its capability spec and its EARS IDs. Sources of the component list: `research/sherlock-final-design-20261111/ARCHITECTURE.md` section 1.1 and the kits of section 2, TERMINOLOGY.md, and the operations tools on the lane branches. Source of the row list: `~/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` (rows T0 to T25, read 2026-11-15).

Withdrawn IDs (not reused; a range that spans one covers only the IDs that exist): ECC-006, ECC-008, ERS-004, EKT-008, EUF-003, EUF-004, WRP-005, D-023 (cancelled), D-072 (merged into D-021).

Reading rules. "EARS IDs" are requirements of this change. "Reused IDs" are requirements of the changes named in proposal.md (processor-final, modular-leaf, completion, Mycroft, governance, setup); a prefix with "(all)" means every requirement of that module. No cell is blank. A cell "none" in the gap column means no gap is known; a gap states its owner lane. A status word in this file repeats LOOP.md; it claims no new result.

Totals: 30 capability specs, 320 EARS requirements in this change; 55 component rows; 33 LOOP rows.

## 1. Capability inventory

| Capability spec | Prefix | Requirements | Slice |
|---|---|---|---|
| sherlock-e2e-batched-settlement | BAT | 12 | C |
| sherlock-e2e-box-fleet | BXF | 14 | B |
| sherlock-e2e-call-binding | CLB | 8 | B |
| sherlock-e2e-compact-compiler | ECC | 12 | A |
| sherlock-e2e-compact-suite | ECS | 14 | A |
| sherlock-e2e-elastic-proving | ELA | 7 | B |
| sherlock-e2e-governance | EGV | 10 | C |
| sherlock-e2e-gp-hooks | GPH | 8 | B |
| sherlock-e2e-gpu-proving | GPV | 7 | B |
| sherlock-e2e-helper-machines | HLP | 8 | C |
| sherlock-e2e-l-transcript | LTR | 8 | B |
| sherlock-e2e-leaf-proving | LFP | 12 | B |
| sherlock-e2e-ledger-contract | SLC | 12 | C |
| sherlock-e2e-ledger-verifier | LVR | 8 | C |
| sherlock-e2e-node-aggregation | NAG | 11 | B |
| sherlock-e2e-observability | OBS | 12 | C |
| sherlock-e2e-preview-deployment | PVW | 13 | C |
| sherlock-e2e-prover-daemon | PDM | 9 | B |
| sherlock-e2e-recovery | FRC | 15 | C |
| sherlock-e2e-rsl-kits | EKT | 10 | A |
| sherlock-e2e-rsl-statement | ERS | 14 | A |
| sherlock-e2e-sdk-cli | SCL | 10 | C |
| sherlock-e2e-setup-mode | STM | 19 | B |
| sherlock-e2e-stark-security | SSG | 7 | B |
| sherlock-e2e-universal-interpreter | EUI | 10 | A |
| sherlock-e2e-universal-keys | UKS | 10 | B |
| sherlock-e2e-user-flow | EUF | 12 | A |
| sherlock-e2e-wallet | WAL | 10 | C |
| sherlock-e2e-wrap | WRP | 11 | C |
| sherlock-e2e-zero-knowledge | ZKE | 7 | B |

## 2. Components

| Component | Context | Slice | Capability spec | EARS IDs | Reused IDs | Decisions | Gap (owner lane) |
|---|---|---|---|---|---|---|---|
| Compact source and stock `compactc` 0.36.0 (external) | Native Compact | A | sherlock-e2e-compact-compiler | ECC-001 | MYC-FE-001 to MYC-FE-005 | D-016 | none |
| Mycroft frontend: ZKIR 3.1 to GIR, types, refusals | Compact Compilation | A | sherlock-e2e-compact-compiler | ECC-001, ECC-014 | MYC-FE-001 to MYC-FE-021; CF-013, CF-014 | D-016, D-019 | Jubjub input types refused (ECC-014): Mycroft lane, row T24 |
| Mycroft lowering: GIR to the 13 `SInstr` shapes, ABI | Compact Compilation | A | sherlock-e2e-compact-compiler | ECC-003 to ECC-005 | MYC-LOW-001 to MYC-LOW-019; CF-004, CF-007 | D-017 | test T-CHIP-RULES not built (ECC-005): Mycroft lane |
| Mycroft modules values, witness-tape, ledger-ops, builtins, refusals | Compact Compilation | A | sherlock-e2e-compact-compiler | ECC-002, ECC-007 | MYC-VAL, MYC-TAPE, MYC-LED, MYC-BLT, MYC-REF (all); CF-005, CF-006 | D-019 | MYC-BLT-007 keccak text stale (ECC-007); MYC-REF-052 clause superseded (ERS-015): Mycroft lane |
| Mycroft modules transcript, seal, entry-binding, acceptance (sealed form, E-1 placement V) | Compact Compilation | A | sherlock-e2e-rsl-statement | ERS-003, ECC-009 | MYC-TRN, MYC-SEAL, MYC-BIND, MYC-ACC (all); CF-008 to CF-010; ANC-015 | D-021, D-022 | GAP: the sealed form and the `bind_label` path are not built for R_SL; the prototype uses the run form (D-022). Owner lane: Mycroft with rekey-chips, after T15 |
| Mycroft modules claims, chunks, composition, keys, cost | Compact Compilation | A | sherlock-e2e-rsl-statement | ERS-006, ERS-015 | MYC-CLM, MYC-CHK, MYC-CMP, MYC-KEY, MYC-COST (all) | D-004, D-011 | GAP: design only for R_SL (no `prog_d`, no chunked R_SL call). Owner lane: rekey-chips (T25) for `prog_d`; chunks after T15 |
| Mycroft spec text that the owner rulings and the built chip set have passed: MYC-BLT-007, MYC-SEAL-021 and MYC-SEAL-005, MYC-CHIP-001, MYC-CHIP-004, MYC-REF-052 | Compact Compilation | A | sherlock-e2e-compact-compiler, sherlock-e2e-rsl-kits, sherlock-e2e-rsl-statement | ECC-007, ECC-009, EKT-001, ERS-003, ERS-015 | MYC-BLT-007, MYC-SEAL-005, MYC-SEAL-021, MYC-CHIP-001, MYC-CHIP-004, MYC-REF-052; CF-013, ANC-015 | D-017, D-019, D-021, D-022 | GAP: the Mycroft text is stale: MYC-BLT-007 routes `keccak256` and secp256k1 to a tier 3 kit (owner 2026-11-15: R_SL kits); MYC-SEAL-021 and MYC-SEAL-005 call the slot-15 rekey OWNER-pending (owner confirmed 2026-11-15); MYC-CHIP-001 lists rules that are not the built chip set; MYC-CHIP-004 and MYC-REF-052 as above. Owner lane: Mycroft lane edits `mycroft/spec/` (AUDIT.md notes M1, M2) |
| Design documents FINAL-DESIGN.md 1.6 and ARCHITECTURE.md (lock form) | Bridge | B (text), lead (edit) | sherlock-e2e-call-binding | CLB-005 | ANC-002 (fixed), ANC-017; LPC-017 | D-013 | GAP: both documents still show `P2(DOM_LOCK)`; the `x_j` lock form is canon (owner 2026-11-14). Owner lane: lead (LPC-017) |
| Mycroft Lean specification and oracle (`mycroft/lean/`) | Compact Compilation | A | sherlock-e2e-compact-compiler | ECC-011 | LPC-019; CF-012; MYC-LOW-010 | D-016 | premise S1, wide arguments, `lake build --wfail` open (LPC-019): Mycroft lane |
| Mycroft factory gates (KRS suite, Quint replay, mutations, compare) | Compact Compilation | A | sherlock-e2e-compact-compiler | ECC-002 | LPC-013 | D-026 | deliverables staged, not committed (LPC-013): Mycroft lane |
| Compile command (`sherlock compile`) | Compact Compilation | A | sherlock-e2e-compact-compiler | ECC-012 to ECC-013 | CF-002 | D-085 | GAP: no `sherlock` binary; the prototype runs `mycroft-suite` (D-085). Owner lane: Mycroft, after T15 |
| Call record `rsl-call/1` and `mycroft-emit` | Sherlock Execution | A | sherlock-e2e-rsl-statement | ERS-001 to ERS-002, ERS-013 to ERS-014 | MYC-TAPE-001, MYC-TAPE-008, MYC-TAPE-012 | D-020 | two formats and no schema check (ERS-013, ERS-014): Mycroft lane with the rekey-chips lane, before T23 live |
| R_SL executor (`compile::build`, `rsl::prove_call`) | Sherlock Execution | A | sherlock-e2e-rsl-statement, sherlock-e2e-universal-interpreter | ERS-005, ERS-009, ERS-011, EUI-004, EUI-006 to EUI-008 | CF-003; MYC-LOW-005 to MYC-LOW-007; RSL-003 | D-017 | none |
| R_SL rule table and chip set 3 AIRs (WORD, ALU, PUBLIC, RANGE, SHA_IN, hash group) | Sherlock Execution | A (rules), B (AIRs under the leaf) | sherlock-e2e-rsl-kits, sherlock-e2e-leaf-proving | EKT-001 to EKT-003, LFP-003 to LFP-004 | RSL-001, RSL-002; MYC-CHIP-001 to MYC-CHIP-008 | D-017 | T-RULES-D not built (EKT-001): rekey-chips lane, T25 |
| Kit SHA-256 | Sherlock Execution | A | sherlock-e2e-rsl-kits | EKT-004, EKT-006 to EKT-007 | RSL-007; MYC-BLT-003 | D-017, D-024 | GAP: a program with SHA-256 and keccak256 has no R_SL proof; temporary named refusal (EKT-004, D-024). Owner lane: rekey-chips; the owner sees it at T15 |
| Kit keccak256 | Sherlock Execution | A | sherlock-e2e-rsl-kits, sherlock-e2e-compact-compiler | EKT-004, EKT-006 to EKT-007, ECC-007 | RSL-004 | D-019, D-024 | Mycroft lowering open (ECC-007); no MEASURED suite run: Mycroft lane, rows T24, T23 |
| Kit Jubjub | Sherlock Execution | A | sherlock-e2e-rsl-kits | EKT-006 to EKT-007, EKT-010 | RSL-006; MYC-BLT-006; MYC-FE-021 | D-027 | GAP: no interface file (EKT-010), no forged-row test (EKT-007), inputs refused (ECC-014). Owner lane: Mycroft with rekey-chips, row T24 |
| Kit secp256k1 | Sherlock Execution | A | sherlock-e2e-rsl-kits | EKT-006 to EKT-007, EKT-009 | RSL-005; CF-013; MYC-FE-013 to MYC-FE-020 | D-019 | interface file outside every repository (EKT-009): rekey-chips lane, row T24 |
| Kit Poseidon (transient hash) | Sherlock Execution | A | sherlock-e2e-rsl-kits | EKT-005 to EKT-006 | MYC-BLT-001, MYC-BLT-002; RSL-008 | D-017 | range checks of Poseidon inputs have no rekey list row (RSL-008): rekey-chips lane, T25 |
| Kit ALU, FIELD emulation, RANGE and Bytes (R_SL) | Sherlock Execution | A | sherlock-e2e-rsl-kits | EKT-003, EKT-011 | RSL-008; ALU-001 to ALU-005; RNG-001 to RNG-004; OD-14 | D-017 | range checks have no rekey list row (RSL-008): rekey-chips lane, T25 |
| Tier 3 kit (sha512) | Sherlock Execution | A | sherlock-e2e-compact-compiler | ECC-010 | MYC-BLT-007, MYC-BLT-008, MYC-BLT-011 | D-019 | GAP: not built; a `sha512` call is refused. No suite call needs it. Owner lane: Mycroft, after T15 |
| Sherlock machine: rule families, `machine_version` (the universal interpreter) | Sherlock Execution | A | sherlock-e2e-universal-interpreter | EUI-001 to EUI-003, EUI-009, GPH-001 | GP-001; CPR-003, CPR-005; UIREQ-001 to UIREQ-035 (retained by reference) | D-004, D-008, D-025 | GAP: the UIREQ retention map does not exist (EUI-009), and R_SL programs carry no `machine_version` yet (EUI-001). Owner lane: slice A (map), rekey-chips lane, T25 |
| Interpreted-family leaf chips: BUS, RANGE, ALU, DEC, HASH, CORE, STACKS, MEM window, MEMORY BACKEND, PAGE/ROM, ZmLogUp variant, VALUE DIGEST, BOUNDARY COMMITMENT, INIT | Leaf Proving | B (A for the rule meaning) | sherlock-e2e-leaf-proving, sherlock-e2e-universal-interpreter | LFP-001 to LFP-002, LFP-005, EUI-005, EUI-010 | BUS, RNG, ALU, DEC, HSH, CORE, STK, MEM, MBK, PAGE, ZML, VDG, BND, INIT (modular-leaf, all) | D-008, D-018 | GAP: 5 subset rules and 2 value rules built of 120 (EUI-010). Owner lane: slice B, modular-leaf groups after V; no LOOP row |
| REFERENCE BRIDGE (semantic reference and projection) | Sherlock Execution | A | sherlock-e2e-universal-interpreter | EUI-006, EUI-010 | RFB-001 to RFB-004 | D-008 | covered for the built rules only (EUI-010): slice B with the reference lane |
| HOST CHECKPOINT STORE and CONTINUATION | Aggregation | B | sherlock-e2e-gp-hooks | GPH-006 | CKPT-001 to CKPT-003; CONT-001 to CONT-005 | D-008 | none |
| Run statement and its binding (`prog_d`, `input_d`, `outcome_d`, FP-1 words) | Sherlock Execution, Leaf Proving | A (form), B (binding) | sherlock-e2e-rsl-statement, sherlock-e2e-leaf-proving, sherlock-e2e-node-aggregation | ERS-006 to ERS-008, ERS-010, ERS-012, ERS-015, LFP-006 to LFP-007, NAG-008 | STL-001; MYC-CHIP-004; MYC-KEY-003; INIT-001 to INIT-008 | D-022, D-071 | GAP (HIGH): no `prog_d`, no INIT and no FP-1 binding for R_SL (ERS-006, ERS-007, ERS-012). Owner lane: rekey-chips (slice B), before T25 |
| Compact test suite corpus (36 programs, native references) | Conformance | A | sherlock-e2e-compact-suite | ECS-001 to ECS-002, ECS-011, ECS-013, ECC-010 | MYC-LOW-018; STL-006 | D-016 | call identity map not built (ECS-013): T23 lane |
| T23 runner (compile, prove, settle, compare) | Conformance | A (Part B), C (settle, Part A) | sherlock-e2e-compact-suite, sherlock-e2e-sdk-cli | ECS-003 to ECS-010, ECS-012, ECS-014, SCL-008, BAT-004 | STL-002 to STL-006 | D-019, D-022, D-084 | stale kit reasons (ECS-006), vacuous zero (ECS-012), 62 refusals DERIVED (ECS-009): T23 lane |
| End-to-end call flow | Call Flow | A | sherlock-e2e-user-flow | EUF-001 to EUF-014 | LPC-008, LPC-014; CF-011 | D-002, D-010, D-014, D-085 | GAP: no run joins all stages (EUF-001); no clean-box run (EUF-011). Owner lane: lead, rows T17 (c), T23 |
| Leaf: app proof and leaf proof of one shard | Leaf Proving | B | sherlock-e2e-leaf-proving | LFP-001 to LFP-012 | RSL-001 to RSL-003; CPR-002 | D-007, D-017, D-018 | GAP (HIGH): R_SL statement, `prog_d` and `input_d` not bound (LFP-010 to LFP-012, HIGH); 137 of 142 calls have no probe (LFP-002); chip set 3 has no named shape vector (LFP-005); both hash kits in one program have no route (LFP-003, D-024): rekey-chips (lane rsl-bind for the bindings) |
| NODE INTEGRATION: N1, N2, N3; PROOF SHAPE; every AIR required | Aggregation | B | sherlock-e2e-node-aggregation | NAG-001 to NAG-011 | NODE-001 to NODE-015; SHP-001 to SHP-009; LPC-018 | D-012 | GP2-C2.1 open (NAG-007); T3 parent constraints never fired (LPC-018): slice B |
| `L` (bridge layer), `L_Fr`, the tape and the transcript; deferred field log | Bridge | B | sherlock-e2e-l-transcript | LTR-001 to LTR-008 | LW-001 to LW-022; FLG-001 to FLG-009; HDM-001 to HDM-010 | D-005, D-007 | `L` is at 1,995 of 2,000 columns before FP-3 and slot 15 (LTR-003); `LfrProofReady` is not a built type (LTR-006, LTR-008): T25, t23-suite |
| Call binding: FP-3, anchors, locks, chunks (iface and `L` side) | Bridge | B | sherlock-e2e-call-binding | CLB-001 to CLB-008 | ANC-001 to ANC-019 | D-011, D-013, D-021 | FP-3 not committed and one test passes without a refusal (CLB-002, CLB-003); slot 15 not merged with the extended lock (CLB-004, UKS-009); three copies of the slot constant (CLB-008); K_MAX = 2 is a model, not code (CLB-007): lanes fp3, t12-slot15, t20-batched |
| GP-2 hook (consuming claims), claim consumption in circuit | Aggregation | B | sherlock-e2e-gp-hooks | GPH-002 | CLM-001 to CLM-009; LPC-001 | D-008 | `L` over SET open (LPC-003): row T8c (no LOOP row), slice B |
| GP-3 hook, general verifier (five subkits), foreign registry | Aggregation | B (C for the registry on the ledger) | sherlock-e2e-gp-hooks | GPH-003 to GPH-004 | FVR-001 to FVR-006; FREG-001 to FREG-005; LPC-003 | D-008 | GAP: X8 in-circuit Compact verifier not built (LPC-003): row T8c (no LOOP row), slice B |
| GP-4 hook (any shape), MERGE-WORK | Aggregation | B | sherlock-e2e-gp-hooks, sherlock-e2e-node-aggregation | GPH-005, GPH-007 to GPH-008, NAG-005 | LPC-002; SQG-013; SHP-008 | D-008, D-012 | GAP: the complete kernel width with STACKS and VALUE DIGEST is 5,121 DERIVED, not MEASURED (4,988 MEASURED without them; openvm `rekey-chips` dcf1b4c `frozen.rs:166`); the earlier 5,417 is superseded (NAG-005, LPC-002): row T8c |
| Universal key set, `key_root`, `keys_d`, the single rekey | Key Management | B | sherlock-e2e-universal-keys | UKS-001 to UKS-010 | RKY-001 to RKY-005; CPR-003 | D-004, D-015 | GAP: rekey list lacks 6 rows: FP-3, slot 15, range checks, R_SL statement bindings, R_SL key leaf index, era-check words (UKS-010); `KEYDAG_IDX = 3` collides with the planned R_SL id 3 (UKS-003); no integrated keygen tree (UKS-009): lanes fp3, t12-slot15, rekey-chips, t22-governance, T25 |
| Setup mode and the box profile; shard plan | Scheduling | B | sherlock-e2e-setup-mode | STM-001 to STM-019 | ST-001 to ST-008, PF-001 to PF-008, SC-001 to SC-014 (setup); SPL-001 to SPL-012 | D-014 | F-3 menu code still in `sched.rs` and called by the part (b) runner (STM-014); per-GPU calibration open (STM-004); profile path not read by code (STM-016): t16-elastic, t23-suite |
| Elastic proving across schedules and boxes | Scheduling | B | sherlock-e2e-elastic-proving | ELA-001 to ELA-007 | LPC-006, LPC-007 | D-014, D-018 | muninn `hw` run and muninn join NOT RUN; two run lengths and 1x to 8x open (LPC-006, LPC-007); the record names no device per stage (ELA-007): row T16 |
| Box fleet: muninn, huginn, odin; heavy-slot; gpu.lock; box bus | Box Fleet | B | sherlock-e2e-box-fleet | BXF-001 to BXF-014 | LPC-020; ZK-018 | D-010 | no script enforces the owned-box roster, the digest check, the muninn route or the lock rule (BXF-001, BXF-007, BXF-014, BXF-005); the tools are outside every repository: box tooling (lead) |
| GPU STARK paths | Box Fleet | B | sherlock-e2e-gpu-proving | GPV-001 to GPV-007 | GPW-001 to GPW-008 (wrap side: slice C) | D-007, D-031 | the leaf layer proves on the CPU (GPV-007); `L` on the GPU is not timed (GPV-005); no GPU against CPU public-value test (GPV-002): t14-gpu, t12-gate |
| Resident prover daemon and warm path | Box Fleet | B | sherlock-e2e-prover-daemon | PDM-001 to PDM-009 | PRD-001 to PRD-005; LPC-005 | D-002, D-040, D-048 | CLOSED 2026-11-15: the owner accepted the per-proof draw as OD-13 one-use slots (D-040); was: GAP: the OD-13 slot pool is not required (owner 2026-11-15, D-040: the per-proof draw satisfies one-use slots); no recorded check of PRD-003, PRD-004 (PDM-007); final timing open (LPC-005): rows T14, T14b |
| ZERO KNOWLEDGE (proving layer) | Assurance | B | sherlock-e2e-zero-knowledge | ZKE-001 to ZKE-007 | ZK-001 to ZK-019; GP-ZK | D-002, D-010 | T11 code and tests untracked, `eq_zk` and `slot_reuse` have no recorded run (ZKE-001, ZKE-007, PDM-009); no code refuses a host outside the roster (ZKE-004, ZK-018); a `test-seed` binary can settle (ZKE-002): T11 lane, box tooling, t23-suite |
| Security qualification gate, derived gates, freeze prerequisites, Quint lanes | Assurance | B | sherlock-e2e-stark-security | SSG-001 to SSG-007 | SQG-001 to SQG-016; DG-001 to DG-011; FPR-001 to FPR-019; LPC-004, LPC-016 | D-003 | G3 legs not built, no gate run (SSG-006, SQG-014 to SQG-016); GP2-C2.1 CONFIRMED with no fix (SSG-003, NAG-007); 8 findings NOT ATTEMPTED (SSG-004); no circuit change list (SSG-005); no D3 on R_SL app keys (SSG-001): t12-gate, t13 |
| WRAP circuit, decider | Wrap | C | sherlock-e2e-wrap | WRP-001 to WRP-004, WRP-006 to WRP-012 | DG-001 to DG-003, DG-010; LW-001 to LW-022; GPW-001 to GPW-008; DCD-001 to DCD-007; RKY-003, RKY-004; ANC-013, ANC-016 | D-001, D-006, D-007, D-083 | re-measure rows and VK bytes at T25 (RKY-003, RKY-004); the public input count is LW-003: wrap lane (slice C), row T25 |
| Midnight ledger 9.1.0.1 verifier (external) and the local ledger check | Ledger Verification | C | sherlock-e2e-ledger-verifier | LVR-001 to LVR-008 | MID-001 to MID-007; CPR-004 | D-005 | none |
| LEDGER CONTRACT: SherlockLedger, `acceptRun`, the argument map | Settlement | C | sherlock-e2e-ledger-contract | SLC-001 to SLC-012 | LDG-001, LDG-002; STL-001 to STL-006; ANC-014, ANC-015 | D-021, D-070, D-071, D-084 | GAP: live settle is off until the leaf statement binds `prog_d`, `input_d`, `outcome_d` (SLC-012): slices A, B; MAP.md and `accept_args.py` staged, not committed. Owner lane: rsl-bind (slices A, B), t23-args (slice C) |
| Batched settlement: `acceptCollection`, claims root, chunk cap; claim consumption on the ledger | Settlement | C | sherlock-e2e-batched-settlement | BAT-001 to BAT-012 | ANC-012; LDG-003 to LDG-008; LW-009, LW-011, DCD-004; LPC-010 (CLM-001 to CLM-009 are in circuit, slice B) | D-009, D-011, D-073, D-074, D-075 | GAP: the `L` collection form, the claims tree proof and the SHK_FCLAIM tag have no code (BAT-009, BAT-012): slice B, row T8c. The registry update entry, the N-8 history rule and leaf-list storage have no LOOP row (proposed T20b). Owner lane: slice C with the lead |
| Governance: sealed contracts, era table, Fenrir operator verbs | Governance | C | sherlock-e2e-governance | EGV-001 to EGV-010, BAT-010 to BAT-011 | GOV, ERT, ELD, CRR (governance, branch `t22-governance`); LPC-012 | D-076, D-077, D-078 | GAP: the era-check words are not in the T25 rekey list and premises A1 to A3 are untested (EGV-005, EGV-009); no sealed deploy tool (EGV-001, EGV-002); no Carbon verdict (LPC-012). Owner lane: row T22, slice B (UKS) for the rekey list |
| Preview deployment and the validium demo | Network Operations | C | sherlock-e2e-preview-deployment | PVW-001 to PVW-013 | LPC-008, LPC-009; STL-003 | D-079, D-081 | T17 (b), (c) and T19 open (LPC-008, LPC-009): rows T17, T19 |
| Wallet | Network Operations | C | sherlock-e2e-wallet | WAL-001 to WAL-010 | ZK-012 | D-080 | derived keys not scrubbed; pins are constants; no mode check on the read path (WAL-002, WAL-003, WAL-010): SDK lane |
| SDK and CLI (settlement side) | Network Operations | C | sherlock-e2e-sdk-cli | SCL-001 to SCL-010 | CF-002 | D-085 | sdk-node fails 3 of 144 without `SHERLOCK_SDK_ROOT` (LPC-018); no one binary (D-085): SDK lane (slice C) |
| Observability: receipts, privacy scans, timing, records, prover GUI page | Network Operations | C | sherlock-e2e-observability | OBS-001 to OBS-012 | LPC-011, LPC-016, LPC-017 | D-081 | prover GUI beyond the stub open (LPC-011): row T21 |
| Failure and recovery; durability; pushes | Network Operations | C | sherlock-e2e-recovery | FRC-001 to FRC-015 | RKY-002; LPC-010, LPC-014, LPC-015 | D-079 | unknown inclusion state, artifact checks and suite resume not built (FRC-012 to FRC-014): row T23 driver |
| Helper machines (case A; case B deferred) | Placement | C | sherlock-e2e-helper-machines | HLP-001 to HLP-008 | ZK-001, ZK-009, ZK-018; PRD-002, PRD-003 | D-010, D-082 | no code guard for case B (ddd-C): slice C |

## 3. LOOP rows

| Row | Work | LOOP status | Slice | Capability spec | EARS IDs | Reused IDs | Open or gap |
|---|---|---|---|---|---|---|---|
| T0 | Make the work durable (commits, patch series) | DONE | C | sherlock-e2e-recovery | FRC-010 | none: process row | none |
| T1 | One integration branch `tested` | DONE | C | sherlock-e2e-recovery, sherlock-e2e-observability | FRC-015, OBS-010 | none: process row | none |
| T2 | Repair the record (ARCHITECTURE 1.12, RESULTS) | DONE | C | sherlock-e2e-observability | OBS-009 to OBS-010 | LPC-017 | FINAL-DESIGN 1.6 and ARCHITECTURE.md keep the old lock form (LPC-017): lead |
| T3 | Every AIR required | DONE, open items | B | sherlock-e2e-node-aggregation | NAG-002, LTR-004 | LPC-018; SHP | parent constraints never fired; sdk-node 3 of 144 (LPC-018): slice B, slice C |
| T4 | Wrap row budget at k = 21 | DONE | C | sherlock-e2e-wrap | WRP-002 to WRP-003 | RKY-003; DG | re-measure at T25 (RKY-003) |
| T5 | Leaf schemas (FP-9), signed | DONE, signed | B | sherlock-e2e-leaf-proving, sherlock-e2e-universal-interpreter | LFP-005, EUI-010 | modular-leaf MEM, VDG, DEC, CORE; SRC-021 (`sherlock-vm-consensus-20261107`) | none |
| T6 | Kits as OpenVM AIRs; one program proves leaf to `L_Fr` | Group V DONE | B | sherlock-e2e-leaf-proving, sherlock-e2e-universal-interpreter | LFP-001 to LFP-005, NAG-008, GPH-001, EUI-005, EUI-010 | modular-leaf chip specs; CPR-002 | GAP: groups after V (115 of 120 rules) have no LOOP row (EUI-010): slice B |
| T7 | `prog_d` and the general key route | DONE | B | sherlock-e2e-leaf-proving, sherlock-e2e-node-aggregation | LFP-006 to LFP-007, LFP-011, NAG-003 to NAG-004, NAG-008, GPH-003 | NODE; CPR-003 | `prog_d` for R_SL open (ERS-006): rekey-chips |
| T7c | Path C, the one-leaf Compact call | DONE | B | sherlock-e2e-l-transcript, sherlock-e2e-prover-daemon | LTR-007, UKS-003, PDM-007 | LW; OD-19 | none |
| T8 | GP hooks X12, X8, X14, X7, N-4 | PARTIAL | B | sherlock-e2e-gp-hooks | GPH-002 to GPH-006 | LPC-001; CONT; FVR | moved to T8b, T8c |
| T8b | Close the GP hooks T8 left open | PARTIAL | B | sherlock-e2e-gp-hooks | GPH-002 to GPH-005, GPH-007 | LPC-001 | open items moved to T8c |
| T8c | STACKS and VALUE DIGEST in MERGE-WORK; X8 in-circuit verifier; `L` over SET | NO LOOP ROW | B | sherlock-e2e-gp-hooks, sherlock-e2e-node-aggregation, sherlock-e2e-batched-settlement | GPH-004, GPH-005, GPH-008, NAG-005, BAT-009, BAT-012 | LPC-002, LPC-003 | GAP: LOOP.md names T8c in row T8b and has no row for it (AUDIT.md M3). Owner lane: lead adds the row; slice B builds |
| T9 | Encodings and contract surface (ENC-2 to ENC-5, N-11, LW-009) | DONE | B, C | sherlock-e2e-call-binding, sherlock-e2e-ledger-contract | CLB-001, SLC-001 | LW-009; DCD-004; ANC | ENC-4 and ENC-5 AIR side waits on T6 (LOOP.md row T9): slice B |
| T10a | Contract half: decoder, design `acceptRun`, devnet call | DONE | C | sherlock-e2e-wrap, sherlock-e2e-ledger-contract | SLC-001 to SLC-004 | LW-003 (public input count); LDG; LW | none |
| T10 | Bound wrap and the design contract | DONE | C | sherlock-e2e-wrap, sherlock-e2e-ledger-verifier | WRP-001 to WRP-004, WRP-008 to WRP-010, LVR-001 to LVR-005 | RKY-003, RKY-004; MID | none |
| T10m | Merge T10 into `tested`, rekey, rerun gates | DONE | C, B | sherlock-e2e-universal-keys, sherlock-e2e-wrap | UKS-002, UKS-006, WRP-008 | RKY | none |
| T10q | Fix the CONFIRMED Quint counterexamples; NT-1 to NT-3 | running; push held on FP-3 | B | sherlock-e2e-node-aggregation, sherlock-e2e-call-binding | NAG-006 to NAG-007, NAG-011, CLB-002 to CLB-006 | ANC-016 to ANC-019; LW-022; LPC-015 | merge of t10q-ntfix and FP-3 pending (AUDIT.md M10): lead |
| T11 | Zero knowledge: blinders, guards, EQ-ZK, slots, GPU half, ZK-L2 | running | B | sherlock-e2e-zero-knowledge, sherlock-e2e-prover-daemon | ZKE-001 to ZKE-007, PDM-005, PDM-009 | ZK-012 to ZK-019 | nothing committed but ZK-L2 (AUDIT.md A5): T11 lane |
| T12 | Security gate; anchor slot pin | part 1 running | B (gate), C (decoder, ledger), A (compiler refusal) | sherlock-e2e-stark-security, sherlock-e2e-call-binding, sherlock-e2e-wrap, sherlock-e2e-ledger-contract, sherlock-e2e-compact-compiler | SSG-001 to SSG-007, CLB-004, CLB-008, WRP-007, SLC-002, ECC-009 | SQG-014 to SQG-016; ANC-013 to ANC-015 | G3 legs not built (SQG-014): T12 lane |
| T13 | Quint lane coverage for every circuit change | first batch DONE | B (A for Mycroft Quint) | sherlock-e2e-stark-security, sherlock-e2e-compact-compiler | SSG-003 to SSG-005, NAG-006, NAG-007, NAG-011, ECC-011 | LPC-004 | batch 2 open; e5-wrap merge pending (LPC-004): T13 lanes |
| T14 | Call time on the integrated warm path | RUNNING | B | sherlock-e2e-prover-daemon, sherlock-e2e-observability | PDM-001 to PDM-004, OBS-007 | LPC-005; PRD-001 | rerun on the T25 keys (LPC-005) |
| T14b | Warm-path speedups: daemon, fixed-base MSM, GPU wrap | RUNNING | B, C | sherlock-e2e-prover-daemon, sherlock-e2e-gpu-proving, sherlock-e2e-wrap | PDM-001 to PDM-009, GPV-001 to GPV-007, WRP-011 | GPW-007; PRD | final timing open (LPC-005) |
| T15 | Interface freeze; land | waits | C (A for the suite table) | sherlock-e2e-recovery, sherlock-e2e-governance, sherlock-e2e-user-flow | FRC-011, EGV-008, BAT-012, HLP-006, EUF-013 | LPC-014, LPC-015, LPC-016 | every LEAD PICK for T15 in INDEX.md waits for the owner |
| T16 | Elastic demo and setup mode | (a) DONE, (b) PARTIAL, (c) BLOCKED, (d) PARTIAL, (e) PASS | B | sherlock-e2e-elastic-proving, sherlock-e2e-setup-mode, sherlock-e2e-box-fleet | ELA-001 to ELA-007, STM-001 to STM-019, BXF-001 to BXF-004, GPH-006, NAG-009, UKS-008 | LPC-006, LPC-007; SPL-011; ST, PF, SC | (c) needs the rekey; two run lengths; two-host join |
| T17 | Preview prototype | (a) DONE; (b), (c) wait | C (A for the flow) | sherlock-e2e-preview-deployment, sherlock-e2e-user-flow | PVW-001 to PVW-010, PVW-012 to PVW-013, EUF-001, EUF-009 to EUF-011 | LPC-008 | (b) and (c) open (LPC-008) |
| T18 | Compact to Sherlock: suite, compiler design, spike, factory run | (a), (b) DONE; (c) compile half DONE; (d) MEASURED, not committed | A | sherlock-e2e-compact-compiler, sherlock-e2e-compact-suite, sherlock-e2e-rsl-statement | ECC-001 to ECC-014, ECS-001 to ECS-002, ERS-001, ERS-013 | MYC (277); LPC-013, LPC-019; CF-001 to CF-014 | proving blocked on the R_SL chip set in the rekey; premise S1 (LPC-019) |
| T19 | Validium demo | waits T17 (b) | C | sherlock-e2e-preview-deployment | PVW-011 | LPC-009 | not started (LPC-009) |
| T20 | Batched Midnight writes | DONE, not merged | C | sherlock-e2e-batched-settlement | BAT-001 to BAT-012 | ANC-012; LDG-001 to LDG-008; LPC-010 | merge after T10q (LPC-010) |
| T21 | Prover GUI | running | C (B for the daemon bridge) | sherlock-e2e-observability, sherlock-e2e-prover-daemon | OBS-012, PDM-003, PDM-008 | LPC-011; PGUI (165 IDs, branch `prover-gui`) | beyond the stub open (LPC-011) |
| T22 | Governance of the on-chain representation | running | C | sherlock-e2e-governance, sherlock-e2e-batched-settlement | EGV-001 to EGV-010, BAT-010 to BAT-011 | GOV, ERT, ELD, CRR; LPC-012 | no Carbon verdict (LPC-012); era-check premises A1 to A3 untested (EGV-009) |
| T23 | Full Compact suite on the finished Sherlock, on Preview | RUNNING (Part A), Part B dry run DONE | A (Part B), C (Part A, settle), B (prove) | sherlock-e2e-compact-suite, sherlock-e2e-ledger-contract, sherlock-e2e-batched-settlement, sherlock-e2e-user-flow, sherlock-e2e-rsl-statement | ECS-001 to ECS-014, SLC-005 to SLC-011, BAT-004, EUF-013, ERS-006 to ERS-008, ERS-012 | STL-001 to STL-006; RSL-003 | waits on T25 and T12; statement binding gaps (ERS-006, ERS-007, ERS-012) |
| T24 | R_SL kits keccak256 and secp256k1 | OPEN, code BUILT | A (lowering, interface), B (AIRs) | sherlock-e2e-rsl-kits, sherlock-e2e-compact-compiler, sherlock-e2e-compact-suite | EKT-004, EKT-006 to EKT-007, EKT-009 to EKT-010, ECC-007, ECC-014, ECS-006, LFP-002 to LFP-003 | RSL-004, RSL-005; CF-013 | no MEASURED run; keccak lowering open (ECC-007) |
| T25 | The single rekey | waits its inputs | B (C for wrap rows and VK bytes) | sherlock-e2e-universal-keys, sherlock-e2e-wrap, sherlock-e2e-rsl-kits | UKS-001 to UKS-010, LTR-003, NAG-005, CLB-001, CLB-008, WRP-002 to WRP-004, WRP-012, EGV-005, EKT-001 | RKY-001 to RKY-005 | rekey list incomplete (AUDIT.md B1); the era-check row is missing from the rekey list (EGV-005, D-078) |

## 4. Hard rules

| Hard rule | Decision | EARS IDs that state it |
|---|---|---|
| Never k = 22; every circuit k at most 21 | D-001 | WRP-002 to WRP-003, SSG-002; CPR-001; RKY-003; MYC-CHIP-007 |
| Zero knowledge end to end; seeded proofs test-only; no plain witness to helpers | D-002 | ZKE-001 to ZKE-007, WRP-006, PDM-005, ERS-002, HLP-003 to HLP-004; GP-ZK; ZK-001 to ZK-019 |
| 100-bit security | D-003 | SSG-001, SSG-006 to SSG-007; RKY-005; SQG-001 to SQG-016 |
| One universal key set | D-004 | UKS-001, WRP-010, EUI-002 to EUI-003, EUF-009, ELA-002, GPH-001; CPR-003; RSL-001; RKY-001 |
| Ledger 9.1.0.1 verifier unchanged | D-005 | LVR-001 to LVR-003; CPR-004; MID-001 |
| Wrap VK at most 50,000 bytes | D-006 | WRP-004; RKY-004 |
| One BLS12-381 wrap; leaves and nodes small-field STARKs | D-007 | WRP-001, LFP-001, NAG-001, LTR-001 to LTR-002, SSG-002 |
| Every proof is a fixed-size shard | D-018 | LFP-005, LFP-008; CPR-002 |
| GP-1 to GP-4 never narrowed | D-008 | GPH-001 to GPH-008, EUI-001, EUI-010; CPR-005; GP-001 to GP-004 |
| Midnight writes batched, one root per checkpoint | D-009 | BAT-001 to BAT-003; LDG-003 |
| Case A only; case B through Veritee later | D-010 | HLP-005 to HLP-006, BXF-001, ERS-002; ZK-018 |
| K_MAX = 2 | D-011 | BAT-005, CLB-007; ANC-012 |
| MERGE-WORK 5,400 columns | D-012 | NAG-005; SQG-013; SHP-008; LPC-002 |
| The x_j lock form is canon | D-013 | CLB-005; ANC-002; ANC-017 |
| F-3 replaced by setup mode | D-014 | STM-009, STM-014, EUF-007; SPL-011 |
| Design open until the end-to-end prototype; hard rules owner-gated | D-015 | UKS-007, EGV-008, EUF-013; LPC-014 |

## 5. Gap list

Each line is a component or a row that has no complete requirement, no code or no LOOP row. Smaller open items stay in the gap column above.

1. **Mycroft modules transcript, seal, entry-binding, acceptance (sealed form, E-1 placement V).** the sealed form and the `bind_label` path are not built for R_SL; the prototype uses the run form (D-022). Owner lane: Mycroft with rekey-chips, after T15
2. **Mycroft modules claims, chunks, composition, keys, cost.** design only for R_SL (no `prog_d`, no chunked R_SL call). Owner lane: rekey-chips (T25) for `prog_d`; chunks after T15
3. **Mycroft spec text that the owner rulings and the built chip set have passed: MYC-BLT-007, MYC-SEAL-021 and MYC-SEAL-005, MYC-CHIP-001, MYC-CHIP-004, MYC-REF-052.** the Mycroft text is stale: MYC-BLT-007 routes `keccak256` and secp256k1 to a tier 3 kit (owner 2026-11-15: R_SL kits); MYC-SEAL-021 and MYC-SEAL-005 call the slot-15 rekey OWNER-pending (owner confirmed 2026-11-15); MYC-CHIP-001 lists rules that are not the built chip set; MYC-CHIP-004 and MYC-REF-052 as above. Owner lane: Mycroft lane edits `mycroft/spec/` (AUDIT.md notes M1, M2)
4. **Design documents FINAL-DESIGN.md 1.6 and ARCHITECTURE.md (lock form).** both documents still show `P2(DOM_LOCK)`; the `x_j` lock form is canon (owner 2026-11-14). Owner lane: lead (LPC-017)
5. **Compile command (`sherlock compile`).** no `sherlock` binary; the prototype runs `mycroft-suite` (D-085). Owner lane: Mycroft, after T15
6. **Kit SHA-256.** a program with SHA-256 and keccak256 has no R_SL proof; temporary named refusal (EKT-004, D-024). Owner lane: rekey-chips; the owner sees it at T15
7. **Kit Jubjub.** no interface file (EKT-010), no forged-row test (EKT-007), inputs refused (ECC-014). Owner lane: Mycroft with rekey-chips, row T24
8. **Tier 3 kit (sha512).** not built; a `sha512` call is refused. No suite call needs it. Owner lane: Mycroft, after T15
9. **Sherlock machine: rule families, `machine_version` (the universal interpreter).** the UIREQ retention map does not exist (EUI-009), and R_SL programs carry no `machine_version` yet (EUI-001). Owner lane: slice A (map), rekey-chips lane, T25
10. **Interpreted-family leaf chips: BUS, RANGE, ALU, DEC, HASH, CORE, STACKS, MEM window, MEMORY BACKEND, PAGE/ROM, ZmLogUp variant, VALUE DIGEST, BOUNDARY COMMITMENT, INIT.** 5 subset rules and 2 value rules built of 120 (EUI-010). Owner lane: slice B, modular-leaf groups after V; no LOOP row
11. **Run statement and its binding (`prog_d`, `input_d`, `outcome_d`, FP-1 words).** (HIGH) no `prog_d`, no INIT and no FP-1 binding for R_SL (ERS-006, ERS-007, ERS-012). Owner lane: rekey-chips (slice B), before T25
12. **End-to-end call flow.** no run joins all stages (EUF-001); no clean-box run (EUF-011). Owner lane: lead, rows T17 (c), T23
13. **Leaf: app proof and leaf proof of one shard.** (HIGH) R_SL statement, `prog_d` and `input_d` not bound (LFP-010 to LFP-012, HIGH); 137 of 142 calls have no probe (LFP-002); chip set 3 has no named shape vector (LFP-005); both hash kits in one program have no route (LFP-003, D-024): rekey-chips (lane rsl-bind for the bindings)
14. **GP-3 hook, general verifier (five subkits), foreign registry.** X8 in-circuit Compact verifier not built (LPC-003): row T8c (no LOOP row), slice B
15. **GP-4 hook (any shape), MERGE-WORK.** the complete kernel width with STACKS and VALUE DIGEST is 5,121 DERIVED, not MEASURED (4,988 MEASURED without them; openvm `rekey-chips` dcf1b4c `frozen.rs:166`); the earlier 5,417 is superseded (NAG-005, LPC-002): row T8c
16. **Universal key set, `key_root`, `keys_d`, the single rekey.** rekey list lacks 6 rows: FP-3, slot 15, range checks, R_SL statement bindings, R_SL key leaf index, era-check words (UKS-010); `KEYDAG_IDX = 3` collides with the planned R_SL id 3 (UKS-003); no integrated keygen tree (UKS-009): lanes fp3, t12-slot15, rekey-chips, t22-governance, T25
17. **Resident prover daemon and warm path.** the blinding-slot pool of OD-13 and PRD-002 is not built (D-040, TO BUILD, owner lane T11); no recorded check of PRD-003, PRD-004 (PDM-007); final timing open (LPC-005): rows T14, T14b
18. **LEDGER CONTRACT: SherlockLedger, `acceptRun`, the argument map.** live settle is off until the leaf statement binds `prog_d`, `input_d`, `outcome_d` (SLC-012): slices A, B; MAP.md and `accept_args.py` staged, not committed. Owner lane: rsl-bind (slices A, B), t23-args (slice C)
19. **Batched settlement: `acceptCollection`, claims root, chunk cap; claim consumption on the ledger.** the `L` collection form, the claims tree proof and the SHK_FCLAIM tag have no code (BAT-009, BAT-012): slice B, row T8c. The registry update entry, the N-8 history rule and leaf-list storage have no LOOP row (proposed T20b). Owner lane: slice C with the lead
20. **Governance: sealed contracts, era table, Fenrir operator verbs.** the era-check words are not in the T25 rekey list and premises A1 to A3 are untested (EGV-005, EGV-009); no sealed deploy tool (EGV-001, EGV-002); no Carbon verdict (LPC-012). Owner lane: row T22, slice B (UKS) for the rekey list
21. **LOOP row T6.** groups after V (115 of 120 rules) have no LOOP row (EUI-010): slice B
22. **LOOP row T8c.** LOOP.md names T8c in row T8b and has no row for it (AUDIT.md M3). Owner lane: lead adds the row; slice B builds
