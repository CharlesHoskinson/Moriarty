# astra-1-5: challenge the NTT speedup budget

## Question
1. Does a 10-20x GPU NTT speedup hold for BLS12-381? Give field-specific CPU vs GPU dimensions.
2. Find exact BLS12-381 NTT absolute microbenchmarks at 2^20/2^22/2^23 (including snarkify/zprize-2023-prize1b).
3. Challenge the 20x bulk-acceleration assumption with an integer-ALU roofline (not FP32 TFLOPS). Source RTX 5090 integer throughput.
4. Correct the illustrative 64-column memory model with the historical node4 inventory, using peak liveness rather than a sum.

## Answer
- **(1) Speedup, partly supported.** ZKProphet Table II (A40, fastest library per row) reports NTT speedup over CPU of 35.4x at 2^20 (cuZK), 50.6x at 2^22 (cuZK), and 50.3x at 2^23 (cuZK). The table does not name the curve per row; the paper says it covers BLS12-377 and BLS12-381. The CPU baseline is arkworks on dual EPYC 7742; thread count is not in the captured text. So "10-20x" is met on the paper's workload, but against an unspecified baseline, not against blst or Midnight's own path. No absolute CPU NTT time for BLS12-381 is captured, so no absolute speedup can be derived. Confidence: medium.
- **(2) Absolute BLS12-381 NTT numbers, sparse.**
  - Snarkify GPU micro-benchmarks (self-reported, GPU not stated for this table, built with `nvcc -arch=sm_80`): BLS12381 NTT 2^22 = 1.13 ms, INTT 2^22 = 1.23 ms, MSM 2^22 = 28.40 ms, NTT 2^25 = 13.35 ms, coset-NTT 2^25 = 14.45 ms. Main README says the proof was run on an A6000 Ada, but the micro-benchmark table does not name a device. Mark as VENDOR, unverified device and field size.
  - No primary BLS12-381 NTT absolute number at 2^20 or 2^23 was found. ZKProphet gives only speedup ratios. cuZK's V100 results are MSM and Groth16 proof times, not NTT times.
- **(3) Integer roofline (RTX 5090, compute capability 12.0).**
  - Spec: 21,760 CUDA cores, boost 2.41 GHz (NVIDIA page). SM count 170 is DERIVED from 21,760 / 128 FP32 lanes per SM; the spec page does not state it.
  - CUDA Best Practices throughput table (results per clock per SM, columns include 12.0/12.1): 32-bit integer add `add.s32` = 128; `mad.lo.s32` = 64; `shl.b32` = 64; `xor.b32` = 64. Extended-precision mad = 32 (footnote 6). Cells were resolved from the raw HTML colspans, not the markdown.
  - DERIVED peak: IMAD-class 170 x 64 x 2.41 GHz ≈ 26.2 Tops/s; add-class 170 x 128 x 2.41 GHz ≈ 52.4 Tops/s.
  - A 2^22 NTT has 2^21 x 22 = 46,137,344 butterflies. ESTIMATE of integer ops per butterfly: 128 IMAD-class plus 128 ALU (optimistic) to 256 plus 256 (CIOS-style 255-bit Montgomery on 8 x 32-bit limbs, rough). Ideal lower bound on the 5090: 0.23 ms to 0.45 ms per 2^22 NTT, limited by IMAD. Snarkify's 1.13 ms is about 2.5x to 5x above this roof, which is plausible for a non-5090 device and unverified. Conclusion: a roofline does not forbid 20x over CPU, but the GPU-side headroom is large. The real question is the CPU baseline and the existing GPU kernel utilization, which are not measured.
  - ZKProphet's claim that per-SM 32-bit integer throughput has been constant across generations (paper line 133) supports scaling by SM count, not per-SM improvement.
- **(4) Memory correction, historical node4.** keygen.stats.json (recl9/keys/node4) records k=20, lookups=5, permutations=18, advice_columns=15, fixed_columns=45 (includes 18 permutation columns, so 27 fixed), rows=958,095, table_rows=1,032,465, usable_rows_at_k=1,048,569. Instances=2 and "six product and lookup triples" are taken from the brief and are NOT verified in this file (5 lookups plus 1 permutation grouping is my guess at the 6).
  - All columns extended at once: 27+18+15+2+(6x3)+1 = 81 columns. That is 10.125 GiB at N=2^22 and 20.25 GiB at N=2^23 (128 and 256 MiB per column).
  - Peak liveness (fixed+sigma cached, advice and instances resident, 2 triples and accumulator streamed): 8.625 GiB at N=2^22 and 17.25 GiB at N=2^23. Adding ~6 GiB for MSM and runtime reserves: 14.6 GiB and 23.3 GiB.
  - Compared with loki-03's 64-column illustration (8 GiB at N=2^22), the historical inventory has 17 more columns when all are live. Both N=2^22 and N=2^23 still fit a 32 GB card under the streamed-liveness model. The all-live model at N=2^23 with reserves is 26.3 GiB, still under 32 GiB nominal but thin if 32 GB is decimal (29.8 GiB).
  - N for node4 is not stated in these files. The 2^22 and 2^23 values assume N is the extended domain size; the quotient degree was not checked.

