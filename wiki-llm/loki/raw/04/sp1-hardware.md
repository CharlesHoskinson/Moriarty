Hardware Acceleration | Succinct Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

[![Succinct Logo](/img/logo-black.svg)![Succinct Logo](/img/logo-black.svg)

**Succinct Docs**](/)[SP1](/docs/sp1/introduction)

[Prover Network](#)

* [Overview](/docs/protocol/introduction)
* [Provers](/docs/provers/introduction)

[Solutions](#)

* [OP Succinct](https://succinctlabs.github.io/op-succinct/)
* [SP1 Helios](https://succinctlabs.github.io/sp1-helios)

[Legal / Disclaimer](/legal/disclaimer)[GitHub](https://github.com/succinctlabs)

Search

* [Overview](/docs/sp1/introduction)

  + [Introduction](/docs/sp1/introduction)
  + [What is a zkVM?](/docs/sp1/what-is-a-zkvm)
  + [Usecases](/docs/sp1/usecases)
* [Getting Started](/docs/sp1/getting-started/install)

  + [Installation](/docs/sp1/getting-started/install)
  + [Quickstart](/docs/sp1/getting-started/quickstart)
  + [Hardware Requirements](/docs/sp1/getting-started/hardware-requirements)
  + [Recommended Workflow](/docs/sp1/getting-started/recommended-workflow)
  + [Migrating from V5 to V6](/docs/sp1/getting-started/migration)
* [Writing Programs](/docs/sp1/writing-programs/basics)

  + [Basics](/docs/sp1/writing-programs/basics)
  + [Setup](/docs/sp1/writing-programs/setup)
  + [Compiling Programs](/docs/sp1/writing-programs/compiling)
  + [Proof Aggregation](/docs/sp1/writing-programs/proof-aggregation)
* [Proving](/docs/sp1/generating-proofs/basics)

  + [Basics](/docs/sp1/generating-proofs/basics)
  + [Proof Types](/docs/sp1/generating-proofs/proof-types)
  + [Hardware Acceleration](/docs/sp1/generating-proofs/hardware-acceleration)
  + [Advanced](/docs/sp1/generating-proofs/advanced)
  + [Offchain Verification](/docs/sp1/generating-proofs/off-chain-verification)
* [Optimization & Profiling](/docs/sp1/optimizing-programs/basics)

  + [Basics](/docs/sp1/optimizing-programs/basics)
  + [Precompiles](/docs/sp1/optimizing-programs/precompiles)
  + [Precompile Specification](/docs/sp1/optimizing-programs/precompile-specification)
  + [Prover Gas](/docs/sp1/optimizing-programs/prover-gas)
  + [Cycle Tracking](/docs/sp1/optimizing-programs/cycle-tracking)
  + [Profiling](/docs/sp1/optimizing-programs/profiling)
* [Prover Network](/docs/sp1/prover-network/quickstart)

  + [Quickstart](/docs/sp1/prover-network/quickstart)
  + [Advanced Usage](/docs/sp1/prover-network/advanced-usage)
  + [Reserved Capacity](/docs/sp1/prover-network/reserved-capacity)
  + [Migrate to Mainnet](/docs/sp1/prover-network/migration-guide)
  + [Troubleshooting](/docs/sp1/prover-network/troubleshooting)
  + [FAQ](/docs/sp1/prover-network/faq)
  + [TEE 2FA](/docs/sp1/prover-network/tee)
  + [TEE Private Proving](/docs/sp1/prover-network/private-proving)
* [On-Chain Verification](/docs/sp1/verification/getting-started)

  + [Setup](/docs/sp1/verification/getting-started)
  + [Contract Addresses](/docs/sp1/verification/contract-addresses)
  + [Solidity Verifier](/docs/sp1/verification/solidity-sdk)
* [Troubleshooting & CI](/docs/sp1/developers/common-issues)

  + [Common Issues](/docs/sp1/developers/common-issues)
  + [Usage in CI](/docs/sp1/developers/usage-in-ci)
  + [Upgrade Guides](/docs/sp1/developers/upgrades)
* [SP1 Proof System](/docs/sp1/hypercube/)

  + [Introduction](/docs/sp1/hypercube/)
  + [Shard Proofs](/docs/sp1/hypercube/shards)
  + [Main Trace Commitment](/docs/sp1/hypercube/trace)
  + [LogUp GKR](/docs/sp1/hypercube/logupgkr)
  + [Zerocheck](/docs/sp1/hypercube/zerocheck)
  + [Jagged PCS](/docs/sp1/hypercube/jagged)
  + [Dense PCS](/docs/sp1/hypercube/pcs)
  + [Global Memory Argument](/docs/sp1/hypercube/global)
  + [Recursion](/docs/sp1/hypercube/recursion)
* [Security](/docs/sp1/security/security-model)

  + [Security Model](/docs/sp1/security/security-model)
  + [Legal / Disclaimer](/legal/disclaimer)
  + [Safe Usage of SP1 Precompiles](/docs/sp1/security/safe-precompile-usage)
  + [Security Policy](https://github.com/succinctlabs/sp1/security/policy)
  + [Security Advisories](https://github.com/succinctlabs/sp1/security/advisories)
  + [Audit Reports](https://github.com/succinctlabs/sp1/tree/main/audits)
  + [Bug Bounty](https://code4rena.com/bounties/succinct)
* [Resources](https://explorer.mainnet.succinct.xyz)

  + [Explorer (Mainnet)](https://explorer.mainnet.succinct.xyz)
  + [Explorer (Reserved)](https://explorer.reserved.succinct.xyz)
  + [Staking](https://staking.mainnet.succinct.xyz)

* Proving
* Hardware Acceleration

On this page

Hardware Acceleration
=====================

SP1 supports hardware acceleration through [CUDA](https://en.wikipedia.org/wiki/CUDA) on Nvidia GPUs and [AVX256/AVX512](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions) on Intel x86 CPUs.

GPU Acceleration[​](#gpu-acceleration "Direct link to GPU Acceleration")
------------------------------------------------------------------------

SP1 supports GPU acceleration with [CUDA](https://en.wikipedia.org/wiki/CUDA), which can provide dramatically better latency and cost performance compared to using the CPU prover, even with AVX acceleration.

### Software Requirements[​](#software-requirements "Direct link to Software Requirements")

Please make sure you have the following before using the CUDA prover:

* Linux x86\_64
* The [CUDA 12 runtime](https://developer.nvidia.com/cuda-downloads) and a compatible NVIDIA driver

The CUDA prover runs as a native `sp1-gpu-server` binary that the SDK downloads and manages automatically. Docker and the NVIDIA Container Toolkit are no longer required.

### Hardware Requirements[​](#hardware-requirements "Direct link to Hardware Requirements")

* **CPU**: We recommend having at least 4 CPU cores with 16GB of RAM available to fully utilize the GPU.
* **GPU**: 24GB or more VRAM.

warning

SP1 supports GPUs with Compute Capability 8.0 or higher (e.g., A100, RTX 30/40 series, H100). You can find the compute capability for your GPU [on this table in the NVIDIA documentation](https://developer.nvidia.com/cuda-gpus).

### Usage[​](#usage "Direct link to Usage")

To use the CUDA prover, you have two options:

1. Use `ProverClient::from_env` to build the client and set the `SP1_PROVER` environment variable to `cuda`.
2. Use `ProverClient::builder().cuda().build()` to build the client.

Then, use your standard methods on the `ProverClient` to generate proofs.

### Recommended Workflow[​](#recommended-workflow "Direct link to Recommended Workflow")

By default, the CUDA prover uses the `0th` GPU. To target a different device, pass the device id to the builder: `ProverClient::builder().cuda().with_device_id(1).build()`.

In general, the best practice is to keep an instance of a `ProverClient` in an `Arc` as they have initialization overhead.

CPU Acceleration[​](#cpu-acceleration "Direct link to CPU Acceleration")
------------------------------------------------------------------------

SP1 can achieve significant CPU performance improvements through [AVX vector instruction extensions](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions).
Support for both AVX256 and AVX512 acceleration on Intel x86 CPUs is provided via [Plonky3](https://github.com/Plonky3/Plonky3).
For optimal performance, we recommend using AVX512 acceleration when your hardware supports it.

### Checking for AVX Support[​](#checking-for-avx-support "Direct link to Checking for AVX Support")

To verify if your CPU supports AVX, run:

`grep avx /proc/cpuinfo`

Look for the flags `avx2` and `avx512` in the output.

### Enabling AVX256 Acceleration[​](#enabling-avx256-acceleration "Direct link to Enabling AVX256 Acceleration")

To enable AVX256 acceleration, set the `RUSTFLAGS` environment variable:

```
RUSTFLAGS="-C target-cpu=native" cargo run --release
```

### Enabling AVX512 Acceleration[​](#enabling-avx512-acceleration "Direct link to Enabling AVX512 Acceleration")

For even better performance with AVX512, use:

```
RUSTFLAGS="-C target-cpu=native -C target-feature=+avx512f" cargo run --release
```

Note that the `+avx512f` flag is required to enable AVX512 acceleration.

[Previous

Proof Types](/docs/sp1/generating-proofs/proof-types)[Next

Advanced](/docs/sp1/generating-proofs/advanced)

* [GPU Acceleration](#gpu-acceleration)
  + [Software Requirements](#software-requirements)
  + [Hardware Requirements](#hardware-requirements)
  + [Usage](#usage)
  + [Recommended Workflow](#recommended-workflow)
* [CPU Acceleration](#cpu-acceleration)
  + [Checking for AVX Support](#checking-for-avx-support)
  + [Enabling AVX256 Acceleration](#enabling-avx256-acceleration)
  + [Enabling AVX512 Acceleration](#enabling-avx512-acceleration)