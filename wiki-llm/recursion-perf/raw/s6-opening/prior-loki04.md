# Earlier GPU Halo2 study
> Source: file:///home/charl/code/Moriarty-perf/wiki-llm/loki/nodes/loki-04-halo2-gpu-precedents.md
> Collected: 2026-10-26
> Published: 2026-10-24

---
title: "GPU Halo2 precedents: preserve Midnight, replace the arithmetic backend"
project: loki
node: 04
author: gpt-6.1-sol
date: 2026-10-24
tags: [halo2, gpu, cuda, kzg, bls12-381, precedents, compatibility]
sources:
  - https://github.com/midnightntwrk/midnight-zk/pull/157
  - https://github.com/Nocy-io/nocy-gpu-proof-server
  - https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2
  - https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover
  - https://raw.githubusercontent.com/ingonyama-zk/halo2/main/halo2_proofs/src/icicle.rs
  - https://api.github.com/repos/ingonyama-zk/halo2
  - https://github.com/ingonyama-zk/open-icicle
  - https://raw.githubusercontent.com/ingonyama-zk/open-icicle/main/LICENSE
  - https://dev.ingonyama.com/start/architecture/install_gpu_backend
  - https://github.com/axiom-crypto/halo2-gpu
  - https://api.github.com/repos/axiom-crypto/halo2-gpu/git/trees/main?recursive=1
  - https://github.com/axiom-crypto/halo2-lib/releases
  - https://github.com/scroll-tech/zkvm-prover/pull/260
  - https://github.com/scroll-tech/zkevm-circuits/issues/654
  - https://scroll.io/blog/zkEVM
  - https://github.com/privacy-ethereum/halo2
  - https://github.com/taikoxyz/raiko
  - https://github.com/DelphinusLab/halo2-gpu-specific
  - https://github.com/matter-labs/era-bellman-cuda
  - https://raw.githubusercontent.com/matter-labs/era-bellman-cuda/main/src/ff_config.cuh
  - https://github.com/matter-labs/era-boojum
  - https://github.com/matter-labs/era-boojum-cuda
  - https://api.github.com/repos/matter-labs/zksync-crypto-gpu/git/trees/main?recursive=1
  - https://github.com/filecoin-project/bellperson
  - https://github.com/filecoin-project/ec-gpu
  - https://api.github.com/repos/filecoin-project/bellperson
  - https://api.github.com/repos/filecoin-project/ec-gpu
  - https://cysic.xyz/
  - https://github.com/cysic-labs/ZPrize-23-Prize1
  - https://github.com/cysic-labs/ZPrize-23-Prize1/blob/main/Prize%201A/README.md
  - https://raw.githubusercontent.com/cysic-labs/ZPrize-23-Prize1/main/Prize%201B/LICENSE
  - https://api.github.com/repos/cysic-labs/ZPrize-23-Prize1
  - https://www.irreducible.com/posts/accelerating-polygon-zkevm
  - https://www.irreducible.com/posts/irreducible-shutting-down
  - https://github.com/IrreducibleOSS/binius
  - https://github.com/snarkify/zprize-2023-prize1b
  - https://raw.githubusercontent.com/snarkify/zprize-2023-prize1b/main/submission/LICENSE
  - https://github.com/lurk-lab/bellperson
  - https://github.com/argumentcomputer/sphinx
  - https://api.github.com/repos/argumentcomputer/sphinx
  - https://threadreaderapp.com/thread/1792525776379850940.html
  - https://github.com/succinctlabs/sp1-gpu
  - https://github.com/succinctlabs/sp1
  - https://docs.succinct.xyz/docs/sp1/generating-proofs/hardware-acceleration
  - https://api.github.com/repos/succinctlabs/sp1/git/trees/dev?recursive=1
  - https://github.com/risc0/risc0
  - https://github.com/risc0/risc0/blob/main/risc0/zkp/src/hal/cuda.rs
  - https://dev.risczero.com/api/zkvm/benchmarks
  - https://eprint.iacr.org/2022/1321
  - https://github.com/Tao-Lu-123/cuZK
  - https://github.com/SJTU-Liquid/halo2
  - https://github.com/tie-pilot-qxw/halo2
  - https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/native-plonk/cuda/src/field.cuh
  - https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/native-plonk/native-plonk/tests/gpu_cross.rs
  - https://api.github.com/repos/Nocy-io/nocy-gpu-proof-server
  - https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/sources/midnight-ledger/cuda-prover-backend/cuda-prover-backend/src/lib.rs
  - https://github.com/supranational/sppark
  - https://github.com/supranational/sppark/blob/main/poc/msm-cuda/src/lib.rs
  - https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/msm
  - https://raw.githubusercontent.com/axiom-crypto/halo2-gpu/main/halo2_proofs/tests/gpu_keygen_byte_identity.rs
  - https://github.com/midnightntwrk/midnight-zk/tree/gpu-integration
  - https://github.com/midnightntwrk/midnight-ledger/tree/gpu-integration
