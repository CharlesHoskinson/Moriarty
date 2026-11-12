# D10-HEIGHTS: measured trace heights of the last layer L

**Verdict.** At the OD-19 parameter sets (L child at the internal UD set, L itself at the UD `L` set with log blowup 3), tracegen of `L` over one real node proof gives **`w_stack` 18** for `L` with the child DAG as a cached trace (17 stacked columns of common main plus 1 cached), **`T` = 21** (`n_logup` 19), **14 distinct heights**, maximum height 2^16 and 17,263,856 main cells, MEASURED. Without the cached trace (the OpenVM `RootProver` setting) `w_stack` is 26. A BN254 root proof of `L` at the UD set proves and verifies for both variants. `w_stack` stays far below the OD-3 bound of 48. At the measured `w_stack` the D3 weakest term of `L` is 101.1427 bits (query, every gate passes), and the G4 wrap row is 634,707 rows mid (666,513 high), so **k = 20 holds**.

Date: 2026-11-11. Branch `d10-heights` (from `main`, with `l-structure` commit 3957ddf7c merged). Ollama was not used. Nothing was pushed.

The D3 numbers in section 8 come from the current D3 calculator. Under UD the `L` and internal terms rest on results that `UD-SIZING.md` (line 7) marks DISCHARGED. The MCA bound under proof (OD-19, `research/sherlock-mca-proof-20261111/`) is behind the folding terms of the old list-decoding rows; those rows change if that proof changes the bound.

Files next to this one:

- `d10/l_heights_example.rs`: copy of the tracegen program (`~/.cache/stark-bench/src/openvm-org_openvm/crates/stark-bench-ovm/examples/l_heights.rs`).
- `d10/Cargo-toml-addition.txt`: the manifest addition of the bench crate (feature `bn254`, example entry). The bench binary and its cells do not change.
- `d10/internal_ud-2-20-1.json`, `d10/leaf-2-20-0.json`, `d10/internal_ud-2-16-0.json`, `d10/internal_ud-1-20-0.json`: raw output per run.
- `D10-HEIGHTS.json`: the primary and old-set runs, the input of `d10_wrap.py`.
- `d10/stack_check.py`: an independent Python copy of the `StackedLayout` rule, and the log add-on sensitivity.
- `d10/d10_d3.rs`: the D3 calculator test at the measured `w_stack`.
- `d10_wrap.py`: the wrap rows with measured heights (imports `wrap_model.py`, unchanged).
- `d10/build.sh`, `d10/runs.sh`, `d10/d3.sh`: the run scripts.

## 1. Method

The chain is the bench chain of `L-STRUCTURE.md` section 1, now with proofs:

1. Two W1 proofs (Poseidon2 AIR, 2^20 rows, app params), proved and verified on the CPU engine.
2. The node `VerifierSubCircuit::<2>` over the W1 vk, proved and verified. Primary child set: the OD-19 internal set (`WhirProximityStrategy::UniqueDecoding`, log blowup 3, queries 97/82/81, folding PoW 0, `mu` PoW 9, query PoW 21, LogUp PoW 18, `l_skip` 2, `n_stack` 17, `w_stack` 512; `research/sherlock-design-paper-review-20261111/UD-SIZING.md` section 5.1, main working copy). Old child set: stock `leaf_params_with_100_bits_security()` (UD, log blowup 2, queries 118/84/81, `l_skip` 4, `n_stack` 17), which is the bench node of `L-STRUCTURE.md`.
3. `L` = `VerifierSubCircuit::<1>` over the node vk. Tracegen is `generate_proving_ctxs_base` over the one node proof, for `has_cached` true (`L_cached`, `CachedTraceCtx::PcsData`) and false (`L_nocached`, `CachedTraceCtx::Records`, as `RootProver`). Each variant runs twice: with the BabyBear engine at stock root params, and with `BabyBearBn254Poseidon2CpuEngine` at the OD-19 `L` set (UD, log blowup 3, queries 98/83/82, folding PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18, `l_skip` 2, `n_stack` 18, `w_stack` 48).
4. In the primary run the BN254 engine also proves and verifies `L`, and the program checks that the `log_height` of every AIR in the proof's `trace_vdata` equals the tracegen height.

