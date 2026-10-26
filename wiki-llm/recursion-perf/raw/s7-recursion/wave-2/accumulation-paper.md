# Proof-Carrying Data from Accumulation Schemes
> Source: https://eprint.iacr.org/2020/499
> Collected: 2026-10-26
> Published: 2020-04-30

Proof-Carrying Data from Accumulation Schemes


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

#### Paper 2020/499

### [Proof-Carrying Data from Accumulation Schemes](/2020/499.pdf)

Benedikt Bünz, Alessandro Chiesa, Pratyush Mishra, and Nicholas Spooner

##### Abstract

Recursive proof composition has been shown to lead to powerful primitives such as incrementally-verifiable computation (IVC) and proof-carrying data (PCD). All existing approaches to recursive composition take a succinct non-interactive argument of knowledge (SNARK) and use it to prove a statement about its own verifier. This technique requires that the verifier run in time sublinear in the size of the statement it is checking, a strong requirement that restricts the class of SNARKs from which PCD can be built. This in turn restricts the efficiency and security properties of the resulting scheme.
Bowe, Grigg, and Hopwood (ePrint 2019/1021) outlined a novel approach to recursive composition, and applied it to a particular SNARK construction which does \*not\* have a sublinear-time verifier. However, they omit details about this approach and do not prove that it satisfies any security property. Nonetheless, schemes based on their ideas have already been implemented in software.
In this work we present a collection of results that establish the theoretical foundations for a generalization of the above approach. We define an \*accumulation scheme\* for a non-interactive argument, and show that this suffices to construct PCD, even if the argument itself does not have a sublinear-time verifier. Moreover we give constructions of accumulation schemes for SNARKs, which yield PCD schemes with novel efficiency and security features.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2020/499.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   A major revision of an IACR publication in TCC 2020

Keywords
:   [succinct arguments](/search?q=succinct%20arguments)[proof-carrying data](/search?q=proof-carrying%20data)[recursive proof composition](/search?q=recursive%20proof%20composition)

Contact author(s)
:   benedikt @ cs stanford edu  
    alexch @ berkeley edu  
    pratyush @ berkeley edu  
    nick spooner @ berkeley edu

History
:   2020-09-29: last of 6 revisions
:   2020-04-30: received
:   [See all versions](/archive/versions/2020/499)

Short URL
:   <https://ia.cr/2020/499>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2020/499,
      author = {Benedikt Bünz and Alessandro Chiesa and Pratyush Mishra and Nicholas Spooner},
      title = {Proof-Carrying Data from Accumulation Schemes},
      howpublished = {Cryptology {ePrint} Archive, Paper 2020/499},
      year = {2020},
      url = {https://eprint.iacr.org/2020/499}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.