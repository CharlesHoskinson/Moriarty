# MicroNova folding-based arguments
> Source: https://eprint.iacr.org/2024/2099
> Collected: 2026-10-26
> Published: 2024-12-31

MicroNova: Folding-based arguments with efficient (on-chain) verification


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

#### Paper 2024/2099

### [MicroNova: Folding-based arguments with efficient (on-chain) verification](/2024/2099.pdf)

Jiaxing Zhao, University of Science and Technology of China, Microsoft Research

Srinath Setty, Microsoft Research

Weidong Cui, Microsoft Research

Greg Zaverucha, Microsoft Research

##### Abstract

We describe the design and implementation of MicroNova, a folding-based recursive argument for producing proofs of incremental computations of the form $y = F^{(\ell)}(x)$, where $F$ is a possibly non-deterministic computation (encoded using a constraint system such as R1CS), $x$ is the initial input, $y$ is the output, and $\ell > 0$. The proof of an $\ell$-step computation is produced step-by-step such that the proof size nor the time to verify it depends on $\ell$. The proof at the final iteration is then compressed, to achieve further succinctness in terms of proof size and verification time. Compared to prior folding-based arguments, a distinguishing aspect of MicroNova is the concrete efficiency of the verifier—even in a resource-constrained environment such as Ethereum’s blockchain. In particular, the compressed proof consists of $O(\log{N})$ group elements and it can be verified with $O(\log{N})$ group scalar multiplications and two pairing operations, where $N$ is the number of constraints for a single invocation of $F$. MicroNova requires a universal trusted setup and can employ any existing setup material created for the popular KZG univariate polynomial commitment scheme. Finally, we implement and experimentally evaluate MicroNova. We find that MicroNova’s proofs can be efficiently verified on the Ethereum blockchain with $\approx$2.2M gas. Furthermore, MicroNova’s prover incurs minimal overheads atop its baseline Nova’s prover.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/2099.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   Published elsewhere. Minor revision. IEEE Security & Privacy 2025

Keywords
:   [proof systems](/search?q=proof%20systems)[folding schemes](/search?q=folding%20schemes)[incrementally verifiable computation](/search?q=incrementally%20verifiable%20computation)[SNARKs](/search?q=SNARKs)

Contact author(s)
:   jiaxingzhao @ mail ustc edu cn  
    srinath @ microsoft com  
    weidong cui @ microsoft com

History
:   2025-04-24: revised
:   2024-12-31: received
:   [See all versions](/archive/versions/2024/2099)

Short URL
:   <https://ia.cr/2024/2099>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/2099,
      author = {Jiaxing Zhao and Srinath Setty and Weidong Cui and Greg Zaverucha},
      title = {{MicroNova}: Folding-based arguments with efficient (on-chain) verification},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/2099},
      year = {2024},
      url = {https://eprint.iacr.org/2024/2099}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.
