# Chip leaf experiment E8: selector cuts and the M3 memory commitment

Run `sherlock-processor-analogy-20261110`, 2026-11-10, fourth session. This continues
`experiments-3.md` §3.3 and §4. E7 left the leaf at 888,466 node rows. That key carried the M3 C = 8
hash count, but its sponges hashed free inputs: there was no Merkle walker, no active-prefix flag, no
aligned allocation, and the state digest was a linear placeholder. Here three selector cuts are measured
one by one, and then M3 is wired for real, together with a Poseidon state digest. The node and the run
decider are measured over the final real key.

The Gemini Quint lane on layout B and M3 (`qa/gemini-quint/chipleaf-pairrow-m3/verdict.md`, branch
`worktree-agent-abe0ec92c1cc58c8f`) arrived during this session. It found M3 sound only with its
constraints C1 to C6. C1 to C5 are built in, and each has a MockProver test (§3). C6 is reported in §4.

The Gemini Quint lane on the HASH chip (`qa/gemini-quint/chipleaf-hash/verdict.md`, on `main`) also
arrived during this session. Its conditions H-C1 to H-C6 are handled in §3.5.

Code: `crates/mori-chip/src/vsel.rs` (packed selectors), `src/m3.rs` (records, segment root, walker,
digests), `src/mem.rs` (`SegMemConfig`), and `src/poseidon.rs`, `src/rfcore.rs`, `src/rf.rs`,
`tests/e3_spike.rs`. Commits `6ad01ddee` (cuts and M3), `432729244` (degree print), `329c4ddef` (review
follow-up tests) and `b2b4ffce7` (HASH verdict). The commits after `6ad01ddee` change tests, test knobs
and configure-time asserts only; the measured keys are unchanged. New `RfParams`: `one_abs` (cut 1), `pack_core` (cut 2), `m3`.
New preset suffixes in `e3_spike.rs`: `+a1`, `+pc`, and the preset `pos_m3`. Scripts are in
`~/.cache/chip4/scripts/`, logs in `~/.cache/chip4/logs/` and outputs in `~/.cache/chip4/out/`. The run
records are in `qa/sherlock/runs.jsonl`.

Labels: MEASURED is a `cost_model`, a key or a test result from a run named here. ESTIMATE is arithmetic
on measured numbers. All node and decider rows are `cost_model` rows over keys made on an unsafe SRS
(rows do not depend on the SRS). No proof was made.

| Run | Content | Exit |
|---|---|---|
| `20261110T224403Z-chip4-check1` | `cargo check`: 5 compile errors | 0 (check failed) |
| `20261110T224433Z-chip4-check2` | `cargo check` after the fixes | 0 |
| `20261110T224743Z-chip4-test1` | unit tests: 2 fail (`m3_leaves_chain`, `m3_append_skipped_rejected`; test errors, §3.3); e3_spike non-ignored: 6 pass, including `rf_leaf_s16_mock_prover_final_e8` and the rotation guard | 0 (unit tests failed) |
| `20261110T224900Z-chip4-unit1` | unit tests: the same 2 fail | 0 (tests failed) |
| `20261110T224932Z-chip4-unit2` | unit tests after the test fixes: 71 pass | 0 |
| `20261110T225006Z-chip4-unit3` | unit tests, one thread, failing gates printed (`E8-NEG` lines): 71 pass | 0 |
| `20261110T225040Z-chip4-node1` | node over 5 keys (E7 baseline, cut 1, cut 2, M3 without cuts, final); node keygen and run decider over the final node | 0 |
| `20261110T230036Z-chip4-final` | after `cargo fmt`: 71 unit tests; 6 e3_spike non-ignored tests; clippy | 0 |
| `20261110T230131Z-chip4-t0` | T0 on the Rust commit: 8 pass, 1 skip | 0 |
| `20261110T230612Z-chip4-degs` | `e8_degree_and_columns` with the core gate degrees printed | 0 |
| `20261110T230640Z-chip4-t0b` | T0 on the follow-up commit `432729244` (the degree print): the Rust and format checks pass; the factory unit test `test_aggregate_kill_cgroup_slice` (doctorwatson cgroup kill, not mori-chip) fails. The commit was made after this run by a script error (the commit did not wait for the T0 exit code) | 1 |
| `20261110T230940Z-chip4-t0doc` | T0 on this document; the factory unit tests, `test_aggregate_kill_cgroup_slice` included, pass | 0 |
| `20261110T231556Z-chip4-unit4` | after a review: the core negatives also on packed selectors (every layout), every register word on the final key, the generic walker gate, and padding left on the bus: 73 unit tests pass | 0 |
| `20261110T231725Z-chip4-fmt`, `20261110T231725Z-chip4-t0c` | `cargo fmt`; T0 on the follow-up tests (commit `329c4ddef`): 8 pass, 1 skip | 0 |
| `20261110T231943Z-chip4-unit5` | after the HASH verdict changes (§3.5): 75 unit tests pass | 0 |
| `20261110T232039Z-chip4-spike3` | e3_spike non-ignored: 6 pass | 0 |
| `20261110T232101Z-chip4-t0d` | T0 on the HASH verdict changes (commit `b2b4ffce7`): 8 pass, 1 skip | 0 |
| `20261110T232314Z-chip4-t0doc2` | T0 on this document's update | 0 |

