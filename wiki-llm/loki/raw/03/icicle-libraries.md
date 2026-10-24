ICICLE libraries | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[3.1.0](/3.1.0/icicle/libraries)

* [4.0.0](/)
* [3.9.0](/3.9.0)
* [3.8.0](/3.8.0/icicle/libraries)
* [3.7.0](/3.7.0/icicle/libraries)
* [3.6.0](/3.6.0/icicle/libraries)
* [3.5.0](/3.5.0/icicle/libraries)
* [3.4.0](/3.4.0/icicle/libraries)
* [3.3.0](/3.3.0/icicle/libraries)
* [3.2.0](/3.2.0/icicle/libraries)
* [3.1.0](/3.1.0/icicle/libraries)
* [3.0.0](/3.0.0/icicle/libraries)
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

* [Introduction](/3.1.0)
* [ICICLE](/3.1.0/icicle/overview)

  + [Getting started](/3.1.0/icicle/getting_started)
  + [Architecture overview](/3.1.0/icicle/arch_overview)
  + [ICICLE libraries](/3.1.0/icicle/libraries)
  + [Compute API](/3.1.0/icicle/primitives/overview)
  + [Migrate from ICICLE v2](/3.1.0/icicle/migrate_from_v2)
  + [Google Colab Instructions](/3.1.0/icicle/colab-instructions)
  + [ICICLE Provers](/3.1.0/icicle/integrations)
* [Ingonyama Grant program](/3.1.0/grants)
* [Contributor guide](/3.1.0/contributor-guide)
* Additional Resources

  + [YouTube](https://www.youtube.com/@ingo_ZK)
  + [Ingonyama Blog](https://www.ingonyama.com/blog)
  + [Ingopedia](https://www.ingonyama.com/ingopedia)
  + [GitHub](https://github.com/ingonyama-zk)

This is documentation for ICICLE Docs **3.1.0**, which is no longer actively maintained.

For up-to-date documentation, see the **[latest version](/)** (4.0.0).

* [ICICLE](/3.1.0/icicle/overview)
* ICICLE libraries
Version: 3.1.0

On this page

ICICLE libraries
================

ICICLE is composed of two main logical parts:

1. [**ICICLE device library**](#icicle-device)
2. [**ICICLE template core library**](#icicle-core)

ICICLE device[​](#icicle-device "Direct link to ICICLE device")
---------------------------------------------------------------

The ICICLE device library serves as an abstraction layer for interacting with various hardware devices. It provides a comprehensive interface for tasks such as setting the active device, querying device-specific information like free and total memory, determining the number of available devices, and managing memory allocation. Additionally, it offers functionality for copying data to and from devices, managing task queues (streams) for efficient device utilization, and abstracting the complexities of device management away from the user.

See programmers guide for more details. [C++](/3.1.0/icicle/programmers_guide/cpp#device-management), [Rust](/3.1.0/icicle/programmers_guide/rust#device-management), [Go](/3.1.0/icicle/programmers_guide/go)

ICICLE Core[​](#icicle-core "Direct link to ICICLE Core")
---------------------------------------------------------

ICICLE Core is a template library written in C++ that implements fundamental cryptographic operations, including field and curve arithmetic, as well as higher-level APIs such as MSM and NTT.

The Core can be [instantiated](/3.1.0/icicle/build_from_source) for different fields, curves, and other cryptographic components, allowing you to tailor it to your specific needs. You can link your application to one or more ICICLE libraries, depending on the requirements of your project. For example, you might only need the babybear library or a combination of babybear and a Merkle tree builder.

### Rust[​](#rust "Direct link to Rust")

Each library has a corresponding crate. See [programmers guide](/3.1.0/icicle/programmers_guide/general) for more details.

### Supported curves, fields and operations[​](#supported-curves-fields-and-operations "Direct link to Supported curves, fields and operations")

#### Supported curves and operations[​](#supported-curves-and-operations "Direct link to Supported curves and operations")

| Operation\Curve | [bn254](https://neuromancer.sk/std/bn/bn254) | [bls12-377](https://neuromancer.sk/std/bls/BLS12-377) | [bls12-381](https://neuromancer.sk/std/bls/BLS12-381) | [bw6-761](https://eprint.iacr.org/2020/351) | grumpkin |
| --- | --- | --- | --- | --- | --- |
| [MSM](/3.1.0/icicle/primitives/msm) | ✅ | ✅ | ✅ | ✅ | ✅ |
| G2 MSM | ✅ | ✅ | ✅ | ✅ | ❌ |
| [NTT](/3.1.0/icicle/primitives/ntt) | ✅ | ✅ | ✅ | ✅ | ❌ |
| ECNTT | ✅ | ✅ | ✅ | ✅ | ❌ |
| [Vector operations](/3.1.0/icicle/primitives/vec_ops) | ✅ | ✅ | ✅ | ✅ | ✅ |
| [Polynomials](/3.1.0/icicle/polynomials/overview) | ✅ | ✅ | ✅ | ✅ | ❌ |
| [Poseidon](/3.1.0/icicle/primitives/hash#poseidon) | ✅ | ✅ | ✅ | ✅ | ✅ |

#### Supported fields and operations[​](#supported-fields-and-operations "Direct link to Supported fields and operations")

| Operation\Field | [babybear](https://eprint.iacr.org/2023/824.pdf) | [Stark252](https://docs.starknet.io/documentation/architecture_and_concepts/Cryptography/p-value/) | m31 |
| --- | --- | --- | --- |
| [Vector operations](/3.1.0/icicle/primitives/vec_ops) | ✅ | ✅ | ✅ |
| [Polynomials](/3.1.0/icicle/polynomials/overview) | ✅ | ✅ | ❌ |
| [NTT](/3.1.0/icicle/primitives/ntt) | ✅ | ✅ | ❌ |
| Extension Field | ✅ | ❌ | ✅ |
| [Poseidon](/3.1.0/icicle/primitives/hash#poseidon) | ✅ | ✅ | ✅ |

### Misc[​](#misc "Direct link to Misc")

| Operation | Description |
| --- | --- |
| [Keccak](/3.1.0/icicle/primitives/hash#keccak-and-sha3) | supporting 256b and 512b digest |
| [SHA3](/3.1.0/icicle/primitives/hash#keccak-and-sha3) | supporting 256b and 512b digest |
| [Blake2s](/3.1.0/icicle/primitives/hash#blake2s) | digest is 256b |
| [Merkle-Tree](/3.1.0/icicle/primitives/merkle) | works with any combination of hash functions |

Backend[​](#backend "Direct link to Backend")
---------------------------------------------

Each backend must implement the device API interface.
Each backend may implement

* One or more ICICLE library. For example implement only bn254 curve.
* One or more APIs in this library. For example MSM only.

See [CUDA backend](/3.1.0/icicle/install_cuda_backend) and [Build Your Own Backend](/3.1.0/icicle/build_your_own_backend) for more info about implementing a backend.

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-3.1.0/icicle/libraries.md)

Last updated on **Aug 6, 2025**

[Previous

Multi-Device Support](/3.1.0/icicle/multi-device)[Next

ICICLE Compute APIs](/3.1.0/icicle/primitives/overview)

* [ICICLE device](#icicle-device)
* [ICICLE Core](#icicle-core)
  + [Rust](#rust)
  + [Supported curves, fields and operations](#supported-curves-fields-and-operations)
  + [Misc](#misc)
* [Backend](#backend)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.