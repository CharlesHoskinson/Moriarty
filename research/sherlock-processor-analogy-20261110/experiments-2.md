# Chip leaf experiments E5 (register file) and E6 (HASH)

Run `sherlock-processor-analogy-20261110`, 2026-11-10, second session. This closes the two largest open
items of design 2.7 (`openspec/changes/sherlock-vm-consensus-20261107/design.md` 2.7.10, "Design
consequences" and "Open"): the next-state check of the full register file under the rotation limit,
and the HASH cost of `mem_root`. Code: `crates/mori-chip/src/rf.rs` (32-word machine),
`src/rfcore.rs` (layouts and leaf), `src/hashchip.rs` (HASH), tests in `tests/e3_spike.rs` (E5/E6
section). Outputs: `~/.cache/chip2/out/*.json`; scripts `~/.cache/chip2/scripts/`; logs
`~/.cache/chip2/logs/`; run records in `qa/sherlock/runs.jsonl`.

Labels: MEASURED is a `cost_model`, a key or a count from a run named here. ESTIMATE is arithmetic
on measured numbers and stated assumptions. All rows are `cost_model` rows; no proof was made. Keys
use an unsafe SRS (rows do not depend on the SRS).

| Run | Content |
|---|---|
| `20261110T210452Z-chip2-job1` | build; 44 unit tests of `mori-chip` pass (34 old, 10 new) |
| `20261110T210718Z-chip2-job2` | keyed rotation guards; S = 16 MockProver per layout; leaf rows; HASH rows; stdlib calibration |
| `20261110T210755Z-chip2-job3` | node over the S = 16 leaf key of layouts A, A-inline, B, C, and A with 4x constraints |
| `20261110T211146Z-chip2-job4` | node over B and C with the stdlib HASH chip; run decider over B + HASH |
| `20261110T212440Z-chip2-job5` | unit tests and keyed rotations again; node over B, C and A-inline with a Poseidon-only HASH key shape; B with the stdlib HASH at zero hashes |
| `20261110T212829Z-chip2-job6` | node over B + Poseidon-only HASH with 1 and 3 extra lookup arguments, and B with 2 |

## 1. Results

| Gate (design 2.7.9) | Limit | Result |
|---|---|---|
| Node, register file only, layout B (pair rows) | 950,000 rows, k = 20 | 891,770 rows, k = 20: PASS (58,230 headroom) |
| Node, layouts A (wide), A-inline, C (staggered) | 950,000 rows, k = 20 | 1,373,001 / 1,061,231 / 1,058,485 rows, k = 21: FAIL |
| Node, B + stdlib HASH chip (`NativeChip` + `PoseidonChip`) | 950,000 rows, k = 20 | 999,488 rows, k = 20: FAIL by 49,488 |
| Node, B + Poseidon-only HASH key shape | 950,000 rows, k = 20 | 953,150 rows, k = 20: FAIL by 3,150 |
| Node, B + Poseidon-only HASH, RANGE on 1 limb lookup instead of 4 | 950,000 rows, k = 20 | ESTIMATE about 893,000 (953,150 - 3 x 22,550 per lookup, measured, + 7,300 for a running-sum column) |
| Run decider over the node keyed on B + stdlib HASH | 1,000,000 rows, k = 20 | 929,166 rows, k = 20: PASS |
| `k_leaf` at S = 16, every layout, every HASH option but one | 16 | 13; Merkle per append provisioned for 3 appends per step: 16 |
| Keyed rotations, every layout with and without HASH | -1, 0, +1 | -1, 0, +1 |

Recommendations:
- Item 1: layout B, pair rows with copy links (section 2). It is the only layout under the node gate.
  The node pays for leaf columns, not for gate size, so the layout with the fewest committed columns
  wins even though it has the most rows.
- Item 2: commit appended cells once per leaf as one aligned segment subtree of C = 8 cells, with
  its root appended to a depth-17 top tree (M3, section 3.4). This needs 41 two-input hashes, 8
  record hashes and the 2 boundary digests per leaf. That is 2,226 leaf rows at S = 16 on layout B
  (k = 13), or 121 HASH rows per step. The row count is not the problem; the node is. The stdlib
  HASH composition puts the node at 999,488, and a Poseidon-only HASH chip still measures 953,150.
  Build a Poseidon-only HASH chip and put RANGE on one limb lookup (ESTIMATE node about 893,000;
  section 3.3). Neither is built yet, so the gate with HASH is not yet passed by a measured key.

## 2. Item 1: register-file next state

### 2.1 What was built

`rf.rs` is a 32-word register file in the word layout of `study.md` 1.3: `ctl` 5, `st` 8, `env` 1,
`fp`/`tp`/`len` 3, `top` 2, `task` 2, `h` 6, `r` 2, `v0..v2` 3. The digest covers words 0..29 and
leaves out `v0..v2`. There are six rules over the cached frame and task protocol of D7. `Push` checks
ALU `fcount < limit`, appends `(top0, top1, fp)` at `len`, sets `fp := len` and loads `top` from
`ctl`. `Rewrite` changes `top0` and makes no bus request. `TPush` and `TPop` do the same for the task
stack. `Arith` is a checked add. `Pop` checks ALU `fp < len`, loads `mem[fp] = (a, b, link)`, sets
`top := (a, b)` and `fp := link`, and returns the old `top0` in `r1`. Every step writes `work`,
`fuel` and `v0..v2`. The run `Push, Rewrite, TPush, Arith, TPop, Pop` repeats. A step writes 4 to
10 words (E0 census: 2 to 5 register groups).

`rfcore.rs` writes every constraint once, over logical cells (`Pre(j)`, `Post(j)`, flags, the two
bus lanes, the count). A layout states where each cell lives and on which slot rows gates sit.
`Layout::host` puts each constraint on the first gate row that sees all its cells within rotations
-1..+1. Configuration panics if a constraint has no host, so no constraint is dropped. The chips are
the spike's: ALU (one 3-row 128-bit operation per slot), MEM (one row per slot), RANGE (4,096-row
table, 4 limb columns), DEC (6 rows). The state digest is the spike's linear placeholder. Its 29
words are copied (copy constraints) into a 3-row digest region in every layout, because a Poseidon
digest needs its inputs copied or requested, never one gate. A-inline instead computes the
placeholder in one gate on the register row. That is a lower bound, not a design option.

