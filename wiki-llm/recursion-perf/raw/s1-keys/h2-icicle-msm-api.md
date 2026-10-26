# MSM - Multi Scalar Multiplication | ICICLE Docs
> Source: https://dev.ingonyama.com/api/cpp/msm
> Collected: 2026-10-26
> Published: Unknown

Page footer says last updated 2025-08-06; API version 4.0.0.

MSM - Multi Scalar Multiplication | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[4.0.0](/api/cpp/msm)

* [4.0.0](/api/cpp/msm)
* [3.9.0](/3.9.0/api/cpp/msm)
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

* [API Overview](/apioverview)
* [C++](/cppstart)

  + [C++ Overview](/cppstart)
  + [MSM](/api/cpp/msm)
  + [NTT / ECNTT](/api/cpp/ntt)
  + [Vector operations](/api/cpp/vec_ops)
  + [Program](/api/cpp/program)
  + [Polynomials](/api/cpp/polynomials/overview)
  + [Hash](/api/cpp/hash)
  + [Merkle-Tree](/api/cpp/merkle)
  + [Sumcheck](/api/cpp/sumcheck)
  + [FRI](/api/cpp/fri)
  + [Pairings](/icicle/primitives/pairings)
  + [PQC ML-KEM](/api/cpp/lattice/pqc_ml_kem)
  + [Serialization](/api/cpp/serialization)
  + [Matrix Operations](/api/cpp/matrix_ops)
* [Golang Bindings](/gooverview)

  + [Golang Overview](/gooverview)
  + [MSM](/api/golang-bindings/msm)
  + [MSM pre-computation](/api/golang-bindings/msm-pre-computation)
  + [NTT](/api/golang-bindings/ntt)
  + [EC-NTT](/api/golang-bindings/ecntt)
  + [Vector operations](/api/golang-bindings/vec-ops)
  + [Multi GPU Support](/api/golang-bindings/multi-gpu)
  + [Hash](/api/golang-bindings/hash)
  + [Merkle-Tree](/api/golang-bindings/merkle)
  + [PQC ML-KEM](/api/golang-bindings/lattice/pqc-ml-kem)
* [Rust Bindings](/rustoverview)

  + [Rust Overview](/rustoverview)
  + [MSM](/api/rust-bindings/msm)
  + [NTT](/api/rust-bindings/ntt)
  + [ECNTT](/api/rust-bindings/ecntt)
  + [Vector operations](/api/rust-bindings/vec-ops)
  + [Program](/api/rust-bindings/program)
  + [Polynomials](/api/rust-bindings/polynomials)
  + [Hash](/api/rust-bindings/hash)
  + [Merkle-Tree](/api/rust-bindings/merkle)
  + [Sumcheck](/api/rust-bindings/sumcheck)
  + [FRI](/api/rust-bindings/fri)
  + [Lattice SNARKs](/api/rust-bindings/lattice/lattice-snarks)
  + [PQC ML-KEM](/api/rust-bindings/lattice/pqc-ml-kem)
  + [Serialization](/api/rust-bindings/serialization)
  + [Pairings](/api/rust-bindings/pairing)
  + [Matrix Operations](/api/rust-bindings/matrix_ops)

* C++
* MSM
Version: 4.0.0

On this page

MSM - Multi Scalar Multiplication
=================================