## 1. Results

| Gate (design 2.7.9) | Limit | Result |
|---|---|---|
| Node over the final real key | 950,000 rows, k = 20 | **823,466, k = 20: PASS, 126,534 headroom** |
| Run decider over that node | 1,000,000 rows, k = 20 | **929,166, k = 20: PASS** (70,834 headroom) |
| `k_leaf` at S = 16, final key | 16 | 13 (6,511 rows) |
| Keyed rotations, every layout, presets with `+a1`, `+pc` and `pos_m3` | -1, 0, +1 | -1, 0, +1 |

The final key is `narrow,1,pos_m3+r1+a1+pc`: layout B, the Poseidon-only HASH chip with one absorb column,
packed core selectors, RANGE on one limb lookup, and M3 wired. Its leaf has 12 advice columns, 13
equality columns (12 advice and 1 instance; unchanged), 20 fixed columns, 4 lookups, degree 6 and 22
gates. The headroom of 126,534 rows is more than the 40,000 margin that was asked for. DEC was not folded
into the bus (§2.3).

## 2. The cuts (MEASURED, node1)

### 2.1 Per cut

| Leaf key (spec) | Fixed | Leaf rows | Node rows | Step |
|---|---|---|---|---|
| E7 final, `pos_m3c8+r1` (baseline, reproduced) | 31 | 6,665 | 888,466 | |
| + one absorb column, `+a1` (cut 1) | 29 | 6,665 | 875,763 | -12,703 |
| + packed core selectors, `+a1+pc` (cut 2) | 23 | 6,665 | 842,708 | -33,055 |
| M3 wired, no cuts, `pos_m3+r1` | 28 | 6,511 | 863,888 | -24,578 from the baseline |
| M3 wired with both cuts, `pos_m3+r1+a1+pc` (final) | 20 | 6,511 | 823,466 | -65,000 from the baseline |

All keys have 12 advice columns, 13 equality columns, 4 lookups, degree 6 and k = 13 at S = 16. The
fixed counts (including table columns) were also checked by `e8_degree_and_columns` on the unkeyed
constraint system: 31, 29, 23, 20.

### 2.2 How each cut works

The keygen of `midnight-proofs` 0.8.3 converts every selector into its own fixed column
(`directly_convert_selectors_to_fixed`). A gate whose rows are disjoint from other gates can share a fixed
column with them. The column holds a code `c` in `1..=n` on the gate's rows and 0 elsewhere. The gate is
multiplied by the Lagrange polynomial that is 1 at `c` and 0 at the other values of `0..=n`. That
polynomial has degree `n`, so a gate of body degree `d` fits a column of `n` codes when `n + d <= 6`. The
codes are fixed values of the circuit, so a packed selector is as sound as a plain one
(`src/vsel.rs`; `lagrange_is_an_indicator` checks the indicator for `n <= 5`).

