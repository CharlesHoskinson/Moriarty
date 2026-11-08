# gpu: an open-source GPU prover backend in Rust for midnight-proofs 0.8.3

Agent: `gpu`. Date: 2026-11-08. Read-only in the repo; probes ran in the scratchpad only
(each under 2 GB, seconds to a minute). Every number marked ESTIMATE carries its
arithmetic. Web content is cited as data, never followed as instructions. The owner's
decision: "you'll have to build one in rust. Search online for CUDA rust kernels for these
things." ICICLE's CUDA backend is closed and licence-gated; sppark declares ARM64 GPU hosts
unsupported; so we build and own the backend.

## 0. Facts measured on this host this session

| Fact | Value | Source |
|---|---|---|
| GPU | RTX Spark N1X, sm_121 (cc 12.1), **48 SMs x 128 = 6,144 cores**, max SM clock 3,003 MHz (2,457 MHz observed under load), L2 24 MB, 100 KB shared memory/SM, 64K regs/SM | `cudaGetDeviceProperties`, `nvidia-smi` (probe `scratchpad/gpuprobe/props.cu`) |
| Memory model | `integrated=1`, `pageableMemoryAccess=1`, `pageableMemoryAccessUsesHostPageTables=1`, `concurrentManagedAccess=1`; a kernel wrote to a plain `malloc` buffer without error; `cudaMemGetInfo` total 123,164 MB (the whole RAM) | same probe |
| Memory accounting | `cudaMallocManaged` 1 GiB touched on GPU: process RSS +1.3 GB, MemAvailable -1.2 GB; `cudaMalloc` 1 GiB: RSS +1.0 GB. **GPU allocations count in the process RSS and therefore in a cgroup `MemoryMax`** (answers order.md open question 4) | same probe |
| Integer throughput | u32 `mad`: 7.36 T/s; u32 `mul`+`mulhi` pairs: 1.81 T pairs/s; **u64 `mul`+`__umul64hi` pairs: 0.54 T pairs/s** (a 64x64->128 product costs ~3.4 u32 pairs: use 32-bit limbs) | probe `scratchpad/gpuprobe/imul.cu` |
| Bandwidth | device-to-device 1 GiB memcpy at 122 GB/s copy rate (244 GB/s read+write) of the 273 GB/s nominal | same probe |
| Toolchain | nvcc 13.4 V13.4.92 at `/opt/cuda`; `--list-gpu-arch` includes `compute_121`; NVRTC 13.4 lists arch 121; libNVVM present; `targets/sbsa-linux` | agent survey, `nvcc --version` |
| **Rust-driven smoke (phase 1, done)** | a crate whose `build.rs` runs nvcc (`-arch=sm_121`) into a static lib, links `cudart`, and calls an `extern "C"` launcher from Rust: 65,536 8-limb schoolbook products on **plain `Vec<u32>` memory (zero-copy)**, 0 mismatches vs a CPU u128 reference, 367 ms including context creation | `scratchpad/rustsmoke/` |
| CPU BLST MSM, 2^20 random 255-bit scalars, 20 cores | **0.51 s** per `G1Projective::multi_exp` (3 runs: 519/507/515 ms), of which the projective->affine batch conversion `p1_affines::from` is 39 ms; 2.6 s on 4 cores; 128-bit scalars 0.33 s | `scratchpad/blstbench/` |
| CPU `best_fft` over Fq, 20 threads | 2^20: 60-78 ms; 2^21: 120-129 ms; 2^22: 246-260 ms; **2^23: 468-544 ms**; on 4 threads 2^20 173 ms, 2^23 1.40 s | same bench |

Prover facts that fix the integration points (all stock 0.8.3, registry copy identical to
the vendored one in `msm.rs`):
- `msm_specific` (`src/poly/kzg/msm.rs:163-191`) filters zero scalars, then for BLS12-381
  casts to `G1Projective::multi_exp(bases, coeffs)`; the bases are **projective**
  (`params.g`, `params.g_lagrange: Vec<E::G1>`), so BLST converts them to affine on
  **every call** (`midnight-curves-0.3.1/src/bls12_381/g1.rs:628-648`, `p1_affines::from`,
  then `mult(scalar_bytes, 255)`). Callers: `commit` and `commit_lagrange`
  (`kzg/mod.rs:74-99`), `MSMKZG::eval` (`msm.rs:140-146`, verifier side).
- Commit sites per proof: advice columns (`plonk/prover.rs:569`), instance columns when
  committed (`prover.rs:436`), lookup `m`/products (`lookup/prover.rs:119,283`),
  permutation products (`permutation/prover.rs:155`), trash (`trash/prover.rs:59`), the
  vanishing random poly and the `h` pieces (`vanishing/prover.rs:78,124`), the multiopen
  `f_com` and the final witness (`kzg/mod.rs:171` and below). All go through `msm_specific`.
- `best_fft` (`midnight-curves/src/fft.rs:30`, radix-2, rayon) is called from
  `EvaluationDomain` only: `lagrange_to_coeff`, `coeff_to_lagrange`, `coeff_to_extended`
  (coset by `distribute_powers_zeta` then FFT at `extended_k`), `extended_to_coeff`,
  `extended_to_lagrange` (`poly/domain.rs:225-320`). `extended_k = k + ceil(log2 d)`, so
  the node at k=20 with d <= 9 runs 2^23 transforms (prover.md F8).
