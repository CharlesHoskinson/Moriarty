TensorZKP: Repurposing GPU Tensor Cores for High-Performance Zero-Knowledge Proofs


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

#### Paper 2026/1250

### [TensorZKP: Repurposing GPU Tensor Cores for High-Performance Zero-Knowledge Proofs](/2026/1250.pdf)

Tao Lu, National University of Singapore

Jipeng Zhang, National University of Singapore

Yanpei Guo, National University of Singapore

Xuanming Liu, Zhejiang University

Wenjie Qu, National University of Singapore

Zonghui Wang, Zhejiang University

Wenzhi Chen, Zhejiang University

Jiaheng Zhang, National University of Singapore

##### Abstract

GPU Tensor Cores, specialized hardware units designed to accelerate matrix multiplication, have served as the primary engine behind the AI revolution. Given the exponential performance gains they have delivered, aligning cryptographic implementations with this hardware evolution is critical. This is particularly acute for zero-knowledge proofs (ZKPs), a cryptographic primitive that currently grapples with high proof generation costs. Existing GPU implementations for ZKPs rely exclusively on general-purpose SIMT cores, leaving the massive computational power of Tensor Cores untapped.
In this paper, we introduce TensorZKP, the first GPU framework to harness Tensor Cores for ZKP acceleration. Since Tensor Cores are designed for low-precision matrix multiplication, mapping ZKP's arithmetic to this hardware is non-trivial. To bridge this gap, we develop Tensor-Core-compatible finite field arithmetic and reformulate ZKP modules, specifically sum-check protocols and Spielman code, into matrix multiplication tasks. Furthermore, we design an asynchronous warp-specialized framework that pipelines memory access, Tensor Core matrix operations, and SIMT-based modular reductions. We instantiate these optimizations with HyperPlonk as the Polynomial Interactive Oracle Proof (PIOP) and Brakedown as the Polynomial Commitment Scheme (PCS) to enable end-to-end proof generation.
The evaluation results show that TensorZKP exhibits remarkable efficiency. At a $2^{25}$ scale, the underlying building blocks complete in $0.85$ ms for inner product, $0.91$ ms for scalar-vector multiplication, $4.04$ ms for degree-2 sum-check, and $11.58$ ms for the encoder. For a circuit with $2^{25}$ multiplication gates, TensorZKP achieves a proof generation time of only $215.28$ milliseconds, representing a $955\times$ speedup over the CPU baseline and a $36.2\times$ improvement over state-of-the-art SIMT-based GPU implementations.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2026/1250.pdf)

Category
:   [Implementation](/search?category=IMPLEMENTATION)

Publication info
:   Published elsewhere. Minor revision. Usenix Security 2026

Keywords
:   [Tensor Core; GPU; Zero-Knoweledge Proof;](/search?q=Tensor%20Core%3B%20GPU%3B%20Zero-Knoweledge%20Proof%3B)

Contact author(s)
:   lutao @ nus edu sg  
    jipengzhang @ nus edu sg  
    guo yanpei @ u nus edu  
    hinsliu @ zju edu cn  
    wenjiequ @ u nus edu  
    zhwang @ zju edu cn  
    chenwz @ zju edu cn  
    jhzhang @ nus edu sg

History
:   2026-06-16: revised
:   2026-06-13: received
:   [See all versions](/archive/versions/2026/1250)

Short URL
:   <https://ia.cr/2026/1250>

License
:   [![Creative Commons Attribution](/img/license/CC_BY.svg "Creative Commons Attribution")  
    CC BY](https://creativecommons.org/licenses/by/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2026/1250,
      author = {Tao Lu and Jipeng Zhang and Yanpei Guo and Xuanming Liu and Wenjie Qu and Zonghui Wang and Wenzhi Chen and Jiaheng Zhang},
      title = {{TensorZKP}: Repurposing {GPU} Tensor Cores for High-Performance Zero-Knowledge Proofs},
      howpublished = {Cryptology {ePrint} Archive, Paper 2026/1250},
      year = {2026},
      url = {https://eprint.iacr.org/2026/1250}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.