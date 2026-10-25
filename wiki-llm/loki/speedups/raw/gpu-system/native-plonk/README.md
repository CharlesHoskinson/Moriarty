# Native CUDA PLONK Prover (Standalone)

This module is a standalone, native CUDA PLONK prover implementation.
It targets proof byte compatibility with the existing Rust prover while
keeping all code and build artifacts isolated from the ledger workspace.

Structure
- `cuda`: CUDA/C++ implementation and C ABI (`libmidnight_native_plonk.so`).
- `native-plonk`: Rust wrapper + tests (loads the C ABI at runtime).

Build (local)

```
cmake -S cuda -B cuda/build -DCMAKE_BUILD_TYPE=Release
cmake --build cuda/build --config Release
```

```
cargo test --manifest-path native-plonk/Cargo.toml
```

Docker build (CUDA LTS)

```
docker build -f native-plonk/Dockerfile.cuda .
```

Runtime controls
- `MIDNIGHT_CUDA_TESTS=1` enables GPU-only test cases.
- `MIDNIGHT_CUDA_DEVICE=0` selects the CUDA device.

Notes
- The C ABI follows `docs/cuda-prover-design.md` (raw proof bytes only).
- Proof serialization and transcript ordering must match `midnight-proofs`.
