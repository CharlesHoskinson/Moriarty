# Experiments of the final-design night plan (2026-11-11)

Labels: MEASURED is a `circuit_model` row count or an assignment offset of a circuit that was built and passed MockProver. ESTIMATE is a computation from measured parts or from an earlier fit; the inputs are named.

## C1 dense tail-buffered memory

First backend of night-plan step C1: the dense tail-buffered Merkle memory of `research/sherlock-runlength-consensus-20261111/CONSENSUS.md` 1.2.3 item 2, with the invariants of `AUDIT.md` finding 8, behind a MEM interface that a write-once LogUp backend can implement later. The second backend is not built.

### What was built

Code in `crates/mori-chip/src/` (spike crate; nothing in the production leaf changed):

| File | Contents |
|---|---|
| `membackend.rs` | The `MemBackend` trait (`persist`, `cold_read`) and its cells (`Window`, `WinRow`, `ColdRead`, opaque token `Vec<Cell>`). The backend-neutral MEM module `WindowCfg`: frontier decomposition, window rows, increment row. |
| `dense.rs` | `DenseCfg: MemBackend` (Merkle walks, masks, flush row, gated equality), the native model `DenseModel` (sparse top tree), domain-tagged hashes. |
| `dense/tests.rs` | Harness circuit (MEM + backend + a CORE stand-in with append and read ports, boundary `(len, root, tail_d)` per arena as instance values), 9 positive tests, 21 named mutants, the row measurement `c1_rows`. |
| `poseidon.rs` | New `sponge_iv` / `sponge_native_iv`: a domain constant in the capacity element. `sponge` is unchanged (it calls `sponge_iv` with 0), so the stdlib differential tests still hold. |

Design as built (one arena; the four-arena variant is four copies with the kind pinned per arena):

