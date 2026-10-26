# cq: Cached quotients for fast lookups

> Source: https://eprint.iacr.org/2022/1763
> Collected: 2026-10-26
> Published: 2022-12-25 (received; ePrint history)

cq: Cached quotients for fast lookups


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

#### Paper 2022/1763

### [cq: Cached quotients for fast lookups](/2022/1763.pdf)

Liam Eagen, Blockstream

Dario Fiore, IMDEA Software

Ariel Gabizon, Zeta Function Technologies

##### Abstract

We present a protocol called $\mathsf{cq}$ for checking the values of a committed polynomial $f(X)\in \mathbb{F}\_{<n}(X)$ over a multiplicative subgroup $H\subset \mathbb{F}$ of size $n$ are contained in a table $t\in \mathbb{F}^N$. After an $O(N \log N)$ preprocessing step, the prover algorithm runs in time $O(n\log n)$.
Thus, we continue to improve upon the recent breakthrough sequence of results [ZBKMNS,PK,GK,ZGKMR] starting from $\mathsf{Caulk}$ [ZBKMNS], which achieve sublinear complexity in the table size $N$. The two most recent works in this sequence $\mathsf{Ba}\mathit{loo}$ [ZGKMR] and $\mathsf{flookup}$ [GK] achieved prover complexity $O(n\log^2 n)$.
Moreover, $\mathsf{cq}$ has the following attractive features.
1. As in [ZBKMNS,PK,ZGKMR] our construction relies on homomorphic table commitments, which makes them amenable to vector lookups.
2. As opposed to the previous four works, our verifier doesn't involve pairings with prover defined $\mathbb{G}\_2$ points, which makes recursive aggregation of proofs more convenient.

**Note:** Fixes by Marek

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2022/1763.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Preprint.

Keywords
:   [zk-snarks](/search?q=zk-snarks)[lookup protocols](/search?q=lookup%20protocols)

Contact author(s)
:   ariel gabizon @ gmail com

History
:   2024-07-05: last of 5 revisions
:   2022-12-25: received
:   [See all versions](/archive/versions/2022/1763)

Short URL
:   <https://ia.cr/2022/1763>

License
:   [![No rights reserved](/img/license/CC0.svg "No rights reserved")  
    CC0](https://creativecommons.org/publicdomain/zero/1.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2022/1763,
      author = {Liam Eagen and Dario Fiore and Ariel Gabizon},
      title = {cq: Cached quotients for fast lookups},
      howpublished = {Cryptology {ePrint} Archive, Paper 2022/1763},
      year = {2022},
      url = {https://eprint.iacr.org/2022/1763}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.