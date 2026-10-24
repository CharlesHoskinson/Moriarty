CycleFold: Folding-scheme-based recursive arguments over a cycle of elliptic curves


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

#### Paper 2023/1192

### [CycleFold: Folding-scheme-based recursive arguments over a cycle of elliptic curves](/2023/1192.pdf)

Abhiram Kothapalli, Carnegie Mellon University

Srinath Setty, Microsoft Research

##### Abstract

This paper introduces CycleFold, a new and conceptually simple approach to instantiate folding-scheme-based recursive arguments over a cycle of elliptic curves, for the purpose of realizing incrementally verifiable computation (IVC). Existing approach to solve this problem originates from BCTV (CRYPTO'14) who describe their approach for a SNARK-based recursive argument, and it was adapted by Nova (CRYPTO'22) to a folding-scheme-based recursive argument. A downside of this approach is that it represents a folding scheme verifier as a circuit on both curves in the cycle. (e.g., with Nova, this requires $\approx$10,000 multiplication gates on both curves in the cycle).
CycleFold’s starting point is the observation that folding-scheme-based recursive arguments can be efficiently instantiated without a cycle of elliptic curves—except for a few scalar multiplications in their verifiers (2 in Nova, 1 in HyperNova, and 3 in ProtoStar). Accordingly, CycleFold uses the second curve in the cycle to merely represent a single scalar multiplication ($\approx$1,000--1,500 multiplication gates). CycleFold then folds invocations of that tiny circuit on the first curve in the cycle. This is nearly an order of magnitude improvement over the prior state-of-the-art in terms of circuit sizes on the second curve. CycleFold is particularly beneficial when instantiating folding-scheme-based recursive arguments over “half pairing” cycles (e.g., BN254/Grumpkin) as it keeps the circuit on the non-pairing-friendly curve minimal. The running instance in a CycleFold-based recursive argument consists of an instance on the first curve and a tiny instance on the second curve. Both instances can be proven using a zkSNARK defined over the scalar field of the first curve.
On the conceptual front, with CycleFold, an IVC construction and nor its security proof has to explicitly reason about the cycle of elliptic curves. Finally, due to its simplicity, CycleFold-based recursive argument can be more easily be adapted to support parallel proving with the so-called "binary tree" IVC.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/1192.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   Preprint.

Keywords
:   [succinct arguments](/search?q=succinct%20arguments)[recursive arguments](/search?q=recursive%20arguments)[cycles of elliptic curves](/search?q=cycles%20of%20elliptic%20curves)

Contact author(s)
:   akothapalli @ cmu edu  
    srinath @ microsoft com

History
:   2023-08-07: approved
:   2023-08-04: received
:   [See all versions](/archive/versions/2023/1192)

Short URL
:   <https://ia.cr/2023/1192>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/1192,
      author = {Abhiram Kothapalli and Srinath Setty},
      title = {{CycleFold}: Folding-scheme-based recursive arguments over a cycle of elliptic curves},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/1192},
      year = {2023},
      url = {https://eprint.iacr.org/2023/1192}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.