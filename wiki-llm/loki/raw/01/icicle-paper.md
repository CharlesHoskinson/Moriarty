ICICLE v2: Polynomial API for Coding ZK Provers to Run on Specialized Hardware


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

#### Paper 2024/973

### [ICICLE v2: Polynomial API for Coding ZK Provers to Run on Specialized Hardware](/2024/973.pdf)

Karthik Inbasekar[![](/img/orcid.svg)](https://orcid.org/0000-0002-1020-1397), Ingonyama

Yuval Shekel, Ingonyama

Michael Asa, Ingonyama

##### Abstract

Polynomials play a central role in cryptography. In the context of Zero Knowledge Proofs (ZKPs), protocols can be exclusively expressed using polynomials, making them a powerful abstraction tool, as demonstrated in most ZK research papers. Our first contribution is a high-level framework that enables practitioners to implement ZKPs in a more natural way, based solely on polynomial primitives.
ZK provers are considered computationally intensive algorithms with a high degree of parallelization. These algorithms benefit significantly from hardware acceleration, and deployed ZK systems typically include specialized hardware to optimize the performance of the prover code. Our second contribution is leveraging our polynomial API to abstract away low-level hardware primitives and automate their memory management. This device-agnostic design allows ZK engineers to prototype and build solutions while taking advantage of the performance gains offered by specialized hardware, such as GPUs and FPGAs, without needing to understand the hardware implementation details.
Finally, our polynomial API is integrated into version 2 of the ICICLE library and is running in production. This paper also serves as a comprehensive documentation for the ICICLE v2 polynomial API.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/973.pdf)

Category
:   [Applications](/search?category=APPLICATIONS)

Publication info
:   Preprint.

Keywords
:   [ZKP](/search?q=ZKP)[Hardware Acceleration](/search?q=Hardware%20Acceleration)[Polynomial API](/search?q=Polynomial%20API)

Contact author(s)
:   karthik @ ingonyama com  
    yuval shekel @ ingonyama com  
    miki asa @ ingonyama com

History
:   2024-06-17: approved
:   2024-06-16: received
:   [See all versions](/archive/versions/2024/973)

Short URL
:   <https://ia.cr/2024/973>

License
:   [![Creative Commons Attribution-NonCommercial](/img/license/CC_BY_NC.svg "Creative Commons Attribution-NonCommercial")  
    CC BY-NC](https://creativecommons.org/licenses/by-nc/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/973,
      author = {Karthik Inbasekar and Yuval Shekel and Michael Asa},
      title = {{ICICLE} v2: Polynomial {API} for Coding {ZK} Provers to Run on Specialized Hardware},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/973},
      year = {2024},
      url = {https://eprint.iacr.org/2024/973}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.