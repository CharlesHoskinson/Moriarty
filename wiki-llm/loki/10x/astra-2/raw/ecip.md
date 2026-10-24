Zero Knowledge Proofs of Elliptic Curve Inner Products from Principal Divisors and Weil Reciprocity


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

#### Paper 2022/596

### [Zero Knowledge Proofs of Elliptic Curve Inner Products from Principal Divisors and Weil Reciprocity](/2022/596.pdf)

Liam Eagen

##### Abstract

Zero Knowledge proofs of Elliptic Curve Inner Products (ECIPs) and elliptic curve operations more generally are an increasingly important part of zero knowledge protocols and a significant bottle neck in recursive proof composition over amicable cycles of elliptic curves. To prove ECIPs more efficiently, I represent a collection of points that sum to zero using a polynomial element of the function field and evaluate this function at a random principal divisor. By Weil reciprocity, this is equal to the function interpolating the random divisor evaluated at the original points. Taking the logarithmic derivative of both expressions allows the prover to use a similar technique to the Bulletproofs++ permutation argument and take linear combinations logarithmic derivatives of divisor witnesses and collect terms for the same basis point by adding the multiplicities. The linear combination can be random or can be structured to cancel intermediate points in computing the sum. Since the multiplicities are field elements, this system can prove ECIP relations in zero knowledge with respect to the linear combination, the curve points, or both. Compared to existing techniques, the witness size is reduced by up to a factor of 10 and the number of multiplications by a factor of about 100 with significantly more flexibility in the organization of the protocol. The specific improvement will depend on the instantiating proof system, number of curve points, and which information is zero knowledge. This technique also works, with small modification, for proving multiexponentiations in the multiplicative group of the field.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2022/596.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint. MINOR revision.

Keywords
:   [zero knowledge elliptic curves](/search?q=zero%20knowledge%20elliptic%20curves)

Contact author(s)
:   liameagen @ protonmail com

History
:   2022-05-17: received

Short URL
:   <https://ia.cr/2022/596>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2022/596,
      author = {Liam Eagen},
      title = {Zero Knowledge Proofs of Elliptic Curve Inner Products from Principal Divisors and Weil Reciprocity},
      howpublished = {Cryptology {ePrint} Archive, Paper 2022/596},
      year = {2022},
      url = {https://eprint.iacr.org/2022/596}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.