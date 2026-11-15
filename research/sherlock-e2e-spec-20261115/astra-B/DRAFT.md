# Slice B: independent proving draft

This draft proposes requirements. It does not report a new test run or a completed integration.
No build, prover, test, network operation or git command ran for this draft.
The draft uses no Fable draft or end-to-end bus content.

## Source locators and interpretation

The path prefixes below form part of each `file:line` locator in this draft.
Paths without a prefix are relative to the current worktree.

| Prefix | Exact directory or file |
|---|---|
| `LOOP.md` | `/home/hoskinson/Projects/Moriarty/research/sherlock-tested-design-20261113/LOOP.md` |
| `CHECKLIST.md` | `/home/hoskinson/.cache/lead/CHECKLIST-20261115.md` |
| `AUDIT.md` | `/home/hoskinson/Projects/Moriarty-wt/spec-audit/research/sherlock-spec-audit-20261115/AUDIT.md` |
| `COMP/` | `/home/hoskinson/Projects/Moriarty-wt/spec-audit/openspec/changes/sherlock-completion-20261115/` |
| `FINAL/` | `openspec/changes/sherlock-processor-final-20261111/` |
| `MOD/` | `openspec/changes/sherlock-modular-leaf-20261111/` |
| `VM/` | `openspec/changes/sherlock-vm-consensus-20261107/` |
| `DESIGN/` | `research/sherlock-final-design-20261111/` |
| `MAIN-ARCH` | `/home/hoskinson/Projects/Moriarty/research/sherlock-final-design-20261111/ARCHITECTURE.md` |
| `WIKI/` | `/home/hoskinson/Projects/Moriarty/llm-wiki/wiki/` |
| `OVM/` | `/home/hoskinson/.cache/m7-wrap/src/openvm-rekey-chips/` |
| `SB/` | `/home/hoskinson/.cache/m7-wrap/src/stark-backend-tested/` |
| `FP3/` | `/home/hoskinson/.cache/m7-wrap/src/openvm-fp3/` |
| `SLOT/` | `/home/hoskinson/.cache/m7-wrap/src/openvm-t12-slot15/` |
| `ZKOVM/` | `/home/hoskinson/.cache/m7-wrap/src/openvm-t11zk/` |
| `T16OVM/` | `/home/hoskinson/.cache/m7-wrap/src/openvm-t16/` |
| `T11/` | `/home/hoskinson/Projects/Moriarty-wt/t11-zk/` |
| `T12/` | `/home/hoskinson/Projects/Moriarty-wt/t12-gate/` |
| `T12SLOT/` | `/home/hoskinson/Projects/Moriarty-wt/t12-slot15/` |
| `T16/` | `/home/hoskinson/Projects/Moriarty-wt/t16-elastic/` |
| `SETUP.md` | `T16/research/sherlock-t16-elastic-20261114/SETUP-SPEC.md` |
| `ZK-RESULTS.md` | `T11/research/sherlock-t11-zk-20261115/RESULTS.md` |
| `RUNBOOK.md` | `T12/research/sherlock-tested-design-20261113/t12/RUNBOOK.md` |
| `LIST.md` | `/home/hoskinson/.cache/rekey-t11/LIST.md` |
| `SECP.md` | `/home/hoskinson/.cache/rekey-chips/SECP-INTERFACE.md` |
| `METHOD/` | `/home/hoskinson/Projects/software-factory/skills/specs/spec-architect/references/` |

`REUSE` preserves an existing requirement ID. It does not create another authoritative copy.
`REUSE, AMEND` identifies wording that the final change must explicitly supersede.
`BOP` is a proposed prefix for requirements with no located ID. The integrator must check its allocation.
Verification names a check to use. It does not assert that the check passed on the final keys.
`NEW:` identifies missing acceptance work. `NO EVIDENCE FOUND` identifies an unsupported implementation or completion claim.

The user names OVM commit `dcf1b4c`. This draft cites source lines read from that worktree and does not independently verify HEAD.
The single integration target remains the full chain under one key set, as stated in `LOOP.md:5` and `LOOP.md:47`.

## 1. Capability list

| Capability | ID prefixes | Scope | LOOP rows | Components |
|---|---|---|---|---|
| `sherlock-rsl-chipset` | RSL, SPL, CPR, BOP | Prove admitted app traces through a universal leaf interface. | T3, T5, T6, T7, T24, T25 | R_SL statement boundary, app keys, leaf keys, fixed shapes, required AIRs, SHA-256, Keccak, jubjub, secp256k1, Poseidon, ALU |
| `sherlock-proof-composition` | GP, NODE, SHP, LPC, CPR, BOP | Authenticate child proofs and compose runs without reducing GP scope. | T7, T7c, T8, T8b, T10q, T13, T20 | N1, N2, N3, key_root, keys_d, PCD, SET, SEG, FV, X8, X12, X14, MERGE-WORK, NW, NW2, STACKS, VALUE DIGEST, continuation |
| `sherlock-l-and-wrap` | LW, BOP | Bind the root and field log, then verify L in the Fr-native layer. | T3, T7c, T9, T10, T10m, T25 | L, L_Fr, tapes, typed MEM_BUS, public-value packing, transcript, DAG commitment, private handoff to wrap |
| `sherlock-anchor-chunks` | ANC | Bind effect lanes, chunk sequence and the slot-15 anchor. | T9, T10q, T11, T12, T25 | FP-3, forward claim chain, x_j, ordinary/chunk mode, K_MAX, anchor templates |
| `sherlock-rekey` | RKY, SQG, DG, CPR, BOP | Produce and qualify one integrated universal key set. | T3, T10m, T10q, T11, T12, T24, T25, T15 | Single rekey manifest, key provenance, shape bounds, 100-bit gate, wrap rows and VK bytes, G1 through G5 |
| `sherlock-elastic-proving` | ST, PF, SC, SPL, LPC, BOP | Profile owned boxes and schedule fixed shards under unchanged keys. | T14, T14b, T16, T23 | Setup mode, machine profile, CPU/GPU memory, concurrency, partition, resume, muninn/huginn/odin, heavy-slot, gpu.lock, box bus |
| `sherlock-zero-knowledge` | ZK | Keep intermediate proofs private and publish a freshly blinded wrap. | T11, T12, T16, T17, T23 | Deployment salt, run salt, proof blinders, seed guards, one-use slots, case A, case B refusal, ZK-L2 |
| `sherlock-gpu-proving` | GPW, PRD, LPC | Run supported stages on CPU or GPU with measured fallback and cache behavior. | T14, T14b, T16, T25 | CUDA STARK path, GPU L_Fr work, wrap MSM/NTT, resident daemon, key/SRS cache, overlap and eviction, final-key timing |

No slice B component lacks a proposed capability above. This is coverage of the draft, not implementation coverage.
Slice A retains language semantics, kit lowering and the full R_SL statement schema.
Slice C retains the wrap implementation, ledger contract, settlement, registry epochs and deployment.
The shared boundaries appear below. This allocation follows `research/sherlock-e2e-spec-20261115/BRIEF.md:18` through `:20`.

## 2. EARS draft

### sherlock-rsl-chipset

#### RSL-001 — REUSE: universal leaf key

When the prover proves a suite call's R_SL app proof, the universal leaf key shall produce the universal N1 pre-hash.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:4`.
Scenario: GIVEN two suite programs, WHEN their leaf proofs are made, THEN both N1 pre-hashes equal the universal value.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:2008`, `rsl_probe`. NEW: repeat after every T24 kit change and across the full admitted suite.

#### RSL-002 — REUSE: AIR bucket

The R_SL chip set shall give each app 6 or 9 AIRs with FP-1.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:15`.
Scenario: GIVEN each supported kit combination, WHEN its app key is built, THEN its AIR count meets the bucket and RSL-001 holds.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:2008`, `rsl_probe`. NEW: all kit-combination coverage.

#### RSL-003 — REUSE: app proofs

When the suite prover runs the 142 R_SL suite calls, it shall produce one stock-verifier-accepted app proof per call.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:26`.
Scenario: GIVEN a suite call, WHEN execution matches native words, THEN proof acceptance is checked separately.
Verification: `OVM/crates/sherlock-leaf/examples/rsl_suite.rs`. NEW: retain per-call proving and verifier records on the integrated keys.

#### RSL-004 — REUSE: Keccak proof admission

Where the machine version includes Keccak, the R_SL chip set shall prove `keccak256` inside the 6-or-9 AIR bucket.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:37`.
Scenario: GIVEN the s02 Keccak call, WHEN its proof is verified, THEN native words match and RSL-001 holds.
Verification: `OVM/crates/sherlock-leaf/tests/rsl_keccak.rs:50`, `rsl_keccak_call_proves`, and `:172`, `rsl_keccak_wrong_request_key_refused`. NEW: final universal-leaf probe.

#### RSL-005 — REUSE: secp256k1 proof admission

Where the machine version includes secp256k1, the chip set shall prove its admitted operations inside the 6-or-9 AIR bucket.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:48`. The operation interface is `SECP.md:50` through `:75`.
Scenario: GIVEN valid and invalid ECDSA inputs, WHEN the chip set proves them, THEN it accepts valid semantics and refuses invalid witnesses.
Verification: `OVM/crates/sherlock-leaf/tests/rsl_secp.rs:195`, `rsl_secp_ecdsa_proves`, and `:253`, `rsl_secp_word_soundness`. NEW: final universal-leaf probe.

#### RSL-006 — REUSE: jubjub constraints

When a call uses jubjub, the chip set shall check curve membership and `8Q = P` through micro-ops without another AIR.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:59`.
Scenario: GIVEN a point outside the subgroup, WHEN it enters a jubjub operation, THEN the constraint checker refuses it.
Verification: `OVM/crates/sherlock-leaf/tests/rsl_jubjub.rs:69`, `rsl_jubjub_leaf_proof`, and `:119`, `rsl_jubjub_s01_ec_call`. NEW: explicit final-key subgroup forgery run.

#### RSL-007 — REUSE: SHA-256 constraints

When a call uses `persistentHash`, the chip set shall prove it through SHA_IN, SHA_REQ and the OpenVM SHA-256 block hasher.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:70`.
Scenario: GIVEN a forged SHA request, WHEN its app proof is checked, THEN the checker refuses the request.
Verification: `OVM/crates/sherlock-leaf/tests/rsl_sha.rs:37`, `rsl_sha_calls_prove`, and `:51`, `rsl_sha_forged_rows_refused`.