- Cut 1, one absorb column: the selectors `hash absorb first`, `chain` and `tail` become codes 1, 2 and 3
  of one column. Each absorb gate keeps its name and its body (degree 1). The indicator has degree 3, and
  the provider pins `q_first * q_prov * (chip - HASH)` reach degree 5. -2 fixed columns, -12,703 rows
  (6,352 per column). This is in line with the ESTIMATE of -13,000. Every HASH negative of E7 runs in both
  modes and must fail in the same gates (`poseidon::tests::run`).
- Cut 2, packed core: the body degree of each core gate row is read from a scratch constraint system
  (`core_degrees`). On layout B the gate rows 1, 3, 5, 7, 9, 11 and 12 of a slot hold 23, 8, 6, 6, 6, 2
  and 2 constraints, of body degree 2, 3, 2, 2, 1, 1 and 2 (`e8_degree_and_columns`, run
  `20261110T230612Z-chip4-degs`). `rf count0` has degree 1. The gate on the DEC row (slot row 1) uses the
  existing complex selector `q_dec`, which is enabled on the same rows. The other 6 gates and `rf count0`
  go into code columns by first fit, largest degree first: one column of 3 codes (degrees 3, 2, 2) and one
  of 4 codes (degrees 2, 1, 1, 1). 8 selector columns are replaced by 2 code columns: -6 fixed columns,
  -33,055 rows. Per removed column this is 5,509. E7 found that equal commitments seem to be shared by the
  node, and the DEC-row gate selector had the same rows as `q_dec`. So 5 distinct commitments went away,
  6,611 rows each.
- M3 replaced 6 selector columns with 3: the 3 placeholder-digest selectors and the 2 old MEM selectors
  went, and the MEM segment code column and the walker code column came in. That is -3 fixed columns,
  -24,578 rows on the uncut key. Its 22 gates are 4 more than E7's 18. The gates themselves are close to
  free.

### 2.3 Not done

- DEC into the bus table: not needed, because the headroom is 126,534 without it. It also does not
  fit as stated: both bus lanes of the request row are in use, so DEC would need a third lane, which is a
  lookup argument too. ESTIMATE if needed later: -22,500 only with a free lane.
- Further packing is available. RANGE `q_first` and `q_step` (degree 1) fit one code column (-1, about
  6,500 ESTIMATE). The bus, RANGE and DEC lookup selectors must stay degree 1: the lookup degree is
  `2 + input + table`.

## 3. M3 wired (MEASURED; `src/m3.rs`, `src/mem.rs`)

### 3.1 What the circuit does

- MEM segment: 8 rows at `base .. base + 8`. Row columns are: 0..5 the bus tuple, 5 the address, 6
  `len_end`, 7 `act`. One code column holds codes first / middle / last.
  - `mem row`: `act` is boolean; `chip = act * MEM`, `key = act * addr`, `(1 - act) * x = 0` for the
    data words. A padding row puts the zero tuple on the table side (D8).
  - `mem chain`: consecutive addresses, `len_end` constant, `act` non-increasing, and where the prefix
    ends, `addr + 1 = len_end`.
  - `mem first`: when row 0 is padding, `addr = len_end`. `mem last`: when row 7 is active,
    `addr + 1 = len_end`.
  - Row 0's address is a copy of slot 0's LEN cell (`base`). `len_end` is a copy of slot S's LEN cell, the
    core's running `len`. So the active rows are exactly the addresses below `len_end`.
- Records: `H(act, a, b, c)`, a 4-input sponge (2 permutations). Its inputs are copies of the MEM row
  cells. `act` is the record's tag word: it is 1 on every written record and 0 on padding.
