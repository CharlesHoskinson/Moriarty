Two Algorithms for Fast GPU Implementation of NTT


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

#### Paper 2023/1410

### [Two Algorithms for Fast GPU Implementation of NTT](/2023/1410.pdf)

Ali Şah Özcan[![](/img/orcid.svg)](https://orcid.org/0000-0001-9042-2650), Sabanci University

Erkay Savaş[![](/img/orcid.svg)](https://orcid.org/0000-0002-4869-5556), Sabanci University

##### Abstract

The number theoretic transform (NTT) permits a very efficient method to perform multiplication of very large degree polynomials, which is the most time-consuming operation in fully homomorphic encryption (FHE) schemes and a class of non-interactive succinct zero-knowledge proof systems such as zk-SNARK. Efficient modular arithmetic plays an important role in the performance of NTT, and therefore it is studied extensively. The access pattern to the memory, on the other hand, may play much greater role, as the NTT execution time is mostly memory-bound due to large degree polynomials. In this paper, we propose two algorithms for fast computation of NTT on a class of graphical processing units (GPU) by optimizing the memory access patterns. We present an approach i) to optimize the number of accesses to slow global memory for thread synchronization, and ii) to make better use of spatial locality in global memory accesses. It turns out that by controlling certain parameters in CUDA platform for general-purpose GPU computing (GPGPU) such as kernel count, block size and block shape, we can affect the performance of NTT. To best of our knowledge, this work is unique for it suggests a recipe for selecting optimum CUDA parameters to obtain the best NTT performance for a given polynomial degree. Our implementation results on various GPU devices for all power-of-two polynomial degrees from $2^{12}$ to $2^{28}$ show that our algorithms compare favorably with the other state-of-the-art GPU implementations in the literature with the optimum selection of these three CUDA parameters.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2023/1410.pdf)

Category
:   [Implementation](/search?category=IMPLEMENTATION)

Publication info
:   Preprint.

Keywords
:   [Number Theoretic Transform](/search?q=Number%20Theoretic%20Transform)[GPU](/search?q=GPU)[Homomorphic Encryption](/search?q=Homomorphic%20Encryption)[zk-SNARK](/search?q=zk-SNARK)[Hardware Acceleration](/search?q=Hardware%20Acceleration)[Zero Knowledge Proof](/search?q=Zero%20Knowledge%20Proof)

Contact author(s)
:   alisah @ sabanciuniv edu  
    erkays @ sabanciuniv edu

History
:   2023-10-06: revised
:   2023-09-19: received
:   [See all versions](/archive/versions/2023/1410)

Short URL
:   <https://ia.cr/2023/1410>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2023/1410,
      author = {Ali Şah Özcan and Erkay Savaş},
      title = {Two Algorithms for Fast {GPU} Implementation of {NTT}},
      howpublished = {Cryptology {ePrint} Archive, Paper 2023/1410},
      year = {2023},
      url = {https://eprint.iacr.org/2023/1410}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.