Derivations in the program follow the verifier:

- `w_stack`: `StackedLayout::new(l_skip, n_stack + l_skip, ...)` over the common-main widths sorted by descending height, plus one layout per cached commit, summed (`SB/verifier/proof_shape.rs:370-376,540-573`). `d10/stack_check.py` reimplements the rule and gives the same widths.
- `T`: the lifted interaction count `sum num_interactions x 2^max(log_height, l_skip)`, then `n_logup = calculate_n_logup(l_skip, total)` and `T = l_skip + n_logup` (`SB/verifier/proof_shape.rs:378-389`; `SB/lib.rs:83-94`).

Commands (each run through `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run`, inside the `systemd-run --user` units `d10-build`, `d10-runs` and `d10-d3`, logs in `~/.cache/d10-heights/`):

- Build: `cargo build --release --locked --message-format short -p stark-bench-ovm --features bn254 --example l_heights` with `CARGO_TARGET_DIR=~/.cache/stark-bench/target/ovm`, `RUSTFLAGS=-C target-cpu=native` (`d10/build.sh`).
- Runs: `RAYON_NUM_THREADS=20 l_heights <child_set> <arity> <w1_log_rows> <prove_l>`, for `internal_ud 2 20 1` (primary), `leaf 2 20 0` (old set), `internal_ud 2 16 0` and `internal_ud 1 20 0` (sensitivity) (`d10/runs.sh`).

Anchor: the node has 37,512,232 main cells, which is the 37,463,080 common-main cells of `ovm-gpu-node2-20.json` plus its 12 cached columns at 2^12 (49,152 cells), MEASURED. The node shape is the same for both child sets, because the node's heights depend on its W1 children and not on its own parameters.

## 2. Per-AIR heights of L

MEASURED, run `internal_ud 2 20 1`, variant `L_cached`, BN254 engine. The heights are the same in all four engine and variant combinations of a run (checked by the program output). `L_nocached` has the same heights; only SymbolicExpressionAir changes width (316 common columns, no cached part). The last column is the old child set (run `leaf 2 20 0`).

