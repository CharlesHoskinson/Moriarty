Poseidon2: A Faster Version of the Poseidon Hash Function


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

#### Paper 2023/323

### [Poseidon2: A Faster Version of the Poseidon Hash Function](/2023/323.pdf)

Lorenzo Grassi, Ponos Technology

Dmitry Khovratovich, Ethereum Foundation

Markus Schofnegger, Horizen Labs

##### Abstract

Zero-knowledge proof systems for computational integrity have seen a rise in popularity in the last couple of years. One of the results of this development is the ongoing effort in designing so-called arithmetization-friendly hash functions in order to make these proofs more efficient. One of these new hash functions, Poseidon, is extensively used in this context, also thanks to being one of the first constructions tailored towards this use case. Many of the design principles of Poseidon have proven to be efficient and were later used in other primitives, yet parts of the construction have shown to be expensive in real-word scenarios.
In this paper, we propose an optimized version of Poseidon, called Poseidon2. The two versions differ in two crucial points. First, Poseidon is a sponge hash function, while Poseidon2 can be either a sponge or a compression function depending on the use case. Secondly, Poseidon2 is instantiated by new and more efficient linear layers with respect to Poseidon. These changes allow to decrease the number of multiplications in the linear layer by up to 90% and the number of constraints in Plonk circuits by up to 70%. This makes Poseidon2 the currently fastest arithmetization-oriented hash function without lookups.
Besides that, we address a recently proposed algebraic attack and propose a simple modification that makes both Poseidon and Poseidon2 secure against this approach.

**Note:** Clarified definition of external matrix and added concrete round numbers for some instances.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/323.pdf)

Category
:   [Cryptographic protocols](/search?category=PROTOCOLS)

Publication info
:   Published elsewhere. Minor revision. AFRICACRYPT 2023

Keywords
:   [Hash Function](/search?q=Hash%20Function)[Poseidon](/search?q=Poseidon)[Poseidon2](/search?q=Poseidon2)[Zero Knowledge](/search?q=Zero%20Knowledge)[Sponge/Compression Mode](/search?q=Sponge/Compression%20Mode)

Contact author(s)
:   lorenzo @ ponos technology  
    khovratovich @ gmail com  
    markus schofnegger @ gmail com

History
:   2024-02-08: last of 4 revisions
:   2023-03-04: received
:   [See all versions](/archive/versions/2023/323)

Short URL
:   <https://ia.cr/2023/323>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/323,
      author = {Lorenzo Grassi and Dmitry Khovratovich and Markus Schofnegger},
      title = {Poseidon2: A Faster Version of the Poseidon Hash Function},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/323},
      year = {2023},
      url = {https://eprint.iacr.org/2023/323}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.