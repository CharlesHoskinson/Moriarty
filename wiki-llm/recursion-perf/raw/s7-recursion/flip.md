# FLIP-and-Prove R1CS
> Source: https://eprint.iacr.org/2024/1364
> Collected: 2026-10-26
> Published: 2024-08-29

FLIP-and-prove R1CS


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

#### Paper 2024/1364

### [FLIP-and-prove R1CS](/2024/1364.pdf)

Anca Nitulescu[![](/img/orcid.svg)](https://orcid.org/0000-0001-9149-2036), Input Output

Nikitas Paslis, Universitat Pompeu Fabra

Carla Ràfols[![](/img/orcid.svg)](https://orcid.org/0000-0002-7035-9049)

##### Abstract

We present the first folding framework that achieves sublinear verification and communication when a single prover must convince a verifier of $k$ independent R1CS instances.
- $\mathbf{FLIP}$ (Fold-Inner-Product) folds the $k$ instance-witness pairs in only $\log k$ rounds. Built on the homomorphic two-tier commitment of Abe et al. (CRYPTO 2010), FLIP transmits $O(\log k)$ group elements.
- $\mathbf{r \operatorname{-} Groth}$ is a commit-and-prove variant of Groth16 that natively handles relaxed R1CS. It retains Groth16’s three-element proof and two pairing checks, requires only a slightly modified (instance-independent) trusted setup, and does not rely on elliptic-curve cycles or foreign-field arithmetic.
Combined, FLIP + r-Groth replace the $k-1$ extra Groth16 proofs demanded by aggregation schemes and avoid the heavy verifier-in-circuit logic of recursive systems. The total prover work is essentially one Groth16 run plus light folding, while the verifier processes $O(\log k)$ group elements and two pairings.
This design is immediately applicable to roll-ups, Proof-of-Space, and other "proving-as-a-service" scenarios where all witnesses reside on a single machine.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/1364.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Published by the IACR in CIC 2025

DOI
:   [https://doi.org/10.62056/ayc3tx4e- ![](/img/open-outline.svg)](https://doi.org/https://doi.org/10.62056/ayc3tx4e-)

Keywords
:   [zero-knowledge proofs](/search?q=zero-knowledge%20proofs)[SNARKs](/search?q=SNARKs)[blockchain](/search?q=blockchain)

Contact author(s)
:   anca nitulescu @ iohk io  
    nikitas paslis @ upf edu  
    carla rafols @ upf edu

History
:   2026-08-28: revised
:   2024-08-29: received
:   [See all versions](/archive/versions/2024/1364)

Short URL
:   <https://ia.cr/2024/1364>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/1364,
      author = {Anca Nitulescu and Nikitas Paslis and Carla Ràfols},
      title = {{FLIP}-and-prove {R1CS}},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/1364},
      year = {2024},
      doi = {https://doi.org/10.62056/ayc3tx4e-},
      url = {https://eprint.iacr.org/2024/1364}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.
