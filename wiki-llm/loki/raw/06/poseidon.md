Poseidon: A New Hash Function for Zero-Knowledge Proof Systems


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

#### Paper 2019/458

### [Poseidon: A New Hash Function for Zero-Knowledge Proof Systems](/2019/458.pdf)

Lorenzo Grassi

Dmitry Khovratovich

Christian Rechberger

Arnab Roy

Markus Schofnegger

##### Abstract

The area of practical computational integrity proof systems, like SNARKs, STARKs, Bulletproofs, is seeing a very dynamic development with several constructions having appeared recently with improved properties and relaxed setup requirements. Many use cases of such systems involve, often as their most expensive part, proving the knowledge of a preimage under a certain cryptographic hash function, which is expressed as a circuit over a large prime field. A notable example is a zero-knowledge proof of coin ownership in the Zcash cryptocurrency, where the inadequacy of the SHA-256 hash function for such a circuit caused a huge computational penalty.
In this paper, we present a modular framework and concrete instances of cryptographic hash functions which work natively with GF(p) objects. Our hash function Poseidon uses up to 8x fewer constraints per message bit than Pedersen Hash.
Our construction is not only expressed compactly as a circuit, but can also be tailored for various proof systems using specially crafted polynomials, thus bringing another boost in performance. We demonstrate this by implementing a 1-out-of-a-billion membership proof with Merkle trees in less than a second by using Bulletproofs.

**Note:** Updated cryptanalysis with results from follow-up work. This modification \*does not change the round numbers\* in any of the use cases we are aware of, in particular those mentioned in the paper.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2019/458.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Published elsewhere. USENIX Security '21

Keywords
:   [Zero-Knowledge Proof Systems](/search?q=Zero-Knowledge%20Proof%20Systems)[SNARK](/search?q=SNARK)[STARK](/search?q=STARK)[Hash Function](/search?q=Hash%20Function)

Contact author(s)
:   Lorenzo Grassi @ ruhr-uni-bochum de  
    khovratovich @ gmail com  
    christian rechberger @ tugraz at  
    arnab roy @ aau at  
    markus schofnegger @ gmail com

History
:   2023-07-05: last of 11 revisions
:   2019-05-10: received
:   [See all versions](/archive/versions/2019/458)

Short URL
:   <https://ia.cr/2019/458>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2019/458,
      author = {Lorenzo Grassi and Dmitry Khovratovich and Christian Rechberger and Arnab Roy and Markus Schofnegger},
      title = {Poseidon: A New Hash Function for Zero-Knowledge Proof Systems},
      howpublished = {Cryptology {ePrint} Archive, Paper 2019/458},
      year = {2019},
      url = {https://eprint.iacr.org/2019/458}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.