#### RSL-008 — REUSE: canonical inputs

If an ALU or Poseidon input is not canonical for its declared field, then the chip set shall refuse the row.
Source: `COMP/specs/sherlock-rsl-chipset/spec.md:81`. Field-specific secp moduli are specified in `SECP.md:31` and `:101`.
Scenario: GIVEN a word `m + x` in place of `x`, WHEN the row is checked, THEN canonical range constraints fail.
Verification: `OVM/crates/sherlock-leaf/tests/rsl_secp.rs:253`, `rsl_secp_word_soundness`. NEW: forged noncanonical ALU and Poseidon inputs for every admitted field path.

#### SPL-001 — REUSE, AMEND: complete layer stack

The prover shall use BabyBear STARK leaves, nodes and L, an Fr-native L_Fr STARK, and one BLS12-381 PLONK-KZG wrap.
Source: `FINAL/specs/sherlock-shard-plan/spec.md:4`, amended to include `FINAL/specs/sherlock-l-and-wrap/spec.md:155` and OD-24.
Scenario: GIVEN a full-tree run, WHEN it reaches settlement, THEN exactly one BLS12-381 wrap contains the private L_Fr proof.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:891`, `chain`, then the ledger run in `LOOP.md:31`. NEW: repeat on T25 keys.

#### SPL-004 — REUSE: program keys and universal keys

The prover shall bind per-program keys under `key_root` and use the same recursive, L, L_Fr and wrap keys across programs.
Source: `FINAL/specs/sherlock-shard-plan/spec.md:29`, `DESIGN/TERMINOLOGY.md:36`, `LOOP.md:25`.
Scenario: GIVEN two different programs, WHEN their proof chains are built, THEN the universal key bytes match.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:615`, universal L equality assertion. NEW: serialized equality across all final universal keys.

#### CPR-002 — REUSE: fixed shards

The prover shall increase work by adding fixed-size shards rather than increasing a circuit's size.
Source: `FINAL/specs/sherlock-general-purpose/spec.md:28`, `FINAL/specs/sherlock-shard-plan/spec.md:43`.
Scenario: GIVEN a longer admitted run, WHEN it exceeds one shard, THEN shard count grows and the keys remain unchanged.
Verification: NEW: T16 1x, 2x, 4x and 8x real runs on the R_SL-integrated keys, as required by LPC-007.

#### BOP-001 — NEW ID: required proof-shape AIRs

If a proof omits a required AIR, then each recursive verifier shall refuse the proof.
Source: `LOOP.md:20`. This adds an explicit cross-layer acceptance rule to the T3 work.
Scenario: GIVEN a proof without its public-value or proof-shape AIR, WHEN N1, N3 or L checks it, THEN verification fails.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:1250`, `omit`, and the T3 run recorded at `LOOP.md:20`. NEW: trigger the remaining parent constraints identified by LPC-018.

### sherlock-proof-composition

#### GP-001 — REUSE: universal VM

The prover shall prove every admitted Sherlock program, input, outcome and run length below `2^64` under one universal key set.
Source: `VM/specs/sherlock-general-purpose/spec.md:4`. “Admitted” means the Sherlock language, not a reduced benchmark subset.
Scenario: GIVEN Halted, Rejected and Exhausted runs, WHEN they are proved, THEN each terminal outcome verifies under that key set.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:1108`, `cases_mode`. NEW: full interpreter coverage, committed-memory runs and length-bound checks.

#### GP-002 — REUSE: cross-run composition

When a run consumes another run's claim, the recursion shall verify that claim in circuit and bind the dependency.
Source: `VM/specs/sherlock-general-purpose/spec.md:18`.
Scenario: GIVEN two runs with one consumed claim, WHEN the claim changes, THEN the composed proof fails.
Verification: `OVM/crates/sherlock-wrap-export/examples/t8_gp.rs`, GP-2 run recorded at `LOOP.md:27`. NEW: L over the real sealed SET root.

#### GP-003 — REUSE: foreign proofs

The recursion shall verify registered Compact and other admitted PLONK/halo2 proofs in circuit through an authenticated key and statement adapter.
Source: `VM/specs/sherlock-general-purpose/spec.md:32`.
Scenario: GIVEN a foreign proof with a substituted registry key, WHEN FV verification runs, THEN no foreign claim is emitted.
Verification: NEW: real X8 verifier for `compact-zkir3-kzg-bls12381-blake2b/1`, plus X14 proof-stream, point, transcript and completion mutants. Existing partial status: `LOOP.md:27`.

#### GP-004 — REUSE: PCD

The recursion shall compose chains, trees and DAGs of any depth with one key set and constant final proof size.
Source: `VM/specs/sherlock-general-purpose/spec.md:42`. Nodes retain arity of at least two plus new work.
Scenario: GIVEN a chain, balanced tree and shared-child DAG, WHEN each is proved, THEN final proof size does not grow with depth.
Verification: `OVM/crates/sherlock-wrap-export/examples/t8_gp.rs`, X7 run recorded at `LOOP.md:27`. NEW: full DAG and depth-extension acceptance cases.

#### NODE-001 — REUSE: adjacent segment merge

When a node merges segments, it shall require equal `prog_d`, adjacent height intervals and matching left-post and right-pre state digests.
Source: `MOD/specs/sherlock-node-integration/spec.md:4`.
Scenario: GIVEN a valid pair of segments, WHEN one connecting state digest changes, THEN the merge proof fails.
Verification: `OVM/crates/sherlock-leaf/tests/quint_regressions.rs` and `OVM/crates/sherlock-leaf/tests/qr/afk.rs`. NEW: rerun the relevant T10q cases on T25 keys.

#### NODE-002 — REUSE: bounded heights

The node shall range-check height limbs and reject overflow of `h_pre + steps` beyond the `u64` range.
Source: `MOD/specs/sherlock-node-integration/spec.md:15`.
Scenario: GIVEN an out-of-range high limb, WHEN MERGE-WORK or its parent verifies it, THEN verification fails.
Verification: `OVM/crates/sherlock-leaf/tests/qr/mw.rs`, MW-F01 regression named in `LOOP.md:32`.

#### NODE-004 — REUSE: authenticated child routing

The node shall bind each enabled child's key, statement tag and lanes through one selector and an authenticated key commitment.
Source: `MOD/specs/sherlock-node-integration/spec.md:37`.
Scenario: GIVEN an unlisted leaf key, WHEN N1 checks its key path, THEN the node refuses it.
Verification: `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:367`, `off_menu`. Path C: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:1771` onward.

#### SHP-006 — REUSE: one inline kernel relation

The MERGE-WORK kernel shall use the leaf kit AIR and trace implementations for its new owner steps.
Source: `MOD/specs/sherlock-proof-shape/spec.md:55`.
Scenario: GIVEN identical machine steps in a leaf and MERGE-WORK, WHEN both constraints run, THEN both enforce the same transition relation.
Verification: NEW: `graph_inline_kernel_separate_encoding` check with real STACKS and VALUE DIGEST work, plus positive and forged transitions.

#### LPC-002 — REUSE: MERGE-WORK bound

When STACKS and VALUE DIGEST enter the inline kernel, the MERGE-WORK key shall have at most 5,400 columns.
Source: `COMP/specs/sherlock-loop-completion/spec.md:15`, `LOOP.md:106`.
Scenario: GIVEN the complete inline kernel, WHEN its key is generated, THEN its measured width is at most 5,400.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs`, shape mode, and `OVM/crates/sherlock-wrap-export/examples/fp8_d3.rs`. NEW: complete-kernel measurement.

#### SHP-009 — REUSE: parent over MERGE-WORK

The recursion shall authenticate MERGE-WORK, NW, NW2 and N3 through their correct rows in the seven-row `keys_d` table.
Source: `MOD/specs/sherlock-proof-shape/spec.md:84`, confirmed at `LOOP.md:106`.
Scenario: GIVEN a valid MERGE-WORK child in the wrong row, WHEN its parent verifies it, THEN verification fails.
Verification: `OVM/crates/sherlock-wrap-export/examples/t8_gp.rs`, X7 run at `LOOP.md:27`. NEW: final-key row-substitution run.

#### LPC-003 — REUSE: complete GP hooks

When T8c closes, the prover shall include the X8 Compact verifier and L verification over a SET root on real proofs.
Source: `COMP/specs/sherlock-loop-completion/spec.md:26`.
Scenario: GIVEN an admitted foreign claim in a SET, WHEN L opens the final root, THEN the full proof verifies and a forged claim fails.
Verification: NEW: X8-to-SET-to-L integration run. `LOOP.md:27` records these hooks as open.

#### CPR-005 — REUSE: no reduced GP target

If a proof gate fails, then the project shall retain the full GP-1 through GP-4 acceptance requirements.
Source: `FINAL/specs/sherlock-general-purpose/spec.md:49`, `FINAL/specs/sherlock-derived-gates/spec.md:70`.
Scenario: GIVEN an oversized MERGE-WORK kernel, WHEN the gate fails, THEN the task remains open and its GP-4 test remains required.
Verification: NEW: requirement-to-gate coverage review that detects deleted outcomes, foreign-proof paths, DAG cases and length cases.

#### BOP-006 — NEW ID: checkpoint root handoff

When operator jobs settle a checkpoint, the prover shall supply one authenticated collection root for the batched Midnight write.
Source: `LOOP.md:42`, `research/sherlock-e2e-spec-20261115/BRIEF.md:54`.
Scenario: GIVEN several claims in one checkpoint, WHEN B hands the result to C, THEN one collection root commits to those claims.
Verification: NEW: real SET-to-L-to-acceptCollection run with membership checks and one-root-per-checkpoint transaction accounting.