- Segment root: 7 compressions over the 8 records, with the inputs copied from the outputs below.
- Top tree (depth 17): an empty segment slot is 0, and an empty node of level `l + 1` is `H(E_l, E_l)`. The
  old walk starts at 0 and gives `pre.mem_root`. The new walk starts at the segment root and gives
  `post.mem_root`. The new walk's sibling, bit and accumulator cells are copies of the old walk's, so both
  walks use one path. The walker gate sits on the absorb row of each compression, as a code column with 3
  codes:
  - `m3 walk` (every level): `b` is boolean, `(in0, in1) = b ? (s, x) : (x, s)`,
    `acc_in = 2 acc_out + b`;
  - `m3 walk first` (old walk, level 0): `x = 0`, `8 acc_in = base`, `len_post = base + 8`;
  - `m3 walk last` (old walk, level 16): `acc_out = 0`.

  Level `l + 1` copies `acc_out` of level `l`, so `base / 8 = sum b_l 2^l < 2^17`. `base` is therefore a
  multiple of 8 and below 2^20, and the bits that steer the walks are the bits of `base / 8`.
- State digest: `H(w_0, .., w_28, mem_root)`, 30 inputs (15 permutations), every input a copy. `pre_d`
  takes slot 0's register cells and the old root. `post_d` takes slot S's register cells, except LEN: LEN
  is the close cell `len_post` of the walker. It also takes the new root. The placeholder digest region is
  not configured when `m3` is set.
- Genesis: `len = 8` (`rf::SEG_C`). The first segment, which contains `NIL = 0`, is reserved and never
  written.

HASH work per leaf: 8 records x 2 + 7 + 34 + 2 x 15 = 87 permutations, 6,090 rows. The leaf at S = 16 is
6,511 rows, k = 13. That is 154 fewer than E7, because the MEM region has 8 rows, not 16, the digest
region is gone, and the digests have 30 inputs, not 31.

### 3.2 Positive tests (MEASURED, unit3, final, test1)

- `rf_leaf_s16_mock_prover_final_e8`: the final key at S = 16 with a witness (16 steps, 6 appends, 2
  padding rows) passes MockProver. `circuit_model`: 6,511 rows, k = 13.
- `m3_honest_leaves_pass`: bases 8, 40 and 2^19, a leaf with 0 steps, and partial leaves. The cuts without
  M3 pass on layouts A, B and C.
- `m3_leaves_chain`: two leaves in sequence. The second leaf's `pre_d` equals the first leaf's `post_d`:
  `len = base + 8`, and the new root is the next old root, with a non-empty sibling at level 1. Both
  leaves pass MockProver.
- Every E5/E6/E7 test still passes (75 unit tests after unit5; 6 e3_spike tests), with genesis
  `len = 8`. The core tests `honest_leaves_pass`, `every_register_word_is_checked`,
  `push_link_is_the_core_fp`, `enable_is_a_prefix`, `narrow_copies_are_checked`,
  `wrong_digest_statement_rejected` and `rotations_are_unit` run on every layout with plain and with packed
  core selectors (unit4).

### 3.3 Negatives (MEASURED, unit3; the failing gate or argument of each refused witness)

Each M3 negative recomputes the public statement from the forged witness, so that only the circuit can
refuse it.

