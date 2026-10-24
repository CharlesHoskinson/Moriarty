Notes and Proofs for Divisor Techniques - ZK/SEC Quarterly


[ZK/SEC](../../index.html)
Research notes from zkSecurity
[Series](../../series.html)
[Reports](https://reports.zksecurity.xyz)

Search
`⌘K`

[All posts](../../index.html)

educative · zk · elliptic-curves

Notes and Proofs for Divisor Techniques
=======================================

Mathias Hall-Andersen
May 08, 2026
3 min

![Elliptic Artwork](./thumbnail.jpg)

On the request of [MAGIC Grants](https://magicgrants.org/)
we were asked to review the security of the divisor-based protocol for proving Elliptic Curve Inner Products (ECIPs)
introduced by Liam Eagen in [his 2022 eprint](https://eprint.iacr.org/2022/596).
The motivation is the upcoming upgrade to Monero, which will employ this technique;
then billions are on the line, you want to be sure.
Our contributions include a new proof of extraction,
more exact definitions of the relations proven,
and a formal treatment of the composition of the protocol with a non-interactive proof.
Our treatment goes all the way from the algebraic geometry to the concrete constraints that must be checked.

[**Download The Notes (PDF)**](./divisor-techniques.pdf)

Application
-----------

The intended application is the upcoming Full-Chain Membership Proofs (FCMP++) for Monero,
which composes the ECIP with [Curve Trees](https://eprint.iacr.org/2022/756)
to get set-membership proofs much more efficient than a standard Merkle tree verified inside a proof system.
This will allow Monero to scale its membership proofs to include every possible transaction.
Curve trees requires:

* A single lookup.
* A single scalar multiplication.

Proven using a circuit over a field that is the base field of the elliptic curve.
The usual way to prove the scalar multiplication involves emulating the group operation in-circuit,
e.g. proving the execution of the standard double-and-add inside the circuit.
Eagen's technique instead exploits non-determinism and interaction:
instead of computing the group operations,
the prover constructs a proof (witness),
commits to it,
and then the verifier samples a random challenge to verify the validity of the proof.
A small teaser follows.

The Trick
---------

We think the original paper is underappreciated; the core observation is very neat.
The key insight is that there is a 1-to-1 correspondence (up to nonzero scalar) between rational functions on an elliptic curve $E$ and *principal divisors*: formal sums of points with integer multiplicities, of degree zero, that sum to the identity on $E$.
Concretely, $P\_1, \ldots, P\_n \in E$ satisfy $\sum\_i P\_i = \mathcal{O}$ (counted with multiplicity) if there exists a rational function $f \in \mathbb{F}(E)$ with divisor:

$$
\mathrm{div}(f) = \sum\_i (P\_i) - n(\mathcal{O})
$$

The useful direction: a witness for the sum is a rational function with the prescribed zeros at the $P\_i$ and a matching pole of order $n$ at infinity.
Hence, rather than emulating the group law inside the circuit, the prover commits to the coefficients of $f$ and the verifier checks that $f$ has exactly that divisor.
Taking the logarithmic derivative reduces this further: the check becomes an inner product between the prover's secret coefficients and a public vector derived from a random challenge point, instead of evaluating $f$ at every $P\_i$.

Done naively, this saves nothing over the simple group law emulation approach,
however, by applying [Haböck's logarithmic derivative trick](https://eprint.iacr.org/2022/1530.pdf) to the expression,
everything becomes a linear combination between public challenge values and the divisor polynomial $f(X, Y) \in \mathbb{F}[E]$.
The end result is that an MSM of arbitrary length can be verified in [*7 multiplicative constraints*](https://github.com/kayabaNerve/fcmp-plus-plus-paper/blob/divisor-paper/divisors.pdf).
All this is covered in much more detail in [our writeup](./divisor-techniques.pdf).

We Can Help
-----------

With broad practical/academic expertise
zkSecurity helps its clients review/develop/employ the most advanced cryptography with confidence.
Including providing additional formalism beyond the original paper,
helping you go from paper to practice without fear of security/proof gaps.
Whether you need extra eyes on your own work, or work you plan to incorporate into your solution,
we are happy to discuss how we may help.

Share
[X](#)
[LinkedIn](#)
[Email](#)
[Markdown source →](../../posts/divisor-notes.md)

**zkSecurity** offers auditing, research and development services for cryptographic systems: zero-knowledge proofs, MPC, FHE, consensus protocols and more.

[Learn more →](https://zksecurity.xyz)

Keep reading

Latest

[### Zero Knowledge Episode 414: zkGolf, AI, Clean and Optimising ZK Circuits

Mathias and Giorgio join Anna Rose and fellow zkSecurity teammates Nico and Kobi on the Zero Knowledge podcast to talk about Clean, zkGolf, and what happens when AI agents compete to build better circuits with proofs of correctness.

ZK/SEC · October 21, 2026](../../posts/zkpod-zkgolf/index.html)
[### Checking the Checkers and Auditing the Ironwood FV

Cryptographic software is error-prone and failures are catastrophic,
therefore formal verification is a powerful and increasingly
practical tool for greatly improving the assurances of our cryptographic software.
But who verifies the formal verification?
Formal verification itself is software too,
what is proved, under which assumptions and its relation to the real-world implementation lie beyond the scope of the machine-checked proof.
In this post, we want to give some insights into our three-week audit of the Ironwood formalization
and some of the common pitfalls that formal verification more broadly can encounter.

Mathias Hall-Andersen · October 05, 2026](../../posts/auditing-formal-verifications/index.html)
[### Archetype x zkSecurity - Proof is in the Pudding: zkML

In Session 11 of "Proof is in the Pudding," we look at what it takes to prove model inference. We cover transformer computation, sumcheck, GKR, lookup arguments, quantization, and KV caching, then discuss scaling and possible uses for verifiable ML.

ZK/SEC · September 30, 2026](../../posts/pudding-11-zkml/index.html)

Recommended

[### ZNARKs: SNARKs for The Integers

Hey there! Interested in learning about SNARKs that work beyond finite fields? We’ve been diving into $\mathbb{Z}$NARKs, which are SNARKs tailored for computations involving integers. Our latest post unpacks this intriguing area, showing how we can construct efficient proof systems for integer-based computations. You'll discover nifty tricks like range checks without bit decomposition and mixed field emulation, plus how these techniques can simplify RSA computations. Intrigued by the idea of using randomness for more reliable proofs or exploring an intellectual curiosity like $\mathbb{Q}$-circuits? This post covers it all, including a peek into the future of polynomial commitments. Dive in and explore with us!

Mathias Hall-Andersen · November 11, 2024](../../posts/znarks/index.html)
[### WE-KZG: Encrypt to KZG.

Ever wondered if you could create a ciphertext that's only decrypted when a polynomial inside a commitment has a particular value? We’ve explored this notion using KZG commitments in our latest Asiacrypt 2024 paper. Dive into the elegant world of Witness Encryption and see how it can be applied in cool ways like Laconic Oblivious Transfer. This approach keeps things as efficient as regular KZG operations and might just spark some creative applications of your own! Curious to learn more? Let’s explore together!

Mathias Hall-Andersen · October 08, 2024](../../posts/kzg-we/index.html)
[### Groth16, Intuitively

Groth16 is still the gold standard for succinct SNARKs: 128-byte proofs, constant-size verification, and a decade of real-world deployment. But despite its ubiquity, almost nobody explains \*why\* it works the way it does. In this post, we build Groth16 from the ground up, starting from R1CS and QAPs, then layer in pairings, trusted setup parameters, and the separator tricks (α, β, γ, δ) that make the scheme sound. By the end, you should have an intuitive grasp of every term in the final verifier equation.

David Wong · May 01, 2026](../../posts/groth16/index.html)

More to explore

[### Auditing Self: Collaborating with Celo on Privacy and Identity Infrastructure

We recently teamed up with Celo for an in-depth security audit of the Self project, exploring its innovative approach to on-chain identity using biometric passports and zero-knowledge proofs. During our three-week dive, we examined everything from cryptographic primitives to smart contract architecture and a unique proof delegation system using AWS Nitro Enclaves. The Celo team impressed us with their commitment and responsiveness, and we collaborated on refining the system with strategic improvements. Curious about the nitty-gritty details and our insights? Check out the full report!

ZK/SEC · March 24, 2025](../../posts/self-audit/index.html)
[### Archetype x zkSecurity - Proof in the Pudding: Introduction to Data Availability (Sampling)

In the latest "Proof is in the Pudding" session, we team up with Archetype to break down the essentials of Data Availability Sampling. We dive into how rollups and Ethereum's DA system work, explore the role of DA chains, and touch on the basics of verifiable sharding. This introduction is perfect for anyone curious about the foundations of data availability sampling and how these concepts are playing out in the blockchain world.

ZK/SEC · October 02, 2025](../../posts/pudding-6-DA/index.html)
[### Variants of KZG: Part III, Multilinear Commitments with Zeromorph

In this blog post, we extend univariate KZG commitments to multilinear polynomials through Zeromorph. We introduce the univariatization map, encode the multilinear quotient identity as a univariate identity, and explain why the quotient encodings require degree checks. We then show how Zeromorph batches these checks into a single degree-bounded KZG opening and walk through its end-to-end opening protocol. We conclude by examining its proof size, prover cost, and verifier cost.

Varun Thakore · August 04, 2026](../../posts/kzg-3/index.html)

Research publication
ZK/SEC Quarterly

Volume
2026

Author
Mathias Hall-Andersen

Links

[Home](https://zksecurity.xyz)
[Reports](https://reports.zksecurity.xyz)
[Series](../../series.html)

ESC