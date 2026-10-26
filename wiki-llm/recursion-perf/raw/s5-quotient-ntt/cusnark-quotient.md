# cuSnark quotient strategy
> Source: https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/quotient-polynomial-evaluation
> Collected: 2026-10-26
> Published: Unknown

Quotient Polynomial Evaluation | Snarkify

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

For the complete documentation index, see [llms.txt](https://docs.snarkify.io/llms.txt). This page is also available as [Markdown](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover/quotient-polynomial-evaluation.md).

Copy

On this page

1. [🏚️High-Performance ZKP](/high-performance-zkp/zkevm-halo2-gpu-prover)
2. [zkEVM Halo2 GPU Prover](/high-performance-zkp/zkevm-halo2-gpu-prover)

Quotient Polynomial Evaluation
==============================

Overview
--------

The quotient polynomial evaluation stage of Halo2, denoted as "Eval\_h" in the table below, is the most computationally expensive stage of the Halo2 proof generation process, taking between 1/3 and 2/3 of the total proof runtime, depending on the specific proof. This stage consists of applying a large graph of simple arithmetic operations (addition, subtraction, multiplication, etc) to a set of input polynomials (fixed, advice, instance, etc) in the extended coset domain, combining them into a single output polynomial H.

GPU Implementation
------------------

The individual operations (addition, subtraction, multiplication, etc) that comprise the quotient polynomial evaluation are trivially implemented on the GPU, where each arithmetic operation is applied in parallel to the corresponding coefficients of the polynomials used as inputs to the operation. However, two difficulties arise: a) some method is required for transpiling the evaluation graph into GPU code; the graphs are big enough such that doing so manually is intractable and explicitly writing every operation into a kernel can lead to hours-long NVCC compilation of the kernel, and b) a large proof will have far more input and intermediate polynomials than can be stored in device memory at once.
The cuSnark library addresses these difficulties by using a single kernel which processes the evaluation graph as a list of opcodes, taking as an argument a set of input polynomials, storing intermediate computations in a buffer, and writing any output values to a set of output polynomials. A python script is used to break up the evaluation graph into smaller opcode lists that comprise sequential launches of this generic kernel, such that the number of input/output arrays needed by any individual kernel launch do not exceed available device memory.

Performance
-----------

The following table outlines the performance improvements yielded with just the cuSnark polynomial evaluation employed, showing the (CPU baseline) and accelerated results for the different proof stages in a set of proofs of various sizes. Times are in seconds, obtained on a AMD EPYC 7702 64-Core Processor with 4x NVIDIA GeForce RTX 3090 (24 GB) GPUs.

Proof Stage

2^20 rows, 1135 columns

2^25 rows, 5 columns

Initialization

(6.04) 6.09

(1.40) 1.39

Generate Instance

(0.05) 0.05

(1.05) 1.05

Generate Advice

(381.78) 382.20

(6.68) 6.82

Generate Lookups

(57.99) 58.27

(2.10) 2.14

Commit Permutations

(146.59) 146.10

(23.72) 23.45

Eval\_h

(1069.09) 42.13

(66.58) 5.68

Compute Evaluations

(9.81) 10.03

(35.79) 36.29

Multiopen

(18.71) 18.79

(28.82) 27.80

Total

(1690.05) 663.66

(166.15) 104.61

[PreviousNTT](/high-performance-zkp/zkevm-halo2-gpu-prover/ntt)[NextKZG Multiopen](/high-performance-zkp/zkevm-halo2-gpu-prover/kzg-multiopen)

Last updated 2 years ago

* [Overview](#overview)
* [GPU Implementation](#gpu-implementation)
* [Performance](#performance)