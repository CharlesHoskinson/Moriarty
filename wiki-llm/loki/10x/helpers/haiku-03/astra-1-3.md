# astra-1-3: Blackwell kernels and memory roofline

## Question
Audit RTX 5090 / BLS12-381 GPU kernel options (sppark, ICICLE v4, BLST integration, CUDA/Blackwell support, licenses). Find primary measured 2^20 BLS12-381 MSM and scalar NTT numbers, separating estimates. Compute the k20 memory floor per polynomial, bases/precompute, and lookup buffers; check N=2^22 and 2^23 fit in 32 GB.

## Answer
- **MEASURED (vendor, RTX 3090 Ti):** ICICLE 1.10.1 docs report BLS12-381 2^20 MSM = 25.5 ms (no precompute), 20.3 (p=2), 18.1 (p=4), 17.2 (p=8). Timing boundary not specified. Source: `helpers/haiku-03/raw/h3-icicle-precomp.md` (https://dev.ingonyama.com/1.10.1/icicle/golang-bindings/msm-pre-computation). The docs are for an old version; they are not an RTX 5090 number.
- **NOT FOUND:** No primary measured RTX 5090 BLS12-381 MSM figure, no sppark BLS12-381 MSM number, and no measured BLS12-381 scalar NTT 2^20 number from ICICLE or sppark. Web searches returned only unrelated hits (FPGA papers, Ethereum precompile specs). The 5090 MSM value in loki-02 is an ESTIMATE, not a measurement.
- **sppark:** Apache-2.0 (`h3-sppark.md`, LICENSE). Supports Volta+ NVIDIA GPUs on Linux/Windows; README says highest performance needs language-specific memory management work and that the Rust/Go interfaces are case-by-case. The README does not state sm_120/Blackwell support, so that remains unverified.
- **ICICLE v4:** GPU backends (CUDA, Metal) are closed-source and need a license. By default the backend uses a free Research & Development license via Ingonyama's license server. Production use requires contacting sales. Floating and node-locked licenses are supported. Source: `h3-icicle-gpu-backend.md` (https://dev.ingonyama.com/start/architecture/install_gpu_backend). v4 release notes (`h3-icicle-v4-release.md`) list an open-source ML-KEM CUDA implementation and no BLS12-381 speedup. This does not show that the MSM/NTT CUDA kernels are open source.
- **Blackwell/CUDA:** NVIDIA's Blackwell compatibility guide says apps built with CUDA 12.8 are compatible with Blackwell if they include PTX (or sm_100 cubin). Binaries with cubin only must be rebuilt. Source: `h3-cuda-blackwell.md` (https://docs.nvidia.com/cuda/blackwell-compatibility-guide/index.html). Loki-02's sm_120 build with PTX retained is the correct approach. A sm_100-only cubin will not cover RTX 5090 (sm_120).
- **RTX 5090 spec:** 32 GB GDDR7, PCIe Gen 5. Source: `h3-nvidia-5090.md` (https://www.nvidia.com/en-us/geforce/graphics-cards/50-series/rtx-5090/).
- **k20 memory floor (computed):** scalar vector 32 MiB; affine SRS base 96 MiB per table; projective 144 MiB. Two affine tables at p=4 = 768 MiB (matches loki-02). At p=14 the two tables = 2.625 GiB.
- **N=2^22 fit:** Loki-03 allocation (ASSUMED column inventory: 64 extended columns) totals 19 GiB, which fits 32 GB with about 13 GB headroom. Caveat: "32 GB" may be 29.8 GiB if decimal, so the headroom is still about 10.8 GiB.
- **N=2^23 fit:** the same allocation totals 28 GiB. This fits a 32 GiB nominal card with about 4 GiB headroom. If 32 GB is decimal (29.8 GiB), headroom is only about 1.8 GiB, which is too thin. Treat N=2^23 as conditional on the real column count.
- **Lookup sort/scan buffers:** not in any source I read. Column-inventory dependent. Left unestimated, with the open question in loki-03 unchanged.

## Raw numbers table
| Item | Value | Label | Source |
|---|---|---|---|
| BLS12-381 2^20 MSM, p=1 | 25.5 ms | MEASURED (vendor, 3090 Ti) | h3-icicle-precomp.md |
| BLS12-381 2^20 MSM, p=4 | 18.1 ms | MEASURED (vendor, 3090 Ti) | h3-icicle-precomp.md |
| BLS12-381 2^20 MSM, p=8 | 17.2 ms | MEASURED (vendor, 3090 Ti) | h3-icicle-precomp.md |
| BLS12-381 2^12 MSM, batch 2^10, p=4 | 117.8 ms | MEASURED (vendor, 3090 Ti) | h3-icicle-precomp.md |
| Scalar vector k20 | 32 MiB | COMPUTED | 2^20 x 32 B |
| Affine base table k20 | 96 MiB each | COMPUTED | 2^20 x 96 B |
| Two affine tables, p=4 | 768 MiB | COMPUTED | 2 x 2^20 x 4 x 96 B |
| Two affine tables, p=14 | 2.625 GiB | COMPUTED | 2 x 2^20 x 14 x 96 B |
| Extended poly N=2^22 | 128 MiB | COMPUTED | 2^22 x 32 B |
| Extended poly N=2^23 | 256 MiB | COMPUTED | 2^23 x 32 B |
| 64 ext columns, N=2^22 | 8 GiB | COMPUTED | 64 x 128 MiB |
| 64 ext columns, N=2^23 | 16 GiB | COMPUTED | 64 x 256 MiB |
| Total allocation, N=2^22 | 19 GiB | ESTIMATE (loki-03 table) | loki-03-gpu-ntt.md |
| Total allocation, N=2^23 | 28 GiB | ESTIMATE (loki-03 table) | loki-03-gpu-ntt.md |
| RTX 5090 memory | 32 GB GDDR7 | SPEC | h3-nvidia-5090.md |

## Caveats
- The 18.1 ms number is a 2023-era RTX 3090 Ti vendor figure with unspecified timing boundaries. Do not use it as a 5090 measurement.
- sm_120 support for sppark and ICICLE was not verified in this pass.
- Column counts, lookup buffers, and WSL pinned-memory limits are unmeasured.
- Loki-02 and loki-03 cite raw captures under `raw/02` and `raw/03` that I did not re-fetch. I checked only the MSM benchmark, licenses, Blackwell guide, and 5090 spec directly.
- Raw captures are in `/home/charles/repos/Moriarty-factory/wiki-llm/loki/10x/helpers/haiku-03/raw/h3-*.md`. The task asked for `astra-1/raw/haiku03-*.md`, but my write scope is the helpers/haiku-03 directory, so I wrote them there.
