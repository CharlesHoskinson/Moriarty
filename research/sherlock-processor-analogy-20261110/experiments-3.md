# Chip leaf experiment E7: a real HASH chip under the node gate

Run `sherlock-processor-analogy-20261110`, 2026-11-10, third session. This continues
`experiments-2.md` §3.3 and §3.6. The previous session measured layout B (pair rows) under the node
gate at 891,770 rows without HASH. Every measured key with HASH failed: B with the stdlib Poseidon
composition at 999,488, and B with a Poseidon-only key-shape stand-in at 953,150. Here a real
Poseidon-only HASH chip and RANGE on one limb lookup are built and measured over their real keys.

Code: `crates/mori-chip/src/poseidon.rs` (HASH chip), `src/range.rs` (running-sum mode),
`src/alu.rs`, `src/rfcore.rs` (parameters `pos_hash`, `range1`, probe `extra_selectors`), tests in
`tests/e3_spike.rs` (presets `pos_<preset>`, suffixes `+r1`, `+xs<n>`; test `chip_e7_shapes`). Commit
`e8aad0eeb`. Outputs: `~/.cache/chip3/out/*.json`, scripts `~/.cache/chip3/scripts/`, logs
`~/.cache/chip3/logs/`, run records in `qa/sherlock/runs.jsonl`.

Labels: MEASURED is a `cost_model`, a key or a test result from a run named here. ESTIMATE is
arithmetic on measured numbers. All rows are `cost_model` rows over keys made on an unsafe SRS (rows
do not depend on the SRS). No proof was made.

| Run | Content | Exit |
|---|---|---|
| `20261110T215653Z-chip3-job1` | script path missing; nothing ran | 127 |
| `20261110T215704Z-chip3-job1b` | build; unit tests: 53 pass, 3 fail (`q_prov` not assigned on non-provider absorb rows) | 0 (tests failed) |
| `20261110T215744Z-chip3-job1c` | unit tests after the fix: 56 pass | 0 |
| `20261110T215809Z-chip3-job2` | keyed rotation guards; key shapes; node over B, B + HASH, B + HASH + 1-limb RANGE, B + 1-limb RANGE | 0 |
| `20261110T220102Z-chip3-job3` | node over the final key with 0, 2 and 4 extra selector columns; node keygen and run decider over the final node | 0 |
| `20261110T221020Z-chip3-job4` | after `cargo fmt`: 56 unit tests; e3_spike non-ignored tests (rotation guards, S = 16 MockProvers): 4 pass | 0 |
| `20261110T221047Z-chip3-t0` | T0 on the Rust commit: 8 pass, 1 skip | 0 |

## 1. Results

| Gate (design 2.7.9) | Limit | Result |
|---|---|---|
| Node, B (reproduced) | 950,000 rows, k = 20 | 891,770, k = 20 (equal to experiments-2) |
| Node, B + Poseidon-only HASH chip (step 1) | 950,000 rows, k = 20 | 943,466, k = 20: PASS, 6,534 headroom |
| Node, B + HASH chip + RANGE on 1 limb lookup (steps 1 and 2; final key) | 950,000 rows, k = 20 | **888,466, k = 20: PASS, 61,534 headroom** |
| Run decider over the final node | 1,000,000 rows, k = 20 | **929,166, k = 20: PASS** (70,834 headroom) |
| `k_leaf` at S = 16, final key | 16 | 13 (6,665 rows) |
| Keyed rotations, final key and every layout with the new options | -1, 0, +1 | -1, 0, +1 |

Steps needed: 1 and 2. Step 1 alone passes with 6,534 rows of headroom, which is not a margin (one
filled fixed column costs about 6,000). Step 2 brings the headroom to 61,534. That is more than the
45,000 that experiments-2 called a clear margin, so the work stopped there.

Step 3 (M3 aligned segment with `mem_root`) was **not built as wiring**. The final key carries the M3
C = 8 hash workload: 41 bus-served 2-input hashes, eight 3-input sponges and two 31-input sponges.
But the sponges absorb free inputs, not copies of the MEM rows and registers. There is no Merkle
walker, no path select and no index bits, and the state digest is still the placeholder (§4). Rows
and k include the full M3 workload. The node rows do not include the walker's selectors. §3.3 bounds
that cost with a measured probe.

## 2. The HASH chip

### 2.1 Design

