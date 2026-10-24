Fast elliptic curve scalar multiplications in SN(T)ARK circuits


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

#### Paper 2025/933

### [Fast elliptic curve scalar multiplications in SN(T)ARK circuits](/2025/933.pdf)

Liam Eagen, Alpen Labs

Youssef El Housni, Linea

Simon Masson, ZKNox

Thomas Piellard, Linea

##### Abstract

Proof systems of arbitrary computations have found many applications in recent years. However, the proving algorithm has a consequent complexity tied to the size of the computation being proved. Thus, proving large computations is quite ineﬃcient. One of these large computations is the scalar multiplication over an elliptic curve. In this work, we provide new techniques for reducing the time corresponding to proving a scalar multiplication, using integer lattice reduction or a (half) extended Euclidean algorithm in a ring of integers. We investigate optimizations in the case of small (complex multiplication) discriminant curves, and its generalization for multi scalar multiplications as used in signature verification. We provide an optimized Golang implementation for diﬀerent elliptic curves in diﬀerent proof systems settings. The speed-up in proving time is between 22% and 53% compared to the previous state-of-the-art.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2025/933.pdf)

Category
:   [Implementation](/search?category=IMPLEMENTATION)

Publication info
:   Published elsewhere. LATINCRYPT 2025

DOI
:   [10.1007/978-3-032-06754-8\_4 ![](/img/open-outline.svg)](https://doi.org/10.1007/978-3-032-06754-8_4)

Keywords
:   [SNARK](/search?q=SNARK)[STARK](/search?q=STARK)[proof systems](/search?q=proof%20systems)[elliptic curves](/search?q=elliptic%20curves)[scalar multiplication](/search?q=scalar%20multiplication)

Contact author(s)
:   liameagen @ protonmail com  
    youssef elhousni @ consensys net  
    simon masson @ protonmail com  
    thomas piellard @ consensys net

History
:   2025-10-17: revised
:   2025-05-22: received
:   [See all versions](/archive/versions/2025/933)

Short URL
:   <https://ia.cr/2025/933>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2025/933,
      author = {Liam Eagen and Youssef El Housni and Simon Masson and Thomas Piellard},
      title = {Fast elliptic curve scalar multiplications in {SN}(T){ARK} circuits},
      howpublished = {Cryptology {ePrint} Archive, Paper 2025/933},
      year = {2025},
      doi = {10.1007/978-3-032-06754-8_4},
      url = {https://eprint.iacr.org/2025/933}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.