# Chip leaf experiments E0, E1 and E3

Run `sherlock-processor-analogy-20261110`, 2026-11-10. Experiments of `study.md` section 6.
Design: `openspec/changes/sherlock-vm-consensus-20261107/design.md` 2.7 (D1 to D8, D7 decided as
the cached frame protocol). Code: `crates/mori-chip/` (E3 spike) and
`crates/mori-backend/src/sherlock_machine/chip_census_tests.rs` (E0, E1). Outputs:
`~/.cache/chip/out/*.json`; run records in `qa/sherlock/runs.jsonl` (run ids below).

Labels: MEASURED is a `cost_model`, key or native count from a run named here. ESTIMATE is
arithmetic on measured numbers and stated assumptions.

## 1. Results

| Gate (design 2.7.9, study E3) | Limit | Result | Run |
|---|---|---|---|
| Node over the spike leaf key, S = 16 | 950,000 rows, k = 20 | 785,128 rows, k = 20: PASS | `20261110T180927Z-chip-job6` |
| Node over the spike leaf key, S = 1 | 950,000 rows, k = 20 | 768,872 rows, k = 20: PASS | `20261110T180927Z-chip-job6` |
| Run decider over that node | 1,000,000 rows, k = 20 | 929,166 rows, k = 20: PASS | `20261110T180927Z-chip-job6` |
| In-circuit verifier accepts the bus constraint system | accepts | accepts (node synthesized and keyed) | `20261110T180927Z-chip-job6` |
| Spike leaf rows per step before page-ins | 2^11 = 2,048 | 6: PASS (spike rules only) | `20261110T181935Z-chip-job8` |
| `k_leaf` at S = 16 | 16 | 13 (set by the 4,096-row RANGE table) | `20261110T181935Z-chip-job8` |

E1: one shared PROG walk cuts the dispatch block from 1,273,895 to 1,274,039 IR instructions per
rule to 338,807 to 338,951, a factor of 3.76 (MEASURED, instructions; `20261110T161224Z-chip-job1`).
Rows: 2,674,552 to 731,523 (k = 22 to 20), a factor of 3.66 (MEASURED, `20261110T181945Z-chip-e1`). Both are
above the kill line of 2x. Acceptance parity: 186 of 186 valid checks (136 fact(3) in run 1, 50 fold in the rerun) agree and match the expected
verdict (section 4.2).

## 2. The verifier-gadget panic (job4)

Symptom. `chip_e3_node_and_decider` panicked 12.7 s into run `20261110T164419Z-chip-job4` at
`midnight-circuits-7.3.0/src/verifier/verifier_gadget.rs:609:22`.

Cause. Line 609 is the `_ => panic!("We do not support other rotations")` arm of `get_point`
(lines 604 to 610). The in-circuit verifier evaluates a query only at `x`, `omega x` and
`omega^-1 x`, so every advice, fixed and instance query of the leaf must be at rotation -1, 0
or +1. The leaf of job2 and job4 queried beyond that. The public-input count, the arch and the
VK were not the cause: the stand-in calibration in the same run passed with the same node code.

Evidence that it was the layout:
- Job2 (`20261110T164333Z-chip-job2`, before the panic) measured 9, 30, 114 and 450 region rows
  at S = 1, 4, 16, 64 (7 S + 2) with 9 point sets. The current layout measures 8, 26, 98 and 386
  (6 S + 2) with 5 point sets (job8). Fewer point sets is consistent with fewer distinct
  rotations.
- The previous agent rewrote `cpu.rs`, `range.rs`, `alu.rs`, `bus.rs` and `leaf.rs` at about
  16:47Z to 16:49Z, after the panic at 16:45Z, and added `leaf.rs::rotations_are_unit`, which
  cites line 609. The run that would have tested the rewrite (job5) was cut by the reboot.
- The source of the old layout is not on disk, so the offending rotation is not known.

Fix (the previous agent's, verified here). A slot is 2 rows: a REG row (8 registers, 3 rule
flags, the step count) and a REQ row with two bus request lanes (columns 0..5 and 5..10). The
core step gate sits on the REQ row and reads the slot's REG row at -1 and the next REG row at
+1. A 128-bit value is 12 limb cells over 4 limb columns, 3 rows, and the user's gate sits on
the centre row. Regression guards: `rotations_are_unit` (configured constraint system) and
`tests/e3_spike.rs::spike_leaf_vk_rotations_are_unit` (keyed `vk.cs()`; added in this run). Job6
printed the keyed rotations: advice {-1, 0, +1}, fixed {0}, instance {0}.

