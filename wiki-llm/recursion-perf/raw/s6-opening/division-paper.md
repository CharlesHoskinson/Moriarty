# Improved Polynomial Division in Cryptography
> Source: https://eprint.iacr.org/2024/1279
> Collected: 2026-10-26
> Published: 2024-08-13

Improved Polynomial Division in Cryptography


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

#### Paper 2024/1279

### [Improved Polynomial Division in Cryptography](/2024/1279.pdf)

Kostas Kryptos Chalkias[![](/img/orcid.svg)](https://orcid.org/0000-0002-3252-9975), Mysten Labs

Charanjit Jutla, IBM Research - Thomas J. Watson Research Center

Jonas Lindstrom[![](/img/orcid.svg)](https://orcid.org/0000-0002-1989-3019), Mysten Labs

Varun Madathil[![](/img/orcid.svg)](https://orcid.org/0009-0001-4390-3096), North Carolina State University

Arnab Roy[![](/img/orcid.svg)](https://orcid.org/0009-0005-3770-9982), Mysten Labs

##### Abstract

Several cryptographic primitives, especially succinct proofs of various forms, transform the satisfaction of high-level properties to the existence of a polynomial quotient between a polynomial that interpolates a set of values with a cleverly arranged divisor. Some examples are SNARKs, like Groth16, and polynomial commitments, such as KZG. Such a polynomial division naively takes $O(n \log n)$ time with Fast Fourier Transforms, and is usually the asymptotic bottleneck for these computations.
Several works have targeted specific constructions to optimize these computations and trade-off one-time setup costs with faster online computation times. In this paper, we present a unified approach to polynomial division related computations for a diverse set of schemes. We show how our approach provides a common abstract lens which recasts and improves existing approaches. Additionally, we present benchmarks for the Groth16 and the KZG systems, illustrating the significant practical benefits of our approach in terms of speed, memory, and parallelizability. We get a speedup of $2\times$ over the state-of-the-art in computing all openings for KZG commitments and a speed-up of about $2-3\%$ for Groth16 proofs when compared against the Rust Arkworks implementation. Although our Groth16 speedup is modest, our approach supports twice the number of gates as Arkworks and SnarkJS as it avoids computations at higher roots of unity. Conversely this reduces the need for employing larger groups for bigger circuits. For example, our approach can support $2^{28}$ gates with BN254, as compared to $2^{27}$ for coset-based approaches, without sacrificing computational advantages.
Our core technical contributions are novel conjugate representations and compositions of the derivative operator and point-wise division under the Discrete Fourier Transform. These allow us to leverage l'Hôpital's rule to efficiently compute polynomial division, where in the evaluation basis such divisions maybe of the form $0/0$. Our techniques are generic with potential applicability to many existing protocols.

**Note:** Added Inner Product Arguments, and detailed examples.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/1279.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [Vector Commitments](/search?q=Vector%20Commitments)[SNARKs](/search?q=SNARKs)[Zero-Knowledge](/search?q=Zero-Knowledge)[Mathematical foundations](/search?q=Mathematical%20foundations)

Contact author(s)
:   kostas @ mystenlabs com  
    csjutla @ us ibm com  
    jonas @ mystenlabs com  
    vrmadath @ ncsu edu  
    arnabr @ gmail com

History
:   2024-10-18: revised
:   2024-08-13: received
:   [See all versions](/archive/versions/2024/1279)

Short URL
:   <https://ia.cr/2024/1279>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/1279,
      author = {Kostas Kryptos Chalkias and Charanjit Jutla and Jonas Lindstrom and Varun Madathil and Arnab Roy},
      title = {Improved Polynomial Division in Cryptography},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/1279},
      year = {2024},
      url = {https://eprint.iacr.org/2024/1279}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.
