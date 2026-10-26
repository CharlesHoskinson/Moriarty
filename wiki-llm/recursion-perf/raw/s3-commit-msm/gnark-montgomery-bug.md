# BLS12-381 ICICLE Montgomery chunking bug report
> Source: https://raw.githubusercontent.com/polytope-labs/gnark-cuda/main/ICICLE_MSM_MONTGOMERY_CHUNKING_BUG.md
> Collected: 2026-10-26
> Published: Unknown

# Bug: CUDA MSM returns wrong result with `are\_\*\_montgomery\_form=true` when the MSM is chunked
\*\*Component:\*\* open-icicle CUDA backend — MSM (`bls12\_381`, and by symmetry all curves)
\*\*Severity:\*\* correctness (silent wrong result, no error returned)
\*\*Found:\*\* 2026-06-26, RTX 5090 (sm\_120), CUDA 12.8.1, open-icicle built `-DCUDA\_BACKEND=local -DCURVE=bls12\_381`
## Summary
`bls12\_381\_msm` (and the templated `msm`) returns an \*\*incorrect point\*\* when \*\*both\*\*:
1. `config.are\_scalars\_montgomery\_form == true` and/or `config.are\_points\_montgomery\_form == true`, \*\*and\*\*
2. the MSM is large enough — relative to \*free\* GPU memory — to be split into multiple chunks by `multi\_chunked\_msm`.
With canonical-form inputs (`are\_\*\_montgomery\_form=false`) the same MSM is correct at every size. With Montgomery inputs the result is correct only while the MSM fits in a \*\*single chunk\*\*; once chunking kicks in, the result is wrong.
This is silent — `eIcicleError::SUCCESS` is returned.
## Why it hides
- icicle's own MSM tests (`tests/test\_curve\_api.cpp::MSM\_test`) use `logn=12` (~4096 points) on an otherwise-empty GPU, so they (a) use canonical inputs and (b) never trigger chunking. The Montgomery + chunked path is untested.
- Chunking is decided dynamically from \*\*free\*\* memory (`cuda\_msm.cuh::get\_min\_nof\_chunks` → `compute\_required\_memory` → `cudaMemGetInfo`), so the same `(N, inputs)` can be single-chunk on a clean GPU and multi-chunk under memory pressure. The bug only appears in the latter.
## Suspected root cause
In `backend/cuda/src/msm/cuda\_msm.cuh::multi\_chunked\_msm` the per-chunk Montgomery state is tracked by:
```cpp
bool internal\_are\_points\_montgomery\_form =
(batch\_same\_points && i) || config.precompute\_factor > 1
? false
: config.are\_points\_montgomery\_form; // lines ~1319-1323
```
and each chunk's `bucket\_method\_msm` converts its slice from Montgomery in place (`upload\_scalars`/`upload\_points`, lines ~503-547). The scalar side passes `config.are\_scalars\_montgomery\_form` unconditionally to every chunk (line ~1349). The interaction across chunks (which slices get converted, in which buffer, and whether the conversion is double-applied or skipped) appears to be the defect — non-batched multi-chunk MSM with Montgomery inputs yields a wrong sum.
## Minimal reproduction
`test\_icicle\_mont.cu` (pure icicle C-API; no external framework). For each N it computes the \*\*same\*\* MSM two ways on the CUDA device and compares (via `to\_affine` + memcmp):
- (1) canonical inputs, `are\_\*\_montgomery\_form=false`
- (2) the same inputs converted to Montgomery (`convert\_montgomery`, `is\_into=true`), `are\_\*\_montgomery\_form=true`
A `cudaMalloc` "hog" leaves ~3 GB free to force chunking of the 8M case.
```
== forced chunking: hogged 29.9 GB, 3.2 GB free ==
N=2^10 … 2^20 canonical-MSM == montgomery-MSM ? YES
N=2^23 (8M) canonical-MSM == montgomery-MSM ? NO <-- MISMATCH
```
Without the hog (clean GPU, 8M fits in one chunk) all sizes match — confirming the trigger is chunking, not size per se.
Build:
```
nvcc -std=c++17 -O2 -arch=sm\_120 test\_icicle\_mont.cu -o test\_icicle\_mont \
-I/include -I/backend/cuda/include \
-L/lib -licicle\_field\_bls12\_381 -licicle\_curve\_bls12\_381 -licicle\_device \
--expt-relaxed-constexpr
ICICLE\_BACKEND\_INSTALL\_DIR=/lib/backend ./test\_icicle\_mont
```
(Repro source: `test/test\_icicle\_mont.cu` in this repo.)
## Workaround (what we do)
Convert inputs Montgomery→canonical \*\*explicitly\*\* on device once (`bls12\_381\_{scalar,affine}\_convert\_montgomery`, `is\_into=false`), then call MSM with `are\_\*\_montgomery\_form=false`. This avoids icicle's internal per-chunk conversion entirely and is correct at all sizes regardless of chunking. (It's also cacheable for reused bases.)