### sherlock-l-and-wrap

#### LW-001 — REUSE: root and interface binding

The L circuit shall verify the full-tree root or path-C child and bind its statement, field log, anchor, DAG commitment and call vector.
Source: `FINAL/specs/sherlock-l-and-wrap/spec.md:4`.
Scenario: GIVEN an honest root, WHEN a call-vector word or log entry changes, THEN the L proof fails.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:891`, full chain, and path-C cases at `:1771`. NEW: FP-3-integrated mutations on final keys.

#### LW-002 — REUSE: key-driven verifier schedule

The L_Fr circuit shall verify L with the frozen key schedule and transcript order rather than proof-selected structure.
Source: `FINAL/specs/sherlock-l-and-wrap/spec.md:15`.
Scenario: GIVEN a proof with reordered opening data, WHEN L_Fr executes its tape, THEN it refuses the proof.
Verification: `OVM/crates/sherlock-lfr/tests/i5_l_transcript.rs:60`, `i5_l_transcript_on_tape`, and `OVM/crates/sherlock-lfr/tests/i9_whir.rs`. NEW: final-key schedule mutation run.

#### LW-018 — REUSE: Fr-native bridge

The L_Fr circuit shall emulate L's BabyBear arithmetic in Fr and expose the L key pre-hash followed by public values packed eight per Fr word.
Source: `FINAL/specs/sherlock-l-and-wrap/spec.md:155`.
Scenario: GIVEN a noncanonical reduction or changed packed public value, WHEN L_Fr checks it, THEN verification fails.
Verification: `OVM/crates/sherlock-lfr/tests/i2_alu.rs`, `i4_tape.rs`, `j1_public.rs` and `i10_gate.rs`. NEW: final-key real proof through wrap.

#### LW-020 — REUSE: typed memory bus

The L_Fr circuit shall include `TY_BB = 0` or `TY_FR = 1` as the sixth element of every MEM_BUS message.
Source: `COMP/specs/sherlock-l-and-wrap/spec.md:4`.
Scenario: GIVEN an Fr writer and a BabyBear reader at one address, WHEN the bus argument is checked, THEN it fails.
Verification: `OVM/crates/sherlock-lfr/tests/t13_lfr_poseidon.rs`, F3a and F3c cases identified in `COMP/specs/sherlock-l-and-wrap/spec.md:12`.

#### LW-021 — REUSE: tape type refusal

If a tape contains an ill-typed operand, then the L_Fr host shall refuse it before witness generation.
Source: `COMP/specs/sherlock-l-and-wrap/spec.md:15`.
Scenario: GIVEN an Fr cell passed to BbAlu, WHEN `Machine::try_new` runs, THEN it returns an error.
Verification: NEW: explicit host-refusal acceptance test around `OVM/crates/sherlock-lfr/src/machine.rs:1113`, including the C-A2c cases.

#### BOP-002 — NEW ID: merged log binding

When L verifies multiple segments, it shall bind the flattened field log to the authenticated TAG_NODE merge tree.
Source: `LOOP.md:23`, `FINAL/specs/sherlock-l-and-wrap/spec.md:4`.
Scenario: GIVEN two segment logs, WHEN their order or one merge child changes, THEN L rejects the flattened log.
Verification: `OVM/crates/sherlock-lfr/src/iface.rs`, `enc2_node_merge`. NEW: real multi-segment L proof with a forged merge, not only reference vectors.

### sherlock-anchor-chunks

#### ANC-013 — REUSE: slot-15 key constant

The wrap circuit shall pin anchor-read lane 6 and chunk-write lane 15 to top-level slot 15.
Source: `COMP/specs/sherlock-anchor-chunks/spec.md:4`, `WIKI/log.md:4107`.
Scenario: GIVEN a valid call, WHEN a prover substitutes slot 0, THEN the template constraint refuses it.
Verification: `SLOT/crates/sherlock-lfr/src/iface.rs`, `t12_anchor_slot_15`, and `T12/tools/sherlock-wrap/tests/t12_anchor_slot.rs`. NEW: final-key slot-15 deployment parity with slice C.

#### ANC-016 — REUSE: ordinary-call anchor-write refusal

If an ordinary call carries the anchor-write template, then the wrap circuit shall refuse the witness.
Source: `COMP/specs/sherlock-anchor-chunks/spec.md:37`.
Scenario: GIVEN mode 0, WHEN the anchor-write template occupies lanes 11 through 19 and 21, THEN the witness fails.
Verification: NT-1 run at openvm `e3b407d` and Moriarty `bb9dc9ae0`, recorded in `COMP/specs/sherlock-anchor-chunks/spec.md:45`. NEW: rerun at slot 15.

#### ANC-017 — REUSE: canonical extended lock

When a chunk reads or writes a lock, L_Fr shall derive `x_j` from `c`, `run_tag`, `k`, `d_in`, `d_out` and `j`.
The canonical expression is `x_j = sponge(U_LOCK=8; c, pack(run_tag), k, d_in, d_out, j)`.
The outer lock is `A_j = PoseidonFr_4(SHK_LOCK, x_j)`.
Source: `COMP/specs/sherlock-anchor-chunks/spec.md:48`, `AUDIT.md:101`, `LIST.md:5`, `WIKI/log.md:4107`.
Scenario: GIVEN equal run tags and different inputs, WHEN a chunk is spliced between runs, THEN the lock check fails.
Verification: NT-2 and NT-3 at openvm `6a00df3` and `e3b407d`, recorded in `COMP/specs/sherlock-anchor-chunks/spec.md:56`. NEW: integrated final-key replay.

#### ANC-018 — REUSE: FP-3 effect binding

The L circuit shall bind each call-vector effect lane to the guest effect word at that lane.
Source: `COMP/specs/sherlock-anchor-chunks/spec.md:59`.
Scenario: GIVEN an ordinary call, WHEN an anchor write shifts one lane, THEN L refuses the forged effect vector.
Verification: NEW: `nt1_residual_shifted_write_is_fp3` acceptance run on the integrated L circuit. Existing implementation entry: `FP3/crates/sherlock-lfr/src/iface_l.rs:777`.

#### ANC-019 — REUSE: forward claim chain

When a run has k chunks, the L circuit shall bind k to the actual slice count through the forward claim chain.
Source: `COMP/specs/sherlock-anchor-chunks/spec.md:70`.
Scenario: GIVEN a two-chunk run, WHEN k changes to one or the final chain tail changes, THEN L refuses it.
Verification: `FP3/crates/sherlock-lfr/src/fp3.rs:602`, `chain_and_record_shapes`. NEW: real-proof truncation and terminal-tail mutants.

#### ANC-012 — REUSE, AMEND: capped chunk fallback

If a proposed fallback needs more than two chunks, then the prover shall refuse that fallback.
Source: existing ANC-012 identified in `AUDIT.md:118`, owner acceptance at `LOOP.md:106`.
Scenario: GIVEN `k = 3`, WHEN fallback planning runs, THEN it reports a capacity refusal before proving.
Verification: NEW: host and circuit boundary cases for `k = 1`, `2` and `3` on the final path. Do not infer a global GP run-length cap.

ANC-014 and ANC-015 stay single requirements owned by slices C and A. They require slot-15 deployment and compiler collision refusal.
Sources: `COMP/specs/sherlock-anchor-chunks/spec.md:15` and `:26`.

### sherlock-rekey

#### RKY-001 — REUSE: one manifest, one rekey

When all rekey inputs are integrated, the rekey runner shall regenerate L, L_Fr and wrap keys once from the complete rekey list.
Source: `COMP/specs/sherlock-rekey/spec.md:4`, `LOOP.md:47`.
Scenario: GIVEN NT fixes, typed MEM_BUS, kits, FP-3 and slot 15, WHEN keygen starts, THEN the manifest identifies every input revision.
Verification: NEW: keygen receipt with manifest hash, source revisions, artifact hashes and compatibility checks. A late change invalidates that receipt and records the repeat.

#### RKY-002 — REUSE, AMEND: committed input guard

If a required manifest row lacks an integrated commit, then the rekey runner shall refuse to start.
Source: `COMP/specs/sherlock-rekey/spec.md:15`. “Integrated” strengthens the audit's branch-commit test to cover T25's integration target at `LOOP.md:47`.
Scenario: GIVEN a committed slot fix outside the keygen tree, WHEN preflight checks the manifest, THEN it refuses keygen.
Verification: NEW: manifest completeness and integration checks with absent-row and wrong-revision cases.

#### RKY-003 — REUSE: row budget

When the rekey completes, the runner shall measure at most 1,782,573 wrap rows at `k = 21`.
Source: `COMP/specs/sherlock-rekey/spec.md:26`.
Scenario: GIVEN a larger wrap, WHEN qualification runs, THEN it fails without increasing k to 22.
Verification: `tools/sherlock-wrap/src/bin/wrap.rs:32` and `:49` provide guards. NEW: final-key row-count receipt.

#### RKY-004 — REUSE: ledger-readable VK size

When the rekey completes, the runner shall measure a serialized wrap VK of at most 50,000 bytes.
Source: `COMP/specs/sherlock-rekey/spec.md:37`.
Scenario: GIVEN the final VK, WHEN the unchanged ledger 9.1.0.1 verifier reads it, THEN it parses and verifies the honest proof.
Verification: NEW: final VK byte count and muninn ledger-check run. Earlier integrated run: `LOOP.md:31`.

#### RKY-005 — REUSE: security margins

When the rekey completes, D3 shall report at least 100 bits for every term and at least 101 bits for every grindable term.
Source: `COMP/specs/sherlock-rekey/spec.md:48`.
Scenario: GIVEN a grindable term at 100.9 bits, WHEN D3 evaluates the key, THEN qualification fails.
Verification: `OVM/crates/sherlock-wrap-export/examples/fp8_d3.rs`. NEW: complete final-key and shape-bound report, including MERGE-WORK.

#### SQG-014 — REUSE: transcript parity

When G3 runs, the gate shall compare CPU, GPU, stark-backend verifier, L-in-L_Fr and wrap transcript vectors on the rekeyed keys.
Source: `COMP/specs/sherlock-security-gate/spec.md:4`.
Scenario: GIVEN one changed challenge in one implementation, WHEN the gate compares vectors, THEN it fails and names that implementation.
Verification: `OVM/crates/sherlock-lfr/tests/enc_air_vectors.rs`, `tools/sherlock-wrap/tests/i1_transcript.rs` and `i9_enc.rs`. NEW: missing prover/verifier legs and real-proof transcript logs in `RUNBOOK.md:61` through `:66`.

#### SQG-015 — REUSE: final-key gate

When the rekey completes, the security gate shall check G1, G2, G4 including L_Fr, FP-8 and ledger 9.1.0.1 acceptance.
Source: `COMP/specs/sherlock-security-gate/spec.md:15`.
Scenario: GIVEN a passing old key and a new key, WHEN qualification runs, THEN only evidence for the new key counts.
Verification: `RUNBOOK.md:68` onward and `OVM/crates/sherlock-lfr/tests/g1_frozen.rs:62`, `g1_refuses_changed_parameters`. NEW: exact G4 command and final-key receipts.

#### SQG-001 — REUSE: complete security claim

The gate shall claim proven 100-bit security only after G1 through G4 pass with the G5 theorem-to-configuration mapping.
Source: `FINAL/specs/sherlock-security-gate/spec.md:4` and `:79`.
Scenario: GIVEN a passing D3 report without transcript parity, WHEN status is published, THEN it reports calculator figures and an open gate.
Verification: NEW: evidence checker covering G1 through G5, explicit theorem hypotheses and the integrated key hashes.

#### SQG-016 — REUSE: no pass without evidence

If a required runbook step is unverified, then the security gate shall report that the gate has not passed.
Source: `COMP/specs/sherlock-security-gate/spec.md:26`.
Scenario: GIVEN a missing log, WHEN the gate report is formed, THEN the report cannot say PASS.
Verification: NEW: missing-log and stale-key tests for the gate report.

#### SPL-009 — REUSE: prototype freeze

When security qualification and FP-8 pass, the prover shall bind all freeze items to the universal key manifest.
Source: `FINAL/specs/sherlock-shard-plan/spec.md:64`, `LOOP.md:101`.
Scenario: GIVEN changed hash domains, layout or shape parameters, WHEN a cached key is requested, THEN the old manifest cannot qualify it.
Verification: NEW: manifest drift tests covering log encoding, capacities, domains, templates, public layout, query vectors and key hashes. Owner presentation remains T15.

#### CPR-001 — REUSE: circuit size ceiling

The prover shall refuse any BLS12-381 circuit with k greater than 21.
Source: `FINAL/specs/sherlock-general-purpose/spec.md:21` and `research/sherlock-e2e-spec-20261115/BRIEF.md:54`.
Scenario: GIVEN `k = 22`, WHEN any production circuit entry point starts, THEN it refuses before proving.
Verification: `tools/sherlock-wrap/src/bin/wrap.rs:32` provides one guard. NEW: all-entry-point refusal coverage.

#### CPR-004 — REUSE: unchanged ledger verifier

The proving path shall produce proofs accepted by the unchanged Midnight ledger 9.1.0.1 verifier.
Source: `FINAL/specs/sherlock-general-purpose/spec.md:42`.
Scenario: GIVEN the final wrap VK and proof, WHEN stock ledger verification runs, THEN the honest proof passes without a verifier patch.
Verification: NEW: final-key ledger acceptance and tamper refusal receipt on muninn, following the earlier run recorded at `LOOP.md:31`.

#### BOP-005 — NEW ID: design change authority

If a proposed change alters a hard rule, then the project shall require an owner ruling before adopting that change.
While the end-to-end prototype is incomplete, the project shall permit other parameter and key changes with repeated affected gates.
Source: `LOOP.md:95`, `:101`, `research/sherlock-e2e-spec-20261115/BRIEF.md:54`.
Scenario: GIVEN a proposed k=22 workaround, WHEN the proposal is reviewed, THEN an ordinary key-change receipt cannot authorize it.
Verification: NEW: decision-to-requirement review that separates hard-rule authority from ordinary prototype key revisions.

### sherlock-elastic-proving

#### ST-001 — REUSE: GPU inventory

When setup runs, the profiler shall record each GPU's model, compute capability, memory and maximum SM clock.
Source: `SETUP.md:9`.
Scenario: GIVEN a box with multiple GPUs, WHEN setup reads the inventory, THEN the profile preserves separate device limits.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:161`, `parse_gpus`, and `:600`, `cx_f6_mixed_gpus_keep_each_discrete_gpu_in_its_vram`. NEW: complete inventory acceptance test.