| # | AIR | Height, UD child | Columns (cached) | Cells, UD child | Interactions | Height, leaf child |
|---:|---|---:|---:|---:|---:|---:|
| 0 | SymbolicExpressionAir | 2^15 | 12 (+12); 316 without cache | 786,432 | 13 | 2^15 |
| 1 | FractionsFolderAir | 2^6 | 29 | 1,856 | 17 | 2^6 |
| 2 | UnivariateSumcheckAir | 2^4 | 24 | 384 | 14 | 2^7 |
| 3 | MultilinearSumcheckAir | 2^7 | 33 | 4,224 | 14 | 2^7 |
| 4 | EqNsAir | 2^5 | 41 | 1,312 | 10 | 2^5 |
| 5 | Eq3bAir | 2^14 | 25 | 409,600 | 3 | 2^14 |
| 6 | EqSharpUniAir | 2^2 | 17 | 68 | 5 | 2^4 |
| 7 | EqSharpUniReceiverAir | 2^2 | 17 | 68 | 3 | 2^4 |
| 8 | EqUniAir | 2^2 | 16 | 64 | 3 | 2^3 |
| 9 | ExpressionClaimAir | 2^7 | 32 | 4,096 | 7 | 2^7 |
| 10 | InteractionsFoldingAir | 2^12 | 37 | 151,552 | 13 | 2^12 |
| 11 | ConstraintsFoldingAir | 2^12 | 25 | 102,400 | 10 | 2^12 |
| 12 | EqNegAir | 2^3 | 40 | 320 | 8 | 2^4 |
| 13 | TranscriptAir | 2^12 | 44 | 180,224 | 17 | 2^12 |
| 14 | Poseidon2Air | 2^15 | 301 | 9,863,168 | 2 | 2^15 |
| 15 | MerkleVerifyAir | 2^14 | 38 | 622,592 | 6 | 2^14 |
| 16 | ProofShapeAir | 2^6 | 45 | 2,880 | 77 | 2^6 |
| 17 | PublicValuesAir | 2^0 | 8 | 8 | 3 | 2^0 |
| 18 | RangeCheckerAir | 2^8 | 2 | 512 | 1 | 2^8 |
| 19 | GkrInputAir | 2^0 | 26 | 26 | 19 | 2^0 |
| 20 | GkrLayerAir | 2^5 | 46 | 1,472 | 30 | 2^5 |
| 21 | GkrLayerSumcheckAir | 2^8 | 45 | 11,520 | 21 | 2^8 |
| 22 | GkrXiSamplerAir | 2^0 | 10 | 10 | 7 | 2^0 |
| 23 | OpeningClaimsAir | 2^11 | 63 | 129,024 | 22 | 2^11 |
| 24 | UnivariateRoundAir | 2^3 | 27 | 216 | 13 | 2^5 |
| 25 | SumcheckRoundsAir | 2^5 | 57 | 1,824 | 21 | 2^5 |
| 26 | StackingClaimsAir | 2^9 | 35 | 17,920 | 17 | 2^11 |
| 27 | EqBaseAir | 2^2 | 51 | 204 | 8 | 2^3 |
| 28 | EqBitsAir | 2^11 | 16 | 32,768 | 5 | 2^12 |
| 29 | WhirRoundAir | 2^2 | 46 | 184 | 31 | 2^2 |
| 30 | SumcheckAir | 2^4 | 38 | 608 | 16 | 2^4 |
| 31 | WhirQueryAir | 2^9 | 32 | 16,384 | 5 | 2^9 |
| 32 | InitialOpenedValuesAir | 2^14 | 89 | 1,458,176 | 13 | 2^13 |
| 33 | NonInitialOpenedValuesAir | 2^12 | 28 | 114,688 | 4 | 2^12 |
| 34 | WhirFoldingAir | 2^12 | 31 | 126,976 | 4 | 2^13 |
| 35 | FinalPolyMleEvalAir | 2^8 | 34 | 8,704 | 13 | 2^10 |
| 36 | FinalPolyQueryEvalAir | 2^16 | 45 | 2,949,120 | 5 | 2^18 |
| 37 | PowerCheckerAir | 2^5 | 4 | 128 | 2 | 2^5 |
| 38 | ExpBitsLenAir | 2^14 | 16 | 262,144 | 2 | 2^14 |

Totals, MEASURED:

| Quantity | UD child, `L_cached` | UD child, `L_nocached` | Leaf child, `L_cached` | Leaf child, `L_nocached` |
|---|---:|---:|---:|---:|
| Main cells (common + cached) | 17,263,856 | 26,832,112 | 25,626,076 | 35,194,332 |
| Maximum height | 2^16 | 2^16 | 2^18 | 2^18 |
| Lifted interactions | 1,496,228 | 1,496,228 | 2,437,624 | 2,437,624 |
| `n_logup` | 19 | 19 | 20 | 20 |
| `T` | 21 | 21 | 22 | 22 |
| Distinct heights | 14 | 14 | 16 | 16 |
| Stacked width, common + cached | 17 + 1 | 26 + 0 | 25 + 1 | 34 + 0 |
| `w_stack` needed | **18** | 26 | 26 | 34 |

The UD child shrinks `L`: its `l_skip` 2 (against 4 at leaf params) and its smaller final polynomial cut FinalPolyQueryEvalAir from 2^18 to 2^16, StackingClaimsAir from 2^11 to 2^9 and FinalPolyMleEvalAir from 2^10 to 2^8. InitialOpenedValuesAir rises from 2^13 to 2^14, and WhirFoldingAir and EqBitsAir halve. Net, `L` has 8.36 M fewer cells, DERIVED. The driver is the child's parameter set, not its size: two UD child nodes with stacked widths 73 and 37 (arity 2 and 1, MEASURED) give the same heights of `L` (section 5).

## 3. GKR depth T

`T` = 21 for the primary row (`n_logup` 19, lifted interaction count 1,496,228 below 2^21), MEASURED. The `L` proof made at the UD set has 21 GKR layers, which the verifier checks against the same formula. The old child gives `T` = 22. The scenario rule of `wrap_model.py` gave 23 (ESTIMATE), and the leaf-node proxy of `L-STRUCTURE.md` gave 22.