- State `(len, root, tail_d)`. `len_pre` and `len_post` are each decomposed into `TOP + 3` boolean bits inside MEM (`WindowCfg::window`), so both are integers below `2^(TOP+3)`; `q_pre = floor(len_pre / 8)` is the high part.
- Window: `8 (W + 1)` provider rows at addresses `8 q_pre + i`. `imp` (imported tail) and `act` (active) are boolean prefixes, `imp <= act`, the `imp` prefix ends at `len_pre` and the `act` prefix at `len_post`, the last row is inactive. A padding row has zero kind and data and serves the zero tuple. An active row serves `(MEM, 8 addr + kind, a, b, c)`; `kind` is in `0..=4` (a degree-5 gate) and equals the fixed `kfix` when `kfix != 0`.
- Hashes: record `H_REC(kind, a, b, c)` (2 permutations), pad record `PAD = H_PAD(0, 0)`, `EMPTY_SEG = H_EMPTY(0, 0)` (non-zero, differs from `PAD`), node of height `h` `H_NODE_h(x, y)` (1 permutation; heights 1..3 inside a segment, 4..TOP+3 in the top tree). Every tag is in the sponge capacity (fixed by the circuit), not a first input.
- Every leaf opens the tail: `Tree8(imp ? rec : PAD)` over window rows 0..7 is copy-constrained to `pre.tail_d`. The post tail is `T_k = Tree8(act ? rec : PAD)` of the window segment after the last flush. So `tail_d` is canonical, and a leaf with no append passes `(len, root, tail_d)` through unchanged (test `c1_pass_through_and_every_tail_occupancy` checks `pre == post`).
- Flush unit `j` is on iff `f_j = act(row 8j + 7)`, that is iff `floor(len_post / 8) > q_pre + j`. Its old walk starts at `EMPTY_SEG` (gate) and must reach the running root when `f_j = 1`; its new walk from `T_j` copies the old walk's siblings, bits and accumulators; `root' = root + f_j (new - root)`. The walk index is `idx_0 = q_pre`, `idx_j = idx_(j-1) + f_j`, decomposed into TOP bits by the walk accumulator (last `acc_out = 0`).
- Cold read: record hash, then `TOP + 3` levels whose bits are the bits of `addr` (accumulator chain from the copied `addr`), result equal to `pre.root` when `enable = 1`. The CORE stand-in routes a read to cold iff `addr < base` (a range-checked difference) and checks `addr < len_cur` (range-checked `len_cur - 1 - addr`).

### Commands and runs

Every build and test ran through `scripts/doctorwatson heavy` with `CARGO_TARGET_DIR=~/.cache/final-design/c1/target`. Run ids are in `qa/sherlock/runs.jsonl` and traces in `qa/sherlock/traces/`.

| Run id | Command | Result |
|---|---|---|
| `20261111T065403Z-c1-check` | `cargo check -p mori-chip --lib --locked` | exit 0 |
| `20261111T065515Z-c1-test` | `cargo test -p mori-chip --lib --locked dense::` | 29 passed, 1 ignored (`c1_rows`), first version |
| `20261111T065751Z-c1-clippy` | `cargo clippy -p mori-chip --no-deps --all-targets --locked -- -D warnings` | no finding in the C1 code (3 found in run `20261111T065705Z-c1-clippy` were fixed); 11 findings in files C1 did not change (`cpu.rs`, `m3.rs`, `mem.rs`, `rfcore.rs`, two old loops in `poseidon.rs`) under the pinned toolchain; `mori-lexer` also fails `collapsible_if` without `--no-deps` |
| `20261111T065759Z-c1-libtest` | `cargo test -p mori-chip --lib --locked` | 104 passed, 1 ignored |
| `20261111T065836Z-c1-rows` | `cargo test -p mori-chip --lib --locked dense::tests::c1_rows -- --ignored --nocapture` | rows below |
| `20261111T071031Z-c1-test` | `cargo test -p mori-chip --lib --locked dense::` (frontier moved into MEM, three mutants isolated, W = 2 last-segment test) | 30 passed, 1 ignored |
| `20261111T071045Z-c1-rows` | as `065836Z` | identical rows; log `~/.cache/final-design/c1/logs/c1-rows.txt` |
| `20261111T071101Z-c1-clippy` | as `065751Z` | no finding in the C1 code |

### Tests

Positive (MockProver accepts):

- `c1_pass_through_and_every_tail_occupancy`: tail occupancy `r = 0..7`, with 0 appends (state unchanged) and with 1 append (flush exactly at `r = 7`).
- `c1_cross_boundary_reads_and_chain`: 6 appends from `r = 5` (one flush), a cold read, a read of an imported tail record, a read of a new record, a read at `len_cur = len_pre`; the next leaf imports the post tail and flushes again.
- `c1_two_flush_units`: `W = 2`, 17 appends from `r = 6` (two flushes).
- `c1_partition_independence`: 37 appends under 4 partitions (`[37]`, `[8,8,8,8,5]`, `[1,7,0,9,3,8,1,8]`, `[15,15,7]`) give the same `(len, root, tail_d)` (native model; each leaf of at most 8 appends also in the circuit).
- `c1_top61_and_top63_positive`: TOP = 61 and 63, with a cold and a tail read.
- `c1_counter_edge_2_64_minus_1`: TOP = 61, `len_pre = 2^64 - 2`, one append gives `len_post = 2^64 - 1`.
- `c1_last_segment_two_units`: the same edge with W = 2; the index chain `idx_j = idx_(j-1) + f_j` keeps every walk index below 2^61, so the last segment is provable at any W.
- `c1_four_arenas`: four arenas in one circuit, kind pinned per arena, reads in each.

Named mutants (MockProver refuses; the test asserts the failing gate or argument):

| Mutant | Refused by |
|---|---|
| `forged_tail_across_leaf_boundary` (leaf 2 imports a tail value that differs from leaf 1's post tail; the post tail is recomputed from the forged value) | tail-open equality only, no gate |
| `non_canonical_tail` (pre tail with a non-pad record above `r`) | tail-open equality |
| `flush_skipped` (segment fills, root kept) | `dense flush` |
| `flush_spurious` (no boundary crossed, root changed) | `dense flush` |
| `address_reuse_occupied_slot` (pre root already holds a segment at `q_pre`) | `dense flush` (old walk from `EMPTY_SEG` misses the root) |
| `empty_seg_forgery` (old walk started at the occupying segment root) | `dense walk empty` |
| `address_reuse_window_row` (row 1 claims row 0's address, key consistent) | `mem win first` (address chain), not `mem win row` |
| `read_at_len` (`addr = len_cur`) | `mem bits last` |
| `read_beyond_len` (past `len_cur` and past the active window) | `mem bits last` and the bus lookup |
| `future_record_read` (read at `len_cur = len_pre` of a record this leaf appends) | `mem bits last` |
| `wrong_kind_warm` (FIELD read of a DATA window record) | bus lookup |
| `wrong_kind_cold` (ENV read of a FIELD old record) | `dense geq` |
| `cold_value_forged` | `dense geq` |
| `cold_route_forged` (window address sent to the cold path) | `mem bits last` |
| `counter_wrap_2_64` (TOP = 61, `len_pre = 2^64 - 1`, one append) | `mem bits last` (len_post = 2^64 is not below 2^64) |
| `counter_field_wrap` (`len = p - 1`) | frontier bits |
| `pad_row_on_bus` (inactive row offers `chip = MEM`) | `mem win row` |
| `kind_out_of_range` (kind 9 with a consistent key `8 x 16 + 9 = 8 x 17 + 1`, which would serve address 17) | `mem win row` (kind range), not the address chain |
| `arena_kind_pinned` (FIELD record in the DATA arena) | `mem win row` |
| `close_advances_len` (`len_post` one above the appends) | window and append-chain constraints |
| `append_value_mismatch` (window value differs from the append request) | bus lookup |

### Measured rows (MEASURED, run `20261111T065836Z-c1-rows`)

Sections are assignment offsets inside the harness region; totals are `circuit_model` rows. Harness key shape: 12 advice, 13 equality, 28 fixed (incl. permutation columns), 2 lookups (the bus), degree 6, rotations -1/0/+1. TOP does not change the key shape: the walk gates are per row.

| Item | TOP = 61 | TOP = 63 |
|---|---:|---|
| One flush unit (old + new walk, flush row) | 8,541 | 8,821 |
| One flush incl. the segment's `Tree8` (7 compressions) | 9,031 | 9,311 |
| One cold read (record hash, `TOP + 3` levels, gated equality) | 4,621 | 4,761 |
| Read port of the CORE stand-in (route and `addr < len_cur`, 2 range checks) | 130 | 134 |
| One window slot (window row, record hash, mask, 1/8 of `Tree8`) | 203 | 203 |
| One append in the CORE stand-in (append row, request row, increment) | 3 | 3 |
| Tail open (8 masks + `Tree8`) | 498 | 498 |
| Frontier (two length decompositions) | 128 | 132 |
| `persist` at W = 1 / 2 / 3 / 4 / 5 | 12,137 / 22,298 / 32,459 / 42,620 / 52,781 | 12,417 / 22,858 / 33,299 / 43,740 / 54,181 |
| Harness, W = 1, 8 appends, 0 cold reads | 12,305 (k = 14) | 12,589 (k = 14) |
| Harness, W = 1, 8 appends, 4 cold reads | 31,309 (k = 15) | 32,169 (k = 15) |
| Harness, 4 arenas, W = 1 each, 4 appends each | 49,172 (k = 16) | 50,308 (k = 16) |

So `persist(W) = 12,137 + 10,161 (W - 1)` at TOP = 61 and `12,417 + 10,441 (W - 1)` at TOP = 63. TOP = 63 costs 280 rows per flush unit and 140 per cold read, as the AUDIT finding 3 ESTIMATE says (2 x 2 levels x 70 rows; 2 levels x 70 rows). Against the untagged figures: a flush is 7 + 122 permutations = 9,030 rows plus 1 flush row; a cold read is 2 + 64 permutations plus 1 row (the 4,480-row figure of the consensus omits the record hash, as `astra-area2.md` says).

"One memory write" has two costs. The circuit pays the window capacity: 203 rows per slot, fixed by W, whether the slot is used or not. A used slot adds 3 CORE rows. The harness with 0, 1 and 8 appends at W = 1 is 12,281, 12,284 and 12,305 rows.

Tag variant not built: a tag as the first hash input makes each node 3 inputs, 2 permutations. ESTIMATE from the same parts: flush unit 17,080 rows, cold read 9,100 rows at TOP = 61. The capacity tag costs nothing.

### Leaf rows at S = 16, 64, 128 (ESTIMATE, composed from the measured parts)

Inputs: CORE and the rest of the leaf from the E8 leaf (6,511 rows at S = 16, of which M3 is 6,090; so 421 rows, 26.3 per step, scaled linearly to S = 64 and 128, ESTIMATE); two state digests over 29 register words plus `root` and `tail_d` (31 inputs, 2 x 1,120 rows, ESTIMATE); frontier, window and `persist` MEASURED; cold reads at 4,751 (TOP = 61) or 4,895 (TOP = 63) rows each with the read port, MEASURED. W from the append census of `experiments-2.md` 3.2 (MEASURED there): at most 8 appends in a 16-step window and 18 in a 64-step window, plus at most 7 tail records: W = 1 at S = 16, W = 3 at S = 64, W = 5 at S = 128 (128-step maximum taken as 2 x 18 = 36, ESTIMATE). A leaf that needs more closes early (design 2.7.7). `N_page` (cold reads per leaf) has no census; the table gives 0, 4 and 8.

| S | W | TOP | N_page = 0 | N_page = 4 | N_page = 8 |
|---|---|---|---:|---:|---:|
| 16 | 1 | 61 | 14,942 (k 14) | 33,946 (k 16) | 52,950 (k 16) |
| 16 | 1 | 63 | 15,226 (k 14) | 34,806 (k 16) | 54,386 (k 16) |
| 64 | 3 | 61 | 36,543 (k 16) | 55,547 (k 16) | 74,551 (k 17) |
| 64 | 3 | 63 | 37,387 (k 16) | 56,967 (k 16) | 76,547 (k 17) |
| 128 | 5 | 61 | 58,565 (k 16) | 77,569 (k 17) | 96,573 (k 17) |
| 128 | 5 | 63 | 59,969 (k 16) | 79,549 (k 17) | 99,129 (k 17) |

Four arenas (the audit's default until the contiguity invariant is mechanised), ESTIMATE from the same parts, S = 16, W = 1 per arena, digests over 29 words plus 4 x (`root`, `tail_d`) = 37 inputs (2 x 1,330 rows):

| Variant | TOP | N_page = 0 | Cold reads that fit `k_leaf <= 16` |
|---|---|---:|---:|
| 4 arenas, one flush unit each | 61 | 52,205 (k 16) | 2 |
| 4 arenas, one flush unit each | 63 | 53,341 (k 16) | 2 |
| 4 arenas, 2 shared flush units (arena multiplexers not built) | 61 | 35,123 (k 16) | 6 |
| 4 arenas, 2 shared flush units (arena multiplexers not built) | 63 | 35,699 (k 16) | 6 |

The tables exclude hash-stack spills (FRAMES, TASKS), PROG and Init openings, TRAP and the range table rows.

At S = 16 the gate `k_leaf <= 16` holds up to about 10 cold reads per leaf at either TOP. The cold reads dominate: each costs as much as 23 window slots. TOP = 63 adds 284 to 2,556 rows across the table, and no k changes between TOP = 61 and 63 in any cell.

### Node rows (ESTIMATE, not measured)

The node grows with the leaf's columns and lookups, and by about 4 rows per bit of leaf k (design 2.7.11 fit: 6,004 rows per fixed column, 7,298 per advice column, 8,189 per equality column, about 22,600 per lookup). MEM and the backend add no advice column, no equality column (the leaf already has all 12 advice columns in the permutation) and no lookup (the window uses the existing bus). They add 6 fixed columns: `kfix` and 3 packed code columns (MEM), 2 packed code columns (backend). They remove the 2 code columns of M3 (`SegMemConfig`, `WalkConfig`). Net +4 fixed columns: about +24,016 node rows, and +12 for leaf k 13 to 16, so the node goes from 823,466 to about 847,494 rows (gate 950,000). TOP adds nothing to the node. Packing the new codes into the leaf's existing code columns could remove part of the +4; not tried. The node was not run over a leaf with this backend: that needs the backend wired into `RfLeaf` in place of M3.

### Arenas (R-6)

MEASURED: four arenas with one flush unit each cost 4 x 12,137 rows of `persist` (49,172 harness rows), against 12,281 for one arena. ESTIMATE: four arenas sharing two flush units (lanes A, B) would be 4 x (12,137 - 8,541) + 2 x 8,541 = 31,466 rows plus the arena-select multiplexers (not built). The one-arena variant with tagged records is 12,137 at W = 1. Not done: the mechanical check of the 108-rule non-interleaving claim over the reference machine. It stays open (needs an assertion in `mori-core` run on the corpus, or a Quint model).

### Deviations from the audit text (for `DECISIONS.md`)

- Domain tags are in the sponge capacity, not in the first input as AUDIT finding 8 says. Same separation (the capacity is fixed by the circuit and differs per domain, height and input length); cost 0 permutations against about 2x on every node hash (flush unit 17,080 and cold read 9,100 rows at TOP = 61, ESTIMATE).
- An active window row with kind 0 is allowed (no reader can request kind 0, and its key is still injective).
- `q_post = q_pre + sum f_j` is not a separate gate: it follows from the `act` prefix, the address chain and the decomposition of `len_post`.

### Quint lane (Gemini, argon-run) for this circuit change

Modules: MEM frontier and window (`membackend.rs`, `WindowCfg`), dense `persist` and `cold_read` (`dense.rs`), the CORE port assumptions of the harness (append chain from `len_pre` to `len_post`, read route and `addr < len_cur`, kind from the rule). Properties:

1. `(len, root, tail_d)` after a leaf sequence is a function of the append sequence only, under any partition.
2. A flush happens iff a segment boundary is crossed; no segment slot is written twice; the old leaf is `EMPTY_SEG`.
3. `tail_d` is canonical, and the boundary passes through unchanged when a leaf appends nothing.
4. `8 addr + kind` is injective (kind in `0..=4`), also across four arenas through the kind pin.
5. Window and cold providers are disjoint (`addr >= base` against `addr < base`).
6. Every read is below `len_cur` and carries the kind the rule expects.
7. `len <= 2^(TOP + 3) - 1`; at TOP = 61 `2^64 - 1` is reachable and `2^64` is not; a leaf at the last segment is provable for any W.
8. Appends and new window rows correspond one to one (rests on CORE's append chain).

### Not covered and open

- The LogUp backend (C1 second half) is not built. The trait keeps the token opaque so that it can return obligation cells.
- Hash-chain FRAMES and TASKS stacks are not built.
- The backend is not wired into `RfLeaf`; the leaf and node figures above are composed, not measured.
- The CORE stand-in reads after all appends or at `len_pre`; a read between two appends of one leaf needs the step's own `len` from CORE.
- An active window row with kind 0 is allowed (no reader can request kind 0). A gate `act * kind != 0` needs an inverse witness; not added.
- Write-before-read of scratch registers (AUDIT finding 9, R-8) is a CORE obligation; C1 does not cover it.
- The per-step maximum of cold reads, appends and arenas (AUDIT finding 6) is not measured; the leaf rows above depend on it through `N_page` and W.
- Cold reads of records in segment `q_pre` that the leaf flushes are served by the window, not by the backend, as designed.
