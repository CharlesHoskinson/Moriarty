SuperNova: Proving universal machine executions without universal circuits


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

#### Paper 2022/1758

### [SuperNova: Proving universal machine executions without universal circuits](/2022/1758.pdf)

Abhiram Kothapalli, Carnegie Mellon University

Srinath Setty, Microsoft Research

##### Abstract

This paper introduces SuperNova, a new recursive proof system for incrementally producing succinct proofs of correct execution of programs on a stateful machine with a particular instruction set (e.g., EVM, RISC-V). A distinguishing aspect of SuperNova is that the cost of proving a step of a program is proportional only to the size of the circuit representing the instruction invoked by the program step. This is a stark departure from prior works that employ universal circuits where the cost of proving a program step is proportional at least to the sum of sizes of circuits representing each supported instruction—even though a particular program step invokes only one of the supported instructions. Naturally, SuperNova can support a rich instruction set without affecting the per-step proving costs. SuperNova achieves its cost profile by building on Nova, a prior high-speed recursive proof system, and leveraging its internal building block, folding schemes, in a new manner. We formalize SuperNova’s approach as a way to realize non-uniform IVC, a generalization of IVC. Furthermore, SuperNova’s prover costs and the recursion overhead are the same as Nova’s, and in fact, SuperNova is equivalent to Nova for machines that support a single instruction.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2022/1758.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   Preprint.

Keywords
:   [incrementally verifiable computation](/search?q=incrementally%20verifiable%20computation)[recursive proof composition](/search?q=recursive%20proof%20composition)[zero-knowledge arguments](/search?q=zero-knowledge%20arguments)

Contact author(s)
:   akothapa @ andrew cmu edu  
    srinath @ microsoft com

History
:   2022-12-27: approved
:   2022-12-22: received
:   [See all versions](/archive/versions/2022/1758)

Short URL
:   <https://ia.cr/2022/1758>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2022/1758,
      author = {Abhiram Kothapalli and Srinath Setty},
      title = {{SuperNova}: Proving universal machine executions without universal circuits},
      howpublished = {Cryptology {ePrint} Archive, Paper 2022/1758},
      year = {2022},
      url = {https://eprint.iacr.org/2022/1758}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.