## 4. Distinct heights

14 distinct heights for the primary row: 2^0, 2^2 to 2^9, 2^11, 2^12 and 2^14 to 2^16 (histogram in the JSON), MEASURED. The scenario value was 17. The old child gives 16.

## 5. w_stack

`L_cached` needs `w_stack` **18**, MEASURED: 17 stacked columns of 2^20 cells for 16,870,640 common-main cells, plus 1 column for the 12 cached columns at 2^15. This is the stock root default (18) and less than half the OD-3 bound of 48. `L_nocached` needs 26.

The proof check confirms it: with the BN254 engine at the UD set and `w_stack` 48, both `L` variants prove and verify (`L_cached` 119.7 s, `L_nocached` 103.8 s on the CPU, load average above 30 during the run), MEASURED. A key with a smaller `w_stack` bound only lowers the bound the verifier checks the measured width against.

Sensitivity: the heights of `L` do not move with the child proof at the UD set. Runs `internal_ud 2 16 0` (node over two W1 proofs at 2^16 rows, node 35,334,472 cells) and `internal_ud 1 20 0` (node `VerifierSubCircuit::<1>` over one W1 proof, the path C shape, node 18,781,016 cells and maximum height 2^17) give the same height for all 39 AIRs of `L` as the primary run, in both variants, so the same `w_stack` 18 and 26, `T` 21 and 14 distinct heights, MEASURED. The heights of `L` follow the child's parameter set (section 2), not its workload. This is four child proofs, not a proof that every child gives these heights; the `L` key fixes them (WRAP.md section 1), and the RootProver `required_heights` path pads a smaller trace up to them.

## 6. Keygen on the BN254 root configuration

`L` keyed with `BabyBearBn254Poseidon2CpuEngine` at the UD `L` set, MEASURED (dumps `L_cached_bn254_ud_keygen` and `L_nocached_bn254_ud_keygen` in each run JSON):

- Over the leaf-parameter child (the `L-STRUCTURE.md` chain), the BN254 key has the same structure as the BabyBear key of `L-STRUCTURE.json`: 39 AIRs in the same order, and per AIR the same widths, constraints, DAG nodes, interactions and message lengths (0 differences). The BN254 config changes the hash of `L`, not its AIRs.
- Over the UD internal child, `L_cached` has 39 AIRs, 1,537 columns (1,525 common and 12 cached), 484 interactions, 2,210 constraints, 15,312 DAG nodes and maximum constraint degree 4. `L_nocached` has 1,829 columns, 2,522 constraints, 18,069 DAG nodes and 8 public values. Against the leaf child, SumcheckAir has 16 interactions instead of 19 (76 message fields instead of 88), and 10 AIRs have 1 to 7 fewer DAG nodes. The child's `l_skip` (2 against 4) drives these, DERIVED from the AIRs that change.
- Key parameters as printed: `l_skip` 2, `n_stack` 18, `w_stack` 48, log blowup 3, `k_whir` 4, UD, queries 98/83/82, folding PoW 0, `mu` PoW 6, query PoW 20, LogUp PoW 18.

## 7. Log add-on AIRs

The log add-on is not built. The list below follows PROPOSAL section 3.2 at the recommended capacity of OD-6 (`B` = `V` = `P` = 256, `E` = 0). Everything here is ESTIMATE unless marked.

| AIR | Rows | Columns (ESTIMATE) | Job |
|---|---:|---:|---|
| Log entry table | `B` = 256 (2^8) | about 20: `op`, `param`, the value and permutation running offsets, 8 digest words of the onion step, selectors | the `B` x 2 entry table of `pv`; running sums `off_(i+1) = off_i + vals(op_i, param_i)` and the `perms` analogue; the onion `d_(i+1) = P2(tagStep, d_i, P2(tagEnt, enc(e_i)))` by lookups on the Poseidon2 bus |
| Log value table | `V` = 256 (2^8) | about 49: 9 limbs of 30 bits, range and `Canon` (`dec(x) < r`) helper columns, the owner entry index | the `V` x 9 value table of `pv`; feeds `enc(e_i)` of its entry |