`poseidon.rs` hashes with the permutation of `midnight-circuits` 7.3.0, the one `midnight-zk-stdlib`
2.4.0 composes. Width 3, rate 2, S-box `x^5`, 8 full and 60 partial rounds, the crate's MDS matrix and
round constants (public through `PoseidonField`), in the crate's shifted form. The sponge is the
stdlib fixed-length sponge: the capacity starts at the input length, inputs are added two at a time,
one permutation per chunk, and the output is state element 0. The stdlib chip's selectors, its
`permutation` and its partial-round skip identities are private (`pub(crate)`), so the gates are new
code. The output is the same function: `hash_native` (stdlib `PoseidonChip::hash`) is the reference
in every test.

The layout is chosen for the node, which pays per committed column:

- No `NativeChip`: no coefficient columns, no constant column, no instance equality.
- No partial-round skips: 3 round-constant columns instead of 8.
- No S-box hints: a round is one gate of degree 6, the leaf's degree already.
- State in shared advice columns 0..3. A permutation is 70 rows: an absorb row, 68 round rows and
  an output row.

Gates (5 selectors, 3 round-constant fixed columns, no new advice or equality column):

- `hash full round`, `hash partial round`: `next = MDS * sbox(s) + rc`, where `sbox` raises all
  three elements (full) or element 2 only (partial) to the fifth power.
- `hash absorb first`: the next state is `(in0, in1, 0) + rc`. On the absorb row the fixed column
  holds `RC[0] + (0, 0, len)`, so the capacity is fixed by the circuit, never by the prover. On a
  provider row (`q_prov = 1`) the gate also pins `chip = HASH` and `key = 2`. A provider serves only
  2-input hashes, so its tuple `(HASH, 2, in0, in1, out)` is a function of `(in0, in1)`, as `bus.rs`
  requires. The E6 provider rows of experiments-2 pinned neither.
- `hash absorb chain` and `hash absorb tail`: the next state is `prev + (in0, in1, 0) + RC[0]`, where
  `prev` is the previous permutation's output row (the row above). The tail form absorbs `in0` only
  (an odd last chunk, as the stdlib does).
- `out` of a provider row is copy-constrained to element 0 of the output row. Inputs given as cells
  are copied into `in0`, `in1`. Columns 0, 2, 3 and 4 are already equality-enabled in B.

### 2.2 Soundness evidence (MEASURED; job1c, job4; `poseidon::tests`)

- `differential_cpu`: the chip's CPU form equals the stdlib sponge on 4 random inputs of every
  length 1 to 40 (seeded ChaCha20).
- `differential_circuit`: in MockProver, random sponges of 1, 2, 3, 4, 8 and 31 inputs. Their
  inputs are copied from cells, and their outputs are tied to an instance column holding
  `hash_native(inputs)`. Four random bus-served hashes serve raw request rows carrying the stdlib
  output. All pass.
- Negatives, each asserting the name of the gate that fails:
  - `tampered_state_rejected`: one state cell is changed and the later rows are recomputed from it,
    so only the producing gate can catch it. All three columns at rows 2, 5, 6, 40, 65, 66 and 69
    are refused by the full or partial round gate that produced the cell, and never by an absorb
    gate. A tampered cell in the second permutation of a 3-input sponge is refused by the partial
    round gate. A tampered first state after a chain absorb is refused by `hash absorb chain`.
  - `wrong_round_constant_rejected`: one fixed constant is changed by +1 and the witness keeps the
    true constant. Every column at round rows 1, 4, 5, 64, 65 and 68 (including the last round,
    whose constant is zero) is refused by that round's gate. The absorb row (capacity and `RC[0]`)
    is refused by `hash absorb first`.
  - `skipped_absorb_rejected`: an absorb row that does not add its inputs to the state is refused by
    `hash absorb first` (2 inputs), `hash absorb chain` (4 inputs, permutation 1; 31 inputs,
    permutation 9) and `hash absorb tail` (3 inputs; 31 inputs, permutation 15).
  - `provider_key_is_pinned`: a provider row with key 5, requested with key 5, is refused by
    `hash absorb first`.
  - Also refused: a request with a wrong output (lookup failure), a request with swapped inputs, and
    a sponge output that differs from the instance.
- In the leaf: `pos_hash_and_range1_leaves_pass` (MockProver, B, S = 4, 3 bus-served hashes and
  sponges of 3 and 31 inputs, with and without the 1-limb RANGE). The existing leaf tests and
  `rf_leaf_s16_mock_prover_all_layouts` still pass.
