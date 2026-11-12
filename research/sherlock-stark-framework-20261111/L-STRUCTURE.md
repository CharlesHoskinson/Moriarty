# L-STRUCTURE: measured shape of the last layer L, plugged into the wrap model

**Verdict.** With the measured structure of L (default `VerifierConfig`, mid case), the wrap needs **2,440,107 rows in shape 1** (70-row Poseidon). That is 116.4 percent of usable rows at k = 21 (WRAP.md had 126.8 percent), so shape 1 **does not fit k = 21**. Shape 2 needs **434,692 rows**, 41.5 percent of usable rows at k = 20 (WRAP.md had 46.0 percent), so **k = 20 holds**.

Shape 2 stays inside the k = 20 gate (891,284 rows, 85 percent of usable) in every measured case except one. The exception is the worst gadget case, with no limb overlap and 60 percent packing: 939,160 rows, which needs k = 21. k = 20 also holds when `w_stack` is raised to 36 to hold node-sized heights (mid 468,675 rows).

The structural inputs are MEASURED by keygen. The trace heights of L are NOT MEASURED, because no proof was made. The GKR depth `T`, the distinct-height count and the final `w_stack` therefore remain ESTIMATE, bounded by the `w_stack` 36 sensitivity rows.

Date: 2026-11-11. Keygen only: no tracegen and no proof. The build and the run each ran through `heavy-run` inside `flock ~/.cache/cost-opt/measure/heavy.lock`. No existing bench cell was changed. Ollama was not used. Nothing was committed or pushed.

Files next to this one:

- `L-STRUCTURE.csv`: one row per AIR for `L_cached`, `L_nocached` and `node`.
- `L-STRUCTURE-params.csv`: the stacked PCS parameters per variant, plus the S4 constants of the model.
- `L-STRUCTURE.json`: the full dump, with totals and parameters per variant and the DAG node kinds per AIR.
- `l_structure_post.py`: converts the raw dump to the CSV and the JSON.
- `l_structure_example.rs`: a copy of the keygen program.
- `wrap_model.py`: the cost model, which now takes `--measured`.

## 1. What was keyed

The keygen program is `~/.cache/stark-bench/src/openvm-org_openvm/crates/stark-bench-ovm/examples/l_structure.rs`. It is a new example in the bench crate, with no `Cargo.toml` change. It was built with the campaign flags: `--release --features cuda`, `CUDA_ARCH=121`, `RUSTFLAGS=-C target-cpu=native` and target dir `~/.cache/stark-bench/target/ovm`. Raw output: `~/.cache/stark-bench/logs/l-structure/l-structure.json`.

The chain uses verifying keys only:

1. W1 (`Poseidon2SubAir`, the bench leaf) is keyed at app params.
2. The node is `VerifierSubCircuit::<2>` over the W1 vk at leaf params. This is the bench node, and it is the sanity anchor. Keygen gives 39 AIRs and 1,531 common-main columns, the same numbers that `ovm-gpu-node2-20.json` measured. The node also has 12 cached columns (the child DAG), so its total width is 1,543.
3. L is `VerifierSubCircuit::<1>` over the node vk at `root_params_with_100_bits_security()`. L was keyed in two variants:
   - `L_cached`: the default `VerifierConfig` (`has_cached = true`), with the child DAG as a cached trace.
   - `L_nocached`: `has_cached = false`, which is the setting of OpenVM's `RootProver`.

Engine: L was keyed with `BabyBearPoseidon2CpuEngine` at root params. The BN254 root config (`baby_bear_bn254_poseidon2`) is behind the stark-sdk feature `baby-bear-bn254-poseidon2`, which the bench crate does not enable, and turning it on needs a manifest change. The AIR list does not depend on L's own hash, because `airs::<SC>()` fixes only the config type, so the per-AIR structure is the same. The three continuation AIRs of `RootCircuit` (user public values, memory) expect a VM child and were not keyed.

## 2. Measured totals against the old guesses

| Input of the wrap model | Old mid (ESTIMATE) | `L_cached` (MEASURED) | `L_nocached` (MEASURED) |
|---|---:|---:|---:|
| AIRs | 41 (39 + 2 log) | 39 (+2 log add-on = 41) | 39 (+2 = 41) |
| Trace columns (preprocessed + cached + common) | 1,600 | 1,537 (1,525 common + 12 cached) (+69 log = 1,606) | 1,829 (+69 = 1,898) |
| AIRs with `need_rot` | all assumed | 35 of 39 (no rotation: Poseidon2Air, MerkleVerifyAir, WhirFoldingAir, FinalPolyMleEvalAir) | 35 of 39 |
| Column openings (`EF`) | 3,200 (2 per column) | 2,670 (+138 log = 2,808) | 3,254 (+138 = 3,392) |
| Interactions | 700 | 487 | 487 |
| Message fields (sum over interactions) | 3,500 (700 x 5) | 2,687 (mean 5.52) | 2,687 |
| Constraints | 3,500 | 2,210 | 2,522 |
| Constraint DAG nodes | 15,000 | 15,331 | 18,088 |
| of which `EF` products / products by a constant / add, sub, neg | 6,000 / 1,500 / 7,500 | 5,541 / 923 / 5,994 | 6,134 / 1,122 / 7,494 |
| of which variables / constants / selectors (free in the wrap) | not split | 2,290 / 513 / 70 | 2,597 / 669 / 72 |
| Max constraint degree | 4 | 4 (3 for 4 AIRs, 2 for 2 AIRs) | 4 |
| Commitments `C` | 2 | 2 (common main + 1 cached part; no preprocessed AIR) | 1 |
| Public values | 200 | 0 (log add-on keeps 200) | 8 |
| Bus ids | not counted | 78 (0 to 35, 38 to 79) | 78 |