Consequence for the full leaf: every gate sees at most 3 consecutive rows. The full register
file (about 32 words, study 2.1) does not fit one 12-column row, so the "unchanged unless named"
frame condition cannot be one gate over pre and post rows. It needs either wider rows (more
advice columns, more node cost per column) or copy constraints between register rows. Neither
is measured.

## 3. E3: the spike leaf, the node and the decider

### 3.1 Leaf (MEASURED, `20261110T181935Z-chip-job8`, `e3_leaf_rows.json`)

| S | Region rows | Rows per step | k | Table rows |
|---|---|---|---|---|
| 1 | 8 | | 13 | 4,096 |
| 4 | 26 | | 13 | 4,096 |
| 16 | 98 | 6 + 2 fixed | 13 | 4,096 |
| 64 | 386 | | 13 | 4,096 |

Rows per step: core 2 (REG, REQ) + ALU 3 (one 128-bit decomposition) + MEM 1 = 6; 2 rows close
the core region. Constraint system: 12 advice columns, 19 fixed columns in the cost model (14 in
the key), 2 instance columns (committed first, unused), 7 lookup arguments (4 RANGE limb columns,
2 bus lanes, DEC), 5 permutation columns, degree 6, 55 column queries, 5 point sets, proof 5,296
bytes. The superseded job2 numbers (9, 30, 114, 450) belong to the layout that panicked.

Tests (job8): 34 unit tests of `mori-chip` pass, including MockProver negatives N1 to N7 (forged
bus tuple, wrong ALU result, wrong memory read, wrong next control, unrequested chip row,
duplicated forged requests, request from a disabled slot), and `spike_leaf_s16_mock_prover`.

### 3.2 Node (MEASURED, `20261110T180927Z-chip-job6`, `e3_node_decider.json`)

`ChipNode` (in `tests/e3_spike.rs`) is the SHKIVC06 node of `mori-recursion/src/ivc.rs` with the
leaf verified under a raw `VerifyingKey`, because `MidnightVK` can only hold a stdlib relation.
Calibration (`20261110T164419Z-chip-job4`, `e3_node_calibration.json`): over the stand-in leaf
keyed at k = 13, the mirror and the real node both have 652,461 rows at k = 20, with equal
columns, lookups and queries.

| Leaf key | Node rows | k | Table rows |
|---|---|---|---|
| Stand-in leaf (stdlib), k = 13 | 652,461 | 20 | 1,032,211 |
| Spike leaf, S = 1, k = 13 | 768,872 | 20 | 1,032,211 |
| Spike leaf, S = 16, k = 13 | 785,128 | 20 | 1,032,211 |

The spike leaf costs the node 116,411 (S = 1) to 132,667 (S = 16) rows more than the stand-in
leaf. `study.md` 4.3 predicted 768,708 to 813,708 from a different baseline (741,708, a real
leaf); the two baselines are not the same leaf, so that delta is not compared here. The node has
165,448 rows of headroom under 950,000 at S = 16.

S = 1 and S = 16 have the same constraint-system shape and k, yet the node differs by 16,256
rows. The leaf's fixed and permutation commitments are constants in the node
(`bls.assign_fixed` of each base, then `resolve_fixed_bases`), so the node's rows depend on the
key's content. A likely cause is that some fixed commitments of the S = 1 key are the identity or
otherwise cheaper constants; this was not checked. Take S = 16 as the reference.

### 3.3 Decider (MEASURED, `20261110T180927Z-chip-job6`)

