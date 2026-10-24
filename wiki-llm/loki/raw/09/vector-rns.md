Efficient Modular Multiplication Using Vector Instructions on Commodity Hardware


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

#### Paper 2025/1068

### [Efficient Modular Multiplication Using Vector Instructions on Commodity Hardware](/2025/1068.pdf)

Simon Langowski[![](/img/orcid.svg)](https://orcid.org/0000-0003-2408-8441), Massachusetts Institute of Technology

Srini Devadas[![](/img/orcid.svg)](https://orcid.org/0000-0001-8253-7714), Massachusetts Institute of Technology

##### Abstract

Modular arithmetic is the computational backbone of many cryptographic and scientific algorithms.
In particular, modular multiplication in a large prime field is computationally expensive and dictates the runtime of many algorithms. While it is relatively easy to utilize vectorization to accelerate batches of independent modular multiplications, our goal is to reduce the latency of a $\textit{single}$ modular multiplication under a generic prime using vectorization, while maintaining constant-time execution.
We investigate the technique of Residue Number System (RNS) Montgomery modular multiplication. We first contribute a unified view of algorithmic optimizations in prior art. This view enables us to further reduce the number of elementwise multiplications in an algorithm with a simplified structure that we prove correct.
We explore AVX512 acceleration on CPUs, and show how to map our algorithm to vector instructions. We implement our algorithm in C++ and achieve $\approx 4 \times$ speedup, which is nearly maximal, for $1024+$-bit primes on a CPU with AVX512 over optimized library implementations of standard Montgomery modular multiplication algorithms. GPUs contain vector cores that each support tens of physical threads. We show how to intelligently map our algorithm to threads in a vector core, ``overparallelizing'' to minimize latency. We show substantial speedups over a commercial library implementation of standard modular multiplication algorithms across a wide range of prime sizes.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2025/1068.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   Preprint.

Keywords
:   [modular multiplication](/search?q=modular%20multiplication)[residue number system](/search?q=residue%20number%20system)[vector instructions](/search?q=vector%20instructions)[Montgomery multiplication](/search?q=Montgomery%20multiplication)

Contact author(s)
:   slangows @ mit edu  
    devadas @ mit edu

History
:   2025-06-09: approved
:   2025-06-06: received
:   [See all versions](/archive/versions/2025/1068)

Short URL
:   <https://ia.cr/2025/1068>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2025/1068,
      author = {Simon Langowski and Srini Devadas},
      title = {Efficient Modular Multiplication Using Vector Instructions on Commodity Hardware},
      howpublished = {Cryptology {ePrint} Archive, Paper 2025/1068},
      year = {2025},
      url = {https://eprint.iacr.org/2025/1068}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.