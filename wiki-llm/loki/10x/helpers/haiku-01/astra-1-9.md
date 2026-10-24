# astra-1-9: exact benchmark audit, Snarkify zprize 1b, Snarkify cuSnark zkEVM tables, ICICLE 25x claim

## Question
Audit the two Snarkify benchmark sources (zprize-2023-prize1b GitHub README/harness, and the cuSnark zkEVM Halo2 GPU prover docs tables) for timing scope, hardware, curve, units and internal consistency. Locate the ICICLE "25x" blog that astra-1 referred to.

## Answer
- **Snarkify zprize-2023-prize1b (BLS12-381, Poseidon-Merkle PLONK/KZG): 2.16 s for a batch of 4 proofs = ~540 ms/proof on "A6000 Ada"; CPU baseline "around 490 seconds per proof"; README claims ~900x.** Source: README.md in github.com/snarkify/zprize-2023-prize1b (commit d26de99, 2024-10-07; local clone at helpers/haiku-01/zprize-src). The 490 s baseline is a README statement; the repo does not contain a CPU run to reproduce it.
- **Timed region is exactly the prove loop, nothing else** (submission/benches/zprize_bench.rs:60-88): `gen_proof_accel` x4 (witness generation on device, per the code comment at :66-67). Excluded: KZG setup (:35), compile and `MerkleTreeCircuit::init` (:40-45), trace/tree construction (:48-58), verification (:96+). Keygen is excluded. Host<->device transfers are not separately reported and sit inside `gen_proof_accel` if present. Lookups are excluded per README phase 1 ("eliminate unused ... lookups").
- **Tree height mismatch:** the harness uses `HEIGHT = 15` (merkle-tree/src/lib.rs:20), i.e. circuit size `1 << (HEIGHT+9)` = 2^24 KZG powers (zprize_bench.rs:34). The task spec in baseline/gpu/README.md says "Merkle tree of height 18". So the 2.16 s figure is for the harness's height-15 tree unless the harness is changed; I found no run at height 18.
- **Speedup arithmetic is consistent:** 2.16/4 = 0.54 s; 490/0.54 = 907x (README rounds to 900x).
- **Micro-benchmarks (GPU-Micro-Benchmarks/, cudaEvent timing, MicroBenchmarks.cu:118-140):** the Times-A6000.txt output matches the README table. msm-22 avg 28.395 ms (min 25.955, max 30.710). ntt-22 1.129 ms; ntt-25 13.348 ms; intt-22 1.231 ms; coset-ntt-25 14.452 ms; polynomial-evaluate-22 0.161 ms; polynomial-evaluate-25 1.179 ms; polynomial-divide-22 0.689 ms; batch-invert-22 0.774 ms. Units are milliseconds (README: "All times in milliseconds"). The timed call is `run_once(op)` between events, so it measures device-side op time as wrapped by `run_once`; I did not trace what `run_once` does for MSM (point copy or not).
- **Hardware label is inconsistent.** README headline says "A6000 Ada card". Times-A6000.txt says "A6000". The micro-benchmark build line is `nvcc -arch=sm_80` (README.md:49), which is the A100 (Ampere GA100) target. From general NVIDIA product knowledge (not captured, treat as unverified): the RTX A6000 is GA102 (sm_86) and the RTX 6000 Ada is AD102 (sm_89). The sm_80 flag would not match either. Which card produced the numbers is therefore not established.
- **cuSnark zkEVM Halo2 GPU prover (docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover):** the 217.76 -> 30.24 and 1762.75 -> 463.13 numbers are real and on the live page.
  - Proof 1, aggregation, 2^25 rows, 5 columns: CPU 217.76 s, GPU 30.24 s, speedup 7.20x.
  - Proof 2, chunk_inner, 2^20 rows, 1135 columns: CPU 1762.75 s, GPU 463.13 s, speedup 3.81x.
  - Host: "AMD EPYC 7702 64-Core Processor with 4x NVIDIA GeForce RTX 3090 (24 GB)", stated for both proofs.
  - Units: seconds (column headers "CPU/s", "GPU/s"). Per-stage speedup column is present.
  - **Curve is not stated** on the page. This is a Halo2 zkEVM (Scroll-style) setup, typically BN254, but that is my inference and is not in the source. Do not treat as BLS12-381.
