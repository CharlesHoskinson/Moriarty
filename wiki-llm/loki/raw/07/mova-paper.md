Mova: Nova folding without committing to error terms


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

#### Paper 2024/1220

### [Mova: Nova folding without committing to error terms](/2024/1220.pdf)

Nikolaos Dimitriou, Nethermind Research

Albert Garreta, Nethermind Research

Ignacio Manzur, Nethermind Research

Ilia Vlasov, Nethermind Research

##### Abstract

We present Mova, a folding scheme for R1CS instances that does not require committing to error or cross terms, nor makes use of the sumcheck protocol. We compute concrete costs and provide benchmarks showing that, for reasonable parameter choices, Mova's Prover is about $5$ to $10$ times faster than Nova's Prover, and about $1.05$ to $1.3$ times faster than Hypernova's Prover (applied to R1CS instances) -- assuming the R1CS witness vector contains only small elements. Mova's Verifier has a similar cost as Hypernova's Verifier, but Mova has the advantage of having only $3$ rounds of communication, while Hypernova has a logarithmic number of rounds.
Mova, which is based on the Nova folding scheme, manages to avoid committing to Nova's so-called error term $E$ and cross term $T$ by replacing said commitments with evaluations of the Multilinear Extension (MLE) of $E$ and $T$ at a random point sampled by the Verifier. A key observation used in Mova's soundness proofs is that $E$ is implicitly committed by a commitment to the input-witness vector $Z$, since $E=(A\cdot Z)\circ (B\cdot Z) -u (C\cdot Z)$.
We also note that ProtoGalaxy [EG23] can be specialized to a R1CS folding scheme with similar properties. Some of our further contributions are that 1) Mova is described with a language that sheds new insights into the topic of "Nova-style folding"; 2) we provide concrete costs, benchmarks, and optimizations for the Prover; 3) we describe how to fold two accumulated instances (which is important for applications in Proof Carrying Data); and 4) provide non-trivial knowledge soundness proofs in the context of multilinear polynomials.

**Note:** Updated IACR's abstract and lowered Hypernova's costs

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/1220.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [Zero-knowledge](/search?q=Zero-knowledge)[SNARK](/search?q=SNARK)[Folding scheme](/search?q=Folding%20scheme)[Nova](/search?q=Nova)

Contact author(s)
:   nikolaos @ nethermind io  
    albert @ nethermind io  
    ignacio @ nethermind io  
    ilia @ nethermind io

History
:   2024-08-13: last of 7 revisions
:   2024-07-31: received
:   [See all versions](/archive/versions/2024/1220)

Short URL
:   <https://ia.cr/2024/1220>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/1220,
      author = {Nikolaos Dimitriou and Albert Garreta and Ignacio Manzur and Ilia Vlasov},
      title = {Mova: Nova folding without committing to error terms},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/1220},
      year = {2024},
      url = {https://eprint.iacr.org/2024/1220}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.