| Layout | Slot | Columns | Copies |
|---|---|---|---|
| A wide | 1 row: lanes 0..10, `ovf`, count, 6 flags, 32 registers | 50 advice | digest words only |
| A-inline | as A, plus a digest cell | 51 advice | none (lower bound) |
| B pair rows | 13 rows: 6 pair rows of `(pre, post)` for 6 registers, each between a lane row and a flag row; 7 routed operand cells | 12 advice | `post` to the next `pre`; lane and flag replicas; routed operands; digest words |
| C staggered | 2 rows: `lo` (25 registers) and `hi` (lanes, flags, count, 7 registers whose update needs only `hi` cells); gate on `hi` checks `lo`, gate on the next `lo` checks `hi` | 25 advice | digest words only |

On (B) as worded in the task: a copy constraint is unconditional, so it cannot express "unchanged
unless the rule names it". It can only link a slot's post cell to the next slot's pre cell. The frame
condition is still a gate over each `(pre, post)` pair, and that gate needs the flags and lanes
within one row. So B places replicas of the flag row and lane row next to every pair row.

Tests (MEASURED, job1 and job2). Every layout passes MockProver on honest leaves at S = 1, 6, 8 and
16 and on a partial leaf. For each of the 32 words, a wrong `post_j` after the last step (a `Pop`) is
refused by the core gate in every layout (`every_register_word_is_checked`). Also refused: a push
link that is not the core's `fp` (D7), a non-prefix enable, a wrong post digest, and for B a broken
post-to-pre link, a broken flag replica and a broken routed operand (permutation failures). The
configured and the keyed constraint systems query rotations -1, 0, +1 only in every layout, with and
without HASH (`rotations_are_unit`, `rf_leaf_vk_rotations_are_unit`). The spike's guard
`spike_leaf_vk_rotations_are_unit` still passes.

### 2.2 Leaf (MEASURED, job2, `e5_leaf_rows.json`)