- Rotations: `rotations_are_unit` (configured constraint system, every layout with `pos_hash`,
  `range1` and both) and `rf_leaf_vk_rotations_are_unit` (keyed, presets `pos_m3c8`, `pos_m3c8+r1`
  and `none+r1` added for every layout) give -1, 0, +1 only (job2, job4).
  `spike_leaf_vk_rotations_are_unit` passes.

The circuit is a spike. It has no Quint model and no Gemini Quint lane was run (§5).

### 2.3 Rows (MEASURED, job2, `e7_shapes.json`)

One permutation takes 70 leaf rows, against 20 for the stdlib chip (experiments-2 §3.1). A
2-input hash takes 70 rows, a 3-input hash 140 and a 31-input digest 1,120. The M3 C = 8 workload
per leaf is 41 + 8 x 2 + 2 x 16 = 89 permutations, 6,230 rows: 389 HASH rows per step at S = 16,
against 121 with the stdlib chip. The rows cost the leaf, not the node: k stays 13.

## 3. Node and decider (MEASURED unless marked)

### 3.1 Keys at S = 16

| Leaf key (spec) | Advice | Permutation | Fixed | Lookups | Leaf rows | k | Node rows | k |
|---|---|---|---|---|---|---|---|---|
| B (`narrow,1,none`) | 12 | 13 | 21 | 7 | 291 | 13 | 891,770 | 20 |
| B + HASH (`narrow,1,pos_m3c8`) | 12 | 13 | 29 | 7 | 6,521 | 13 | 943,466 | 20 |
| B + HASH + 1-limb RANGE (`narrow,1,pos_m3c8+r1`), final | 12 | 13 | 31 | 4 | 6,665 | 13 | 888,466 | 20 |
| B + 1-limb RANGE (`narrow,1,none+r1`) | 12 | 13 | 23 | 4 | 435 | 13 | 836,770 | 20 |
| final + 2 filled selector columns (`+xs2`) | 12 | 13 | 33 | 4 | 6,665 | 13 | 896,019 | 20 |
| final + 4 filled selector columns (`+xs4`) | 12 | 13 | 35 | 4 | 6,665 | 13 | 898,137 | 20 |

Earlier keys from experiments-2, for comparison: B + stdlib HASH 999,488 (12/15/36/7); B +
Poseidon-only stand-in 953,150 (12/13/31/7, two selectors all zero).

Run decider over the node keyed on the final leaf: 929,166 rows, k = 20 (node keygen 35.3 s). It
is the same as in E3 and experiments-2, as expected: the decider does not depend on the leaf.

### 3.2 What each step cost

- HASH chip: +51,696 node rows for 8 filled fixed columns (3 round-constant and 5 selector columns),
  6,462 per column. The stand-in of experiments-2 was called a floor, but the real chip is 9,684 rows
  below it, because it has 3 round-constant columns, not 8. The stdlib composition cost 107,718.
- RANGE on 1 limb lookup: -55,000 node rows on B alone (891,770 to 836,770) and on B + HASH
  (943,466 to 888,466), for 3 fewer lookup arguments and 2 more selectors. ESTIMATE: with about
  6,000 per selector, a lookup argument is about 22,300 rows, in line with the 22,547 to 22,672
  measured by adding lookups. No new advice column was needed: the running sum uses column 10, which
  is free on ALU rows. The experiments-2 ESTIMATE of +7,300 for that column does not apply. The ALU
  goes from 3 to 12 rows per operation and B from 17 to 26 rows per step (435 rows at S = 16).
- Selectors are not compressed. Every selector of the leaf is its own fixed column in the key (B: 21
  fixed columns, B + HASH: 29 = 21 + 3 + 5). The 5 HASH selectors and the 2 RANGE selectors are
  therefore direct node costs.

### 3.3 Headroom for what is not built (ESTIMATE from the probe)

Two extra filled selector columns cost 7,553 rows and four cost 9,671. The probe's selectors are
enabled on every 7th core row only, and its per-column cost is below the 6,000 to 6,500 that the
HASH columns cost. For a planning bound, use about 6,500 per filled fixed column (the HASH figure).
M3 wiring needs about 4 selectors: the path select with the bit check and the shift-right index
accumulator `2 acc_t = acc_(t-1) + b_t 2^17`, the first level, the `EMPTY` segment constant, and
`8 acc = base`. The MEM active-prefix flag needs about 2 more. Together that is about 39,000 node rows
(ESTIMATE). It leaves about 22,500 under the gate for PAGE, TRAP and the real state digest.
Replacing the placeholder digest region with HASH requests removes its 3 selectors (about +19,500
back). A further bus lane costs a lookup, about 22,500. So after M3, at most one more lane fits.

