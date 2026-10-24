Announcing the 2023 ZPrize Winners | ZPrize Blog

![](https://aleo.matomo.cloud/matomo.php?idsite=6&rec=1)

![ZPrize](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/621ceaf456149b151a4fc98a_Group%2048095548.svg)

[documentation](#)

[Github](#)

[Whitepaper](#)

[Get Started](#)

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/627439812fb1c9d02c32bd48_Close.svg)

[About](/#about)

[Prizes](/#prizes)

[Partners](/#partners)

[Blog](/blog)

[FAQ](/faq)

[Learn More](#learn-more)

[Twitter

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/62743fe006989d482acfdac6_Arrow%20Mobile%20Menu.svg)](https://twitter.com/z_prize)[Discord

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/62743fe006989d482acfdac6_Arrow%20Mobile%20Menu.svg)](https://discord.gg/MqvB6EsrxX)[Youtube

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/62743fe006989d482acfdac6_Arrow%20Mobile%20Menu.svg)](https://www.youtube.com/channel/UC5eMCqLVuYKOh8QJXbDqrDg)

© 2022 ZPrize. All Rights Reserved.

[![ZPrize](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/621ceaf456149b151a4fc98a_Group%2048095548.svg)](/)

[about](/#about)

[2023](/#2023)

[Partners](/#partners)

[Get Involved](/#get-involved)

[Blog](/blog)

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/627435ed6806e4b4174f00f4_Menu%20Icon%202.0.svg)

Announcing the 2023 ZPrize Winners
==================================

Alex Pruden

May 7, 2024

![]()

ZPrize was founded to encourage the rapid acceleration of zero-knowledge technology development, providing valuable funding towards projects pushing the boundaries of speed, efficiency, and security of zero-knowledge algorithms implemented across a spectrum of hardware. In 2022, competitors [achieved several breakthroughs](https://www.zprize.io/blog/announcing-zprize-results), with ZPrize submissions leading to an [improvement from baseline of over 5x](https://drive.google.com/file/d/1TEYcLTGPtfPMK1p6O7n82L0aCh874jkF/view) across all submitted prizes in 2022 and improved proof generation times, often by more than 20%.

This year’s ZPrize categories emphasized low-level optimizations, such as accelerating multi-scalar multiplication, as well as more complex applications like high throughput signature verification. This approach ensures that innovations are both tangible and relevant for advancing the efficiency and application of zero-knowledge proofs in blockchain technologies, driving faster, more secure cryptographic processes which can then directly be applied to real-world products like identity verification platform [zPass](https://zpass.aleo.org/).

For the past six months, builders around the globe have worked in these categories while competing for a prize pool of up to **$1.5 million** in US dollars, as well as an additional pool of [Aleo Credits](https://aleo.org/aleo-credits/) associated with the forthcoming Aleo Network.

Before we announce the winners, we’d like to recognize this year’s prize architects: Evan Marshall from [Demox Labs](https://www.zprize.io/blog/zprize-ii-spotlight-introducing-the-architects-evan-marshall-demox-labs), Leo Fan from [Cysic](https://www.zprize.io/blog/zprize-ii-spotlight-introducing-the-architects-leo-fan-cysic), and David Wong from [zkSecurity](https://www.zprize.io/blog/zprize-ii-spotlight-introducing-the-architects-david-wong-zksecurity). These individuals played an essential role of designing and specifying the challenges, coordinating the competition, and judging the results.

We’d also like to thank our sponsors: [Aleo](https://aleo.org/), [Aztec](https://aztec.network/) and [Trapdoor](https://www.trapdoortech.com/) and technology providers [AMD](https://www.amd.com/en.html), [VMAccel](https://www.vmaccel.com/), and [Piknik](https://www.piknik.cloud/).

Finally, we’d like to thank everyone who entered a submission this year and participated in our [Discord community](https://discord.com/invite/MqvB6EsrxX). ZPrize is truly a community-driven effort; we couldn’t have done it without you.

ZPrize 2023 Overview
--------------------

The 2023 ZPrize competition consisted of three prize categories:

[Beat the Best (FPGA/GPU)](https://github.com/cysic-labs/ZPrize-23-Prize1)Challenging builders to develop efficient MSM implementations  
**Prize Architect** - Leo Fan, Cystic  
**Prize Pool** - 500,000 Aleo Credits and $500,000

[Beat the Best (WASM)](https://github.com/demox-labs/webgpu-msm)Optimizing MSM operations via web-based technologies (such as WebGPU)  
**Prize Architect** - Evan Marshall, Demox Labs **Prize Pool** - $650,000, plus 50,000 Aleo Credits

[High throughput signature verification  
‍](https://hackmd.io/JxmufR2OTgqDOoUSuR1dlw?view)Quickly verifying ECDSA signatures on Aleo  
‍**Prize Architect** - David Wong, zkSecurity  
**Prize Pool** - $500,000  
‍  
  
Submissions were evaluated based on innovation, security, and how well they addressed the criteria set forth in each prize specification. Now, let’s take a look at the winners.

2023 Zprize Winners
-------------------

**Prize 1a: Beat the Best (Accelerating MSM Competition)**

This prize is awarded for developing [the most efficient Multi-Scalar Multiplication](https://www.zprize.io/blog/zprize-i-spotlight-open-division-prize-1a-1b) (MSM) implementation on graphics processing units (GPUs) or fully programmable gate arrays (FPGAs). Computing MSM operations is a critical part of both the proof generation and verification process with a zk-SNARK.

Last year, Yrrid Software and Matter Labs tied for first place with a time of 2.52 seconds. This year, we challenged competitors to “beat the best”, building on the successes of last year and achieving an even faster time.

The winners for Prize 1a are:

‍**GPUs**

| Position | Team Name | Score | % Improvement | Repository |
| --- | --- | --- | --- | --- |
| 1 | Marco Zhou of StorSwift | 430 ms | 17.5% | --- |
| Special mention | Yrrid Software Snarkify ZKP | 367 ms | 28.6%\*\* | --- |

‍

*\*\* Faster time received a special mention instead of the prize due to a late submission.*

‍

‍**FPGA**

HTML

| Position | Team Name | Score | Repository |
| --- | --- | --- | --- |
| 1 | Yrrid Software (Niall Emmart and Tony Wu) | --- | --- |
| Special mention | SuperScalar (for their work on the BLS 12-377 G1 curve) | 1465 ms | --- |

‍

As you can see, the winning submissions substantially improved upon last year’s work for the GPU category. For the FPGA category, this year’s challenge was substantially different in that it required optimizing for MSM on *two* separate cryptographic curves. So this work will become the foundation of future hardware implementations and pave the way for ASICs.

Thanks to the work done by these competitors, MSM is starting to become less of a bottleneck for the proof generation process, which will make more use cases for zero-knowledge technology possible.

For this prize category, the competition is actually continuing, with competitors utilizing the results from the MSM acceleration work and applying it to create end-to-end proofs. The results from this prize category will be announced later this summer.

**Prize 2: Beat the Best (E2E Acceleration)**

Prize 2 sought to optimize for consumer-grade devices running MSMs in a browser environment. ZPrize 2022 focused strictly on WASM, but this year, the prize was expanded to  allow competitors to use different web-based technologies including Web Workers, SIMD instructions in WASM, and WebGPU to provide the competitors with more tools to account for a variety of different size MSM problems.

Submissions for this category were judged based on correctness and performance, using Demox Lab’s WebGPU MSM implementation as a baseline. In addition, competitors could choose to submit an implementation for either a BLS 12-377 curve (Prize 2A) or the Twisted Edwards curve variant (Prize 2B). MSM over BLS 12-377 generally improves proof generation times, while the Twisted Edwards improvements affect other UX aspects, such as scanning for unspent transactions on blockchains such as [Aleo](http://aleo.org).

The winners for Prize 2 are:

**2A**

| Position | Team Name | Score | Improvement | Repository |
| --- | --- | --- | --- | --- |
| 1 | Yrrid Software Snarkify\_ZKP | 940 ms | 150x | --- |
| Special mention | Gregor Mitscha-Baude | 2.26 sec | 30x | --- |

‍

**2B**

| Position | Team Name | Score | Improvement | Repository |
| --- | --- | --- | --- | --- |
| 1 | Yrrid Software Snarkify ZKP | 450 ms | ~136x | --- |
| Special mention (best WebGPU solution) | Tal Derei (Penumbra Labs) Koh Wei Jie | 1.45 sec | N/A | --- |

‍

**Prize 3: High Throughput Signature Verification**

The third and final prize was awarded for verifying the greatest number of ECDSA signatures in a given time using [Varuna](https://www.youtube.com/watch?v=ZdHmk2cBlQ0), a new universal proof system based on [Marlin](https://eprint.iacr.org/2019/1047.pdf). ECDSA signatures are used to prove authenticity of encrypted messages over the internet, as well as blockchain transactions on networks such as Bitcoin and Ethereum. Making it more efficient to cryptographically verify those signatures inside of a zero-knowledge proof is important for creating a better user experience and enables numerous real-world applications.

Submissions were judged based on useability and ease of auditing, and resulted in a tie.

The winners for Prize 3 are:

| Position | Team Name | Repository |
| --- | --- | --- |
| 1 | zProof | --- |
| 1 | PolyU Team | --- |

‍

Both winners took different approaches to the challenge. Zproof used 3 different proof systems (Starky, Plonky, and Varuna, which is Aleo’s improvement on the Marlin scheme), which paid off for large message sizes, whereas the PolyU team wrote constraints directly in Leo and optimized the system that way.

The future of ZK is bright
--------------------------

The results of ZPrize 2023 have created an even stronger foundation for the development of efficient, scaleable ZK technology. By developing a new ecosystem of open-source resources, we hope to enable real-world products built on this technology that make the web, and our lives, safer and more secure. We’re excited to see what comes next, and will be adding repos for the winners to the [ZPrize Github](https://github.com/z-prize) and this post in the coming weeks as they become available.

Thank you again to all of the participants and congratulations to the awardees for their work. For updates on ZPrize, follow us on [X](https://twitter.com/z_prize) and reach out to [interest@zprize.io](mailto:interest@zprize.io) to explore sponsorship opportunities.

Type your email here

Thank you! Your submission has been received!

Oops! Something went wrong while submitting the form.

[![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/621d2cba395c98b0cdf0bf95_ZPrize%20Logo%20Black.svg)](/)

Prize winners will be determined in good faith and in the sole discretion of prize sponsors

[About](/#about)[Partners](/#partners)[BLog](/blog)

[Twitter

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/625a053e2e9f795335e87398_Vector%2013.svg)](https://twitter.com/z_prize)[Discord

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/625a053e2e9f795335e87398_Vector%2013.svg)](https://discord.gg/MqvB6EsrxX)[YouTUbe

![](https://cdn.prod.website-files.com/621ce49c2450667f776c49d8/625a053e2e9f795335e87398_Vector%2013.svg)](https://www.youtube.com/channel/UC5eMCqLVuYKOh8QJXbDqrDg)

© 2023 ZPrize. All Rights Reserved.