- Points reach the transcript as 48-byte compressed encodings
  (`transcript/implementors.rs:138-146`, `GroupEncoding::to_bytes`). Projective triples
  never leave the process, so "bit-identical" means **group equality**, checked on
  `to_affine()` / compressed bytes; different (X,Y,Z) for the same point is fine.
- The commitment-scheme type is baked into keys and gadgets:
  `ProvingKey<Fq, KZGCommitmentScheme<Bls12>>` in zk-stdlib 2.4.0 (`lib.rs:1543,1615`),
  `KZGCommitmentScheme<Bls12>` in `mori-recursion/src/ivc.rs:219`, `inner.rs:104`, and in
  the circuits verifier gadget. A substitute scheme type is not an option; **hooks inside a
  patched midnight-proofs are**.
- Upstream HEAD (`repos/midnightntwrk/midnight-zk`, unreleased) already stores the SRS as
  affine with an affine MSM path (#350) and caches twiddles with a pruned DIF FFT for
  `coeff_to_extended` (#352) (`proofs/CHANGELOG.md:101-102`). The next ledger pin brings
  those CPU gains and makes the GPU input simpler (affine bases).

## 1. Findings, ranked

### F1. Host side in Rust, kernels in CUDA C++, compiled by nvcc from `build.rs`: the only path that is ready on sm_121 today
Every Rust-kernel route is blocked on this host right now (details in §2):
Rust-CUDA needs `nightly-2026-04-02` plus `rustc-dev` and libNVVM (LLVM 7 or 21), is
git-only since 2022, emulates i128 and has no verified sm_121 build; NVIDIA's cuda-oxide
0.3.0 (2026-10-31) runs on stable 1.99.1 but lists sm_80..sm_120a, not sm_121, and has no
u128 or mul-hi; CubeCL 0.11 is alpha with API churn; rust-gpu/wgpu have no u128 and buggy
u64. The pattern that works (smoke above) is also the pattern candle-kernels uses
(`cudaforge` 0.1.x driving nvcc in build.rs). The kernels stay small C++ templates we own;
the whole algorithm, dispatch, caching, fallback and tests are Rust.

### F2. Reuse sppark's field and curve templates (Apache-2.0); take the MSM plan from Yrrid/Snarkify 2023 (MIT/Apache); ignore the ARM64 README caveat until it bites
sppark is the best-maintained permissive BLS12-381 CUDA code (commits 2026-10-14/20,
Apache-2.0, `msm/pippenger.cuh`: Booth signed digits, XYZZ buckets 8M+2S mixed add,
windows up to 18 bits, per-window sort, two-stream overlap; `ntt/`: mixed-radix CT/GS
passes of 6-10 bits with warp shuffles and shared memory; its `rust/src/build.rs` already
probes `compute_120/sm_120`, and `--dryrun`-probes other archs)
([repo](https://github.com/supranational/sppark), [pippenger.cuh](https://raw.githubusercontent.com/supranational/sppark/main/msm/pippenger.cuh),
[build.rs](https://raw.githubusercontent.com/supranational/sppark/main/rust/src/build.rs)).
Its README's "primarily supports x86_64 ... non-GPU portions on ARM64" is a support
statement; the build script has no `target_arch` check. ZKProphet measured sppark as the
fastest open MSM up to 2^20 ([arXiv 2509.22684](https://arxiv.org/html/2509.22684v1)).
Yrrid/Snarkify's ZPrize 2023 entry is the fastest open **BLS12-381** CUDA MSM found:
22-bit signed windows, 12 precomputed multiples per base so one window is accumulated,
buckets sorted largest-first, XYZZ for 381 (dual MIT/Apache-2.0, "Copyright (c) 2022 ZPrize";
[repo](https://github.com/yrrid/zprize-2023-uniform-scalars)). Its precompute table costs
12 x 96 MB = 1.15 GB per 2^20 base set; we use the idea at a smaller factor (§4).
Fallbacks with clean licences: ICICLE **v2.8.0 tag** (MIT, 2024-07-16, full CUDA
MSM+NTT source, unsigned digits, CUB sort;
[LICENSE](https://raw.githubusercontent.com/ingonyama-zk/icicle/v2.8.0/LICENSE),
[msm.cu](https://raw.githubusercontent.com/ingonyama-zk/icicle/v2.8.0/icicle/src/msm/msm.cu))
and ec-gpu (MIT/Apache, 32-bit `mad.lo.cc/madc.hi.cc` PTX chains and a radix-256
Stockham FFT; MSM is a 2021 design with w <= 10 and no signed digits;
[repo](https://github.com/filecoin-project/ec-gpu), ec-gpu-gen 0.7.1 of 2024-08-27, last
commit 2025-07-30). **Do not copy** from: open-icicle 2026 (MIT file vs README "special
license" conflict for the CUDA backend), polytope-labs/gnark-cuda (no licence file),
Nocy's proof server (no licence; naive per-point MSM), Snarkify's ZPrize 1B code (MPL-2.0),
the td-kwj WebGPU MSM (no licence file).

### F3. The GPU wins both MSM and NTT on this host; NTT by more (ESTIMATE, §4)
2^20 MSM: 50-90 ms on GPU vs 0.51 s BLST = **6-10x**. 2^20 NTT: 2-3 ms vs 60-78 ms;
2^23 NTT: 15-25 ms vs 0.47-0.54 s = **20-30x**. The NTT gain is larger because the CPU
FFT is memory-bound and the GPU moves each 256 MB column three times through 244 GB/s,
while the CPU's 20 cores also share that bus. Whether either is worth doing in the node
depends on the C0 shares (`commitment` and `Ntt` owners); with the counts in §0 the node's
~80 column-sized MSMs cost ~41 s and its ~80 transforms at 2^20 plus 2^23 cost 40 s or more
if they ran one after another at 20 threads: both are large (ESTIMATE; the prover runs
columns in parallel, so the real shares are lower and only C0 can set them).

### F4. Unified memory changes the design: no copies, one address space, one cap
Kernels address pageable host memory (ATS, `pageableMemoryAccess=1`), so the SRS, the
scalar vectors and the polynomial buffers need no `cudaMemcpy`; sppark's per-call copy
and sync weakness (s3 cluster) disappears. The price: GPU allocations are process RSS
(measured) and compete for the same 273 GB/s and the same 48 GB / 100 GB caps. Random
gathers over ATS-mapped pageable memory carry a translation cost we have not measured;
§4 keeps hot data (affine SRS, precompute tables, buckets, twiddles) in `cudaMalloc`
buffers and streams the per-call inputs zero-copy, and P3 measures both.

### F5. Bit-identity is a group-equality property with four edge classes
MSM is a pure function; the GPU result must equal BLST in G1. The classes to test: the
identity point as a base and as the result (snark-verifier CVE-2026-62337 is about an
identity-point check, the same hazard in verification form; the brief names it for the
negatives), zero scalars (filtered before the call, but the kernel must also accept them),
duplicate bases (bucket adds of equal points: the XYZZ mixed-add formula is incomplete at
P = Q and must branch to a doubling), and the exceptional cases of signed digits (digit
= -2^(c-1), top-window carry, scalar r-1, scalar 1).

### F6. The dispatch hook needs a patched midnight-proofs, and the patch must live outside the main workspace
`msm_specific` and the `EvaluationDomain` transforms are free functions with no trait
seam. The minimal patch is two registries (`hooks::MSM`, `hooks::FFT` as `OnceLock`
function pointers) consulted at those sites: about 60 lines, inert when nothing is
registered. A `[patch.crates-io]` in the main `Cargo.toml` would rewrite `Cargo.lock` for
every consumer including the ledger crates (midnight-circuits 7.3.0, mori-backend,
mori-ledger, midnight-zkir, blake2b_halo2 and sha3-circuit all resolve `midnight-proofs
0.8.3`; `Cargo.lock:2137`), which RPP-007 and order.md 5.8 ("the registry crate is the
default") forbid. The pattern that already exists is `tools/profile/`: a separate
workspace with its own `Cargo.lock`, a vendored patched crate and `[patch.crates-io]`
(`tools/profile/Cargo.toml:30-31`). So: `tools/gpu-prover/` with the same shape.

## 2. Toolchain survey (versions as of 2026-11-07; agent survey, URLs as given)

| Option | Version, licence | sm_121 / aarch64 | Kernel language | 64-bit mul | Verdict |
|---|---|---|---|---|---|
| **nvcc 13.4 from `build.rs` + `extern "C"` + static `cudart`** | toolkit installed; `cc` 1.11.1 has `Build::cuda(true)`; `cudaforge` 0.1.6 (MIT/Apache) wraps nvcc with `CUDA_COMPUTE_CAP`; candle-kernels 0.11.1 uses it ([cc docs](https://docs.rs/cc/latest/cc/struct.Build.html), [cudaforge](https://github.com/guoqingbao/cudaforge), [candle build.rs](https://raw.githubusercontent.com/huggingface/candle/main/candle-kernels/build.rs)) | proven here today | CUDA C++ | `__umul64hi` native; 32-bit `mad.cc` chains preferred (measured) | **Rank 1** |
| cudarc | 0.19.10, 2026-09-24, MIT/Apache; feature `cuda-13040`; NVRTC and driver API; default dynamic loading ([repo](https://github.com/chelsea0x3b/cudarc), [v0.19.10](https://github.com/chelsea0x3b/cudarc/releases/tag/v0.19.10)) | host-only crate; link-search list lacks `targets/sbsa-linux` (dlopen path unaffected) | CUDA C via NVRTC or prebuilt PTX | n/a | Optional host layer in rank 1 if we want NVRTC or streams/events wrappers; not required |
| sppark Rust crate | 0.1.16, Apache-2.0, commits 2026-10 ([crate](https://raw.githubusercontent.com/supranational/sppark/main/rust/Cargo.toml)) | probes `sm_120`; no `target_arch` check; ARM64 GPU "unsupported" by README | CUDA C++ templates | 32-bit PTX | Source of field/curve/NTT templates for rank 1; P1 proves it compiles here |
| CubeCL | 0.11.3, 2026-10-22, MIT/Apache, "alpha"; `mul_hi` on u64 -> `__umul64hi`; sends `sm_121a` to NVRTC, falls back to `sm_120a` ([repo](https://github.com/tracel-ai/cubecl), [artifact.rs](https://raw.githubusercontent.com/tracel-ai/cubecl/main/crates/cubecl-cuda/src/compute/artifact.rs)) | yes via cudarc 0.19.6 | Rust `#[cube]` -> C++ -> NVRTC | u64 yes, no u128 | Rank 3: a plausible future home for Rust kernels; alpha API and JIT cold start |
| NVIDIA cuda-rust: cuda-oxide | 0.3.0, 2026-10-31, Apache-2.0; stable 1.99.1 + `rustc-dev`; CUDA >= 13, LLVM 21 `llc`; "early-stage ... not production-ready"; `--arch sm_120a` verified, sm_121 not listed; no u128/mul-hi ([blog](https://developer.nvidia.com/blog/introducing-cuda-rust-two-tracks-for-writing-gpu-kernels/), [repo](https://github.com/NVIDIA/cuda-rust), [features](https://nvidia.github.io/cuda-rust/cuda-oxide/appendix/supported-features.html)) | aarch64 work since 0.2.0 (GB200, DGX Spark docs) | Rust -> PTX AOT | u64 yes | Rank 4 today; re-check quarterly |
| NVIDIA cuda-rust: cutile-rs | 0.5.0, 2026-11-05, Apache-2.0; stable 1.89+; "runs on the DGX Spark's GB10 (sm_121, aarch64) with CUDA 13.3 and 13.4" ([v0.4.0 notes](https://github.com/NVIDIA/cuda-rust/releases/tag/cutile-rs%2Fv0.4.0)) | yes | Rust tile model -> Tile IR | i64/u64 elements; no mul-hi documented | Not a fit for per-thread limb arithmetic |
| Rust-CUDA | git-only (crates.io 0.3 from 2022), MIT/Apache; `nightly-2026-04-02`; LLVM 7 or 21; `NvvmArch::Compute121` exists; i128 emulated ([repo](https://github.com/Rust-GPU/Rust-CUDA), [update](https://rust-gpu.github.io/blog/2025/08/11/rust-cuda-update/)) | ARM64 Docker images; no stated aarch64 support | Rust `cuda_std` | u64 yes | Rank 5: nightly pin and libNVVM in the production build |
| rust-gpu / wgpu | spirv-builder 0.11.0 (2026-10-13); wgpu 30.0.1 `SHADER_INT64`; no u128 (#359), u64 bugs (#501), no mul-hi ([rust-gpu #359](https://github.com/Rust-GPU/rust-gpu/issues/359), [wgpu Features](https://docs.rs/wgpu/latest/wgpu/struct.Features.html)) | n/a | Rust/WGSL | 32-bit limbs only | Not viable for 384-bit arithmetic |
| bare `nvptx64-nvidia-cuda` | tier 2, `rust-std` shipped on stable; `extern "ptx-kernel"` and `core::arch::nvptx` are nightly; `-Ctarget-cpu=sm_121` needs `-Zbuild-std` ([platform page](https://doc.rust-lang.org/nightly/rustc/platform-support/nvptx64-nvidia-cuda.html)) | LLVM knows sm_121 | Rust | folds to `mul.hi.u64` (UNVERIFIED) | nightly: no |
| ec-gpu-gen | 0.7.1, 2024-08-27, MIT/Apache; kernels are `.cl` text compiled by nvcc at build time; `EC_GPU_CUDA_NVCC_ARGS` for arch | no arch logic | OpenCL-C/CUDA text generated from `ff` constants | 32-bit PTX | Rank 6 for "extend": the generator adds a layer we do not need; borrow its FFT and field snippets |

**Recommendation (ranked):** (1) `crates/mori-gpu`: Rust host, CUDA C++ kernels derived
from sppark (field, XYZZ, NTT passes) and the Yrrid 2023 plan (signed windows, sorted
buckets, small precompute), built by nvcc 13.4 in `build.rs`, linked statically with
`cudart` so a binary on a CUDA-less host falls back to CPU at runtime; cudarc only if we
want NVRTC. (2) The same crate design with kernels in CubeCL or cuda-oxide once one of
them verifies sm_121 and exposes u64 `mul_hi` on stable: the Rust API, tests and hooks do
not change, only the `cuda/` directory. (3) Extending ec-gpu-gen: licence fine, algorithm
stale, extra generator layer; no. Pure-Rust kernels are a migration target, not the first
step: the owner's "build one in Rust" is met by owning the whole backend in a Rust crate
with a small, tested CUDA core.

## 3. Crate design

```
crates/mori-gpu/                      (main workspace member; no midnight-proofs dependency)
  Cargo.toml        features: cuda (default off); deps: midnight-curves =0.3.1, blst =0.3.17, ff, group
  build.rs          nvcc -O3 -std=c++17 -Xcompiler -fPIC -gencode arch=compute_121,code=sm_121
                    (MORI_GPU_ARCH overrides, e.g. compute_120 PTX for portability) -> libmorikernels.a
  cuda/field.cuh    Fp (12x32 limbs) and Fr (8x32) Montgomery, mad.lo.cc/madc.hi.cc chains (sppark-derived)
  cuda/curve.cuh    affine, XYZZ, mixed add with the P==Q and identity branches, dbl
  cuda/msm.cu       recode (signed digits), bucket index sort, accumulate, reduce, final sum
  cuda/ntt.cu       radix-2^r shared-memory passes, bit-reversal, twiddle tables; forward and inverse
  src/lib.rs        Gpu::try_init() -> Option<Gpu>; env MORI_GPU=auto|on|off, MORI_GPU_MIN_K (default 14)
  src/msm.rs        msm_g1(scalars: &[Fq], bases: &[G1Projective]) -> G1Projective
  src/srs_cache.rs  affine SRS tables keyed by (ptr, len, blake2b of first/last 64 points); optional
                    shifted-base precompute (factor f, default 1; 4 or 8 when MSM share justifies 0.4-0.8 GB)
  src/ntt.rs        fft(a: &mut [Fq], omega: Fq, log_n: u32) matching best_fft's contract exactly
  src/fallback.rs   the CPU path (BLST multi_exp, best_fft) used when !cuda, init fails, or n < 2^MIN_K
  tests/            differential tests (§5); benches/ criterion micro-benchmarks that write runs.jsonl rows

tools/gpu-prover/                     (separate workspace, own Cargo.lock, like tools/profile)
  Cargo.toml        [patch.crates-io] midnight-proofs = { path = "vendor/midnight-proofs-0.8.3-gpu" }
  vendor/midnight-proofs-0.8.3-gpu/   registry 0.8.3 + patches/gpu-hooks.diff (~60 lines, SHA-256 checked)
  patches/gpu-hooks.diff   src/hooks.rs: pub static MSM_G1: OnceLock<fn(&[Fq],&[G1Projective])->G1Projective>,
                           pub static FFT_FQ: OnceLock<fn(&mut [Fq], Fq, u32)>; msm_specific and the
                           EvaluationDomain transforms consult them (Fq/G1 specialisations only)
  src/main.rs       `heavy --gpu` entry: mori_gpu::install() registers the hooks, then calls the same
                    mori-recursion / mori-backend APIs the CPU heavy path calls
```

- **Dependency direction:** midnight-proofs never depends on mori-gpu (no cycle); the hook
  signatures use midnight-curves types both sides already depend on. The patched crate
  with no hook registered is behaviourally identical to the registry crate.
- **Fallback:** three levels. Compile-time (`cuda` off: the crate is the CPU path);
  init-time (`cudaGetDeviceCount` fails or `MORI_GPU=off`); call-time (`n < 2^MIN_K`, or
  the kernel reports an error: fall back and record `gpu_fallbacks` in the receipt). The
  GPU is never a correctness dependency.
- **Patch or vendor midnight-proofs?** Vendor a patched copy in `tools/gpu-prover/` only.
  The main workspace keeps the registry 0.8.3 and the T0 isolation check
  (`scripts/qa/check-profile-isolation.py`) is extended to the second vendored copy. The
  patch is a diff against the pinned version; a ledger re-pin (task 2.1 style) means
  re-applying a 60-line diff and re-running the byte-identity test (§5). If upstream ships
  #350/#352 in the next pin, the MSM hook takes affine bases and the FFT hook moves to the
  pruned DIF path; the crate API does not change.
- **Receipts:** `runs.jsonl` rows gain `gpu: {arch, driver, toolkit, msm_calls, ntt_calls,
  fallbacks, device_bytes_peak}`; `budgets.json` gets `gpu-msm-node-step` (order.md A21,
  slot `gpu0`, 300 s / 48 GB).
- **Concurrency:** one GPU; `heavy --gpu` processes take `gpu.lock` (already in
  `scripts/qa/gpu-runner.py`) or accept CUDA time-slicing; §6 sizes the device memory per
  process at 1.5-2.5 GB so three GPU provers fit under the aggregate cap.
- **General-purpose rule:** the backend takes any `n <= SRS size` and any `log_n <= 32`
  (`Fq::S = 32`); nothing in it knows the program or the circuit; the SRS cache is keyed
  by content, not by circuit.

## 4. Kernel algorithms, parameters, estimates

Cost model (ESTIMATE, from the §0 probes): one Fp (381-bit, 12 limbs) Montgomery
multiplication = 144 `mad.lo` + 144 `mad.hi` for the product and about the same for the
reduction = ~580 IMAD, plus ~300 carry adds at half rate = ~900 IMAD slots; at 7.36 T
IMAD/s that is 8.2 G Fp mul/s peak; I use **4 G Fp mul/s** (50%) and 2.5 G/s as the
pessimistic bound. One Fr (255-bit, 8 limbs) multiplication = ~400 slots -> 18 G/s peak,
**9 G/s** used. Effective bandwidth 150 GB/s for strided or gathered traffic, 240 GB/s
streaming (measured 244 GB/s read+write).

### 4.1 MSM: signed-digit Pippenger with sorted buckets, XYZZ accumulators, cached affine SRS
- Scalars arrive as `Fq` in Montgomery form; the recode kernel converts (one Montgomery
  multiply by 1) and emits signed digits in [-2^(c-1), 2^(c-1)] with carry, c = 16 for
  2^19-2^21 points (16 windows over 255 bits; sppark's rule `lg2(1.5 n) - 8` gives 12-13,
  Yrrid uses 22 with precompute; 16 balances 2^15 buckets/window against the 24 MB L2).
- Per window: sort (digit, index) pairs by |digit| (sppark's cooperative sort or CUB radix
  sort), accumulate buckets by warps over sorted runs (no atomics), running-sum reduce,
  then one Horner combination over windows on the host or in a tiny kernel.
- Bases: affine SRS cached on device once per (SRS, size) (96 MB at 2^20, 39 ms to build
  with BLST's batch inversion, or on the GPU). Optional shifted-base precompute factor f:
  windows/f accumulations, f x 96 MB per table; f = 1 until C0 shows the MSM share.
- Edge handling in the mixed add: identity base -> skip; bucket identity -> copy; equal x
  -> doubling or identity. Zero digits are skipped by the sort.
- **2^20 estimate:** bucket accumulation 2^20 x 16 windows = 16.8 M mixed adds x 10 Fp mul
  (XYZZ madd 8M+2S) = 168 M; bucket reduction 16 x 2^15 x 2 full adds x 14 = 14.7 M; total
  183 M Fp mul / 4 G/s = **46 ms** (73 ms at 2.5 G/s). Memory: 16 gathers of the 96 MB base
  table = 1.5 GB at 150 GB/s = 10 ms; digits and sort ~0.4 GB = 3 ms; mostly overlapped.
  **Total 50-90 ms vs 510 ms BLST: 6-10x.** With f = 8: compute /8, gathers unchanged ->
  15-25 ms (20-30x) for 768 MB of tables per base set (two sets: `g` and `g_lagrange`).
- 2^21: 2x; 2^19: 0.5x; below 2^14 the fixed cost (~0.3 ms of launches and syncs plus
  recode) exceeds BLST's time, so `MIN_K = 14` is the starting threshold.
- Precision of the comparison: BLST's 510 ms includes 39 ms of affine conversion that the
  cache removes from the GPU path and that upstream #350 will remove from the CPU path.

### 4.2 NTT: radix-2^r shared-memory passes over device-resident buffers
- Three passes for 2^20-2^23 (r = 7,7,6 .. 8,8,7), each pass loads a tile of 2^r elements
  per block into shared memory (2^8 x 32 B = 8 KB; 100 KB/SM allows 12 blocks), applies r
  radix-2 stages with on-chip twiddles, and writes back; sppark's `ct_mixed_radix_wide`
  and ec-gpu's `fft.cl` are both this shape. Twiddle table per (omega, log_n): 2^(log_n-1)
  Fr = 128 MB at 2^23, cached on device; or computed per block from `omega^(2^i)` (32
  values) at a few extra multiplications per butterfly, which the memory-bound passes
  absorb. The inverse is the same kernel with `omega_inv` and a scale pass, exactly as
  `EvaluationDomain::ifft` does it today; `distribute_powers_zeta` stays on the CPU (or
  fuses into the first pass later).
- Data movement: the host `Vec<Fq>` is pageable; copy in and out of a device scratch
  (2 x 256 MB at 2^23, resident per process) at ~120 GB/s: 2.1 ms each way at 2^23, 0.3 ms
  at 2^20; alternatively zero-copy in the first and last pass (P4 measures which is faster).
- **Estimates:** 2^20: 10.5 M butterflies x 1 Fr mul / 9 G/s = 1.2 ms compute; 3 passes x
  64 MB = 192 MB / 240 GB/s = 0.8 ms; copies 0.6 ms -> **2-3 ms vs 60-78 ms (25x)**.
  2^23: 96 M butterflies / 9 G/s = 10.7 ms; 3 x 512 MB = 1.5 GB / 240 GB/s = 6.4 ms; copies
  4.2 ms -> **15-25 ms vs 470-540 ms (20-30x)**. 2^21: 4-6 ms; 2^22: 8-12 ms.
- Is NTT worth doing on the GPU? Yes on this host, because the CPU transform is
  bandwidth-bound on the same bus and parallelises poorly (4 -> 20 threads gave 2.9x at
  2^20, 2.6x at 2^23). The per-proof bus budget stays small: ~80 transforms x 1.5 GB =
  120 GB = 0.5 s of bus time per node step. The decision rule stays C0's: adopt NTT if the
  `Ntt` owner share t >= 0.15 (bound 1/(1-t) >= 1.18x).

### 4.3 Node step (ESTIMATE; C0 fixes the shares)
With MSM share m and NTT share t of the 95 s node step and GPU factors 7x and 15x:
m = 0.4, t = 0.3 -> 1/(0.3 + 0.4/7 + 0.3/15) = **2.65x** (95 -> 36 s);
m = 0.25, t = 0.15 -> 1/(0.6 + 0.036 + 0.01) = **1.55x** (95 -> 61 s). MSM alone at
m = 0.3: 1.35x (prover.md F5). The same factors shrink at k = 19 because the
domain-proportional phases halve and the fixed phases (synthesis, key load) do not:
re-evaluate after P4 of prover.md with the k19 profile. Contention: a GPU prover beside
leaf workers adds its 120-280 GB of bus traffic per node step to theirs; at 273 GB/s that
is 0.5-1 s per step, under 2% of the step.

## 5. Correctness and assurance plan

1. **Differential MSM tests vs BLST** (`mori-gpu/tests/msm_diff.rs`, run on CPU-only hosts
   as a no-op): random scalars and bases at n = 1, 2, 3, 2^k and 2^k +- 1 for k in 10..=21
   (the post-filter length is arbitrary); scalars 0, 1, 2, r-1, 2^(c-1) and multiples of
   2^c at every window (digit = -2^(c-1) and the carry), all-zero vector (identity result),
   one non-zero; bases: identity at the first, last and random positions, duplicate bases
   (expected sum of scalars times the base), projective bases with Z != 1, whole vectors
   repeating one base; inputs host-resident vs device-resident (the ICICLE/gnark Montgomery
   chunking bug class, s3 cluster). Oracle: `G1Projective::multi_exp` and, for n <= 2^12, a
   naive sum of `base * scalar`. Equality on `to_affine()` and compressed bytes.
2. **Differential NTT tests vs `best_fft`**: log_n in 1..=23, omega and omega_inv at each
   size, random and structured vectors (all zero, delta, constant, alternating), in-place
   round trip (fft then ifft with the divisor equals the input), coset transforms through
   the patched `EvaluationDomain` vs the registry one on the same vector. Bit-exact
   comparison of every element.
3. **Proof byte-identity** (the A5/G10 predicate of the design): run the node step of
   `entry 7;` at the same key, same witness and a fixed RNG seed on the CPU path and the
   GPU path; the proof bytes must be equal, and both must pass the stock verifier and the
   mori-recursion `verify_chain`; repeat for one leaf (k13, below `MIN_K`: exercises the
   fallback) and the decider. A seedable RNG entry must exist in the prove API or the
   test uses `ChaCha20Rng::seed_from_u64` through the existing test harness; if blinding is
   drawn from `OsRng` only, the predicate degrades to "verifies and all commitments equal
   given logged blinding", which must be stated in the receipt.
4. **Fuzzing**: `cargo fuzz` targets `msm_diff` and `ntt_diff` with `arbitrary`-derived
   inputs biased to the edge classes above (small n, special scalars, identity and
   duplicate bases); run under `gpu.lock` on this host, nightly, 30 min each; any mismatch
   is a release blocker and gets a regression test.
5. **Kernel unit tests**: field multiplication vs blst on 10^6 random pairs plus the
   boundaries (0, 1, p-1, R, R^2 in Montgomery form); XYZZ add/dbl vs blst on random and
   equal points and identity; digit recode vs a Rust reference that reconstructs the scalar.
6. **Formal models (small, worth it):**
   - Quint: signed-digit Pippenger over a small abstract group (integers mod a prime q as
     the additive group, scalar modulus r small): recode with carries, bucket accumulation,
     running-sum reduction, Horner over windows; invariant: result = sum s_i * P_i for all
     inputs including digit -2^(c-1), the top-window carry, zero digits and duplicate
     indices. This is the bug class that sorting and signed digits introduce (an off-by-one
     in the bucket index or a lost carry), and it is cheap to model.
   - Quint or a Rust proptest: the NTT index permutation, i.e. that three bit-sliced passes
     with the chosen r's compose to the bit-reversal `best_fft` applies, for log_n in 1..=23.
   - Lean: the digit recoding identity `sum d_i 2^(c i) = s` with d_i in [-2^(c-1), 2^(c-1)]
     for all s < 2^255, and completeness of the XYZZ mixed-add case split (P = Q, P = -Q,
     identity operands) over the formulas used. Both are short; both are exactly where
     "bit-identical" could fail silently.

## 6. Phased task list (predicates, gates, kill criteria)

| Phase | Work | Predicate (gate) | Kill criterion | Effort (ESTIMATE) |
|---|---|---|---|---|
| **G0 toolchain smoke** | nvcc sm_121 kernel (done 21:57); Rust `build.rs` -> nvcc -> static lib -> kernel on plain `Vec` memory (done this session, 0 mismatches) | both pass | - | done |
| **G1 source compile** | compile sppark's `poc/msm-cuda` for BLS12-381 and its `ntt` test with `-arch=sm_121` on this aarch64 host; run 2^16 vs BLST | compiles and matches BLST on 2^16 random | no compile after 2 working days -> switch to ICICLE v2.8.0 or Yrrid sources; if none compiles, write `field.cuh` ourselves (+1-2 weeks) | 2-4 days |
| **G2 field kernels** | `field.cuh`, `curve.cuh` in `crates/mori-gpu`, unit tests vs blst (§5.5) | all tests pass; Fp mul throughput >= 2 G/s on a 2^24-element micro-benchmark | throughput < 1 G/s after tuning (the MSM could not beat BLST by 3x) -> stop, record | 1 week |
| **G3 MSM** | recode, sort, accumulate, reduce; SRS cache; differential tests (§5.1); criterion bench | 2^20 random MSM <= 100 ms and all differential tests bit-identical; `MIN_K` measured | > 250 ms at 2^20 after two tuning rounds (< 2x over BLST) -> close MSM, keep the crate | 2-3 weeks |
| **G4 NTT** | three-pass kernel, twiddles, inverse, differential tests (§5.2) | 2^23 <= 40 ms, 2^20 <= 5 ms, bit-exact | > 200 ms at 2^23 (< 2.5x) -> close NTT, MSM stands alone | 1-2 weeks |
| **G5 integration** | `tools/gpu-prover/` workspace, hooks patch, `heavy --gpu`, receipts, isolation check | proof byte-identity on `entry 7;` node, leaf and decider (§5.3); stock verify; T0 isolation passes | byte mismatch not explained and fixed within a week -> hold at CPU | 1-2 weeks |
| **G6 node-step measurement** | `gpu-msm-node-step` budget row; node k20 (and k19 once cut) with 0, 2, 3 leaf workers beside it | `node_step_wall` improvement >= C0 Amdahl bound - 20% (order.md A21); per-process RSS incl. device <= 48 GB; aggregate <= 100 GB | < 1.2x on the node -> GPU row closed, crate kept for later pins | 2-3 days |

Gates with the rest of the plan: G1-G2 can start now (no circuit dependency, no heavy
build); G3/G4 adoption waits for C0's `commitment` and `Ntt` shares (order.md table 2.5:
open if m >= 0.45 after k19 for MSM; this file adds t >= 0.15 for NTT); G5 waits for the
post-2.1 pins to settle so the patch is cut once. Fuzzing (§5.4) starts at G3 and runs for
the life of the crate.

## 7. Risks

- **sm_121 and CUDA 13.4 are months old.** Only CUDA >= 13.0 knows cc 12.1; NVIDIA's
  compatibility guide does not name sm_121 but states same-major, higher-minor cubin
  compatibility (sm_120 cubins run), and the forum threads that reported NVRTC failing on
  sm_121 were on CUDA 12.8 ([guide](https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html),
  [forum](https://forums.developer.nvidia.com/t/request-for-sm-121-tuned-kernels-in-cudnn-cublas-dgx-spark-training-throughput-gap/371094)).
  Mitigation: build `sm_121` SASS plus `compute_120` PTX; pin the toolkit version in the
  receipt; G1 is time-boxed.
- **aarch64 host compile of borrowed CUDA code.** sppark's host-side helpers may carry
  x86 assumptions (unverified); the s3 cluster's "non-GPU portions on ARM64" note is the
  only statement. G1 answers it in two days.
- **Unified-memory contention.** Every GPU byte shares 273 GB/s with the CPU prover and
  the leaf workers. §4.3 bounds the GPU's own traffic at 0.5-1 s per node step; the
  unknown is ATS translation cost on gathers, which P3 measures (device-resident tables
  are the fallback).
- **Memory caps.** Device allocations are RSS (measured) and count under `MemoryMax`.
  Per GPU prover: SRS affine tables 2 x 96 MB (k20), optional precompute up to 1.5 GB, NTT
  scratch 2 x 256 MB, MSM scratch ~0.3 GB, twiddles 0.13 GB: **1.5-2.5 GB**, inside the
  48 GB cap and the 100 GB aggregate with the 26 GB (k20) or 13 GB (k19) CPU side. The
  CUDA context itself took ~0.3 GB in the probe.
- **Correctness surface.** The incomplete-addition and signed-digit edge cases of §F5; the
  Montgomery/chunking class seen in gnark+ICICLE (s3 cluster); any silent difference shows
  up only as a decider that stagenet refuses (prover.md risk list). The differential tests,
  the byte-identity gate and fuzzing are the defence; none of them need a GPU-free CI to
  pass vacuously (they must run on this host under `gpu.lock`).
- **Maintenance burden.** A CUDA C++ core in a Rust repo: nvcc in the build, a vendored
  patched midnight-proofs that is re-based at every ledger pin, kernels retuned per GPU
  generation, and Apache-2.0 attribution for sppark-derived files. Keep the CUDA core under
  ~2,000 lines, keep the hooks patch under 100 lines, and keep the registry crate the
  default so the main workspace never depends on any of it.
- **Diminishing dividend.** k19 halves the domain-proportional phases the GPU accelerates;
  upstream #350/#352 speed up the CPU MSM input and FFT; S = 8 cuts the number of node
  steps. The GPU's value is highest exactly when those have not landed; G6's kill criterion
  is measured after them, not before.
- **Licence hygiene.** Nothing from ICICLE v3+/open-icicle CUDA backends, Nocy,
  gnark-cuda or the WebGPU entries enters the tree; sppark (Apache-2.0), Yrrid 2023
  (MIT/Apache), ICICLE v2.8.0 (MIT) and ec-gpu (MIT/Apache) may, with headers preserved.

## 8. Open questions

1. C0 shares `commitment` (m) and `Ntt` (t) of the node step at the new pins (the only
   inputs §4.3 lacks). Also the number of MSM and NTT calls per proof, which the
   `recursion-phase-profile` events already record.
2. Does sppark's host code compile on aarch64 with nvcc 13.4 (G1)?
3. ATS gather cost: is a zero-copy base table within 1.5x of a device-resident one?
4. Is there a seedable RNG path through the node prove API for the byte-identity test, or
   does blinding come from `OsRng` only?
5. Will the next ledger pin carry #350 (affine SRS) and #352 (pruned DIF FFT)? It changes
   the hook signatures and the CPU baseline.
6. Should concurrent `heavy --gpu` processes serialise on `gpu.lock` or share the GPU by
   time-slicing (measure both in G6 with 2 and 3 workers)?
7. When cuda-oxide or CubeCL verifies sm_121 with u64 `mul_hi` on stable Rust, do we port
   the kernels (same tests, same hooks) to retire nvcc from the build?
