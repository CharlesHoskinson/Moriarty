# Current Jolt architecture overview
> Source: https://jolt.a16zcrypto.com/how/architecture/architecture.html
> Collected: 2026-10-26
> Published: Unknown

Architecture overview - JoltBook


1. [**1.** Intro](../../intro.html)
2. [**2.** Usage](../../usage/usage.html)
3. 1. [**2.1.** Quickstart](../../usage/quickstart.html)
   2. [**2.2.** Guests and hosts](../../usage/guests_hosts/guests_hosts.html)
   3. 1. [**2.2.1.** Guests](../../usage/guests_hosts/guests.html)
      2. [**2.2.2.** Hosts](../../usage/guests_hosts/hosts.html)
   4. [**2.3.** Runtime Advice](../../usage/runtime_advice.html)
   5. [**2.4.** Profiling](../../usage/profiling/profiling.html)
   6. 1. [**2.4.1.** zkVM profiling](../../usage/profiling/zkvm_profiling.html)
      2. [**2.4.2.** Guest profiling](../../usage/profiling/guest_profiling.html)
   7. [**2.5.** Troubleshooting](../../usage/troubleshooting.html)
4. [**3.** How it works](../../how/how-it-works.html)
5. 1. [**3.1.** Architecture overview](../../how/architecture/architecture.html)
   2. 1. [**3.1.1.** RISC-V emulation](../../how/architecture/emulation.html)
      2. [**3.1.2.** R1CS constraints](../../how/architecture/r1cs_constraints.html)
      3. 1. [**3.1.2.1.** Spartan](../../how/architecture/spartan.html)
      4. [**3.1.3.** Registers](../../how/architecture/registers.html)
      5. [**3.1.4.** RAM](../../how/architecture/ram.html)
      6. [**3.1.5.** Instruction execution](../../how/architecture/instruction_execution.html)
      7. [**3.1.6.** Bytecode](../../how/architecture/bytecode.html)
      8. [**3.1.7.** Batched opening proof](../../how/architecture/opening-proof.html)
   3. [**3.2.** Twist and Shout](../../how/twist-shout.html)
   4. [**3.3.** Dory](../../how/dory.html)
   5. [**3.4.** Akita](../../how/akita.html)
   6. [**3.5.** BlindFold (Zero Knowledge)](../../how/blindfold.html)
   7. [**3.6.** Optimizations](../../how/optimizations/optimizations.html)
   8. 1. [**3.6.1.** Batched sumcheck](../../how/optimizations/batched-sumcheck.html)
      2. [**3.6.2.** Batched openings](../../how/optimizations/batched-openings.html)
      3. [**3.6.3.** Inlines](../../how/optimizations/inlines.html)
      4. [**3.6.4.** Small value optimizations](../../how/optimizations/small-value.html)
      5. [**3.6.5.** EQ optimizations](../../how/optimizations/eq.html)
      6. [**3.6.6.** Torus compression](../../how/optimizations/compression.html)
   9. [**3.7.** Formal Verification](../../how/formal-verification/formal-verification.html)
   10. 1. [**3.7.1.** Field Kernels](../../how/formal-verification/field-kernels.html)
       2. 1. [**3.7.1.1.** Scalar Fp64 Proofs](../../how/formal-verification/field-kernels-fp64.html)
          2. [**3.7.1.2.** From Rust to Machine Bytes](../../how/formal-verification/field-kernels-source-to-bytes.html)
          3. [**3.7.1.3.** Reading a Machine Theorem](../../how/formal-verification/field-kernels-reading-theorem.html)
          4. [**3.7.1.4.** Trust Boundary and Review Guide](../../how/formal-verification/field-kernels-trust-boundary.html)
       3. [**3.7.2.** Z3 Verifier](../../how/formal-verification/z3-verifier.html)
   11. [**3.8.** Appendix](../../how/appendix/appendix.html)
   12. 1. [**3.8.1.** Terminology and nomenclature](../../how/appendix/terminology.html)
       2. [**3.8.2.** Multilinear extensions](../../how/appendix/multilinear-extensions.html)
       3. [**3.8.3.** Sumcheck](../../how/appendix/sumcheck.html)
       4. [**3.8.4.** Polynomial commitment schemes](../../how/appendix/pcs.html)
       5. [**3.8.5.** Memory checking and lookup arguments](../../how/appendix/memory-checking.html)
       6. [**3.8.6.** RISC-V](../../how/appendix/risc-v.html)
       7. [**3.8.7.** Jolt 0.1.0](../../how/appendix/jolt-classic.html)
       8. [**3.8.8.** Additional resources](../../how/appendix/resources.html)
