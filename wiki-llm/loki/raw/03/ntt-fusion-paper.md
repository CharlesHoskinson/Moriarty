[2209.01290] Accelerating Polynomial Multiplication for Homomorphic Encryption on GPUs

[Skip to main content](#content)
[![archive](https://static.arxiv.org/static/design-system/latest/logos/arxiv-logo.svg)](https://arxiv.org/)
[Search](https://arxiv.org/search)


[Submit](https://arxiv.org/submit)
[Donate](https://info.arxiv.org/about/donate.html)

[Log in](https://arxiv.org/login) 

Search arXiv

Press Enter to search · [Advanced search](https://arxiv.org/search/advanced)

Computer Science > Cryptography and Security
============================================

**arXiv:2209.01290** (cs)

[Submitted on 2 Sep 2022]

Title:Accelerating Polynomial Multiplication for Homomorphic Encryption on GPUs
===============================================================================

Authors:[Kaustubh Shivdikar](https://arxiv.org/search/cs?searchtype=author&query=Shivdikar,+K), [Gilbert Jonatan](https://arxiv.org/search/cs?searchtype=author&query=Jonatan,+G), [Evelio Mora](https://arxiv.org/search/cs?searchtype=author&query=Mora,+E), [Neal Livesay](https://arxiv.org/search/cs?searchtype=author&query=Livesay,+N), [Rashmi Agrawal](https://arxiv.org/search/cs?searchtype=author&query=Agrawal,+R), [Ajay Joshi](https://arxiv.org/search/cs?searchtype=author&query=Joshi,+A), [Jose Abellan](https://arxiv.org/search/cs?searchtype=author&query=Abellan,+J), [John Kim](https://arxiv.org/search/cs?searchtype=author&query=Kim,+J), [David Kaeli](https://arxiv.org/search/cs?searchtype=author&query=Kaeli,+D)

View a PDF of the paper titled Accelerating Polynomial Multiplication for Homomorphic Encryption on GPUs, by Kaustubh Shivdikar and 7 other authors

[View PDF](/pdf/2209.01290)
[HTML (experimental)](https://arxiv.org/html/2209.01290v1)
> Abstract:Homomorphic Encryption (HE) enables users to securely outsource both the storage and computation of sensitive data to untrusted servers. Not only does HE offer an attractive solution for security in cloud systems, but lattice-based HE systems are also believed to be resistant to attacks by quantum computers. However, current HE implementations suffer from prohibitively high latency. For lattice-based HE to become viable for real-world systems, it is necessary for the key bottlenecks - particularly polynomial multiplication - to be highly efficient.
>   
> In this paper, we present a characterization of GPU-based implementations of polynomial multiplication. We begin with a survey of modular reduction techniques and analyze several variants of the widely-used Barrett modular reduction algorithm. We then propose a modular reduction variant optimized for 64-bit integer words on the GPU, obtaining a 1.8x speedup over the existing comparable implementations. Next, we explore the following GPU-specific improvements for polynomial multiplication targeted at optimizing latency and throughput: 1) We present a 2D mixed-radix, multi-block implementation of NTT that results in a 1.85x average speedup over the previous state-of-the-art. 2) We explore shared memory optimizations aimed at reducing redundant memory accesses, further improving speedups by 1.2x. 3) Finally, we fuse the Hadamard product with neighboring stages of the NTT, reducing the twiddle factor memory footprint by 50%. By combining our NTT optimizations, we achieve an overall speedup of 123.13x and 2.37x over the previous state-of-the-art CPU and GPU implementations of NTT kernels, respectively.

|  |  |
| --- | --- |
| Comments: | Accepted, to be pusblished at SEED 2022 conference (IEEE International Symposium on Secure and Private Execution Environment Design) |
| Subjects: | Cryptography and Security (cs.CR); Hardware Architecture (cs.AR); Distributed, Parallel, and Cluster Computing (cs.DC); Performance (cs.PF) |
| Cite as: | [arXiv:2209.01290](https://arxiv.org/abs/2209.01290) [cs.CR] |
|  | (or  [arXiv:2209.01290v1](https://arxiv.org/abs/2209.01290v1) [cs.CR] for this version) |
|  | <https://doi.org/10.48550/arXiv.2209.01290> Focus to learn more  arXiv-issued DOI via DataCite |

Submission history
------------------

From: Kaustubh Shivdikar [[view email](/show-email/6197edcc/2209.01290)]   
**[v1]**
Fri, 2 Sep 2022 22:28:51 UTC (5,859 KB)

Full-text links:

Access Paper:
-------------

View a PDF of the paper titled Accelerating Polynomial Multiplication for Homomorphic Encryption on GPUs, by Kaustubh Shivdikar and 7 other authors

* [View PDF](/pdf/2209.01290)
* [HTML (experimental)](https://arxiv.org/html/2209.01290v1)
* [TeX Source](/src/2209.01290)

[![license icon](https://arxiv.org/icons/licenses/by-4.0.png)view license](http://creativecommons.org/licenses/by/4.0/ "Rights to this article")

### Additional Features

* [Audio Summary](/audio/2209.01290)

### Current browse context:

cs.CR

[< prev](/prevnext?id=2209.01290&function=prev&context=cs.CR "previous in cs.CR (accesskey p)")
  |   
[next >](/prevnext?id=2209.01290&function=next&context=cs.CR "next in cs.CR (accesskey n)")

[new](/list/cs.CR/new)
 | 
[recent](/list/cs.CR/recent)
 | [2022-09](/list/cs.CR/2022-09)

Change to browse by:

[cs](/abs/2209.01290?context=cs)  
[cs.AR](/abs/2209.01290?context=cs.AR)  
[cs.DC](/abs/2209.01290?context=cs.DC)  
[cs.PF](/abs/2209.01290?context=cs.PF)

### References & Citations

* [SciX](https://scixplorer.org/abs/arXiv:2209.01290)
* [Google Scholar](https://scholar.google.com/scholar_lookup?arxiv_id=2209.01290)
* [Semantic Scholar](https://api.semanticscholar.org/arXiv:2209.01290)

export BibTeX citation
Loading...

BibTeX formatted citation
-------------------------

×

loading...

Data provided by:

### Bookmark

[![BibSonomy](/static/browse/0.3.4/images/icons/social/bibsonomy.png)](http://www.bibsonomy.org/BibtexHandler?requTask=upload&url=https://arxiv.org/abs/2209.01290&description=Accelerating%20Polynomial%20Multiplication%20for%20Homomorphic%20Encryption%20on%20GPUs "Bookmark on BibSonomy")
[![Reddit](/static/browse/0.3.4/images/icons/social/reddit.png)](https://reddit.com/submit?url=https://arxiv.org/abs/2209.01290&title=Accelerating%20Polynomial%20Multiplication%20for%20Homomorphic%20Encryption%20on%20GPUs "Bookmark on Reddit")

Bibliographic Tools

Bibliographic and Citation Tools
================================

Bibliographic Explorer Toggle

Bibliographic Explorer *([What is the Explorer?](https://info.arxiv.org/labs/showcase.html#arxiv-bibliographic-explorer))*

Connected Papers Toggle

Connected Papers *([What is Connected Papers?](https://www.connectedpapers.com/about))*

Litmaps Toggle

Litmaps *([What is Litmaps?](https://www.litmaps.co/))*

scite.ai Toggle

scite Smart Citations *([What are Smart Citations?](https://www.scite.ai/))*

Code, Data, Media

Code, Data and Media Associated with this Article
=================================================

alphaXiv Toggle

alphaXiv *([What is alphaXiv?](https://alphaxiv.org/))*

Links to Code Toggle

CatalyzeX Code Finder for Papers *([What is CatalyzeX?](https://www.catalyzex.com))*

DagsHub Toggle

DagsHub *([What is DagsHub?](https://dagshub.com/))*

GotitPub Toggle

Gotit.pub *([What is GotitPub?](http://gotit.pub/faq))*

Huggingface Toggle

Hugging Face *([What is Huggingface?](https://huggingface.co/huggingface))*

ScienceCast Toggle

ScienceCast *([What is ScienceCast?](https://sciencecast.org/welcome))*

Demos

Demos
=====

Replicate Toggle

Replicate *([What is Replicate?](https://replicate.com/docs/arxiv/about))*

Spaces Toggle

Hugging Face Spaces *([What is Spaces?](https://huggingface.co/docs/hub/spaces))*

Spaces Toggle

TXYZ.AI *([What is TXYZ.AI?](https://txyz.ai))*

Related Papers

Recommenders and Search Tools
=============================

Link to Influence Flower

Influence Flower *([What are Influence Flowers?](https://influencemap.cmlab.dev/))*

Core recommender toggle

CORE Recommender *([What is CORE?](https://core.ac.uk/services/recommender))*

* Author
* Venue
* Institution
* Topic

About arXivLabs

arXivLabs: experimental projects with community collaborators
=============================================================

arXivLabs is a framework that allows collaborators to develop and share new arXiv features directly on our website.

Both individuals and organizations that work with arXivLabs have embraced and accepted our values of openness, community, excellence, and user data privacy. arXiv is committed to these values and only works with partners that adhere to them.

Have an idea for a project that will add value for arXiv's community? [**Learn more about arXivLabs**](https://info.arxiv.org/labs/index.html).

[Which authors of this paper are endorsers?](/auth/show-endorsers/2209.01290) |
[Disable MathJax](javascript:setMathjaxCookie()) ([What is MathJax?](https://info.arxiv.org/help/mathjax.html))

We gratefully acknowledge support from our **major funders**,
[**member institutions**](https://info.arxiv.org/about/ourmembers.html), ,
and all contributors.

[About](https://info.arxiv.org/about) ·
[Help](https://info.arxiv.org/help) ·
[Contact](https://info.arxiv.org/help/contact.html) ·
[Subscribe](https://info.arxiv.org/help/subscribe.html) ·
[Copyright](https://info.arxiv.org/help/license/index.html) ·
[Privacy](https://info.arxiv.org/help/policies/privacy_policy.html) ·
[Accessibility](https://info.arxiv.org/help/web_accessibility.html) ·
[Operational Status (opens in new tab)](https://status.arxiv.org/)

Major funding support from

[![Simons Foundation](https://static.arxiv.org/static/design-system/latest/funders/simons-foundation.png) (opens in new tab)](https://www.simonsfoundation.org/)
[![Simons Foundation International](https://static.arxiv.org/static/design-system/latest/funders/simons-foundation-international.png) (opens in new tab)](https://www.sfi.org.bm/)
[![Schmidt Sciences](https://static.arxiv.org/static/design-system/latest/funders/schmidt-sciences.png) (opens in new tab)](https://www.schmidtsciences.org/)