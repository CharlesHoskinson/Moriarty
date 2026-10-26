# cuSnark polynomial inversion
> Source: https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/polynomial-inversion
> Collected: 2026-10-26
> Published: Unknown

Polynomial Inversion | Snarkify

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

For the complete documentation index, see [llms.txt](https://docs.snarkify.io/llms.txt). This page is also available as [Markdown](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/polynomial-inversion.md).

Copy

On this page

1. [🏚️High-Performance ZKP](/high-performance-zkp/zkevm-halo2-gpu-prover)
2. [zkEVM Halo2 GPU Prover](/high-performance-zkp/zkevm-halo2-gpu-prover)

Polynomial Inversion
====================

Overview
--------

At multiple points in the Halo2 proof generation process, large sets of finite field elements must be inverted, which can be a significant computational expense.

GPU Implementation
------------------

These inversions can be trivially implemented on the GPU by simply computing the inverse of each element concurrently.

Performance
-----------

The following table outlines the performance improvements yielded with just the cuSnark polynomial inversion employed, showing the (CPU baseline) and accelerated results for the different proof stages in a set of proofs of various sizes. Times are in seconds, obtained on a AMD EPYC 7702 64-Core Processor with 4x NVIDIA GeForce RTX 3090 (24 GB) GPUs.

Proof Stage

2^20 rows, 1135 columns

2^25 rows, 5 columns

Initialization

(6.04) 6.12

(1.40) 1.41

Generate Instance

(0.05) 0.05

(1.05) 1.08

Generate Advice

(381.78) 355.14

(6.68) 5.53

Generate Lookups

(57.99) 57.92

(2.10) 2.12

Commit Permutations

(146.59) 145.17

(23.72) 23.58

Eval\_h

(1069.09) 1070.33

(66.58) 66.70

Compute Evaluations

(9.81) 9.85

(35.79) 35.39

Multiopen

(18.71) 18.93

(28.82) 28.69

Total

(1690.05) 1663.51

(166.15) 164.48

[PreviousKZG Multiopen](/high-performance-zkp/zkevm-halo2-gpu-prover/kzg-multiopen)[NextPermutation Generation](/high-performance-zkp/zkevm-halo2-gpu-prover/permutation-generation)

Last updated 2 years ago

* [Overview](#overview)
* [GPU Implementation](#gpu-implementation)
* [Performance](#performance)