- **Internal consistency of the cuSnark tables:**
  - Proof 2 (chunk_inner): CPU stage rows sum to 1762.75 (matches total). GPU rows sum to 463.14 (matches 463.13).
  - Proof 1 (aggregation): GPU rows sum to 30.23 (matches 30.24). **CPU rows sum to 168.57 s against the stated 217.76 s total, a gap of 49.19 s (22.6%).** The CPU e2e % column sums to 77.4%, not 100%. The gap is in the source page (the live page has the same rows), so it is not a capture error. Per-row speedups are internally consistent (e.g. Eval_h 67.40/6.30 = 10.70). The 7.20x headline uses the total, so it is unaffected in the arithmetic but cannot be decomposed without the missing ~49 s of CPU time. I could not identify the missing stage.
- **Snarkify's "1.25x" claim** (lu.ma/fz3x308c talk listing, search snippet only) is a different claim from the 7.20x / 3.81x tables. Not reconciled; the talk page gives no baseline.
- **ICICLE "25x" blog: FOUND and verified (earlier "not found" was a search failure).** Source: https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2 (line 19 of astra-1/raw/icicle-halo2-v2.md: "up to 25x performance boost"). The 25x is a lookup-circuit peak (about 26-27x at 2^16+ vs ezkl-halo2); plonk and simple-lookup are about 6-8x. Hardware: RTX 4080 vs i9-13900K. Curve not stated. See the ICICLE addendum below.
- **Nothing was run.** Only source reading and the published numbers. The 490 s CPU baseline, the 2.16 s GPU batch, and the cuSnark numbers are all self-reported. No independent reproduction.
- **Curves:** zprize harness is BLS12-381 (`KZG10::<Bls12_381>`, zprize_bench.rs:35). cuSnark curve not stated.
- **Scope of the zprize 2.16 s:** excludes setup and key compile, includes device witness generation (per code comment), transfer not separately reported.
- **Tree height:** 15 in code vs 18 in spec. This changes circuit size; the spec-height number is not in the repo.
- **ICICLE curve:** not stated in text; no BLS12-381 evidence.
- **Micro-bench runs:** `run_once` is not traced, so MSM timing scope (point transfer or not) is not established.
- **Local clones (untrusted, read-only, not built):** helpers/haiku-01/zprize-src (snarkify/zprize-2023-prize1b @ d26de99). Captures: helpers/haiku-01/raw/haiku01-snarkify-cusnark-live.md. Peer captures read (not modified): astra-1/raw/snarkify-zprize.md, astra-1/raw/snarkify-cusnark.md, astra-1/raw/icicle-halo2-v2.md.

## ICICLE-Halo2 v2 "25x" (addendum, verified)
- Source: https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2 (July 28, 2025). Text claim: "our own fork of ezkl-halo2 with up to 25x performance boost". Hardware in the figure caption: NVIDIA RTX 4080 (GPU) and CPU i9-13900K. Curve and the proof-size/k range are not stated in the text.
- The numbers are in six chart screenshots (not text). Downloaded to helpers/haiku-01/raw/icicle-shots/shot1..7.png; values below are read off the charts, so approximate.
  - Lookups circuit, speed-up vs ezkl-halo2 (shot3): icicle-halo2 v2 about 12.5x at 2^11, rising to about 26-27x at 2^16-2^19. This is the source of the "up to 25x" headline.
  - Plonk circuit, speed-up vs ezkl-halo2 (shot5): v2 about 5.6-8x across 2^11-2^20 (peak about 8x at 2^16).
  - Simple-lookup circuit, speed-up vs ezkl-halo2 (shot6): v2 about 5.6-8x (same shape as the plonk chart).
  - Proving times (shot2, shot4, shot7, ms, log scale): ezkl-halo2 about 50 s at 2^19 (lookups) and about 55 s at 2^20 (plonk); v2 about 2.5 s at 2^19 (lookups) and about 7 s at 2^20 (plonk). Readings are approximate.
- Verdict: the 25x is real as a chart claim but only for the lookup-heavy circuit at 2^16+, on RTX 4080 vs ezkl-halo2 CPU-side baseline. Plonk and simple-lookup are about 6-8x. Not BLS12-381 evidence (curve not stated; ezkl-halo2 is conventionally BN254, which is my inference only).
- Corrects my earlier "not found" entry: the source is now identified and the 25x is a lookup-circuit peak.