| Attack | Test | Refused by |
|---|---|---|
| Append not requested (row 6 marked active with data, `len_end` unchanged) | `m3_append_not_requested_rejected` | `mem chain` (no lookup failure) |
| Append skipped (last append's row zeroed and marked padding) | `m3_append_skipped_rejected` | `mem chain`, lookup (the append request is not served) |
| Append skipped, data kept | same | `mem row`, `mem chain`, lookup |
| Non-zero padding (each data word of row 7) | `m3_nonzero_padding_rejected` | `mem row` |
| Read of a padding cell: as the zero tuple at `len_end` and at row 7, and as a non-zero tuple | `mem::tests::seg_padding_cell_is_unreadable` | lookup |
| `len_end` off by one at the prefix's end (none active, all active, 3 active) | same | `mem first`, `mem last`, `mem chain` |
| `len`/root desync: close cell `len_post = 24` at base 8 | `m3_len_root_desync_rejected` | `m3 walk first` |
| Unaligned base (genesis `len = 9`) | same | `m3 walk first` |
| Walk bits of another segment (idx 3 at base 8) | same | `m3 walk first` |
| An 18-bit index (slot outside the top tree) | same | `m3 walk first`, `m3 walk last` |
| `len_end` on MEM rows is not slot S's `len` | same | `mem chain`, permutation |
| Forged sibling in the new walk only (levels 0, 1, 9, 16) | `m3_forged_sibling_rejected` | permutation (the shared sibling's copy), no walker gate |
| Forged sibling in both walks (same levels) | same | permutation (the old root is not `pre_d`'s root) |
| Digest not binding a register: each of the 29 words and `mem_root`, in `pre_d` and `post_d` (60 cases) | `m3_digest_binds_every_word` | permutation (the input's copy) |
| A written zero tuple marked padding (verdict C2) | `m3_zero_record_is_tagged` | `mem chain`, `mem first`, lookup |
| A padding row that keeps `chip = MEM`, `key = addr` on the bus, with a request for it | `mem::tests::seg_padding_cell_is_unreadable` | `mem row` (the request is served, so no lookup failure) |
| A select that does not follow the bit (`in0 + 1`), old and new walk, levels 0, 5, 16; statement recomputed from the forged walk | `m3_walk_gate_rejects_bad_select_and_bit` | `m3 walk` only |
| A non-boolean bit: idx 1 as bits `(-1, 1, 0, ..)`, select and accumulators consistent | same | `m3 walk` only |
| Any of the 32 register words wrong after the last step, final key (packed selectors, M3) | `m3_every_register_word_is_checked` | `rf core` (word 16, `len`, also permutation through `len_end`) |

The negatives that print their failing gates ran in unit3 and unit4 (`E8-NEG` lines in
`~/.cache/chip4/logs/unit.log`). The read-of-padding negatives are chip-level (the segment with raw bus
requests), not in the leaf: in the leaf a read below `len` cannot reach a padding row, and the ALU
`addr < len` check refuses a read above it.

The first test run had two test errors, not circuit errors (test1, unit1):
- `m3_leaves_chain` gave leaf 1 a non-empty sibling for the reserved segment 0, and so leaf 2's old root
  could not match.
- In `m3_append_skipped_rejected`, the "data kept" case kept data that is the zero tuple. Every honest
  record of the spike trace is the zero tuple (§3.4), so `mem row` had nothing to refuse. The case now
  writes non-zero data.

### 3.4 Gemini Quint verdict C1 to C5

- C1 (registers outside the digest are write-only): `m3_scratch_registers_are_write_only` checks that no
  core constraint except `next v_k` reads `Pre(v0..v2)`. It also checks that a free slot-0 value of
  `v0..v2` changes neither acceptance nor the statement. This is a structural check plus a positive
  MockProver test. There is no mutant core to run as a negative.
- C2 (no written record is the zero tuple): the committed record is `(act, a, b, c)` with `act = 1` on
  every written row. In the honest spike trace every spill is D7's sentinel `(0, 0, NIL)`, so every
  written cell is the zero tuple. Each is accepted, and its POP is served, because the bus tuple has
  `chip = MEM` and the record hash has tag 1. Padding has `chip = 0` and tag 0. Negatives: the zero record
  marked padding is refused (above), and a padding cell cannot be read (above). The tag is the active
  flag, not a separate kind word: frame, task and data records are not told apart [stand-in, §5].
- C3 (aligned base in the circuit, aligned genesis): `8 acc_in = base` with a 17-bit `acc_in`. Genesis
  `len = 8`. Negative: unaligned base. `Init` with input rows is not in the spike.
- C4 (active prefix = `len_end - base`, padding zero, hash over all 8 rows, `post_d.len := base + 8` as a
  separate close cell): `mem row/chain/first/last` and the walker's `len_post`. Negatives: as listed.
- C5 (`len` + 1 per append; reads refuse the zero tuple; the root update proves the slot empty with the
  same siblings): LEN's next state is unchanged (`every_register_word_is_checked` covers word 16). The
  padding tuple is the bus's zero tuple. The two walks share sibling and bit cells. Negatives: as listed.
- D2 is kept unchanged. There is no PAGE in the spike.

### 3.5 Gemini Quint HASH verdict H-C1 to H-C6

- H-C1: the build-time asserts `n >= 2` and `!provide || n == 2` in `PoseidonCfg::sponge` are unchanged.
  Every M3 sponge has 2, 4 or 30 inputs and no provider row.
- H-C2, H-C3: unchanged. The capacity is in the fixed `rc[2]` of the first absorb row. The absorb and round
  gates keep their bodies; cut 1 changes only their selector (§2.2), and the E7 negatives run in both
  modes.
- H-C4: the `q_first * q_prov` constraints pin `chip = HASH` and `key = 2`, and `out` is copied from the
  final output row. New negative `provider_chip_is_pinned_and_out_is_copied`: a provider row with
  `chip = ALU`, requested as such, is refused by `hash absorb first`. A provider `out` that differs from
  the output row, with a request for that value, is refused by the permutation argument only.
  `provider_key_is_pinned` (E7) covers the key.
- H-C5: chip tags are gate-pinned advice on every `q_prov` row (HASH, ALU, MEM, and the M3 segment, where
  `chip = act * MEM`). D4 as written (a fixed column) is not met; §6 proposes the text. The E6 provider
  rows (`params.hash`, `hash_tuple`) pin nothing. They are now refused at configure time together with
  `pos_hash` (`e6_rows_never_with_poseidon`, a `should_panic` test). They stay only for the E6 keys of
  `experiments-2.md`, which are measurement records, not leaves.
- H-C6: every M3 sponge takes copied cells (MEM record words, the outputs below it, register cells) and
  feeds a consumer: records go to the segment root, the segment root to the new walk, the walks to the
  digests, and the digests to the instance. The walk compressions take `in0`, `in1` as advice that the
  `m3 walk` gate fixes from copied `x`, `s` and `b` cells (`m3_walk_gate_rejects_bad_select_and_bit`).
  With `m3` set, the free-input E7 workload is refused by an assert. The final key has no HASH provider
  rows. No core rule requests HASH: the spike rules do not hash. A requester that gates `chip = HASH`
  (and `key = 2`) is needed with the first rule that hashes, such as PAGE [OPEN].

## 4. C6: raw addresses in the Sherlock machine (finding, not fixed)

Verdict C6: under M3, `len`, `fp`, stored links and `mem_root` depend on where the prover closes leaves. M3
is sound only if no program-visible value and no published statement depends on a raw address. An audit
of `crates/mori-core/src/sherlock/` and the recursion statement gave this result. A read-only subagent
made the audit (paths relative to `crates/`; `owner_runtime.rs` is
`mori-core/src/sherlock/helper_machine/shared/owner_runtime.rs`). The lead confirmed these citations by
reading them: `RefC` derives only `Clone, Copy, PartialEq, Eq, Debug` (`shared.rs:701-705`); SI-13
(`owner_runtime.rs:4744-4753`); the fresh-id equality (`owner_runtime.rs:3021-3028`); `fin_d =
Poseidon(dom, halted, result, lanes_d)` with `lanes_d` a free assigned witness
(`mori-recursion/src/run_decider.rs:279-288`); and `carrier_digest` over the raw image, data and helper
lanes (`mori-backend/src/sherlock_c6_witness_stream.rs:216-241`). The other citations are the subagent's
and were not re-read.

- No raw address reaches a program. Machine A (`mori-core/src/sherlock/reference.rs`) has no references.
  In machine B / C6 (`helper_machine/`), handles are `RefC::{Const, Data}`, `EnvRootC` and `SliceC`.
  `RefC` derives only `PartialEq, Eq`. Every primitive dereferences first (`owner_si21_closure.rs:131-195`),
  equality is structural (`families/structural.rs:130-200`, `shared.rs:1635-1639`), and nothing turns a
  reference into a UInt or Bool. There are no `fp`/`tp` registers there. Depth and the alloc counters are
  logical counts.
- Step invariants and transitions need dense, monotone bump allocation, which M3 breaks:
  - the fresh id is the table length (`owner_runtime.rs:2406, 3023, 3027, 3044-3054, 2819-2822`;
    `owner_adt_runtime.rs:723`; `owner_si21_closure.rs:575-579`);
  - field slices are contiguous (SI-12, `owner_runtime.rs:~4700-4735`; `2997-3001`);
  - ordering checks: SI-13 `id < owner` (`owner_runtime.rs:4746-4752`), SI-14 `previous < i`
    (`4810-4816`), ADT `d < row_idx` (`owner_adt_runtime.rs:1274-1277`).
  
  Monotone segment addresses keep the `<` checks, but the `fresh == len` and contiguity equalities break
  if `len` jumps to `base + 8` at a leaf close. The codec bounds `index < max_data_nodes`
  (`c6_phys.rs:453-457`, `c6_phys_data.rs:170`) would refuse addresses above the logical node count.
- Published values hash raw addresses:
  - `fin_d`, the run decider's public value `V_FIN` (`mori-recursion/src/run_decider.rs:48-56, 279-293`),
    is `Poseidon(tag, result, lanes_d)`;
  - `lanes_d` hashes the raw final carrier: every `RefC`, env root, slice start, the alloc/active
    counters and the full tables (`mori-backend/src/sherlock_c6_witness_stream.rs:216-241`,
    `sherlock_c6_child.rs:686-692`, `c6_phys.rs:1345-1381`);
  - the leaf and node digests `pre_d`/`post_d`/`cur_d` (`mori-recursion/src/ivc.rs:67-101`) and `init_d`
    hash the same carrier.
  
  `lanes_d` is a free witness in the decider, so `fin_d` is not compared with an independently computed
  value. But under M3 it becomes a public value that the prover can vary for the same run. The verified
  result is dereferenced to a UInt before publication.
- Diagnostics print raw addresses:
  - `OutcomeC::Halted { value: RefC }` (`shared.rs:1041-1046`);
  - the profile /2 report (`compile_artifact/suspension2.rs:555-563` with `owner_suspend.rs:500-508, 533-538`)
    prints `"Data(17)"` for a non-scalar value;
  - the wire, simulate and resource-report JSON print raw references and alloc lengths (`wire.rs:1050-1058,
    1450-1456`; `resource_report.rs`).

The spike leaf of this experiment has the same property. Its `pre_d` and `post_d` hash FP, TP, LEN, the
stored links and `mem_root`, and under M3 all of these depend on where the prover closed earlier leaves.

So: programs see no addresses, but the published `fin_d`, the chain digests, and the dense-allocation
invariants SI-12 to SI-14 do depend on raw addresses. Before M3 can be adopted, the C6 machine needs
either a logical (dense) address inside the carrier with M3 addresses only in the leaf's memory tree, or a
`fin_d` that does not hash the raw carrier, or a statement that names the leaf partition. This is not
resolved here [OPEN].

## 5. What is still a stand-in

- The spike rules (`rf.rs`) are stand-ins for the C6 rules. Every honest record of the trace is the zero
  tuple (D7 sentinel spills), so the non-zero data paths are exercised only by the negatives and by
  `seg_padding_cell_is_unreadable`.
- The record tag is the active flag, not a kind tag. There is no PAGE (reads below `base`), no PROG
  opening, no TRAP and no task-stack modelling beyond the spike rules. D2 holds only because no PAGE
  exists.
- `pre_root` siblings in the tests are stand-in values for filled segments. There is no host-side memory
  tree that produces them from a real run.
- The leaf statement is still `[0, 0, pre_d, post_d, 0, 0, steps]`. The digest covers 29 register words
  and `mem_root`, not `prog_root` or the four `(root, length)` pairs of the Init table (`experiments-2.md`
  §3.5).
- The node is the `ChipNode` mirror (calibrated in E3), the SRS is unsafe, and only `cost_model` rows were
  measured. No proof was made.
- No core rule requests HASH (H-C6); the bus-served HASH path is tested only at chip level.
- There is no Quint model of the packed selectors or of this M3 circuit, and no Gemini Quint lane was run
  on this circuit (the owner's rule wants one for every Sherlock circuit change; it is open). The verdict above modelled M3 abstractly (C = 4 and C = 8, exact hash maps). The HASH chip and
  the running-sum RANGE also still lack their own Quint model (`experiments-3.md` §4).

## 6. Proposed text for design.md 2.7 (text only; design.md was not edited)

For 2.7.8, replace D3 with:

> - D4 Chip tag per provider row. On every row with `q_provider = 1`, a gate pins the `chip` field to the
>   row's own non-zero chip id (or `act * id` on a MEM segment row), and the table side is
>   `q_provider * tuple`. (Replaces "the region's fixed selector column"; HASH verdict H-C5.)
> - D3 Aligned segment (M3, C = 8). `base = pre_d.len` is a multiple of C, enforced in the circuit
>   (`base = C * seg`, `seg` given by 17 boolean bits). Genesis `len` is C (the first segment is reserved),
>   and an Init with input rows pads `len` to a multiple of C. MEM holds C rows at `base .. base + C`. A row
>   is active exactly when its address is below `len_end`, the running `len` after the last slot. Padding
>   rows are the zero tuple and put the zero tuple on the bus. The committed record of a row is
>   `H(tag, a, b, c)`, with tag 1 on every written row and 0 on padding. The segment root hashes all C
>   rows. The old walk proves slot `base / C` empty in `pre_d.mem_root`. The new walk uses the same
>   sibling and bit cells and gives `post_d.mem_root`. `post_d.len` is a close cell `base + C`, not slot
>   S's `len`. A read refuses padding. Registers outside the digest are write-only. [OPEN: addresses
>   depend on the leaf partition; see 2.7.10 and `experiments-4.md` §4.]

For 2.7.9, after the node-cost text proposed in `experiments-3.md` §5:

> Selectors MAY share a fixed column as codes with Lagrange indicators when their rows are disjoint and
> `codes + body degree <= 6`. Lookup selectors stay degree 1. Measured 2026-11-10 (`experiments-4.md`):
> one absorb column instead of three saves 12,703 node rows. Packing the core gate selectors saves 33,055.
> The leaf with pair rows, the Poseidon-only HASH chip, one-limb RANGE and M3 wired for real (12 advice, 13
> equality, 20 fixed columns, 4 lookups; 6,511 rows, k = 13 at S = 16) puts the node at 823,466 rows
> (k = 20) and the run decider at 929,166 (k = 20). That leaves 126,534 node rows of headroom.

For 2.7.10, replace the M3 sentences of the HASH paragraph proposed in `experiments-3.md` §5 with:

> Per leaf at S = 16, M3 with C = 8 is 8 tagged records (4 inputs), 7 segment compressions, 17 + 17
> top-tree compressions with a shared path, and 2 state digests of 30 inputs: 87 permutations, 6,090 rows.
> The state digest is Poseidon over the 29 digested register words and `mem_root`. [OPEN: the C6 machine
> hashes raw addresses into `fin_d` and the chain digests, and its invariants SI-12 to SI-14 assume dense
> allocation (`experiments-4.md` §4); a Quint model and a Gemini Quint lane for this circuit, the packed
> selectors, the HASH chip and the running-sum RANGE.]
