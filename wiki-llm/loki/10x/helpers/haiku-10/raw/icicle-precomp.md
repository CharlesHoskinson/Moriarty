MSM Pre computation | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[1.10.1](/1.10.1/icicle/golang-bindings/msm-pre-computation)

* [4.0.0](/)
* [3.9.0](/3.9.0)
* [3.8.0](/3.8.0/icicle/golang-bindings/msm-pre-computation)
* [3.7.0](/3.7.0/icicle/golang-bindings/msm-pre-computation)
* [3.6.0](/3.6.0/icicle/golang-bindings/msm-pre-computation)
* [3.5.0](/3.5.0/icicle/golang-bindings/msm-pre-computation)
* [3.4.0](/3.4.0/icicle/golang-bindings/msm-pre-computation)
* [3.3.0](/3.3.0/icicle/golang-bindings/msm-pre-computation)
* [3.2.0](/3.2.0/icicle/golang-bindings/msm-pre-computation)
* [3.1.0](/3.1.0/icicle/golang-bindings/msm-pre-computation)
* [3.0.0](/3.0.0/icicle/golang-bindings/msm-pre-computation)
* [2.8.0](/2.8.0/icicle/golang-bindings/msm-pre-computation)
* [1.10.1](/1.10.1/icicle/golang-bindings/msm-pre-computation)

