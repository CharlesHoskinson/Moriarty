zkEVM Halo2 GPU Prover | Snarkify

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

For the complete documentation index, see [llms.txt](https://docs.snarkify.io/llms.txt). This page is also available as [Markdown](https://docs.snarkify.io/high-performance-zkp/zkevm-halo2-gpu-prover.md).

Copy

On this page

1. [🏚️High-Performance ZKP](/high-performance-zkp/zkevm-halo2-gpu-prover)

zkEVM Halo2 GPU Prover
======================

Snarkify's cuSnark library is a C++/CUDA project which provides a set of API functions, with rust bindings, designed to be plugged into the Halo2 proof system, replacing various compute-intensive operations with an accelerated GPU backend.

The following tables outline the performance improvements yielded when all of these optimizations are employed. The impact on end-to-end (e2e) proof time each optimization has depends on the size of the proof, which can be generally characterized by the number of rows and columns in the proof's trace table. Two proofs with different dimensions have been selected to demonstrate the variance in optimization impact. These benchmarks were obtained on a AMD EPYC 7702 64-Core Processor with 4x NVIDIA GeForce RTX 3090 (24 GB) GPUs.

### Proof 1 (aggregation): 2^25 rows, 5 columns

Proof Stage

CPU/s

CPU e2e %

GPU/s

GPU e2e %

Speedup

Initialization

1.40

0.64

1.40

4.63

1.00

Generate Instance

1.08

0.50

0.46

1.52

2.35

Generate Advice

6.99

3.21

2.73

9.03

2.56

Generate Lookups

2.22

1.02

1.88

6.22

1.18

Commit Permutations

24.01

11.03

10.72

35.45

2.24

Eval\_h

67.40

30.95

6.30

20.83

10.70

Compute Evaluations

35.73

16.41

5.02

16.60

7.12

Multiopen

29.74

13.66

1.72

5.69

17.29

**Total**

**217.76**

**30.24**

**7.20**

### Proof 2 (chunk\_inner): 2^20 rows, 1135 columns

Proof Stage

CPU/s

CPU e2e %

GPU/s

GPU e2e %

Speedup

Initialization

6.15

0.35

6.11

1.32

1.01

Generate Instance

0.05

0.00

0.13

0.03

0.38

Generate Advice

393.58

22.33

306.44

66.17

1.28

Generate Lookups

59.63

3.38

56.84

12.27

1.05

Commit Permutations

152.79

8.67

42.27

9.13

3.61

Eval\_h

1115.43

63.28

36.19

7.81

30.82

Compute Evaluations

10.22

0.58

7.60

1.64

1.34

Multiopen

24.90

1.41

7.56

1.63

3.29

**Total**

**1762.75**

**463.13**

**3.81**

This document outlines the following GPU modules and the acceleration they provide for Halo2 proofs of various dimensions:

* [Multi-Scalar Multiplication (MSM)](/high-performance-zkp/zkevm-halo2-gpu-prover/msm)
* [Number Theoretic Transform (NTT)](/high-performance-zkp/zkevm-halo2-gpu-prover/ntt)
* [Polynomial Evaluation](/high-performance-zkp/zkevm-halo2-gpu-prover/quotient-polynomial-evaluation)
* [KZG Multiopen](/high-performance-zkp/zkevm-halo2-gpu-prover/kzg-multiopen)
* [Polynomial Inversion](/high-performance-zkp/zkevm-halo2-gpu-prover/polynomial-inversion)
* [Permutation Generation](/high-performance-zkp/zkevm-halo2-gpu-prover/permutation-generation)

[PreviousWebhook](/serverless-gpu-proving/advanced-topics/webhook)[NextMSM](/high-performance-zkp/zkevm-halo2-gpu-prover/msm)

Last updated 9 months ago

* [Proof 1 (aggregation): 2^25 rows, 5 columns](#proof-1-aggregation-2-25-rows-5-columns)
* [Proof 2 (chunk\_inner): 2^20 rows, 1135 columns](#proof-2-chunk_inner-2-20-rows-1135-columns)