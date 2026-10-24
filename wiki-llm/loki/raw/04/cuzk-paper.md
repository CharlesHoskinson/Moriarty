cuZK: Accelerating Zero-Knowledge Proof with A Faster Parallel Multi-Scalar Multiplication Algorithm on GPUs


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

#### Paper 2022/1321

### [cuZK: Accelerating Zero-Knowledge Proof with A Faster Parallel Multi-Scalar Multiplication Algorithm on GPUs](/2022/1321.pdf)

Tao Lu[![](/img/orcid.svg)](https://orcid.org/0000-0002-4633-6058), Zhejiang University

Chengkun Wei[![](/img/orcid.svg)](https://orcid.org/0000-0001-8849-8808), Zhejiang University

Ruijing Yu, Zhejiang University

Chaochao Chen, Zhejiang University

Wenjing Fang, Ant Group

Lei Wang, Ant Group

Zeke Wang[![](/img/orcid.svg)](https://orcid.org/0000-0001-8550-9241), Zhejiang University

Wenzhi Chen[![](/img/orcid.svg)](https://orcid.org/0000-0003-1674-4701), Zhejiang University

##### Abstract

Zero-knowledge proof is a critical cryptographic primitive. Its most practical type, called zero-knowledge Succinct Non-interactive ARgument of Knowledge (zkSNARK), has been deployed in various privacy-preserving applications such as cryptocurrencies and verifiable machine learning. Unfortunately, zkSNARK like Groth16 has a high overhead on its proof generation step, which consists of several time-consuming operations, including large-scale matrix-vector multiplication (MUL), number-theoretic transform (NTT), and multi-scalar multiplication (MSM). Therefore, this paper presents cuZK, an efficient GPU implementation of zkSNARK with the following three techniques to achieve high performance. First, we propose a new parallel MSM algorithm. This MSM algorithm achieves nearly perfect linear speedup over the Pippenger algorithm, a well-known serial MSM algorithm. Second, we parallelize the MUL operation. Along with our self-designed MSM scheme and well-studied NTT scheme, cuZK achieves the parallelization of all operations in the proof generation step. Third, cuZK reduces the latency overhead caused by CPU-GPU data transfer by 1) reducing redundant data transfer and 2) overlapping data transfer and device computation. The evaluation results show that our MSM module provides over 2.08× (up to 2.94×) speedup versus the state-of-the-art GPU implementation. cuZK achieves over 2.65× (up to 4.86×) speedup on standard benchmarks and 2.18× speedup on a GPU-accelerated cryptocurrency application, Filecoin.

##### Metadata

Available format(s)
:   [![](/img/file-pdf.svg)PDF](/2022/1321.pdf)

Category
:   [Implementation](/search?category=IMPLEMENTATION)

Publication info
:   Published by the IACR in TCHES 2023

Keywords
:   [Zero-knowledge proof](/search?q=Zero-knowledge%20proof)[parallel algorithm](/search?q=parallel%20algorithm)[multi-scalar multiplication](/search?q=multi-scalar%20multiplication)[zkSNARK](/search?q=zkSNARK)[GPU](/search?q=GPU)

Contact author(s)
:   lutao2020 @ zju edu cn  
    weichengkun @ zju edu cn  
    rjyu @ zju edu cn  
    zjuccc @ zju edu cn  
    bean fwj @ antgroup com  
    shensi wl @ antgroup com  
    wangzeke @ zju edu cn  
    chenwz @ zju edu cn

History
:   2023-04-15: last of 4 revisions
:   2022-10-05: received
:   [See all versions](/archive/versions/2022/1321)

Short URL
:   <https://ia.cr/2022/1321>

License
:   [![Creative Commons Attribution-ShareAlike](/img/license/CC_BY_SA.svg "Creative Commons Attribution-ShareAlike")  
    CC BY-SA](https://creativecommons.org/licenses/by-sa/4.0/)

**BibTeX** 
![](/img/copy-outline.svg)Copy to clipboard

```
@misc{cryptoeprint:2022/1321,
      author = {Tao Lu and Chengkun Wei and Ruijing Yu and Chaochao Chen and Wenjing Fang and Lei Wang and Zeke Wang and Wenzhi Chen},
      title = {{cuZK}: Accelerating Zero-Knowledge Proof with A Faster Parallel Multi-Scalar Multiplication Algorithm on {GPUs}},
      howpublished = {Cryptology {ePrint} Archive, Paper 2022/1321},
      year = {2022},
      url = {https://eprint.iacr.org/2022/1321}
}
```

[![IACR Logo](/img/iacrlogo_small.png)](https://iacr.org/)

Note: In order to protect the privacy of readers, eprint.iacr.org
does not use cookies or embedded third party content.