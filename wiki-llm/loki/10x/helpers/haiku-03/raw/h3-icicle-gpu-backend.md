GPU Backends | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[4.0.0](/start/architecture/install_gpu_backend)

* [4.0.0](/start/architecture/install_gpu_backend)
* [3.9.0](/3.9.0/start/architecture/install_gpu_backend)
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

* Architecture
* GPU Backends
Version: 4.0.0

On this page

GPU Backends
============

Overview[​](#overview "Direct link to Overview")
------------------------------------------------

ICICLE v3 supports high-performance GPU backends, including **CUDA** (for NVIDIA GPUs) and **Metal** (for Apple Silicon GPUs), to accelerate cryptographic computations. These backends provide optimized libraries for various cryptographic primitives such as MSM, NTT, and elliptic curve operations, offering significant performance improvements across supported hardware platforms.

Installation[​](#installation "Direct link to Installation")
------------------------------------------------------------

GPU backends are closed-source components that require a valid license. [To install and configure GPU backends in ICICLE, see here](/setup#installing-and-using-icicle)

Licensing[​](#licensing "Direct link to Licensing")
---------------------------------------------------

note

By default, ICICLE connects to Ingonyama’s backend license server and runs with a **free Research & Development license**, enabling GPU backends (CUDA and Metal) for development and testing purposes—no setup required.
For **production use**, please contact [sales@ingonyama.com](mailto:sales@ingonyama.com).

For additional support, email [support@ingonyama.com](mailto:support@ingonyama.com).

Each backend (CUDA or Metal) requires a valid license to function. ICICLE supports two types of licenses:

1. **Floating license**:  
   You host a license server (provided as a binary). This license supports a limited number of concurrent GPUs (N), which can be used across multiple machines. Each process using a GPU decrements N by 1. When the process ends or crashes, the license is released.
2. **Node-locked license**:  
   The license is bound to a specific machine. The backend will only operate on the licensed device.

### Setting the License[​](#setting-the-license "Direct link to Setting the License")

To specify the license server or license file:

```
export ICICLE_LICENSE=port@ip            # For license server  
export ICICLE_LICENSE=/path/to/license   # For node-locked license
```

For further assistance, contact our support team for assistance [support@ingonyama.com](mailto:support@ingonyama.com).

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-4.0.0/start/architecture/install_gpu_backend.md)

Last updated on **Aug 6, 2025**

[Previous

Build Your Own Backend](/start/architecture/build_your_own_backend)[Next

ICICLE libraries](/start/architecture/libraries)

* [Overview](#overview)
* [Installation](#installation)
* [Licensing](#licensing)
  + [Setting the License](#setting-the-license)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.