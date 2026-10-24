ProtoStar: Generic Efficient Accumulation/Folding for Special Sound Protocols


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

#### Paper 2023/620

### [ProtoStar: Generic Efficient Accumulation/Folding for Special Sound Protocols](/2023/620.pdf)

Benedikt Bünz[![](/img/orcid.svg)](https://orcid.org/0000-0003-2082-4480), Espresso Systems

Binyi Chen[![](/img/orcid.svg)](https://orcid.org/0000-0003-0835-9678), Espresso Systems

##### Abstract

Accumulation is a simple yet powerful primitive that enables incrementally verifiable computation (IVC) without the need for recursive SNARKs. We provide a generic, efficient accumulation (or folding) scheme for any $(2k-1)$-move special-sound protocol with a verifier that checks $\ell$ degree-$d$ equations. The accumulation verifier only performs $k+2$ elliptic curve multiplications and $k+d+O(1)$ field/hash operations. Using the compiler from BCLMS21 (Crypto 21), this enables building efficient IVC schemes where the recursive circuit only depends on the number of rounds and the verifier degree of the underlying special-sound protocol but not the proof size or the verifier time. We use our generic accumulation compiler to build ProtoStar. ProtoStar is a non-uniform IVC scheme for Plonk that supports high-degree gates and (vector) lookups. The recursive circuit is dominated by $3$ group scalar multiplications and a hash of $d^\*$ field elements, where $d^\*$ is the degree of the highest gate. The scheme does not require a trusted setup or pairings, and the prover does not need to compute any FFTs. The prover in each accumulation/IVC step is also only logarithmic in the number of supported circuits and independent of the table size in the lookup.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/620.pdf)

Category
:   [Public-key cryptography](/search?category=PUBLICKEY)

Publication info
:   A minor revision of an IACR publication in ASIACRYPT 2023

Keywords
:   [incremental verifiable computation](/search?q=incremental%20verifiable%20computation)[accumulation schemes](/search?q=accumulation%20schemes)[recursive proof composition](/search?q=recursive%20proof%20composition)[proof-carrying data](/search?q=proof-carrying%20data)

Contact author(s)
:   benedikt @ espressosys com  
    binyi @ espressosys com

History
:   2023-12-21: last of 12 revisions
:   2023-05-01: received
:   [See all versions](/archive/versions/2023/620)

Short URL
:   <https://ia.cr/2023/620>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/620,
      author = {Benedikt Bünz and Binyi Chen},
      title = {{ProtoStar}: Generic Efficient Accumulation/Folding for Special Sound Protocols},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/620},
      year = {2023},
      url = {https://eprint.iacr.org/2023/620}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.