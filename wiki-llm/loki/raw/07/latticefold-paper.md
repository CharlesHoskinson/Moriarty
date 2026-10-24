LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems


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

#### Paper 2024/257

### [LatticeFold: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems](/2024/257.pdf)

Dan Boneh[![](/img/orcid.svg)](https://orcid.org/0000-0003-0820-0421), Stanford University

Binyi Chen[![](/img/orcid.svg)](https://orcid.org/0000-0003-0835-9678), Stanford University

##### Abstract

Folding is a recent technique for building efficient recursive SNARKs. Several elegant folding protocols have been proposed, such as Nova, Supernova, Hypernova, Protostar, and others. However, all of them rely on an additively homomorphic commitment scheme based on discrete log, and are therefore not post-quantum secure and require a large (256-bit) field. In this work we present LatticeFold, the first lattice-based folding protocol based on the Module SIS problem. This folding protocol naturally leads to an efficient recursive lattice-based SNARK and an efficient PCD scheme. LatticeFold supports folding low-degree relations, such as R1CS, as well as high-degree relations, such as CCS. The key challenge is to construct a secure folding protocol that works with the Ajtai commitment scheme. The difficulty is ensuring that extracted witnesses are low norm through many rounds of folding. We present a novel technique using the sumcheck protocol to ensure that extracted witnesses are always low norm no matter how many rounds of folding are used. Since LatticeFold can operate over a small (64-bit) field, our evaluation of the final proof system suggests that it is as performant as Hypernova, while providing plausible post-quantum security. Moreover, LatticeFold operates over the same module structure used by fully homomorphic encryption (FHE) and lattice signatures schemes, and can therefore benefit from software optimizations and custom hardware designed to accelerate these lattice schemes.

**Note:** Add a missing citation.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/257.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [Succinct Proof Systems](/search?q=Succinct%20Proof%20Systems)[IVC/PCD](/search?q=IVC/PCD)[Folding](/search?q=Folding)[Lattice-based Cryptography](/search?q=Lattice-based%20Cryptography)

Contact author(s)
:   dabo @ cs stanford edu  
    binyi @ cs stanford edu

History
:   2025-12-15: last of 5 revisions
:   2024-02-16: received
:   [See all versions](/archive/versions/2024/257)

Short URL
:   <https://ia.cr/2024/257>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/257,
      author = {Dan Boneh and Binyi Chen},
      title = {{LatticeFold}: A Lattice-based Folding Scheme and its Applications to Succinct Proof Systems},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/257},
      year = {2024},
      url = {https://eprint.iacr.org/2024/257}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.