Frequently Asked Questions | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[4.0.0](/start/integration-&-support/faq_and_troubleshooting)

* [4.0.0](/start/integration-&-support/faq_and_troubleshooting)
* [3.9.0](/3.9.0/start/integration-&-support/faq_and_troubleshooting)
* [3.8.0](/3.8.0)
* [3.7.0](/3.7.0)
* [3.6.0](/3.6.0)
* [3.5.0](/3.5.0)
* [3.4.0](/3.4.0)
* [3.3.0](/3.3.0)
* [3.2.0](/3.2.0)
* [3.1.0](/3.1.0)
* [3.0.0](/3.0.0)
* [2.8.0](/2.8.0)
* [1.10.1](/1.10.1)

[Community](#)

* [Discord](https://discord.gg/6vYrE7waPj)
* [LinkedIn](https://www.linkedin.com/company/ingonyama)
* [X/Twitter](https://x.com/Ingo_zk)
* [YouTube](https://www.youtube.com/@ingo_ZK)
* [Mailing List](https://wkf.ms/3LKCbdj)

[Leave Feedback](https://forms.monday.com/forms/7b51e0bdad766b71da8869704c301472?r=use1)

[![Ingonyama logo](/img/Ingologo.svg)](https://ingonyama.com "Visit Ingonyama")

Search

* [Introduction](/)

  + [Overview](/)
  + [Setup Guide](/setup)
* [Architecture](/start/architecture/arch_overview)

  + [Architecture Overview](/start/architecture/arch_overview)
  + [Build Your Own Backend](/start/architecture/build_your_own_backend)
  + [GPU Backends](/start/architecture/install_gpu_backend)
  + [ICICLE libraries](/start/architecture/libraries)
  + [Multi-Device with ICICLE](/start/architecture/multi-device)
* [Programmer's Guide](/start/programmers_guide/general)

  + [ICICLE Programmer's Guide](/start/programmers_guide/general)
  + [Build ICICLE from source](/start/programmers_guide/build_from_source)
  + [ICICLE C++ Usage Guide](/start/programmers_guide/cpp)
  + [ICICLE Golang Usage Guide](/start/programmers_guide/go)
  + [ICICLE Rust Usage Guide](/start/programmers_guide/rust)
* [Integration & Support](/start/integration-&-support/contributor-guide)

  + [Contributor's Guide](/start/integration-&-support/contributor-guide)
  + [Frequently Asked Questions](/start/integration-&-support/faq_and_troubleshooting)
  + [Ingonyama Grants Program 2.0](/start/integration-&-support/grants)
  + [ICICLE Integrated Provers](/start/integration-&-support/integrations)
  + [Run ICICLE on Google Colab](/start/integration-&-support/colab-instructions)
  + [Migration from ICICLE v2 to v3](/start/integration-&-support/migrate_from_v2)
  + [Migration from ICICLE v3 to v4](/start/integration-&-support/migrate_from_v3)
  + [Benchmarks](/start/integration-&-support/benchmarks)

* Integration & Support
* Frequently Asked Questions
Version: 4.0.0

Frequently Asked Questions
==========================

What is ICICLE?▾

ICICLE is a cryptography library designed to accelerate algorithms and protocols—starting with Zero-knowledge proofs—across diverse compute backends, including CPUs, GPUs, Apple Silicon, ZPU™ and more.

What problem does ICICLE solve?▾

It significantly accelerates core cryptographic operations like MSM, NTT, modular multiplication, and hashing on any backend.

Is ICICLE open-source?▾

ICICLE is partially open-source:

* The core CUDA backend is source available under Ingonyama’s custom ICICLE-SA license.
* The frontend and language bindings (Rust & Go) are open-source under the MIT license.
* Full backend license details: [ICICLE Licensing](https://dev.ingonyama.com/start/architecture/install_gpu_backend#licensing).

Which cryptographic primitives are supported?▾

ICICLE supports a wide range of field, curve, vector, polynomial, MSM, NTT, FRI, and hashing operations across native, extension, and RNS fields. Learn more [here](https://dev.ingonyama.com/start/architecture/libraries).

Which provers can integrate with ICICLE?▾

ICICLE is designed to integrate with virtually any prover. We've already integrated it into Gnark, Halo2 and also provide a native Groth16 prover in our icicle-snark repository.

What hardware is required to run ICICLE & which GPU models are recommended?▾

For the CPU backend, any 64-bit CPU is fully supported. We've also successfully tested ICICLE on 32-bit CPUs, such as those found on Raspberry Pi, though with limited performance.

For the CUDA backend, ICICLE runs on any NVIDIA GPU from the RTX 2060 and newer. We currently build for compute capabilities 7.5, 8.0, 8.6, and 8.9. For more details on GPU compatibility, see this [reference](https://arnon.dk/matching-sm-architectures-arch-and-gencode-for-various-nvidia-cards/).

Are non-Nvidia GPUs supported?▾

Metal (Apple GPUs) is partially supported but still experimental. Vulkan support is in progress and will expand compatibility in the future.

What programming languages does ICICLE support?▾

The core library is written in CUDA C++. Bindings exist for Rust and Go.

Does ICICLE support multi-GPU setups?▾

Yes, ICICLE supports multi-GPU configurations. Learn more [here](https://dev.ingonyama.com/api/rust-bindings/multi-gpu).

How much speedup can I expect using ICICLE?▾

Check out the Benchmark Dashboard to explore real-world results.

Who is using ICICLE today?▾

ICICLE is used by leading projects and research teams across cryptography and infrastructure. See the full list [here](https://dev.ingonyama.com/#ecosystem).

Where can I get help if I encounter issues?▾

GitHub issues on the ICICLE repo are the primary support channel. Community discussions often happen in our Discord Server’s [ICICLE channel](https://discord.gg/EVVXTdt6DF).

Can I contribute to ICICLE?▾

Yes — contributions to ICICLE are welcome! You can submit pull requests or open issues on our [public GitHub repository](https://github.com/ingonyama-zk/icicle). If you're working on research or development aligned with ICICLE, consider applying to our [Ingonyama Research Grant Program 2025](https://www.ingonyama.com/post/ingonyama-research-grant-2025).

What is the maximum MSM size tested?▾

MSM sizes up to 230 have been tested successfully on the 3090Ti. Note that this applies only to certain backends. For example, Metal may not yet support streaming, which could limit maximum MSM size on that backend.

What class of GPU is required to run ICICLE?▾

Nvidia CUDA GPUs only. Some mobile/workstation GPUs (e.g. Quadro P520) may hit compilation issues that require patching CMake flags.

Why does my benchmark report zero time per iteration?▾

Likely due to LLVM optimization eliminating empty loops. Use `black_box` to prevent optimization during benchmarks.

Is omega equal to root\_of\_unity?▾

Yes, but ICICLE stores precomputed omegas for multiple powers. The specific omega used depends on the NTT size.

Is there documentation for ntt\_template\_kernel\_shared\_rev?▾

No formal docs. But the butterfly operation is here: [ntt.cuh L153](https://github.com/ingonyama-zk/icicle/blob/main/icicle/appUtils/ntt/ntt.cuh#L153).

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-4.0.0/start/integration-&-support/faq_and_troubleshooting.md)

Last updated on **Aug 6, 2025**

[Previous

Contributor's Guide](/start/integration-&-support/contributor-guide)[Next

Ingonyama Grants Program 2.0](/start/integration-&-support/grants)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.