The node (`VerifierSubCircuit::<2>` at leaf params) has almost the same structure: 500 interactions, 2,212 constraints and 14,981 DAG nodes. The AIR shapes of the recursion circuit hardly depend on the child vk. WRAP.md was correct that there is no preprocessed trace.

With `has_cached = false`, the child DAG moves into the common main trace: SymbolicExpressionAir goes from 24 to 316 columns and from 249 to 3,006 DAG nodes. The cached commitment goes away. In the wrap these effects cancel to within 2 percent (section 4).

Largest AIRs in `L_cached` (full list in the CSV):

- Poseidon2Air: 301 columns, 282 constraints, 2,692 DAG nodes, degree 3, no rotation.
- ProofShapeAir<4, 8>: 77 interactions, 295 message fields, 949 DAG nodes.
- InitialOpenedValuesAir: 89 columns, 855 DAG nodes.
- EqBaseAir: 789 DAG nodes.
- SumcheckRoundsAir: 771 DAG nodes.
- OpeningClaimsAir: 732 DAG nodes.

There is no after-challenge trace in stark-backend v2.1.0 (`TraceWidth` has only preprocessed, cached and common main), so the CSV writes 0 in that column.

## 3. PCS parameters of L

Stock root params, read from the keyed `pk.params`: `l_skip` 2, `n_stack` 18, `w_stack` 18, log blowup 4, `k_whir` 4, 3 WHIR rounds with **61, 30 and 20 queries** (list decoding, `m = 1`), final polynomial 2^8 = 256 coefficients, PoW of 15 bits for `mu`, folding and query, LogUp PoW 18, max constraint degree 4.

The model keeps the S4 constants of WRAP.md section 1 for the verdict: 57, 28 and 19 queries, with the owner's 21-bit folding and `mu` PoW. The stock query counts are shown below as a separate variant row. `--measured` does not overwrite the PCS constants.

## 4. Wrap rows from the measured structure

Commands:

- Verdict rows: `python3 -I wrap_model.py --measured L-STRUCTURE.json --variant L_cached`.
- Variant rows add `--variant L_nocached`, `--queries 61,30,20`, `--ws 36` and `--heights proxy`.

Without `--measured`, the model prints the old ESTIMATE tables unchanged. This was checked with a byte diff against the previous version (mid 2,658,487 and 482,173).

What `--measured` changes:

- It replaces A, W, the openings (`width x (1 + need_rot)`, as in `SB/verifier/transcript_extractor.rs:985` and `SB/proof.rs:138`), I, the message-field sum, the DAG split, Nc and C.
- It drops the old `0.3 x message fields` product term of line 10, because the measured DAG already contains the interaction expressions.
- The +2 log AIRs and their 69 columns remain a labelled ESTIMATE add-on.
- Low, mid and high now differ only in knobs that are not structural: `w_stack` (16 or 18), the coset lever, +1 commitment in high, the public-value count and the area.

Model changes outside `--measured`: the round-0 leaf hash now costs ceil(`w_stack` / 16) permutations per row of the main commit, and packing above `w_stack` 32 is ceil(`w_stack` / 8). Both formulas give the old values at `w_stack` 16 and 18.

Usable rows (2^k - 7) and gates (85 percent of usable):

| k | Usable rows | Gate |
|---|---:|---:|
| 21 | 2,097,145 | 1,782,573 |
| 20 | 1,048,569 | 891,284 |
| 19 | 524,281 | 445,639 |

