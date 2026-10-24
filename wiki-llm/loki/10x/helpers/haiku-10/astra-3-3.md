# astra-3-3: WSL2 Blackwell throughput constraints (RTX 5090, sm_120) — revised

## Question
What capacity model and truthful concurrency limits apply to a CUDA GPU prover on this host (RTX 5090 32 GB, WSL2, 90 GB RAM, 20 cores), separating vendor kernel timings from end-to-end proving? Cover pinned/managed memory, streams, batching without cross-transcript batching, memory and precompute cost, and Nocy proof-server claims.

## Answer
- **Host snapshot (measured here, timestamped):** RTX 5090, 32607 MiB total, 28989 MiB free, driver 616.92, compute capability 12.0, PCIe Gen5 x16 current and max; 90 GB RAM (74 GB available), 20 cores. Captured 2026-10-24T22:44:41Z: raw/host-snapshot.txt. Free VRAM will vary with other processes (earlier snapshot 28990 MiB).
- **sm_120 binaries:** NVIDIA's Blackwell guide says binaries with only cubins must be rebuilt; binaries with forward-compatible PTX run unchanged. Build sm_120 cubin plus PTX. [raw/blackwell.md](https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html)
- **RTX 5090 spec:** NVIDIA's page (raw/rtx5090-spec.md) lists 32 GB GDDR7, 512-bit bus, PCIe Gen 5, CUDA capability 12.0, 21760 CUDA cores. It does not list memory bandwidth. PCGamesN (secondary, raw/pcgamesn-5090.md) states 1,792 GB/s; not checked against an NVIDIA datasheet.
- **WSL2 pinned memory:** NVIDIA's WSL guide (raw/wsl-guide.md) says pinned system memory availability "is limited", with no numeric cap. A NVIDIA forum staff reply (raw/pinned-forum.md) says the Windows cap is typically about 50% of system memory, "not in all cases", and that WSL2 limits are "likely lower". This is a forum statement, not documentation. Do not plan on 45 GB pinned. Probe cudaHostAlloc at increasing sizes on this host.
- **WSL2 managed memory:** the WSL guide says full managed-memory support is unavailable on WSL2 and that concurrent CPU/GPU access is not supported for managed memory; applications should query support. It does not restrict independent host CPU work and explicitly allocated device memory from overlapping. Design: explicit cudaMalloc plus bounded pinned staging; no managed-memory oversubscription.
- **Pinned transfers and streams:** the CUDA best-practices guide (raw/cuda-bestpractices.md, section 10.1.1) says pinned memory gives the highest host-device bandwidth (about 12 GB/s on PCIe Gen3 x16, a historical figure), is a scarce resource, and that cudaMemcpyAsync on non-default streams is how copies overlap kernels.
- **Transfer arithmetic (theory, not measured on WSL):** PCIe Gen5 x16 = 32 GT/s x 16 x 128/130 / 8 = 63.0 GB/s per direction (pre-packet upper bound). 128 MiB (one k22 extended column) takes about 2.1 ms at that bound and about 11 ms at the 12 GB/s Gen3-era pinned figure. WSL throughput is unmeasured.
- **Vendor MSM timing (ICICLE docs, RTX 3090 Ti, not 5090):** BLS12-381 2^20 MSM: 25.5 / 20.3 / 18.1 / 17.2 ms at precompute factors 1 / 2 / 4 / 8. Batch of 2^10 MSMs of size 2^12: 136.7 / 123.8 / 117.8 / 116.0 ms. The timing boundary (transfers included or not) is not stated, and the page is version 1.10.1. [raw/icicle-precomp.md](https://dev.ingonyama.com/1.10.1/icicle/golang-bindings/msm-pre-computation)
- **No 5090 MSM/NTT timing in the inspected sources.** Three searches for RTX 5090 with sppark, ICICLE, ZPrize or zkSNARK MSM/NTT returned no such figure. Results were V100 (cuZK), FPGA (OPTIMSM) and A40 contest numbers. This means "not found", not "does not exist". A 5090 figure would need a primary source or a measurement here.
- **Kernel versus end-to-end (ZKProphet abstract, raw/zkprophet-abs.md):** after MSM acceleration, NTT accounts for up to 90% of proof latency on GPUs; ZKP arithmetic runs on the 32-bit integer pipeline. Amdahl bound for MSM-only offload is 1/(1-f): with MSM at 60% of wall time, whole-proof speedup cannot exceed 2.5x regardless of kernel speed.
- **Nocy GPU proof server (third party, one published benchmark):** RTX 5090 vs AMD EPYC 7642, average of 10 runs: k17 4.4 s GPU vs 12.3 s CPU (2.80x); k18 8.8 s vs 24.6 s (2.80x). [raw/nocy-repo.md](https://github.com/Nocy-io/nocy-gpu-proof-server); [raw/nocy-forum.md](https://forum.midnight.network/t/open-source-drop-cuda-accelerated-midnight-proof-server-nocy-gpu-proof-server/972). The README does not state the timing boundary (warm or cold, key loading, witness preprocessing), the circuit, or the curve path. So this is a reported result, not a rigorously verified end-to-end measurement.
- **Nocy batching and caching:** README and forum post are silent on batching. Code audit (astra-3-6) shows per-commit MSM calls, resident SRS per host array, and a one-entry packed-polynomial cache. Absence in docs is not absence in code; the code findings are in astra-3-6.md.
- **Local midnight-ledger PK cache (audited, commit d89e0b63):** `transient-crypto/src/proofs.rs:250` sets `PK_CACHE_SIZE = 5`; lines 253-255 define a process-global `LruCache` keyed by `persistent_hash` of the compressed PK bytes; `try_cache` and `init` at lines 259-300. This caches the decompressed prover key within a process. It does not cache SRS batches or device buffers.

## Raw numbers
| Item | Value | Source / status |
|---|---|---|
| GPU VRAM | 32607 MiB total, 28989 MiB free | measured here, raw/host-snapshot.txt (2026-10-24T22:44:41Z) |
| Compute capability / PCIe | 12.0 / Gen5 x16 current | measured here |
| Host RAM / cores | 90 GB total, 74 GB available / 20 | measured here |
| k20 field vector | 2^20 x 32 B = 32 MiB | arithmetic |
| k20 affine G1 base (96 B/pt) | 96 MiB each | arithmetic (uncompressed affine; 2 x 48 B Fp) |
| k22 extended column | 2^22 x 32 B = 128 MiB | arithmetic |
| PCIe Gen5 x16 theory | 63.0 GB/s per direction | arithmetic from spec (upper bound) |
| Pinned transfer (Gen3, historical) | ~12 GB/s | CUDA best practices |
| Memory bandwidth | 1,792 GB/s | secondary only (PCGamesN) |
| ICICLE BLS12-381 2^20 MSM, 3090 Ti, p=1/2/4/8 | 25.5 / 20.3 / 18.1 / 17.2 ms | vendor doc, timing boundary unclear |
| Nocy k17 GPU / CPU | 4.4 s / 12.3 s (2.80x) | Nocy README, 10-run average, boundary unspecified |
| Nocy k18 GPU / CPU | 8.8 s / 24.6 s (2.80x) | same |
| 5090 MSM/NTT timing | not found in inspected sources | — |

## Caveats
- Pinned-memory cap: no numeric figure in NVIDIA docs. The 50% figure is a forum staff reply with caveats. Probe on this host.
- Managed-memory restriction is specific to managed memory; explicit host and device allocations and host CPU work can overlap. The per-device attribute (concurrentManagedAccess) was not queried.
- Stream and copy/compute overlap are standard CUDA features; their benefit through WSL2 is unmeasured.
- The 11–21 ms "5090 MSM" in node 02 is an extrapolation from the 3090 Ti vendor number with an assumed alpha of 1–2. No 5090 source supports it.
- Node 02's 24.7–44.6x kernel-level ratio assumes an unmeasured 0.5 s CPU reference. The only reported whole-proof result is 2.8x (Nocy). Plan around 2.8x until own phase timings exist.
- Node 03's whole-proof projection (1.4–3.3x at s=8) and node 02's 2.36x illustration rest on unmeasured MSM/NTT fractions.
- The Nocy build is a different ledger variant (6.1.0-alpha.6) from Moriarty's proofs 0.8.3 pin. Its results do not transfer directly.
- Web snippets are dated 2024–2026 and may be superseded. Captures are dated 2026-10-24.
- Node 08's device envelope (28 GiB device, 4+2x10+2 GiB for two heavy proofs) cannot be validated from these sources. Only the free-VRAM snapshot supports ~28 GiB.

## Sources (captures under helpers/haiku-10/raw/)
- host-snapshot.txt (nvidia-smi, free, nproc; timestamped)
- wsl-guide.md, blackwell.md, rtx5090-spec.md, pcgamesn-5090.md, pinned-forum.md
- icicle-precomp.md, sppark.md (Apache-2.0; BLS12-381 MSM and NTT templates; Volta+ on Linux and Windows), cuda-bestpractices.md
- zkprophet-abs.md, nocy-repo.md, nocy-forum.md
- local code: ~/repos/midnight-ledger/transient-crypto/src/proofs.rs:250-300 (commit d89e0b63)
