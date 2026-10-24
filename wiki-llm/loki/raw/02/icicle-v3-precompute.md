MSM Pre computation | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[3.0.0](/3.0.0/icicle/golang-bindings/msm-pre-computation)

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

* [Introduction](/3.0.0)
* [ICICLE](/3.0.0/icicle/overview)

  + [Getting started](/3.0.0/icicle/getting_started)

    - [Build ICICLE from source](/3.0.0/icicle/build_from_source)
    - [Programmers guide](/3.0.0/icicle/programmers_guide/general)

      * [C++](/3.0.0/icicle/programmers_guide/cpp)
      * [Rust](/3.0.0/icicle/programmers_guide/rust)
      * [Go](/3.0.0/icicle/programmers_guide/go)
  + [Architecture overview](/3.0.0/icicle/arch_overview)

    - [CUDA Backend](/3.0.0/icicle/install_cuda_backend)
    - [Multi-Device Support](/3.0.0/icicle/multi-device)
    - [Build Your Own Backend](/3.0.0/icicle/build_your_own_backend)
  + [ICICLE libraries](/3.0.0/icicle/libraries)
  + [Compute API](/3.0.0/icicle/primitives/overview)

    - [MSM](/3.0.0/icicle/primitives/msm)
    - [NTT / ECNTT](/3.0.0/icicle/primitives/ntt)
    - [Vector operations](/3.0.0/icicle/primitives/vec_ops)
    - [Polynomials](/3.0.0/icicle/polynomials/overview)
    - [Golang bindings](/3.0.0/icicle/golang-bindings)

      * [MSM](/3.0.0/icicle/golang-bindings/msm)

        + [MSM pre computation](/3.0.0/icicle/golang-bindings/msm-pre-computation)
      * [NTT](/3.0.0/icicle/golang-bindings/ntt)
      * [EC-NTT](/3.0.0/icicle/golang-bindings/ecntt)
      * [Vector operations](/3.0.0/icicle/golang-bindings/vec-ops)
      * [Multi GPU Support](/3.0.0/icicle/golang-bindings/multi-gpu)
    - [Rust bindings](/3.0.0/icicle/rust-bindings)
  + [Migrate from ICICLE v2](/3.0.0/icicle/migrate_from_v2)
  + [Benchmarks](/3.0.0/icicle/benchmarks)
  + [FAQ and Troubleshooting](/3.0.0/icicle/faq_and_troubleshooting)
  + [Google Colab Instructions](/3.0.0/icicle/colab-instructions)
  + [ICICLE Provers](/3.0.0/icicle/integrations)