| Layout | Advice | Permutation columns | Fixed columns in the key | Advice queries | Lookups | Rows S = 1 / 4 / 16 / 64 | Rows per step | k (S = 16) |
|---|---|---|---|---|---|---|---|---|
| A wide | 50 | 42 | 15 | 97 | 7 | 12 / 27 / 87 / 327 | 5 | 13 |
| A-inline | 51 | 5 | 13 | 98 | 7 | 6 / 21 / 81 / 321 | 5 | 13 |
| B pair rows | 12 | 13 | 21 | 36 | 7 | 36 / 87 / 291 / 1,107 | 17 | 13 |
| C staggered | 25 | 26 | 16 | 75 | 7 | 14 / 32 / 104 / 392 | 6 | 13 |
| E3 spike (8 registers) | 12 | 5 | 14 | | 7 | 8 / 26 / 98 / 386 | 6 | 13 |

Rows per step are core rows plus 3 ALU and 1 MEM. B's core is 13 rows per slot. In every layout k is
set by the 4,096-row RANGE table, so the register file does not move `k_leaf`.

### 2.3 Node (MEASURED, job3 and job4, `e5_node_*.json`)

`ChipNode` (the SHKIVC06 mirror of E3, calibrated equal to the real node) over the S = 16 leaf key.

| Leaf key at S = 16 | Node rows | k | Gate 950,000 |
|---|---|---|---|
| E3 spike (reference) | 785,128 | 20 | pass |
| A wide | 1,373,001 | 21 | FAIL |
| A wide, every core constraint emitted 4 times | 1,374,624 | 21 | FAIL |
| A-inline (lower bound) | 1,061,231 | 21 | FAIL |
| B pair rows | 891,770 | 20 | pass, 58,230 headroom |
| C staggered | 1,058,485 | 21 | FAIL |

The node pays per committed column, not per constraint. With the core constraints repeated 4 times,
A grows by 1,623 rows (0.1 %). A least-squares fit of node rows against the leaf key's column counts,
over the seven distinct keys measured here and in E3, gives (`fit_e5_node.json`; largest residual
3,917 rows):

    node rows ~ 570,957 + 7,298 x advice + 8,189 x permutation columns + 6,004 x fixed columns

The fit is an ESTIMATE used only to explain the measured rows. A permutation (equality) column
costs as much as an advice column. That is why wide rows lose: A needs every register column
equality-enabled for the digest copies (42 permutation columns). A-inline, which avoids the copies,
still has 51 advice columns. C halves both and still fails. B keeps the spike's 12 columns and pays
8 more permutation and 7 more fixed columns.

Consequence for the 108-rule core: gate size is close to free in the node, so wide control-word
expressions are not the risk. Columns and lookups are. An advice, equality or filled fixed column
costs about 6,000 to 8,000 node rows, and a lookup argument (a bus lane, a RANGE limb column) about
22,600 (section 3.3). The layout should keep the 12-column budget the ALU and the two bus
lanes already need.

### 2.4 Recommendation for item 1

Layout B. It is the only layout under the node gate (891,770). Its leaf is 17 rows per step and stays
at k = 13 at S = 16. Its cost is 13 rows per slot and 7 routed operand cells, which a rule set with
more cross-register updates may need more of (a spare cell is 0 node rows; a new column is about
7,000). A hybrid did not beat it: C is the best hybrid of wide and narrow rows (two staggered rows),
and it fails at 1,058,485. The node headroom left after B is 58,230 rows, less than either HASH
chip measured here costs (section 3.3).

## 3. Item 2: HASH cost

### 3.1 Rows per hash (MEASURED, job2, `e6_hash_rows_*.json`, `e6_stdlib_hash.json`)

The HASH chip is `PoseidonChip` of `midnight-circuits` 7.3.0 with `NativeChip`, configured as
`midnight-zk-stdlib` 2.4.0 configures them, on the leaf's shared advice columns 0..8. A bus provider
row `(HASH, id, in0, in1, out)` is copy-tied to the hash's inputs and output. Rows are leaf rows
with N hashes minus the leaf without them, on layout C (B and A give the same per-hash rows).

| Hash | Rows in the leaf | Stdlib relation (calibration) |
|---|---|---|
| 2 inputs, served on the bus | 21 (20 in the HASH region + 1 provider row) | 22 |
| 2 inputs | 21 | 22 |
| 3 inputs (a memory cell `(a, b, c)`) | 43 | 44 |
| 4 inputs | 44 | 46 |
| 8 inputs | 90 | |
| 31 inputs (state digest: 29 words, `mem_root`, `prog_root`) | 365 | 380 |
| 32 inputs | 366 | |