[Community](#)

* [Discord](https://discord.gg/6vYrE7waPj)
* [LinkedIn](https://www.linkedin.com/company/ingonyama)
* [X/Twitter](https://x.com/Ingo_zk)
* [YouTube](https://www.youtube.com/@ingo_ZK)
* [Mailing List](https://wkf.ms/3LKCbdj)

[Leave Feedback](https://forms.monday.com/forms/7b51e0bdad766b71da8869704c301472?r=use1)

[![Ingonyama logo](/img/Ingologo.svg)](https://ingonyama.com "Visit Ingonyama")

Search

* [Introduction](/1.10.1)
* [ICICLE](/1.10.1/icicle/overview)

  + [Getting started](/1.10.1/icicle/introduction)
  + [ICICLE Provers](/1.10.1/icicle/integrations)
  + [Golang bindings](/1.10.1/icicle/golang-bindings)

    - [MSM](/1.10.1/icicle/golang-bindings/msm)

      * [MSM pre computation](/1.10.1/icicle/golang-bindings/msm-pre-computation)
    - [NTT](/1.10.1/icicle/golang-bindings/ntt)
    - [Vector operations](/1.10.1/icicle/golang-bindings/vec-ops)
    - [Multi GPU Support](/1.10.1/icicle/golang-bindings/multi-gpu)
  + [Rust bindings](/1.10.1/icicle/rust-bindings)
  + [Primitives](/1.10.1/icicle/primitives/overview)
  + [Multi GPU Support](/1.10.1/icicle/multi-gpu)
  + [Supporting additional curves](/1.10.1/icicle/supporting-additional-curves)
  + [Google Colab Instructions](/1.10.1/icicle/colab-instructions)
* [ZK Containers](/1.10.1/ZKContainers)
* [Ingonyama Grant program](/1.10.1/grants)
* [Contributor guide](/1.10.1/contributor-guide)
* [Additional Resources](https://www.youtube.com/@ingo_ZK)

  + [YouTube](https://www.youtube.com/@ingo_ZK)
  + [Ingonyama Blog](https://www.ingonyama.com/blog)
  + [Ingopedia](https://www.ingonyama.com/ingopedia)
  + [GitHub](https://github.com/ingonyama-zk)

This is documentation for ICICLE Docs **1.10.1**, which is no longer actively maintained.

For up-to-date documentation, see the **[latest version](/)** (4.0.0).

* [ICICLE](/1.10.1/icicle/overview)
* [Golang bindings](/1.10.1/icicle/golang-bindings)
* [MSM](/1.10.1/icicle/golang-bindings/msm)
* MSM pre computation
Version: 1.10.1

On this page

MSM Pre computation
===================

To understand the theory behind MSM pre computation technique refer to Niall Emmart's [talk](https://youtu.be/KAWlySN7Hm8?feature=shared&t=1734).

### Supported curves[​](#supported-curves "Direct link to Supported curves")

`bls12-377`, `bls12-381`, `bn254`, `bw6-761`

Core package[​](#core-package "Direct link to Core package")
------------------------------------------------------------

MSM `PrecomputeBases`[​](#msm-precomputebases "Direct link to msm-precomputebases")
-----------------------------------------------------------------------------------

`PrecomputeBases` and `G2PrecomputeBases` exists for all supported curves.

#### Description[​](#description "Direct link to Description")

This function extends each provided base point (P)(P)(P) with its multiples (2lP,22lP,...,2(precomputefactor−1)⋅lP)(2^lP, 2^{2l}P, ..., 2^{(precompute\_factor - 1) \cdot l}P)(2lP,22lP,...,2(precomputef​actor−1)⋅lP), where (l)(l)(l) is a level of precomputation determined by the `precompute_factor`. The extended set of points facilitates faster MSM computations by allowing the MSM algorithm to leverage precomputed multiples of base points, reducing the number of point additions required during the computation.

The precomputation process is crucial for optimizing MSM operations, especially when dealing with large sets of points and scalars. By precomputing and storing multiples of the base points, the MSM function can more efficiently compute the scalar-point multiplications.

#### `PrecomputeBases`[​](#precomputebases "Direct link to precomputebases")

Precomputes bases for MSM by extending each base point with its multiples.

```
func PrecomputeBases(points core.HostOrDeviceSlice, precomputeFactor int32, c int32, ctx *cr.DeviceContext, outputBases core.DeviceSlice) cr.CudaError
```

##### Parameters[​](#parameters "Direct link to Parameters")

* **`points`**: A slice of the original affine points to be extended with their multiples.
* **`precomputeFactor`**: Determines the total number of points to precompute for each base point.
* **`c`**: Currently unused; reserved for future compatibility.
* **`ctx`**: CUDA device context specifying the execution environment.
* **`outputBases`**: The device slice allocated for storing the extended bases.

##### Example[​](#example "Direct link to Example")

```
cfg := GetDefaultMSMConfig()  
points := GenerateAffinePoints(1024)  
precomputeFactor := 8  
var precomputeOut core.DeviceSlice  
_, e := precomputeOut.Malloc(points[0].Size()*points.Len()*int(precomputeFactor), points[0].Size())  
  
err := PrecomputeBases(points, precomputeFactor, 0, &cfg.Ctx, precomputeOut)  
if err != cr.CudaSuccess {  
    log.Fatalf("PrecomputeBases failed: %v", err)  
}
```

#### `G2PrecomputeBases`[​](#g2precomputebases "Direct link to g2precomputebases")

This method is the same as `PrecomputeBases` but for G2 points. Extends each G2 curve base point with its multiples for optimized MSM computations.

```
func G2PrecomputeBases(points core.HostOrDeviceSlice, precomputeFactor int32, c int32, ctx *cr.DeviceContext, outputBases core.DeviceSlice) cr.CudaError
```

##### Parameters[​](#parameters-1 "Direct link to Parameters")

* **`points`**: A slice of G2 curve points to be extended.
* **`precomputeFactor`**: The total number of points to precompute for each base.
* **`c`**: Reserved for future use to ensure compatibility with MSM operations.
* **`ctx`**: Specifies the CUDA device context for execution.
* **`outputBases`**: Allocated device slice for the extended bases.

##### Example[​](#example-1 "Direct link to Example")

```
cfg := G2GetDefaultMSMConfig()  
points := G2GenerateAffinePoints(1024)  
precomputeFactor := 8  
var precomputeOut core.DeviceSlice  
_, e := precomputeOut.Malloc(points[0].Size()*points.Len()*int(precomputeFactor), points[0].Size())  
  
err := G2PrecomputeBases(points, precomputeFactor, 0, &cfg.Ctx, precomputeOut)  
if err != cr.CudaSuccess {  
    log.Fatalf("G2PrecomputeBases failed: %v", err)  
}
```

### Benchmarks[​](#benchmarks "Direct link to Benchmarks")

Benchmarks where performed on a Nvidia RTX 3090Ti.

| Pre-computation factor | bn254 size `2^20` MSM, ms. | bn254 size `2^12` MSM, size `2^10` batch, ms. | bls12-381 size `2^20` MSM, ms. | bls12-381 size `2^12` MSM, size `2^10` batch, ms. |
| --- | --- | --- | --- | --- |
| 1 | 14.1 | 82.8 | 25.5 | 136.7 |
| 2 | 11.8 | 76.6 | 20.3 | 123.8 |
| 4 | 10.9 | 73.8 | 18.1 | 117.8 |
| 8 | 10.6 | 73.7 | 17.2 | 116.0 |

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-1.10.1/icicle/golang-bindings/msm-pre-computation.md)

Last updated on **Aug 6, 2025**

[Previous

MSM](/1.10.1/icicle/golang-bindings/msm)[Next

NTT](/1.10.1/icicle/golang-bindings/ntt)

* [Supported curves](#supported-curves)
* [Core package](#core-package)
* [MSM `PrecomputeBases`](#msm-precomputebases)
  + [Benchmarks](#benchmarks)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.