* [Ingonyama Grant program](/3.0.0/grants)
* [Contributor guide](/3.0.0/contributor-guide)
* Additional Resources

  + [YouTube](https://www.youtube.com/@ingo_ZK)
  + [Ingonyama Blog](https://www.ingonyama.com/blog)
  + [Ingopedia](https://www.ingonyama.com/ingopedia)
  + [GitHub](https://github.com/ingonyama-zk)

This is documentation for ICICLE Docs **3.0.0**, which is no longer actively maintained.

For up-to-date documentation, see the **[latest version](/)** (4.0.0).

* [ICICLE](/3.0.0/icicle/overview)
* [Compute API](/3.0.0/icicle/primitives/overview)
* [Golang bindings](/3.0.0/icicle/golang-bindings)
* [MSM](/3.0.0/icicle/golang-bindings/msm)
* MSM pre computation
Version: 3.0.0

On this page

MSM Pre computation
===================

To understand the theory behind MSM pre computation technique refer to Niall Emmart's [talk](https://youtu.be/KAWlySN7Hm8?feature=shared&t=1734).

Core package[​](#core-package "Direct link to Core package")
------------------------------------------------------------

### MSM PrecomputeBases[​](#msm-precomputebases "Direct link to MSM PrecomputeBases")

`PrecomputeBases` and `G2PrecomputeBases` exists for all supported curves.

#### Description[​](#description "Direct link to Description")

This function extends each provided base point (P)(P)(P) with its multiples (2lP,22lP,...,2(precomputefactor−1)⋅lP)(2^lP, 2^{2l}P, ..., 2^{(precompute\_factor - 1) \cdot l}P)(2lP,22lP,...,2(precomputef​actor−1)⋅lP), where (l)(l)(l) is a level of precomputation determined by the `precompute_factor`. The extended set of points facilitates faster MSM computations by allowing the MSM algorithm to leverage precomputed multiples of base points, reducing the number of point additions required during the computation.

The precomputation process is crucial for optimizing MSM operations, especially when dealing with large sets of points and scalars. By precomputing and storing multiples of the base points, the MSM function can more efficiently compute the scalar-point multiplications.

#### `PrecomputeBases`[​](#precomputebases "Direct link to precomputebases")

Precomputes points for MSM by extending each base point with its multiples.

```
func PrecomputeBases(bases core.HostOrDeviceSlice, cfg *core.MSMConfig, outputBases core.DeviceSlice) runtime.EIcicleError
```

##### Parameters[​](#parameters "Direct link to Parameters")

* **`bases`**: A slice of the original affine points to be extended with their multiples.
* **`cfg`**: The MSM configuration parameters.
* **`outputBases`**: The device slice allocated for storing the extended points.

##### Example[​](#example "Direct link to Example")

```
package main  
  
import (  
	"log"  
  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/core"  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/curves/bn254"  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/curves/bn254/msm"  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/runtime"  
)  
  
func main() {  
	// Load backend using env path  
	runtime.LoadBackendFromEnvOrDefault()  
	// Set Cuda device to perform  
	device := runtime.CreateDevice("CUDA", 0)  
	runtime.SetDevice(&device)  
  
	cfg := core.GetDefaultMSMConfig()  
	points := bn254.GenerateAffinePoints(1024)  
	cfg.PrecomputeFactor = 8  
	var precomputeOut core.DeviceSlice  
	precomputeOut.Malloc(points[0].Size(), points.Len()*int(cfg.PrecomputeFactor))  
  
	err := msm.PrecomputeBases(points, &cfg, precomputeOut)  
	if err != runtime.Success {  
		log.Fatalf("PrecomputeBases failed: %v", err)  
	}  
}
```

#### `G2PrecomputeBases`[​](#g2precomputebases "Direct link to g2precomputebases")

This method is the same as `PrecomputePoints` but for G2 points. Extends each G2 curve base point with its multiples for optimized MSM computations.

```
func G2PrecomputeBases(bases core.HostOrDeviceSlice, cfg *core.MSMConfig, outputBases core.DeviceSlice) runtime.EIcicleError
```

##### Parameters[​](#parameters-1 "Direct link to Parameters")

* **`bases`**: A slice of the original affine points to be extended with their multiples.
* **`cfg`**: The MSM configuration parameters.
* **`outputBases`**: The device slice allocated for storing the extended points.

##### Example[​](#example-1 "Direct link to Example")

```
package main  
  
import (  
	"log"  
  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/core"  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/curves/bn254/g2"  
	"github.com/ingonyama-zk/icicle/v3/wrappers/golang/runtime"  
)  
  
func main() {  
	// Load backend using env path  
	runtime.LoadBackendFromEnvOrDefault()  
	// Set Cuda device to perform  
	device := runtime.CreateDevice("CUDA", 0)  
	runtime.SetDevice(&device)  
  
	cfg := core.GetDefaultMSMConfig()  
	points := g2.G2GenerateAffinePoints(1024)  
	cfg.PrecomputeFactor = 8  
	var precomputeOut core.DeviceSlice  
	precomputeOut.Malloc(points[0].Size(), points.Len()*int(cfg.PrecomputeFactor))  
  
	err := g2.G2PrecomputeBases(points, &cfg, precomputeOut)  
	if err != runtime.Success {  
		log.Fatalf("PrecomputeBases failed: %v", err)  
	}  
}
```

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-3.0.0/icicle/golang-bindings/msm-pre-computation.md)

Last updated on **Aug 6, 2025**

[Previous

MSM](/3.0.0/icicle/golang-bindings/msm)[Next

NTT](/3.0.0/icicle/golang-bindings/ntt)

* [Core package](#core-package)
  + [MSM PrecomputeBases](#msm-precomputebases)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.