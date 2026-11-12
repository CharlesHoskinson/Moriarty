# S10x: multi-leaf node at k <= 21 and the fixed-size range table

Date: 2026-11-12. Lane s10x (LOOP-AGENDA S10, PLAN step 10, items "a multi-leaf k = 21 node" and "a fixed-size range table"). Owner asleep: every choice below is the recommended option, marked **lead decision, owner to confirm**. Nothing was pushed. Ollama was not used.

Labels: MEASURED is read from a run record in `s10x/`. DERIVED is computed by a stated rule from MEASURED numbers. ESTIMATE is a model figure with named inputs.

## Verdict

1. **Multi-leaf node.** Over W1 leaf proofs (2^20 rows, app parameters), the tallest AIR of the internal node is `FinalPolyQueryEvalAir`, with exactly 117,800 valid rows per child proof (MEASURED at arity 1 to 12). At the OD-19 internal set the binding cap is not k = 21. It is the stacked height `2^(l_skip + n_stack)` = 2^19 of that set. **For this leaf (stacked width 19), the largest arity that fits is 4**: 471,200 valid rows, padded height 2^19 = 524,288, margin 53,088 rows (10.1 percent) to the 2^19 cap and 1,258,285 rows to the k = 21 gate. That node proves and verifies at the OD-19 set (MEASURED). The bound holds for app leaves of stacked width at most about 336; a wider leaf makes `InitialOpenedValuesAir` the tallest AIR (DERIVED, section 2 scope notes). Arity 5 and above fail the stacked layout with `LayoutHeightExceeded { log_height: 20, log_stacked_height: 19 }` (MEASURED). By trace height alone, arity 8 is the largest at padded height <= 1,782,573 (2^20; MEASURED tracegen). Arity 12 pads to 2^21 = 2,097,152 rows, above the k = 21 gate (MEASURED). **The gate rule fired**: the sweep stopped at arity 12, and arities 15, 16 and 32 did not run. Experiment 2 started in the same second (04:21:00, `s10x/runs.log`), because the run script ran the two experiments in sequence; see LD-S10x-5.
2. **Fixed-size range table.** The FP-1 RANGE table (`(x, w)`, `w` in 0..=16) is the stock `VariableRangeCheckerAir` at `range_max_bits` 16: 131,072 rows (2^17), 4 main columns, 0 preprocessed columns, 1 interaction (MEASURED). It does not change the tallest AIR of the node at any arity, and so it does not change the largest arity (4). It raises a leaf below 2^17 rows to 2^17, adds 589,824 main cells to each leaf (table plus test sender), adds 63,042 node cells per child and at most 1 stacked column to the node (MEASURED).
3. **No result needs k = 22.** The one row above the gate (arity 12, padded 2^21) is a probe outside every admitted shape.

## 1. Setup

Bench crate `~/.cache/stark-bench` (OpenVM stark-backend v2.1.0, recursion circuit of the openvm checkout used by D10). New example `s10x_node` (copy: `s10x/s10x_node.rs`), built as `cargo build --release --locked -p stark-bench-ovm --example s10x_node` (`s10x/build.sh`). Every build and run went through `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run`; the runs ran in the `systemd-run --user` unit `s10x-runs` (`s10x/runs.sh`, log `s10x/runs.log`, raw files in `~/.cache/s10x/`).

Chain, as in D10 (`Moriarty-wt/d10-heights/research/sherlock-stark-framework-20261111/D10-HEIGHTS.md` section 1):

1. `N` distinct W1 leaf proofs (Poseidon2 AIR, `app_params_with_100_bits_security(MAX_APP_LOG_STACKED_HEIGHT)`: log blowup 1, `l_skip` 4, `n_stack` 20, queries 193/88/81/81, final polynomial 2^8), each proved and verified on the CPU engine. Fixed-size shards: every leaf of one run has the same shape.
2. Node `VerifierSubCircuit::<N>` over the leaf key at the OD-19 internal set: unique decoding, log blowup 3, queries 97/82/81, folding PoW 0, `mu` PoW 9, query PoW 21, LogUp PoW 18, `l_skip` 2, `n_stack` 17, `w_stack` 512. Keygen, then tracegen (`generate_proving_ctxs_base`, child DAG as a cached trace).
3. Per AIR: padded height, valid rows (the last row with a nonzero cell plus one), widths and interactions. Most recursion trace generators fill padding with zeros, so this count is the valid row count for them (for `FinalPolyQueryEvalAir` it equals the formula of section 2). `Poseidon2Air` and `ExpBitsLenAir` have nonzero padding rows: their count equals the padded height at every arity, so their valid rows are not known from these runs. The stacked layout uses `StackedLayout::new(l_skip, l_skip + n_stack, ...)` as the verifier does. An error from it is recorded, and the stacked height is not raised.
4. Arities ascend. The run stops after the first arity whose tallest padded AIR is over the gate 1,782,573 rows (85 percent of `2^21 - 7`).