## Raw numbers table
| Item | Value | Label | Source |
|---|---|---|---|
| NTT speedup over CPU, 2^20 | 35.4x (cuZK, A40) | SOURCE, curve unspecified | ZKProphet Table II (`astra-1/raw/zkprophet-full.md`) |
| NTT speedup over CPU, 2^22 | 50.6x (cuZK, A40) | SOURCE | same |
| NTT speedup over CPU, 2^23 | 50.3x (cuZK, A40) | SOURCE | same |
| MSM speedup over CPU, 2^20 | 176.1x (sppark, A40) | SOURCE | same |
| BLS12381 NTT 2^22 | 1.13 ms | VENDOR, device unstated | `helpers/haiku-03/raw/h3b-snarkify-zprize.md` (snarkify/zprize-2023-prize1b) |
| BLS12381 INTT 2^22 | 1.23 ms | VENDOR | same |
| BLS12381 NTT 2^25 | 13.35 ms | VENDOR | same |
| BLS12381 MSM 2^22 | 28.40 ms | VENDOR | same |
| BLS12381 2^20 MSM (cuZK vs bellperson, V100) | 0.19 s vs 0.41 s | SOURCE | `helpers/haiku-03/raw/h3b-cuzk.md` |
| RTX 5090 CUDA cores | 21,760 | SPEC | `helpers/haiku-03/raw/h3-nvidia-5090.md` |
| RTX 5090 boost clock | 2.41 GHz | SPEC | same |
| RTX 5090 SM count | 170 | DERIVED (21,760/128) | derived |
| add.s32 per SM per clock (12.0) | 128 | SOURCE | `helpers/haiku-03/raw/h3b-bp-raw.html` (Table 5) |
| mad.lo.s32 per SM per clock (12.0) | 64 | SOURCE | same |
| IMAD-class peak, 5090 | ~26.2 Tops/s | DERIVED | 170 x 64 x 2.41e9 |
| ALU add peak, 5090 | ~52.4 Tops/s | DERIVED | 170 x 128 x 2.41e9 |
| 2^22 NTT butterflies | 46,137,344 | COMPUTED | 2^21 x 22 |
| 2^22 NTT ideal, 128 IMAD/butterfly | 0.225 ms | ESTIMATE | 46.1M x 128 / 26.2T |
| 2^22 NTT ideal, 256 IMAD/butterfly | 0.450 ms | ESTIMATE | same with 256 |
| Historical node4 all-live columns | 81 | COMPUTED from keygen.stats | fixed 27 + sigma 18 + advice 15 + inst 2 + 18 triple cols + 1 |
| all-live GiB, N=2^22 / 2^23 | 10.125 / 20.25 | COMPUTED | 81 x 128 MiB / 81 x 256 MiB |
| streamed peak GiB, N=2^22 / 2^23 | 8.625 / 17.25 | COMPUTED | liveness model above |

## Caveats
- The NTT speedup ratios are over an unspecified CPU and on an A40. They do not transfer to a 5090 or to Midnight's blst/Fr path without a measured baseline.
- The snarkify 1.13 ms figure is self-reported with no device or field verification. Treat as an unverified upper-bound-level claim, not a benchmark.
- The integer ops-per-butterfly counts are estimates. No Midnight or blst NTT instruction count was measured.
- ICICLE/sppark BLS12-381 NTT microbenchmarks at 2^20/2^22/2^23 were not found in primary sources.
- Historical node4 instances=2 and the 6 triples are not verified in the keygen file. The N used for node4 is not stated.
- Captures are in `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/helpers/haiku-03/raw/` (`h3b-*`). The brief asked for `astra-1/raw/haiku03b-*`, but my write scope is `helpers/haiku-03/`, so I stored them there.

## Addendum: Blackwell (CC 12.0) integer rows, official table (answers astra-1 "Blackwell instruction caveat")
Source: NVIDIA CUDA C Best Practices Guide, Table 5 "Throughput of Native Arithmetic Instructions" (number of results per clock cycle per SM), capture `helpers/haiku-03/raw/h3b-bp-raw.html` (https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html). Columns: 7.5 | 8.0 | 8.6/8.9 | 9.0 | 10.0 | 10.3 | 12.0/12.1. Parsed from the HTML colspans.

