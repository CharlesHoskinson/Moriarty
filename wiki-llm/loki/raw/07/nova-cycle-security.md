Revisiting the Nova Proof System on a Cycle of Curves


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

#### Paper 2023/969

### [Revisiting the Nova Proof System on a Cycle of Curves](/2023/969.pdf)

Wilson Nguyen, Stanford University

Dan Boneh, Stanford University

Srinath Setty, Microsoft Research

##### Abstract

Nova is an efficient recursive proof system built from an elegant folding scheme for (relaxed) R1CS statements. The original Nova paper (CRYPTO'22) presented Nova using a single elliptic curve group of order $p$. However, for improved efficiency, the implementation of Nova alters the scheme to use a 2-cycle of elliptic curves. This altered scheme is only described in the code and has not been proven secure. In this work, we point out a soundness vulnerability in the original implementation of the 2-cycle Nova system. To demonstrate this vulnerability, we construct a convincing Nova proof for the correct evaluation of $2^{75}$ rounds of the Minroot VDF in only 1.46 seconds. We then present a modification of the 2-cycle Nova system and formally prove its security. The modified system also happens to be more efficient than the original implementation. In particular, the modification eliminates an R1CS instance-witness pair from the recursive proof. The implementation of Nova has now been updated to use our optimized and secure system. We also show that Nova's IVC proofs are malleable and discuss several mitigations.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/969.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [incremental verifiable computation](/search?q=incremental%20verifiable%20computation)[recursive proof systems](/search?q=recursive%20proof%20systems)[vulnerability](/search?q=vulnerability)[cycle of elliptic curves](/search?q=cycle%20of%20elliptic%20curves)[IVC](/search?q=IVC)

Contact author(s)
:   wdnguyen @ cs stanford edu  
    dabo @ cs stanford edu  
    srinath @ microsoft com

History
:   2023-06-20: approved
:   2023-06-20: received
:   [See all versions](/archive/versions/2023/969)

Short URL
:   <https://ia.cr/2023/969>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/969,
      author = {Wilson Nguyen and Dan Boneh and Srinath Setty},
      title = {Revisiting the Nova Proof System on a Cycle of Curves},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/969},
      year = {2023},
      url = {https://eprint.iacr.org/2023/969}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.