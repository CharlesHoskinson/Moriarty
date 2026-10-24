Accelerating Polygon zkEVM - Irreducible


[Team](../team)

[Blog](../posts)

[Contact](../contact)

[Team](../team)

[Blog](../posts)

[Contact](../contact)

Article

Accelerating Polygon zkEVM
==========================

We report the first end-to-end, FPGA-accelerated Polygon zkEVM prover.

![](https://framerusercontent.com/images/GFUga7CbRwPqkuA84EoKTLM34Q.jpg?width=2048&height=1365)

![](https://framerusercontent.com/images/GFUga7CbRwPqkuA84EoKTLM34Q.jpg?width=2048&height=1365)

4/24/24

![](https://framerusercontent.com/images/r6BfaHNk9iLYkIHPqzlCXi3XcFs.png?width=480&height=481)

Irreducible Team

### Benchmarking Performance and Cost

Leveraging our previously reported self-contained FPGA modules for accelerated [Merkle tree construction](https://www.ulvetanna.io/news/poseidon-merkle-trees-in-hardware) and [LDE computation](https://www.ulvetanna.io/news/fpga-architecture-for-goldilocks-ntt), as well as memory layout modifications to the [Polygon zkProver](https://docs.polygon.technology/zkEVM/architecture/zkprover/) for improved data transpose performance, we generated a 500-transaction batch proof in 84 seconds. The table below indicates how this performance compares to the non-accelerated zkProver.

![](https://framerusercontent.com/images/7lpdJCSOri88fhfbm9uxK0xUJZA.jpg?width=2400&height=1424)

Last year, Polygon’s zkProver [demonstrated](https://twitter.com/eduadiez/status/1635994534852153345) an impressive 500-transaction batch-proof generation time of under 2 minutes on a 224-thread Google Cloud Platform (GCP) server. Our FPGAs are currently integrated with a less powerful 64-thread server. Still, we found that our accelerated zkProver achieved a ~2x performance increase compared to the unmodified version on the same gen1-64 and, more importantly, a **1.4x increase compared to the more powerful GCP instance.**

![](https://framerusercontent.com/images/P0VbXehGtqOWWwJ9BpV91U56o.jpg?width=2400&height=1256)

For protocols, this means that Irreducible’s accelerated zkProver can provide the same performance as the unmodified version, but at a much lower cost. Since hardware capex is proprietary, we can compare cost efficiency on the basis of service provider pricing. The table below uses current GCP spot and reserved prices to illustrate the cost of generating a proof.

![](https://framerusercontent.com/images/LXhdiEEvlYw1ObhIxtlr7yw5Ctk.jpg?width=2400&height=624)

###### GCP pricing can be found [*here*](https://cloud.google.com/products/calculator <redacted-token>), and assumes 1-year commitment in the Iowa (US-central1) region

Polygon’s [report](https://twitter.com/eduadiez/status/1635994534852153345) quoted spot instance pricing. Because Irreducible can offer a lower cost of $0.05 per proof, it’s more cost-effective to use our accelerated service than to run the zkProver on Google Cloud. This is particularly true given that reserved pricing may be the better benchmark. Spot instances are notoriously difficult to manage due to service interruptions, capacity issues, and volatile pricing. So customers needing reliable and robust proof generation will have to pay for reserved or on-demand instances, which are generally 3-4x more expensive. Irreducible offers proving-as-a-service to meet this need, not only at a significantly lower price than current reserved/on-demand rates, but additionally saving customers the expense and hassle of managing their own infrastructure.

Our accelerated zkProver is not only compatible with Polygon’s imminent zkEVM type 1 upgrade, Polygon Miden, and zkSync Boojum, but also anticipates further improved performance with additional enhancements. By applying FPGA acceleration to the codeword batching and quotient polynomial computation stages, we predict a proof generation time under 60 seconds.

### Technical Overview

Polygon zkEVM is a complex system with several stages of the proof generation process. We focused our acceleration efforts for this initial proof-of-concept on the polynomial commitment operation, which is the most expensive and accounts for 63% of the overall proving time (before acceleration). This operation commits a batch of polynomials, and it executes four separate times over the course of one proof on polynomial batches of different sizes. All of the polynomial batches contain polynomials with 2²³ coefficients. The first batch is 669 polynomials, the second 132, the third 372, and the fourth 6 polynomials. The polynomial commitment operation begins with a low-degree extension (LDE) column-wise over the matrix, producing an extended matrix with 2²⁴ rows and 669 columns. A low-degree extension consists of multiple Number Theoretic Transforms (NTT). The prover then performs a leaf-hashing operation, hashing each row of the extended matrix with the Poseidon hash, and finally computes a Merkle tree over each row digest. The total size of the input data over all four rounds is 73.7 GiB, and the total size of the LDE output (which must be stored until the end of the proof generation) is 147 GiB.

The integrated system accelerates the LDE and leaf-hashing operations using two FPGA cards. They are connected to each other directly via QSFP and communicate using AMD’s Aurora link-layer protocol. The host streams data via PCIe 4.0 to one FPGA device, which performs the LDE. Once the LDE operation completes, the device streams the output both back to the host and to the second device, which runs the leaf-hashing engine (LHE). This architecture, shown below, both reduces the time to compute the LDE and almost completely hides the latency of leaf-hashing.

![](https://framerusercontent.com/images/zL6a3ZLZOb3SfEJtNIRcuIl3us.png?width=1520&height=858)

This integration requires several modifications to Polygon’s open-source C++ [zkevm-prover](https://github.com/0xPolygonHermez/zkevm-prover) software, which has a sophisticated memory management strategy, storing all polynomials in a pre-allocated [arena](https://en.wikipedia.org/wiki/Region-based_memory_management) at precomputed offsets. The prover implementation stores the polynomial batches in interleaved order, meaning each polynomial is not contiguous in memory. This is convenient during the trace generation and Merkleization steps, and also assists parallelization of the LDE computation. Because our FPGAs only have 16 GiB of High Bandwidth Memory (HBM), we must transpose the stored polynomials before transferring them to the device. Similarly, we transpose the polynomials received from the device before writing them back to program memory. We took two measures to minimize the cost of the transposes. First, we modified the zkevm-prover memory layout so that every row of the matrices are padded to 64-byte boundaries to align with CPU cache lines. This enables our high-throughput, AVX2-accelerated matrix transpose implementation. Second, rather than performing the entire transposes before the LDE begins and after it completes, we pipeline the transposes with the computation in the device to hide the software latency. The software must both transpose the data sent to and received from the LDE module concurrently in separate threads until all polynomials in a batch are processed. We emphasize that even with our software modifications, the proofs we generate are completely valid when checked by the reference verifier.

**Ultimately, these improvements enabled us to generate a proof ~40% faster than ever previously reported by Polygon’s zkProver. With further FPGA-based acceleration addressing two additional computation steps, we’ll be able to deliver proofs in under one minute flat.**

#### Subscribe to stay updated.

#### Subscribe to stay updated.

#### Subscribe to stay updated.

[### Read More](../posts#:x_mmzjSLg)

[11/12/25

![](https://framerusercontent.com/images/PkbTpEpeynFoOla3ZUpC1byx9EE.png?width=2400&height=1600)

Article

#### Irreducible is Shutting Down

Thank you to everyone who supported us along the way](./irreducible-shutting-down)

[![](https://framerusercontent.com/images/PkbTpEpeynFoOla3ZUpC1byx9EE.png?width=2400&height=1600)

Article

11/12/25

#### Irreducible is Shutting Down

Thank you to everyone who supported us along the way

Read More](./irreducible-shutting-down)

[11/12/25

![](https://framerusercontent.com/images/PkbTpEpeynFoOla3ZUpC1byx9EE.png?width=2400&height=1600)

Article

#### Irreducible is Shutting Down

Thank you to everyone who supported us along the way](./irreducible-shutting-down)

[9/9/25

![](https://framerusercontent.com/images/K7u0oB3Dx6lDbJuouv3WfW3Y.png?width=1024&height=683)

News

#### Reinventing Irreducible

You may have noticed we’ve been quiet recently. That’s because we’ve been busy reinventing ourselves.](./reinventing-irreducible)

[![](https://framerusercontent.com/images/K7u0oB3Dx6lDbJuouv3WfW3Y.png?width=1024&height=683)

News

9/9/25

#### Reinventing Irreducible

You may have noticed we’ve been quiet recently. That’s because we’ve been busy reinventing ourselves.

Read More](./reinventing-irreducible)

[9/9/25

![](https://framerusercontent.com/images/K7u0oB3Dx6lDbJuouv3WfW3Y.png?width=1024&height=683)

News

#### Reinventing Irreducible

You may have noticed we’ve been quiet recently. That’s because we’ve been busy reinventing ourselves.](./reinventing-irreducible)

[9/9/25

![](https://framerusercontent.com/images/2eIgkmJj7oD1a63U4hwppCePyA.png?width=2048&height=1366)

News

#### Announcing Binius64

The next evolution of Binius, optimized for simplicity and CPU performance.](./announcing-binius64)

[![](https://framerusercontent.com/images/2eIgkmJj7oD1a63U4hwppCePyA.png?width=2048&height=1366)

News

9/9/25

#### Announcing Binius64

The next evolution of Binius, optimized for simplicity and CPU performance.

Read More](./announcing-binius64)

[9/9/25

![](https://framerusercontent.com/images/2eIgkmJj7oD1a63U4hwppCePyA.png?width=2048&height=1366)

News

#### Announcing Binius64

The next evolution of Binius, optimized for simplicity and CPU performance.](./announcing-binius64)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/KHWIcgQzwSwEAkwEkwEX0tKjuOk.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/lYC3xVoofMQ42zVZdNKbswS8PE8.png)

![](https://framerusercontent.com/images/a2Hk4bgM5zHpCFZIP3G97e6Ut4.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/KHWIcgQzwSwEAkwEkwEX0tKjuOk.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/KHWIcgQzwSwEAkwEkwEX0tKjuOk.png)

![](https://framerusercontent.com/images/favu4RIncSqbQhXCd6rf5tZpiZs.png)

![](https://framerusercontent.com/images/faErMiuyietIjK6QeFnErIwA.png)

![](https://framerusercontent.com/images/0YKirk76YNv9ktjFAY6SMYq4lk.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/Gth6fNucx9mGbcTBmx8NlSHs84.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/favu4RIncSqbQhXCd6rf5tZpiZs.png)

![](https://framerusercontent.com/images/faErMiuyietIjK6QeFnErIwA.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/nOvnVq2HaXx4rfpGHTAThzmOCdA.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/nOvnVq2HaXx4rfpGHTAThzmOCdA.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/favu4RIncSqbQhXCd6rf5tZpiZs.png)

![](https://framerusercontent.com/images/faErMiuyietIjK6QeFnErIwA.png)

![](https://framerusercontent.com/images/lYC3xVoofMQ42zVZdNKbswS8PE8.png)

![](https://framerusercontent.com/images/a2Hk4bgM5zHpCFZIP3G97e6Ut4.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/nOvnVq2HaXx4rfpGHTAThzmOCdA.png)

![](https://framerusercontent.com/images/lYC3xVoofMQ42zVZdNKbswS8PE8.png)

![](https://framerusercontent.com/images/a2Hk4bgM5zHpCFZIP3G97e6Ut4.png)

![](https://framerusercontent.com/images/0YKirk76YNv9ktjFAY6SMYq4lk.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/Gth6fNucx9mGbcTBmx8NlSHs84.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/favu4RIncSqbQhXCd6rf5tZpiZs.png)

![](https://framerusercontent.com/images/faErMiuyietIjK6QeFnErIwA.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/KHWIcgQzwSwEAkwEkwEX0tKjuOk.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/KHWIcgQzwSwEAkwEkwEX0tKjuOk.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/0lKYnATMOtMrEG6DY51xGBoLo.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/wShT4WrLp9adQHPW15ZKW0o64E.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/KHWIcgQzwSwEAkwEkwEX0tKjuOk.png)

![](https://framerusercontent.com/images/favu4RIncSqbQhXCd6rf5tZpiZs.png)

![](https://framerusercontent.com/images/faErMiuyietIjK6QeFnErIwA.png)

![](https://framerusercontent.com/images/0YKirk76YNv9ktjFAY6SMYq4lk.png)

![](https://framerusercontent.com/images/MogJQAqODh3ropTPyVGmPHwcCIk.png)

![](https://framerusercontent.com/images/Gth6fNucx9mGbcTBmx8NlSHs84.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/favu4RIncSqbQhXCd6rf5tZpiZs.png)

![](https://framerusercontent.com/images/faErMiuyietIjK6QeFnErIwA.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/bKDb1RZTztC6bBUhq63rqgRI.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/iZNXe5jOas823nPtUYaI9vuw.png)

![](https://framerusercontent.com/images/7SGniddaVQKToIZzNx0INtKJ1E.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/nOvnVq2HaXx4rfpGHTAThzmOCdA.png)

![](https://framerusercontent.com/images/e4578f114wMLBC1xhG5f6dKn3BU.png)

![](https://framerusercontent.com/images/nOvnVq2HaXx4rfpGHTAThzmOCdA.png)

![](https://framerusercontent.com/images/lYC3xVoofMQ42zVZdNKbswS8PE8.png)

![](https://framerusercontent.com/images/a2Hk4bgM5zHpCFZIP3G97e6Ut4.png)

© 2025 Irreducible Inc. All rights reserved.

[Team](../team)

[Blog](../posts)

[Media Kit](../media-kit)

[Contact](../contact)

© 2025 Irreducible Inc. All rights reserved.

[Team](../team)

[Blog](../posts)

[Media Kit](../media-kit)

[Contact](../contact)

© 2025 Irreducible Inc. All rights reserved.

[Team](../team)

[Blog](../posts)

[Media Kit](../media-kit)

[Contact](../contact)

[Team](../team)

[Blog](../posts)

[Contact](../contact)

[Team](../team)

[Blog](../posts)

[Contact](../contact)