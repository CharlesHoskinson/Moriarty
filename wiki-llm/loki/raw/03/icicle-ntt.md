NTT | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[4.0.0](/api/rust-bindings/ntt)

* [4.0.0](/api/rust-bindings/ntt)
* [3.9.0](/3.9.0/api/rust-bindings/ntt)
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

* Rust Bindings
* NTT
Version: 4.0.0

On this page

NTT
===

NTT API overview[​](#ntt-api-overview "Direct link to NTT API overview")
------------------------------------------------------------------------

```
pub fn ntt<T, F>(  
    input: &(impl HostOrDeviceSlice<T> + ?Sized),  
    dir: NTTDir,  
    cfg: &NTTConfig<F>,  
    output: &mut (impl HostOrDeviceSlice<T> + ?Sized),  
) -> Result<(), IcicleError>;  
  
pub fn ntt_inplace<T, F>(  
    inout: &mut (impl HostOrDeviceSlice<T> + ?Sized),  
    dir: NTTDir,  
    cfg: &NTTConfig<F>,  
) -> Result<(), IcicleError>
```

* **`input`** - buffer to read the inputs of the NTT from.
* **`dir`** - whether to compute forward or inverse NTT.
* **`cfg`** - config used to specify extra arguments of the NTT.
* **`output`** - buffer to write the NTT outputs into. Must be of the same size as input.

The `input` and `output` buffers can be on device or on host. Being on host means that they will be transferred to device during runtime.

### NTT Config[​](#ntt-config "Direct link to NTT Config")

```
pub struct NTTConfig<S: IntegerRing> {  
    pub stream_handle: IcicleStreamHandle,  
    pub coset_gen: S,  
    pub batch_size: i32,  
    pub columns_batch: bool,  
    pub ordering: Ordering,  
    pub are_inputs_on_device: bool,  
    pub are_outputs_on_device: bool,  
    pub is_async: bool,  
    pub ext: ConfigExtension,  
}
```

The `NTTConfig` struct is a configuration object used to specify parameters for an NTT instance.

#### Fields[​](#fields "Direct link to Fields")

* **`stream_handle: IcicleStreamHandle`**: Specifies the stream (queue) to use for async execution
* **`coset_gen: S`**: Defines the coset generator used for coset (i)NTTs. By default, this is set to `S::one()`, indicating that no coset is being used.
* **`batch_size: i32`**: Determines the number of NTTs to compute in a single batch. The default value is 1, meaning that operations are performed on individual inputs without batching. Batch processing can significantly improve performance by leveraging parallelism in GPU computations.
* **`columns_batch`**: If true the function will compute the NTTs over the columns of the input matrix and not over the rows. Defaults to `false`.
* **`ordering: Ordering`**: Controls the ordering of inputs and outputs for the NTT operation. This field can be used to specify decimation strategies (in time or in frequency) and the type of butterfly algorithm (Cooley-Tukey or Gentleman-Sande). The ordering is crucial for compatibility with various algorithmic approaches and can impact the efficiency of the NTT.
* **`are_inputs_on_device: bool`**: Indicates whether the input data has been preloaded on the device memory. If `false` inputs will be copied from host to device.
* **`are_outputs_on_device: bool`**: Indicates whether the output data is preloaded in device memory. If `false` outputs will be copied from host to device. If the inputs and outputs are the same pointer NTT will be computed in place.
* **`is_async: bool`**: Specifies whether the NTT operation should be performed asynchronously. When set to `true`, the NTT function will not block the CPU, allowing other operations to proceed concurrently. Asynchronous execution requires careful synchronization to ensure data integrity and correctness.
* **`ext: ConfigExtension`**: extended configuration for backend.

#### Example[​](#example "Direct link to Example")

```
// Setting Bn254 points and scalars  
println!("Generating random inputs on host for bn254...");  
let scalars = ScalarField::generate_random(size);  
let mut ntt_results = DeviceVec::<ScalarField>::device_malloc(size).unwrap();  
  
// constructing NTT domain  
initialize_domain(  
    ntt::get_root_of_unity::<ScalarField>(  
        size.try_into()  
            .unwrap(),  
    ),  
    &ntt::NTTInitDomainConfig::default(),  
)  
.unwrap();  
  
// Using default config  
let cfg = ntt::NTTConfig::<ScalarField>::default();  
  
// Computing NTT  
ntt::ntt(  
    HostSlice::from_slice(&scalars),  
    ntt::NTTDir::kForward,  
    &cfg,  
    &mut ntt_results[..],  
)  
.unwrap();
```

### NTT Domain[​](#ntt-domain "Direct link to NTT Domain")

Before performing NTT operations, it is mandatory to construct the domain as [explained here](/api/cpp/ntt#ntt-domain).
In rust, we have the following functions to construct, destruct the domain and retrieve a root of unity from it:

```
pub trait NTTDomain<S: IntegerRing> {  
    pub fn initialize_domain<S>(primitive_root: S, config: &NTTInitDomainConfig) -> Result<(), IcicleError>;  
    pub fn release_domain<S>() -> Result<(), IcicleError>;  
    pub fn get_root_of_unity<S>(max_size: u64) -> S;  
}
```

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-4.0.0/api/rust-bindings/ntt.md)

Last updated on **Aug 6, 2025**

[Previous

MSM](/api/rust-bindings/msm)[Next

ECNTT](/api/rust-bindings/ecntt)

* [NTT API overview](#ntt-api-overview)
  + [NTT Config](#ntt-config)
  + [NTT Domain](#ntt-domain)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.