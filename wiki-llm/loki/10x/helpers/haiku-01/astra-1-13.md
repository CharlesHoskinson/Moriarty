# astra-1-13: Pipelonk (PPoPP 2026) primary benchmark audit

## Question
Locate the primary Pipelonk paper/repo. Report curve (BLS12-381?), CPU vs GPU baseline, GPUs actually used, whether witness generation is included, absolute 2^20/2^22 timings, and transfer/memory scope. Precise findings only, no benchmark.

## Answer
- **Primary paper found; full text NOT reachable.** "Pipelonk: Accelerating End-to-End Zero-Knowledge Proof Generation on GPUs for PLONK-Based Protocols", PPoPP 2026 main conference, DOI 10.1145/3774934.3786448 (talk at 14:10, 20 min; PPoPP 2026 program page). The ACM DOI page returned a Cloudflare challenge to Scrapling, so the full text was not captured.
- **Authors** (PPoPP program page author block, truncated in capture): Zhiyuan Zhang, Yanxin Cai, Wenhao Yin, Xu... (Shandong University); Yi Wang (Shenzhen University) is confirmed via the researchr profile. The author list from astra-1's message ("Zhang, Cai, Yin, Ju") is consistent with the first names captured; the rest are not verified.
- **Abstract, verbatim-quoted claims** (raw/haiku01-pipelonk-ppopp.md, Abstract section):
  - "Evaluation shows that Pipelonk runs efficiently on devices with 8 GB to 80 GB memory, achieving an average speedup of 10.7x and up to 19.4x over the state-of-the-art baseline."
  - Contribution 1: a segmentable operator library that offloads all operations, including those not trivially parallelized, to GPU; each operator supports segmented execution.
  - Contribution 2: a pipeline executor that overlaps computation and data transfer, schedules compute- and memory-intensive tasks globally, and picks per-operator segment sizes by modeling memory capacity.
- **Not in the abstract, therefore NOT verified:**
  - Curve: not stated (BLS12-381 is neither confirmed nor refuted).
  - CPU vs GPU baseline: the "state-of-the-art baseline" is not named in the abstract. It may be GPU or CPU; unknown.
  - GPUs used: only the memory range "8 GB to 80 GB" is given; no model is named in the abstract.
  - Witness generation inclusion: not stated.
  - Absolute 2^20 / 2^22 timings: not in the abstract.
  - Transfer and memory scope: described qualitatively (pipeline overlaps transfer; segmentation for memory limits) but no numbers.
- **Code/artifact:** no public repository found (one search; results unrelated, e.g. ZK-Garage/plonk, jellyfish). Not confirmed either way.
- **Related but different work** (do not conflate): ZKPoG (Tsinghua, eprint 2025/765) reports average 22.8x end-to-end vs CPU on RTX 4090 (from search snippet; not captured). The 2023 PolyU ZPRIZE GPU Plonk work is also separate.

## Raw numbers
| Item | Value | Source |
|---|---|---|
| Average speedup | 10.7x | PPoPP program abstract |
| Max speedup | 19.4x | PPoPP program abstract |
| GPU memory range evaluated | 8 GB to 80 GB | PPoPP program abstract |
| DOI | 10.1145/3774934.3786448 | PPoPP program page |
| Curve | not stated | — |
| Baseline | "state-of-the-art" (unnamed) | — |
| Absolute 2^20/2^22 timings | not available | — |

## Caveats
- The full paper PDF and the ACM DOI page were not captured (Cloudflare challenge). All statements about curve, baseline, GPUs, and timing scope remain open until the full text is read.
- The 10.7x/19.4x figures are the paper's self-reported averages and max across an unspecified workload set. Their denominator (which baseline) is unknown, so they cannot be compared with the BLS12-381 numbers already in astra-1-1 or -9 without the full text.
- Captures: raw/haiku01-pipelonk-doi.md (challenge page, unusable), raw/haiku01-pipelonk-ppopp.md (program page with abstract).