6. [**4.** Roadmap](../../roadmap/roadmap.html)
7. 1. [**4.1.** Streaming](../../roadmap/streaming.html)
   2. [**4.2.** Recursion](../../roadmap/recursion.html)
   3. [**4.3.** On-chain verifier](../../roadmap/on-chain-verifier.html)
8. [**5.** Testing gates](../../dev/testing-gates.html)

* Light (default)
* Rust
* Coal
* Navy
* Ayu

JoltBook
========

[Architecture overview](#architecture-overview)
===============================================

This section gives a overview of the architecture of the Jolt codebase.

The following diagram depicts the end-to-end Jolt pipeline, which is typical of RISC-V zkVMs.

![e2e](../../imgs/end_to_end.png)

The green steps correspond to **preprocessing** steps, which can be computed ahead of time from the guest program's bytecode.
Note that they do not depend on the program inputs, so the preprocessing data can be reused across multiple invocations of the Jolt prover/verifier.

The red steps correspond to the **prover's** workflow: it runs the guest program on some inputs inside of a [RISC-V emulator](./emulation.html), producing the output of the program as well as an execution trace.
This execution trace, along with the inputs, outputs, and preprocessing, are then passed to the Jolt prover, which uses them to generate a succinct proof that the outputs are the result of correctly executing the given guest program on the inputs.

The blue steps correspond to the **verifier's** workflow. The verifier does *not* emulate the guest program, not does it receive the execution trace (which may be billions of cycles long) from the prover. Instead, it receives the proof from the prover, and uses it to check that the guest program produces claimed outputs when run on the given inputs.

For the sake of readability, the diagram above abstracts away all of Jolt's proof system machinery.
The rest of this section aims to disassemble the underlying machinery in useful-but-not-overwhelming detail.

As a starting point, we will present two useful mental models of the Jolt proof system: Jolt as a CPU, and Jolt as a DAG (a different DAG than the one above; we love DAGs).

[Jolt as a CPU](#jolt-as-a-cpu)
-------------------------------

One way to understand Jolt is to map the components of the proof system to the components of the virtual machine (VM) whose functionality they prove.

### [Jolt's five components](#jolts-five-components)

A VM does two things:

* Repeatedly execute the fetch-decode-execute logic of its instruction set architecture.
* Perform reads and writes to memory.

The Jolt paper depicts these two tasks mapped to three components in the Jolt proof system:

![Fig 2](../../imgs/figure2.png)

The Jolt codebase is similarly organized, but instead distinguishes read-write memory (comprising registers and RAM) from program code (aka bytecode, which is read-only), for a total of five components:

![Jolt as CPU](../../imgs/jolt_as_cpu.png)

Note that in Jolt as described in the paper, as well as in the first version of the Jolt implementation ([v0.1.0](../appendix/jolt-classic.html)), the memory checking and lookup argument used are [Spice](https://eprint.iacr.org/2018/907) and [Lasso](https://eprint.iacr.org/2023/1216), respectively. As of v0.2.0, Jolt replaces Spice and Lasso with [Twist and Shout](../twist-shout.html), respectively.

#### [R1CS constraints](#r1cs-constraints)

To handle the "fetch" part of the fetch-decode-execute loop, there is a minimal R1CS instance (about 20 constraints per cycle of the RISC-V VM). These constraints handle program counter (PC) updates and serves as the "glue" enforcing consistency between polynomials used in the components below. Jolt uses [Spartan](https://eprint.iacr.org/2019/550), optimized for the highly-structured nature of the constraint system (i.e. the same small set of constraints are applied to every cycle in the execution trace).

*For more details: [R1CS constraints](./r1cs_constraints.html)*

#### [RAM](#ram)

To handle reads/writes to RAM Jolt uses the Twist memory checking argument.

*For more details: [RAM](./ram.html)*

#### [Registers](#registers)

Similar to RAM, Jolt uses the Twist memory checking argument to handle reads/writes to registers.

*For more details: [Registers](./registers.html)*

#### [Instruction execution](#instruction-execution)

To handle the "execute" part of the fetch-decode-execute loop, Jolt invokes the Shout lookup argument. The lookup table, in this case, effectively maps every instruction to its correct output.

*For more details: [Instruction execution](./instruction_execution.html)*

#### [Bytecode](#bytecode)

To handle the "decode" part of the fetch-decode-execute loop, Jolt uses another instance of Shout. The bytecode of the guest program is "decoded" in preprocessing, and the prover subsequently invokes offline memory-checking on the sequence of reads from this decoded bytecode corresponding to the execution trace being proven.

*For more details: [Bytecode](./bytecode.html)*

[Jolt as a DAG](#jolt-as-a-dag)
-------------------------------

One useful way to understand Jolt is to view it as a directed acyclic graph (DAG). That might sound surprising -- how can a zkVM be represented as a DAG? The key lies in the structure of its sumchecks, and in particular, the role of virtual polynomials.

### [Virtual vs. Committed Polynomials](#virtual-vs-committed-polynomials)

**Virtual polynomials**, a concept introduced in the [Binius paper](https://eprint.iacr.org/2023/1784), are used heavily in Jolt.

A virtual polynomial is a part of the witness that is never committed directly. Instead, any claimed evaluation of a virtual polynomial is proven by a subsequent sumcheck.
In contrast, committed polynomials are committed to explicitly and their evaluaions are proven using the opening proof of the PCS.

### [Sumchecks as Nodes](#sumchecks-as-nodes)

Each sumcheck in Jolt corresponds to a node in the DAG. Consider the following hypothetical sumcheck expression:

InputClaim=?x∈{0,1}n∑​f(x)⋅g(x)+h(x)

Here, the left-hand side is the input claim, and the right-hand side is a sum over the Boolean hypercube of a multivariate polynomial of degree 2, involving three multilinear terms f, g, and h.
The sumcheck protocol reduces this *input* claim to three *output* claims about f, g, and h:

OutputClaimf​=?f(r)OutputClaimg​=?g(r)OutputClaimh​=?h(r)

where r∈Fn consists of the random challenges obtained over the course of the sumcheck.

An output claim of one sumcheck might be the input claim of another sumcheck.
Recall that [by definition](#virtual-vs-committed-polynomials), the output claim for a virtual polynomial is necessarily the input claim of another sumcheck.

It follows that input claims can be viewed as in-edges and output claims as out-edges, thus defining the graph structure. This is what the Jolt sumcheck DAG looks like:

![dag](../../imgs/jolt_dag.png)

Note the white boxes corresponding to the five components of Jolt under the CPU paradigm –– plus two extra components ("R1CS input virtualization" and "One-hot checks") which don't have a CPU analogue.

In the diagaram above, sumchecks are color-coded based on "batch"; sumchecks in the same batch are "run in parallel" (see [Batched sumcheck](../optimizations/batched-sumcheck.html)).
Note that a given component may include sumchecks in different batches. For example, the sumchecks under "Registers" span stages 3, 4, 5, and 6.

The prover stages live in `crates/jolt-prover/src/stages/`, while the verifier
stages live in `crates/jolt-verifier/src/stages/`. Each stage module declares
the sumcheck instances it contains.

Observe that the DAG defines a partial ordering over the nodes (sumchecks), which consequently defines the minimum number of batches -- two sumchecks cannot be batched together if one "depends" on the other.

The terminal nodes in the graph –– namely, the "Inc claim reduction" and the Hamming weight + ra claim reduction" –– output claims that are directly proven using the PCS opening proof.
For more details on the batched opening proof subprotocol, see [Opening proof](./opening-proof.html).

### [Managing state and data flow](#managing-state-and-data-flow)

The `ProverOpeningAccumulator` (resp. `VerifierOpeningAccumulator`) is responsible for managing the flow of opening claims –– the edges of the DAG.
The opening accumulator contains a mapping from `OpeningId` to claimed polynomial evaluation.
As sumchecks are proven, their output claims (for both virtual and committed polynomials) are inserted into the map. Later sumchecks can then consume the virtual polynomial openings as input claims.

While virtual polynomial claims are used internally and passed between sumchecks, committed polynomial claims are tracked because they must ultimately be verified via the selected backend's [batched opening proof](./opening-proof.html). [Dory](../dory.html) uses a random linear combination of commitments; [Akita](../akita.html) uses a native grouped opening over native trace columns and precommitted objects.

The `input_claim` and `cache_openings` methods on the `SumcheckInstanceProver` and `SumcheckInstanceVerifier` traits provide hooks to the respective accumulator objects.
`input_claim` effectively declares the in-edges for the sumcheck instance, while `cache_openings` effectively declares the out-edges.