#### ST-003 — REUSE: host inventory

When setup runs, the profiler shall record usable CPU cores, RAM, process memory headroom and output-directory disk space.
Source: `SETUP.md:11`.
Scenario: GIVEN a cgroup limit below available RAM, WHEN setup calculates the budget, THEN it records the lower limit.
Verification: NEW: host inventory cases for cgroup pressure, missing GPU and low disk space. Implementation entry: `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:670`.

#### ST-004 — REUSE: measured calibration

When setup calibrates a shape, the prover shall perform a warm-up and the configured repeated full-size leaf proofs on each supported device.
Source: `SETUP.md:12`.
Scenario: GIVEN three configured repetitions, WHEN calibration finishes, THEN the profile separates warm-up from measured samples.
Verification: `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:670`, `setup`. NEW: sample-count and measurement-record assertions.

#### ST-005 — REUSE: calibration proof verification

If the stock verifier refuses a calibration proof, then setup shall stop.
Source: `SETUP.md:13`.
Scenario: GIVEN one corrupted calibration proof, WHEN setup checks it, THEN no successful profile is published.
Verification: NEW: fault-injected setup run through the stock verifier.

#### ST-007 — REUSE: profile cannot change proof identity

The setup mode shall not create, change or select a key, a shape, `plan_d` or a proof value.
Source: `SETUP.md:15`. Run-time selection among existing shapes belongs to SC-001.
Scenario: GIVEN two hardware profiles for one program, WHEN setup completes, THEN its program and universal-key identities remain equal.
Verification: NEW: before-and-after identity test around `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:670`.

#### PF-001 — REUSE: versioned profile

The profiler shall emit a `sherlock-profile/1` object with host, key identities, hardware, memory, disk and calibration records.
Source: `SETUP.md:20` through `:23` defines the exact fields and units.
Scenario: GIVEN successful setup, WHEN the profile is loaded, THEN its key and measurement identities survive serialization.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:79` and `:93`. NEW: round-trip schema test over every required field.

#### PF-005 — REUSE: invalid profile refusal

If a profile has a wrong format, missing field or invalid leaf time, then the prover shall refuse it.
Source: `SETUP.md:24`.
Scenario: GIVEN NaN, zero or a negative calibration time, WHEN the scheduler reads it, THEN scheduling fails.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:612`, `cx_f7_a_leaf_time_that_is_not_positive_is_refused`. NEW: missing-field and format tests.

#### PF-006 — REUSE: calibration identity

If profile key identities differ from the loaded keys, then the prover shall report that the run is not calibrated for those keys.
Source: `SETUP.md:25`.
Scenario: GIVEN a T10m profile and T25 keys, WHEN scheduling begins, THEN old timings cannot be reported as calibrated T25 results.
Verification: NEW: `hw_profile_key_set_is_this_key_set` failure on a mismatched profile.

#### SC-001 — REUSE: schedule inputs

When a profiled run starts, the scheduler shall derive its shape, device, concurrency and cut from the profile, run length and limits.
Source: `SETUP.md:31`.
Scenario: GIVEN the same program on muninn, huginn and odin, WHEN their profiles differ, THEN each uses its measured device limits.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:520`, `faster_device_wins_and_cut_is_fewest_leaves`, and the T16 hardware runner at `common/t16.rs:553`.

#### SC-004 — REUSE: memory admission

The scheduler shall bound concurrency by measured host and device memory and count unified-memory consumption in one shared pool.
Source: `SETUP.md:34`.
Scenario: GIVEN a smaller memory budget, WHEN one shape is scheduled, THEN its concurrency cannot exceed the budget.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:531`, `no_gpu_and_memory_cap_scale_down`, and `:544`, `unified_pool_counts_device_and_host_bytes_once`.

#### SC-006 — REUSE: no fitting shard

If no existing leaf shape fits the budget, then the scheduler shall refuse the run and state the cause.
Source: `SETUP.md:36`.
Scenario: GIVEN less memory than one leaf needs, WHEN the scheduler runs, THEN it allocates no proof job.
Verification: NEW: budget-below-one-leaf case around `T16OVM/crates/sherlock-leaf/src/profile.rs:238`.

#### SC-010 — REUSE: schedule validity without a menu