---

## Question

**PROPOSAL:** Which existing GPU Halo2/PLONKish implementations offer reusable kernels, integration patterns, and credible performance evidence for Sherlock, while preserving the exact proof system, transcript, SRS, and VK accepted by Midnight ledger-9?

## Summary

- **PROPOSAL:** Keep `midnight-proofs` 0.8.3 as the protocol implementation; start with a narrow **sppark BLS12-381 MSM/NTT adapter**, then add resident polynomial evaluation. Use Axiom and ICICLE as architectural precedents.
- **SOURCE:** Midnight already has public GPU experiments: upstream POC PR #157 and Nocy's RTX 5090 proof server. Neither establishes compatibility with today's Sherlock recursion. [PR](https://github.com/midnightntwrk/midnight-zk/pull/157) ([capture](../raw/04/midnight-pr157.md)); [Nocy](https://github.com/Nocy-io/nocy-gpu-proof-server) ([capture](../raw/04/nocy-repo.md)).
- **SOURCE:** ICICLE-Halo2 reports up to 25×; Snarkify reports 7.20× and 3.81× on two different circuit shapes. These involve much more than MSM replacement. [ICICLE](https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2) ([capture](../raw/04/icicle-halo2-v2.md)); [Snarkify](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover) ([capture](../raw/04/snarkify-docs.md)).
- **CODE:** Midnight's BLS12-381 commitment field is named `Fq`; its engine associates `Fr = Fq`. BN254 and small-field zkVM kernels require porting. `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-curves-0.3.1/src/bls12_381/mod.rs:30`.
- **ESTIMATE:** An illustrative 500-second proof becomes 313 seconds with 45% MSM work accelerated 8× plus 10 seconds overhead: `500×(0.55+0.45/8)+10=313.125`, or **1.60×**. Large kernel claims do not imply equivalent end-to-end gains.

## Findings

### What must remain compatible

**CODE:** Moriarty pins proofs 0.8.3, circuits 7.2.4, and stdlib 2.3.5 at `/home/charles/repos/Moriarty-factory/crates/mori-backend/Cargo.toml:14`. Fenrir's separate recl9 manifest pins proofs **0.8.2**, at `/home/charles/repos/projectfenrir/recursion/stagenet-strawman/recl9/Cargo.toml:25`. Its leaf/parent transcripts are Poseidon and terminal transcript is Blake2b, at `recl9/src/main.rs:6` and `:14` under that same directory. **PROPOSAL:** Treat cross-version equivalence as an independent obligation.

**CODE:** The ledger uses BLS12-381 parameters and Blake2b transcripts, then calls `midnight_zk_stdlib::verify` on its stored VK and statement: `/home/charles/repos/midnight-ledger/transient-crypto/src/proofs.rs:64`, `:81`, `:545`. The node's strict transaction path calls `well_formed`: `/home/charles/repos/midnight-node/ledger/src/versions/common/mod.rs:956`. **PROPOSAL:** Require actual ledger acceptance in addition to host verification.

**CODE:** Upstream checkout `31435e9` is not API-identical to installed 0.8.3. Upstream has labeled `commit_many` and `KZGMultiCommitment`, at `/home/charles/repos/midnight-zk/proofs/src/poly/commitment.rs:47` and `/home/charles/repos/midnight-zk/proofs/src/poly/kzg/mod.rs:69`. Installed 0.8.3 has separate `commit`/`commit_lagrange` returning G1, at `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:61`. The local Halo2 checkout also retains the separate commitment methods, at `/home/charles/repos/halo2/src/poly/commitment.rs:39`. **PROPOSAL:** A GPU patch must target the deployed dependency cohort explicitly.

### Survey of implementations

**PROPOSAL:** Read each row as an evidence boundary. “Unestablished” means this acquisition did not establish that property; it does not mean the capability cannot exist. Dates below concern captured repositories, not support commitments.

