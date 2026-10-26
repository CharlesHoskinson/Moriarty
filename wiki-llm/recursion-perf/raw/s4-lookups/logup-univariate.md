# Improving logarithmic derivative lookups using GKR
> Source: https://eprint.iacr.org/2023/1284
> Collected: 2026-10-26
> Published: 2023-08-27

Improving logarithmic derivative lookups using GKR


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

#### Paper 2023/1284

### [Improving logarithmic derivative lookups using GKR](/2023/1284.pdf)

Shahar Papini, StarkWare

Ulrich Haböck, Polygon Labs

##### Abstract

In this informal note, we instantiate the Goldwasser-Kalai-Rothblum (GKR) protocol to prove fractional sumchecks as present in lookup arguments based on logarithmic derivatives, with the following impact on the prover cost of logUp (IACR eprint 2022/1530):
When looking up $M\geq 1$ columns in a (for the sake of simplicity) single column table, the prover has to commit only to a single extra column, i.e. the multiplicities of the table entries.
In order to carry over the GKR fractional sumcheck to the univariate setting, we furthermore introduce a simple, yet (as far as we know) novel transformation for turning a univariate polynomial commitment scheme into a multilinear one. The transformation complements existing approaches and might be of independent interest for its elegant way to prove arbitrary powers of the lexicographic shift over the Boolean hypercube.

**Note:** Further typos corrected.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/1284.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [lookup arguments](/search?q=lookup%20arguments)[GKR](/search?q=GKR)[multilinear commitment scheme](/search?q=multilinear%20commitment%20scheme)

Contact author(s)
:   spapini @ starkware co  
    ulrich haboeck @ gmail com

History
:   2025-02-20: last of 3 revisions
:   2023-08-27: received
:   [See all versions](/archive/versions/2023/1284)

Short URL
:   <https://ia.cr/2023/1284>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/1284,
      author = {Shahar Papini and Ulrich Haböck},
      title = {Improving logarithmic derivative lookups using {GKR}},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/1284},
      year = {2023},
      url = {https://eprint.iacr.org/2023/1284}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.