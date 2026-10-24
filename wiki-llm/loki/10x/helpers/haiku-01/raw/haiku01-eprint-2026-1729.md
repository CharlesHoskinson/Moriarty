A Unified Framework for Contract-Validated Benchmarking of Zero-Knowledge Proving Systems


What a lovely hat
=================

#### Is it made out of [tin foil](https://iacr.org/tinfoil.html)?

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

[Cryptology ePrint Archive](/)

* [Papers](#)
  Updates from the last:+ [7 days](/days/7)
  + [31 days](/days/31)
  + [6 months](/days/183)
  + [365 days](/days/365)
  + ---
  + [Listing by year](/byyear)
  + [All papers](/complete)
  + [Compact view](/complete/compact)
  + [Subscribe](https://www.iacr.org/news/subscribe)
  + ---
  + [How to cite](/citation.html)
  + ---
  + [Harvesting metadata](/rss)
* [Submissions](#)
  + [Submit a paper](/submit)
  + [Revise or withdraw a paper](/revise)
  + [Acceptance and publishing conditions](/operations.html)
* [About](#)
  + [Goals and history](/about.html)
  + [News](/news.html)
  + [Statistics](/stats)
  + [Contact](/contact.html)

![Search Button](/img/search.svg)

Search

[Advanced search](/search)

#### Paper 2026/1729

### [A Unified Framework for Contract-Validated Benchmarking of Zero-Knowledge Proving Systems](/2026/1729.pdf)

Matej Hulek, Brno University of Technology

Martin Peresini[![](/img/orcid.svg)](https://orcid.org/0000-0002-2875-9567), Brno University of Technology

Ivan Homoliak[![](/img/orcid.svg)](https://orcid.org/0000-0002-0790-0875), Brno University of Technology

##### Abstract

Zero-knowledge proofs are an operational infrastructure for rollups, private payments, verifiable execution, and zkVM applications. We treat them as succinct proofs of validity whose privacy properties depend on the system and the mode of use. Published benchmarks often compare different workloads, proof modes, and hardware paths, which makes their results hard to interpret. This work presents a unified host-agent framework for benchmarking heterogeneous proving systems under a common execution and reporting method. Each prover runs in an isolated containerized agent and is selected through a scheme-adapter layer. The agent reports canonical JSON artifacts linked to a hashed run manifest and workload-specific semantic contracts. The host checks the public semantic fields and the agent's verification flag to ensure backends are compared on the same declared public claim, even when their circuits, traces, or guest pipelines differ. We evaluate circuit-based, transparent, GPU-accelerated, folding, and zkVM systems: Groth16, ICICLE, Halo2, Plonky3, Winterfell, Nova, RISC Zero with CUDA and Groth16 wrapping, and SP1 with and without AVX. The suite uses six primary workloads with comparable semantics, together with secondary hash workloads that expose specialization. No prover family dominates all deployment criteria. At 8 CPU cores, Plonky3 and Winterfell have the smallest across-case proving-time and RAM figures. Groth16 and ICICLE produce sub-kilobyte proofs. Winterfell has the lowest host verification time. zkVM systems trade larger proofs and higher memory for programmability. On the Merkle-path micro-benchmark, pairing artifacts remain sub-kilobyte, whereas wrapping a zkVM receipt yields a Groth16-sized artifact at a high proving cost. GPU acceleration helps most after fixed setup costs are amortized. The main contribution is methodological: validate the statement being proved, disclose the security profile, and report resource limits in addition to proving time.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2026/1729.pdf)

Category
:   [Applications](/search?category=APPLICATIONS)

Publication info
:   Preprint.

Keywords
:   [Zero-knowledge proofs](/search?q=Zero-knowledge%20proofs)[Rollups](/search?q=Rollups)[zkVM](/search?q=zkVM)[zk-SNARK](/search?q=zk-SNARK)[zk-STARK](/search?q=zk-STARK)[GPU acceleration](/search?q=GPU%20acceleration)[Benchmarking](/search?q=Benchmarking)[Verifiable computation](/search?q=Verifiable%20computation)

Contact author(s)
:   xhulek02 @ stud fit vut cz  
    iperesini @ fit vut cz  
    homoliak @ fit vutbr cz

History
:   2026-08-21: approved
:   2026-08-18: received
:   [See all versions](/archive/versions/2026/1729)

Short URL
:   <https://ia.cr/2026/1729>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2026/1729,
      author = {Matej Hulek and Martin Peresini and Ivan Homoliak},
      title = {A Unified Framework for Contract-Validated Benchmarking of Zero-Knowledge Proving Systems},
      howpublished = {Cryptology {ePrint} Archive, Paper 2026/1729},
      year = {2026},
      url = {https://eprint.iacr.org/2026/1729}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.