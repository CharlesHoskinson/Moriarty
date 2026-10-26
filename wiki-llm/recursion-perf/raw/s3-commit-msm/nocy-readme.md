# Nocy GPU proof server README
> Source: https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/README.md
> Collected: 2026-10-26
> Published: Unknown

# Nocy Proof Server
> \*\*Version:\*\* Variant of 6.1.0-alpha.6
> \*\*Benchmarked for:\*\* NVIDIA GeForce RTX 5090
A high-performance proof server for generating zero-knowledge proofs.
## Build
Prerequisites: CUDA toolkit, CMake, Rust toolchain.
Build CUDA kernels:
```bash
cmake -S cuda -B cuda/build -DCMAKE\_BUILD\_TYPE=Release
cmake --build cuda/build -j
```
Build native-plonk CUDA:
```bash
cmake -S native-plonk/cuda -B native-plonk/cuda/build -DCMAKE\_BUILD\_TYPE=Release
cmake --build native-plonk/cuda/build -j
```
Build the server:
```bash
cargo build -p midnight-proof-server --release --manifest-path sources/midnight-ledger/Cargo.toml
```
## Run
```bash
export LD\_LIBRARY\_PATH=native-plonk/cuda/build:cuda/build
export MIDNIGHT\_PP=params
./sources/midnight-ledger/target/release/midnight-proof-server --port 6300
```
## Routes
| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/prove` | Generate a zero-knowledge proof |
| POST | `/check` | Validate preimages |
| GET | `/health` | Health check endpoint |
## Benchmarks
Comparison between NVIDIA GeForce RTX 5090 (GPU) and AMD EPYC 7642 (CPU).
Results averaged over 10 runs.
| Circuit Size | GPU (RTX 5090) | CPU (EPYC 7642) |
|--------------|----------------|-----------------|
| k17 | 4.4s | 12.3s |
| k18 | 8.8s | 24.6s |
\*\*Performance:\*\* 2.8× faster compared to CPU-only proving.