| Instruction | 8.6/8.9 | 9.0 | 10.0 | 12.0/12.1 |
|---|---|---|---|---|
| add.s32 (32-bit int add) | 128 | 128 | 128 | 128 |
| mad.lo.s32 (footnote 6: extended-precision variant = 32) | 64 | 64 | 64 | 64 |
| add extended-precision (multi-PTX, add.cc-style) | 64 | 64 | 64 | 64 |
| shl.b32 | 64 | 64 | 64 | 64 |
| xor.b32 | 64 | 64 | 64 | 64 |
| add.s64 | 32 | 32 | 64 | 64 |
| add.f32 | 128 | 128 | 128 | 128 |

- The table has NO rows for `mul.hi`, `mad.wide`, or `add.cc` as separate PTX instructions. The closest rows are mad.lo.s32 and the extended-precision add/mad above. Those do not appear as separate CC 12.0 rows, so they cannot be cited directly.
- For CC 12.0 the table does NOT show a blanket 2x integer gain for the Montgomery-relevant rows (add.s32, mad.lo.s32, extended add, shl, xor are unchanged from 8.6/8.9). The only doublings I see are add.s64 (32 to 64) and min.s32/max-type rows in the 9.0+ columns (64 to 128). Do not project a 2x uplift onto Montgomery kernels.
- The NVIDIA forum thread astra-1 cited was not fetched. It is secondary and was not used for any number here.
- Consequence for the roofline above: the 5090 integer roof uses add 128 and mad.lo 64 per SM per clock, which is what the official table gives. The ideal 2^22 NTT bound (0.23 to 0.45 ms) stands, with no 2x uplift applied.

