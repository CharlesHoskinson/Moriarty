# Cofactor-torsion attacks on hinted scalar multiplications in SNARK circuits
> Source: https://eprint.iacr.org/2026/1776
> Collected: 2026-10-26
> Published: 2026-08-24

Cofactor-torsion attacks on hinted scalar multiplications in SNARK circuits


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

#### Paper 2026/1776

### [Cofactor-torsion attacks on hinted scalar multiplications in SNARK circuits](/2026/1776.pdf)

Youssef El Housni, Consensys, Linea

##### Abstract

Proving an elliptic curve scalar multiplication [k]P= Q ∈ E(Fp) inside a SNARK is much cheaper when the output Q is hinted by the prover and only verified in-circuit, rather than recomputed. The recent scalar multiplication techniques of Eagen, El Housni, Masson and Piellard (Latincrypt 2025) certify a hinted Q with a short lattice reduction (fraction) decomposition of the scalar and a single group identity, and yield the fastest known in-circuit scalar multiplications. We observe that the soundness of this whole family rests on an implicit prime-order hypothesis: the certifying identity is checked over the full group E(Fp), and only when E(Fp) has trivial cofactor does it force Q into the prime-order subgroup. On cofactor curves the certification is unsound. We give two concrete forgery classes against these hinted gadgets: an any-scalar attack that, for a target scalar fixed by the statement, adapts the decomposition so a small rational torsion point cancels from the identity; and a chosen-scalar attack that vanishes the output-side coefficients modulo a small cofactor prime and solves for the scalar afterwards. Both make the gadget accept Q′ = [k]P + T for a non-zero torsion point T as if it were [k]P. We quantify the reachable torsion in terms of the sub-scalar range bound and validate the attacks on widely deployed curves such as BLS12-381, BN254 and BW6-761. The straightforward fix is to check that Q lies in the subgroup but is expensive. We propose a cheaper one that binds the hinted output through a hinted preimage, with the minimal such
constant that suffices against each attack model.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2026/1776.pdf)

Category
:   [Attacks and cryptanalysis](/search?category=ATTACKS)

Publication info
:   Preprint.

Contact author(s)
:   youssef elhousni @ consensys net

History
:   2026-08-24: approved
:   2026-08-22: received
:   [See all versions](/archive/versions/2026/1776)

Short URL
:   <https://ia.cr/2026/1776>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2026/1776,
      author = {Youssef El Housni},
      title = {Cofactor-torsion attacks on hinted scalar multiplications in {SNARK} circuits},
      howpublished = {Cryptology {ePrint} Archive, Paper 2026/1776},
      year = {2026},
      url = {https://eprint.iacr.org/2026/1776}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.
