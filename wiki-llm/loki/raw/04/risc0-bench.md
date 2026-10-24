Performance Benchmarks | RISC Zero Developer Docs


[Skip to main content](#__docusaurus_skipToContent_fallback)

[![RISC Zero](/img/logo.svg)![RISC Zero](/img/logo.svg)](/)[Documentation](/api)[Terminology](/terminology)[FAQ](/faq)[Education Hub](/education)

[3.0](/api/zkvm/benchmarks)

* [Next](/api/next/zkvm/benchmarks)
* [3.0](/api/zkvm/benchmarks)
* [2.3](/api/2.3/zkvm/benchmarks)
* [2.2](/api/2.2/zkvm/benchmarks)
* [2.1](/api/2.1/zkvm/benchmarks)
* [2.0.0](/api/2.0.0/zkvm/benchmarks)
* [1.2](/api/1.2/zkvm/benchmarks)
* [1.1](/api/1.1/zkvm/benchmarks)
* [1.0](/api/1.0/zkvm/benchmarks)

[Blog](https://www.risczero.com/blog)[GitHub](https://github.com/risc0)

[Community](#)

* [Twitter](https://twitter.com/risczero)
* [YouTube](https://www.youtube.com/@risczero)
* [Stack Overflow](https://stackoverflow.com/questions/tagged/risczero)
* [Contributor's Guide](/contributors-guide)

Search

* #### LEARN
* [Introduction](/api/)
* [What does RISC Zero enable?](/api/use-cases)
* [What is a zkVM Application?](/api/zkvm/)
* #### Build
* [Getting Started](/api/getting-started)
* [Developing your zkVM Application](/api/zkvm/install)
* [Generating Proofs for your zkVM Application](/api/generating-proofs/proving-options)
* [Optimizing your zkVM Application](/api/zkvm/profiling)

  + [Profiling](/api/zkvm/profiling)
  + [Guest Optimization](/api/zkvm/optimization)
  + [Cryptography Precompiles](/api/zkvm/precompiles)
  + [Performance Benchmarks](/api/zkvm/benchmarks)
* [Blockchain Integration](/api/blockchain-integration/risc-zero-on-eth)
* #### Miscellaneous
* [Security](/api/secure-sdlc)
* [zkVM Technical Reference](/api/recursion)
* [Awesome risc0](https://github.com/inversebrah/awesome-risc0)

* Optimizing your zkVM Application
* Performance Benchmarks
Version: 3.0

On this page

Performance Benchmarks
======================

Generating your own datasheet[​](#generating-your-own-datasheet "Direct link to Generating your own datasheet")
---------------------------------------------------------------------------------------------------------------

You can generate a datasheet yourself to get performance data for your exact
system, or if you want benchmarks for a different RISC Zero zkVM version than
used in the datasheet.

To generate a datasheet, first [install Rust and the RISC Zero
toolchain](/api/zkvm/install) and clone the [risc0 repository](https://github.com/risc0/risc0) if you haven't already done
so. Navigate to the risc0 source code directory and, if necessary, check out the
branch or tag you wish to generate a datasheet for. Then run:

```
cargo run --release --example datasheet
```

This will produce the benchmark data shown in the datasheet for your system
(using the CPU) on the checked out version of the RISC Zero zkVM.

If you want to benchmark a GPU, you will need to build with the `cuda` or
`metal` feature enabled (whichever is appropriate for your hardware). To do
this, use the following commands:

* **Metal**

  ```
  cargo run --release -F metal --example datasheet
  ```
* **CUDA**

  ```
  cargo run --release -F cuda --example datasheet
  ```

We also have a benchmark based on running a simple Fibonacci guest program,
which you can run with:

```
cargo bench --bench fib
```

This will compute the 100th, 1000th, and 10000th Fibonacci numbers modulo 2^64
(ten times for each). It will report both time and throughput (how many numbers
were added per second) with separate statistics for [execution](/terminology#execute) and
[proving](/terminology#prover). As with the loop benchmark, the Fibonacci benchmark will use
the CPU by default, and you can benchmark a CUDA or Metal GPU by setting the
appropriate feature flag.

[Edit this page](https://github.com/risc0/risc0/edit/main/website/api/zkvm/benchmarks.md)

[Previous

Cryptography Precompiles](/api/zkvm/precompiles)[Next

RISC Zero on Ethereum](/api/blockchain-integration/risc-zero-on-eth)

* [Generating your own datasheet](#generating-your-own-datasheet)

* [Blog](https://www.risczero.com/blog)
* [Careers](https://jobs.ashbyhq.com/RiscZero)
* [Bug Bounties](https://hackenproof.com/company/risc-zero/programs)

* [GitHub](https://github.com/risc0)
* [X](https://x.com/RiscZero)
* [YouTube](https://www.youtube.com/@risczero)

[![RISC Zero](/img/logo.svg)![RISC Zero](/img/logo.svg)](https://risczero.com)

©2026 RISC Zero