The binding of both tables to `pv` by `T(x) = sum_i eq(x, i) pv_i` adds public values (`W_pv` = 2B + 9V = 2,816 at 256, DERIVED in PROPOSAL 3.2) and no stacked column. The 69-column total is the old residual of the wrap model's mid scenario (1,600 columns minus the 1,531 of the node), not a designed width, DERIVED from `wrap_model.py` `SCEN`.

Effect on `w_stack`, DERIVED by `d10/stack_check.py` on the measured heights:

- Both AIRs at 2^8 or at 2^10 (the old capacity 1,024), 69 columns: `w_stack` stays 18 (`L_cached`) and 26 (`L_nocached`). Their 17,664 to 70,656 cells fit in the last partly filled stacked column.
- The onion digest adds about 2 x 256 Poseidon2 compressions plus the `enc(e)` hashing of up to 2,304 value words, about 800 to 1,100 permutations (ESTIMATE). They go to Poseidon2Air of `L` (2^15 rows, 301 columns). If they push it to 2^16, `w_stack` becomes 27 (`L_cached`) and 36 (`L_nocached`). The run does not read how many of the 2^15 rows are padding, so this case stays open; both values are below 48.

## 8. Effect on D3 and on the wrap rows

### 8.1 D3 at the measured w_stack

Calculator: the D3 patch of `d3-soundness` (`d3/calculator-d3.patch`, commits 234954947 and 81abb8f7a) applied to a copy of stark-backend v2.1.0 at `~/.cache/d10-heights/sb` (the patched `soundness/` tree is byte-identical to the UD sizing copy). Test `d10/d10_d3.rs`: `D10_WS=18,26,48 cargo test --release --locked -p openvm-stark-backend --test d10_d3 -- --nocapture --test-threads=1`, log `~/.cache/d10-heights/d3.log`. The shape bound of PROPOSAL 11.2 (height 2^20, 50 AIRs, 100 interactions per AIR) sets `n_logup` to 29 by the calculator's own bound rule. A second pass sets `n_logup` to the measured 19. MEASURED.

| `L` set | `w_stack` | Weakest term (bits) | `mu` term | Gates (grindable at least 101, every term at least 100) |
|---|---:|---|---:|---|
| **UD b3 (OD-19)** | **18 (measured, `L_cached`)** | **101.1427 (query and `shift_rbr`)** | 102.5401 | PASS / PASS |
| UD b3 | 26 (measured, `L_nocached`) | 101.1427 (query and `shift_rbr`) | 101.9837 | PASS / PASS |
| UD b3 | 48 (bound) | 101.0730 (`mu`) | 101.0730 | PASS / PASS |
| LD PROPOSAL 3.4 (old) | 18 | 101.1727 (folding) | 103.0847 | PASS / PASS |
| LD old | 26 (measured, leaf child, `L_cached`) | 101.1727 (folding) | 102.5283 | PASS / PASS |
| LD old | 48 (bound) | 101.1727 (folding) | 101.6176 | PASS / PASS |

The other UD terms at every `w_stack`: LogUp 102.7205, GKR sumcheck 122.0426, GKR batching 123.6276, zerocheck 119.7207, constraint batching 113.6546, stacked reduction 111.6618, folding 101.6276, `gamma` 116.9982, OOD 108.6276. The LD 48 row equals D3 section 8 (101.6176 `mu`, 101.1727 folding), which checks the copy.

Results:

- Only the `mu` term moves with `w_stack`, as D3 section 8 found. At the measured 18 the weakest UD term is the query term 101.1427, not `mu`; `mu` has 1.54 bits of margin over 101.
- The second pass with `n_logup` 19 (measured) in place of the bound 29 prints the same value for every term in all six rows, to 4 decimals. The height-dependent `n_logup` does not move D3 at these sets.
- The UD `mu` PoW of 6 is sized for `w_stack` 48. At a key bound of 18, `mu` PoW 5 would give 101.54 bits and still pass, DERIVED from the 1 bit per PoW bit slope; this saves one grind bit and is not proposed here.

### 8.2 G4 wrap rows at the measured heights

