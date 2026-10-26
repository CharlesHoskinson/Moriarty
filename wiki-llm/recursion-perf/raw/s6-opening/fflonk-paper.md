# fflonk: a Fast-Fourier inspired verifier efficient version of PlonK
> Source: https://eprint.iacr.org/2021/1167
> Collected: 2026-10-26
> Published: 2021-09-14

fflonk: a Fast-Fourier inspired verifier efficient version of PlonK


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

#### Paper 2021/1167

### [fflonk: a Fast-Fourier inspired verifier efficient version of PlonK](/2021/1167.pdf)

Ariel Gabizon and Zachary J. Williamson

##### Abstract

We present a variant of the Kate, Zaverucha and Goldberg polynomial commitment scheme [KZG] where $d$ polynomials can be opened at a point that is a $d$'th power, such that the amount of verifier group operations does not depend on $d$.
Our method works by reducing opening multiple polynomials at a single point $x$, to opening a single polynomial at many points via an ``FFT-like identity''.
As an application we present a version of the PlonK zk-SNARK[GWC] with significantly improved verifier performance, at the cost roughly tripling the prover time. Specifically, in addition to the two pairings, the verifier only performs five scalar multiplications, rather than 16 or 18 as in the versions presented in [GWC].

**Note:** typos

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2021/1167.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint. MINOR revision.

Keywords
:   [zk-SNARKs](/search?q=zk-SNARKs)[Polynomial Commitment Schemes](/search?q=Polynomial%20Commitment%20Schemes)

Contact author(s)
:   ariel gabizon @ gmail com

History
:   2021-11-01: last of 4 revisions
:   2021-09-14: received
:   [See all versions](/archive/versions/2021/1167)

Short URL
:   <https://ia.cr/2021/1167>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2021/1167,
      author = {Ariel Gabizon and Zachary J.  Williamson},
      title = {fflonk: a Fast-Fourier inspired verifier efficient version of {PlonK}},
      howpublished = {Cryptology {ePrint} Archive, Paper 2021/1167},
      year = {2021},
      url = {https://eprint.iacr.org/2021/1167}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.