## Addendum 2: scrutiny of the snarkify NTT microbenchmark (answers astra-1 "Direct NTT number discovered")
Sources (captured): `helpers/haiku-03/raw/h3c-MicroBenchmarks.cu`, `h3c-BLS12381_fr.cu`, `h3c-NTT.cu`, `h3c-times-a6000b.txt` (from https://github.com/snarkify/zprize-2023-prize1b, path `GPU-Micro-Benchmarks/`), and the README capture `loki/raw/04/snarkify-zprize.md`.

- **Field:** the BLS12-381 scalar field Fr. `BLS12381_fr.cu:79` has limbs ending `0x73EDA753`, matching the Fr modulus (0x73eda753299d7d48...). Not the base field Fq.
- **Hardware: ambiguous.** The results file is named `Times-A6000.txt` (no "Ada"). The README says the 540 ms/proof run was on "A6000 Ada". The README build line is `nvcc -arch=sm_80` (Ampere). Do not assume A6000 Ada without a device string.
- **Timing boundary:** `MicroBenchmarks.cu:118-141` wraps each `run_once(op)` in cudaEvent start/stop, 20 repetitions, reporting min/max/average. For `ntt-22`: min 1.114, max 1.155, avg 1.129 ms. No memcpy appears in `NTT.cu`, `host_ntt.cpp` or `MicroBenchmarks.cu` (grep). I did not locate the `ntt_22` host wrapper, so copies inside it are unverified.
- **Batching:** none seen. Each op is one transform of 2^22 (SMALL) or 2^25 (LARGE) elements (`MicroBenchmarks.cu:48-49`).
- **Algorithm:** 3-pass mixed radix 7/7/8 for 2^22 (`NTT.cu:90-129`), shared-memory transposes, `mod_mul` on 8x32-bit limbs. The 2^25 kernel uses 7/9/9.
- **Correctness:** I saw no output comparison against a reference in the microbenchmark. Treat the timings as performance only.
- **Coset/INTT:** `intt-22` avg 1.231 ms (min 1.137, max 1.409). `coset-ntt-25` is a separate op (avg 14.452 ms); the 2^22 coset variant is not measured.
- **Scaling check:** 2^25 NTT 13.348 ms vs 2^22 1.129 ms is 11.8x for 8x the points, while the work grows about 9.1x (8 x 25/22). Per-element cost is about 30% higher at 2^25.
- **Arithmetic for the budget:** 39 forward transforms at 2^22 x 1.13 ms = 44.1 ms. This is a microbench-only sum. It excludes coset scaling, quotient evaluation, transfers and synchronization, so it is not a proof-time budget.
- **Relevance:** real code with a public timing harness, but the device is not pinned, no CPU baseline is given, and there is no correctness check. Usable as a rough reference, not as evidence for a 10x claim on Midnight's path.

| Op | Size | min ms | avg ms | max ms | Source |
|---|---|---|---|---|---|
| ntt-22 | 2^22 | 1.114 | 1.129 | 1.155 | `h3c-times-a6000b.txt` |
| intt-22 | 2^22 | 1.137 | 1.231 | 1.409 | same |
| ntt-25 | 2^25 | 13.125 | 13.348 | 14.265 | same |
| coset-ntt-25 | 2^25 | 14.231 | 14.452 | 14.578 | same |
| msm-22 | 2^22 | 25.955 | 28.395 | 30.710 | same |
| batch-invert-22 | 2^22 | 0.767 | 0.774 | 0.785 | same |

## Correction 3: node4 extended-column inventory (answers astra-1 "Task5 memory correction")
Addendum 1 above is WRONG on the column model. I treated the 18 permutation columns and 18 lookup columns as separate extended columns. Corrected, verified against code:
- Permutation sets: `chunk_len = cs_degree - 2` (`midnight-proofs-0.8.3/src/plonk/permutation/prover.rs:63`), so sets S = ceil(18/(d-2)). Under the assumed d=5, S = 6. The "six" is a permutation set count, NOT six lookup triples.
- Lookups: 5 lookups (keygen.stats.json: `lookups: 5`), each computing product, permuted-input and permuted-table cosets inside its own loop iteration (`midnight-proofs-0.8.3/src/plonk/evaluation.rs:443-460`). That is 3 per lookup = 15 aux columns, streamed one lookup at a time (peak 3).
- Permutation block: all 6 permutation product cosets are held together during the permutation constraints (`evaluation.rs:363`), and the block ends before the lookup loop begins. So peak = permanent + max(6 perm, 3 lookup) = permanent + 6.

Corrected model (d=5 assumed, so N=2^22 = 4n with n=2^20; ASSUMED d, instances and masks, as astra-1 stated):
| Quantity | Columns | N=2^22 (128 MiB/col) | N=2^23 (256 MiB/col) |
|---|---|---|---|
| All-live ext: fixed 27 + sigma 18 + advice 15 + inst 2 + perm 6 + lookup 15 + masks 3 + numerator 1 | 87 | 10.875 GiB | 21.75 GiB |
| Permanent ext (fixed+sigma 45, advice+inst 17, masks 3, numerator 1) | 66 | 8.25 GiB | 16.5 GiB |
| Peak ext (permanent + max(perm 6, lookup 3)) | 72 | 9.0 GiB | 18.0 GiB |
| Fixed ordinary+coeff (45 x 2 x 32 MiB) | - | 2.8125 GiB | 2.8125 GiB |
| Trace (~41 x 32 MiB) | - | 1.28125 GiB | 1.28125 GiB |
| SRS (two affine tables at p=4) | - | 0.75 GiB | 0.75 GiB |
| Peak sum before reserves (bucket, scratch, twiddle, runtime not included) | - | 13.84 GiB | 22.84 GiB |

- All numbers above are arithmetic on assumed inventories (d, instances, masks, trace). They are not measured Sherlock peaks.
- At N=2^23 with d=5 set counts, the peak is 22.8 GiB before reserves. A 32 GiB nominal card leaves about 9 GiB; if 32 GB is decimal (29.8 GiB), about 7 GiB. Whether that holds after bucket, scratch and runtime reserves is not established.
- Addendum 1's 81-column and 10.1/20.3 GiB all-live figures should be read as superseded by the 87-column model above.

## Caveat 4 (from astra-1 direct check, confirmed in my capture)
- `MicroBenchmarks.cu:157` fills every MSM base with the same generator (`msm_points[i*24+j]=generator[j]`). The snarkify `msm-22` 28.4 ms figure therefore does NOT represent a random-SRS MSM. Do not use it as an MSM number. The NTT timings use random inputs and remain usable as warm resident kernel datapoints.
- The harness runs `run_once(op)` as a warmup before the timed loop, and root/base initialization is outside the timed region.
- The microbench header is MIT/Apache, while the full submission is MPL-2.0 (per astra-1's check; my capture did not check the header).

## Caveat 5 (VRAM budget resolved by a device snapshot reported by astra-3)
- astra-3 reports an nvidia-smi snapshot at 22:44:41Z: 32607 MiB total, 28989 MiB free, compute capability 12.0, Gen5 x16, driver 616.92. 32607 MiB = 31.85 GiB usable total. The "29.8 GiB if decimal" worry in the earlier sections is therefore not the binding case. Use ~31.8 GiB as the device total, minus driver and display overhead (~1.1 GiB at idle per the snapshot's total-vs-free gap, which is not a complete accounting). Snapshot is secondhand and not captured by me.
- Under that total, the corrected d=5 peak at N=2^23 (22.8 GiB before reserves) leaves roughly 9 GiB, and N=2^22 (13.8 GiB) leaves roughly 18 GiB, before bucket, scratch and runtime reserves.