One permutation is 20 rows, as in the crate's golden file (`goldenfiles/cost-model.json`,
`one_permutation`). A sponge of n inputs takes `ceil(n / 2)` permutations. The `id` field of the bus
tuple is not absorbed here: absorbing it makes a 3-input hash (43 rows). Placing it in the capacity
element keeps 21 rows, but the stdlib API sets the capacity to the input length, so that needs a
custom chip (ESTIMATE: 21 rows).

### 3.2 Hashes per step (MEASURED census, `e0_windows.json`)

`~/.cache/chip2/scripts/e0_windows.py` over `e0_owner_runs.json` (E0, `20261110T161224Z-chip-job1`).
Under D7 a rewrite of the cached top costs no append, so appends are table appends plus frame and
task pushes.

| Run | Appends per step, uncached (mean, max) | D7 cached (mean, max) | D7, per 16 steps (aligned max, sliding max, mean) | D7, per 64 steps (sliding max) |
|---|---|---|---|---|
| fact(3) | 0.218, 1 | 0.184, 1 | 5, 5, 2.84 | 14 |
| fold | 0.199, 3 | 0.189, 3 | 5, 7, 2.85 | 16 |
| list123 | 0.201, 3 | 0.182, 3 | 8, 8, 2.89 | 18 |

At S = 16 no 16-step window of these runs appends more than 8 cells. The per-step maximum of 3 is
from three programs, not from the rule set (ESTIMATE as a bound). A capacity C per leaf is a liveness
bound only: when MEM is full the leaf closes early (design 2.7.7). The circuit pays the capacity,
not the mean, so the rows below are capacity rows divided by S.

### 3.3 Node cost of the HASH chip (MEASURED, job4)

| Leaf key at S = 16 | Advice | Permutation | Fixed | Node rows | k |
|---|---|---|---|---|---|
| B | 12 | 13 | 21 | 891,770 | 20 |
| B + stdlib HASH (any hash count from 2 to 1,970) | 12 | 15 | 36 | 999,488 to 999,500 | 20 |
| C + stdlib HASH | 25 | 28 | 31 | 1,166,203 | 21 |
| B + stdlib HASH, zero hashes (HASH fixed columns all zero) | 12 | 15 | 36 | 929,320 | 20 |
| B + Poseidon-only HASH key shape | 12 | 13 | 31 | 953,150 | 20 |
| C + Poseidon-only HASH key shape | 25 | 26 | 26 | 1,120,065 | 21 |
| A-inline + Poseidon-only HASH key shape | 51 | 7 | 23 | 1,135,670 | 21 |

The stdlib composition adds 15 fixed columns (9 `NativeChip` coefficient columns shared with the 8
Poseidon round-constant columns, the native fixed-constant column, and compressed selectors) and 2
permutation columns (the fixed-constant column and the committed instance column). Together they cost
107,718 node rows. The hash count does not matter: 2 hashes and 1,970 hashes cost the same within 12
rows. The run decider over the node keyed on B + stdlib HASH is 929,166 rows (k = 20), the same as in
E3, so the decider does not depend on the leaf.

The Poseidon-only shape (`hashchip::SlimHashShape`) configures `PoseidonChip` on advice columns 0..8
with its 8 round-constant columns and no `NativeChip`. A slim chip would copy the permutation's input
and output cells from the bus provider row, whose columns already allow equality. That adds no
permutation column. The shape cannot hash: `PoseidonChip::permutation` needs the native chip, and
the chip's selectors are private, so only the round-constant columns are filled (with nonzero
constants). It measures the node cost of such a chip, not its rows. A real slim chip is new circuit
code (a Poseidon region fed by the bus row) and needs its own soundness check and Gemini Quint lane.

Measured (job5): the Poseidon-only shape costs B 61,380 node rows (10 fixed columns, no
permutation column), against 107,718 for the stdlib composition. B + slim HASH is 953,150 rows,
3,150 over the gate. Its two Poseidon selectors are never enabled in the stand-in, so they are
all-zero columns. A real chip enables them, which may add up to about 12,000 rows (ESTIMATE: 2 x
6,000 per filled fixed column).