Overview[​](#overview "Direct link to Overview")
------------------------------------------------

Multi-Scalar Multiplication (MSM) is a fundamental operation in elliptic curve cryptography and zero-knowledge proofs. It is defined as:

MSM(a,G)=∑j=0n−1aj⋅GjMSM(a, G) = \sum\_{j=0}^{n-1} a\_j \cdot G\_jMSM(a,G)=j=0∑n−1​aj​⋅Gj​

Where:

* Gj∈GG\_j \in GGj​∈G are points from an elliptic curve group.
* a0,…,ana\_0, \ldots, a\_na0​,…,an​ are scalars.
* MSM(a,G)∈GMSM(a, G) \in GMSM(a,G)∈G is the result, a single elliptic curve point.

MSM is inherently parallelizable, making it a critical operation for optimizing performance in cryptographic protocols like zk-SNARKs. Accelerating MSM can significantly reduce the time required for proof generation.

Accelerating MSM is crucial to a ZK protocol's performance due to the [large percent of run time](https://hackmd.io/@0xMonia/SkQ6-oRz3#Hardware-acceleration-in-action) they take when generating proofs.
You can learn more about how MSMs work from this [video](https://www.youtube.com/watch?v=Bl5mQA7UL2I) and from our resource list on [Ingopedia](https://www.ingonyama.com/ingopedia/msm).

C++ API[​](#c-api "Direct link to C++ API")
-------------------------------------------

### `MSMConfig` Struct[​](#msmconfig-struct "Direct link to msmconfig-struct")

The `MSMConfig` struct configures the MSM operation. It allows customization of parameters like the number of precomputed points, the window bitsize (`c`), and memory management. Here's the configuration structure:

```
struct MSMConfig {  
  icicleStreamHandle stream;  
  int precompute_factor;  
  int c;  
  int bitsize;  
  int batch_size;  
  bool are_points_shared_in_batch;  
  bool are_scalars_on_device;  
  bool are_scalars_montgomery_form;  
  bool are_points_on_device;  
  bool are_points_montgomery_form;  
  bool are_results_on_device;  
  bool is_async;  
  ConfigExtension* ext;  
};
```

#### Default Configuration[​](#default-configuration "Direct link to Default Configuration")

You can obtain a default `MSMConfig` using:

```
  static MSMConfig default_msm_config()  
  {  
    MSMConfig config = {  
      nullptr, // stream  
      1,       // precompute_factor  
      0,       // c  
      0,       // bitsize  
      1,       // batch_size  
      true,    // are_points_shared_in_batch  
      false,   // are_scalars_on_device  
      false,   // are_scalars_montgomery_form  
      false,   // are_points_on_device  
      false,   // are_points_montgomery_form  
      false,   // are_results_on_device  
      false,   // is_async  
      nullptr, // ext  
    };  
    return config;  
  }
```

### `msm` Function[​](#msm-function "Direct link to msm-function")

The `msm` function computes the MSM operation:

```
template <typename S, typename A, typename P>  
eIcicleError msm(const S* scalars, const A* bases, int msm_size, const MSMConfig& config, P* results);
```

note

The API is template and can work with all ICICLE curves (if corresponding lib is linked), including G2 groups.

### Batched MSM[​](#batched-msm "Direct link to Batched MSM")

The MSM supports batch mode - running multiple MSMs in parallel. It's always better to use the batch mode instead of running single msms in serial as long as there is enough memory available. We support running a batch of MSMs that share the same points as well as a batch of MSMs that use different points.

Config fields `are_points_shared_in_batch` and `batch_size` are used to configure msm for batch mode.

### G2 MSM[​](#g2-msm "Direct link to G2 MSM")

for G2 MSM, use the [same msm api](#msm-function) with the G2 types.

note

Supported curves have types for both G1 and G2.

### Precompution[​](#precompution "Direct link to Precompution")

#### What It Does:[​](#what-it-does "Direct link to What It Does:")

* The function computes a set of additional points derived from the original base points. These precomputed points are stored and later reused during the MSM computation.
* Purpose: By precomputing and storing these points, the MSM operation can reduce the number of operations needed at runtime, which can significantly speed up the calculation.

#### When to Use:[​](#when-to-use "Direct link to When to Use:")

* Memory vs. Speed Trade-off: Precomputation increases the memory footprint because additional points are stored, but it reduces the computational effort during MSM, making the process faster.
* Best for Repeated Use: It’s especially beneficial when the same set of base points is used multiple times in different MSM operations.

```
template <typename A>  
eIcicleError msm_precompute_bases(const A* input_bases, int bases_size, const MSMConfig& config, A* output_bases);
```

note

User is allocating the `output_bases` (on host or device memory) and later use it as bases when calling msm.

Rust and Go bindings[​](#rust-and-go-bindings "Direct link to Rust and Go bindings")
------------------------------------------------------------------------------------

The Rust and Go bindings provide equivalent functionality for their respective environments. Refer to their documentation for details on usage.

* [Golang](/api/golang-bindings/msm)
* [Rust](/api/rust-bindings/msm)

CUDA backend MSM[​](#cuda-backend-msm "Direct link to CUDA backend MSM")
------------------------------------------------------------------------

This section describes the CUDA msm implementation and how to customize it (optional).

### Algorithm description[​](#algorithm-description "Direct link to Algorithm description")

We follow the bucket method algorithm. The GPU implementation consists of four phases:

1. Preparation phase - The scalars are split into smaller scalars of `c` bits each. These are the bucket indices. The points are grouped according to their corresponding bucket index and the buckets are sorted by size.
2. Accumulation phase - Each bucket accumulates all of its points using a single thread. More than one thread is assigned to large buckets, in proportion to their size. A bucket is considered large if its size is above the large bucket threshold that is determined by the `large_bucket_factor` parameter. The large bucket threshold is the expected average bucket size times the `large_bucket_factor` parameter.
3. Buckets Reduction phase - bucket results are multiplied by their corresponding bucket number and each bucket module is reduced to a small number of final results. By default, this is done by an iterative algorithm which is highly parallel. Setting `is_big_triangle` to `true` will switch this phase to the running sum algorithm described in the above YouTube talk which is much less parallel.
4. Final accumulation phase - The final results from the last phase are accumulated using the double-and-add algorithm.

Configuring CUDA msm[​](#configuring-cuda-msm "Direct link to Configuring CUDA msm")
------------------------------------------------------------------------------------

Use `ConfigExtension` object to pass backend specific configuration.
CUDA specific msm configuration:

```
ConfigExtension ext;  
ext.set("large_bucket_factor", 15);   
// use the config-extension in the msm config for the backend to see.  
msm_config.ext = &ext;  
// call msm  
msm(..., config,...); // msm backend is reading the config-extension
```

### Choosing optimal parameters[​](#choosing-optimal-parameters "Direct link to Choosing optimal parameters")

`is_big_triangle` should be `false` in almost all cases. It might provide better results only for very small MSMs (smaller than 2^8) with a large batch (larger than 100) but this should be tested per scenario.
Large buckets exist in two cases:

1. When the scalar distribution isn't uniform.
2. When `c` does not divide the scalar bit-size.

`large_bucket_factor` that is equal to 10 yields good results for most cases, but it's best to fine tune this parameter per `c` and per scalar distribution.
The two most important parameters for performance are `c` and the `precompute_factor`. They affect the number of EC additions as well as the memory size. When the points are not known in advance we cannot use precomputation. In this case the best `c` value is usually around log2(msmSize)−4log\_2(msmSize) - 4log2​(msmSize)−4. However, in most protocols the points are known in advanced and precomputation can be used unless limited by memory. Usually it's best to use maximum precomputation (such that we end up with only a single bucket module) combined we a `c` value around log2(msmSize)−1log\_2(msmSize) - 1log2​(msmSize)−1.

Memory usage estimation[​](#memory-usage-estimation "Direct link to Memory usage estimation")
---------------------------------------------------------------------------------------------

The main memory requirements of the MSM are the following:

* Scalars - `sizeof(scalar_t) * msm_size * batch_size`
* Scalar indices - `~6 * sizeof(unsigned) * nof_bucket_modules * msm_size * batch_size`
* Points - `sizeof(affine_t) * msm_size * precomp_factor * batch_size`
* Buckets - `sizeof(projective_t) * nof_bucket_modules * 2^c * batch_size`

where `nof_bucket_modules = ceil(ceil(bitsize / c) / precompute_factor)`

During the MSM computation first the memory for scalars and scalar indices is allocated, then the indices are freed and points and buckets are allocated. This is why a good estimation for the required memory is the following formula:

max(scalars+scalarIndices,scalars+points+buckets)max(scalars + scalarIndices, scalars + points + buckets)max(scalars+scalarIndices,scalars+points+buckets)

This gives a good approximation within 10% of the actual required memory for most cases.

Example parameters[​](#example-parameters "Direct link to Example parameters")
------------------------------------------------------------------------------

Here is a useful table showing optimal parameters for different MSMs. They are optimal for BLS12-377 curve when running on NVIDIA GeForce RTX 3090 Ti. This is the configuration used:

Here are the parameters and the results for the different cases:

| MSM size | Batch size | Precompute factor | c | Memory estimation (GB) | Actual memory (GB) | Single MSM time (ms) |
| --- | --- | --- | --- | --- | --- | --- |
| 10 | 1 | 1 | 9 | 0.00227 | 0.00277 | 9.2 |
| 10 | 1 | 23 | 11 | 0.00259 | 0.00272 | 1.76 |
| 10 | 1000 | 1 | 7 | 0.94 | 1.09 | 0.051 |
| 10 | 1000 | 23 | 11 | 2.59 | 2.74 | 0.025 |
| 15 | 1 | 1 | 11 | 0.011 | 0.019 | 9.9 |
| 15 | 1 | 16 | 16 | 0.061 | 0.065 | 2.4 |
| 15 | 100 | 1 | 11 | 1.91 | 1.92 | 0.84 |
| 15 | 100 | 19 | 14 | 6.32 | 6.61 | 0.56 |
| 18 | 1 | 1 | 14 | 0.128 | 0.128 | 14.4 |
| 18 | 1 | 15 | 17 | 0.40 | 0.42 | 5.9 |
| 22 | 1 | 1 | 17 | 1.64 | 1.65 | 68 |
| 22 | 1 | 13 | 21 | 5.67 | 5.94 | 54 |
| 24 | 1 | 1 | 18 | 6.58 | 6.61 | 232 |
| 24 | 1 | 7 | 21 | 12.4 | 13.4 | 199 |

The optimal values can vary per GPU and per curve. It is best to try a few combinations until you get the best results for your specific case.

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-4.0.0/api/cpp/msm.md)

Last updated on **Aug 6, 2025**

[Previous

C++ Overview](/cppstart)[Next

NTT / ECNTT](/api/cpp/ntt)

* [Overview](#overview)
* [C++ API](#c-api)
  + [`MSMConfig` Struct](#msmconfig-struct)
  + [`msm` Function](#msm-function)
  + [Batched MSM](#batched-msm)
  + [G2 MSM](#g2-msm)
  + [Precompution](#precompution)
* [Rust and Go bindings](#rust-and-go-bindings)
* [CUDA backend MSM](#cuda-backend-msm)
  + [Algorithm description](#algorithm-description)
* [Configuring CUDA msm](#configuring-cuda-msm)
  + [Choosing optimal parameters](#choosing-optimal-parameters)
* [Memory usage estimation](#memory-usage-estimation)
* [Example parameters](#example-parameters)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.