The run decider over the node keyed on the S = 16 spike leaf: 929,166 rows, k = 20, table rows
1,032,467; node keygen 42.7 s; unsafe SRS (rows do not depend on the SRS). This equals the
decider measured before the chip design. The decider consumes only the node key
(`run_decider.rs:86,218,244` resolve the node's own fixed bases); the leaf's fixed bases are
resolved inside the node (`inner.rs:207,313`, mirrored in `ChipNode` step 4). So the decider does
not depend on the leaf key while the node's constraint system is unchanged. `study.md` 4.3
expected a decider cost per leaf fixed base; that expectation was wrong, and that cost lands in
the node (3.2).

## 4. E1: one shared PROG walk

### 4.1 Instructions (MEASURED, `20261110T161224Z-chip-job1`, `e0_dispatch.json`)

Witness: fact(3) first step, qualification config `testdata/dispatch/config.json`; lanes: PROG
167, DATA 1,383, image 256, helper 87.

| Part of the dispatch block | IR instructions |
|---|---|
| Whole block, one rule (108 rules) | 1,273,895 to 1,274,039 |
| Dispatch PROG walk | 309,958 |
| One checked value read, whole | 321,027 |
| of which its own PROG walk | 304,277 |
| of which DATA canonicality | 14,842 |
| of which lane range checks | 1,550 |
| of which selection | 358 |
| Four PROG walks (dispatch + 3 reads) | 1,222,789 (96 % of the block) |
| Facts lane range checks | 343 |
| Rule logic, per rule | 513 to 657 |
| Shared-walk block (`emit_c6_dispatch_block_dedup`), one rule | 338,807 to 338,951 |

The shared-walk block emits one PROG walk, DATA canonicality once and each PROG and DATA lane's
range check once; guards and paths are unchanged. The ratio is 3.76 for every rule. The PROG
walk itself (about 310,000 instructions, of which 97,529 `CondSelect`, 56,947 `TestEq` and 52,438
`Mul`) is now the cost: in the chip design it is replaced by PAGE lookups.

### 4.2 Rows and parity

Run `chip-e1` (`chip_e1_shared_walk`): `cost_model` of the unchanged and the shared-walk circuit
for the first fired rule of fact(3) and for `BHelperMatchArm`, then acceptance parity on sampled
fact(3) and fold states (the fired rule accepts and one other rule rejects, under both circuits).

MEASURED (`20261110T181945Z-chip-e1`, `CHIP-E1 model` lines of `~/.cache/chip/job7-e1.log`;
witness parameters from the fact(3) first step):

| Rule | Circuit | IR instructions | Rows | k | Model seconds |
|---|---|---|---|---|---|
| `KMachineEvalCallArguments` (first fired rule of fact(3)) | unchanged | 1,276,146 | 2,674,552 | 22 | 208.5 |
| `KMachineEvalCallArguments` | shared walk | 341,058 | 731,523 | 20 | 51.2 |
| `BHelperMatchArm` | unchanged | 1,276,058 | 2,674,443 | 22 | 233.1 |
| `BHelperMatchArm` | shared walk | 340,970 | 731,414 | 20 | 44.7 |

Rows fall by a factor of 3.66 for both rules (2,674,552 / 731,523 and 2,674,443 / 731,414), and k
falls from 22 to 20. Rows per instruction: 2.10 unchanged, 2.15 shared walk. The instruction
counts here differ from 4.1 by about 2,100 because the circuit is built with the snapshot's
source parameters, not the census emitter's. The kill criterion of E1 (under 2x) is not met:
about 73 % of the dispatch rows were repetition. A shared-walk dispatch block still has 731,000
rows per step, so E1 alone does not reach the chip design's sizes; the walk itself is the
remaining cost.

Parity, run 1 (`20261110T181945Z-chip-e1`, 6,343 s, `e1_shared_walk_run1.json`): 186 checks. The
unchanged and the shared-walk circuit gave the same verdict in all 186. On fact(3) (68 sampled
states: every 8th step and every step with a pending Validate or Equal read) the fired rule
accepted and the other rule rejected under both circuits: 136 of 136 as expected. On fold all 50
checks failed under both circuits, including the fired rule. Cause: the harness built every
circuit with the fact(3) family parameters and META and PROG lengths, and the checker refuses a
witness of another length or family before it evaluates any constraint
(`c6_dispatch_consumer.rs:249-265`). The test then failed its own assertion
(`chip_census_tests.rs:733`). The harness now builds each circuit for the witness's own family
and lengths and records the checker's error.

Parity, fold rerun (`20261110T200650Z-chip-e1-fold`, 1,650 s, `e1_shared_walk_fold.json`;
`CHIP_E1_MODELS=0 CHIP_E1_RUNS=fold`): 50 checks on 25 sampled fold states (every 8th step), META
and PROG lengths 211 and 188. The fired rule accepted and the other rule rejected under both
circuits: 50 of 50 as expected, 0 mismatches. Every rejection is the checker's `Failed direct
assertion`, the same under both circuits. Only the harness changed between the runs, so the
cause of the run-1 fold failures is confirmed as the harness, not the shared walk.

Parity total: 186 valid checks (136 on fact(3) from run 1, 50 on fold from the rerun) on 93 sampled states of fact(3) and fold. The unchanged and the
shared-walk circuit agree on all of them, and all give the expected verdict when each circuit is
built for its witness's family and lengths. This is sampled parity on two programs and one
alternative rule per state, not an equivalence proof.

## 5. E0: census of the owner runs

MEASURED from `e0_owner_runs.json` (`20261110T161224Z-chip-job1`); summary script
`~/.cache/chip/scripts/e0_summary.py`, output `e0_summary.json`.

| Run | Steps | Distinct rules | Appended or rewritten table, frame and task rows per step (mean, p95, max) | Distinct `control_origin` per 16 steps (mean, max) | per 64 steps (mean, max) |
|---|---|---|---|---|---|
| fact(3) | 293 | 46 | 0.22, 1, 1 | 3.42, 6 | 8.8, 11 |
| fold | 196 | 57 | 0.20, 1, 3 | 3.08, 5 | 5.75, 8 |
| list123 | 303 | 55 | 0.20, 1, 3 | 3.00, 5 | 6.4, 8 |

- No step overwrites a non-zero table row, and no step writes a frame or task below the top.
  This agrees with write-once memory (design 2.7.3) on these three programs.
- Register groups written per step: 1 to 5 (fact(3): 134 steps write 2, 22 steps write 5).
- `control_origin` (the PROG row of the control word) is a proxy for distinct PROG rows. At S = 16
  it is at most 6, against the study's ceiling of 2 S = 32. The rows read by Eval, Find and Bind
  are not in this count.

Not measured: bus requests per step per chip, old cells read per window, the number of distinct
micro-operations per rule (E0(c)), and the per-rule fragment census (no `e0_fragments` output is
on disk). The E0 kill criteria on requests per step and on micro-operations are therefore open.

## 6. What the spike covers

Real (in the circuit, with MockProver negatives):
- CORE: 8 registers (`ctl` tag and two arguments, `fp`, `depth`, `limit`, `len`, `r`), 3 rule
  flags, the enable prefix, the step count, the frame condition and the next-state gate.
- Bus: tuples `(chip, key, a, b, c)`, two request lanes per REQ row, one `lookup_any` per lane
  with request and provider sides gated by fixed columns.
- ALU: 128-bit checked add with carry flag and less-than, one decomposition per operation.
- RANGE: 4,096-row table, 11 limbs of 12 bits and a pad cell for the top limb.
- MEM: write-once append log; first address copied from `len`; consecutive addresses.
- DEC: a 3-row fixed table, control tag to rule.

Stubbed or absent:
- Rules: three analogues, `Push` (`KMachinePushAccept`), `Ret` (`KMachineReturnRight`) and
  `Arith` (`BHelperScalarArithmetic`; the study named `KMachineEvalLiteralValid`). They are not
  the C6 rules and carry no equivalence claim. The other 105 rules are absent.
- PAGE, HASH and TRAP chips; the task stack; the full register file (about 32 words).
- Statement: of the node's 7 leaf lanes only `pre_d`, `post_d` and `steps` are constrained;
  `binding`, `prog_d`, `rule` and `h_pre` are not. `pre_d` and `post_d` are a placeholder linear
  digest (`trace::digest`), not Poseidon. The committed instance column is unused.
- No proof was made; all leaf, node and decider numbers are `cost_model` and keygen.

Design constraints D1 to D8 (design 2.7.8) in the spike:

| | Constraint | Spike |
|---|---|---|
| D1 | Provider-gated table side | Yes: table side `q_prov * tuple`, input `q_req * tuple`, both fixed columns (`bus.rs`) |
| D2 | One provider per key | Partly: MEM serves only this leaf's cells; there is no PAGE |
| D3 | Root covers appended rows only | No: no `mem_root`, no HASH |
| D4 | Fixed chip tag per region | Different: the `chip` field is advice, set by a selector gate on each provider region (`mem row`, ALU) |
| D5 | One enable per step | Yes: one slot per step; the enable is a prefix (`enable_is_a_prefix`) |
| D6 | One base for the MEM region | Different: the first address is copy-constrained to `len`; later addresses by a chain gate `next = addr + 1`, not a fixed offset column |
| D7 | Cached frame protocol | No: the spike uses the uncached protocol. Push appends the new frame with the link `fp` and sets `fp := len`; Ret loads the frame at `fp`. There is no `top` register |
| D8 | No tag equals the padding tuple | Yes: chip ids 1 and 2; an inactive request has `chip = 0` and the lookup forces the whole tuple to zero |

## 7. Leaf rows per step for the full rule set (ESTIMATE)

Measured per-unit costs from the spike: a slot of 8 register words and 2 requests is 2 rows; a
128-bit ALU operation is 3 rows (study 2.5 assumed 8); a MEM append is 1 row.

| Chip | Per step | Basis |
|---|---|---|
| CORE | 5 to 8 | ESTIMATE: 32 register words / 12 columns = 3 REG rows; 2 to 4 requests at 2 per row = 1 to 2 rows; the study's 8 as the high end. Ignores the frame-condition cost of section 2 |
| ALU | 6 to 12 | ESTIMATE: 2 to 4 operations (study 2.5) x 3 rows (MEASURED per operation) |
| MEM | 1 to 3 | ESTIMATE: E0 maximum of 3 appended or rewritten rows per step x 1 row; the mean is 0.2 |
| Subtotal without HASH | 12 to 23 | ESTIMATE |
| HASH for `mem_root` | 200 to 750 | Study 4.1, unmeasured (E4) |
| Total | 212 to 773 | ESTIMATE |

Leaf at S = 16, with `F` = 5,408 (study 4.2: RANGE 4,096, DEC 512, boundary digests 800) and
P distinct page-ins at 1 + depth x 25 rows (study 2.3): P = 6 (the E0 `control_origin` maximum)
at depth 8 gives 5,408 + 16 x 212 + 6 x 201 = 10,006 rows (k = 14); at depth 20 and the high step
cost, 5,408 + 16 x 773 + 6 x 501 = 20,782 rows (k = 15). ESTIMATE: Poseidon rows, page-in cost,
the full register file and the PROG rows read outside the control word are unmeasured. Both ends
are under `k_leaf <= 16`. HASH dominates: without it a step costs 12 to 23 rows.

## 8. Design consequences

- (a) Gates see rotations -1, 0 and +1 only (section 2). The next-state check of the full
  register file (about 32 words, more than one 12-column row) cannot be one gate over the pre
  and post rows. It needs wide rows (more advice columns, each a node cost) or copy constraints
  between register rows. Neither is measured; the CORE row of section 7 does not include it.
- (b) The decider does not grow with the leaf. The leaf's fixed and permutation commitments are
  resolved inside the node, so the decider stays at 929,166 rows for any leaf key while the
  node's constraint system is unchanged (3.3). The leaf's cost is a node cost: 785,128 rows at
  S = 16, 165,448 under the 950,000 gate.
- (c) Node rows depend on the leaf key's content, not only its shape: 768,872 at S = 1 against
  785,128 at S = 16, a difference of 16,256 rows with equal constraint systems and k (3.2). The
  cause is not known. Until it is, measure the node over the final leaf key, not over a smaller
  key of the same shape.
- (d) The spike deviates from design 2.7.8 in five constraints (section 6): D2 is partial (no
  PAGE, so no second provider exists), D3 is absent (no `mem_root`), D4 uses an advice chip tag
  set by a gate instead of the fixed selector column, D6 uses a chain gate `next = addr + 1`
  instead of a fixed offset column, and D7 uses the uncached frame protocol (no `top` register).
  The node and decider numbers hold for this constraint system; a leaf that follows D4, D6 and
  D7 has other fixed columns and register rows and must be re-measured.

## 9. Next

- E4: HASH (Poseidon) and one PAGE page-in in the spike; it settles the two constants that
  dominate section 7.
- Build the cached frame protocol (D7) and the fixed chip tag (D4) and fixed MEM offsets (D6) as
  the design states them, then re-measure the node.
- E0(c): micro-operations per rule, and requests per step per chip.