## Artifact (addendum, from astra-1 pointer)
- Zenodo record 10.5281/zenodo.18034218 (raw/haiku01-pipelonk-zenodo.md): "zlyber/ppopp26-artifact", MIT, authors Zhiyuan Zhang and Wenhao Yin (Shandong University). Repo: github.com/zlyber/ppopp26-artifact, branch `ae`, cloned read-only to helpers/haiku-01/pipelonk-src at commit bd6df64 (2025-12-23).
- **Scope of the artifact (readme.md TL;DR):** only the segmentable operator library's examples: NTT, grand product, polynomial evaluation, polynomial division. The readme says end-to-end instances and larger inputs (paper sec. 4) are NOT included. So the artifact does not reproduce the 10.7x / 19.4x end-to-end claim, and there is no MSM and no full prover.
- **Curve/field:** BLS12-381 scalar field (`BLS12_381_Fr_G1` in ntt.cu and miss_ops.cu; `PLONK/utils/mont/cuda/ff/bls12-381.hpp`). This is the scalar field only; no pairing or MSM code is exercised.
- **Scales:** NTT lg_N = 22 (ntt.cu:240), i.e. 2^22 x 32 B = 128 MB; miss_ops lg_N = 22 (miss_ops.cu:84). Readme says 2^24 via input-24.bin.zip. (The readme's "2^22 (128 MB) or 2^24 (512 MB)" matches.)
- **Hardware:** readme says tested on NVIDIA H100 and RTX 4090, CUDA 12.8, Ubuntu 24.04. The paper's GPU set is not in the artifact (8–80 GB range in the abstract).
- **CPU baseline:** readme (Remark) says the paper's CPU numbers for the missing operators came from arkworks (ZK-Garage/plonk), but the artifact re-implements the same logic with blst. So the artifact's CPU side is not the paper's baseline.
- **Timing scope (checked in code):**
  - NTT "naive" (ntt.cu:87-113): CUDA events around chunked `cudaMemcpy2D` H2D/D2H, kernels, and a CPU `transpose_matrix` (line 106). Transfers and CPU transpose are INSIDE the timer.
  - NTT "ours" (ntt.cu:182-218): CUDA events around CPU `bit_rev_step1_parallel` (line 183), async H2D/D2H (lines 187-215), CPU `bit_rev_step2_parallel` (lines 197, 203) and GPU kernels. Transfers and CPU bit-reversal are INSIDE the timer.
  - miss_ops "naive_gp": std::chrono wall-clock around the CPU routine (miss_ops.cu:100-109, host only). "ours_gp" (miss_ops.cu:112-129): CUDA events around `accumulate_mul_poly(input)`; `input.mutable_gpu_data()` is called before the event (line ~113), so the H2D copy is outside this timer, but any D2H inside the function would be inside. The naive and ours timers are different clocks (chrono vs CUDA events).
  - Each configuration is run 3 times (readme); output is `execution time: X ms`. No averaging or warmup policy beyond the NTT warm-up at ntt.cu:84-90 and 162-166.
- **Absolute 2^20 / 2^22 timings from the paper:** NOT in the artifact or readme. Not verified. The artifact prints times, but no output logs are committed.
- **Witness generation:** not in the artifact.

## Revised answers
- Curve: BLS12-381 scalar field in the artifact's operators (code-verified); the paper's curve is still unstated in text.
- Witness inclusion: not covered by the artifact.
- Transfer scope: included in the artifact's NTT timers (both naive and ours); mixed clocks for miss_ops.
- Absolute 2^20/2^22 timings: unavailable from paper text or artifact; the artifact can produce 2^22 NTT and miss_ops times once run (not run here: no build).

## Current main vs ae artifact (addendum, follow-up from astra-1)
- Repo: github.com/zlyber/pipelonk (redirect from zlyber/ppopp26-artifact), branch main, HEAD c788a0a8 (2026-07-03, "Tune CPU thread counts for each chunk size"). Cloned read-only to helpers/haiku-01/pipelonk-main-src.
- **File-list diff vs ae (bd6df64):**
  - main adds `lde.cu` and `LDE/CMakeLists.txt` (low-degree extension, lambda = 2 or 3; readme lists `--bench 20/22/24` and `--bench-naive`).
  - main removes `PLONK/hello.cu` and `PLONK/hello.hpp`.
  - `ntt.cu` is rewritten: CLI modes `--bench-split <lg_N>`, `--bench-baseline <lg_N>`, `--bench-baseline-custom`, `--bench-sweep`, `--check-*`; readme lists lg_N = 22, 24, 26 (ae: fixed lg_N = 22).
  - Both versions contain `PLONK/utils/zkp/cuda/` and `PLONK/utils/zkp/cpu/` (MSM via sppark-derived Pippenger in `zksnark_msm.cu`, quotient, gate constraint, permutation, lookup, query-table, padding, NTT). My earlier note that ae lacked these kernels was wrong and is withdrawn.
- **Wiring:** in both versions, only the NTT header (`zksnark_ntt/ntt_kernel/ntt.cuh`) is included by drivers (ntt.cu:5, lde.cu:22). No driver calls the MSM, quotient, gate, permutation, or lookup kernels. So the repo contains the PLONK kernels but no runnable end-to-end prover benchmark. The paper's end-to-end 10.7x is not reproducible from either version.
- **Curve:** BLS12-381 Fr is the default (`PLONK/utils/mont/cuda/curve_def.cuh:7`, `using fr = BLS12_381_Fr_G1`). BLS12-377 files also exist but are not the default. BLS12-381 G1/G2 MSM is not exercised by any driver.
- **Timer scope in main's our-ntt (ntt.cu:1240-1249):** the event pair wraps `bit_rev_step1_parallel` (CPU), `run_first_ntt_pass`, and `run_second_ntt_pass_async_scatter`. That includes H2D/D2H (async copies at ntt.cu:970-986 and :1051-1068), GPU kernels, and CPU scatter. Transfers and host-side permutation are INSIDE the timer.
- **Timer scope in main's standard4 baseline (ntt.cu:1298-1344):** wraps chunked H2D/D2H `cudaMemcpy2DAsync`, GPU transpose kernels, CPU `transpose_matrix_parallel` (line 1336), and a memcpy (1337). Transfers and CPU transpose are INSIDE the timer.
- **Hardware in readme:** H100 and RTX 4090, CUDA 12.8, Ubuntu 24.04 (same as ae). No 2^20 timings are in the readme.
- **Baseline:** readme states the paper's missing-op CPU numbers came from arkworks; this artifact uses blst. astra-1 reports the readme says blst is 25-55% faster than the paper's arkworks CPU on miss-ops; I did not find that exact figure in the readme text I read, so treat it as unverified.
- Not run: no build or execution of either version.