The prover shall accept schedules only when keyed shapes and nonempty cuts cover the run with one through S steps per leaf.
No schedule menu or shard-count list shall restrict acceptance.
Source: `SETUP.md:42`, `LOOP.md:105`.
Scenario: GIVEN a valid unlisted cut, WHEN the scheduler checks it, THEN it accepts the cut under the same keys.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:555`, `check_free_has_no_count_list`, and `:575`, empty-cut and overflow regression.

#### SC-012 — REUSE: schedule-independent claim

When one run uses different schedules, the prover shall produce byte-equal `claim_pub` values.
Source: `SETUP.md:44`, `FINAL/specs/sherlock-l-and-wrap/spec.md:95`.
Scenario: GIVEN the same deployment and run salt, WHEN partition, concurrency or host changes, THEN `claim_pub` remains equal.
Verification: `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:210`, `schedules`. NEW: final-key repetitions including suspension and resume.

#### SC-014 — REUSE: cross-box join

When leaves from different owned hosts are joined, the prover shall check equal key manifests and stock-verify every leaf before node proving.
The node shall enforce segment height and state links.
Source: `SETUP.md:46`.
Scenario: GIVEN one wrong host manifest or corrupted leaf, WHEN the join starts, THEN it stops before N1.
Verification: `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:762`, `join`, with checks at `:771` and `:781`. NEW: three-box final-key run.

#### LPC-006 — REUSE: two run lengths

When `pos_plan_schedules` runs two lengths under one `plan_d`, the node shall accept both valid runs.
Source: `COMP/specs/sherlock-loop-completion/spec.md:59`.
Scenario: GIVEN two run lengths and two valid partitions, WHEN each is proved, THEN all proofs use the same plan and key identities.
Verification: NEW: complete T16(b) real-proof run. Partial status remains recorded in `LOOP.md:37`.

#### LPC-007 — REUSE: scale-up demonstration

When the rekey enables longer runs, the elastic demo shall prove one program at 1x, 2x, 4x and 8x length under unchanged keys.
Source: `COMP/specs/sherlock-loop-completion/spec.md:70`.
Scenario: GIVEN these four lengths, WHEN the demo completes, THEN it records verified proofs, shard counts, wall time and peak memory.
Verification: NEW: T16(c) run on the final R_SL path. Compare claims only across schedules of the same run, not across different inputs or lengths.

#### BOP-003 — NEW ID: controlled heavy jobs

When a factory job needs heavy compute, the box runner shall acquire the host's resource slot and GPU lock before starting that work.
Source: `LOOP.md:58`, `:90`, `:96`, `:99`, `WIKI/log.md:4117`.
Scenario: GIVEN an active GPU lock holder, WHEN another GPU job arrives, THEN it waits without terminating the active prover.
Verification: NEW: scheduler test for slot limits, 64G shared budget on muninn, exclusive timing runs and gpu.lock contention. Guard source: `/home/hoskinson/.local/bin/heavy-slot:116`.

#### BOP-004 — NEW ID: box artifact handoff

When a box returns a proof artifact, the coordinator shall record its path and SHA-256 and check its wrap proof on muninn.
Source: `LOOP.md:54`, `:62`, `:96`.
Scenario: GIVEN an artifact whose hash differs from its handoff record, WHEN muninn receives it, THEN it rejects the artifact before qualification.
Verification: NEW: stale artifact, digest mismatch, dropped box and duplicate-result tests. Messages and hashes shall contain no secrets.

#### BOP-007 — NEW ID: orphan GPU lock recovery

If an ended job's sampler holds gpu.lock, then recovery shall terminate it only under the owner's orphan-lock conditions.
The conditions are a missing parent, no progress and a lock held for more than ten minutes.
Recovery shall log to the box bus before SIGTERM, then use SIGKILL only if needed. It shall preserve live provers and wrapd.
Source: `WIKI/log.md:4117`.
Scenario: GIVEN a live prover holding the lock, WHEN another job times out, THEN recovery leaves that process running.
Verification: NEW: simulated process-tree and lock-age cases. Installed remote enforcement: NO EVIDENCE FOUND.

### sherlock-zero-knowledge

#### ZK-001 — REUSE, AMEND: owned-host trust domain

The prover shall keep leaf, node, L, L_Fr and wrap jobs on devices owned by the user whose run is proved.
Source: `FINAL/specs/sherlock-zero-knowledge/spec.md:3`, corrected by `WIKI/log.md:4112` and `:4113`.
Scenario: GIVEN a user's three owned boxes, WHEN a run is partitioned, THEN private intermediates stay within that user's trust domain.
Verification: NEW: placement configuration test and witness-transfer policy check. No enforcing configuration test was located in the audit (`AUDIT.md:92`).

#### ZK-002 — REUSE: only the wrap is public

The prover shall publish only the blinded wrap proof and its permitted call vector.
Source: `FINAL/specs/sherlock-zero-knowledge/spec.md:10`, `DESIGN/TERMINOLOGY.md:52`.
Scenario: GIVEN a completed run, WHEN outputs and logs are inspected, THEN they expose no STARK proof, private operand, salt or private run shape.
Verification: `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:1527`, forbid-list construction, and prior privacy scan in `LOOP.md:39`. NEW: final-key logs and all export paths.

#### ZK-012 — REUSE: deployment salt

When a deployment is prepared, the prover shall draw `r_P` from the OS CSPRNG once and store it in `deploy.secret` with mode 0600.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:4`, confirmed at `WIKI/log.md:4107`.
Scenario: GIVEN a secret file with group or world permissions, WHEN the prover loads it, THEN it refuses the file.
Verification: `ZKOVM/crates/sherlock-leaf/src/blind.rs`, unit tests and file operations at `:125` and `:135`. NEW: production-path acceptance run.

#### ZK-013 — REUSE: run salt

When a new run is prepared, the prover shall draw a fresh input salt and store it in `run.secret`.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:15`.
Scenario: GIVEN two new runs of one input, WHEN both are prepared, THEN their salts and `claim_pub` values differ.
Verification: `ZKOVM/crates/sherlock-leaf/src/blind.rs`, `RunBlind::prepare` tests. NEW: run preparation and resume tests that distinguish a new run from another schedule of one run.

#### ZK-014 — REUSE: seed guards

If a production build receives a deterministic-proof seed setting, then the wrap prover shall exit with code 4 and write no proof.
The settings are `SHERLOCK_TEST_SEED`, `GW_DET`, `GW_CHIP_SEED` and `GW_SEED`.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:26`.
Scenario: GIVEN a build without `test-seed`, WHEN `GW_DET=1` is set, THEN no proof artifact is written.
Verification: `T11/tools/sherlock-wrap/src/zkguard.rs:27`, environment guard, and `:37`, seeded-export guard. NEW: every production entry point and seeded-build production refusal.

#### ZK-015 — REUSE, AMEND: EQ-ZK

When the wrap prover proves one statement twice, it shall use fresh proof blinders and produce unequal proof byte strings that both verify.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:37`, `ZK-RESULTS.md:19`.
Scenario: GIVEN one fixed statement, WHEN two production wraps are made, THEN their byte strings differ and the ledger accepts both proofs.
Verification: NEW: CPU and GPU EQ-ZK runs from `ZK-RESULTS.md:69`. Do not require every individual byte to differ. See gap G-08.

#### ZK-016 — REUSE: one-use blinding slots

If a request reuses a blinding slot, then `wrapd serve` or the STARK worker shall refuse it without making a proof.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:48`.
Scenario: GIVEN a consumed slot ID, WHEN a client replays it after a failed or completed call, THEN the daemon refuses it.
Verification: NEW: daemon reuse and failure-path tests in `ZK-RESULTS.md:77`. Include concurrent requests and worker restart policy.

#### ZK-017 — REUSE: GPU randomness

While the wrap uses odin or huginn's GPU, the prover shall obtain proof blinders from `OsRng`.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:59`.
Scenario: GIVEN a fixed statement on a GPU, WHEN two wraps are made, THEN both verify on muninn and their byte strings differ.
Verification: NEW: GPU half of T11, `ZK-RESULTS.md:99`, with the production caller's RNG path recorded.

#### ZK-018 — REUSE: case B refusal

If a configuration sends a private proving job to a host the user does not own, then the prover shall refuse that configuration.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:70`, `WIKI/log.md:4113`.
Scenario: GIVEN odin proving for someone who does not own odin, WHEN placement is requested, THEN no witness or job is sent.
Verification: NEW: owned-host admission test, negative transfer test and placement model. Veritee support requires a future design.

#### ZK-019 — REUSE: padding theorem

The SherlockSpec library shall prove `pv2_inj` and `pv2_count_det` using only `propext`, `Quot.sound` and `Classical.choice`.
Source: `COMP/specs/sherlock-zero-knowledge/spec.md:81`.
Scenario: GIVEN the final Lean library, WHEN its axiom report is inspected, THEN it contains no additional axiom for these theorems.
Verification: `T11/clean/fork/Clean/Sherlock/ZkPadding.lean` and `ZkPaddingAxioms.lean`, commit `71c4c001b`, recorded in `ZK-RESULTS.md:83` through `:97`. NEW: integration receipt.

### sherlock-gpu-proving

#### GPW-001 — REUSE: GPU path and CPU fallback

Where a supported GPU is available, the wrap prover shall use it with a CPU fallback that preserves verifier acceptance.
Source: `FINAL/specs/sherlock-gpu-wrap/spec.md:3`.
Scenario: GIVEN no usable GPU, WHEN a wrap is requested, THEN the CPU path produces a proof accepted by the same verifier.
Verification: NEW: final-key GPU and CPU full-chain acceptance on each supported box.

#### GPW-003 — REUSE: deterministic parity is test-only

While test-seed mode is active, CPU and GPU wrap provers shall produce byte-identical proofs at the same seed and thread configuration.
Source: `FINAL/specs/sherlock-gpu-wrap/spec.md:17`, qualification on thread count in `WIKI/concepts/ZeroKnowledge.md:29`.
Scenario: GIVEN matching seed and thread count, WHEN the two paths prove one statement, THEN their serialized proofs match.
Verification: NEW: controlled-seed parity at thread counts 1, 4 and 20, plus production refusal under ZK-014.

#### PRD-001 — REUSE: resident prover

The prover daemon shall retain keys, SRS, CUDA contexts, twiddles and preprocessed traces across local-socket requests.
Source: `FINAL/specs/sherlock-prover-daemon/spec.md:3`, `LOOP.md:38`.
Scenario: GIVEN a warm daemon, WHEN another run starts, THEN reusable data remains cached and run secrets reload separately.
Verification: NEW: warm/cold daemon run with cache-identity and per-request-secret tests. Reload rationale: `ZK-RESULTS.md:29`.

#### PF-007 — REUSE: profile-driven overlap

When the scheduler chooses wrap overlap, it shall use measured profile peaks and headroom to select full overlap, eviction or no overlap.
Source: `SETUP.md:26`.
Scenario: GIVEN a smaller GPU budget, WHEN full overlap no longer fits, THEN the scheduler selects the fitting policy.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:662`, `pipeline_from_the_three_boxes`.

#### PF-008 — REUSE: residual wrap memory

