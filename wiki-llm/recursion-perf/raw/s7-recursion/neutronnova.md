# NeutronNova folding
> Source: https://eprint.iacr.org/2024/1606
> Collected: 2026-10-26
> Published: 2024-10-09

NeutronNova: Group-based folding done right


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

#### Paper 2024/1606

### [NeutronNova: Group-based folding done right](/2024/1606.pdf)

Abhiram Kothapalli, Microsoft Research

Srinath Setty, Microsoft Research

##### Abstract

A folding scheme reduces the task of checking multiple NP instances into checking a single instance, providing an efficient route to incrementally verifiable computation (IVC). We identify five desirable properties of a practical folding scheme: constant recursion overheads, multi-folding, linear scaling with the number of instances, no extraneous commitments, and modularity. No existing scheme achieves all five simultaneously.
We introduce NeutronNova, the first folding scheme to achieve all five.
The prover's work is dominated by the cost to commit to its witness -- with no extraneous commitments -- and the recursive verifier performs only three group scalar multiplications and a constant number of hash computations. We construct NeutronNova modularly using the reductions of knowledge (RoK) framework around a core relation called zero-check, and build folding schemes for circuit satisfiability (CCS), grand products, and lookups by reducing each to zero-check. In experiments, NeutronNova folds a SHA-256 circuit with $2^{20}$ constraints in 91 ms, a ${\approx}10\times$ improvement over Nova. Several subsequent works have used NeutronNova to achieve new results, including space-efficient SNARKs with optimal prover time, packed sum-check protocols over small fields, and client-side zero-knowledge proofs with low end-to-end latency.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/1606.pdf)

Category
:   [Foundations](/search?category=FOUNDATIONS)

Publication info
:   A major revision of an IACR publication in CRYPTO 2026

Keywords
:   [folding schemes](/search?q=folding%20schemes)[proof systems](/search?q=proof%20systems)[incrementally verifiable computation](/search?q=incrementally%20verifiable%20computation)[zero-knowledge](/search?q=zero-knowledge)

Contact author(s)
:   akothapalli @ microsoft com  
    srinath @ microsoft com

History
:   2026-08-18: last of 2 revisions
:   2024-10-09: received
:   [See all versions](/archive/versions/2024/1606)

Short URL
:   <https://ia.cr/2024/1606>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/1606,
      author = {Abhiram Kothapalli and Srinath Setty},
      title = {{NeutronNova}: Group-based folding done right},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/1606},
      year = {2024},
      url = {https://eprint.iacr.org/2024/1606}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.
