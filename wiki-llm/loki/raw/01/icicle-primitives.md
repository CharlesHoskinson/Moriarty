NTT - Number Theoretic Transform | ICICLE Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

🚀 We just released **ICICLE v4** — featuring a more intuitive API with lattices. [Check out the migration guide](/start/integration-&-support/migrate_from_v3).

[![Ingonyama Logo](/img/icicledocslogo.png)![Ingonyama Logo](/img/icicledocslogo.png)](/)[Docs](/)[API](/apioverview)

[Go to ICICLE](https://github.com/ingonyama-zk/icicle)

[2.8.0](/2.8.0/icicle/primitives/ntt)

* [4.0.0](/)
* [3.9.0](/3.9.0)
* [3.8.0](/3.8.0/icicle/primitives/ntt)
* [3.7.0](/3.7.0/icicle/primitives/ntt)
* [3.6.0](/3.6.0/icicle/primitives/ntt)
* [3.5.0](/3.5.0/icicle/primitives/ntt)
* [3.4.0](/3.4.0/icicle/primitives/ntt)
* [3.3.0](/3.3.0/icicle/primitives/ntt)
* [3.2.0](/3.2.0/icicle/primitives/ntt)
* [3.1.0](/3.1.0/icicle/primitives/ntt)
* [3.0.0](/3.0.0/icicle/primitives/ntt)
* [2.8.0](/2.8.0/icicle/primitives/ntt)
* [1.10.1](/1.10.1/icicle/primitives/ntt)

[Community](#)

* [Discord](https://discord.gg/6vYrE7waPj)
* [LinkedIn](https://www.linkedin.com/company/ingonyama)
* [X/Twitter](https://x.com/Ingo_zk)
* [YouTube](https://www.youtube.com/@ingo_ZK)
* [Mailing List](https://wkf.ms/3LKCbdj)

[Leave Feedback](https://forms.monday.com/forms/7b51e0bdad766b71da8869704c301472?r=use1)

[![Ingonyama logo](/img/Ingologo.svg)](https://ingonyama.com "Visit Ingonyama")

Search

* [Introduction](/2.8.0)
* [ICICLE](/2.8.0/icicle/overview)

  + [Getting started](/2.8.0/icicle/introduction)
  + [ICICLE Core](/2.8.0/icicle/core)
  + [Primitives](/2.8.0/icicle/primitives/overview)

    - [MSM](/2.8.0/icicle/primitives/msm)
    - [NTT](/2.8.0/icicle/primitives/ntt)
    - [Keccak Hash](/2.8.0/icicle/primitives/keccak)
    - [Poseidon Hash](/2.8.0/icicle/primitives/poseidon)
    - [Poseidon2 Hash](/2.8.0/icicle/primitives/poseidon2)
  + [Polynomials](/2.8.0/icicle/polynomials/overview)
  + [Multi GPU Support](/2.8.0/icicle/multi-gpu)
  + [Golang bindings](/2.8.0/icicle/golang-bindings)
  + [Rust bindings](/2.8.0/icicle/rust-bindings)
  + [Google Colab Instructions](/2.8.0/icicle/colab-instructions)
  + [ICICLE Provers](/2.8.0/icicle/integrations)
* [ZK Containers](/2.8.0/ZKContainers)
* [Ingonyama Grant program](/2.8.0/grants)
* [Contributor guide](/2.8.0/contributor-guide)
* Additional Resources

  + [YouTube](https://www.youtube.com/@ingo_ZK)
  + [Ingonyama Blog](https://www.ingonyama.com/blog)
  + [Ingopedia](https://www.ingonyama.com/ingopedia)
  + [GitHub](https://github.com/ingonyama-zk)

This is documentation for ICICLE Docs **2.8.0**, which is no longer actively maintained.

For up-to-date documentation, see the **[latest version](/)** (4.0.0).

* [ICICLE](/2.8.0/icicle/overview)
* [Primitives](/2.8.0/icicle/primitives/overview)
* NTT
Version: 2.8.0

On this page

NTT - Number Theoretic Transform
================================

The Number Theoretic Transform (NTT) is a variant of the Fourier Transform used over finite fields, particularly those of integers modulo a prime number. NTT operates in a discrete domain and is used primarily in applications requiring modular arithmetic, such as cryptography and polynomial multiplication.

NTT is defined similarly to the Discrete Fourier Transform (DFT), but instead of using complex roots of unity, it uses roots of unity within a finite field. The definition hinges on the properties of the finite field, specifically the existence of a primitive root of unity of order NNN (where NNN is typically a power of 2), and the modulo operation is performed with respect to a specific prime number that supports these roots.

Formally, given a sequence of integers a0,a1,...,aN−1a\_0, a\_1, ..., a\_{N-1}a0​,a1​,...,aN−1​, the NTT of this sequence is another sequence of integers A0,A1,...,AN−1A\_0, A\_1, ..., A\_{N-1}A0​,A1​,...,AN−1​, computed as follows:

Ak=∑n=0N−1an⋅ωnkmod  pA\_k = \sum\_{n=0}^{N-1} a\_n \cdot \omega^{nk} \mod pAk​=n=0∑N−1​an​⋅ωnkmodp

where:

* NNN is the size of the input sequence and is a power of 2,
* ppp is a prime number such that p=kN+1p = kN + 1p=kN+1 for some integer kkk, ensuring that ppp supports the existence of NNNth roots of unity,
* ω\omegaω is a primitive NNNth root of unity modulo ppp, meaning ωN≡1mod  p\omega^N \equiv 1 \mod pωN≡1modp and no smaller positive power of ω\omegaω is congruent to 1 modulo ppp,
* kkk ranges from 0 to N−1N-1N−1, and it indexes the output sequence.

NTT is particularly useful because it enables efficient polynomial multiplication under modulo arithmetic, crucial for algorithms in cryptographic protocols and other areas requiring fast modular arithmetic operations.

There exists also INTT which is the inverse operation of NTT. INTT can take as input an output sequence of integers from an NTT and reconstruct the original sequence.

Using NTT[​](#using-ntt "Direct link to Using NTT")
---------------------------------------------------

### Supported Bindings[​](#supported-bindings "Direct link to Supported Bindings")

* [Golang](/2.8.0/icicle/golang-bindings/ntt)
* [Rust](/2.8.0/icicle/rust-bindings/ntt)

### Examples[​](#examples "Direct link to Examples")

* [Rust API examples](https://github.com/ingonyama-zk/icicle/blob/d84ffd2679a4cb8f8d1ac2ad2897bc0b95f4eeeb/examples/rust/ntt/src/main.rs#L1)
* [C++ API examples](https://github.com/ingonyama-zk/icicle/blob/d84ffd2679a4cb8f8d1ac2ad2897bc0b95f4eeeb/examples/c%2B%2B/ntt/example.cu#L1)

### Ordering[​](#ordering "Direct link to Ordering")

The `Ordering` enum defines how inputs and outputs are arranged for the NTT operation, offering flexibility in handling data according to different algorithmic needs or compatibility requirements. It primarily affects the sequencing of data points for the transform, which can influence both performance and the compatibility with certain algorithmic approaches. The available ordering options are:

* **`kNN` (Natural-Natural):** Both inputs and outputs are in their natural order. This is the simplest form of ordering, where data is processed in the sequence it is given, without any rearrangement.
* **`kNR` (Natural-Reversed):** Inputs are in natural order, while outputs are in bit-reversed order. This ordering is typically used in algorithms that benefit from having the output in a bit-reversed pattern.
* **`kRN` (Reversed-Natural):** Inputs are in bit-reversed order, and outputs are in natural order. This is often used with the Cooley-Tukey FFT algorithm.
* **`kRR` (Reversed-Reversed):** Both inputs and outputs are in bit-reversed order.
* **`kNM` (Natural-Mixed):** Inputs are provided in their natural order, while outputs are arranged in a digit-reversed (mixed) order. This ordering is good for mixed radix NTT operations, where the mixed or digit-reversed ordering of outputs is a generalization of the bit-reversal pattern seen in simpler, radix-2 cases.
* **`kMN` (Mixed-Natural):** Inputs are in a digit-reversed (mixed) order, while outputs are restored to their natural order. This ordering would primarily be used for mixed radix NTT

Choosing an algorithm is heavily dependent on your use case. For example Cooley-Tukey will often use `kRN` and Gentleman-Sande often uses `kNR`.

### Modes[​](#modes "Direct link to Modes")

NTT also supports two different modes `Batch NTT` and `Single NTT`

Deciding weather to use `batch NTT` vs `single NTT` is highly dependent on your application and use case.

#### Single NTT[​](#single-ntt "Direct link to Single NTT")

Single NTT will launch a single NTT computation.

Choose this mode when your application requires processing individual NTT operations in isolation.

#### Batch NTT Mode[​](#batch-ntt-mode "Direct link to Batch NTT Mode")

Batch NTT allows you to run many NTTs with a single API call. Batch NTT mode can significantly reduce read/write times as well as computation overhead by executing multiple NTT operations in parallel. Batch mode may also offer better utilization of computational resources (memory and compute).

Supported algorithms[​](#supported-algorithms "Direct link to Supported algorithms")
------------------------------------------------------------------------------------

Our NTT implementation supports two algorithms `radix-2` and `mixed-radix`.

### Radix 2[​](#radix-2 "Direct link to Radix 2")

At its core, the Radix-2 NTT algorithm divides the problem into smaller sub-problems, leveraging the properties of "divide and conquer" to reduce the overall computational complexity. The algorithm operates on sequences whose lengths are powers of two.

1. **Input Preparation:**
   The input is a sequence of integers a0,a1,…,aN−1, where Na\_0, a\_1, \ldots, a\_{N-1}, \text{ where } Na0​,a1​,…,aN−1​, where N is a power of two.
2. **Recursive Decomposition:**
   The algorithm recursively divides the input sequence into smaller sequences. At each step, it separates the sequence into even-indexed and odd-indexed elements, forming two subsequences that are then processed independently.
3. **Butterfly Operations:**
   The core computational element of the Radix-2 NTT is the "butterfly" operation, which combines pairs of elements from the sequences obtained in the decomposition step.

   Each butterfly operation involves multiplication by a "twiddle factor," which is a root of unity in the finite field, and addition or subtraction of the results, all performed modulo the prime modulus.

   Xk=(Ak+Bk⋅Wk)mod  p X\_k = (A\_k + B\_k \cdot W^k) \mod pXk​=(Ak​+Bk​⋅Wk)modp

   XkX\_kXk​ - The output of the butterfly operation for the kkk-th element

   AkA\_kAk​ - an element from the even-indexed subset

   BkB\_kBk​ - an element from the odd-indexed subset

   ppp - prime modulus

   kkk - The index of the current operation within the butterfly or the transform stage

   The twiddle factors are precomputed to save runtime and improve performance.
4. **Bit-Reversal Permutation:**
   A final step involves rearranging the output sequence into the correct order. Due to the halving process in the decomposition steps, the elements of the transformed sequence are initially in a bit-reversed order. A bit-reversal permutation is applied to obtain the final sequence in natural order.

### Mixed Radix[​](#mixed-radix "Direct link to Mixed Radix")

The Mixed Radix NTT algorithm extends the concepts of the Radix-2 algorithm by allowing the decomposition of the input sequence based on various factors of its length. Specifically ICICLEs implementation splits the input into blocks of sizes 16, 32, or 64 compared to radix2 which is always splitting such that we end with NTT of size 2. This approach offers enhanced flexibility and efficiency, especially for input sizes that are composite numbers, by leveraging the "divide and conquer" strategy across multiple radices.

The NTT blocks in Mixed Radix are implemented more efficiently based on winograd NTT but also optimized memory and register usage is better compared to Radix-2.

Mixed Radix can reduce the number of stages required to compute for large inputs.

1. **Input Preparation:**
   The input to the Mixed Radix NTT is a sequence of integers a0,a1,…,aN−1a\_0, a\_1, \ldots, a\_{N-1}a0​,a1​,…,aN−1​, where NNN is not strictly required to be a power of two. Instead, NNN can be any composite number, ideally factorized into primes or powers of primes.
2. **Factorization and Decomposition:**
   Unlike the Radix-2 algorithm, which strictly divides the computational problem into halves, the Mixed Radix NTT algorithm implements a flexible decomposition approach which isn't limited to prime factorization.

   For example, an NTT of size 256 can be decomposed into two stages of 16×NTT1616 \times \text{NTT}\_{16}16×NTT16​, leveraging a composite factorization strategy rather than decomposing into eight stages of NTT2\text{NTT}\_{2}NTT2​. This exemplifies the use of composite factors (in this case, 256=16×16256 = 16 \times 16256=16×16) to apply smaller NTT transforms, optimizing computational efficiency by adapting the decomposition strategy to the specific structure of NNN.
3. **Butterfly Operations with Multiple Radices:**
   The Mixed Radix algorithm utilizes butterfly operations for various radix sizes. Each sub-transform involves specific butterfly operations characterized by multiplication with twiddle factors appropriate for the radix in question.

   The generalized butterfly operation for a radix-rrr element can be expressed as:

   Xk,r=∑j=0r−1(Aj,k⋅Wjk)mod  pX\_{k,r} = \sum\_{j=0}^{r-1} (A\_{j,k} \cdot W^{jk}) \mod pXk,r​=j=0∑r−1​(Aj,k​⋅Wjk)modp

   where:

   Xk,rX\_{k,r}Xk,r​ - is the output of the radix−rradix-rradix−r butterfly operation for the k−thk-thk−th set of inputs

   Aj,kA\_{j,k}Aj,k​ - represents the j−thj-thj−th input element for the k−thk-thk−th operation

   WWW - is the twiddle factor

   ppp - is the prime modulus
4. **Recombination and Reordering:**
   After applying the appropriate butterfly operations across all decomposition levels, the Mixed Radix algorithm recombines the results into a single output sequence. Due to the varied sizes of the sub-transforms, a more complex reordering process may be required compared to Radix-2. This involves digit-reversal permutations to ensure that the final output sequence is correctly ordered.

### Which algorithm should I choose ?[​](#which-algorithm-should-i-choose- "Direct link to Which algorithm should I choose ?")

Both work only on inputs of power of 2 (e.g., 256, 512, 1024).

Radix 2 is faster for small NTTs. A small NTT would be around logN = 16 and batch size 1. Radix 2 won't necessarily perform better for smaller `logn` with larger batches.

Mixed radix on the other hand works better for larger NTTs with larger input sizes.

Performance really depends on logn size, batch size, ordering, inverse, coset, coeff-field and which GPU you are using.

For this reason we implemented our [heuristic auto-selection](https://github.com/ingonyama-zk/icicle/blob/main/icicle/src/ntt/ntt.cu#L573) which should choose the most efficient algorithm in most cases.

We still recommend you benchmark for your specific use case if you think a different configuration would yield better results.

[Edit this page](https://github.com/ingonyama-zk/icicle/tree/main/docs/versioned_docs/version-2.8.0/icicle/primitives/ntt.md)

Last updated on **Aug 6, 2025**

[Previous

MSM](/2.8.0/icicle/primitives/msm)[Next

Keccak Hash](/2.8.0/icicle/primitives/keccak)

* [Using NTT](#using-ntt)
  + [Supported Bindings](#supported-bindings)
  + [Examples](#examples)
  + [Ordering](#ordering)
  + [Modes](#modes)
* [Supported algorithms](#supported-algorithms)
  + [Radix 2](#radix-2)
  + [Mixed Radix](#mixed-radix)
  + [Which algorithm should I choose ?](#which-algorithm-should-i-choose-)

Copyright © 2025 Ingonyama, Inc. Built with Docusaurus.