When early wrap execution is requested, the device guard shall reserve STARK extra bytes, the profile-derived wrap tail and the margin.
Source: `SETUP.md:27`.
Scenario: GIVEN resident cosets, WHEN the guard computes the tail, THEN it does not count those bytes twice.
Verification: `T16OVM/crates/sherlock-leaf/src/profile.rs:690`, `cx_odin_h70_wrap_tail_counts_resident_cosets`.

#### LPC-005 — REUSE: final-key performance evidence

When the rekey completes, the timing runner shall measure warm time, cold time and peak memory on huginn and odin against their own baselines.
Source: `COMP/specs/sherlock-loop-completion/spec.md:48`, `LOOP.md:96`.
Scenario: GIVEN a timing result from each box, WHEN a speed claim is made, THEN it compares runs on the same box and key configuration.
Verification: NEW: T14/T14b final-key runs with stage times, CPU/GPU placement, thread count, load, clock, peak memory and verifier results.

## 3. Hard-rule matrix

The complete hard-rule source is `research/sherlock-e2e-spec-20261115/BRIEF.md:54`.
The matrix distinguishes a code guard from a completed final-key measurement.

| Hard rule touching B | Requirement | Current code or evidence status |
|---|---|---|
| Never k=22, all circuits k<=21 | CPR-001, RKY-003, SPL-001 | The wrap entry asserts `k <= 21` at `tools/sherlock-wrap/src/bin/wrap.rs:32`. NO EVIDENCE FOUND for an exhaustive audit of every entry point. |
| Zero knowledge end to end | ZK-001, ZK-002, ZK-015, ZK-017 | Production wrap passes `OsRng` at `T11/tools/sherlock-wrap/src/bin/wrap.rs:82`. T11 EQ-ZK remains RESULTS-PENDING at `ZK-RESULTS.md:71`. |
| Seeded proofs test-only | ZK-014, GPW-003 | `T11/tools/sherlock-wrap/src/zkguard.rs:27` rejects seed variables without the feature. `:37` rejects seeded exports. Full production-route closure: NO EVIDENCE FOUND. |
| No plain witness to helpers | ZK-018 | Confirmed case-A boundary at `WIKI/log.md:4113`. Audit reports no configuration test at `COMP/specs/sherlock-zero-knowledge/spec.md:78`. Enforcing placement guard: NO EVIDENCE FOUND. |
| 100-bit security | RKY-005, SQG-001, SQG-014, SQG-015 | Frozen parameter code uses unique decoding at `OVM/crates/sherlock-lfr/src/frozen.rs:245`. G3 legs are missing in `RUNBOOK.md:61` through `:64`. Final qualification: NO EVIDENCE FOUND. |
| One universal key set | RSL-001, SPL-004, RKY-001, SC-012 | Universal-L equality assertion at `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:615`. Five-program r10 evidence at `LIST.md:33`. Whole-suite final-key equality: NO EVIDENCE FOUND. |
| Unchanged ledger 9.1.0.1 verifier | CPR-004, RKY-004, SQG-015 | T10m ledger acceptance recorded at `LOOP.md:31`. Final T25 ledger receipt: NO EVIDENCE FOUND. Slice C owns ledger integration. |
| Wrap VK <=50,000 bytes | RKY-004 | T10m's 1,325-byte measurement is recorded at `LOOP.md:31`. Final VK size: NO EVIDENCE FOUND. |
| One BLS12-381 wrap | SPL-001 | The chain includes L_Fr at `OVM/crates/sherlock-wrap-export/examples/t7_chain.rs:1100`. The wrap proof call is `tools/sherlock-wrap/src/bin/wrap.rs:79`. Final composed receipt: NO EVIDENCE FOUND. |
| Leaves and nodes are small-field STARKs | SPL-001 | Leaf engine selects BabyBear at `OVM/crates/sherlock-leaf/src/harness.rs:26`. Backend field and degree-four extension appear at `SB/crates/stark-sdk/src/config/baby_bear_poseidon2.rs:36` and `:37`. L_Fr is the explicit Fr-native bridge, not a small-field node. |
| GP-1..GP-4 never narrowed | GP-001..GP-004, CPR-005, LPC-003 | Open hooks remain at `LOOP.md:27`. Mixed SHA/Keccak programs are refused at `OVM/crates/sherlock-leaf/src/rsl/trace.rs:114`. Full GP closure: NO EVIDENCE FOUND. |
| One root per checkpoint, batched Midnight writes | BOP-006, GP-002, LPC-003 | T20 implementation is reported at `LOOP.md:42`. B supplies sealed SET roots. Final B-to-C batched proof path: NO EVIDENCE FOUND. |
| Case A only, case B later via Veritee | ZK-001, ZK-018 | Owner ruling: `WIKI/log.md:4112` and `:4113`. Enforcing admission test: NO EVIDENCE FOUND. |
| K_MAX=2 | ANC-012, ANC-019 | Owner confirmation: `LOOP.md:106`. Existing SherlockLedger has no chunked entry at `/home/hoskinson/Projects/Moriarty-wt/t20-batched/tools/sherlock-sdk/sherlock-ledger.compact:21`. Integrated cap enforcement: NO EVIDENCE FOUND. |
| MERGE-WORK 5,400 | LPC-002 | `MERGE_WORK_COLUMNS = 5400` at `OVM/crates/sherlock-lfr/src/frozen.rs:169`. Complete STACKS and VALUE DIGEST fit remains open at `AUDIT.md:78`. |
| x_j lock form canon | ANC-017 | Extended form is listed at `LIST.md:5`. OVM still uses only c, run_tag and j at `OVM/crates/sherlock-lfr/src/iface.rs:289` through `:291`. Integration required. |
| F-3 replaced by setup mode | ST-007, SC-010 | Profile scheduler and free-cut checker exist at `T16OVM/crates/sherlock-leaf/src/profile.rs:238` and `:339`. No-count-list test is at `:555`. |
| Parameters, keys and key handling remain open until the prototype | BOP-005, RKY-001, RKY-002, SPL-009 | Ruling at `LOOP.md:101`. Code can regenerate keys through `OVM/crates/sherlock-lfr/src/frozen.rs:329`. Change qualification is a process requirement, not proof of a runtime guard. |
| Hard-rule changes remain owner-gated | BOP-005 | Explicit authority at `LOOP.md:95` and `:101`. Machine enforcement of this governance rule: NO EVIDENCE FOUND. |

## 4. Decision list: MADR candidates

Reuse the decision identities below. Do not allocate a new D-number for an existing ruling.
The integration lead can map these identities to its decision files.
This follows `METHOD/architecture-decisions.md:43` through `:55` and `METHOD/decision-allocation.md:3` through `:5`.
Options marked “rejected” describe an alternative, not authority to implement it.

| Existing ID | Title and drivers | Options | Outcome and authority | Confirmation | Revisit condition |
|---|---|---|---|---|---|
| OD-24 | Ledger-readable single wrap. Drivers: stock ledger, k limit, VK size. | G4 ledger change, oversized single-lane wrap, two BLS layers, list decoding, Fr-native bridge. | Use option E: L_Fr then one stdlib wrap. OWNER, `DESIGN/DECISIONS.md:35`. | SPL-001, LW-018, RKY-003/004, SQG-015. | A new measured path meets all hard rules and the owner replaces this ruling. |
| OD-4, OD-19 | Proven security and margin. Drivers: theorem scope and fixed shapes. | Unique decoding with explicit vectors, earlier list-decoding design. | Use unique decoding and the margin rule. OWNER, `DESIGN/DECISIONS.md:18` and `:33`. | RKY-005, SQG-001, SQG-014/015. | A changed configuration requires a new theorem mapping and gate run. |
| OD-18 | Separate hash and transcript domains. Drivers: cross-layer separation and parity. | Distinct use codes, reused codes. | Use distinct L and L_Fr domains. OWNER, `DESIGN/DECISIONS.md:32`. | LW-002, LW-018, SQG-014. | Encoding or transcript changes. Rekey affected layers and repeat parity. |
| OD-9, T11-3 | Owned-host ZK boundary. Drivers: privacy and unmasked STARKs. | Owned hosts only, outsourced private proving. | Case A now, no PCS masking required inside that boundary. OWNER for OD-9 at `DESIGN/DECISIONS.md:23`. LEAD PICK confirmed for T11-3 at `WIKI/log.md:4107`. | ZK-001/002/018. | A job or intermediate proof must leave the user's trust domain. |
| T11-1 | Three randomness lifetimes. Drivers: stable anchor, hidden input, fresh proofs. | r_P per proof, r_P per deployment with separate run and proof randomness. | Deployment r_P, run salt, proof blinders. LEAD PICK confirmed, `WIKI/log.md:4107`. Detailed basis: `ZK-RESULTS.md:17` through `:21`. | ZK-012/013/015. | Deployment identity or commitment scheme changes. |
| T11-4 | Test-only deterministic paths. Drivers: repeatable tests and production privacy. | Always-available environment seeds, feature-gated seeds. | Feature-gated seeds and production refusals. LEAD PICK confirmed, `WIKI/log.md:4107`. | ZK-014, GPW-003. | A new executable, seed source or artifact import path appears. |
| OD-13, T11-2 | Resident daemon with one-use slots. Drivers: warm latency and fresh run secrets. | Restart per call, reusable daemon with per-call secret loading. | Resident local-socket daemon. OWNER at `DESIGN/DECISIONS.md:27`. T11-2 LEAD PICK confirmed at `WIKI/log.md:4107`. | PRD-001, ZK-016, LPC-005. | Cache identity, crash recovery or concurrent slot tests fail. |
| F-3 | Replace schedule admission with setup profiling. Drivers: hardware elasticity and fixed keys. | Schedule menu in key_root, profile-derived schedule. | Profile-derived schedule. OWNER, `LOOP.md:105`, `WIKI/log.md:3577`. | ST-007, SC-010/012/014, LPC-006/007. | Hardware measurements no longer predict safe schedules. Keys must remain independent of scheduling. |
| N-6 | MERGE-WORK has its own bounds. Drivers: two prior proofs plus new work. | Internal 3,000-column bounds, dedicated 5,400-column set, remove GP work. | Dedicated set, keep GP-4. OWNER ruling and confirmed bound at `MOD/specs/sherlock-proof-shape/spec.md:77`, `LOOP.md:106`. | LPC-002, SHP-006/009. | Complete kernel exceeds 5,400. Hard-rule changes require the owner. |
| N-7, N-8 | PCD shape and foreign-root treatment. Drivers: universal composition and authenticated registry context. | Address-dependent acceptance, separate shape commitment and authenticated claim/registry rules. | Keep the confirmed T8b picks. LEAD PICK confirmed, `LOOP.md:87` and `:106`. Exact wording must be carried from the governing requirements. | GP-002/003/004, NODE-004, SHP-009, LPC-003. | New foreign-consumption semantics or a demonstrated composition counterexample. |
| O10 | Replace KeyBind pvs chain. Drivers: GP-3 and GP-4 scope. | KeyBind chain, authenticated key_root and universal-key routing. | Withdraw narrowing KeyBind chain. OWNER, `LOOP.md:66` and `:79`. | SPL-004, NODE-004, RSL-001. | A replacement authenticates all supported key paths without narrowing GP scope. |
| O3 | Option A for the prototype path. Drivers: measured time and wrap headroom. | Added L_Fr baseline A, blowup-2 alternative B. | A for prototype freeze. LEAD PICK confirmed by delegated “you decide”, `LOOP.md:83` and `:84`. | LW-018, RKY-003/005, LPC-005. | B fits the row gate with a real key and measured total call time. |
| T12-SLOT15 | Reserve anchor slot 15. Drivers: contract-field separation and fixed circuit pin. | Slot 0, fixed slot 15, prover-selected slot. | Fixed slot 15 in decoder, interface and contract. LEAD PICK confirmed, `WIKI/log.md:4107`, proposal at `:4100`. | ANC-013 plus A-owned ANC-015 and C-owned ANC-014. | Ledger layout or anchor schema changes. |
| T10Q-NT2/NT3 | Bind the lock to the complete run. Drivers: splice and truncation refusal. | Old P2 lock, short x_j, extended x_j. | Extended x_j in LIST. LEAD PICK confirmed, `WIKI/log.md:4107`, `LIST.md:5`. | ANC-017/019. | A counterexample changes a run without changing the lock. |
| T10Q-NTFIX-ORD | Refuse ordinary anchor writes. Driver: NT-1. | Permit matching template, reject matching template in mode 0. | Mode-0 template refusal. LEAD PICK confirmed through the single rekey list, `WIKI/log.md:4107`, `LIST.md:6`. | ANC-016/018. | Effects move or template layout changes. |
| T25 / single rekey | Integrate all key-changing inputs once. Drivers: coherent keys and expensive regeneration. | Separate lane rekeys, one integrated rekey. | One manifest and one integrated generation. LEAD PICK confirmed, `WIKI/log.md:4107`, `LOOP.md:47`. | RKY-001..005, SQG-014/015. | A late circuit change invalidates generated keys. Record the repeat, never conceal it. |
| T20 chunk cap | Bound ledger fallback. Drivers: bounded transactions and batched writes. | Unbounded chunking, K_MAX=2. | K_MAX=2. OWNER, `LOOP.md:106`. | ANC-012/019 and slice-C batching requirements. | A measured path cannot fit the cap. Do not narrow GP run length to hide the failure. |

