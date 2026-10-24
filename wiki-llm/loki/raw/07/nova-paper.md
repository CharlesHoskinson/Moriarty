Nova: Recursive Zero-Knowledge Arguments from Folding Schemes


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

#### Paper 2021/370

### [Nova: Recursive Zero-Knowledge Arguments from Folding Schemes](/2021/370.pdf)

Abhiram Kothapalli, Carnegie Mellon University

Srinath Setty, Microsoft Research

Ioanna Tzialla, New York University

##### Abstract

We introduce a new approach to realize incrementally verifiable computation (IVC), in which the prover recursively proves the correct execution of incremental computations of the form $y=F^{(\ell)}(x)$, where $F$ is a (potentially non-deterministic) computation, $x$ is the input, $y$ is the output, and $\ell > 0$. Unlike prior approaches to realize IVC, our approach avoids succinct non-interactive arguments of knowledge (SNARKs) entirely and arguments of knowledge in general. Instead, we introduce and employ folding schemes, a weaker, simpler, and more efficiently-realizable primitive, which reduces the task of checking two instances in some relation to the task of checking a single instance. We construct a folding scheme for a characterization of $\mathsf{NP}$ and show that it implies an IVC scheme with improved efficiency characteristics: (1) the "recursion overhead" (i.e., the number of steps that the prover proves in addition to proving the execution of $F$) is a constant and it is dominated by two group scalar multiplications expressed as a circuit (this is the smallest recursion overhead in the literature), and (2) the prover's work at each step is dominated by two multiexponentiations of size $O(|F|)$, providing the fastest prover in the literature. The size of a proof is $O(|F|)$ group elements, but we show that using a variant of an existing zkSNARK, the prover can prove the knowledge of a valid proof succinctly and in zero-knowledge with $O(\log{|F|})$ group elements. Finally, our approach neither requires a trusted setup nor FFTs, so it can be instantiated efficiently with any cycles of elliptic curves where DLOG is hard.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2021/370.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   A major revision of an IACR publication in CRYPTO 2022

Keywords
:   [incrementally verifiable computation](/search?q=incrementally%20verifiable%20computation)[zero knowledge arguments](/search?q=zero%20knowledge%20arguments)[recursive proof composition](/search?q=recursive%20proof%20composition)

Contact author(s)
:   akothapa @ andrew cmu edu  
    srinath @ microsoft com  
    it608 @ nyu edu

History
:   2024-07-20: last of 2 revisions
:   2021-03-22: received
:   [See all versions](/archive/versions/2021/370)

Short URL
:   <https://ia.cr/2021/370>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2021/370,
      author = {Abhiram Kothapalli and Srinath Setty and Ioanna Tzialla},
      title = {Nova: Recursive Zero-Knowledge Arguments from Folding Schemes},
      howpublished = {Cryptology {ePrint} Archive, Paper 2021/370},
      year = {2021},
      url = {https://eprint.iacr.org/2021/370}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.