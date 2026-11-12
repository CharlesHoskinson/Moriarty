# Sherlock design experiments

This file holds the deciding experiments of the co-processor design (`research/sherlock-processor-final-20261111/PROPOSAL.md`, applied in `ARCHITECTURE.md`) and the record of the machine-level experiment C1.

Marks: MEASURED is a number with the record that holds it. DERIVED is an ESTIMATE computed by a stated rule from MEASURED numbers; it is not a measurement. ESTIMATE is a computation with named inputs. PENDING is an experiment that has not run.

Prefixes: `CT/` is `research/sherlock-call-time-opt-20261111/`; `CO/` is `research/sherlock-compact-open-problems-20261111/`; `SB/` is `repos/stark/openvm-org_stark-backend/`. Records on other branches: `D3:` is `research/sherlock-processor-final-20261111/D3-SOUNDNESS.md` at commit `234954947` of branch `d3-soundness`; `E1:` is `research/sherlock-compact-open-problems-20261111/E1-devnet-test.md` at commit `4a4abd875` of branch `e1-test`; `LS:` is `research/sherlock-stark-framework-20261111/L-STRUCTURE.md` at commit `3957ddf7c` of branch `l-structure`; `GW:` is `research/sherlock-gpu-wrap-20261111/RESULTS.md` on branch `gpu-wrap` (working copy over commit `b158d4741`).

Hard rules for every experiment: never k = 22 (no BLS12-381 circuit above k = 21; a BabyBear trace of height 2^22 is a STARK scaling point, not a circuit k); every proof is a fixed-size shard; zero knowledge end to end, with every STARK layer on the user's machine and seeded proofs in test mode only; proven 100-bit security with no unproved conjecture, claimed only after the security qualification gate G1 to G5 passes; GP-1 to GP-4 never narrowed; one universal key set; the ledger 9.1.0.1 verifier unchanged. Heavy jobs run one at a time under `heavy-run`, with load average and GPU occupancy recorded.

## 1. Deciding experiments D1 to D15

Source of the rows: `CT/RESULT.md` section 6. Every bit count of the design is a calculator figure, not a proven bound, until the security qualification gate passes; D3 reruns on the frozen parameter sets. Every wrap and `L` cell is an ESTIMATE until D10 and D15 run.