No new decision above has claimed owner confirmation without a source.
FP-3's exact circuit construction is a LEAD PICK for T15, not a separate owner-approved algorithm (`COMP/specs/sherlock-anchor-chunks/spec.md:67`).
Its MADR candidate keeps ID FP-3, compares unbound call lanes with the effect/forward-chain binding, and selects the binding.
Drivers are shifted effects and slice truncation. Confirmation is ANC-018/019. Revisit on a counterexample or failed shape gate.

## 5. DDD draft

This section proposes a domain model. It does not claim that event classes or services already exist.
The factory and product remain separate, as required by `METHOD/architecture-decisions.md:13`.
Events below name facts at interfaces. They do not require an event broker (`METHOD/architecture-decisions.md:9`).

### Bounded contexts and context map

| Context | Responsibility and owner | Upstream/downstream relation | Interface and translation owner |
|---|---|---|---|
| Program admission | Slice A owns language, machine_version, plan and kit semantics. | Upstream of proof construction. | Published language: rsl-call/1, R_SL statement, prog_d, plan_d, key_root. B owns validation at the prover boundary. |
| Proof construction | Slice B owns app, leaf, node and foreign-proof verification. | Downstream of admission, upstream of bridge. | Published language: typed SEG, SET and FV statements plus proof/key identities. |
| Bridge binding | Slice B owns L, L_Fr, effects and forward chain. | Downstream of proof construction, upstream of slice-C wrap. | Published language: frozen L interface and private L_Fr export. C owns the conversion to wrap inputs. |
| Prover placement | Slice B owns profiles, schedules, owned-host placement and checkpoints. | Supporting context for proof construction. | Profile and schedule are host data. They do not redefine proof semantics. |
| Key qualification | Slice B owns the integrated key manifest and gate evidence. | Upstream of all proving stages and slice-C deployment. | Published language: key-set identity, freeze items and gate receipts. |
| Ledger settlement | Slice C owns call-vector ABI, one wrap, acceptRun and acceptCollection. | Downstream of bridge and qualification. | Anti-corruption layer: map typed run statements to ledger lanes without exposing private proof data. |
| Factory box coordination | Maintainer tooling owns slots, bus messages and artifact movement. | Supports measurements and delivery, outside the public prover contract. | Paths, digests, resource claims and verifier receipts. It is not a developer prerequisite. |

The logical statement schema and hash-domain definitions form a proposed shared kernel between A and B.
The L export and call-vector ABI are published languages, not automatic shared kernels.
This distinction follows `METHOD/architecture-decisions.md:33`.
Existing statement binding is specified at `FINAL/specs/sherlock-shard-plan/spec.md:22` and `MOD/specs/sherlock-node-integration/spec.md:37`.

### Aggregates and invariants

| Aggregate root | Context and identity | Invariants | Atomic boundary |
|---|---|---|---|
| AdmittedProgram | Program admission, `(machine_version, prog_d)` | Image, plan and key_root are bound. Preprocessing uses no witness. Per-program keys do not replace universal keys. | Admit or refuse one complete program manifest. |
| KeySet | Key qualification, manifest digest | All required revisions are integrated. Shape/query/domain constants match. Gate receipts refer to these exact key bytes. | Publish one coherent key set or no qualified set. |
| Run | Proof construction, semantic claim plus private run identity | One deployment salt and one run salt. Schedules preserve the claim. Terminal outcome is authenticated. | Bind the run's private inputs and claimed result. |
| SegmentProof | Proof construction, proof digest and interval | Correct key, required AIRs, bounded heights, matching program and state transition. | Accept one verified segment. |
| CompositionProof | Proof construction, root statement | Child keys and tags match. SEG links are adjacent. SET sealing and FV completion rules hold. Shape does not become replay identity. | Accept or refuse a full node proof. |
| CallBinding | Bridge binding, `(run identity, j, k)` | FP-3 effects match guest output. Forward chain ends correctly. Anchor slot and extended lock match. Chunk fallback respects K_MAX. | Bind all call lanes and chain links before export. |
| MachineProfile | Prover placement, `(host, key_root, keys_d, profile version)` | Measurements use verified proofs. Units are explicit. Limits are positive. A stale key identity is not calibrated. | Load a valid profile or refuse it. |
| ProofJob | Prover placement, request ID and key identity | Owned-host placement, bounded resources, immutable input identity and one-use blinding allocation. | Admit one job. Retrying does not reuse proof blinders. |
| Checkpoint | Prover placement, run identity and prefix interval | Key manifest matches. Prefix proof verifies. Recorded height and post-state match the proof. | Restore the complete prefix or refuse it. |

Evidence for the proposed invariants: SPL-002/004/005 in `FINAL/specs/sherlock-shard-plan/spec.md:11`, `:29`, `:36`.
Node invariants: `MOD/specs/sherlock-node-integration/spec.md:4`, `:15`, `:26`, `:37`.
Checkpoint checks exist at `T16OVM/crates/sherlock-wrap-export/examples/common/t16.rs:149` through `:158`.
Call binding is specified by `COMP/specs/sherlock-anchor-chunks/spec.md:59` and `:70`.

### Domain events and failure flow

| Proposed event | Minimum payload | Producer -> consumer | Failure and duplicate rule |
|---|---|---|---|
| ProgramAdmitted | machine_version, prog_d, plan_d, key_root, kit identities | A -> B | Unknown machine version or kit refuses admission. |
| KeySetQualified | manifest digest, key hashes, gate receipt references | B qualification -> A/B/C | Missing or stale receipts cannot emit this event. |
| MachineProfiled | host identity, profile version, calibration key identity | B setup -> B placement | Duplicate profile does not create keys. |
| RunPrepared | opaque run handle, deployment identity, semantic commitment references | A/client -> B | No salt or witness in an event or public log. |
| ShardVerified | run handle, interval, key identity, private artifact reference | B worker -> B composition | Verify a duplicate before reuse. Never treat a bus ACK as proof validity. |
| RootVerified | run handle, typed root, private proof reference | B composition -> B bridge | Wrong root tag or incomplete FV state refuses the handoff. |
| CallBound | run handle, j/k, interface version, private L_Fr reference | B bridge -> C wrap | Export stays in the owned-host trust domain. |
| WrapVerified | claim_pub, proof digest, ledger verifier receipt | C wrap -> C settlement | Retry settlement with its existing replay identity. |
| ProofJobRefused | job ID, public error code, retry class | B -> client/operations | No witness-derived diagnostic data. |
| CheckpointRestored | run handle, verified prefix interval, key identity | B storage -> B scheduler | Corrupt or stale checkpoints cannot advance the run. |

Cross-box jobs are a process across aggregates. A resource lease does not imply proof validity.
Loss of a box leaves the run incomplete. Resume only from a verified checkpoint or reproduce the missing shards.
Every re-created wrap uses new proof blinders. The run salt remains stable for another schedule of the same run.
These proposals bind SC-012/014, ZK-013/016 and BOP-003/004.

