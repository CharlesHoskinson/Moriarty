Efficient Foreign-Field Arithmetic in PLONK


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

#### Paper 2025/695

### [Efficient Foreign-Field Arithmetic in PLONK](/2025/695.pdf)

Miguel Ambrona[![](/img/orcid.svg)](https://orcid.org/0000-0001-5927-9235), Input Output Global

Denis Firsov[![](/img/orcid.svg)](https://orcid.org/0000-0003-1267-7898), Input Output Global

Inigo Querejeta-Azurmendi[![](/img/orcid.svg)](https://orcid.org/0000-0003-3679-2276), Input Output Global

##### Abstract

PLONK is a prominent universal and updatable zk-SNARK for general circuit satisfiability, which allows a prover to produce a short certificate of the validity of a certain statement/computation. Its expressive model of computation and its highly efficient verifier complexity make PLONK a powerful tool for a wide range of blockchain applications.
Supporting standard cryptographic primitives (such us ECDSA over SECP256k1) or advanced recursive predicates (e.g. incrementally verifiable computation) on a SNARK presents a significant challenge. It requires so-called foreign-field arithmetic (enforcing constraints over algebraic fields that differ from the SNARK native field) which was previously believed to incur an overhead of two or three orders of magnitude.
We build on the techniques by Lubarov and Baylina and observe that, by considering tight bounds on their encoding of foreign-field multiplication, the number of PLONK constraints can be significantly reduced. We show that these techniques also extend to elliptic curve emulation, with an overhead of just one order of magnitude (with respect to its native counterpart). We validate soundness and completeness of our main results in EasyCrypt. Finally, we implement an open-source library with support for foreign-field arithmetic. Our experimental results showcase the generality of our techniques and confirm their suitability for real-world applications.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2025/695.pdf)

Category
:   [Implementation](/search?category=IMPLEMENTATION)

Publication info
:   Preprint.

Keywords
:   [SNARKs](/search?q=SNARKs)[zero-knowledge](/search?q=zero-knowledge)[foreign-field arithmetic](/search?q=foreign-field%20arithmetic)

Contact author(s)
:   miguel ambrona @ iohk io  
    denis firsov @ iohk io  
    querejeta azurmendi @ iohk io

History
:   2025-04-17: approved
:   2025-04-16: received
:   [See all versions](/archive/versions/2025/695)

Short URL
:   <https://ia.cr/2025/695>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2025/695,
      author = {Miguel Ambrona and Denis Firsov and Inigo Querejeta-Azurmendi},
      title = {Efficient Foreign-Field Arithmetic in {PLONK}},
      howpublished = {Cryptology {ePrint} Archive, Paper 2025/695},
      year = {2025},
      url = {https://eprint.iacr.org/2025/695}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.