| Family | Integration and kernels | Reported result, curves, license, maintenance |
| --- | --- | --- |
| **SOURCE — ICICLE-Halo2 / EZKL** | EZKL-derived fork replaces MSM/NTT and adds vector arithmetic, inversion, gate/permutation/lookup evaluation; witness generation remains outside the acceleration effort. | Up to **25×**, RTX 4080/i9-13900K, example circuits. Fork is Apache-2.0 OR MIT; shipped adapter imports **BN254**, not BLS12-381. Last recorded push September 2025. [Report](https://www.ingonyama.com/post/2-fast-2-furious-icicle-halo2-v2) ([capture](../raw/04/icicle-halo2-v2.md)); [adapter](https://raw.githubusercontent.com/ingonyama-zk/halo2/main/halo2_proofs/src/icicle.rs) ([capture](../raw/04/icicle-adapter.md)); [metadata](https://api.github.com/repos/ingonyama-zk/halo2) ([capture](../raw/04/icicle-metadata.md)). |
| **SOURCE — ICICLE core / open-icicle** | Runtime/device dispatch with Rust bindings; newer trimmed distribution exposes MSM, NTT, ECNTT and supporting operations for **BN254 and BLS12-381**. | No independent Midnight speedup. `open-icicle` includes CUDA source and a root MIT license, **but its README still excludes GPU backends from MIT**. Standard backend docs require a separate license. **OPEN QUESTION:** resolve the contradictory scope before reuse. [Distribution](https://github.com/ingonyama-zk/open-icicle) ([capture](../raw/04/open-icicle.md)); [license](https://raw.githubusercontent.com/ingonyama-zk/open-icicle/main/LICENSE) ([capture](../raw/04/open-icicle-license.md)); [backend terms](https://dev.ingonyama.com/start/architecture/install_gpu_backend) ([capture](../raw/04/icicle-backend-license.md)). |
| **SOURCE — Axiom** | `halo2-axiom-gpu`: CUDA MSM, NTT, grand products, permutation/lookup quotients, polynomial operations and multiopen; kernels derive partly from sppark. | **BN254** specialization; Apache-2.0 OR MIT. CUDA ≥12.8. Releases wire GPU v1.0.1 into halo2-lib v0.5.5. No controlled CPU/GPU ratio in captured README. **PROPOSAL:** Current integration candidate, requiring a BLS port. [Repo](https://github.com/axiom-crypto/halo2-gpu) ([capture](../raw/04/axiom-gpu.md)); [kernel inventory](https://api.github.com/repos/axiom-crypto/halo2-gpu/git/trees/main?recursive=1) ([capture](../raw/04/axiom-tree.md)); [releases](https://github.com/axiom-crypto/halo2-lib/releases) ([capture](../raw/04/axiom-releases.md)). |
| **SOURCE — Scroll** | Historical issue #654 explicitly used Halo2 GPU proving. Current zkVM integration routes `halo2-gpu → openvm-sdk/halo2-gpu → snark-verifier-sdk/cuda`, using Axiom's backend. | July 2026 merged PR reports **~5.7-second SNARK proving**, ~23.5 GiB peak, on a **2×4090 setup**; no CPU ratio. BN254/Axiom backend is MIT/Apache. Historical circuit repo archived April 2025. The early Scroll blog's **5–10× versus Filecoin** is a separate historical claim, not this benchmark. [Current PR](https://github.com/scroll-tech/zkvm-prover/pull/260) ([capture](../raw/04/scroll-pr260.md)); [old issue](https://github.com/scroll-tech/zkevm-circuits/issues/654) ([capture](../raw/04/scroll-654.md)); [blog](https://scroll.io/blog/zkEVM) ([capture](../raw/04/scroll-blog.md)). |
| **SOURCE — PSE forks** | PSE swapped IPA for KZG and split frontend/backend; its README directs substantial new work to Axiom. | Apache-2.0 OR MIT; **maintenance mode since January 2025**. No upstream GPU speedup or BLS GPU backend established by its README. Curve generality in CPU code is not GPU curve support. [Repo](https://github.com/privacy-ethereum/halo2) ([capture](../raw/04/pse-halo2.md)). |
| **SOURCE — Taiko / Raiko** | Raiko's current multiproof interface enables **RISC Zero CUDA/Metal** and SP1, rather than presenting a maintained dedicated Halo2 CUDA backend. | Apache-2.0; README calls previous zkEVM circuits deprecated. No comparable GPU ratio or BLS12-381 KZG support established. These are different proof engines. [Repo](https://github.com/taikoxyz/raiko) ([capture](../raw/04/raiko.md)). |
| **SOURCE — Delphinus / zkWasm lineage** | Public `halo2-gpu-specific` fork is an additional Halo2 GPU precedent. | Captured README labels it beta and licenses it under **BOSL**, not permissive MIT/Apache. **OPEN QUESTION:** current kernel coverage, speed ratio, GPU curve specialization and support status need direct audit; the generic README does not establish them. [Repo](https://github.com/DelphinusLab/halo2-gpu-specific) ([capture](../raw/04/delphinus.md)). |
| **SOURCE — zkSync era-bellman-cuda** | CMake-built GPU cryptographic library; field configuration fixes eight 32-bit limbs and the **BN254** moduli; source includes NTT. | Apache-2.0 OR MIT. No CPU/GPU ratio in captured README. This is zkSync's separate prover arithmetic, not a Halo2 backend. [Repo](https://github.com/matter-labs/era-bellman-cuda) ([capture](../raw/04/era-bellman-cuda.md)); [field](https://raw.githubusercontent.com/matter-labs/era-bellman-cuda/main/src/ff_config.cuh) ([capture](../raw/04/era-bellman-curve.md)). |
| **SOURCE — zkSync Boojum / Shivini** | Boojum uses the **Goldilocks** field and FRI; GPU tree includes NTT, Goldilocks extensions, Poseidon2, plus BN254-related components. | Apache-2.0 OR MIT. `era-boojum-cuda` is archived and redirects to **zksync-crypto-gpu**, containing gpu-prover, Shivini, FFLONK and wrapper-prover. No isolated acceleration ratio established. BLS12-381 KZG is not its native target. [Boojum](https://github.com/matter-labs/era-boojum) ([capture](../raw/04/boojum-readme.md)); [migration](https://github.com/matter-labs/era-boojum-cuda) ([capture](../raw/04/era-boojum-cuda.md)); [current tree](https://api.github.com/repos/matter-labs/zksync-crypto-gpu/git/trees/main?recursive=1) ([capture](../raw/04/zksync-gpu-tree.md)). |
| **SOURCE — Filecoin bellperson / ec-gpu** | **BLS12-381**, blstrs/blst; Groth16 GPU FFT/MSM. ec-gpu generates CUDA/OpenCL field/curve kernels and exposes batched FFT. | Both Apache-2.0 OR MIT. Bellperson last push October 2026; ec-gpu July 2025. No applicable Halo2 end-to-end ratio reported in their READMEs. **PROPOSAL:** Reuse representation/fallback patterns; importing Groth16 changes the proof system. [Bellperson](https://github.com/filecoin-project/bellperson) ([capture](../raw/04/bellperson.md)); [ec-gpu](https://github.com/filecoin-project/ec-gpu) ([capture](../raw/04/ec-gpu.md)); [activity](https://api.github.com/repos/filecoin-project/bellperson) ([capture](../raw/04/bellperson-metadata.md)); [ec-gpu activity](https://api.github.com/repos/filecoin-project/ec-gpu) ([capture](../raw/04/ec-gpu-metadata.md)). |
| **SOURCE — Cysic** | Website advertises purpose-built hardware/GPU infrastructure. Public ZPrize repository specifies BLS12-377/381 MSM competition and a PLONKish end-to-end task. | No reproducible general Halo2 ratio or reusable complete BLS Halo2 SDK established. Public repository last push September 2024; Prize1B license is **MPL-2.0**, while Prize1A rules require MIT/Apache submissions. These are competition artifacts, not evidence that the commercial SDK has those licenses. [Site](https://cysic.xyz/) ([fetch capture](../raw/04/cysic-website.md)); [repository](https://github.com/cysic-labs/ZPrize-23-Prize1) ([capture](../raw/04/cysic-zprize.md)); [rules](https://github.com/cysic-labs/ZPrize-23-Prize1/blob/main/Prize%201A/README.md) ([capture](../raw/04/cysic-prize1a.md)); [license](https://raw.githubusercontent.com/cysic-labs/ZPrize-23-Prize1/main/Prize%201B/LICENSE) ([capture](../raw/04/cysic-prize-license.md)); [activity](https://api.github.com/repos/cysic-labs/ZPrize-23-Prize1) ([capture](../raw/04/cysic-metadata.md)). |
| **SOURCE — Ulvetanna / Irreducible** | Polygon zkEVM accelerator uses **two FPGAs for Goldilocks NTT/LDE and Poseidon leaf hashing**, not CUDA Halo2. Binius uses binary-field towers. | Reported **84 seconds**, ~2× versus the same 64-thread host; no BLS12-381 Halo2 GPU ratio. **Company shutdown announced November 12, 2025**. Binius archived; Binius64 relicensed MIT/Apache. Hardware implementation license unestablished. [Benchmark](https://www.irreducible.com/posts/accelerating-polygon-zkevm) ([capture](../raw/04/irreducible-polygon.md)); [shutdown](https://www.irreducible.com/posts/irreducible-shutting-down) ([capture](../raw/04/irreducible-shutdown.md)); [Binius](https://github.com/IrreducibleOSS/binius) ([capture](../raw/04/irreducible.md)). |
| **SOURCE — Snarkify cuSnark** | Rust bindings into CUDA/C++ MSM, NTT, quotient evaluation, KZG multiopen, inversion and permutations. | On **4×3090 + EPYC 7702**, aggregation `217.76→30.24 s` (**7.20×**); wide k20 `1762.75→463.13 s` (**3.81×**). Captured docs do not establish cuSnark license or BLS12-381 support for this Halo2 integration. **OPEN QUESTION:** obtain a pinned maintained implementation; do not infer its license from Sirius. [Docs](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover) ([capture](../raw/04/snarkify-docs.md)). |
| **SOURCE — Snarkify ZPrize submission** | Specialized PLONK prover rebuilt in C/CUDA, retaining Rust transcript; removes unused features including lookups. BLS12-381 microbenchmarks include MSM, NTT, inversions and quotient. | Reports **~900×** versus baseline: four proofs/2.16 s on A6000 Ada versus ~490 s each on CPU. Submission license is MPL-2.0; artifact, not general Halo2. Its own scope explains why this number cannot forecast Sherlock. [Submission](https://github.com/snarkify/zprize-2023-prize1b) ([capture](../raw/04/snarkify-zprize.md)); [license](https://raw.githubusercontent.com/snarkify/zprize-2023-prize1b/main/submission/LICENSE) ([capture](../raw/04/snarkify-license.md)). |
| **SOURCE — Lurk / Sphinx** | Lurk's bellperson fork accelerates Groth16 FFT/MSM with CUDA/OpenCL, including BLS12-381. Sphinx is an SP1-derived zkVM using Plonky3 components. | Bellperson lineage and Sphinx expose MIT/Apache licensing. No Sphinx-specific GPU acceleration ratio or BLS KZG backend established; Sphinx last push February 2025. **PROPOSAL:** Check GPU feature inheritance explicitly. [Bellperson fork](https://github.com/lurk-lab/bellperson) ([capture](../raw/04/lurk-bellperson.md)); [Sphinx](https://github.com/argumentcomputer/sphinx) ([capture](../raw/04/sphinx.md)); [activity](https://api.github.com/repos/argumentcomputer/sphinx) ([capture](../raw/04/sphinx-metadata.md)). |
| **SOURCE — SP1** | Native CUDA prover server managed through SDK; its own zkVM proof stack. Current source contains **KoalaBear recursion and BN254 wrapper** code, rather than a BLS12-381 KZG backend. | Succinct's **2024 launch thread** claimed **10× cost/latency**, 20× in some portions, without a controlled configuration in the captured text. Old GPU repository archived February 2026; development moved into SP1, MIT/Apache. Current docs require ≥24 GB VRAM. [Author's mirrored thread](https://threadreaderapp.com/thread/1792525776379850940.html) ([fetch capture](../raw/04/sp1-launch.md)); [migration](https://github.com/succinctlabs/sp1-gpu) ([capture](../raw/04/sp1-gpu.md)); [SP1](https://github.com/succinctlabs/sp1) ([capture](../raw/04/sp1.md)); [current API](https://docs.succinct.xyz/docs/sp1/generating-proofs/hardware-acceleration) ([capture](../raw/04/sp1-hardware.md)); [source inventory](https://api.github.com/repos/succinctlabs/sp1/git/trees/dev?recursive=1) ([capture](../raw/04/sp1-cuda-tree.md)). |
| **SOURCE — RISC Zero** | **BabyBear** CUDA HAL with batched NTT, hashing and circuit-specific kernels; CUDA/Metal feature selection. Recursive STARK/Groth16 system, not BLS12-381 Halo2 KZG. | MIT/Apache; maintained development repository and versioned benchmark instructions. No comparable CPU/GPU ratio in the captured current instructions. **PROPOSAL:** Reuse HAL ownership ideas while keeping Midnight proof encoding. [Repo](https://github.com/risc0/risc0) ([capture](../raw/04/risc0.md)); [HAL](https://github.com/risc0/risc0/blob/main/risc0/zkp/src/hal/cuda.rs) ([capture](../raw/04/risc0-hal.md)); [benchmarks](https://dev.risczero.com/api/zkvm/benchmarks) ([capture](../raw/04/risc0-bench.md)). |
| **SOURCE — cuZK** | Groth16 MSM, matrix-vector work, NTT, transfer/computation overlap; **BLS12-381**, ALT_BN128 and MNT4 examples. | Paper reports **2.65–4.86×** standard-benchmark gains over prior GPU implementations; repository k20 BLS example reports **4.45→1.68 s** versus Bellperson. MIT/Apache research artifact, CUDA11.5/V100 assumptions. Kernel research, not a compatible Halo2 replacement. [Paper](https://eprint.iacr.org/2022/1321) ([capture](../raw/04/cuzk-paper.md)); [code](https://github.com/Tao-Lu-123/cuZK) ([capture](../raw/04/cuzk-code.md)). |
| **SOURCE — FluxZK / Impeller** | Additional recent Halo2 graph-compilation fork: `plonk::jit::create_proof_gpu`, circuit-identifier compilation cache, GPU runtime, xmake and CUDA12.8. | Halo2 wrapper MIT/Apache; third-party kernel licenses need separate checking. Repository describes SC2026/ASPLOS2027 artifacts; no performance ratio or BLS specialization established here. **OPEN QUESTION:** audit before considering as a second-stage graph compiler. [Integration](https://github.com/SJTU-Liquid/halo2) ([capture](../raw/04/impeller.md)); [predecessor](https://github.com/tie-pilot-qxw/halo2) ([capture](../raw/04/fluxzk.md)). |

### The two closest Midnight precedents

**SOURCE:** Upstream **PR #157** describes ICICLE4 dispatch, BLST fallback, batched commitments, lookup/vanishing changes and tests. It was explicitly a do-not-merge POC, closed **December 8, 2025**; the author intended continuation in `gpu-integration` and ledger proof-server integration. This establishes experimentation, not an official release or present roadmap. [PR](https://github.com/midnightntwrk/midnight-zk/pull/157) ([capture](../raw/04/midnight-pr157.md)).

**SOURCE:** **Nocy** calls its server a variant of **6.1.0-alpha.6** and reports 10-run averages: k17 **12.3→4.4 s**, k18 **24.6→8.8 s**, RTX5090 versus EPYC7642. Its source includes native PLONK, MSM/FFT, BLS scalar/base moduli, and an opt-in CPU-verifier/GPU-proof test. These are unusually relevant precedents, but the test uses an older `compact_std_lib` interface and a tiny assertion circuit. [Repo](https://github.com/Nocy-io/nocy-gpu-proof-server) ([capture](../raw/04/nocy-repo.md)); [field code](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/native-plonk/cuda/src/field.cuh) ([capture](../raw/04/nocy-field.md)); [cross-test](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/native-plonk/native-plonk/tests/gpu_cross.rs) ([capture](../raw/04/nocy-cross.md)).

**SOURCE:** Nocy metadata records last push **January 19, 2026**, no detected root license. Its backend accepts serialized parameters, PK, packed witness and RNG seed. [Metadata](https://api.github.com/repos/Nocy-io/nocy-gpu-proof-server) ([capture](../raw/04/nocy-metadata.md)); [API](https://raw.githubusercontent.com/Nocy-io/nocy-gpu-proof-server/main/sources/midnight-ledger/cuda-prover-backend/cuda-prover-backend/src/lib.rs) ([capture](../raw/04/nocy-backend.md)). **OPEN QUESTION:** new-kernel licensing, Poseidon/full-width recursion, trash arguments, committed instances, current serialization and ledger-9 acceptance are unestablished.

## Proposed design

**SOURCE:** Sppark is Apache-2.0, advertises active development, and supplies BLS12-381 MSM/NTT templates. Its Rust MSM example directly accepts `blst_p1_affine` and `blst_scalar`, an attractive boundary beside Midnight's BLST implementation. It documents unfinished integration tutorials and FFI caveats. [Library](https://github.com/supranational/sppark) ([capture](../raw/04/sppark.md)); [Rust boundary](https://github.com/supranational/sppark/blob/main/poc/msm-cuda/src/lib.rs) ([capture](../raw/04/sppark-msm-rust.md)).

**PROPOSAL:** Choose **Midnight + sppark**, not a wholesale foreign fork. Compare ec-gpu as the fallback kernel candidate. Reconsider `open-icicle` after its licensing scope is resolved. Audit Nocy's design independently before copying new files; use Axiom's residency and differential-key tests as implementation references.

**PROPOSAL:** Add optional modules `accel/{mod,cpu,cuda,repr,session}.rs`. Define `ProverArithmetic::{msm_g1, ntt_fq, batch_invert_fq, evaluate_graph}` returning `Result`, plus `CudaSession` owning streams, buffers and parameter caches. CPU remains the default public developer path; CUDA installation is optional. Bind caches to SRS digest, basis, domain, curve and device; graph caches also include circuit/VK identity.

**CODE:** The installed hook points are `KZGCommitmentScheme::{commit,commit_lagrange}` and `msm_specific`, at `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-proofs-0.8.3/src/poly/kzg/mod.rs:74` and `src/poly/kzg/msm.rs:163`. The local MSM path already forces BLST Pippenger for BLS12-381, at `src/poly/kzg/msm.rs:176`; **PROPOSAL:** Benchmark against that actual baseline. **CODE:** The `MSM` trait at `src/utils/arithmetic.rs:320` is an accumulator interface, not the sole prover acceleration switch.

**PROPOSAL:** Begin with commitment dispatch, preserving the existing PCS type and multiopen protocol. Normalize/cache projective SRS bases once in the 0.8.3 adapter. Keep small MSMs on CPU; measure crossover rather than copying another fork's threshold. Later batch commitments inside each transcript round, returning them in their original order before squeezing the next challenge. Keep transcript state and blinding RNG on the host. Treat GPU errors transactionally: expose no partial commitment result to the transcript. Fall back before absorption, or restart the proof from a saved RNG/transcript checkpoint; never continue using a mixture of partial outputs.

**CODE:** NTT hooks are `EvaluationDomain::{lagrange_to_coeff,coeff_to_lagrange,coeff_to_extended,extended_to_coeff}`, at the same registry root's `src/poly/domain.rs:224`. They include inverse scaling and coset-power distribution. Quotient work enters `Evaluator::evaluate_numerator`, including `trash_challenge` and trashcan polynomials, at `src/plonk/evaluation.rs:285`. These Midnight-specific arguments preclude blindly replacing the evaluator with a foreign Halo2 implementation.

**PROPOSAL:** Data flow: existing circuit synthesis → canonical field/point bridge → device columns → NTT/commitments → host transcript challenge → resident permutation/lookup/trash/quotient evaluation → opening arithmetic → unchanged proof writer. Add GPU graph evaluation only after arithmetic equivalence. Port the exact operations and rotations, retaining all constraints.

**CODE:** Stdlib's public `prove` constructs a MidnightCircuit and delegates to `BlstPLONK::prove`; its verifier fixes committed-instance handling: `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-zk-stdlib-2.3.5/src/lib.rs:1984`, `:2017`. Circuits' recursive verifier is KZG-based at `/home/charles/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/midnight-circuits-7.2.4/src/verifier/mod.rs:14`. Preserve those interfaces and gadgets.

**PROPOSAL:** This design changes **no proof format**. Substituting BN254, Boojum, Binius, SP1, RISC Zero, Groth16 or another opening protocol changes the verifier relation/format. A foreign engine would need a new verifier inside a Midnight-compatible wrapper, regenerated circuit keys and recursion integration; direct ledger-verifier replacement also needs network-level adoption. Those costs are outside this acceleration proposal.

## Expected speedup

**ESTIMATE:** Use the owner's >500-second attempt only as context, not a completed benchmark. For an illustrative `T=500 s`, assume CPU fractions MSM45%, NTT20%, evaluator15%, other20%, acceleration factors `8,10,5`, and 10 seconds conversion/transfer overhead. Then `Tgpu=225/8+100/10+75/5+100+10=163.125 s`, **3.07×**. If only MSM moves, the summary's **1.60×** applies. These fractions require profiling in [[loki-01-prover-profile]].

**ESTIMATE:** A 6× result requires much broader coverage: with 90% accelerated 20× and 2% overhead, `S=1/(0.10+0.90/20+0.02)=6.06`. This is a conditional target, not a forecast from another curve or GPU.

**ESTIMATE:** For the reported 120-second VK build, assuming 70% accelerable work at 8× and overhead 2%: `120×(0.30+0.70/8+0.02)=48.9 s`, **2.45×**. Persisting identical keys can remove repeated keygen without changing the protocol.

**ESTIMATE:** At k20, `2^20×32 bytes=32 MiB` per scalar column; 100 resident columns use **3.125 GiB**, or **12.5 GiB** with 4× extension. Four such extended sets require **50 GiB**, exceeding 32 GB before scratch. The 12 GiB host RSS does not determine VRAM usage; admission needs the actual column/extension inventory.

**SOURCE:** Snarkify's MSM-only wide-circuit table changes `1690.05→1575.82 s`, while its full backend achieves much larger gains; its quotient remains essentially unchanged under MSM-only offload. [MSM breakdown](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/msm) ([capture](../raw/04/snarkify-msm.md)). **ESTIMATE:** `1690.05/1575.82=1.072×`, illustrating the coverage problem.

## Risks and soundness concerns

**PROPOSAL:** Validate field modulus, Montgomery radix, endian order, scalar canonicalization, affine/projective coordinates, infinity encoding and subgroup handling explicitly. Do not copy the BN254 adapter's unsafe casts into Midnight. A different reduction schedule may change projective coordinates while representing the same point; compare canonical encodings.

**PROPOSAL:** Preserve SRS/VK bytes, transcript absorption order, challenge width, lookup/permutation/trash semantics, domain roots and blinding. GPU execution must not reuse randomness across proofs. Reject corrupted GPU outputs; verify with the reference verifier before submission. GPU memory/telemetry must not retain private witness material across unrelated jobs.

**CODE:** CUDA Rust supplies host contexts/streams/buffers but describes itself as an early research project: `/home/charles/repos/nvidia-cuda-rust/README.md:23`, `:33`. **PROPOSAL:** Use a small audited FFI boundary first; adopting a new device-language compiler is a separate risk. Validate CUDA13.4/Blackwell by actual execution; old architecture defaults do not establish compatibility.

## Validation plan

**PROPOSAL:** These are future predicates; no builds or benchmarks were run for this research.

1. **PROPOSAL — P0/cohort:** Freeze source revisions and hashes of circuits, stdlib, proofs, features, SRS and keys. Record the local BLST patch. Establish an independent unmodified verifier executable for each intended version.
2. **PROPOSAL — P1/arithmetic:** Differential MSM against BLST for empty, singleton, all-zero, sparse, full-width, repeated-base and infinity inputs; cover threshold boundaries and k20 lengths. Compare canonical points. Differential NTT/IFFT over exact roots, inverse scaling, both layouts, cosets and extended domains; require field-for-field equality.
3. **PROPOSAL — P2/protocol:** Under identical deterministic RNG, require identical commitments, evaluations, transcript challenges and proof bytes when the randomness schedule is unchanged. If scheduling consumes randomness differently, require unchanged proof parsing, reference verification and equivalent statement binding, and document the intentional difference. Never relax key identity accidentally.
4. **SOURCE:** Axiom already tests CPU/GPU VK and PK byte identity using one shared serialized SRS, selector compression both ways, lookups and permutation constraints. [Test](https://raw.githubusercontent.com/axiom-crypto/halo2-gpu/main/halo2_proofs/tests/gpu_keygen_byte_identity.rs) ([capture](../raw/04/axiom-gpu-test.md)). **PROPOSAL:** Adapt this test pattern to Midnight's exact cohort.
5. **PROPOSAL — P3/recursion:** Run honest and corrupted child → parent → terminal cases, both Poseidon and Blake2b, including committed instances and full-width challenges. Reject modified instances, wrong VK/SRS, bad accumulator bindings, malformed/truncated proofs and trailing bytes. Test 0.8.3→0.8.2 compatibility separately if that crossing remains required.
6. **PROPOSAL — P4/acceptance:** Feed GPU-produced terminal proof bytes through the existing ledger-9 transaction path and contract on stagenet with the existing verifier/VK. Record accepted transaction and state transition plus rejected mutations; host self-verification alone does not pass P4.
7. **PROPOSAL — P5/performance:** Through `scripts/doctorwatson heavy`, compare identical circuits/SRS/key caches at fixed CPU parallelism. Separate cold/warm keygen, witness, conversion, transfer, kernels and prove-to-ledger latency; report median/p95, RSS, VRAM and failures. Begin small, then k20; do not extrapolate from a synthetic lookup circuit.

## Open questions

- **OPEN QUESTION:** Can Nocy's new CUDA code be reused legally, and can its old serialized interface support current custom recursive circuits rather than only ZKIR proof-server workloads?
- **OPEN QUESTION:** Where did upstream PR #157's continuation land? **SOURCE:** Public `gpu-integration` URLs returned unavailable pages during acquisition: [ZK](https://github.com/midnightntwrk/midnight-zk/tree/gpu-integration) ([capture](../raw/04/midnight-gpu-branch.md)); [ledger](https://github.com/midnightntwrk/midnight-ledger/tree/gpu-integration) ([capture](../raw/04/midnight-ledger-gpu.md)). **OPEN QUESTION:** Internal continuation remains unknown.
- **OPEN QUESTION:** Does `open-icicle`'s root MIT grant cover its newly published CUDA sources despite the README exclusion, and which commit defines the supported API?
- **OPEN QUESTION:** Which Sherlock phase actually dominates after the local BLST change, and how many extended columns coexist at k20?
- **OPEN QUESTION:** Which public Cysic, Delphinus or Snarkify artifacts support the complete current Midnight argument set? Uncaptured private SDK claims and global survey completeness remain unverified.

## Links

**PROPOSAL:** Coordinate kernel choices with [[loki-02-gpu-msm]] and [[loki-03-gpu-ntt]]; profile with [[loki-01-prover-profile]]. Feed memory admission into [[loki-08-recursion-tree-scheduling]] and representation choices into [[loki-09-cpu-witness-field]].

**PROPOSAL:** Keep protocol changes in [[loki-05-recursion-accumulation]], [[loki-06-recursive-circuit-cost]] and [[loki-07-folding-alternatives]] distinct from arithmetic acceleration; make [[loki-10-integration-verification]] own the ledger-acceptance predicates.