| Case | Low | Mid | High | Old mid (ESTIMATE) |
|---|---:|---:|---:|---:|
| Shape 1, 70-row Poseidon, `L_cached` | 2,064,403 | **2,440,107** | 2,652,569 | 2,658,487 |
| Shape 1, 19-row stdlib Poseidon, `L_cached` | 1,703,221 | 1,836,675 | 1,900,574 | 2,050,057 |
| **Shape 2, model, `L_cached`** | 393,581 | **434,692** | 454,423 | 482,173 |
| Shape 2, no limb overlap | 662,070 | 704,370 | 724,988 | 784,384 |
| Shape 2, 60 percent packing | 524,774 | 579,589 | 605,897 | 642,897 |
| Shape 2, both (worst) | 882,760 | 939,160 | 966,651 | 1,045,846 |
| Shape 2, model, `L_cached`, proxy heights (`T` 22) | 392,403 | 433,515 | 452,421 | |
| Shape 1, 70-row, `L_nocached` | 2,099,239 | 2,362,171 | 2,574,703 | |
| Shape 2, model, `L_nocached` | 409,149 | 441,279 | 461,016 | |
| Shape 2, worst, `L_nocached` | 931,086 | 976,897 | 1,004,396 | |
| Shape 1, 70-row, `L_cached`, stock queries 61/30/20 | 2,115,361 | 2,514,583 | 2,741,613 | |
| Shape 2, model, `L_cached`, stock queries | 402,712 | 445,952 | 467,005 | |
| Shape 2, worst, `L_cached`, stock queries | 902,596 | 961,942 | 991,195 | |
| Shape 1, 70-row, `L_cached`, `w_stack` 36 | 2,260,925 | 2,609,579 | 2,822,041 | |
| Shape 2, model, `L_cached`, `w_stack` 36 | 431,803 | 468,675 | 488,406 | |
| Shape 2, worst, `L_cached`, `w_stack` 36 | 949,157 | 998,361 | 1,025,852 | |
| Shape 2, model, `w_stack` 36 and stock queries | 443,589 | 482,296 | 503,348 | |
| Shape 2, worst, `w_stack` 36 and stock queries | 973,600 | 1,025,250 | 1,054,503 | |

Results:

- **Shape 1 does not fit k = 21** in any measured case. The mid case is 657,534 rows over the gate. Even the 19-row Poseidon variant is over the gate in the mid case (1,836,675, which is 54,102 over). It fits only in the low case (1,703,221, 95.5 percent of the gate).
- **Shape 2 fits k = 20** in every case except the worst gadget case. That case (939,160 mid) is 47,876 rows over the k = 20 gate and uses 44.8 percent of usable rows at k = 21. Mid has 456,592 rows of margin at the k = 20 gate. With `w_stack` 36 and stock queries, mid is 482,296 rows, which still fits k = 20.
- Shape 2 at k = 19: the low case (393,581) and the mid case (434,692, 97.5 percent of the gate) fall under the k = 19 gate. The high case, the stock-query rows and the `w_stack` 36 rows go over it. k = 19 is not a safe target.

Where the measured rows changed (mid, `L_cached`, shape 1 rows):

| Line | Old | Measured |
|---|---:|---:|
| Constraint DAG (line 8) | 213,000 | 192,992 |
| Constraint batching (line 9) | 169,968 | 108,048 |
| Interactions (line 10) | 145,592 | 83,726 |
| Stacked reduction per column (line 12) | 473,616 | 423,476 |
| eq_3b per interaction (line 6) | 50,360 | 36,302 |
| Transcript | 183,389 | 173,001 |

Lines 8, 9, 10 and 12 together fall from 1,002,176 to 808,242 rows. The part of L that WRAP.md had to guess is now measured, and the guess was about 19 percent too large.

## 5. What is still not measured

1. **Trace heights of L.** A verifying key does not fix heights. They come from tracegen over a real proof of the node, which this task does not make. The CSV writes `height_used = NOT MEASURED` and gives `height_proxy_node2`. That proxy is the height that the AIR with the same name had in the measured 2-child bench node (`ovm-gpu-node2-20.json`), and it is not a height of L. The heights enter the model in three places:
   - `T`: with the proxy heights, `T = l_skip + n_logup = 22`. With the area rule of the scenarios, `T = 23`. The rows change by less than 0.3 percent (for example 433,515 against 434,692 for shape 2 mid).
   - The distinct-height count (17 with the proxy, the same as the scenario).
   - The `w_stack` fit: the proxy has 37.5 M cells, which is more than root `w_stack` 18 allows (18 x 2^20 = 18.9 M). Poseidon2Air alone, at 301 columns and the proxy height 2^16, is 19.7 M cells. The child of L is a 39-AIR proof at leaf params with 118, 84 and 81 queries, so its heights need not be half the node's. The `w_stack` 36 rows in section 4 size the stacked width to the proxy area (37.5 M / 2^20 = 35.7). Shape 2 still fits k = 20 there (mid 468,675), and shape 1 gets worse (2,609,579). A larger `n_stack` in place of a larger `w_stack` would add Merkle depth and sumcheck rounds instead; it is not modelled.
   - Cheapest fix: prove one node over W1 (an existing bench cell), then run L tracegen only (`generate_proving_ctxs_base`, no L proof) and read the trace heights.
2. **BN254 config.** L was keyed with the BabyBear engine. The structure is the same, but a run with `--features openvm-stark-sdk/baby-bear-bn254-poseidon2` would confirm it on the real root config.
3. **Log add-on.** The 2 log AIRs and 69 columns are the WRAP.md estimate. Their constraints and interactions are not in the measured counts.
4. **Gadget costs, lane packing and the 19-row Poseidon** stay as in WRAP.md section 5 (items 2, 3 and 7).