Command: `python3 -I d10_wrap.py D10-HEIGHTS.json` (output `~/.cache/d10-heights/wrap.out`). It runs `wrap_model.py` with the measured structure of `L-STRUCTURE.json` (the `L_cached` or `L_nocached` variant that matches the row) and replaces `T`, the distinct-height count, the area and `w_stack` by the measured values. The PCS constants are patched as `~/.cache/ud-sizing/wrap_ud.py` does. The structure of `L` over the UD child differs from `L-STRUCTURE.json` by 3 interactions and 19 DAG nodes (section 6), below 0.7 percent of those inputs; the model keeps the `L-STRUCTURE.json` structure. The log add-on stays the model's labelled ESTIMATE (+2 AIRs, +69 columns). The rows are ESTIMATE (the model), with MEASURED inputs; the smallest k is DERIVED from them. The scenario row reproduces `UD-SIZING.md` section 3.2 (562,558 / 639,486 / 672,117).

Gates (85 percent of `2^k - 7`): k = 19 445,639; k = 20 891,284; k = 21 1,782,573.

| `L` set and heights | `w_stack` | G4 low / mid / high | Smallest k (G4) | Worst low / mid / high | Smallest k (worst) |
|---|---:|---:|---:|---:|---:|
| UD b3, scenario (`T` 23, 17 heights) | 16 / 18 / 18 | 562,558 / 639,486 / 672,117 | 20 | 1,259,468 / 1,364,717 / 1,409,408 | 21 |
| **UD b3, measured, `L_cached`** | **18** | 563,879 / **634,707** / 666,513 | **20** | 1,257,851 / 1,352,330 / 1,394,821 | 21 |
| UD b3, measured, `L_nocached` (`L_nocached` structure) | 26 | 599,439 / 648,150 / 679,962 | 20 | 1,344,766 / 1,409,756 / 1,452,255 | 21 |
| UD b3, measured, `L_cached`, at the bound | 48 | 661,302 / 723,984 / 755,789 | 20 | 1,427,298 / 1,510,916 / 1,553,407 | 21 |
| LD PROPOSAL 3.4 (old), scenario | 16 / 18 / 18 | 383,760 / 422,490 / 440,569 | 19 | 861,615 / 914,706 / 939,995 | 21 |
| LD old, measured leaf child, `L_cached` | 26 | 398,037 / 431,043 / 448,298 | 20 | 885,214 / 929,264 / 952,353 | 21 |
| LD old, measured leaf child, `L_nocached` (`L_nocached` structure) | 34 | 430,910 / 457,118 / 474,378 | 20 | 963,639 / 998,625 / 1,021,722 | 21 |
| LD old, measured leaf child, `L_cached`, at the bound | 48 | 436,981 / 468,134 / 485,389 | 20 | 952,649 / 994,229 / 1,017,318 | 21 |

Results:

- **k = 20 holds** for the UD `L` set at the measured `w_stack` 18: G4 high 666,513 rows is 74.8 percent of the k = 20 gate, DERIVED. It also holds for `L_nocached` at 26 (679,962 high) and at the bound 48 (755,789 high, 84.8 percent).
- The measured heights lower `T` from 23 to 21 and the distinct heights from 17 to 14, which saves 4,779 rows mid at `w_stack` 18 against the scenario row, DERIVED.
- Holding the key at the bound 48 costs 89,277 rows mid against the measured 18, DERIVED.
- The worst gadget row needs k = 21 in every case, as before. It never needs k = 22.
- In the old LD set, the measured leaf-child heights (`w_stack` 26) move G4 high above the k = 19 gate (448,298 against 445,639). k = 19 was already out under UD (`UD-SIZING.md` 3.1).

### 8.3 OD-3

`w_stack` stays at or below 48: the measured value is 18 for `L_cached` and 26 for `L_nocached`, and 27 or 36 in the open Poseidon2Air case of section 7. A tighter key bound is possible; the D3 rows above show what it buys (`mu` term only) and section 8.2 what it saves in the wrap. The `L` key keeps a fixed height per AIR (WRAP.md section 1), so the bound that freezes is the stacked width of those fixed heights plus the log add-on.