Fixed-column content matters. The same key shape as B + stdlib HASH, with no hash assigned (all HASH
fixed columns zero), costs 929,320 rows, against 999,488 when they are filled: 70,168 rows for
content alone. An all-zero fixed column commits to the identity, which the node resolves cheaply.
This probably explains E3's open item, where the node over S = 1 was 16,256 rows cheaper than over
S = 16 with the same constraint-system shape (not checked further). Measure the node over a key whose
fixed columns hold their real content.

Lookup arguments (MEASURED, job6). One, two and three extra lookup arguments add 22,672, 45,094 and
67,716 node rows: 22,547 to 22,672 rows each, on B with and without HASH. A lookup costs the node
about three advice columns. The leaf has 7: 4 RANGE limb columns, 2 bus lanes, DEC.

Ways under the gate with B + slim HASH (ESTIMATE from the measured unit costs):
- RANGE on 1 limb column with a running sum: 3 fewer lookups (-67,650), one more advice column for
  the running sum (+7,300). ALU goes from 3 to 12 rows per operation, so B goes from 17 to 26 rows
  per step (416 core and ALU rows at S = 16, still k = 13). Node about 893,000.
- RANGE on 2 limb columns (6 rows per operation): -45,100 + 7,300; node about 915,000.
- The placeholder digest region goes away when the state digest is a HASH request: 3 selectors, at
  most -18,000.
Each further bus lane costs about 22,500 node rows. Under B + slim HASH + 1-limb RANGE about 57,000
rows remain for PAGE, TRAP and more lanes: two more lanes at most.

### 3.4 `mem_root` options at S = 16 (rows MEASURED for the stated hash counts; the counts are derivations)

Notation: depth 20; a record hash is 3 inputs (43 rows); a compression is 2 inputs (21 rows); every
option also hashes `pre_d` and `post_d` (2 x 31 inputs, 730 rows). "HASH rows per step" is (leaf
rows - 291) / 16 on layout B.

| Option | Hashes per leaf | Leaf rows, B (C) | k | HASH rows per step | Walker rows per leaf (ESTIMATE) |
|---|---|---|---|---|---|
| Register file only (no HASH) | 0 | 291 (104) | 13 | 0 | 0 |
| Boundary digests only | 2 x 31-input | 1,021 (834) | 13 | 46 | 0 |
| M1: Merkle per append, capacity 8 per leaf | 8 records + 8 x 40 = 320 compressions + digests | 8,085 (7,898) | 13 | 487 | 320 |
| M1: Merkle per append, 3 per step provisioned (48 per leaf) | 48 records + 1,920 compressions + digests | 43,405 (43,218) | 16 | 2,695 | 1,920 |
| M2: batched once per leaf, any alignment, C = 8 | 8 records + 64 compressions + digests | 2,709 (2,522) | 13 | 151 | 64 + 40 alignment selects |
| M3: aligned segment, C = 8 | 8 records + 41 compressions + digests | 2,226 (2,039) | 13 | 121 | 41 |
| M3: aligned segment, C = 16 | 16 records + 47 compressions + digests | 2,696 (2,509) | 13 | 150 | 47 |

Derivations:
- M1. An append at `len` proves the old leaf empty (20 compressions from `EMPTY` to the old root) and
  computes the new root with the same siblings (20 more). The new root of one append is the old root
  of the next, but the next append's siblings differ, so each append pays 40. Per append: 43 + 40 x
  21 = 883 rows.
- M2. The old root is computed from the frontier of `base` (20 compressions; this authenticates the
  frontier against `mem_root` in `pre_d`). The new nodes over a range of 8 cells at any alignment
  number at most 5, 3, then 2 per level for levels 3 to 19, then the root: 43. The preset provisions
  64 = 20 + 44. A static circuit also needs a left/right select per level driven by the bits of
  `base` (ESTIMATE about 2 rows per level).
- M3. The leaf's MEM region is a fixed subtree of C = 2^c cells, so its shape is static: C record
  hashes and C - 1 compressions give the segment root. The segment root is appended at index
  `base / C` of a depth-(20 - c) top tree: 2 (20 - c) compressions (old empty slot and new root).
  C = 8: 7 + 34 = 41.
- A page-in of an old cell or a PROG row is unchanged by the choice: 1 record hash and 20
  compressions, 463 rows (study 2.3 estimated 500). The E0 bound of 6 control-word PROG rows per 16
  steps adds at most 2,778 rows: M3 plus these page-ins is 5,004 rows on B, still k = 13 (ESTIMATE).