### Interfaces to A and C

- A supplies the exact R_SL statement and all kit encodings. B checks canonical values, key membership and native-word parity through RSL-001..008.
- The secp boundary uses one 32-byte-limb word per internal field element. Tape scalars use two words and points use five (`SECP.md:31`, `:38`).
- A supplies witness-free plan data. Setup never inserts hardware, cuts or concurrency into prog_d (`SETUP.md:15`, `:42`).
- B supplies C with private L_Fr proof data under the qualified key manifest. Only the permitted call vector becomes public (`FINAL/specs/sherlock-l-and-wrap/spec.md:26`, `:155`).
- B and C share anchor mode, j/k, a_in/a_out, extended x_j and slot-15 templates. A refuses a user field at slot 15.
- B supplies authenticated SET roots for C's batched acceptCollection. C owns one root per checkpoint and consumer membership/nullifier handling (`LOOP.md:42`).
- C returns ledger-check receipts for the exact VK/proof bytes. B cannot infer ledger compatibility from a stock STARK verification.

### Vocabulary that must stay precise

Use “universal, elastic proving co-processor” from `DESIGN/TERMINOLOGY.md:9`.
“App key” can be program-specific. “Universal key set” does not mean every app shares one app key (`DESIGN/TERMINOLOGY.md:36`).
“L_Fr” is an Fr-native STARK, not the BLS12-381 wrap (`DESIGN/TERMINOLOGY.md:51` and `:52`).
“GPU leaf” must state whether only the app stage ran on GPU. The T16 report distinguishes these stages (`T16/research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:52`).
“One run” keeps its salt across schedules. “Another run” draws another salt (ZK-013 and SC-012).

## 6. Open gaps and contradictions, ranked by prototype risk

| Rank / ID | Conflict or missing closure | Evidence on both sides | Required resolution |
|---|---|---|---|
| P0 G-01 | Final security qualification is not a calculator pass. | Ruling requires complete qualification: `FINAL/specs/sherlock-security-gate/spec.md:4`. Missing transcript legs: `RUNBOOK.md:61` through `:64`. | Build G3 legs. Run G1..G5 and FP-8 against T25 keys. Keep security status open until then. |
| P0 G-02 | The single-rekey manifest is incomplete and stale. | T25 requires FP-3 and slot 15 at `LOOP.md:47`. `LIST.md:5` through `:36` omits those rows and marks C3.1 uncommitted at `:7`. Audit records C3.1 commit `95990a6d3` at `AUDIT.md:44`. | Complete the manifest and check integrated revisions before keygen. T11 host-only changes need recorded provenance even when they do not change keys. |
| P0 G-03 | Rekey-chips is not the extended-lock integration tree. | Canonical extended lock: `LIST.md:5`, confirmation `WIKI/log.md:4107`. Short lock in code: `OVM/crates/sherlock-lfr/src/iface.rs:289` through `:291`. | Integrate NT-2/NT-3 before final key generation. Repeat splice and truncation tests. |
| P0 G-04 | FP-3 code exists separately, but final-chain closure is unproved here. | Required effect and slice binding: `COMP/specs/sherlock-anchor-chunks/spec.md:59`, `:70`. EffectAir entry exists at `FP3/crates/sherlock-lfr/src/iface_l.rs:777`. Audit reports open integration at `AUDIT.md:39`. | Run shifted-write and k-truncation attacks through real L, L_Fr and wrap. Final integrated pass: NO EVIDENCE FOUND. |
| P0 G-05 | Case-A-only policy lacks demonstrated configuration enforcement. | Owner refuses case B: `WIKI/log.md:4113`. Audit explicitly finds no configuration test: `COMP/specs/sherlock-zero-knowledge/spec.md:78`. | Enforce owned-host admission before moving witness data. Add negative placement and transfer tests. |
| P0 G-06 | Slot-15 status is distributed across trees. | Audit says decoder and contract unchanged: `COMP/specs/sherlock-anchor-chunks/spec.md:12`. Current separate decoder has slot 15 at `T12SLOT/tools/sherlock-wrap/src/decode.rs:70`; interface has it at `SLOT/crates/sherlock-lfr/src/iface.rs:112`. | Refresh the audit status. Check one integrated decoder, interface, compiler refusal and deployed contract. Old status is not evidence of current absence. |
| P1 G-07 | Universal scope conflicts with the mixed-hash refusal. | GP-001 covers every language program: `VM/specs/sherlock-general-purpose/spec.md:4`. Code asserts SHA and Keccak cannot coexist: `OVM/crates/sherlock-leaf/src/rsl/trace.rs:114`; refusal test at `tests/rsl_keccak.rs:150`. | Support the combination without narrowing the language or universal key rule. Until then list it as an explicit prototype gap. |
| P1 G-08 | EQ-ZK scenario requires every byte range to differ. | Completion scenario says this at `COMP/specs/sherlock-zero-knowledge/spec.md:41`. Governing intent is fresh blinders and unequal proofs: `FINAL/specs/sherlock-l-and-wrap/spec.md:109`, `ZK-RESULTS.md:19`. | Amend ZK-015 to unequal complete byte strings plus validity. Keep RNG evidence separate. Equal individual bytes do not establish blinder reuse. |
| P1 G-09 | Older T11 wording gives the wrong r_P lifetime. | `LOOP.md:33` says fresh per proof. Corrected lifetimes at `ZK-RESULTS.md:17` through `:21`, confirmed at `WIKI/log.md:4107`. OVM bench still fixes r_P at `t7_chain.rs:927`. | Use deployment/run/proof lifetimes. Ensure bench constants cannot enter production paths. |
| P1 G-10 | MERGE-WORK bound is confirmed, but complete-kernel fit remains open. | Bound code: `OVM/crates/sherlock-lfr/src/frozen.rs:169`. Missing STACKS/VALUE DIGEST and stale 5,417 issue: `AUDIT.md:78`. Later narrower width is recorded at `WIKI/log.md:3177`. | Measure the complete kernel. Do not use a derived fit or a reduced kernel as closure. |
| P1 G-11 | GP hooks remain incomplete despite partial successful gates. | Partial passes and open X8/SET/STACKS: `LOOP.md:27`. Full GP demands at `VM/specs/sherlock-general-purpose/spec.md:18`, `:32`, `:42`. | Give the T8c work an explicit task home and retain full GP acceptance tests. |
| P1 G-12 | Profile scheduling replaces stale menu-based text. | Old admitted list: `FINAL/specs/sherlock-shard-plan/spec.md:78`. Owner replacement: `LOOP.md:105`. No-count-list test: `T16OVM/crates/sherlock-leaf/src/profile.rs:555`. | Supersede the schedule-list clauses. Preserve authenticated key admission, which is a different condition. |
| P1 G-13 | T16 results do not establish final-key scale-up. | Full 1x..8x requirement: `LOOP.md:37`. T16 uses T10m keys and a five-step program: `T16/research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:3`, `:56`. | Run two lengths, four scales and three-box placement on T25 keys. Preserve old results as historical measurements. |
| P1 G-14 | r10 probe coverage is smaller than the full kit space. | Five named calls measured at `LIST.md:33`. New Keccak and secp changes follow at `LIST.md:34` and `:35`. | Repeat RSL-001/002 across the completed kit set and suite. Execution equality at `LIST.md:30` is not proof acceptance. |
| P1 G-15 | K_MAX=2 lacks a demonstrated deployed chunk path. | Owner cap at `LOOP.md:106`. No chunked ledger entry at `/home/hoskinson/Projects/Moriarty-wt/t20-batched/tools/sherlock-sdk/sherlock-ledger.compact:21`. | State the current single-call path. Test fallback planning and circuit cap before claiming chunk support. |
| P2 G-16 | heavy-slot memory rules disagree. | Older loop line says 90G at `LOOP.md:99`. Corrected rule says 64G at `LOOP.md:90`. Installed default is 64 at `/home/hoskinson/.local/bin/heavy-slot:28`. | Specify 64G shared on muninn and exclusive admission above it. Use each remote box's measured budget. |
| P2 G-17 | “GPU leaf” can hide CPU recursion work. | T16 describes GPU leaves at `LOOP.md:37`. Stage report says the leaf layer remains CPU at `T16/research/sherlock-t16-elastic-20261114/RESULTS-CDE.md:52`. | Report device placement and time for app, leaf aggregation, nodes, L, L_Fr and wrap separately. |
| P2 G-18 | Canonical main architecture is an older source than the current terminology and STARK rulings. | Main wiki identifies revision B2 and its proposed status at `WIKI/sources/sherlock-architecture.md:16`. Current bridge terms identify L_Fr at `DESIGN/TERMINOLOGY.md:51`; governing LW-018 defines its STARK at `FINAL/specs/sherlock-l-and-wrap/spec.md:155`. | Use current owner rulings and governing STARK requirements. Do not import old halo2 row budgets as current leaf or node gates. |

Additional evidence limits:

- `RUNBOOK.md:77` and `:87` contain unverified steps, as also recorded at `AUDIT.md:48`. Final pass receipts were not located.
- `FP3/crates/sherlock-lfr/src/fp3.rs:3` cites a main-tree `fp3/SPEC.md` that was not available at that named path. Its FP3-001..014 text is NO EVIDENCE FOUND here. Reuse ANC-018/019 for the completion requirements.
- The T11 report describes ZK conditional on the wrap-ZK premise A7 (`ZK-RESULTS.md:39`). EQ-ZK and padding lemmas do not discharge that cryptographic premise.
- The exact installed remote `gpu.lock` implementation was not inspected. Code-level enforcement of the orphan rule is NO EVIDENCE FOUND. The owner rule is explicit at `WIKI/log.md:4117`.
- The factory box bus is an operations mechanism. No evidence here establishes it as a public prover API. Keep it outside the developer compile/prove/deploy dependency chain.

Phase 1 ends with a proposed coverage and acceptance draft. Final-key implementation status remains subject to the cited gaps.
