Multivariate lookups based on logarithmic derivatives


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

#### Paper 2022/1530

### [Multivariate lookups based on logarithmic derivatives](/2022/1530.pdf)

Ulrich Haböck[![](/img/orcid.svg)](https://orcid.org/0000-0003-0467-9260), Orbis Labs, Polygon Labs

##### Abstract

Logarithmic derivatives translate products of linear factors into sums of their reciprocals, turning zeroes into simple poles of same multiplicity. Based on this simple fact, we construct an interactive oracle proof for multi-column lookups over the boolean hypercube, which makes use of a single multiplicity function instead of working with a rearranged union of table and witnesses. For single-column lookups the performance is comparable to the well-known Plookup strategy used by Hyperplonk+. However, the real power of our argument unfolds in the case of batch lookups when multiple columns are subject to a single-table lookup: While the number of field operations is comparable to the Hyperplonk+ lookup (extended to multiple columns), the oracles provided by our prover are much less expensive. For example, for columns of length 2^12, paper-pencil operation counts indicate that the logarithmic derivative lookup is between 1.5 and 4 times faster, depending on the number of columns.

**Note:** Clarified the trade-off between algebraic degree and the number of commitments, previously expressed by two protocol variants. Discussion of Polygon Miden’s bounded multiplicity encoding.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2022/1530.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [Lookup arguments](/search?q=Lookup%20arguments)[Interactive Oracle Proofs](/search?q=Interactive%20Oracle%20Proofs)[SNARKs](/search?q=SNARKs)

Contact author(s)
:   ulrich haboeck @ gmail com

History
:   2025-02-20: last of 3 revisions
:   2022-11-04: received
:   [See all versions](/archive/versions/2022/1530)

Short URL
:   <https://ia.cr/2022/1530>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2022/1530,
      author = {Ulrich Haböck},
      title = {Multivariate lookups based on logarithmic derivatives},
      howpublished = {Cryptology {ePrint} Archive, Paper 2022/1530},
      year = {2022},
      url = {https://eprint.iacr.org/2022/1530}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.