Rows per step against the study: the study's unmeasured 200 to 750 HASH rows per step become 121
(M3, C = 8), 151 (M2) or 487 (M1 with capacity 8), all including the boundary digests (46 per step).
`k_leaf` stays 13 at S = 16 for every option except M1 provisioned per step (k = 16, at the limit).

### 3.5 Soundness of the options

- M1 is D3 as written. Each append updates the root from the old empty slot.
- M2 is D3 with one update per leaf. It covers exactly the rows `0 .. post.len - base - 1` of MEM and
  needs the per-level selects. Its soundness is the same as M1's: the frontier is authenticated by
  recomputing the old root.
- M3 (aligned segments) changes `len`. Where its state lives: no new register. `mem_root` stays one
  word in `pre_d` and `post_d`. `base` is a multiple of C, and at leaf close `len := base + C`, so the
  addresses of the unused MEM rows are allocated but never written. What changes:
  - D3 becomes: the segment root hashes all C MEM rows, and the rows from `post.len_used - base`
    to C - 1 are the zero tuple (MEM needs an active-prefix flag). A rule that reads a cell SHALL
    refuse the zero tuple. `addr < len` stays an integer comparison, but `len` now counts
    allocated, not written, cells.
  - PAGE opens an old cell at `a` with `c` levels in the segment and 20 - c in the top tree: 20
    compressions, as before. D2 is unchanged: PAGE serves `index < base`, MEM serves the segment.
  - Address space: 2^20 / C segments, so 2^17 leaves and 2,097,152 steps at S = 16, C = 8. Depth 24
    costs 8 more compressions per leaf (168 rows) and 4 more per page-in (84 rows).
  - The Init table's four `(root, length)` pairs (design 3.2 item 5, notes) need the four
    lengths kept as registers. They already are (`len_data` .. `len_slots`), so only the one-log
    root changes. This is not resolved here.
  - M3 is not Quint-modelled. Before adoption it needs the D3 clause above in `chipLeaf.qnt` with a
    mutant that reads a padding cell.
- The D2 alternative, "a memory tree that commits its length", is `mem_root = H(tree_root, len)`. It
  costs one compression to open in `pre_d` and one to close in `post_d` (42 rows per leaf). It lets
  PAGE check `index < len` against the committed length instead of relying on `base = pre_d.len`.
  The other alternative, distinct `OLD`/`NEW` tags on the bus, costs no hash. It costs one ALU
  comparison `addr < base` per read to choose the tag. Neither changes the counts above.
- Cheaper in principle but not adopted: keeping the 20-word frontier in the register file makes an
  append cost 1 + (trailing ones of the index) compressions, amortized 2. But PAGE still needs the
  root of `pre_d` every leaf (20 compressions), and the frontier adds 20 words to the digest
  (ESTIMATE 10 permutations, 200 rows, per digest), so it is not cheaper per leaf at C = 8.

### 3.6 Recommendation for item 2

M3 with C = 8, a Poseidon-only HASH chip and RANGE on one limb lookup. The HASH work is 41 compressions, 8 record hashes and 2
state digests per leaf. That is 2,226 leaf rows on B at S = 16 (k = 13), 121 HASH rows per step, plus
about 41 walker rows (ESTIMATE). Fallback if the aligned `len` is refused: M2 (151 rows per step).
M1 is 4x M3 and, provisioned per step, reaches k = 16. The node verdict with HASH is in section 3.3:
the stdlib composition fails (999,488). The
Poseidon-only shape fails by 3,150 (953,150). It passes with RANGE on one or two limb lookups
(ESTIMATE 893,000 or 915,000 from measured unit costs). Build the slim chip and the 1-limb RANGE,
then re-measure the node over the real key.

## 4. Proposed replacement text for design.md 2.7 (text only; design.md was not edited)

For 2.7.1, after the second sentence:

> The register file is laid out in pair rows. A register is a `(pre, post)` pair of cells in a
> 12-column row, and `post` is copy-constrained to the next slot's `pre`. Each pair row sits between
> a copy of the slot's request-lane row and a copy of its flag row, and an operand from another pair
> row is routed by a copy into a spare cell. A copy constraint is unconditional, so "unchanged unless
> named" is a gate over each pair. (Measured 2026-11-10, `experiments-2.md` §2: wide rows of 50
> columns cost the node 1,373,001 rows, k = 21; two staggered rows of 25 columns 1,058,485, k = 21;
> pair rows 891,770, k = 20.)