Cross-check: the arity-2 node has 37,512,232 main cells, which is the D10 anchor value (D10-HEIGHTS section 1), MEASURED.

Host load: the load average was 17 to 31 during experiment 1 and rose to 414 during experiment 2 (other lanes; `s10x/runs.log`). The times below are not cost figures.

## 2. Experiment 1: multi-leaf node

Child: W1 2^20 at app parameters. Node: OD-19 internal set. Run `s10x/e1-w1-20.jsonl`.

| Arity | Tallest AIR | Valid rows | Padded height | Margin to gate 1,782,573 (padded) | Stacked layout at 2^19 | `w_stack` | Main cells | `T` |
|---:|---|---:|---:|---:|---|---:|---:|---:|
| 1 | FinalPolyQueryEvalAir | 117,800 | 2^17 = 131,072 | 1,651,501 | fits | 37 | 18,781,016 | 21 |
| 2 | FinalPolyQueryEvalAir | 235,600 | 2^18 = 262,144 | 1,520,429 | fits | 73 | 37,512,232 | 22 |
| 3 | FinalPolyQueryEvalAir | 353,400 | 2^19 = 524,288 | 1,258,285 | fits | 139 | 71,902,616 | 23 |
| **4** | FinalPolyQueryEvalAir | **471,200** | **2^19 = 524,288** | **1,258,285** | **fits** | **144** | **74,974,664** | **23** |
| 5 | FinalPolyQueryEvalAir | 589,000 | 2^20 = 1,048,576 | 733,997 | `LayoutHeightExceeded` (20 > 19) | none | 143,525,544 | 24 |
| 6 | FinalPolyQueryEvalAir | 706,800 | 2^20 | 733,997 | exceeded | none | 143,755,432 | 24 |
| 8 | FinalPolyQueryEvalAir | 942,400 | 2^20 | 733,997 | exceeded | none | 149,899,528 | 24 |
| 12 | FinalPolyQueryEvalAir | 1,413,600 | 2^21 = 2,097,152 | **-314,579 (over; stop)** | exceeded (21 > 19) | none | 287,461,064 | 25 |

All MEASURED. The second tallest AIR is `Poseidon2Air` at 2^(15 + log2 a) rounded up (2^17 at arity 4, 2^18 at arity 8, 2^19 at arity 12). Every node has 39 AIRs.

Arity 4 at the OD-19 set: proved in 30.0 s on the CPU (20 threads, loaded host) and verified, MEASURED. With the RANGE child of section 3 it proved in 77.7 s (load above 100) and verified, MEASURED.

Rules, DERIVED from the table:

- Valid rows of the tallest AIR are exactly `117,800 a`. The term is `sum_r (q_r + 1)(k_whir (R - r - 1) + 2^8)` over the child's WHIR rounds (`crates/recursion/src/whir/final_poly_query_eval/trace.rs`, `compute_round_offsets`): 194 x 268 + 89 x 264 + 82 x 260 + 82 x 256 = 117,800. It depends on the child's query vector and final polynomial, not on the child's height (the 2^16 child of section 3 gives the same 117,800).
- Padded height is `2^ceil(log2(117,800 a))`. Arities 9 to 11 give 1,060,200 to 1,295,800 valid rows, which pad to 2^21 and are over the gate. They were not run (stop rule).
- At the OD-19 internal set the cap is 2^19 = 524,288 rows: `a <= 4` (4 x 117,800 = 471,200; 5 x 117,800 = 589,000 > 524,288).
- By padded height against the k = 21 gate: `a <= 8`. By valid rows against the gate, `a <= 15` (1,767,000 rows, margin 15,573), but a STARK trace is padded to a power of two, so this bound has no use.

**Lead decision LD-S10x-5, owner to confirm.** The k = 21 gate event of arity 12 was a probe above every admitted shape, and the stop rule ended that sweep. Experiment 2 is independent of it and ran after it. It is kept as a result.

**Lead decision LD-S10x-1, owner to confirm.** For a BabyBear trace, "k" is the log2 of the padded trace height, and the gate test uses the padded height. The EXPERIMENTS hard-rule line already says a BabyBear height of 2^22 is a STARK scaling point, not a circuit k; the binding STARK limit is the stacked height of the layer's parameter set.

