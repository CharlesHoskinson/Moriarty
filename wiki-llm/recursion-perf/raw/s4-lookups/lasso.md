# Unlocking the lookup singularity with Lasso
> Source: https://eprint.iacr.org/2023/1216
> Collected: 2026-10-26
> Published: 2023-08-10

Unlocking the lookup singularity with Lasso


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

#### Paper 2023/1216

### [Unlocking the lookup singularity with Lasso](/2023/1216.pdf)

Srinath Setty, Microsoft Research

Justin Thaler, a16 crypto research and Georgetown University

Riad Wahby, Carnegie Mellon University

##### Abstract

This paper introduces Lasso, a new family of lookup arguments, which allow an untrusted prover to commit to a vector $a \in \mathbb{F}^m$ and prove that all entries of a reside in some predetermined table $t \in \mathbb{F}^n$. Lasso’s performance characteristics unlock the so-called "lookup singularity". Lasso works with any multilinear polynomial commitment scheme, and provides the following efficiency properties.
For $m$ lookups into a table of size $n$, Lasso’s prover commits to just $m + n$ field elements. Moreover, the committed field elements are small, meaning that, no matter how big the field $\mathbb{F}$ is, they are all in the set $\{0, . . . , m\}$. When using a multiexponentiation-based commitment scheme, this results in the prover’s costs dominated by only $O(m + n)$ group operations (e.g., elliptic curve point additions), plus the cost to prove an evaluation of a multilinear polynomial whose evaluations over the Boolean hypercube are the table entries. This represents a significant improvement in prover costs over prior lookup arguments (e.g., plookup, Halo2’s lookups, lookup arguments based on logarithmic derivatives).
Unlike all prior lookup arguments, if the table $t$ is structured (in a precise sense that we define), then no party needs to commit to $t$, enabling the use of much larger tables than prior works (e.g., of size $2^{128}$ or larger). Moreover, Lasso’s prover only "pays" in runtime for table entries that are accessed by the lookup operations. This applies to tables commonly used to implement range checks, bitwise operations, big-number arithmetic, and even transitions of a full-fledged CPU such as RISC-V. Specifically, for any integer parameter $c > 1$, Lasso’s prover’s dominant cost is committing to $3 \cdot c \cdot m + c \cdot n^{1/c}$ field elements. Furthermore, all these field elements are “small”, meaning they are in the set $\{0, . . . , \max{(m, n^{1/c}, q)} − 1\}$, where $q$ is the maximum value in $a$.
Lasso’s starting point is Spark, a time-optimal polynomial commitment scheme for sparse polynomials in Spartan (CRYPTO 2020). We first provide a stronger security analysis for Spark. Spartan’s security analysis assumed that certain metadata associated with a sparse polynomial is committed by an honest party (this is acceptable for its purpose in Spartan, but not for Lasso). We prove that Spark remains secure even when that metadata is committed by a malicious party. This provides the first "standard" commitment scheme for sparse multilinear polynomials with optimal prover costs. We then generalize Spark to directly support a lookup argument for both structured and unstructured tables, with the efficiency characteristics noted above.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/1216.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   Preprint.

Keywords
:   [lookup arguments](/search?q=lookup%20arguments)[succinct arguments](/search?q=succinct%20arguments)[sparse polynomial commitments](/search?q=sparse%20polynomial%20commitments)

Contact author(s)
:   srinath @ microsoft com

History
:   2023-08-11: approved
:   2023-08-10: received
:   [See all versions](/archive/versions/2023/1216)

Short URL
:   <https://ia.cr/2023/1216>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/1216,
      author = {Srinath Setty and Justin Thaler and Riad Wahby},
      title = {Unlocking the lookup singularity with Lasso},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/1216},
      year = {2023},
      url = {https://eprint.iacr.org/2023/1216}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.