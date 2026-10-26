# cuSnark permutation generation
> Source: https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/permutation-generation
> Collected: 2026-10-26
> Published: Unknown

Permutation Generation | Snarkify

[![](https://docs.snarkify.io/~gitbook/image?url=https%3A%2F%2F902313018-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F9nQFlJH6XQCyYmRvUwtw%252Ficon%252FO4QjntUvCCe7CXyNnigv%252FSnarkify_Contained_Logo_Only_Black.png%3Falt%3Dmedia%26token%3Db1a21ba0-2b1c-4e87-a465-711c58c86c25&width=32&dpr=3&quality=100&sign=e7439e32999fcf46ecbb1486ba63dff9&sv=3)![](https://docs.snarkify.io/~gitbook/image?url=https%3A%2F%2F902313018-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F9nQFlJH6XQCyYmRvUwtw%252Ficon%252FO4QjntUvCCe7CXyNnigv%252FSnarkify_Contained_Logo_Only_Black.png%3Falt%3Dmedia%26token%3Db1a21ba0-2b1c-4e87-a465-711c58c86c25&width=32&dpr=3&quality=100&sign=e7439e32999fcf46ecbb1486ba63dff9&sv=3)

Snarkify](/)

`⌘Ctrl``k`

[![](https://docs.snarkify.io/~gitbook/image?url=https%3A%2F%2F902313018-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F9nQFlJH6XQCyYmRvUwtw%252Ficon%252FO4QjntUvCCe7CXyNnigv%252FSnarkify_Contained_Logo_Only_Black.png%3Falt%3Dmedia%26token%3Db1a21ba0-2b1c-4e87-a465-711c58c86c25&width=32&dpr=3&quality=100&sign=e7439e32999fcf46ecbb1486ba63dff9&sv=3)![](https://docs.snarkify.io/~gitbook/image?url=https%3A%2F%2F902313018-files.gitbook.io%2F%7E%2Ffiles%2Fv0%2Fb%2Fgitbook-x-prod.appspot.com%2Fo%2Fspaces%252F9nQFlJH6XQCyYmRvUwtw%252Ficon%252FO4QjntUvCCe7CXyNnigv%252FSnarkify_Contained_Logo_Only_Black.png%3Falt%3Dmedia%26token%3Db1a21ba0-2b1c-4e87-a465-711c58c86c25&width=32&dpr=3&quality=100&sign=e7439e32999fcf46ecbb1486ba63dff9&sv=3)

Snarkify](/)

* 🏗️Serverless GPU Proving

  + [Deploy a Elastic Prover](/serverless-gpu-proving/deploy-a-elastic-prover)
  + [Snakrify CLI](/serverless-gpu-proving/snakrify-cli)
  + [Advanced Topics](/serverless-gpu-proving/advanced-topics)
* 🏚️High-Performance ZKP

  + [zkEVM Halo2 GPU Prover](/high-performance-zkp/zkevm-halo2-gpu-prover)

    - [MSM](/high-performance-zkp/zkevm-halo2-gpu-prover/msm)
    - [NTT](/high-performance-zkp/zkevm-halo2-gpu-prover/ntt)
    - [Quotient Polynomial Evaluation](/high-performance-zkp/zkevm-halo2-gpu-prover/quotient-polynomial-evaluation)
    - [KZG Multiopen](/high-performance-zkp/zkevm-halo2-gpu-prover/kzg-multiopen)
    - [Polynomial Inversion](/high-performance-zkp/zkevm-halo2-gpu-prover/polynomial-inversion)
    - [Permutation Generation](/high-performance-zkp/zkevm-halo2-gpu-prover/permutation-generation)
  + [ZPrize](/high-performance-zkp/zprize)
* 🐺Sirius Folding

  + [Introduction](/sirius-folding/introduction)
  + [Quickstart](/sirius-folding/quickstart)
  + [Examples](/sirius-folding/examples)
  + [Terminologies](/sirius-folding/terminologies)
  + [Sirius Folding APIs](/sirius-folding/sirius-folding-apis)
* LInks

  + [Github](https://github.com/snarkify/)
  + [Telegram Group](https://t.me/+fCIx2XhUiBoyNmJh)
  + [snarkify.io](https://snarkify.io)

[Powered by GitBook](https://www.gitbook.com/powered-by?utm_source=content&utm_medium=trademark&utm_campaign=9nQFlJH6XQCyYmRvUwtw&utm_content=site_ldJlE)

On this page

For the complete documentation index, see [llms.txt](https://docs.snarkify.io/llms.txt). This page is also available as [Markdown](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/permutation-generation.md).

Copy

On this page

1. [🏚️High-Performance ZKP](/high-performance-zkp/zkevm-halo2-gpu-prover)
2. [zkEVM Halo2 GPU Prover](/high-performance-zkp/zkevm-halo2-gpu-prover)

Permutation Generation
======================

Overview
--------

One component of lookup generation in Halo2 involves constructing permutations of columns of the trace table, which can be computationally demanding. Different versions of Halo2 follow different methods for this construction. One method involves creating permutations A' and S' of columns A and S, such that (A'[i] - A'[i+1])\*(S'[i] - A'[i]) = 0.

GPU Implementation
------------------

The GPU algorithm used in the cuSnark library to efficiently construct these permutations is as follows:

1) Apply a hash function H to each element of A to determine an index into a pair of hash tables storing the total count of that element's value, and a corresponding index in A where that value exists.

2) Perform exclusive prefix scan on the count hash table to determine the starting index in A' for the grouping of each unique value in A

3) Insert each element of A into A', using the hash function & tables, and the result of the prefix scan above, to determine the correct index in A'. For the first occurrence of a given value in A', also set the corresponding value in S', and a boolean "set" flag at the same index in a flag array indicating that index of S' has been set.

4) For each element of S, use the hash tables to look up if that element has been inserted into S' already. If so, set the corresponding element of a boolean "used" flag array to true.

5) Perform inclusive prefix scans to sum the elements of the "set" and "used" flag arrays

6) Use the summed set array to create a mapping between [index in a list of leftover elements in S] and [index in S'].

7) For each element of S which has not yet been used in S', use the summed used array to determine the element's index in the leftover list, and map that leftover index to an index in S'. Then, write the leftover element into S'.

Performance
-----------

As mentioned above, not all versions of Halo2 use this method for constructing permutations, so this acceleration is not used in the benchmark proofs referenced by earlier sections. In another Halo2 version which does use this construction, on a proof with 2^24 rows and 1 lookup operation, the CPU lookup generation takes **39.32 seconds**, while the GPU accelerated version (using a single RTX 4090 card) takes **3.94 seconds**.

[PreviousPolynomial Inversion](/high-performance-zkp/zkevm-halo2-gpu-prover/polynomial-inversion)[NextZPrize](/high-performance-zkp/zprize)

Last updated 2 years ago

* [Overview](#overview)
* [GPU Implementation](#gpu-implementation)
* [Performance](#performance)