**Lead decision LD-S10x-2, owner to confirm.** The universal internal node over leaf proofs has fixed arity at most 4 at the OD-19 internal set, and the set keeps `n_stack` 17. Arity 4 is MEASURED for a child of stacked width 19; the plan's shape menu must keep every leaf shape at or below the width limit of the chosen arity (336 at arity 4, 672 at arity 2, DERIVED), or the node arity drops for wide shapes. This holds only after the wide-leaf run of section 5. Raising `n_stack` to 18 (arity 8) changes the frozen internal set, so D3 must run again. The NODE INTEGRATION kit has two child slots (arity 2), which is inside the bound. The per-leaf figure "6.50 s per 2^20 leaf under arity-8 nodes" (PROPOSAL section 2; ARCHITECTURE NODE row budget) uses arity 8, and that node does not fit the OD-19 set. Recommended: replace it with an arity-4 figure when the GPU node cell runs (PENDING).

Scope notes:

- The children are app-parameter leaf proofs. A node over node proofs (children at the internal set) has a different per-child row count. That was not measured here; D10 section 2 shows that the child's parameter set drives the heights.
- In the full tree, the layer that verifies app leaves can also be the leaf-aggregation set (log blowup 2, `l_skip` 4, `n_stack` 17: stacked height 2^21, PROPOSAL section 11.2). Node heights depend on the children and not on the node's own parameters (D10 section 1), so at that set arity 8 fits (2^20) and arity 9 to 15 pad to 2^21, which is inside the set but over the gate. DERIVED; not proved at that set.
- **The bound depends on the leaf's stacked width.** Both leaves of this lane stack to the app set's stacked height 2^24, so `MerkleVerifyAir` has the same rows for a 2^16 and a 2^20 child (31,796 at arity 2, MEASURED). The term that moves is `InitialOpenedValuesAir`: 3,088 rows per child at child stacked width 2 and 9,264 at width 19 (MEASURED), which fits `193 x 16 x ceil(w / 8)` (DERIVED from these two points). At arity 4 it reaches the 2^19 cap at `ceil(w / 8)` = 43, so arity 4 holds for app leaves of stacked width at most 336 (DERIVED); the app set admits `w_stack` 2,048. At arity 2 the width limit is 672 (DERIVED). `Poseidon2Air` may also grow with width; its valid rows are not known (section 1 item 3). A wide-leaf run is open (section 5).

## 3. Experiment 2: fixed-size range table

The design table (final-design `ARCHITECTURE.md`, FP-1 bus registry, bus 5 RANGE): message `(x, w)` with `w: tag(5)`, `0 <= w <= 16`; one row for each `w` in 0..=16 and `x < 2^w`, which is 2^17 - 1 rows; the text says "preprocessed".

The nearest stock AIR is `VariableRangeCheckerAir` (`openvm-circuit-primitives` `var_range`) with `range_max_bits` 16. MEASURED shape: 131,072 rows (the 2^17 - 1 entries plus one closing row with `max_bits` = 17 and multiplicity 0), 4 main columns (`value`, `max_bits`, `two_to_max_bits`, `mult`), 0 preprocessed columns, 1 interaction. The AIR walks the rows with transition constraints instead of a preprocessed trace.

**Lead decision LD-S10x-3, owner to confirm.** The RANGE table of FP-1 is the stock `VariableRangeCheckerAir` at `range_max_bits` 16 (2^17 rows, 4 main columns, no preprocessed trace). Its row set is the FP-1 row set. The FP-1 text changes "preprocessed" to "fixed by constraints". A preprocessed form is not needed and changes no height.

Test leaf (mode 2 of `s10x_node`): W1 plus the table plus a sender AIR of 2^14 rows by 4 columns, each cell one 16-bit RANGE lookup (65,536 lookups per leaf), with the real multiplicities. Runs `s10x/e2-w1r-20.jsonl`, `s10x/e2-w1-16.jsonl`, `s10x/e2-w1r-16.jsonl`, compared with `e1-w1-20.jsonl`.

### 3.1 Leaf

| Leaf | Tallest AIR, padded | Stacked width (`n_stack` 20, `l_skip` 4) | Main cells |
|---|---|---:|---:|
| W1 2^20 | Poseidon2, 2^20 | 19 | 313,524,224 |
| W1 2^20 + RANGE + sender | Poseidon2, 2^20 | 19 | 314,114,048 (+589,824) |
| W1 2^16 | Poseidon2, 2^16 | 2 | 19,595,264 |
| W1 2^16 + RANGE + sender | **RANGE, 2^17** | 2 | 20,185,088 (+589,824) |

All MEASURED. The table adds 524,288 cells (131,072 x 4) and the sender 65,536 (DERIVED from the widths). The table sets a floor of 2^17 on the leaf height: a leaf shape below 2^17 rows rises to 2^17 when it carries the table. The path C leaves of PROPOSAL section 11.2 (stacked height 2^17 to 2^18) are at or above the floor.

### 3.2 Node

