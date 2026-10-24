User Guide: ZK Acceleration of Gnark using ICICLE

[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/637f69e01be00c0226021321_Ingonyama-logo-white.svg)](/old-home-4)

[Snark Chocolate](https://open.spotify.com/show/73VPmr716AjCJRNsP86J1g)[ICICLE](https://dev.ingonyama.com/icicle/overview)

Resources

### Ingopedia: A Repo for All Things ZK

Ingonyama's community-driven hub for all things related to Zero Knowledge Proofs (ZKPs). From beginners to experts, this comprehensive collection is a one-stop-shop for learning and collaboration.

[Go to Ingopedia](https://www.ingonyama.com/ingopedia/communityguide)

[BLOG](/old-blogs)

[VIDEOS](/videos/videos-zk-accelerate-bangkok)

[AUTHORS](/authors)

[MEDIA KIT](/media-kit)

Company

[ABOUT US](/about-us)

[CAREERS](/careers)

[SWAG](/swag)

[EVENTS](/events)

![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/65d6c32ef5be899484d39719_search-icon-white.svg)

![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/64b13138ac248f8f0c418c87_bell_outline.svg)

[#### Events

#### ZK Accelerate Bangkok](https://www.ingonyama.com/blog/zk-accelerate-bangkok-videos-photos-recap)

[#### Latest Blog

#### Hardware-friendliness of HyperPlonk, Part2](/oldblogs/hardware-friendliness-of-hyperplonk-part2)

[#### Latest Podcast](https://open.spotify.com/show/73VPmr716AjCJRNsP86J1g)

[#### Latest Video

![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/63ee13ce04a985e5c7bf464c_play.svg)

#### ZK Accelerate Bangkok](/videos/videos-zk-accelerate-bangkok)

![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/66f0e94abb2895b5e1da87b5_dark-mode-icon.svg)![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/66f0e94a4d2fd5f82afc72ce_light-mode-icon.svg)

[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/64f86263332495b34b7c9bd1_ri_twitter-x-fill.svg)](https://twitter.com/Ingo_zk)[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/669dc2be701d2deee0fb9e33_ingo-in-logo-white-2.svg)](https://www.linkedin.com/company/ingonyama/)[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/635180603629873fa2079364_github.svg)](https://github.com/ingonyama-zk)[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/67244da0583fcf7e759378ab_spotify-icon.svg)](https://open.spotify.com/show/73VPmr716AjCJRNsP86J1g)[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/635fef9f7182743cd672690f_Asset%202.svg)](https://www.youtube.com/@ingo_ZK)[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/6362929e9d33902572a317d1_Asset%205.svg)](https://medium.com/@ingonyama)[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/6410635f057a01370e8fe387_discord-icon.png)](https://discord.gg/qkBVXBpFc4)

User Guide: ZK Acceleration of Gnark using ICICLE
=================================================

Published on:

Aug 1, 2023

### A practical way to generate FAST Zero-Knowledge Proofs today

‍

TL;DR:
------

In this blogpost we present the motivations, background, architecture, integration, and performance of ZK acceleration of [Gnark](https://twitter.com/gnark_team) using [ICICLE](https://github.com/ingonyama-zk/icicle).

**Acceleration** for ZKP seems to be a field being developed by wizards. Companies are presenting complex FPGA/ASIC designs that remain very far away from production, let alone developing integrations for existing infrastructure.

GPUs however are already being used to accelerate ZKPs (examples: Aleo, Filecoin). GPUs are characterized by competitive pricing and the benefits of a high-volume supply chain, making them readily available. GPUs also provide massive parallelization out of the box and crucially, have an abundance of developer frameworks, such as CUDA. These factors make GPUs a prime platform for ZK acceleration.

Motivations for Developing ICICLE
---------------------------------

* SNARK proof generation is slow
* Modern SNARK frameworks (Rapidsnark, Halo2, Gnark, Arkworks only support CPU acceleration
* Hardware accelerators are easy to obtain (e.g cloud FPGA, gaming GPU) and relatively inexpensive
* It was shown that HW acceleration can successfully operate on ZK primitives e.g MSM, NTT (see [Zprize](https://twitter.com/z_prize))
* The logical next step is to integrate HW-based acceleration into SNARK frameworks

Introduction to ICICLE
----------------------

We are excited to present [ICICLE](https://medium.com/@ingonyama/introducing-icicle-an-open-source-gpu-library-for-zero-knowledge-acceleration-aad8c35b7f0a) — an open-source library for Zero-Knowledge Proof Acceleration on CUDA-enabled GPUs.

Who is ICICLE for?
------------------

ICICLE is for developers who need to generate FAST Zero-Knowledge Proofs. It is also for protocol developers who want a GPU-accelerated crypto library without having to implement the low-level primitives from scratch. ICICLE is a first-of-its-kind library for ZKP, distributing acceleration to ZK developers like never before.

Why use ICICLE?
---------------

Unlike other ZK GPU acceleration projects, ICICLE is not application specific. ICICLE implements primitives such as MSM, NTT, polynomial operations, common hash functions, and more that are prevalent throughout ZK protocol implementations.

ICICLE provides users with an easy-to-use library that can be implemented into any Rust or Golang-based project and immediately (with a little engineering work) see massive performance improvements.

This article will guide you through our ICICLE <> Gnark integration. We will explore some of the key integration learning points from our experience developing ICICLE and integrating it into Gnark.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a67fc7bb2f6795d3aa8_0*uQJ1INq-lBkN3a3d.png)

Overall System Design
---------------------

ICICLE implements all primitives as CUDA kernels. These kernels construct our main APIs (illustrated below). In order to cater for simple integration and minimize developer overhead we maintain two high-level wrapper APIs, one in Rust and the other in Golang. These APIs abstract away memory management and provide easy access to the core API functions. Developers may still access the low-level CUDA API directly.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a66b7ce9b886e5dc3a2_0*WxWigJ8GOLmGjmCR.jpeg)

ICICLE software stack overview

To illustrate the relation between CUDA Kernels and higher level CUDA API, we can have a look at *commit\_cuda\_bls12\_377*. This C++ code calls a [bucket msm](https://github.com/ingonyama-zk/icicle/blob/2a3f5a258a7ef2af36832b5f5e4ef43f3538f104/icicle/appUtils/msm/msm.cu#L155C6-L155C23) algorithm we implemented, and within this CUDA code we call kernel’s like this one for [example](https://github.com/ingonyama-zk/icicle/blob/2a3f5a258a7ef2af36832b5f5e4ef43f3538f104/icicle/appUtils/msm/msm.cu#L88). These kernels represent parts of the algorithm that are meant to be executed in parallel.

‍

```
int commit_cuda_bls12_377(BLS12_377::projective_t* d_out, BLS12_377::scalar_t* d_scalars,
  BLS12_377::affine_t* d_points, size_t count, size_t device_id = 0, cudaStream_t stream = 0)

    try

    large_msm(d_scalars, d_points, count, d_out, true, false, stream);
    cudaStreamSynchronize(stream);
    return 0;

    catch (const std::runtime_error &ex)

    printf("error %s", ex.what());
    return -1;
```

(Commit\_cuda\_bls12\_377 — A high-level API)

The Golang/Rust APIs, such as the [goicicle](https://github.com/ingonyama-zk/icicle/tree/dev/goicicle) library, wrap the high-level CUDA API and simplify integration into your existing codebase. There is no need to create C pointers, manage edge cases, or debug bindings; we have all that covered.

‍

```
func MsmG2BN254(out *G2Point, points []G2PointAffine, scalars []ScalarField, device_id int) (*G2Point, error) {
 if len(points) != len(scalars) {
  return nil, errors.New("error on: len(points) != len(scalars)")
 }

 pointsC := (*C.BN254_g2_affine_t)(unsafe.Pointer(&points[0]))
 scalarsC := (*C.BN254_scalar_t)(unsafe.Pointer(&scalars[0]))
 outC := (*C.BN254_g2_projective_t)(unsafe.Pointer(out))

 ret := C.msm_g2_cuda_bn254(outC, pointsC, scalarsC, C.size_t(len(points)), C.size_t(device_id))

 if ret != 0 {
  return nil, fmt.Errorf("msm_g2_cuda_bn254 returned error code: %d", ret)
 }

 return out, nil
}
```

([Golang wrapper for msm on BN254](https://github.com/ingonyama-zk/icicle/blob/2f21ec4aa7f47a2e9f75d3188ccf7b15fb929541/goicicle/curves/bn254/msm.go#L84C1-L84C1))

Memory management
-----------------

A fundamental guideline when using ICICLE involves minimizing data transfer frequency between the host (CPU) and the device (GPU). Data transfer between devices causes latency and will hurt performance. ICICLE is designed to encourage the use of on-device memory.

We encourage developers to structure their programs in a way that loads data onto the device once, performs as many operations as possible, and then retrieves the data back to the host.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a66d717007c658059b7_0*-ZJq_VRzBUeRv3rV.jpeg)

Host to Device data transfer

Setting up ICICLE with Golang
-----------------------------

ICICLE can be installed into your GO project simply by calling:

```
go get https://pkg.go.dev/github.com/ingonyama-zk/icicle/goicicle
```

However there is one more extra step — we must compile a shared library from the CUDA code. This shared library will be linked to our GO library.

‍

```
/*
#cgo LDFLAGS: -L/path/to/shared/libs -lbn254 -lbls12_381 -lbls12_377
#include "icicle.h" // make sure you use the correct header file(s)
*/
import "C"

func main() {
    // Now you can call the C functions from the ICICLE libraries.
    // Note that C function calls are prefixed with 'C.' in Go code.
}
```

( An example of linking — we have this covered for you)

Setting this up is straightforward, all you need to do is compile the required shared library and export it to LD\_LIBRARY\_PATH=<shared\_lib\_dir/>

1. Navigate to the location go get installed the ICICLE package (or clone the [icicle project](https://github.com/ingonyama-zk/icicle/tree/dev)).
2. Execute [goicicle/setup.sh](https://github.com/ingonyama-zk/icicle/blob/2f21ec4aa7f47a2e9f75d3188ccf7b15fb929541/goicicle/setup.sh#L1), and call it sh setup.sh <the\_curve\_you\_want\_to\_compile>. If you wish to compile all curves simply call sh setup.sh.
3. Make sure LD\_LIBRARY\_PATH contains your compiled shared libs path.  
   If LD\_LIBRARY\_PATH does not contain the path to your shared libs directory, export it manually export LD\_LIBRARY\_PATH=<shared\_lib\_dir/>

You can also use our [MakeFile](https://github.com/ingonyama-zk/icicle/blob/dev/goicicle/Makefile), documented more thoroughly [here](https://github.com/ingonyama-zk/icicle/blob/dev/goicicle/README.md#icicle-cuda-to-golang-binding-guide).

GOICICLE Primitives
-------------------

Initially we are supporting three curves BN254, BLS12–377, and BLS12–381. We chose these curves due to their popularity in ZK systems.

Pre-generated constants
-----------------------

Many of our Curves contain pre-generated constants, with the goal being to prevent these constants from being generated during runtime, as it will hurt performance.

To make it easy to add new curve support we created [a script](https://github.com/ingonyama-zk/icicle/blob/dev/curve_parameters/new_curve_script.py) that will generate for you all the necessary files to support a new curve (including CUDA code and Rust/Golang wrapper code).

Detailed instructions can be found [here](https://github.com/ingonyama-zk/icicle/blob/2f21ec4aa7f47a2e9f75d3188ccf7b15fb929541/README.md?plain=1#L101).

Packages
--------

Curves are separated into packages. Each package contains Points, Fields, and algorithms such as MSM, NTT, polynomial operations, hash functions, and more. These packages are independent of one another. However, they share the same interface, and working with them should be the same.

Curves, fields, and points
--------------------------

ICICLE represents all Fields and Points in non-montgomery (short Weierstrass) format.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a6612ebdfe536107bdc_0*_91rbC5j9otKVLxT.png)

Golang wrapper structure

Fields
------

Our Field structs below are modeled after the CUDA fields so that it will be easy to convert between the two. In CUDA many of the field and curve [parameters are pre-generated](https://github.com/ingonyama-zk/icicle/blob/2a3f5a258a7ef2af36832b5f5e4ef43f3538f104/icicle/curves/bn254/params.cuh#L142) and loaded during runtime.

‍

```
const SCALAR_SIZE = 
const BASE_SIZE = 

type G1ScalarField struct {  
  s [SCALAR_SIZE]uint32 
}
  
type G1BaseField struct {  
  s [BASE_SIZE]uint32 
}
```

( Depending on the curve scalar and base size vary)

Points
------

We chose to use Projective and Affine coordinate notations for our points. Our algorithms are implemented from Affine (Affine input is converted to projective internally) to Projective.

‍

```
type G1ProjectivePoint struct { 
  x, y, z G1BaseField 
}

  type G1PointAffine struct {  
  x, y G1BaseField 
}
```

In order to improve the compatibility of ICICLE with the existing [Golang crypto ecosystem](https://github.com/ConsenSys/gnark-crypto), we include methods such as toGnarkand fromGnarkas well as toJac (we plan to extract these compatibility functions to separate packages in the near future). The aim is to keep integration simple and fast.

We also support G2 Points and extension fields; G2 points have their own structs and methods.

‍

```
type ExtentionField struct { 
  A0, A1 [4]uint64
}

type G2PointAffine struct { x, y ExtentionField } 
type G2Point struct { x, y, z ExtentionField }
```

G2 Points are also compatible with Gnark G2 points.

Algorithms
----------

We currently support MSM, NTT, interpolation, Montgomery conversion, and a variety of arithmetics operations. We will cover most of these when reviewing the ICICLE / Gnark integration.

GOICICLE CUDA interface
-----------------------

The [CUDA runtime API](https://docs.nvidia.com/cuda/cuda-runtime-api/modules.html#modules) interface gives us access to the CUDA API through a Go interface.

At the time of writing this article, we only implemented a handful of useful memory management methods (you are welcome to add more if you find any are missing).

[CudaMalloc](https://github.com/ingonyama-zk/icicle/blob/2f21ec4aa7f47a2e9f75d3188ccf7b15fb929541/goicicle/goicicle.go#L18)
---------------------------------------------------------------------------------------------------------------------------

Allocate memory on the device according to a given size. It returns a pointer to the allocated memory.

[CudaFree](http://cudafree/)
----------------------------

Given an unsafe.Pointer it frees the memory allocated on the device.

[CudaMemCpyHtoD](https://github.com/ingonyama-zk/icicle/blob/2f21ec4aa7f47a2e9f75d3188ccf7b15fb929541/goicicle/goicicle.go#L34)
-------------------------------------------------------------------------------------------------------------------------------

Copies memory from host (CPU) to device (GPU). It expects unsafe.Pointer so you should use CudaMalloc to allocate memory before calling CudaMemCpyHtoD.

[CudaMemCpyDtoH](https://github.com/ingonyama-zk/icicle/blob/2f21ec4aa7f47a2e9f75d3188ccf7b15fb929541/goicicle/goicicle.go#L42)
-------------------------------------------------------------------------------------------------------------------------------

Copies memory from the device (GPU) to the host (CPU). It expects unsafe.Pointerso you should make sure to allocate memory using CudaMalloc.

Gnark and ICICLE: A match made in heaven :)
-------------------------------------------

Let’s review the [Gnark <> ICICLE integration](https://github.com/ingonyama-zk/gnark).

Architecture overview
---------------------

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a6663dd4e2f5e95acbb_0*mT_Cx0v16WZ9CmdX.png)

Gnark in its current form primarily relies on a conventional multicore CPU for all computations. However, we have implemented a design enhancement that incorporates a GPU into Gnark.

GOICICLE is used to transfer data between the Host and the Device. The ICICLE CUDA library is where we implement the actual acceleration and computation algorithms.

By offloading compute-intensive tasks to the GPU, we achieve a form of heterogeneous computing in which the CPU and GPU work together to optimize performance for different aspects of the application. This approach allows us to identify specific parts of the Gnark Groth16 protocol that may be significantly improved and accelerated with GPUs, without changing the rest of the protocol.

Gnark Overview
--------------

Gnark facilitates two provers for now: Plonk and Groth16. We focus on the Groth16 prover, and Plonk support is in the works.

Gnark facilitates two main functionalities: first, writing circuits in Golang, and second, generating zk-proofs and verifying them.

The process of generating a proof consists of multiple steps, the first being the [setup phase](https://github.com/ingonyama-zk/gnark/blob/5ded21ab9f98637c5eb2ae6b761fc676c0e5a3ed/backend/groth16/bn254/setup.go), this phase constructs the SRS keys and sets up many variables we will need when generating the proof. In the second phase, we generate the proof, this phase involves the most computationally intensive operations, such as MSM and FFT.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a660e4b5335ba24651a_0*BruazV2G89tw-Dy_.png)

Generating and verifying a proof in Gnark would look something like this:

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a6654cac71068df62c0_0*513g475tWHoJY5dU.png)

Where Vk and Pk are the verification and proving keys generated during setup, and r1cs is a constraint system generated from a circuit written using Gnarks circuit API.

Tackling the Setup
------------------

As discussed above, to lower latency we want to load data into the device memory as early as possible. During the setup we load G1, G2, [Twiddle factors](https://github.com/ingonyama-zk/icicle/blob/2a3f5a258a7ef2af36832b5f5e4ef43f3538f104/icicle/appUtils/ntt/ntt.cuh#L15) and Coset values.

‍

```
G1Device struct {
  A, B, K, Z unsafe.Pointer
}


DomainDevice struct {
  Twiddles, TwiddlesInv unsafe.Pointer
  CosetTable, CosetTableInv unsafe.Pointer
}
```

([Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/setup_gpu.go#L59))

To load values into device memory we use CUDA functions CudaMalloc and CudaMemCpyHtoD:

‍

```
pointsBytesA := len(pk.G1.A) * fp.Bytes * 2 // calculate size of G1.A in bytes
a_d, _ := goicicle.CudaMalloc(pointsBytesA) // allocate device memory 

// copy into device memory 
goicicle.CudaMemCpyHtoD[curve.G1Affine](a_d, pk.G1.A, pointsBytesA) 

// convert from montgomery to none montgomery format
icicle.AffinePointFromMontgomery(a_d, len(pk.G1.A)) pk.G1Device.A = a_d
```

([Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/setup_gpu.go#L423C30-L423C51))

AffinePointFromMontgomery performs Montgomery conversions on GPU; if we didn’t do this during the setup phase we would need to perform these conversions during the prover runtime.

Similar actions are performed for G2Device, Domain, Twiddles, Twiddles Inv and CosetTables. All of these values are stored in GPU memory and accessed in the Proving stage.

The prover
----------

The prover contains three main calculations: NTT, MSM, and polynomial operations.

NTT
---

In the original Gnark implementation NTT’s are performed sequentially on CPU.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a66e9235894314a62d4_0*ZhNThoYxCiCqr1O6.png)

[Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/prove.go#L356)

Using ICICLE we pair NTT and INTT and run three of them in parallel on the GPU.

‍

```
computeInttNttDone := make(chan error, 1)

// define a function that prefomrms NTT and INTT on device
computeInttNttOnDevice := func (devicePointer unsafe.Pointer) {
  a_intt_d := INttOnDevice(devicePointer,     pk.DomainDevice.TwiddlesInv, nil, n, sizeBytes, false)  
        NttOnDevice(devicePointer, a_intt_d, pk.DomainDevice.Twiddles, pk.DomainDevice.CosetTable, n, n, sizeBytes, true)
  computeInttNttDone <- nil
}
 
// compute NTT/INTT for a,b,c in parallel 
 go computeInttNttOnDevice(a_device)
 go computeInttNttOnDevice(b_device)
 go computeInttNttOnDevice(c_device)

// wait for routines to complete 
 _, _, _ = <- computeInttNttDone, <- computeInttNttDone, <- computeInttNttDone
```

([Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/prove_gpu.go#L338))

Pairing both INTT and NTT fully utilized the GPU and greatly improves performance.

Another point worth mentioning is the polynomial operations, which occur after performing the INTT and NTT pairs.

‍

```
// above we preforme the NTT / INTT pairs

var den, one fr.Element
 one.SetOne()
 den.Exp(domain.FrMultiplicativeGen, big.NewInt(int64(domain.Cardinality)))
 den.Sub(&den, &one).Inverse(&den)

 // h = ifft_coset(ca o cb - cc)
 // reusing a to avoid unnecessary memory allocation
 utils.Parallelize(n, func(start, end int) {
  for i := start; i < end; i++ {
   a[i].Mul(&a[i], &b[i]).
    Sub(&a[i], &c[i]).
    Mul(&a[i], &den)
  }
 })

// ifft_coset
domain.FFTInverse(a, fft.DIF, fft.OnCoset())
```

Since ICICLE also supports polynomial operations, we can simply implement this and avoid having to transfer data back from device to host, costing us precious runtime.

‍

```
func PolyOps(a_d, b_d, c_d, den_d unsafe.Pointer, size int) {
 // preform a[i].Mul(&a[i], &b[i]) on GPU
 ret := icicle.VecScalarMulMod(a_d, b_d, size)

 if ret != 0 {
  fmt.Print("Vector mult a*b issue")
 }

 // preform Sub(&a[i], &c[i]).
 ret = icicle.VecScalarSub(a_d, c_d, size)

 if ret != 0 {
  fmt.Print("Vector sub issue")
 }
 
 // preform Mul(&a[i], &den)
 ret = icicle.VecScalarMulMod(a_d, den_d, size)

 if ret != 0 {
  fmt.Print("Vector mult a*den issue")
 }
 
 return
}
```

([Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/icicle_wrapper_gpu.go#L69))

This is a good example of how easy it is to use ICICLE to accelerate existing codebases without having to do a lot of work.

MSM
---

MSM’s are a bit of a different story since they are massive and consume many resources.

At the time of writing this article, we implemented an MSM bucket algorithm (similar to the one from ZPrize).

Due to the large size of MSM we found that they must consume all the GPU threads, thus interestingly enough it’s more efficient to run MSM in serial on GPU.

‍

```
// schedule our proof part computations

computeKRS() // execute 2 MSM
computeAR1() // execute 1 MSM
computeBS1() // execute 1 MSM

// execute 1 G2 MSM
if err := computeBS2(); err != nil {
 return nil, err
}
```

([Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/prove_gpu.go#L264))

MsmOnDevice uses many of the structures we initialized and loaded into device memory during setup time.

‍

```
computeBS1 := func() {
  <-chWireValuesB
  
// pk.G1Device.B has been loaded into device and converted during // setup phase 
  bs1, _, _ = MsmOnDevice(wireValuesBDevice.p, pk.G1Device.B, wireValuesBDevice.size, true)

  bs1.AddMixed(&pk.G1.Beta)
  bs1.AddMixed(&deltas[1])
}
```

([Source](https://github.com/ingonyama-zk/gnark/blob/0fa587a47b26ee30a7b20526748357de1b09605d/backend/groth16/bn254/prove_gpu.go#L191))

Benchmarks
----------

We ran our benchmarks on a setup containing:

GPU — RTX 3090 TI 24 GB,

CPU — I9 12900K

The circuits we benchmarked have a constraint size of > 2²³.

Our benchmarks achieve impressive results, and display how ICICLE can be used to accelerate a protocol with very little engineering effort.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a6663dd4e2f5e95ad0c_0*OBIiBAuGg8U4-jRr.png)

All values are in milliseconds

When reviewing the NTT results we see that times have been improved by x8. This is specifically due to our parallelization of NTT/INTT pairs. Polymorphic operations have been brought down to below 1 millisecond of compute time.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a66451d801bec33ea34_0*seu_6Z4dQKP5wwcP.png)

Total NTT/INTT time

Total computeHtime (460 ms) also includes Covert / Copy of data to the device. These times could theoretically be reduced by a more comprehensive integration between Gnark and ICICLE. The current implementation still requires conversions between Montgomery representations (Gnark-crypto works in Montgomery and we work in non-Montgomery).

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a6688ee4f79978cc16c_0*89EW8YoFPn9jBatF.png)

Total NTT/INTT + additional overheads such as covert / copy

We achieved a 3X improvement with MSM speeds overall. MSM unlike NTT consumes the whole GPU due to the size of computation; simply scaling the number of GPUs in your system would double the speed. A setup with 2 GPUs for example would achieve a 6X easily.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a6a20952c043fab8a1d_0*Xnm2A5qCDikb2zXD.png)

Comparing individual MSM runtimes

If you notice in the chart above when comparing MSM to MSM we see that there is a 10X improvement. While this is true, our MSMs are executed sequentially while Gnark’s implementation executes multiple MSMs in parallel. This is due to the reason stated above where our MSM implementation consumes the entire GPU.

So when we compare the full ICICLE Groth16 MSM runtime we see a 3X improvement, despite individual MSM being significantly faster.

![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/64ca1a67c6fc8e1f6fcca912_0*3KYWXW1DWhDJxsKz.png)

What is next?
-------------

ICICLE is still in its infancy. We have a road map of many new features in the works, from new algorithms to multi-GPU support. You can follow developments in [our github](https://github.com/ingonyama-zk/icicle/issues).

We also have much to explore in the realms of deployment, benchmarking, and support for other protocols, Plonk being the natural next choice.

As usual, contributions, feedback, and questions are welcome — join the conversation with our team in the [Ingonyama Discord channel](https://discord.gg/qkBVXBpFc4)!

This subject matter was also covered in [a talk](https://www.youtube.com/watch?v=IsgD1hkp52g) by Ingonyama CEO Omer Shlomovits at the zkParis event in 2023, part of Ethcc.

Follow Ingonyama
----------------

Twitter: <https://twitter.com/Ingo_zk>

YouTube: <https://www.youtube.com/@ingo_zk>

GitHub: <https://github.com/ingonyama-zk>

LinkedIn: <https://www.linkedin.com/company/ingonyama>

Join us: <https://www.ingonyama.com/careers>

light

### Written by

[![](https://cdn.prod.website-files.com/63970f25aa7b42e284492d52/6453ab5903e6457699248397_profile%20-%20Immanuel%20S.jpg)

Immanuel Segol](/authors/immanuel-segol)

### Table of Contents

### Want to discuss further?

Ingonyama is commited to developing hardware for a private future using Zero Knowledge Proofs.

[Get in touch](#footer-get-in-touch)

[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/63d7eb697cf3316da634b90f_Generic_Feed-icon.svg)

Get our RSS feed](/blog/rss.xml)

[![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/64243edc9d5c2c67ebb5c3e8_ingonyama-logo-full-svg.svg)](/old-home-3)![](https://cdn.prod.website-files.com/63218944607c93b965d5c20a/6579aaf32ec4b3ef208ef478_lion-helmet-white-smallx2.png)

Products

[ICICLE](https://dev.ingonyama.com/icicle/overview)

Resources

[Ingopedia](https://www.ingonyama.com/ingopedia/communityguide)[Blogs](/old-blogs)[Videos](/videos)

Company

[About Us](/about-us)[Careers](/careers)[LinkedIn](https://www.linkedin.com/company/ingonyama/)

Get in touch

Thank you! Your submission has been received!

Oops! Something went wrong while submitting the form.

© 2024 Ingonyama. All rights reserved