For 2.7.8 D3 (replace the bullet):

> - D3 The root covers appended rows only. MEM is one segment of C = 8 rows at addresses
>   `base .. base + 7`, with `base` a multiple of 8. The segment root hashes all 8 rows, and rows from
>   the first unused one onward are the zero tuple. The segment root is appended at index `base / 8`
>   of the top tree in `pre_d.mem_root`, and at leaf close `len := base + 8`. A read refuses the zero
>   tuple. [OPEN: Quint clause and a padding-read mutant; the Init lengths of 3.2 item 5 with one
>   log.]

For 2.7.9, after "Gates:":

> The node pays for the leaf's committed columns: about 7,300 rows per advice column, 8,200 per
> equality column, 6,000 per filled fixed column (fit over seven measured keys) and 22,600 per
> lookup argument (measured). An all-zero fixed column is nearly free, so the node is measured over
> a key with its real fixed content. Gate size is close to
> free (all core constraints repeated 4 times: +1,623 rows). A leaf change that adds a column SHALL be
> re-measured against the node gate.

For 2.7.10, replace the first "Design consequences" bullet:

> - The node's in-circuit verifier evaluates queries only at rotations -1, 0 and +1 (`midnight-circuits`
>   7.3.0 `verifier_gadget.rs:609`). The 32-word register file therefore uses pair rows with copy links
>   (2.7.1). Measured over the S = 16 leaf key (`experiments-2.md`): pair rows 891,770 node rows
>   (k = 20); wide rows 1,373,001 and staggered rows 1,058,485 (both k = 21). Every layout keeps
>   `k_leaf = 13` at S = 16. Regression tests fix the rotations of the spike and of every
>   register-file layout (`tests/e3_spike.rs`, `spike_leaf_vk_rotations_are_unit`,
>   `rf_leaf_vk_rotations_are_unit`).

For 2.7.10, replace "the depth-20 Merkle cost of `mem_root` per leaf; HASH and the task stack are not
yet modelled or measured" with:

> HASH is Poseidon (`midnight-circuits` 7.3.0): 21 leaf rows per 2-input hash, 43 per memory cell, 365
> per 31-input state digest (measured in the leaf, `experiments-2.md` §3). Per leaf at S = 16: 41
> compressions and 8 cell hashes for the segment (D3), 2 state digests. That is 2,226 leaf rows on
> pair rows (k = 13), 121 HASH rows per step, against the earlier ESTIMATE of 200 to 750. The stdlib
> composition of the chip (`NativeChip` and `PoseidonChip`) costs the node 107,718 rows (15 fixed, 2
> equality columns) and puts the node at 999,488 rows, over the gate. HASH SHALL be a Poseidon-only
> chip fed from its bus provider row. The Poseidon-only key shape measures 953,150 node rows, so
> RANGE SHALL use at most two limb lookups. A lookup argument costs the node about 22,600 rows, and
> one limb column brings the node to an ESTIMATED 893,000. The run decider over that node is 929,166 rows:
> it does not depend on the leaf. Page-ins: 463 rows each at depth 20. The task stack is modelled in
> the spike (D7 for tasks) [not in Quint].

## 5. Limits

- The six rules are spike rules over a 32-word file. They are not the C6 rules. Their purpose is to
  size the layouts. The 108-rule core may need more routed operands in B (spare cells cost no node
  rows) or a third bus lane (a lookup and its columns; unmeasured).
- The digest is the spike's linear placeholder, copied into a digest region. With HASH it becomes
  two 31-input Poseidon sponges over the same copies, which the presets include as extra hashes but
  do not wire to the registers.
- The hash presets put unrequested HASH rows in the leaf (the bus allows them). No Merkle walker,
  PAGE, TRAP or alignment select is built. Their rows are ESTIMATE.
- `top` and `task` are 2 words each. A 13-lane frame that needs 3 words needs a 6-wide bus tuple (5
  more lane cells, one more column if the lanes no longer fit the 12) or two cells per push.
- New circuit code here is a spike. A HASH chip, the pair-row core or M3 that is meant to become real
  needs a Gemini Quint lane (not run).