| Node | Tallest AIR (valid rows, padded) | `w_stack` | Main cells | `T` | Lifted interactions |
|---|---|---:|---:|---:|---:|
| arity 1, W1 2^20 | 117,800, 2^17 | 37 | 18,781,016 | 21 | 1,251,516 |
| arity 1, + RANGE | 117,800, 2^17 | 37 | 18,844,058 (+63,042) | 21 | 1,275,840 |
| arity 2, W1 2^20 | 235,600, 2^18 | 73 | 37,512,232 | 22 | 2,501,980 |
| arity 2, + RANGE | 235,600, 2^18 | 73 | 37,638,316 (+126,084) | 22 | 2,551,244 |
| arity 4, W1 2^20 | 471,200, 2^19 | 144 | 74,974,664 | 23 | 5,003,216 |
| arity 4, + RANGE | 471,200, 2^19 | **145** | 75,226,832 (+252,168) | 23 | 5,102,080 |
| arity 2, W1 2^16 | 235,600, 2^18 | 69 | 35,334,472 | 22 | 2,184,956 |
| arity 2, W1 2^16 + RANGE | 235,600, 2^18 | 69 | 35,480,440 (+145,968) | 22 | 2,241,036 |

All MEASURED. The two extra child AIRs (table and sender) change only small node AIRs: per child, `TranscriptAir` +515 rows, `GkrLayerSumcheckAir` +152, `Eq3bAir` +75, `EqBitsAir` +32 (2^20 child) and some AIRs of 1 to 8 rows. The tallest changed AIR stays at or below 2^13 at arity 4. The tallest AIR of the node, its padded height, `T` and the largest arity (4) do not change. The node cost is 63,042 cells per child, DERIVED (linear in arity in the three rows). Part of it is the sender, not the table. A real leaf already has sender kits, so the table's own share is below this figure.

### 3.3 Finding: a table with no lookups breaks the node proof

With the table at zero multiplicity and no sender (mode 1, W1 2^14, arity 2), the leaf proves and verifies, and the node tracegen gives the same heights. But the node proof fails verification with `BatchConstraintError(SumClaimMismatch)`, MEASURED (`s10x/smoke-range1-zero-mult-14.jsonl`). The same node over W1 alone (`smoke-range0-14.jsonl`) and over W1 + table + sender (`smoke-range2-14.jsonl`) verifies, MEASURED. The cause was not isolated. The error class is the one of the generic child key with absent preprocessed AIRs (`F/bench/RESULTS.md:109`).

**Lead decision LD-S10x-4, owner to confirm.** Every leaf shape of the plan's menu gets a node-proof test in the shape test suite, including the case of an unused chip, before the leaf key freezes. Until the cause is known, `sherlock plan` gives every shape that carries the RANGE table at least one RANGE lookup (the RANGE kit is the level-0 provider of every kit, so a real leaf does this already). This workaround is not tested: the passing case had a sender AIR and 65,536 lookups, not one lookup.

## 4. Effect on the design

- PROPOSAL section 11.2 internal row ("height at most 2^19") is the binding node cap. It admits arity 4 over app leaf proofs. The arity-8 bench numbers stay as measurements of a set that is not OD-19.
- The FP-1 RANGE table fits every leaf shape at or above 2^17 rows with no height change and moves the node by at most one stacked column.
- The wrap-side table of PROPOSAL section 3.5 (2^16 rows in the PLONK wrap) is a different table and has no leaf or node height. It is not changed here.

## 5. Open

- GPU node cell at arity 4 over 2^20 leaves at the OD-19 set (seconds per leaf with node share), to replace the arity-8 figure.
- Node over node children at the internal set: rows per child and the largest arity of an internal-over-internal node.
- Cause of the zero-lookup `SumClaimMismatch` (section 3.3), and a test with one lookup.
- Wide-leaf run: node over app leaves of stacked width near 336 at arity 4, to measure `InitialOpenedValuesAir` and `Poseidon2Air` against the 2^19 cap.

## Files

- `s10x/s10x_node.rs`: the example (also in `~/.cache/stark-bench/src/openvm-org_openvm/crates/stark-bench-ovm/examples/`, with an `[[example]]` entry in that crate's `Cargo.toml`).
- `s10x/build.sh`, `s10x/runs.sh`, `s10x/runs.log`: build and run scripts and the run log.
- `s10x/e1-w1-20.jsonl`, `s10x/e2-w1r-20.jsonl`, `s10x/e2-w1-16.jsonl`, `s10x/e2-w1r-16.jsonl`: raw records, one JSON line for the leaf and one for each node.
- `s10x/smoke-range0-14.jsonl`, `s10x/smoke-range1-zero-mult-14.jsonl`, `s10x/smoke-range2-14.jsonl`: the 2^14 smoke runs of section 3.3.
- `s10x/summarize.py`: `python3 -I s10x/summarize.py s10x` prints the tables.
