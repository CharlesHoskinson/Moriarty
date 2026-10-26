# CUDA Programming Guide index
> Source: https://docs.nvidia.com/cuda/cuda-programming-guide/index.html
> Collected: 2026-10-26
> Published: Unknown

CUDA Programming Guide — CUDA Programming Guide

[Skip to main content](#main-content)

![country_code](https://www.nvidia.com/content/dam/1x1-00000000.png)
Back to top


`Ctrl`+`K`

[![CUDA Programming Guide - Home](_static/nvidia-logo-horiz-rgb-blk-for-screen.svg)![CUDA Programming Guide - Home](_static/nvidia-logo-horiz-rgb-wht-for-screen.svg)

CUDA Programming Guide](contents.html)

* v13.4.2 |
* [PDF](https://docs.nvidia.com/cuda/cuda-programming-guide/pdf/cuda-programming-guide.pdf)
* |
* [Archive](https://developer.nvidia.com/cuda-toolkit-archive)

Search
`Ctrl`+`K`

Search
`Ctrl`+`K`

[![CUDA Programming Guide - Home](_static/nvidia-logo-horiz-rgb-blk-for-screen.svg)![CUDA Programming Guide - Home](_static/nvidia-logo-horiz-rgb-wht-for-screen.svg)

CUDA Programming Guide](contents.html)

* v13.4.2 |
* [PDF](https://docs.nvidia.com/cuda/cuda-programming-guide/pdf/cuda-programming-guide.pdf)
* |
* [Archive](https://developer.nvidia.com/cuda-toolkit-archive)

Table of Contents

* [CUDA Programming Guide](#)
  + [1. Introduction to CUDA](part1.html)
    - [1.1. Introduction](01-introduction/introduction.html)
    - [1.2. Programming Model](01-introduction/programming-model.html)
    - [1.3. The CUDA platform](01-introduction/cuda-platform.html)
  + [2. Programming GPUs in CUDA](part2.html)
    - [2.1. Intro to CUDA C++](02-basics/intro-to-cuda-cpp.html)
    - [2.2. Intro to CUDA Python](02-basics/intro-to-cuda-python.html)
    - [2.3. Writing SIMT Kernels](02-basics/writing-cuda-kernels.html)
    - [2.4. Writing Tile Kernels](02-basics/writing-tile-kernels.html)
    - [2.5. Asynchronous Execution](02-basics/asynchronous-execution.html)
    - [2.6. Unified and System Memory](02-basics/understanding-memory.html)
    - [2.7. NVCC: The NVIDIA CUDA Compiler](02-basics/nvcc.html)
  + [3. Advanced CUDA](part3.html)
    - [3.1. Advanced CUDA APIs and Features](03-advanced/advanced-host-programming.html)
    - [3.2. Advanced Kernel Programming](03-advanced/advanced-kernel-programming.html)
    - [3.3. The CUDA Driver API](03-advanced/driver-api.html)
    - [3.4. Programming Systems with Multiple GPUs](03-advanced/multi-gpu-systems.html)
    - [3.5. A Tour of CUDA Features](03-advanced/feature-survey.html)
  + [4. CUDA Features](part4.html)
    - [4.1. Unified Memory](04-special-topics/unified-memory.html)
    - [4.2. CUDA Graphs](04-special-topics/cuda-graphs.html)
    - [4.3. Stream-Ordered Memory Allocator](04-special-topics/stream-ordered-memory-allocation.html)
    - [4.4. Cooperative Groups](04-special-topics/cooperative-groups.html)
    - [4.5. Programmatic Dependent Launch and Synchronization](04-special-topics/programmatic-dependent-launch.html)
    - [4.6. Green Contexts](04-special-topics/green-contexts.html)
    - [4.7. Locality Domains](04-special-topics/locality-domains.html)
    - [4.8. Lazy Loading](04-special-topics/lazy-loading.html)
    - [4.9. Error Log Management](04-special-topics/error-log-management.html)
    - [4.10. Asynchronous Barriers](04-special-topics/async-barriers.html)
    - [4.11. Pipelines](04-special-topics/pipelines.html)
    - [4.12. Asynchronous Data Copies](04-special-topics/async-copies.html)
    - [4.13. Work Stealing with Cluster Launch Control](04-special-topics/cluster-launch-control.html)
    - [4.14. L2 Cache Control](04-special-topics/l2-cache-control.html)
    - [4.15. Memory Synchronization Domains](04-special-topics/memory-sync-domains.html)
    - [4.16. Interprocess Communication](04-special-topics/inter-process-communication.html)
    - [4.17. Virtual Memory Management](04-special-topics/virtual-memory-management.html)
    - [4.18. Compute Fabric Transport](04-special-topics/compute-fabric-transport.html)
    - [4.19. Extended GPU Memory](04-special-topics/extended-gpu-memory.html)
    - [4.20. CUDA Dynamic Parallelism](04-special-topics/dynamic-parallelism.html)
    - [4.21. CUDA Interoperability with APIs](04-special-topics/graphics-interop.html)
    - [4.22. Driver Entry Point Access](04-special-topics/driver-entry-point-access.html)
  + [5. Technical Appendices](part5.html)
    - [5.1. Compute Capabilities](05-appendices/compute-capabilities.html)
    - [5.2. CUDA Environment Variables](05-appendices/environment-variables.html)
    - [5.3. C++ Language Support](05-appendices/cpp-language-support.html)
    - [5.4. C/C++ Language Extensions](05-appendices/cpp-language-extensions.html)
    - [5.5. Floating-Point Computation](05-appendices/mathematical-functions.html)
    - [5.6. Device-Callable APIs and Intrinsics](05-appendices/device-callable-apis.html)
    - [5.7. CUDA C++ Memory Model](05-appendices/cuda-cpp-memory-model.html)
    - [5.8. CUDA C++ Execution model](05-appendices/cuda-cpp-execution-model.html)
  + [6. Notices](notices.html)

* CUDA Programming Guide

[Is this page helpful?](https://surveys.hotjar.com/4904bf71-6484-47a7-83ff-4715cceabdb5)

CUDA Programming Guide[#](#cuda-programming-guide "Link to this heading")
=========================================================================

**CUDA and the CUDA Programming Guide**

CUDA is a parallel computing platform and programming model developed by NVIDIA that enables dramatic increases in computing performance by harnessing the power of the GPU. It allows developers to accelerate compute-intensive applications and is widely used in fields such as deep learning, scientific computing, and high-performance computing (HPC).

This CUDA Programming Guide is the official, comprehensive resource on the CUDA programming model and how to write code that executes on the GPU using the CUDA platform. This guide covers everything from the CUDA programming model and the CUDA platform to the details of language extensions and covers how to make use of specific hardware and software features. This guide provides a pathway for developers to learn CUDA if they are new, and also provides an essential resource for developers as they build applications using CUDA.

**Organization of This Guide**

Even for developers who primarily use libraries, frameworks, or DSLs, an understanding of the CUDA programming model and how GPUs execute code is valuable in knowing what is happening behind the layers of abstraction.
This guide starts with a chapter on the [CUDA programming model](01-introduction/programming-model.html#programming-model) outside of any specific programming language which is applicable to anyone interested in understanding how CUDA works, even non-developers.

The guide is broken down into five primary parts:

* Part 1: Introduction and Programming Model Abstract

  + A language agnostic overview of the CUDA programming model as well as a brief tour of the CUDA platform.
  + This section is meant to be read by anyone wanting to understand GPUs and the concepts of executing code on GPUs, even if they are not developers.
* Part 2: Programming GPUs in CUDA

  + The basics of programming GPUs in C++ and Python.
  + This section is meant to be read by anyone wanting to get started in GPU programming.
  + This section is meant to be instructional, not complete, and teaches the most important and common parts of CUDA programming, including some common performance considerations.
* Part 3: Advanced CUDA

  + Introduces some more advance features of CUDA that enable both fine-grained control and more opportunities to maximize performance, including the use of multiple GPUs in a single application.
  + This section concludes with a [tour of the features covered in part 4](03-advanced/feature-survey.html#tour-of-features) with a brief introduction to the purpose and function of each, sorted by when and why a developer may find each feature useful.
* Part 4: CUDA Features

  + This section contains complete coverage of specific CUDA features such as CUDA graphs, dynamic parallelism, interoperability with graphics APIs, and unified memory.
  + This section should be consulted when knowing the complete picture of a specific CUDA feature is needed. Where possible, care has been taken to introduce and motivate the features covered in this section in earlier sections.
* Part 5: Technical Appendices

  + The technical appendices provide some reference documentation on CUDA’s C++ high-level language support, hardware-specific specifications, and other technical specifications.
  + This section is meant as technical reference for specific description of syntax, semantics, and technical behavior of elements of CUDA.

Parts 1-3 provide a guided learning experience for developers new to CUDA, though they also provide insight and updated information useful for CUDA developers of any experience level.

Parts 4 and 5 provide a wealth of information about specific features and detailed topics, and are intended to provide a curated, well-organized reference for developers needing to know more details as they write CUDA applications.

[previous

Contents](contents.html "previous page")
[next

1. Introduction to CUDA](part1.html "next page")

[![NVIDIA](_static/nvidia-logo-horiz-rgb-1c-blk-for-screen.svg)![NVIDIA](_static/nvidia-logo-horiz-rgb-1c-wht-for-screen.svg)](https://www.nvidia.com)

[Privacy Policy](https://www.nvidia.com/en-us/about-nvidia/privacy-policy/)
|
[Your Privacy Choices](https://www.nvidia.com/en-us/about-nvidia/privacy-center/)
|
[Terms of Service](https://www.nvidia.com/en-us/about-nvidia/terms-of-service/)
|
[Accessibility](https://www.nvidia.com/en-us/about-nvidia/accessibility/)
|
[Corporate Policies](https://www.nvidia.com/en-us/about-nvidia/company-policies/)
|
[Product Security](https://www.nvidia.com/en-us/product-security/)
|
[Contact](https://www.nvidia.com/en-us/contact/)

Copyright © 2007-2026, NVIDIA Corporation & affiliates. All rights reserved.

Last updated on Sep 10, 2026.

![](https://docs.nvidia.com/akam/13/pixel_4b3b8a?a=dD1kNGRiZjMxMDJkZTExMDAwZGY5OTgwOGJjMGMwOTNmMTI0Mjk5MDAxJmpzPW9mZg==)