Cuts not needed now, but available if a later chip breaks the gate:
- One absorb selector instead of three: the chain flag can go into a fourth round-constant value or
  be removed with a pinned zero cell. -2 columns, about -13,000 (ESTIMATE).
- Merge the 7 core gate selectors of B, whose gate rows are disjoint, into fewer fixed columns with
  explicit encoded values (selectors are not compressed by the keygen). Each column saved is about
  6,000 (ESTIMATE). The degree budget is 6, so this works for the low-degree core gates and not
  for the round gates.
- DEC merged into the bus table (-1 lookup, about -22,500; ESTIMATE, not measured).

## 4. Limits

- The 3- and 31-input sponges of the measured key absorb free advice values, and the 41 bus-served
  hashes have no requester in the core. Copying real inputs adds no column, since the source columns
  are equality-enabled, so it should not move the node (not measured). The M3 walker, path selects,
  index bits, the `EMPTY` constant, the MEM active-prefix flag and `len := base + 8` at leaf close are
  not built (§3.3). M3 itself is still not Quint-modelled (experiments-2 §3.5).
- The state digest is still the linear placeholder region. `pre_d` and `post_d` in the instance are
  not Poseidon digests.
- The 31-input digest takes 1,120 rows. With page-ins (each 1 record and 20 compressions, 22
  permutations, 1,540 rows) at the E0 bound of 6 per leaf, the leaf is about 15,900 rows, k = 14
  (ESTIMATE), still under `k_leaf <= 16`.
- The HASH chip and the running-sum RANGE are spike code. Before adoption they need a Quint model
  (absorb and capacity rules, provider function property), a Gemini Quint lane and a review. Neither
  was run.
- All node numbers are `cost_model` rows over keys on an unsafe SRS. They are not proofs.

## 5. Proposed replacement text for design.md 2.7 (text only; design.md was not edited)

For 2.7.1, after the sentence on pair rows proposed in experiments-2 §4:

> HASH is a Poseidon-only chip on the shared advice columns. Its permutation and sponge are those of
> `midnight-circuits` 7.3.0 (width 3, rate 2, 8 full and 60 partial rounds, capacity set to the
> input length), so its digests equal the stdlib Poseidon. Its layout is not the stdlib one: one
> degree-6 gate per round, no partial-round skips, no `NativeChip`, 3 round-constant columns, 70 rows
> per permutation. A provider row `(HASH, 2, in0, in1, out)` is the absorb row of a 2-input hash. The
> absorb gate pins `chip` and `key`, and `out` is copied from the output row. Wider hashes chain
> absorb rows, and the capacity is a fixed value of the first absorb row. RANGE uses one limb lookup
> with a running sum (12 rows per 128-bit value).

For 2.7.9, after the text on node costs proposed in experiments-2 §4:

> Selectors are not compressed by keygen: every selector is a fixed column of the leaf key and costs
> the node about 6,000 to 6,500 rows when filled. Measured 2026-11-10 (`experiments-3.md`): the leaf
> with pair rows, the Poseidon-only HASH chip carrying the M3 C = 8 hash workload and one-limb RANGE
> (12 advice, 13 equality, 31 fixed columns, 4 lookups; 6,665 rows, k = 13 at S = 16) puts the node at
> 888,466 rows (k = 20) and the run decider at 929,166 (k = 20). Both gates pass, with 61,534 node
> rows of headroom. A leaf change that adds a selector, a fixed column or a lookup SHALL be
> re-measured against the node gate.

For 2.7.10, replace the HASH paragraph proposed in experiments-2 §4 with:

> HASH is Poseidon. The stdlib composition (`NativeChip` and `PoseidonChip`, 20 rows per
> permutation) costs the node 107,718 rows and puts it at 999,488, over the gate. The Poseidon-only
> chip of 2.7.1 (70 rows per permutation) costs the node 51,696 rows: 943,466 on pair rows. RANGE on
> one limb lookup removes 55,000 more, giving 888,466. Per leaf at S = 16 the M3 C = 8 workload is
> 41 compressions, 8 cell hashes and 2 state digests, 89 permutations, 6,230 rows (389 HASH rows per
> step); `k_leaf` stays 13. The M3 walker and the MEM active-prefix flag are not built. Their
> selectors are ESTIMATED at about 39,000 node rows, inside the headroom. [OPEN: M3 wiring and its
> Quint clause; the Poseidon state digest in place of the placeholder; a Quint model and a Gemini
> Quint lane for the HASH chip and the running-sum RANGE.]
