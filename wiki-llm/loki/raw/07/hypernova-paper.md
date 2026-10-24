HyperNova: Recursive arguments for customizable constraint systems


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

#### Paper 2023/573

### [HyperNova: Recursive arguments for customizable constraint systems](/2023/573.pdf)

Abhiram Kothapalli, Carnegie Mellon University

Srinath Setty, Microsoft Research

##### Abstract

We introduce HyperNova, a new recursive argument for proving incremental computations whose steps are expressed with CCS (Setty et al. ePrint 2023/552), a customizable constraint system that simultaneously generalizes Plonkish, R1CS, and AIR without overheads. HyperNova makes four contributions, each resolving a major problem in the area of recursive arguments.
First, it provides a folding scheme for CCS where the prover's cryptographic cost is a single multi-scalar multiplication (MSM) of size equal to the number of variables in the constraint system, which is optimal when using an MSM-based commitment scheme. The folding scheme can fold multiple instances at once, making it easier to build generalizations of IVC such as PCD. Second, when proving program executions on stateful machines (e.g., EVM, RISC-V), the cost of proving a step of a program is proportional only to the size of the circuit representing the instruction invoked by the program step ("a la carte" cost profile). Third, we show how to achieve zero-knowledge for "free" and without the need to employ zero-knowledge SNARKs: we show that Nova's folding scheme has a blinding property that can be used to "randomize" IVC proofs and, more generally, to add zero-knowledge to non-ZK SNARKs, via a protocol we call NovaBlindFold. This highlights a new application of folding schemes. Fourth, we show how to efficiently instantiate HyperNova over a cycle of elliptic curves. For this, we provide a general technique, which we refer to as CycleFold, that applies to all modern folding-scheme-based recursive arguments.

**Note:** An extended version of the paper appearing in CRYPTO'24. Compared to an initial version, this version of the paper incorporates material from prior preprints (SuperNova and CycleFold). Additionally, this version provides an approach to achieve zero-knowledge in folding-scheme-based recursive arguments without needing to use zkSNARKs.
Update (02/20/2026): Move results from appendices into a new section 7 and name the result as NovaBlindFold.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/573.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   A major revision of an IACR publication in CRYPTO 2024

Keywords
:   [succinct arguments](/search?q=succinct%20arguments)[customizable constraints](/search?q=customizable%20constraints)[recursive arguments](/search?q=recursive%20arguments)[folding schemes](/search?q=folding%20schemes)

Contact author(s)
:   akothapalli @ cmu edu  
    srinath @ microsoft com

History
:   2026-02-20: last of 5 revisions
:   2023-04-23: received
:   [See all versions](/archive/versions/2023/573)

Short URL
:   <https://ia.cr/2023/573>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/573,
      author = {Abhiram Kothapalli and Srinath Setty},
      title = {{HyperNova}: Recursive arguments for customizable constraint systems},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/573},
      year = {2023},
      url = {https://eprint.iacr.org/2023/573}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.