| # | Experiment | Size | Decides | Status |
|---:|---|---|---|---|
| D1 | Midnight Poseidon permutation loop on 1, 10 and 20 threads, then a seeded 21-bit grind on 20 threads | about 1 minute, under 1 GB | the CPU `L` rate and core count of path 1 | MEASURED under load 15 to 40 (`CT/measured/L-hash-wrap-setup.md` section 1): 37.54 us per permutation on one Cortex-X925 core, 252.96 us on one Cortex-A725 core; 228,746 to 253,270 permutations per second on the whole host, about 8.6 to 9.5 X925-core equivalents; one 21-bit check 8.88 s median and 11.18 s mean on 20 threads over 20 trials. A quiet-host repeat is PENDING; the quiet bound is 305,907 permutations per second, DERIVED from the single-thread rates |
| D2 | `bench msm` and `bench ntt` with the GPU exclusive and the host idle, then under `nsys` | minutes | idle sppark MSM against the loaded 0.479 s per 2^20; fixed and marginal cost per call | PENDING |
| D3 | The stock soundness calculator, after the parameter code of build step 4, on every parameter set of `ARCHITECTURE.md` section 1.9 at its shape bounds, on the frozen query vectors, and for `L` at the `w_stack` that D10 measures | CPU, minutes | every bit count; gate check G2 | MEASURED on the sets of `ARCHITECTURE.md` section 1.9 with the calculator patched for per-fold PoW, two OOD samples and a separate radius (`D3:9-24`): `L` 101.1727 bits at `w_stack` 16 and 101.0869 at 18, with LogUp PoW 21 required (the code rule's 20 gives 100.9126); app 101.0891 and leaf aggregation 101.0110 with query PoW 21; internal 100.0129 (OOD, one sample) with LogUp PoW 20, where the code rule's 19 gives 100.4129; stock m = 3 at S4 gives 46/25/17 queries and OOD 98.0129. The two-OOD bound is DERIVED (`D3:150-158`). Follow-up sweep MEASURED (D3 section 8, `research/sherlock-processor-final-20261111/D3-SOUNDNESS.md` section 8 on branch `d3-soundness`, commit `81abb8f7a`): only the `L` `mu` term moves with `w_stack`; with `mu` PoW 20 the weakest term is 100.6508, 100.2202, 100.0451 and 99.6198 bits at `w_stack` 24, 32, 36 and 48; with `mu` PoW 22 it is 101.1727 at 48. Calculator part of G2 passed; a rerun on the frozen sets of G1 is PENDING |
| D4 | E-1 T0: rebuild `m` from the saved XC-9 call bytes and compare `le31(SHA256(m))` with `public_inputs[0]`; rebuild with a new TTL and with the DUST intent added | CPU seconds | the no-rebuild claim behind placement V; the host keeps the order prove, then `balanceUnbound`, until it passes | T0 MEASURED: the host rebuild of `m` equals the ledger's `binding_input` on the XC-9 call and three devnet calls, window at offset 58 (`E1:41-46,61`). The rebuild with a new TTL and with the DUST intent added, and calls with a fallible transcript or non-empty effects, are PENDING |
| D5 | `wrap_prove` at k = 16, config `msm`, `MORI_GPU_DEBUG=1` | 1 minute | bucket skew as the cause of the in-prover MSM defect | MEASURED on the real decider at k = 20 in place of the k = 16 run (`GW:10-11`): 45 MSM calls take 622 to 647 s, because witness columns repeat small range-checked values and sppark accumulates each bucket serially; the witness-independent MSM skew mask takes them to 23.2 s with unchanged proof bytes |
| D6 | The k = 20 decider pinned to the X925 cores only, the A725 cores only and all 20 cores | about 15 minutes, no build | static chunking against the memory bus; the A725 speed | PENDING |
| D7 | Fr Poseidon `.cu` microbench over sppark `fr_t`: 2^24 permutations and a grind loop at 11 to 22 bits | one nvcc compile, 2 minutes of GPU | the rate of the Fr Poseidon GPU kernel for `L`: target 5 to 15 M permutations per second (ESTIMATE, `CT/RESULT.md` section 6) | PENDING; no GPU Poseidon over Fr exists today (`CT/measured/L-hash-wrap-setup.md` section 1) |
| D8 | Wrap setup timers around `srs_any()`, `setup_pk` and `prove` on a quiet host; warm against cold in one process | about 3 to 15 minutes | the cold setup row | MEASURED once under load (`CT/measured/L-hash-wrap-setup.md` section 2): wrap setup of the XC-9 path 82.87 s at k = 21 and 79.99 s at k = 20; peak RSS 30.24 GB at k = 21. The quiet value 22.2 s at k = 21 is an ESTIMATE (DERIVED from the same-run contention factor 3.73) and holds only if `setup_pk` slows as `prove` does; the k = 20 cross-check fails that condition. The quiet-host run is PENDING |
| D9 | sppark one-batch patch and window from the call size, then D2 again | days of build | path 2 against path 3 | PENDING |
| D10 | Keygen of `L` (`VerifierSubCircuit::<1>` at root parameters) with the per-AIR dump, then `levers.py L-STRUCTURE.json` | keygen only, minutes | the `L` shape, `w_stack`, the high row case at k = 19. The `L` key bound is `w_stack` at most 48 with `mu` PoW 22 (OD-3); D10 may tighten it, and a tighter bound reruns D3 before the `L` key freezes | Keygen MEASURED (`LS:3-7,38-48`): 39 AIRs, 1,537 trace columns, 487 interactions, 2,210 constraints; G4 wrap 434,692 rows mid (k = 20 holds), worst gadget case 939,160 (needs k = 21), ESTIMATE on that structure. The trace heights, and so `w_stack`, are PENDING: they need `L` tracegen over one node proof (`LS:141-146`); the bench-node proxy gives 37.5 M cells, about `w_stack` 36 |
| D11 | Direct advice writer for the k = 20 decider against its `synthesize` | about 1 week of build | the 0.1 to 0.3 s wrap advice fill (stock layouter 7.15 s, MEASURED) | PENDING |
| D12 | Path C cells: leaf at stacked height 17, 18, 20 and 24; node of arity 1 at internal parameters | about 15 minutes of GPU after one bench rebuild | the 1.3 to 3.2 s STARK part of path C | PENDING |
| D13 | GPU wrap at k = 20 with configs `cpu;all`, 2 repetitions, idle host | about 15 minutes | the first whole-prover GPU wall time with today's kernels | MEASURED once under load (load average 16 to 24, `GW:11`): CPU 243.3 s, full GPU 95.0 s, GPU without the GPU MSM 131.6 s, byte-identical proofs, ledger 9 verifies; quotient numerator 101.6 s against 8.4 s. A quiet host proves the decider on the CPU in 68.1 s. The idle-host repeat, k = 21 and k = 19 are PENDING |
| D14 | NVRTC quotient numerator for the k = 20 decider graph at 0 differences | about 1 week of build | 7.94 s against 0.34 to 0.67 s | PENDING |
| D15 | M7: the G4 wrap circuit with the wrap-rows bundle, V+, the anchor statement checks, the DAG key check and the direct writer; its cost model on the real `L` shape; one proof through the ledger 9 verifier | the wrap build | the k of the wrap key and every wrap row figure | PENDING |

## 2. Transcript vectors T-FS

Gate check G3 needs them. Each vector gives the same digits and the same final state (`SB/crates/stark-backend/src/transcript/multi_field.rs:82-100`) on the CPU prover, the GPU prover, the stark-backend verifier and the wrap's verifier of `L` under the mock prover. One fixed vector per transition of the `L` transcript (`ARCHITECTURE.md` section 1.8):

1. the key pre-hash, then one sample;
2. 1 to 8 words observed, then a sample (each partial pack);
3. 16 words, then a sample (no added permutation);
4. 9 words, then a commitment (flush, then the digest);
5. 11 samples in a row (digit refill at the 6th, a permutation at `sample_idx` 0 at the 11th);
6. sample, observe, sample (dropped digits);
7. one non-final WHIR round of `L` with both OOD points, both answers, the query PoW, the query indices and `gamma`;
8. the full transcript of one `L` proof from D10;
9. one vector each for the fixed-length hashes `u` = 1, 2 and 4 against Midnight's CPU sponge with the Sherlock initial capacity.

Status: PENDING.

## 3. Native-routed anchor controls T2-N and T2-N0

They run with the atomic-chunk test T2 on the XC-9 devnet (`chunky.compact`). OD-8 (hybrid contracts) is gated on T2-N.

- **T2-N.** `chunky.compact` gains an entry `nat`, routed native under its own key, with the native anchor prologue (`ARCHITECTURE.md`, kit COMPACT FRONTEND). A `nat` call placed between chunks `c1` and `c2` is refused: either no proof exists for its prologue (the anchor holds `A_1`), or a proof made against the state before the run fails the anchor `popeq` on the ledger.
- **T2-N0 (control).** The same entry without the prologue, between `c1` and `c2`, is included and its write lands inside the run. This shows that T2-N can detect an interleaving.

Status: PENDING.

## 4. Pending evidence

Source: `PROPOSAL.md` section 13.

| Item | What is missing |
|---|---|
| Bench write-up | every Plonky3 cell, the remaining OpenVM CPU cells, CPU thread scaling, CPU nodes; ranking by seconds per proved leaf step |
| E-1 tests | the remaining T2 mutants and the `label_d` link; T3 to T6; T0 on calls with a fallible transcript or non-empty effects; the bind window under variable-length SHA-256; D4 |
| Atomic chunks | T1 and T2 results in the `atomic-chunks` worktree, with the native-routed controls T2-N and T2-N0; T3 after frontend M5; the source of the claim address and `ep_hash` per chunk under placement V |
| GPU wrap | the k = 21 and k = 19 rows; a quiet-host repeat at k = 20; idle MSM (D2, D9); the G4 wrap on the GPU |
| G4 wrap rows | M7 (D15): the real wrap circuit and cost model; the k of the key |
| `L` heights | D10: the trace heights of `L` by tracegen over one node proof, then `T`, the distinct heights and `w_stack`; keygen on the BN254 root configuration; the log add-on AIRs |
| Soundness calculator | D3 again on the frozen sets of G1, `L` at the `w_stack` of D10; the calculator change in the prover and verifier |
| Lean | the restatement after OD-1 to OD-3 |
| Security qualification | gate checks G1, G3, G4 and G5, and the rest of G2 (`ARCHITECTURE.md` section 1.9) |
| Theorem mapping (G5) | fetch WHIR (ACFY24, ePrint 2024/1586) and read it in full, with BCGM26; then the mapping from WHIR Theorem 5.2 and the mutual correlated agreement theorem to every frozen set, the two-OOD protocol included |
| Transcript vectors T-FS | section 2 |
| Papers | WHIR, BCGM26, Jagged PCS, LogUp-GKR, Twist and Shout, WARP, Arc, Blaze, LatticeFold+, Neo, StarkPack, Accumulation Without Homomorphism full version: not on disk, not cited |
| Fr Poseidon kernel rate | D7: 5 to 15 M permutations per second |
| Preprocessing savings | the cached trace on and off; a planned shard against a generic shard in seconds per proved step (M13) |
| GP-3 cost | foreign-proof shards (M11) |
| Wrap on chain | verifier-key bytes against the 50,000-byte limit, proof bytes, verify time and fee at `2 + L_TX` public inputs; devnet and Preview deploy of the wrap key |

## 5. Machine-level experiment C1

C1 is a halo2 prototype of the dense memory backend in the spike crate `crates/mori-chip`. Its MockProver tests and row counts measure that prototype circuit. They carry over to the machine-level kits as tested invariants of the memory mechanism; the row counts and the gate `k_leaf <= 16` below are figures of the halo2 prototype and are not gates of the STARK leaf shards.

### C1 dense tail-buffered memory

First memory backend: the dense tail-buffered Merkle memory of `research/sherlock-runlength-consensus-20261111/CONSENSUS.md` 1.2.3 item 2, with the invariants of `AUDIT.md` finding 8, behind a MEM interface that a write-once LogUp backend can implement later. The second backend is not built.

#### What was built

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

#### Commands and runs

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

#### Tests

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

#### Measured rows (MEASURED, run `20261111T065836Z-c1-rows`)

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

#### Leaf rows at S = 16, 64, 128 (ESTIMATE, composed from the measured parts)

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

#### Node rows (ESTIMATE, not measured)

The node grows with the leaf's columns and lookups, and by about 4 rows per bit of leaf k (design 2.7.11 fit: 6,004 rows per fixed column, 7,298 per advice column, 8,189 per equality column, about 22,600 per lookup). MEM and the backend add no advice column, no equality column (the leaf already has all 12 advice columns in the permutation) and no lookup (the window uses the existing bus). They add 6 fixed columns: `kfix` and 3 packed code columns (MEM), 2 packed code columns (backend). They remove the 2 code columns of M3 (`SegMemConfig`, `WalkConfig`). Net +4 fixed columns: about +24,016 node rows, and +12 for leaf k 13 to 16, so the node goes from 823,466 to about 847,494 rows (gate 950,000). TOP adds nothing to the node. Packing the new codes into the leaf's existing code columns could remove part of the +4; not tried. The node was not run over a leaf with this backend: that needs the backend wired into `RfLeaf` in place of M3.

#### Arenas (R-6)

MEASURED: four arenas with one flush unit each cost 4 x 12,137 rows of `persist` (49,172 harness rows), against 12,281 for one arena. ESTIMATE: four arenas sharing two flush units (lanes A, B) would be 4 x (12,137 - 8,541) + 2 x 8,541 = 31,466 rows plus the arena-select multiplexers (not built). The one-arena variant with tagged records is 12,137 at W = 1. Not done: the mechanical check of the 108-rule non-interleaving claim over the reference machine. It stays open (needs an assertion in `mori-core` run on the corpus, or a Quint model).

#### Deviations from the audit text (for `DECISIONS.md`)

- Domain tags are in the sponge capacity, not in the first input as AUDIT finding 8 says. Same separation (the capacity is fixed by the circuit and differs per domain, height and input length); cost 0 permutations against about 2x on every node hash (flush unit 17,080 and cold read 9,100 rows at TOP = 61, ESTIMATE).
- An active window row with kind 0 is allowed (no reader can request kind 0, and its key is still injective).
- `q_post = q_pre + sum f_j` is not a separate gate: it follows from the `act` prefix, the address chain and the decomposition of `len_post`.

#### Quint lane (Gemini, argon-run) for this circuit change

Modules: MEM frontier and window (`membackend.rs`, `WindowCfg`), dense `persist` and `cold_read` (`dense.rs`), the CORE port assumptions of the harness (append chain from `len_pre` to `len_post`, read route and `addr < len_cur`, kind from the rule). Properties:

1. `(len, root, tail_d)` after a leaf sequence is a function of the append sequence only, under any partition.
2. A flush happens iff a segment boundary is crossed; no segment slot is written twice; the old leaf is `EMPTY_SEG`.
3. `tail_d` is canonical, and the boundary passes through unchanged when a leaf appends nothing.
4. `8 addr + kind` is injective (kind in `0..=4`), also across four arenas through the kind pin.
5. Window and cold providers are disjoint (`addr >= base` against `addr < base`).
6. Every read is below `len_cur` and carries the kind the rule expects.
7. `len <= 2^(TOP + 3) - 1`; at TOP = 61 `2^64 - 1` is reachable and `2^64` is not; a leaf at the last segment is provable for any W.
8. Appends and new window rows correspond one to one (rests on CORE's append chain).

#### Not covered and open

- The LogUp backend (C1 second half) is not built. The trait keeps the token opaque so that it can return obligation cells.
- Hash-chain FRAMES and TASKS stacks are not built.
- The backend is not wired into `RfLeaf`; the leaf and node figures above are composed, not measured.
- The CORE stand-in reads after all appends or at `len_pre`; a read between two appends of one leaf needs the step's own `len` from CORE.
- An active window row with kind 0 is allowed (no reader can request kind 0). A gate `act * kind != 0` needs an inverse witness; not added.
- Write-before-read of scratch registers (AUDIT finding 9, R-8) is a CORE obligation; C1 does not cover it.
- The per-step maximum of cold reads, appends and arenas (AUDIT finding 6) is not measured; the leaf rows above depend on it through `N_page` and W.
- Cold reads of records in segment `q_pre` that the leaf flushes are served by the window, not by the backend, as designed.
