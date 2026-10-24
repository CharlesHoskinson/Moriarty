5.1. Compute Capabilities — CUDA Programming Guide

[Skip to main content](#main-content)

![country_code](https://www.nvidia.com/content/dam/1x1-00000000.png)
Back to top


`Ctrl`+`K`

[![CUDA Programming Guide - Home](../_static/nvidia-logo-horiz-rgb-blk-for-screen.svg)![CUDA Programming Guide - Home](../_static/nvidia-logo-horiz-rgb-wht-for-screen.svg)

CUDA Programming Guide](../contents.html)

* v13.4.2 |
* [PDF](https://docs.nvidia.com/cuda/cuda-programming-guide/pdf/cuda-programming-guide.pdf)
* |
* [Archive](https://developer.nvidia.com/cuda-toolkit-archive)

Search
`Ctrl`+`K`

Search
`Ctrl`+`K`

[![CUDA Programming Guide - Home](../_static/nvidia-logo-horiz-rgb-blk-for-screen.svg)![CUDA Programming Guide - Home](../_static/nvidia-logo-horiz-rgb-wht-for-screen.svg)

CUDA Programming Guide](../contents.html)

* v13.4.2 |
* [PDF](https://docs.nvidia.com/cuda/cuda-programming-guide/pdf/cuda-programming-guide.pdf)
* |
* [Archive](https://developer.nvidia.com/cuda-toolkit-archive)

Table of Contents

* [CUDA Programming Guide](../index.html)
  + [1. Introduction to CUDA](../part1.html)
    - [1.1. Introduction](../01-introduction/introduction.html)
    - [1.2. Programming Model](../01-introduction/programming-model.html)
    - [1.3. The CUDA platform](../01-introduction/cuda-platform.html)
  + [2. Programming GPUs in CUDA](../part2.html)
    - [2.1. Intro to CUDA C++](../02-basics/intro-to-cuda-cpp.html)
    - [2.2. Intro to CUDA Python](../02-basics/intro-to-cuda-python.html)
    - [2.3. Writing SIMT Kernels](../02-basics/writing-cuda-kernels.html)
    - [2.4. Writing Tile Kernels](../02-basics/writing-tile-kernels.html)
    - [2.5. Asynchronous Execution](../02-basics/asynchronous-execution.html)
    - [2.6. Unified and System Memory](../02-basics/understanding-memory.html)
    - [2.7. NVCC: The NVIDIA CUDA Compiler](../02-basics/nvcc.html)
  + [3. Advanced CUDA](../part3.html)
    - [3.1. Advanced CUDA APIs and Features](../03-advanced/advanced-host-programming.html)
    - [3.2. Advanced Kernel Programming](../03-advanced/advanced-kernel-programming.html)
    - [3.3. The CUDA Driver API](../03-advanced/driver-api.html)
    - [3.4. Programming Systems with Multiple GPUs](../03-advanced/multi-gpu-systems.html)
    - [3.5. A Tour of CUDA Features](../03-advanced/feature-survey.html)
  + [4. CUDA Features](../part4.html)
    - [4.1. Unified Memory](../04-special-topics/unified-memory.html)
    - [4.2. CUDA Graphs](../04-special-topics/cuda-graphs.html)
    - [4.3. Stream-Ordered Memory Allocator](../04-special-topics/stream-ordered-memory-allocation.html)
    - [4.4. Cooperative Groups](../04-special-topics/cooperative-groups.html)
    - [4.5. Programmatic Dependent Launch and Synchronization](../04-special-topics/programmatic-dependent-launch.html)
    - [4.6. Green Contexts](../04-special-topics/green-contexts.html)
    - [4.7. Locality Domains](../04-special-topics/locality-domains.html)
    - [4.8. Lazy Loading](../04-special-topics/lazy-loading.html)
    - [4.9. Error Log Management](../04-special-topics/error-log-management.html)
    - [4.10. Asynchronous Barriers](../04-special-topics/async-barriers.html)
    - [4.11. Pipelines](../04-special-topics/pipelines.html)
    - [4.12. Asynchronous Data Copies](../04-special-topics/async-copies.html)
    - [4.13. Work Stealing with Cluster Launch Control](../04-special-topics/cluster-launch-control.html)
    - [4.14. L2 Cache Control](../04-special-topics/l2-cache-control.html)
    - [4.15. Memory Synchronization Domains](../04-special-topics/memory-sync-domains.html)
    - [4.16. Interprocess Communication](../04-special-topics/inter-process-communication.html)
    - [4.17. Virtual Memory Management](../04-special-topics/virtual-memory-management.html)
    - [4.18. Compute Fabric Transport](../04-special-topics/compute-fabric-transport.html)
    - [4.19. Extended GPU Memory](../04-special-topics/extended-gpu-memory.html)
    - [4.20. CUDA Dynamic Parallelism](../04-special-topics/dynamic-parallelism.html)
    - [4.21. CUDA Interoperability with APIs](../04-special-topics/graphics-interop.html)
    - [4.22. Driver Entry Point Access](../04-special-topics/driver-entry-point-access.html)
  + [5. Technical Appendices](../part5.html)
    - [5.1. Compute Capabilities](#)
    - [5.2. CUDA Environment Variables](environment-variables.html)
    - [5.3. C++ Language Support](cpp-language-support.html)
    - [5.4. C/C++ Language Extensions](cpp-language-extensions.html)
    - [5.5. Floating-Point Computation](mathematical-functions.html)
    - [5.6. Device-Callable APIs and Intrinsics](device-callable-apis.html)
    - [5.7. CUDA C++ Memory Model](cuda-cpp-memory-model.html)
    - [5.8. CUDA C++ Execution model](cuda-cpp-execution-model.html)
  + [6. Notices](../notices.html)

* [CUDA Programming Guide](../index.html)
* [5. Technical Appendices](../part5.html)
* 5.1. Compute Capabilities

[Is this page helpful?](https://surveys.hotjar.com/4904bf71-6484-47a7-83ff-4715cceabdb5)

5.1. Compute Capabilities[#](#compute-capabilities "Link to this heading")
==========================================================================

The general specifications and features of a compute device depend on its compute capability (see [Compute Capability and Streaming Multiprocessor Versions](../01-introduction/cuda-platform.html#cuda-platform-compute-capability-sm-version)).

[Table 29](#compute-capabilities-table-features-and-technical-specifications-feature-support-per-compute-capability), [Table 30](#compute-capabilities-table-device-and-streaming-multiprocessor-sm-information-per-compute-capability), and
[Table 31](#compute-capabilities-table-memory-information-per-compute-capability) show the features and technical specifications associated with each compute capability that is currently supported.

All NVIDIA GPU architectures use a little-endian representation.

5.1.1. Obtain the GPU Compute Capability[#](#obtain-the-gpu-compute-capability "Link to this heading")
------------------------------------------------------------------------------------------------------

The [CUDA GPU Compute Capability](https://developer.nvidia.com/cuda-gpus) page provides a comprehensive mapping from NVIDIA GPU models to their compute capability.

Alternatively, the [nvidia-smi](https://docs.nvidia.com/deploy/nvidia-smi/index.html) tool, provided with the [NVIDIA Driver](https://www.nvidia.com/en-us/drivers/), can be used to get the compute capability of a GPU. For example, the following command will output the GPU names and compute capabilities available on the system:

```
nvidia-smi --query-gpu=name,compute_cap
```

At runtime, the compute capability can be obtained using the CUDA Runtime API [cudaDeviceGetAttribute()](https://docs.nvidia.com/cuda/cuda-runtime-api/group__CUDART__DEVICE.html#group__CUDART__DEVICE_1gb22e8256592b836df9a9cc36c9db7151) , CUDA Driver API [cuDeviceGetAttribute()](https://docs.nvidia.com/cuda/cuda-driver-api/group__CUDA__DEVICE.html#group__CUDA__DEVICE_1g9c3e1414f0ad901d3278a4d6645fc266), or NVML API [nvmlDeviceGetCudaComputeCapability()](https://docs.nvidia.com/deploy/nvml-api/group__nvmlDeviceQueries.html#group__nvmlDeviceQueries_1g1f803a2fb4b7dfc0a8183b46b46ab03a):

```
#include <cuda_runtime_api.h>

int computeCapabilityMajor, computeCapabilityMinor;
cudaDeviceGetAttribute(&computeCapabilityMajor, cudaDevAttrComputeCapabilityMajor, device_id);
cudaDeviceGetAttribute(&computeCapabilityMinor, cudaDevAttrComputeCapabilityMinor, device_id);
```

```
#include <cuda.h>

int computeCapabilityMajor, computeCapabilityMinor;
cuDeviceGetAttribute(&computeCapabilityMajor, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MAJOR, device_id);
cuDeviceGetAttribute(&computeCapabilityMinor, CU_DEVICE_ATTRIBUTE_COMPUTE_CAPABILITY_MINOR, device_id);
```

```
#include <nvml.h> // required linking with -lnvidia-ml

int computeCapabilityMajor, computeCapabilityMinor;
nvmlDeviceGetCudaComputeCapability(nvmlDevice, &computeCapabilityMajor, &computeCapabilityMinor);
```

5.1.2. Feature Availability[#](#feature-availability "Link to this heading")
----------------------------------------------------------------------------

Most compute features introduced with a compute architecture are intended to be available on all subsequent architectures. This is shown in [Table 29](#compute-capabilities-table-features-and-technical-specifications-feature-support-per-compute-capability) by the “yes” for availability of a feature on compute capabilities subsequent to its introduction.

### 5.1.2.1. Architecture-Specific Features[#](#architecture-specific-features "Link to this heading")

Beginning with devices of Compute Capability 9.0, specialized compute features that are introduced with an architecture may not be guaranteed to be available on all subsequent compute capabilities. These features are called *architecture-specific* features and target acceleration of specialized operations, such as Tensor Core operations, which are not intended for all classes of compute capabilities or may significantly change in future generations. Code must be compiled with an architecture-specific compiler target (see [Feature Set Compiler Targets](#compute-capabilities-feature-set-compiler-targets)) to enable architecture-specific features. Code compiled with an architecture-specific compiler target can only be run on the exact compute capability it was compiled for.

### 5.1.2.2. Family-Specific Features[#](#family-specific-features "Link to this heading")

Beginning with devices of Compute Capability 10.0, some architecture-specific features are common to devices of more than one compute capability. The devices that contain these features are part of the same family and these features can also be called *family-specific* features. Family-specific features are guaranteed to be available on all devices in the same family. A family-specific compiler target is required to enable family-specific features. See [Section 5.1.2.3](#compute-capabilities-feature-set-compiler-targets). Code compiled for a family-specific target can only be run on GPUs which are members of that family.

### 5.1.2.3. Feature Set Compiler Targets[#](#feature-set-compiler-targets "Link to this heading")

There are three sets of compute features which the compiler can target:

**Baseline Feature Set**: The predominant set of compute features that are introduced with the intent to be available for subsequent compute architectures. These features and their availability are summarized in [Table 29](#compute-capabilities-table-features-and-technical-specifications-feature-support-per-compute-capability).

**Architecture-Specific Feature Set**: A small and highly specialized set of features called architecture-specific, that are introduced to accelerate specialized operations, which are not guaranteed to be available or might change significantly on subsequent compute architectures. These features are summarized in the respective “Compute Capability #.#” subsections. The architecture-specific feature set is a superset of the family-specific feature set. Architecture-specific compiler targets were introduced with Compute Capability 9.0 devices and are selected by using an **a** suffix in the compilation target, for example by specifying `compute_100a` or `compute_120a` as the compute target.

**Family-Specific Feature Set**: Some architecture-specific features are common to GPUs of more than one compute capability. These features are summarized in the respective “Compute Capability #.#” subsections. With a few exceptions, later-generation devices with the same major compute capability are in the same family. [Table 28](#compute-capabilities-family-specific-compatibility) indicates the compatibility of family-specific targets with device compute capability, including exceptions. The family-specific feature set is a superset of the baseline feature set. Family-specific compiler targets were introduced with Compute Capability 10.0 devices and are selected by using an **f** suffix in the compilation target, for example by specifying `compute_100f` or `compute_120f` as the compute target.

All devices starting from compute capability 9.0 have a set of features that are architecture-specific. To utilize the complete set of these features on a specific GPU, the architecture-specific compiler target with the suffix **a** must be used. Additionally, starting from compute capability 10.0, there are sets of features that appear in multiple devices with different minor compute capabilities. These sets of instructions are called family-specific features, and the devices which share these features are said to be part of the same family. The family-specific features are a subset of the architecture-specific features that are shared by all members of that GPU family. The family-specific compiler target with the suffix **f** allows the compiler to generate code that uses this common subset of architecture-specific features.

For example:

* The `compute_100` compilation target does not allow the use of architecture-specific features. This target will be compatible with all devices of compute capability 10.0 and later.
* The `compute_100f` *family-specific* compilation target allows the use of the subset of architecture-specific features that are common across the GPU family. This target will only be compatible with devices that are part of the GPU family. In this example, it is compatible with devices of Compute Capability 10.0, 10.3, and 10.7. The features available in the family-specific `compute_100f` target are a superset of the features available in the baseline `compute_100` target.
* The `compute_100a` *architecture-specific* compilation target allows the use of the complete set of architecture-specific features in Compute Capability 10.0 devices. This target will only be compatible with devices of Compute Capability 10.0 and no others. The features available in the `compute_100a` target form a superset of the features available in the `compute_100f` target.

Table 28 Family-Specific Compatibility[#](#compute-capabilities-family-specific-compatibility "Link to this table")

| Compilation Target | Compatible with Compute Capability | | |
| --- | --- | --- | --- |
| `compute_100f` | 10.0 | 10.3 | 10.7 |
| `compute_103f` | 10.3 | 10.7 | |
| `compute_107f` | 10.7 [[1]](#family2) | | |
| `compute_110f` | 11.0 [[1]](#family2) | | |
| `compute_120f` | 12.0 | 12.1 | |
| `compute_121f` | 12.1 [[1]](#family2) | | |

[1]
([1](#id2),[2](#id3),[3](#id4))

Some families only contain a single member when they are created. They may be expanded in the future to include more devices.

5.1.3. Features and Technical Specifications[#](#features-and-technical-specifications "Link to this heading")
--------------------------------------------------------------------------------------------------------------

Table 29 Feature Support per Compute Capability[#](#compute-capabilities-table-features-and-technical-specifications-feature-support-per-compute-capability "Link to this table")

| **Feature Support** | **Compute Capability** | | | | | |
| --- | --- | --- | --- | --- | --- | --- |
| (Unlisted features are supported for all compute capabilities) | 7.x | 8.x | 9.0 | 10.x | 11.0 | 12.x |
| Atomic functions operating on 128-bit integer values in shared and global memory ([Atomic Functions](cpp-language-extensions.html#atomic-functions)) | No | | Yes | | | |
| Atomic addition operating on `float2` and `float4` floating point vectors in global memory ([atomicAdd()](cpp-language-extensions.html#atomicadd)) | No | | Yes | | | |
| Warp reduce functions ([Warp Reduce Functions](cpp-language-extensions.html#warp-reduce-functions)) | No | Yes | | | | |
| Bfloat16-precision floating-point operations | No | Yes | | | | |
| 128-bit-precision floating-point operations | No | | | Yes | | |
| Hardware-accelerated `memcpy_async` ([Pipelines](../04-special-topics/pipelines.html#pipelines)) | No | Yes | | | | |
| Hardware-accelerated Split Arrive/Wait Barrier ([Asynchronous Barriers](../04-special-topics/async-barriers.html#asynchronous-barriers)) | No | Yes | | | | |
| L2 Cache Residency Management ([L2 Cache Control](../04-special-topics/l2-cache-control.html#advanced-kernels-l2-control)) | No | Yes | | | | |
| DPX Instructions for Accelerated Dynamic Programming ([Dynamic Programming eXtension (DPX) Instructions](cpp-language-extensions.html#dpx-instructions)) | Multiple Instr. | | Native | | Multiple Instr. | |
| Distributed Shared Memory | No | | Yes | | | |
| Thread Block Cluster ([Thread Block Clusters](../02-basics/intro-to-cuda-cpp.html#thread-block-clusters)) | No | | Yes | | | |
| Tensor Memory Accelerator (TMA) unit ([Using the Tensor Memory Accelerator (TMA)](../04-special-topics/async-copies.html#async-copies-tma)) | No | | Yes | | | |

Note that the KB and K units used in the following tables correspond to 1024 bytes (i.e., a KiB) and 1024 respectively.

Table 30 Device and Streaming Multiprocessor (SM) Information per Compute Capability[#](#compute-capabilities-table-device-and-streaming-multiprocessor-sm-information-per-compute-capability "Link to this table")

|  | **Compute Capability** | | | | | | | | | | |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 7.5 | 8.0 | 8.6 | 8.7 | 8.9 | 9.0 | 10.0 | 10.3 | 10.7 | 11.0 | 12.x |
| Ratio of FP32 to FP64 Throughput [[2]](#fn-cc-throughput) | 32:1 | 2:1 | 64:1 | | | 2:1 | | 64:1 | 4:1 | 64:1 | |
| Maximum number of resident grids per device (Concurrent Kernel Execution) | 128 | | | | | | | | | | |
| Maximum dimensionality of a grid | 3 | | | | | | | | | | |
| Maximum x-dimension of a grid | 231-1 | | | | | | | | | | |
| Maximum y- or z-dimension of a grid | 65535 | | | | | | | | | | |
| Maximum dimensionality of a thread block | 3 | | | | | | | | | | |
| Maximum x- or y-dimensionality of a thread block | 1024 | | | | | | | | | | |
| Maximum z-dimension of a thread block | 64 | | | | | | | | | | |
| Maximum number of threads per block | 1024 | | | | | | | | | | |
| Warp size | 32 | | | | | | | | | | |
| Maximum number of resident blocks per SM | 16 | 32 | 16 | | 24 | 32 | | | 16 | 24 | |
| Maximum number of resident warps per SM | 32 | 64 | 48 | | | 64 | | | 32 | 48 | |
| Maximum number of resident threads per SM | 1024 | 2048 | 1536 | | | 2048 | | | 1024 | 1536 | |
| Green contexts: minimum SM partition size for useFlags 0 | 2 | 4 | | | | 8 | | | | | |
| Green contexts: SM co-scheduled alignment per partition for useFlags 0 | 2 | | | | | 8 | | | | | |

[[2](#id5)]

Non-Tensor Core throughputs. For more information on throughput see the [CUDA Best Practices Guide](https://docs.nvidia.com/cuda/cuda-c-best-practices-guide/index.html#arithmetic-instructions-throughput-native-arithmetic-instructions)

Table 31 Memory Information per Compute Capability[#](#compute-capabilities-table-memory-information-per-compute-capability "Link to this table")

|  | **Compute Capability** | | | | | | | | | | |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | 7.5 | 8.0 | 8.6 | 8.7 | 8.9 | 9.0 | 10.0 | 10.3 | 10.7 | 11.0 | 12.x |
| Number of 32-bit registers per SM | 64 K | | | | | | | | | | |
| Maximum number of 32-bit registers per thread block | 64 K | | | | | | | | | | |
| Maximum number of 32-bit registers per thread | 255 | | | | | | | | | | |
| Maximum amount of shared memory per SM | 64 KB | 164 KB | 100 KB | 164 KB | 100 KB | 228 KB | | | 328 KB | 228 KB | 100 KB |
| Maximum amount of shared memory per thread block [[3]](#fn33) | 64 KB | 163 KB | 99 KB | 163 KB | 99 KB | 227 KB | | | 327 KB | 227 KB | 99 KB |
| Number of shared memory banks | 32 | | | | | | | | | | |
| Maximum amount of local memory per thread | 512 KB | | | | | | | | | | |
| Constant memory size | 64 KB | | | | | | | | | | |
| Cache working set per SM for constant memory | 8 KB | | | | | | | | | | |
| Cache working set per SM for texture memory | 32 or 64 KB | 28 KB ~ 192 KB | 28 KB ~ 128 KB | 28 KB ~ 192 KB | 28 KB ~ 128 KB | 28 KB ~ 256 KB | | | 8 KB ~ 256 KB | 28 KB ~ 256 KB | 28 KB ~ 128 KB |

[[3](#id6)]

Kernels relying on shared memory allocations over 48 KB per block must use dynamic shared memory and require an explicit opt-in, see [Configuring L1/Shared Memory Balance](../03-advanced/advanced-kernel-programming.html#advanced-kernel-l1-shared-config).

Table 32 Shared Memory Capacity per Compute Capability[#](#compute-capabilities-table-shared-memory-capacity-per-compute-capability "Link to this table")

| Compute Capability | Unified Data Cache Size (KB) | SMEM Capacity Sizes (KB) |
| --- | --- | --- |
| 7.5 | 96 | 32, 64 |
| 8.0 | 192 | 0, 8, 16, 32, 64, 100, 132, 164 |
| 8.6 | 128 | 0, 8, 16, 32, 64, 100 |
| 8.7 | 192 | 0, 8, 16, 32, 64, 100, 132, 164 |
| 8.9 | 128 | 0, 8, 16, 32, 64, 100 |
| 9.0 | 256 | 0, 8, 16, 32, 64, 100, 132, 164, 196, 228 |
| 10.0 | 256 | 0, 8, 16, 32, 64, 100, 132, 164, 196, 228 |
| 10.3 | 256 | 0, 8, 16, 32, 64, 100, 132, 164, 196, 228 |
| 10.7 [[4]](#fn107) | 336 | 0, 8, 16, 32, 64, 100, 132, 164, 196, 228, 328 |
| 11.0 | 256 | 0, 8, 16, 32, 64, 100, 132, 164, 196, 228 |
| 12.x | 128 | 0, 8, 16, 32, 64, 100 |

[[4](#id7)]

For devices of compute capability 10.7, kernels that use the 328 KB shared-memory configuration must explicitly enable `cudaSharedMemoryModeAllowOversizedSharedMemory` by setting either the `cudaFuncAttributeSharedMemoryMode` function attribute or the `cudaLaunchAttributeSharedMemoryMode` launch attribute.

[Table 33](#compute-capabilities-table-tensor-core-data-types-per-compute-capability) shows the input data types supported by Tensor Core acceleration. The Tensor Core feature set is available within the CUDA compilation toolchain through inline PTX. It is strongly recommended that applications use this feature set through CUDA-X libraries such as cuDNN, cuBLAS, and cuFFT, for example, or through [CUTLASS](https://docs.nvidia.com/cutlass/index.html), a collection of CUDA C++ template abstractions and Python domain-specific languages (DSLs) designed to enable high-performance matrix-matrix multiplication (GEMM) and related computations across all levels within CUDA.

Table 33 Input Data Types Supported by Tensor Core Acceleration per Compute Capability[#](#compute-capabilities-table-tensor-core-data-types-per-compute-capability "Link to this table")

| Compute Capability | Tensor Core Input Data Types | | | | | | | | |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | FP64 | TF32 | BF16 | FP16 | FP8 | FP6 | FP4 | INT8 | INT4 |
| 7.5 |  | | | Yes |  | | | Yes | Yes |
| 8.0 | Yes | Yes | Yes | Yes |  | | | Yes | Yes |
| 8.6 |  | Yes | Yes | Yes |  | | | Yes | Yes |
| 8.7 |  | Yes | Yes | Yes |  | | | Yes | Yes |
| 8.9 |  | Yes | Yes | Yes | Yes |  | | Yes | Yes |
| 9.0 | Yes | Yes | Yes | Yes | Yes |  | | Yes |  |
| 10.0 | Yes | Yes | Yes | Yes | Yes | Yes | Yes | Yes |  |
| 10.3 |  | Yes | Yes | Yes | Yes | Yes | Yes | Yes |  |
| 10.7 | Yes | Yes | Yes | Yes | Yes | Yes | Yes |  |  |
| 11.0 |  | Yes | Yes | Yes | Yes | Yes | Yes | Yes |  |
| 12.x |  | Yes | Yes | Yes | Yes | Yes | Yes | Yes |  |

[previous

5. Technical Appendices](../part5.html "previous page")
[next

5.2. CUDA Environment Variables](environment-variables.html "next page")

On this page

* [5.1.1. Obtain the GPU Compute Capability](#obtain-the-gpu-compute-capability)
* [5.1.2. Feature Availability](#feature-availability)
  + [5.1.2.1. Architecture-Specific Features](#architecture-specific-features)
  + [5.1.2.2. Family-Specific Features](#family-specific-features)
  + [5.1.2.3. Feature Set Compiler Targets](#feature-set-compiler-targets)
* [5.1.3. Features and Technical Specifications](#features-and-technical-specifications)

[![NVIDIA](../_static/nvidia-logo-horiz-rgb-1c-blk-for-screen.svg)![NVIDIA](../_static/nvidia-logo-horiz-rgb-1c-wht-for-screen.svg)](https://www.nvidia.com)

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

![](https://docs.nvidia.com/akam/13/pixel_44dcce21?a=dD1hMjU2MzkwNDEwZTk4YjhmOGRiN2JiMzRmMDYxYzgzZDdjMDI2MTA5JmpzPW9mZg==)