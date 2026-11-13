# Rust MSM for the K2 kernel: plan

Owner request, 2026-11-11: rewrite the MSM kernel in Rust to match the enhancements from ZKProphet (arXiv 2509.22684, IISWC 2025). Branch `msm-rust` from `gpu-wrap` (52ad95069). Never pushed; main is not touched.

## 1. Target

K2 of the Sherlock GPU wrap prover: the BLS12-381 G1 MSM `sum s_i * P_i` with `s_i` in Fr (255 bits) and fixed SRS bases `P_i`, sizes 2^16 to 2^21, on the RTX Spark N1X (Blackwell, sm_121, 6144 CUDA cores, 120 GB unified memory).

- Device code is Rust: `crates/mori-msm/kernel`, `#![no_std]`, compiled to PTX for `nvptx64-nvidia-cuda` by the pinned Rust 1.99.0 (section 0 of RESULTS.md). The driver JIT compiles the PTX for sm_121.
- Host code is Rust: `crates/mori-msm`, cudarc 0.19.10 (driver API, PTX embedded in the binary).
- Baseline to beat (GWR section 2, MEASURED on a loaded host): sppark `msm_t` 0.48 s at 2^20 and 0.75 s at 2^21 for random scalars; BLST 0.51 s at 2^20 on a quiet host. The coordinator floor ESTIMATE is about 28 ms per 2^20. PROPOSAL.md section 8 needs 40 to 70 ms per 2^20 for the full-device wrap.

## 2. Enhancements, in build order

1. **Skew-robust bucket accumulation.** Our witness columns repeat small range-checked values; sppark gives one thread per bucket, so a hot bucket is serial (622 to 647 s for 45 calls without the mask). The new kernel never gives a bucket to one thread:
   - signed digits, window `c` chosen by size (default 16 at 2^20), digits of all windows as one key array `(window, |digit|)`; zero digits are dropped;
   - counting sort by key on the device (histogram with atomics, exclusive scan, scatter);
   - load-balanced segmented reduction: each thread takes a fixed count of consecutive sorted entries, whatever bucket they belong to, and emits one partial sum per key run; the partial array is sorted again by construction, and the same pass runs on it until every key has one entry. A hot bucket costs more rounds (log base chunk size of its length), not one long serial thread;
   - batch same-size commitments: an optional `msm_g1_batch` entry point puts several scalar vectors over one base table into one launch. The drop-in API stays one call per commitment.
2. **Batch-affine accumulation with Montgomery batch inversion.** The first round adds affine base points in pairs with `lambda = (y2 - y1) / (x2 - x1)`; the denominators of a round are inverted together (one inversion per batch, 3 multiplications per element). ZKProphet counts 3.3x fewer FF_mul than XYZZ (analytical, p.10) and about 300 MB of intermediates per 2^20 batch, which fits in unified memory. Equal x (doubling or P + (-P)) falls back to the XYZZ path for that pair.
3. **Register and ILP tuning, not occupancy.** ZKProphet Fig. 10: more warps raise math-pipe-throttle stalls; Table VI: sppark uses 216 registers per thread at 25 percent occupancy. Tune limbs (6 x 64 against 12 x 32 with carry chains), launch bounds and entries per thread by measurement; report registers and spills from `ptxas -v` for each kernel.
4. **Branch-free conditional reduction in FF_add, FF_sub and the final Montgomery subtraction.** ZKProphet Table IV and Section IV-C3: the branch of the conditional reduction is 70.5 percent of FF_add latency on uniform inputs. Every reduction uses a borrow mask and select, never a data-dependent branch. Point-level special cases (infinity, equal points) stay as branches, because they are rare.
5. **Precomputed shifted fixed bases (last).** Tables `2^(c w) P_i` kept per base table in the process (the resident daemon of OD-13 keeps them across proofs); all windows then share one bucket set, which removes the per-window bucket reduction (about 11 percent at 2^20, c = 16, reviewer ESTIMATE). Built only if steps 1 to 4 leave the reduction visible in the profile.

## 3. API (drop-in for the K2 call site)

The gpu-wrap integration calls `mori_gpu::msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective>` from the hook in `tools/gpu-prover/src/lib.rs` (`Fq` is the BLS12-381 scalar field in midnight-curves naming; `bases` may be longer than `scalars`; `None` means the caller runs BLST). `mori-msm` exports the same signature:

```rust
pub fn mori_msm::msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> Option<G1Projective>;
```

- Base tables are cached per (address, length) with the first and last point as a fingerprint, as in `mori-gpu`.
- The skew mask stays as it is: the wrapper computes `MSM(s + r) - MSM(r)` with the same splitmix64 vector and the cached `MSM(r)`. `MORI_GPU_MSM_MASK=0` turns it off for the skew benchmark.
- The hook selects the implementation at run time: `MORI_MSM=rust` routes the MSM phase to `mori-msm`, anything else keeps sppark. Same proof bytes either way, because the result is the same group element.

## 4. Correctness gate

- Every test: the result equals BLST (`blst_p1s_mult_pippenger` through midnight-curves) and the sppark path of `mori-gpu`, compared as group elements, for n = 2^10 to 2^21:
  - random scalars;
  - recorded witness columns of a real wrap run (`MORI_MSM_RECORD=dir` in the hook dumps the scalar vectors of one k = 20 proof; none are saved today);
  - edge vectors: all zero, all one value, one hot value at 99 percent, scalars p - 1, single nonzero entry.
- Then a full wrap proof at k = 20 with `MORI_MSM=rust` and `GW_DET=1`: proof SHA-256 equal to the CPU proof of the same run, library verifier, ledger 9 `VerifierKey::verify` and the negative control, as GWR section 1 did. Blinding and the skew mask are not changed.

## 5. Benchmark gate

- Sizes 2^16 to 2^21; inputs random scalars and the recorded wrap witness columns, each with the mask off (raw skew) and on (the shipped path).
- At least 3 repetitions per cell, min and median, first call (table build) reported apart from steady state.
- Same host, same lock hold: mori-msm, sppark (`mori-gpu`), BLST.
- Host load recorded at run time (`uptime`, `nvidia-smi`). MEASURED numbers only; no quiet-host claim from a loaded host.
- Heavy runs only through `flock ~/.cache/cost-opt/measure/heavy.lock /home/hoskinson/.local/bin/heavy-run`; runs over 30 minutes as `systemd-run --user --unit=msm-rust-... --collect`, logs in `~/.cache/msm-rust/`.

## 6. Steps and commits

1. Spike (done, RESULTS.md section 0).
2. Field and curve layer in the kernel: Fp add, sub, mul (branch free), XYZZ and affine point ops, device tests against BLST.
3. Sort-based skew-robust MSM with XYZZ accumulation; correctness gate on random and edge vectors.
4. Batch-affine first round.
5. Tuning (limbs, entries per thread, window size per n) with ptxas register reports.
6. Hook switch, witness recording, recorded-column tests, benchmark gate.
7. Full wrap proof at k = 20.
8. Precomputed shifted bases if the profile asks for it.
