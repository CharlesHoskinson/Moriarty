Chimera: A Hybrid GPU Backend for Sumcheck Acceleration in Zero-Knowledge Provers


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

#### Paper 2026/1388

### [Chimera: A Hybrid GPU Backend for Sumcheck Acceleration in Zero-Knowledge Provers](/2026/1388.pdf)

Kashfia Farheen, University of Delaware

Nektarios Georgios Tsoutsos[![](/img/orcid.svg)](https://orcid.org/0000-0002-5769-0124), University of Delaware

##### Abstract

Sumcheck is increasingly used in proof systems based on multilinear polynomials, but its execution structure differs from that of conventional GPU targets such as MSM and NTT. Each round contains substantial data-parallel polynomial computation, but consecutive rounds depend on challenges derived from previous rounds and interact with the surrounding prover's proof-generation operations. Efficient acceleration therefore depends not only on GPU arithmetic throughput, but also on where polynomial data resides, what crosses the CPU-GPU boundary, and when execution should return to the CPU.
We present Chimera, a hybrid CPU-GPU backend for Sumcheck computations in the Spartan R1CS prover. Chimera keeps large polynomial data on the GPU across profitable rounds, returns only the compact polynomial values needed by Spartan's transcript, and sends the resulting challenge back to the GPU for folding. It switches sufficiently small late rounds to Spartan's native CPU implementation. When polynomial data exceeds GPU memory, Chimera processes it in smaller portions and transitions to resident GPU execution once the remaining data fits. Chimera preserves Spartan's proof format and verifier behavior while also reducing CPU-side overhead directly surrounding the accelerated Sumcheck computations.
On BN254 Spartan R1CS instances, Chimera reduces the measured
Sumcheck-related region from \(672.785\) ms to \(84.245\) ms at
\(N=2^{17}\), a \(7.99\times\) speedup. Complete proving time improves at every measured size, reaching \(1.27\times\)-\(1.64\times\) speedup for \(N=2^{24}\)-\(2^{27}\). Separately, our standalone quadratic GPU Sumcheck executes in \(28.7\) ms at \(N=2^{22}\), compared with \(84.2\) ms for ICICLE's CUDA Sumcheck implementation, a \(2.94\times\) GPU-to-GPU improvement. These results show that effective Sumcheck acceleration depends on both efficient GPU execution and careful integration with the surrounding proof system.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2026/1388.pdf)

Category
:   [Implementation](/search?category=IMPLEMENTATION)

Publication info
:   Preprint.

Keywords
:   [Zero-knowledge proofs](/search?q=Zero-knowledge%20proofs)[Sumcheck](/search?q=Sumcheck)[GPU acceleration](/search?q=GPU%20acceleration)

Contact author(s)
:   tsoutsos @ udel edu

History
:   2026-10-16: revised
:   2026-07-07: received
:   [See all versions](/archive/versions/2026/1388)

Short URL
:   <https://ia.cr/2026/1388>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2026/1388,
      author = {Kashfia Farheen and Nektarios Georgios Tsoutsos},
      title = {Chimera: A Hybrid {GPU} Backend for Sumcheck Acceleration in Zero-Knowledge Provers},
      howpublished = {Cryptology {ePrint} Archive, Paper 2026/1388},
      year = {2026},
      url = {https://eprint.iacr.org/2026/1388}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.