# GPU phase-1 (G0) probes, 2026-11-07/08

These are the source files of the probes cited in `../gpu.md` §0 and in task 2.10a. They were copied from session scratchpads; no binaries are included.

| Directory | What it showed |
|---|---|
| `nvcc-smoke/k.cu` | nvcc 13.4.3 `-arch=sm_121` kernel: the `__umul64hi` result over 2^20 elements matched the CPU |
| `rust-buildrs-smoke/` | `build.rs` runs nvcc into a static library, and Rust launches the kernel on plain `Vec` memory (zero-copy): 65,536 products with 8x32-bit limbs, 0 mismatches |
| `gpuprobe/` | device properties (`props.cu`) and integer throughput (`imul.cu`): u32 mad at 7.36 T/s, u64 mul+mulhi at 0.54 T pairs/s |
| `blstbench/` | CPU baselines: BLST 2^20 MSM and `best_fft` |

These crates are not workspace members. Build them only in a scratch copy, outside the repo, under `scripts/doctorwatson heavy --gpu`.
