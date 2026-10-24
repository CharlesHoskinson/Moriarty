[2509.22684] ZKProphet: Understanding Performance of Zero-Knowledge Proofs on GPUs

[Skip to main content](#content)
[![archive](https://static.arxiv.org/static/design-system/latest/logos/arxiv-logo.svg)](https://arxiv.org/)
[Search](https://arxiv.org/search)


[Submit](https://arxiv.org/submit)
[Donate](https://info.arxiv.org/about/donate.html)

[Log in](https://arxiv.org/login) 

Search arXiv

Press Enter to search · [Advanced search](https://arxiv.org/search/advanced)

Computer Science > Distributed, Parallel, and Cluster Computing
===============================================================

**arXiv:2509.22684** (cs)

[Submitted on 17 Sep 2025]

Title:ZKProphet: Understanding Performance of Zero-Knowledge Proofs on GPUs
===========================================================================

Authors:[Tarunesh Verma](https://arxiv.org/search/cs?searchtype=author&query=Verma,+T) (1), [Yichao Yuan](https://arxiv.org/search/cs?searchtype=author&query=Yuan,+Y) (1), [Nishil Talati](https://arxiv.org/search/cs?searchtype=author&query=Talati,+N) (1), [Todd Austin](https://arxiv.org/search/cs?searchtype=author&query=Austin,+T) (1) ((1) Computer Science and Engineering, University of Michigan, USA)

View a PDF of the paper titled ZKProphet: Understanding Performance of Zero-Knowledge Proofs on GPUs, by Tarunesh Verma (1) and 5 other authors

[View PDF](/pdf/2509.22684)
[HTML (experimental)](https://arxiv.org/html/2509.22684v1)
> Abstract:Zero-Knowledge Proofs (ZKP) are protocols which construct cryptographic proofs to demonstrate knowledge of a secret input in a computation without revealing any information about the secret. ZKPs enable novel applications in private and verifiable computing such as anonymized cryptocurrencies and blockchain scaling and have seen adoption in several real-world systems. Prior work has accelerated ZKPs on GPUs by leveraging the inherent parallelism in core computation kernels like Multi-Scalar Multiplication (MSM). However, we find that a systematic characterization of execution bottlenecks in ZKPs, as well as their scalability on modern GPU architectures, is missing in the literature. This paper presents ZKProphet, a comprehensive performance study of Zero-Knowledge Proofs on GPUs. Following massive speedups of MSM, we find that ZKPs are bottlenecked by kernels like Number-Theoretic Transform (NTT), as they account for up to 90% of the proof generation latency on GPUs when paired with optimized MSM implementations. Available NTT implementations under-utilize GPU compute resources and often do not employ architectural features like asynchronous compute and memory operations. We observe that the arithmetic operations underlying ZKPs execute exclusively on the GPU's 32-bit integer pipeline and exhibit limited instruction-level parallelism due to data dependencies. Their performance is thus limited by the available integer compute units. While one way to scale the performance of ZKPs is adding more compute units, we discuss how runtime parameter tuning for optimizations like precomputed inputs and alternative data representations can extract additional speedup. With this work, we provide the ZKP community a roadmap to scale performance on GPUs and construct definitive GPU-accelerated ZKPs for their application requirements and available hardware resources.

|  |  |
| --- | --- |
| Comments: | To appear at 2025 IEEE International Symposium on Workload Characterization |
| Subjects: | Distributed, Parallel, and Cluster Computing (cs.DC); Hardware Architecture (cs.AR); Cryptography and Security (cs.CR); Performance (cs.PF) |
| Cite as: | [arXiv:2509.22684](https://arxiv.org/abs/2509.22684) [cs.DC] |
|  | (or  [arXiv:2509.22684v1](https://arxiv.org/abs/2509.22684v1) [cs.DC] for this version) |
|  | <https://doi.org/10.48550/arXiv.2509.22684> Focus to learn more  arXiv-issued DOI via DataCite |

Submission history
------------------

From: Tarunesh Verma [[view email](/show-email/1f237531/2509.22684)]   
**[v1]**
Wed, 17 Sep 2025 22:52:19 UTC (287 KB)

Full-text links:

Access Paper:
-------------

View a PDF of the paper titled ZKProphet: Understanding Performance of Zero-Knowledge Proofs on GPUs, by Tarunesh Verma (1) and 5 other authors

* [View PDF](/pdf/2509.22684)
* [HTML (experimental)](https://arxiv.org/html/2509.22684v1)
* [TeX Source](/src/2509.22684)

[![license icon](https://arxiv.org/icons/licenses/by-4.0.png)view license](http://creativecommons.org/licenses/by/4.0/ "Rights to this article")

### Additional Features

* [Audio Summary](/audio/2509.22684)

### Current browse context:

cs.DC

[< prev](/prevnext?id=2509.22684&function=prev&context=cs.DC "previous in cs.DC (accesskey p)")
  |   
[next >](/prevnext?id=2509.22684&function=next&context=cs.DC "next in cs.DC (accesskey n)")

[new](/list/cs.DC/new)
 | 
[recent](/list/cs.DC/recent)
 | [2025-09](/list/cs.DC/2025-09)

Change to browse by:

[cs](/abs/2509.22684?context=cs)  
[cs.AR](/abs/2509.22684?context=cs.AR)  
[cs.CR](/abs/2509.22684?context=cs.CR)  
[cs.PF](/abs/2509.22684?context=cs.PF)

### References & Citations

* [SciX](https://scixplorer.org/abs/arXiv:2509.22684)
* [Google Scholar](https://scholar.google.com/scholar_lookup?arxiv_id=2509.22684)
* [Semantic Scholar](https://api.semanticscholar.org/arXiv:2509.22684)

export BibTeX citation
Loading...

BibTeX formatted citation
-------------------------

×

loading...

Data provided by:

### Bookmark

[![BibSonomy](/static/browse/0.3.4/images/icons/social/bibsonomy.png)](http://www.bibsonomy.org/BibtexHandler?requTask=upload&url=https://arxiv.org/abs/2509.22684&description=ZKProphet:%20Understanding%20Performance%20of%20Zero-Knowledge%20Proofs%20on%20GPUs "Bookmark on BibSonomy")
[![Reddit](/static/browse/0.3.4/images/icons/social/reddit.png)](https://reddit.com/submit?url=https://arxiv.org/abs/2509.22684&title=ZKProphet:%20Understanding%20Performance%20of%20Zero-Knowledge%20Proofs%20on%20GPUs "Bookmark on Reddit")

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

[Which authors of this paper are endorsers?](/auth/show-endorsers/2509.22684) |
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