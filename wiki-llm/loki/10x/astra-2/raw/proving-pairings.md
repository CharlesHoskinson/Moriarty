On Proving Pairings


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

#### Paper 2024/640

### [On Proving Pairings](/2024/640.pdf)

Andrija Novakovic, Geometry Research

Liam Eagen, Alpen Labs, Zeta Function Technologies

##### Abstract

In this paper we explore efficient ways to prove correctness of elliptic curve pairing relations. Pairing-based cryptographic protocols such as the Groth16 and Plonk SNARKs and the BLS signature scheme are used extensively in public blockchains such as Ethereum due in large part to their small size. However the relatively high cost of pairing computation remains a practical problem for many use cases such as verification ``in circuit" inside a SNARK. This naturally arises in recursive SNARK composition and SNARKs of BLS based consensus protocols.
To improve pairing verification, we first show that the final exponentiation step of pairing verification can be replaced with a more efficient ``residue check," which can be incorporated into the Miller loop. Then, we show how to reduce the cost of the Miller loop by pre-computing all the necessary lines, and how this is especially efficient when the second pairing argument is fixed in advance. This is the case for BLS signatures with a fixed public key, as well as for KZG based SNARKs like Plonk and two of the three Groth16 pairings. Finally, we show how to improve of the protocol of [gar] by combining quotients, which allows us to more efficiently prove higher degree relations. These techniques also carry over naturally to pairing verification, for example on-chain verification or as part of the BitVM(2) protocol for Bitcoin smart contracts. We instantiate algorithms and show results for the BN254 curve.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2024/640.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [elliptic curves](/search?q=elliptic%20curves)[pairings](/search?q=pairings)[zero knowledge proofs](/search?q=zero%20knowledge%20proofs)[SNARKs](/search?q=SNARKs)

Contact author(s)
:   andrija @ geometry dev  
    liameagen @ protonmail com

History
:   2024-04-29: approved
:   2024-04-26: received
:   [See all versions](/archive/versions/2024/640)

Short URL
:   <https://ia.cr/2024/640>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2024/640,
      author = {Andrija Novakovic and Liam Eagen},
      title = {On Proving Pairings},
      howpublished = {Cryptology {ePrint} Archive, Paper 2024/640},
      year = {2024},
      url = {https://eprint.iacr.org/2024/640}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.