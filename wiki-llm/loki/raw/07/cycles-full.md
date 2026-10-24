On cycles of pairing-friendly elliptic curves



##### Report GitHub Issue

×

Title:


Content selection saved. Describe the issue below:

Description:

Submit without GitHub
Submit in GitHub

![](/static/base/1.0.1/images/icons/smileybones-small.svg)arXiv is now an independent nonprofit!
[Learn more](https://info.arxiv.org/about)
×

[![arXiv logo](/static/base/1.0.1/images/arxiv-logo-primary-light.svg)Back to arXiv](/)

[Why HTML?](https://info.arxiv.org/about/accessible_HTML.html)
[Report Issue](# "Report an Issue")
[Back to Abstract](/abs/1803.02067v3 "Back to abstract page")
[Download PDF](/pdf/1803.02067v3 "Download PDF")

1. [Abstract](#abstract1 "In On cycles of pairing-friendly elliptic curves")
2. [1 Introduction](#S1 "In On cycles of pairing-friendly elliptic curves")
   1. [1.1 Overview of results](#S1.SS1 "In 1 Introduction ‣ On cycles of pairing-friendly elliptic curves")
   2. [1.2 Open problems](#S1.SS2 "In 1 Introduction ‣ On cycles of pairing-friendly elliptic curves")
3. [2 Preliminaries](#S2 "In On cycles of pairing-friendly elliptic curves")
   1. [2.1 Elliptic curves and pairings](#S2.SS1 "In 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")
   2. [2.2 Families of pairing-friendly elliptic curves](#S2.SS2 "In 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")
4. [3 Cycles of pairing-friendly elliptic curves](#S3 "In On cycles of pairing-friendly elliptic curves")
5. [4 MNT cycles](#S4 "In On cycles of pairing-friendly elliptic curves")
6. [5 Two-cycles of specific embedding degrees](#S5 "In On cycles of pairing-friendly elliptic curves")
7. [6 Cycles with the same discriminant](#S6 "In On cycles of pairing-friendly elliptic curves")
8. [7 Cycles with cofactors](#S7 "In On cycles of pairing-friendly elliptic curves")
9. [8 Other cycles on parametrized families](#S8 "In On cycles of pairing-friendly elliptic curves")
10. [A Dual Elliptic Primes](#A1 "In On cycles of pairing-friendly elliptic curves")
11. [References](#bib "In On cycles of pairing-friendly elliptic curves")

[License: arXiv.org perpetual non-exclusive license](https://info.arxiv.org/help/license/index.html#licenses-available)

arXiv:1803.02067v3 [math.NT] 02 Nov 2018

On cycles of pairing-friendly elliptic curves
=============================================

Alessandro Chiesa

alexch@berkeley.edu

UC Berkeley



Lynn Chua

chualynn@berkeley.edu

UC Berkeley



Matthew Weidner

malw2@cam.ac.uk

Cambridge

###### Abstract

A cycle of elliptic curves is a list of elliptic curves over finite
fields such that the number of points on one curve is equal to the
size of the field of definition of the next, in a cyclic way. We study
cycles of elliptic curves in which every curve is
pairing-friendly. These have recently found notable applications in
pairing-based cryptography, for instance in improving the scalability
of distributed ledger technologies.

We construct a new cycle of length 4 consisting of MNT curves, and characterize all the possibilities for cycles consisting of MNT curves. We rule out cycles of length 2 for particular choices of small embedding degrees. We show that long cycles cannot be constructed from families of curves with the same complex multiplication discriminant, and that cycles of composite order elliptic curves cannot exist. We show that there are no cycles consisting of curves from only the Freeman or Barreto–Naehrig families.

Keywords: pairing-friendly elliptic curves; cycles of elliptic curves; cryptography

###### Contents

1. [1 Introduction](#S1 "In On cycles of pairing-friendly elliptic curves")
   1. [1.1 Overview of results](#S1.SS1 "In 1 Introduction ‣ On cycles of pairing-friendly elliptic curves")
   2. [1.2 Open problems](#S1.SS2 "In 1 Introduction ‣ On cycles of pairing-friendly elliptic curves")
2. [2 Preliminaries](#S2 "In On cycles of pairing-friendly elliptic curves")
   1. [2.1 Elliptic curves and pairings](#S2.SS1 "In 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")
   2. [2.2 Families of pairing-friendly elliptic curves](#S2.SS2 "In 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")
3. [3 Cycles of pairing-friendly elliptic curves](#S3 "In On cycles of pairing-friendly elliptic curves")
4. [4 MNT cycles](#S4 "In On cycles of pairing-friendly elliptic curves")
5. [5 Two-cycles of specific embedding degrees](#S5 "In On cycles of pairing-friendly elliptic curves")
6. [6 Cycles with the same discriminant](#S6 "In On cycles of pairing-friendly elliptic curves")
7. [7 Cycles with cofactors](#S7 "In On cycles of pairing-friendly elliptic curves")
8. [8 Other cycles on parametrized families](#S8 "In On cycles of pairing-friendly elliptic curves")
9. [A Dual Elliptic Primes](#A1 "In On cycles of pairing-friendly elliptic curves")
10. [References](#bib "In On cycles of pairing-friendly elliptic curves")

1 Introduction
--------------

A cycle of elliptic curves is a list of elliptic curves defined over finite fields in which the number of points on one curve equals the size of the field of definition of the next, cyclically.

###### Definition 1.

An *mm-cycle of elliptic curves* is a list of mm distinct elliptic curves E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}},…\ldots, Em/𝔽qmE\_{m}/\mathbb{F}\_{q\_{m}}, where q1,…,qmq\_{1},\ldots,q\_{m} are prime, such that the numbers of points on these curves satisfy

|  |  |  |  |
| --- | --- | --- | --- |
|  | #​E1​(𝔽q1)=q2,…,#​Ei​(𝔽qi)=qi+1,…,#​Em​(𝔽qm)=q1.\displaystyle\#E\_{1}(\mathbb{F}\_{q\_{1}})=q\_{2}\,,\ldots,\#E\_{i}(\mathbb{F}\_{q\_{i}})=q\_{i+1}\,,\ldots,\#E\_{m}(\mathbb{F}\_{q\_{m}})=q\_{1}\,. |  | (1) |

This notion was introduced in [[SS11](#bib.bibx34)] with the name of *aliquot cycles*. The case of 2-cycles of ordinary curves, also called amicable pairs, was introduced in the context of primality proving by [[Mih97](#bib.bibx24), [Mih07](#bib.bibx25)] under the equivalent notion of *dual elliptic primes* (see [Appendix A](#A1 "Appendix A Dual Elliptic Primes ‣ On cycles of pairing-friendly elliptic curves")).

Silverman and Stange [[SS11](#bib.bibx34)] showed that cycles of arbitrary lengths exist, and gave conjectural estimates, for any elliptic curve E/ℚE/\mathbb{Q}, of the number of prime pairs (q1,q2)(q\_{1},q\_{2}) such that reducing EE modulo q1q\_{1} and q2q\_{2} gives an amicable pair. Cycles of elliptic curves were further studied in [[BBC+12](#bib.bibx1), [Jon13](#bib.bibx20), [Par15](#bib.bibx28), [Par17](#bib.bibx29)], and some of these works refined and proved on average the conjectured estimates, showing that amicable pairs are asymptotically common.

In [[BCTV14](#bib.bibx4)] the notion of cycles of elliptic curves was extended for applications to pairing-based cryptography.

###### Definition 2.

A *pairing-friendly mm-cycle of elliptic curves* is an mm-cycle
such that every elliptic curve in the cycle is ordinary and has a small embedding degree.

Pairing-friendly cycles were used in [[BCTV14](#bib.bibx4)] to achieve recursive composition of *zkSNARKs* (also known as *proof carrying data*). A zkSNARK is a cryptographic scheme that allows one party (the prover) to convince another party (the verifier) that the prover knows a certain secret, via a short proof that is cheap to verify and reveals no information about the secret. Efficient zkSNARK constructions are obtained via pairing-friendly elliptic curves, and the cycle condition in [Eq. 1](#S1.E1 "In Definition 1. ‣ 1 Introduction ‣ On cycles of pairing-friendly elliptic curves") enables their recursive composition, while avoiding expensive modular arithmetic across fields of different characteristics. (See [[BCTV14](#bib.bibx4)] for details.)

Practitioners are interested in recursive composition of zkSNARKs, because it can be used to boost the scalability of distributed ledger technologies [[Bre17](#bib.bibx10)]. For example, there are commercial efforts in this space whose core technology *is* recursive composition [[Cod18](#bib.bibx12)], and such technology thus rests on properties of cycles of pairing-friendly elliptic curves.

This motivates the question: *what types of pairing-friendly cycles exist?*

A pairing-friendly 22-cycle can be obtained from pairing-friendly prime-order curves of embedding degrees 44 and 66[[KT08](#bib.bibx22), [BCTV14](#bib.bibx4)]. Beyond this, there are *no* other known constructions, and very little is known about pairing-friendly cycles. Indeed, requiring a small embedding degree as in [Definition 2](#Thmdefinition2 "Definition 2. ‣ 1 Introduction ‣ On cycles of pairing-friendly elliptic curves") is a strong restriction and techniques used in previous work to study aliquot cycles do not seem to apply to pairing-friendly cycles.

This is unfortunate because the aforementioned MNT cycle is not ideal for applications: its unequal embedding degrees make one curve less secure than the other and, moreover, the fact that both embedding degrees are so small implies that using the cycle at high security levels is inefficient. It would be desirable, e.g., to have a 22-cycle with embedding degrees (12,12)(12,12) or (20,20)(20,20) and, more generally, to understand this mathematical object better.

### 1.1 Overview of results

The stark difference in the current understanding of pairing-friendly cycles when compared to aliquot cycles, as well as applications to pairing-friendly cryptography in the real world, motivates a systematic study of pairing-friendly cycles. In this paper we initiate such a study, and our main results are the following.

1. 1.

   Prior to this work, the *only* construction of pairing-friendly cycles was a 2-cycle from a family of curves called *MNT curves*, named after Miyaji, Nakabayashi, and Takano [[MNT01](#bib.bibx26)]. A natural question to ask is: can one construct other cycles consisting of MNT curves? In this work, we construct a new pairing-friendly cycle of length 4 using MNT curves. We also characterize *all* the possibilities for cycles consisting of MNT curves, showing that any MNT cycle must have length 2 or 4, and that the curves must have embedding degrees alternating between 4 and 6. See [Section 4](#S4 "4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") for details.
2. 2.

   We then study *arbitrary* pairing-friendly 22-cycles (not derived from a particular family). We prove that 22-cycles of elliptic curves with embedding degrees (5,10)(5,10), (8,8)(8,8), or (12,12)(12,12) do *not* exist. The technique that we use relies on the fact that the cyclotomic polynomials of these embedding degrees have degree 44. In particular, we do not know how to extend this result to any other embedding degrees (k1,k2)(k\_{1},k\_{2}). See [Section 5](#S5 "5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves") for details.
3. 3.

   We move to study pairing-friendly cycles of arbitrary length. One strategy to construct cycles could be to pick a parametrized family of elliptic curves and try to construct cycles consisting of curves from the same family (like for MNT curves). What must the parameters of the family satisfy for such constructions to be possible? We prove that if the curves have the same discriminant for complex multiplication D>3D>3, then we cannot construct cycles of length greater than 2 ([Section 6](#S6 "6 Cycles with the same discriminant ‣ On cycles of pairing-friendly elliptic curves")). This implies that to construct elliptic curve cycles, we must use curves from families of varying discriminants.
4. 4.

   So far we discussed cycles consisting of elliptic curves of *prime* order. What if we relax the definition of cycles to allow *composite* (non-prime) order elliptic curves in which the number of points on one curve is a multiple of (but not necessarily equal to) the size of the field of definition of the next? We prove that composite-order cycles *cannot exist* (see [Section 7](#S7 "7 Cycles with cofactors ‣ On cycles of pairing-friendly elliptic curves")). This is a strong restriction as it implies that we must construct cycles using pairing-friendly elliptic curves of *prime* order. Unfortunately, there are very few constructions of families of such curves in the literature, *regardless of cycles*.
5. 5.

   Lastly, we study the other known families of pairing-friendly elliptic curves of prime order (apart from MNT curves): the Freeman curves and the Barreto–Naehrig curves. We prove that cycles within each of these families do not exist ([Section 8](#S8 "8 Other cycles on parametrized families ‣ On cycles of pairing-friendly elliptic curves")). This means that, if one wants to obtain cycles from curve families, one must consider combinations of current families (or study future constructions of prime-order elliptic curves).

Overall, cycles of pairing-friendly elliptic curves seem much harder to understand, and to construct, than cycles of arbitrary elliptic curves. While our results have for the most part established limitations of pairing-friendly cycles, our outlook is optimistic. Our work demonstrates that studying pairing-friendly cycles is tractable and, moreover, points the way to concrete research questions that could lead to more tools for studying these cycles. We thus conclude the introduction with a selection of open problems.

### 1.2 Open problems

1. 1.

   Do there exist cycles consisting of elliptic curves with the *same* embedding degree? The varying embedding degrees in current constructions of cycles is inconvenient because, in practice, curves in the cycle have different security levels.
2. 2.

   Can we construct cycles of embedding degrees greater than 6? All known pairing-friendly cycles involve embedding degrees at most 66, which means that it is inefficient to use such cycles at high security levels (e.g., 128 bits of security). It would be desirable to construct, or rule out, cycles of higher embedding degrees (say, 20).
3. 3.

   In particular, can we construct 2-cycles of higher embedding degrees? Our technique for ruling out pairs with embedding degrees (5,10)(5,10), (8,8)(8,8), or (12,12)(12,12) sheds some light on other pairs (k1,k2)(k\_{1},k\_{2}) for which Φk1​(x)=Φk2​(−x)\Phi\_{k\_{1}}(x)=\Phi\_{k\_{2}}(-x), but it does not seem to extend to the case deg⁡Φk1​(x)>4\deg\Phi\_{k\_{1}}(x)>4. We believe that it would be especially interesting to study pairs with embedding degrees (16,16)(16,16), which have cyclotomic polynomial x8+1x^{8}+1.
4. 4.

   Do there exist cycles consisting of elliptic curves with the same discriminant and the same embedding degrees? Our work demonstrates that sharing the same discriminant is already quite limiting, and it would be interesting to understand how this requirement interacts with that of sharing the same embedding degree.
5. 5.

   Are there cycles from combinations of MNT, Freeman, and Barreto–Naehrig curves? Our preliminary investigations via Gröbner bases suggest small cycles are unlikely, but the question remains open for arbitrary-length cycles.

2 Preliminaries
---------------

### 2.1 Elliptic curves and pairings

Let EE be an elliptic curve over a finite field 𝔽q\mathbb{F}\_{q}, where
qq is a prime. We denote this by E/𝔽qE/\mathbb{F}\_{q}, and we denote by
E⁡(𝔽q)E(\mathbb{F}\_{q}) the group of points of EE over 𝔽q\mathbb{F}\_{q}, with
order n=#​E​(𝔽q)n=\#E(\mathbb{F}\_{q}). The *trace* of E/𝔽qE/\mathbb{F}\_{q} is
t=q+1−nt=q+1-n. By Hasse’s theorem [[Sil09](#bib.bibx32), Theorem V.1.1], tt
satisfies |t|≤2​q|t|\leq 2\sqrt{q}. We say that EE
is *supersingular* if t≡0(modq)t\equiv 0\pmod{q}, otherwise EE is *ordinary*.

The *endomorphism ring* End​(E)\mbox{End}(E) of EE consists of morphisms from EE to itself that are also group homomorphisms on its points. If EE is supersingular, then End​(E)\mbox{End}(E) is an order in a quaternion algebra. If EE is ordinary, then End​(E)\mbox{End}(E) is an order in an imaginary quadratic field ℚ⁡(−D)\mathbb{Q}(\sqrt{-D}), for some positive squarefree integer DD. We call DD the *discriminant*, and we say that EE has *complex multiplication* in ℚ⁡(−D)\mathbb{Q}(\sqrt{-D}).111
Some works use the convention that DD is negative. Throughout this work we take DD to be positive.

Let r≥2r\geq 2 be an integer relatively prime to qq. We denote the rr-torsion points of EE by E⁡[r]E[r], and we denote the group of rr-th roots of unity in the algebraic closure of 𝔽q\mathbb{F}\_{q} by μr\mu\_{r}. The *Weil pairing* is a bilinear non-degenerate map

|  |  |  |  |
| --- | --- | --- | --- |
|  | er:E⁡[r]×E⁡[r]→μr.e\_{r}\colon E[r]\times E[r]\rightarrow\mu\_{r}\,. |  | (2) |

The *embedding degree* with respect to rr is the smallest integer kk such that rr divides qk−1q^{k}-1. In the case of prime-order curves, if r=nr=n we simply say that EE has embedding degree kk.

The Weil pairing was first used in cryptography to reduce the discrete logarithm problem on E⁡[r]E[r] to a discrete logarithm problem in μr\mu\_{r}, which is contained in 𝔽qk∗\mathbb{F}\_{q^{k}}^{\*}[[MOV93](#bib.bibx27), [FR94](#bib.bibx17)]. Subsequently, starting with the work of [[BF03](#bib.bibx6), [Jou04](#bib.bibx21)], the Weil pairing was used to achieve numerous cryptographic capabilities. For security, it is necessary to choose the embedding degree kk such that the discrete logarithm problem in 𝔽qk∗\mathbb{F}\_{q^{k}}^{\*} is computationally infeasible. On the other hand, the embedding degree cannot be too large, or the computation of the Weil pairing (which grows linearly in kk) would not be efficient enough for cryptographic applications.

We say that an elliptic curve E/𝔽qE/\mathbb{F}\_{q} is *pairing-friendly* if E⁡(𝔽q)E(\mathbb{F}\_{q}) has a large prime-order subgroup, and if the embedding degree is small (see [[FST10](#bib.bibx19)] for a more precise definition). A random elliptic curve has a large embedding degree and thus is not pairing-friendly. Constructing pairing-friendly curves with specified parameters is a difficult problem with strong practical motivations that has been extensively studied. It was shown in [[MOV93](#bib.bibx27)] that supersingular elliptic curves can have embedding degree at most 66, and if the characteristic of qq is not 22 or 33, the embedding degree is at most 33. As we are interested in large values of qq and higher values of kk for applications in cryptography, we focus on ordinary elliptic curves.

The known methods to construct ordinary pairing-friendly curves proceed by first finding parameters q,r,t,kq,r,t,k such that there exists an elliptic curve E/𝔽qE/\mathbb{F}\_{q} with trace tt, a prime-order subgroup of size rr, and embedding degree kk. The *complex multiplication method* is then used to find the equation of the curve. This works if the *CM equation* 4​q−t2=D​y24q-t^{2}=Dy^{2} has a solution with y∈ℤy\in\mathbb{Z} and small positive discriminant D∈ℤD\in\mathbb{Z}. Indeed, state-of-the-art algorithms run in time O⁡(D​polylog​D)O(D\,\mbox{polylog}\,D) and are only feasible for DD of size up to 101610^{16}[[Sut12](#bib.bibx35)].

It is useful to view the condition on the embedding degree via cyclotomic polynomials. Let Φm\Phi\_{m} be the mm-th cyclotomic polynomial (the minimal polynomial over the rationals of an irreducible mm-th root of unity). It is known that (see for example [[Was97](#bib.bibx37)])

|  |  |  |  |
| --- | --- | --- | --- |
|  | xm−1=∏d|mΦd​(x).x^{m}-1=\prod\_{d|m}\Phi\_{d}(x)\,. |  | (3) |

###### Lemma 1.

Let E/𝔽qE/\mathbb{F}\_{q} have prime order nn. Then EE has embedding degree kk if and only if kk is minimal such that nn divides Φk​(q)\Phi\_{k}(q).

###### Proof.

The condition that kk is the embedding degree implies that kk is minimal such that qk≡1(modn)q^{k}\equiv 1\pmod{n}. Using basic results on cyclotomic polynomials (see [[Was97](#bib.bibx37), Lemma 2.9]), this is equivalent to the condition that n|Φk​(q)n|\Phi\_{k}(q).
∎

This can be converted into a result relating nn to the trace tt.

###### Lemma 2 ([[BLS02](#bib.bibx8)]).

E/𝔽qE/\mathbb{F}\_{q} has embedding degree kk if and only if n|Φk​(t−1)n|\Phi\_{k}(t-1) and n∤Φi​(t−1)n\nmid\Phi\_{i}(t-1) for all 0<i<k0<i<k.

### 2.2 Families of pairing-friendly elliptic curves

We consider families of pairing-friendly elliptic curves with a fixed embedding degree, whose parameters are defined by polynomials. These are useful for generating curves for applications, where curves of arbitrary size are desired. Each family is parametrized by polynomials (qk​(x),nk​(x),tk​(x))(q\_{k}(x),n\_{k}(x),t\_{k}(x)), representing the field of definition, number of rational points, and trace respectively, where kk is the embedding degree. These have to satisfy that nk​(x)=qk​(x)+1−tk​(x)n\_{k}(x)=q\_{k}(x)+1-t\_{k}(x), nk​(x)n\_{k}(x) divides Φk​(tk​(x)−1)\Phi\_{k}(t\_{k}(x)-1), and there must be infinitely many integer solutions (x,y)(x,y) to the CM equation 4​qk​(x)−tk​(x)2=D​y24q\_{k}(x)-t\_{k}(x)^{2}=Dy^{2}, for some small positive discriminant D∈ℤD\in\mathbb{Z}.

Miyaji, Nakabayashi, and Takano [[MNT01](#bib.bibx26)] characterized all families of ordinary prime-order elliptic curves with embedding degrees k=3,4,6k=3,4,6. For these embedding degrees, the cyclotomic polynomial is quadratic, and the CM equation can be transformed into a generalized Pell equation. These families are parametrized by the polynomials in [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"). We refer to elliptic curves belonging to the MNT families in [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves") as *MNT curves*.

Table 1: MNT curves.

| kk | qk​(x)q\_{k}(x) | nk​(x)n\_{k}(x) | tk​(x)t\_{k}(x) |
| --- | --- | --- | --- |
| 33 | 12​x2−112x^{2}-1 | 12​x2−6​x+112x^{2}-6x+1 | 6​x−16x-1 |
| 44 | x2+x+1x^{2}+x+1 | x2+2​x+2,x2+1x^{2}+2x+2,\,x^{2}+1 | −x,x+1-x,\,x+1 |
| 66 | 4​x2+14x^{2}+1 | 4​x2+2​x+14x^{2}+2x+1 | −2​x+1-2x+1 |

For other embedding degrees, there is no analogous characterization of all elliptic curves with a given embedding degree. Moreover, there is currently no method to construct families of prime-order elliptic curves of arbitrary embedding degrees. (If we allow for composite orders, there are algorithms to construct elliptic curves of arbitrary embedding degrees [[CP01](#bib.bibx13), [DEM05](#bib.bibx14)].) There are two other constructions of prime-order families, stated below.

Freeman [[Fre06](#bib.bibx18)] has constructed a family of prime-order elliptic curves with k=10k=10, which is parametrized by the following polynomials:

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | | | | |
|  | q10​(x)\displaystyle q\_{10}(x) | =25​x4+25​x3+25​x2+10​x+3,\displaystyle=25x^{4}+25x^{3}+25x^{2}+10x+3\,, |  | (4a) |
|  | n10​(x)\displaystyle n\_{10}(x) | =25​x4+25​x3+15​x2+5​x+1,\displaystyle=25x^{4}+25x^{3}+15x^{2}+5x+1\,, |  | (4b) |
|  | t10​(x)\displaystyle t\_{10}(x) | =10​x2+5​x+3.\displaystyle=10x^{2}+5x+3\,. |  | (4c) |

Barreto and Naehrig [[BN05](#bib.bibx9)] have another construction with k=12k=12, parametrized by

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | | | | |
|  | q12​(x)\displaystyle q\_{12}(x) | =36​x4+36​x3+24​x2+6​x+1,\displaystyle=36x^{4}+36x^{3}+24x^{2}+6x+1\,, |  | (5a) |
|  | n12​(x)\displaystyle n\_{12}(x) | =36​x4+36​x3+18​x2+6​x+1,\displaystyle=36x^{4}+36x^{3}+18x^{2}+6x+1\,, |  | (5b) |
|  | t12​(x)\displaystyle t\_{12}(x) | =6​x2+1.\displaystyle=6x^{2}+1\,. |  | (5c) |

Other constructions of pairing-friendly elliptic curves have *composite* orders. These include the families of Brezing and Weng [[BW05](#bib.bibx11)] and of Barreto, Lynn, and Scott [[BLS02](#bib.bibx8)].

3 Cycles of pairing-friendly elliptic curves
--------------------------------------------

In this paper we study cycles of pairing-friendly elliptic curves. This notion was introduced in [[BCTV14](#bib.bibx4)] for applications in cryptography. We re-state [Definition 1](#Thmdefinition1 "Definition 1. ‣ 1 Introduction ‣ On cycles of pairing-friendly elliptic curves") below.

###### Definition 3.

An *mm-cycle of elliptic curves* is a list of mm distinct elliptic curves E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}},…\ldots, Em/𝔽qmE\_{m}/\mathbb{F}\_{q\_{m}}, where q1,…,qmq\_{1},\ldots,q\_{m} are prime, such that the numbers of points on these curves satisfy

|  |  |  |  |
| --- | --- | --- | --- |
|  | #​E1​(𝔽q1)=q2,…,#​Ei​(𝔽qi)=qi+1,…,#​Em​(𝔽qm)=q1.\displaystyle\#E\_{1}(\mathbb{F}\_{q\_{1}})=q\_{2}\,,\ldots,\#E\_{i}(\mathbb{F}\_{q\_{i}})=q\_{i+1}\,,\ldots,\#E\_{m}(\mathbb{F}\_{q\_{m}})=q\_{1}\,. |  | (6) |

Cryptographic applications require curves in the cycle to have small embedding degree.

###### Definition 4.

A *(k1,…,km)(k\_{1},\dots,k\_{m})-cycle* is an mm-cycle of distinct ordinary
elliptic curves E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}}, …\ldots, Em/𝔽qmE\_{m}/\mathbb{F}\_{q\_{m}} such that Ei/𝔽qiE\_{i}/\mathbb{F}\_{q\_{i}} has embedding degree kik\_{i}, for each i=1,…,mi=1,\ldots,m. A (k1,…,km)(k\_{1},\ldots,k\_{m})-cycle is *pairing-friendly* if all the kik\_{i}’s are small (recall [Definition 2](#Thmdefinition2 "Definition 2. ‣ 1 Introduction ‣ On cycles of pairing-friendly elliptic curves")).

An mm-cycle is a special case of a (k1,…,km)(k\_{1},\ldots,k\_{m})-cycle where the kik\_{i}’s are arbitrary positive integers (or possibly infinity).

If we require that q1,…,qmq\_{1},\ldots,q\_{m} are *distinct* primes, [Definition 3](#Thmdefinition3 "Definition 3. ‣ 3 Cycles of pairing-friendly elliptic curves ‣ On cycles of pairing-friendly elliptic curves") is equivalent to the notion of *aliquot cycles* for elliptic curves E/ℚE/\mathbb{Q} by Silverman and Stange [[SS11](#bib.bibx34)]. An aliquot mm-cycle for E/ℚE/\mathbb{Q} is a sequence of distinct primes (q1,…,qm)(q\_{1},\ldots,q\_{m}) such that EE has good reduction at each prime and, if we denote the reduction of EE at qiq\_{i} by E~qi\tilde{E}\_{q\_{i}}, then

|  |  |  |  |
| --- | --- | --- | --- |
|  | #​E~q1​(𝔽q1)=q2,…,#​E~qi​(𝔽qi)=qi+1,…,#​E~qm​(𝔽qm)=q1.\displaystyle\#\tilde{E}\_{q\_{1}}(\mathbb{F}\_{q\_{1}})=q\_{2}\,,\ldots,\#\tilde{E}\_{q\_{i}}(\mathbb{F}\_{q\_{i}})=q\_{i+1}\,,\ldots,\#\tilde{E}\_{q\_{m}}(\mathbb{F}\_{q\_{m}})=q\_{1}\,. |  | (7) |

Given an aliquot mm-cycle, we can construct an mm-cycle of elliptic curves by setting Ei:=E~qiE\_{i}:=\tilde{E}\_{q\_{i}} for each ii. Conversely, given an mm-cycle where q1,…,qmq\_{1},\ldots,q\_{m} are distinct, we can construct a curve E/ℚE/\mathbb{Q} by computing its coefficients via the Chinese Remainder Theorem in such a way that EE’s reduction at each qiq\_{i} is EiE\_{i}.

It is known that cycles of arbitrary lengths exist, based just on the Hasse bound and the fact that every trace in the Hasse bound is realized by an elliptic curve [[Deu41](#bib.bibx15)].

###### Proposition 1 ([[SS11](#bib.bibx34), Theorem 5.1]).

For every m≥1m\geq 1 there exists an elliptic curve E/ℚE/\mathbb{Q} with an aliquot mm-cycle.

However, the foregoing result does not take into account the embedding degrees of the curves. In particular, it is not known if pairing-friendly cycles of arbitrary lengths exist.

The focus of this paper is the study of *pairing-friendly* cycles of elliptic curves. This is a significantly more restrictive notion than the aliquot cycles introduced in [[SS11](#bib.bibx34)], since a random elliptic curve would not have a small embedding degree. Moreover, there are only few known families of prime-order elliptic curves with small embedding degrees (see [Section 2.2](#S2.SS2 "2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves") for a list of all such families). Even without the condition that the curves form a cycle, it is already a difficult problem to construct pairing-friendly elliptic curves of prime order.

We list below a few observations that we will use in this paper. First, the lemma below implies that to construct cycles of elliptic curves for applications (where the size of the finite fields tend to be large), we need only consider *ordinary* elliptic curves.

###### Lemma 3.

Let E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}},…\ldots, Em/𝔽qmE\_{m}/\mathbb{F}\_{q\_{m}} be an mm-cycle of elliptic curves, where q1,…,qm≥5q\_{1},\ldots,q\_{m}\geq 5 are prime. Then all the curves must be ordinary elliptic curves.

###### Proof.

It is known that for any elliptic curve E/𝔽qE/\mathbb{F}\_{q} with q≥5q\geq 5 prime, EE is supersingular if and only if
#​E​(𝔽q)=q+1\#E(\mathbb{F}\_{q})=q+1, see for example [[Sil09](#bib.bibx32), Exercise 5.10]. Suppose Ei/𝔽qiE\_{i}/\mathbb{F}\_{q\_{i}} is supersingular for some ii, then #​E​(𝔽qi)=qi+1=qi+1\#E(\mathbb{F}\_{q\_{i}})=q\_{i}+1=q\_{i+1}. But since qiq\_{i} is prime, qi+1q\_{i}+1 is even, hence this cannot hold.
∎

Next, we present a necessary condition for mm elliptic curves to form an mm-cycle. This condition is *not* sufficient as every trace in the Hasse interval can be realized by an elliptic curve [[Deu41](#bib.bibx15)], hence this condition is not a strong restriction on the curves in the cycle.

###### Lemma 4.

Let E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}},…\ldots, Em/𝔽qmE\_{m}/\mathbb{F}\_{q\_{m}} be an mm-cycle of elliptic curves, with traces t1,…,tmt\_{1},\ldots,t\_{m} respectively. Then the sum of their traces satisfies

|  |  |  |  |
| --- | --- | --- | --- |
|  | t1+⋯+tm=m.\displaystyle t\_{1}+\cdots+t\_{m}=m\,. |  | (8) |

###### Proof.

Let ni=#​Ei​(𝔽qi)n\_{i}=\#E\_{i}(\mathbb{F}\_{q\_{i}}), for each i=1,…,mi=1,\ldots,m. Since the curves form a cycle, we have the constraints n1=q2n\_{1}=q\_{2}, …\ldots, ni=qi+1n\_{i}=q\_{i+1}, …\ldots, nm=q1n\_{m}=q\_{1}. If we sum up these mm equations, we get n1+⋯+nm=q1+⋯+qmn\_{1}+\cdots+n\_{m}=q\_{1}+\cdots+q\_{m}. Using the fact that ni=qi+1−tin\_{i}=q\_{i}+1-t\_{i}, we get t1+⋯+tm=mt\_{1}+\cdots+t\_{m}=m.
∎

4 MNT cycles
------------

We consider pairing-friendly cycles consisting of MNT curves (see [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")), which are the ordinary prime-order elliptic curves of embedding degrees 3,4,63,4,6. For brevity, we use the term *MNT cycles* for cycles where every curve is an MNT curve.

In [[KT08](#bib.bibx22), [BCTV14](#bib.bibx4)], MNT curves were used to give the first construction of pairing-friendly 2-cycles. In this section, we construct MNT 4-cycles, and characterize the possible MNT cycles.

###### Proposition 2.

All MNT cycles have lengths 2 or 4, and they are either (6,4)(6,4)-cycles or (6,4,6,4)(6,4,6,4)-cycles.

The proof of this result proceeds in a few steps. First in [Lemma 5](#Thmlemma5 "Lemma 5. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") we show that no curve in an MNT cycle can have embedding degree 3. Then in [Lemmas 7](#Thmlemma7 "Lemma 7. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") and [8](#Thmlemma8 "Lemma 8. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") we show that no two consecutive curves in an MNT cycle can both have embedding degree 4 or 6. Finally we consider MNT cycles with alternating embedding degrees 4 and 6, and we show that these can only have lengths 2 or 4.

###### Lemma 5.

Let E1/𝔽qk1​(x1),…,Em/𝔽qkm​(xm)E\_{1}/\mathbb{F}\_{q\_{k\_{1}}(x\_{1})},\ldots,E\_{m}/\mathbb{F}\_{q\_{k\_{m}}(x\_{m})} be an MNT cycle, with x1,…,xm∈ℤx\_{1},\ldots,x\_{m}\in\mathbb{Z} and embedding degrees k1,…,km∈{3,4,6}k\_{1},\ldots,k\_{m}\in\{3,4,6\}. Then none of the embedding degrees can be 3.

To show [Lemma 5](#Thmlemma5 "Lemma 5. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"), we make use of the following result.

###### Lemma 6 ([[Was97](#bib.bibx37), Proposition 2.10]).

Let qq be a prime such that q∤kq\nmid k. Then qq divides Φk​(a)\Phi\_{k}(a) for some a∈ℤa\in\mathbb{Z} if and only if q≡1(modk)q\equiv 1\pmod{k}.

###### Proof of [Lemma 5](#Thmlemma5 "Lemma 5. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves").

By [Lemma 1](#Thmlemma1 "Lemma 1. ‣ 2.1 Elliptic curves and pairings ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), the condition that Ei/𝔽qki​(xi)E\_{i}/\mathbb{F}\_{q\_{k\_{i}}(x\_{i})} has embedding degree kik\_{i} implies that nki​(xi)|Φki​(qki​(xi))n\_{k\_{i}}(x\_{i})\,|\,\Phi\_{k\_{i}}(q\_{k\_{i}}(x\_{i})). Since nki​(xi)=qki+1​(xi+1)n\_{k\_{i}}(x\_{i})=q\_{k\_{i+1}}(x\_{i+1}), [Lemma 6](#Thmlemma6 "Lemma 6 ([, Proposition 2.10]). ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") implies

|  |  |  |  |
| --- | --- | --- | --- |
|  | qki+1​(xi+1)≡1(modki).q\_{k\_{i+1}}(x\_{i+1})\equiv 1\pmod{k\_{i}}\,. |  | (9) |

Suppose that kj=3k\_{j}=3 for some jj. From [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"),

|  |  |  |  |
| --- | --- | --- | --- |
|  | q3​(xj)=12​xj2−1≡1(modkj−1).q\_{3}(x\_{j})=12x\_{j}^{2}-1\equiv 1\pmod{k\_{j-1}}\,. |  | (10) |

However, this is not possible since 12​xj2−1≡−1(mod3,4,6)12x\_{j}^{2}-1\equiv-1\pmod{3,4,6}.
∎

We show that for any MNT cycle, no two consecutive curves can both have embedding degree 44 or 66.

###### Lemma 7.

Let E1/𝔽qk1​(x1),…,Em/𝔽qkm​(xm)E\_{1}/\mathbb{F}\_{q\_{k\_{1}}(x\_{1})},\ldots,E\_{m}/\mathbb{F}\_{q\_{k\_{m}}(x\_{m})} be an MNT cycle, with x1,…,xm∈ℤx\_{1},\ldots,x\_{m}\in\mathbb{Z}. Then no two consecutive curves can both have embedding degree 44.

###### Proof.

Suppose to the contrary that ki=ki+1=4k\_{i}=k\_{i+1}=4 for some ii. Then n4​(xi)=q4​(xi+1)n\_{4}(x\_{i})=q\_{4}(x\_{i+1}). From [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), q4​(xi+1)=xi+12+xi+1+1q\_{4}(x\_{i+1})=x\_{i+1}^{2}+x\_{i+1}+1, and there are two possibilities for n4​(xi)n\_{4}(x\_{i}).

Suppose n4​(xi)=xi2+2​xi+2n\_{4}(x\_{i})=x\_{i}^{2}+2x\_{i}+2. Then xi2+2​xi+2=xi+12+xi+1+1x\_{i}^{2}+2x\_{i}+2=x\_{i+1}^{2}+x\_{i+1}+1, which implies

|  |  |  |  |
| --- | --- | --- | --- |
|  | (xi+1)2=xi+1​(xi+1+1).(x\_{i}+1)^{2}=x\_{i+1}(x\_{i+1}+1)\,. |  | (11) |

This is a contradiction if xi≠−1x\_{i}\neq-1, since the product of two consecutive nonzero integers is not a square.222
Suppose that for some nonzero x,y∈ℤx,y\in\mathbb{Z}, x⁡(x+1)=y2x(x+1)=y^{2}. If x>0x>0, then
x2<y2<(x+1)2x^{2}<y^{2}<(x+1)^{2}, which has no integer solutions for x,yx,y. If x<0x<0, then x2>y2>(x+1)2x^{2}>y^{2}>(x+1)^{2}, which also has no integer solutions for x,yx,y. But if xi=−1x\_{i}=-1, then n4​(xi)=1n\_{4}(x\_{i})=1 would not be prime.

Suppose n4​(xi)=xi2+1n\_{4}(x\_{i})=x\_{i}^{2}+1. Then xi2+1=xi+12+xi+1+1x\_{i}^{2}+1=x\_{i+1}^{2}+x\_{i+1}+1, which implies

|  |  |  |  |
| --- | --- | --- | --- |
|  | xi2=xi+1​(xi+1+1).x\_{i}^{2}=x\_{i+1}(x\_{i+1}+1)\,. |  | (12) |

This is a contradiction by the same argument as above.
∎

###### Lemma 8.

Let E1/𝔽qk1​(x1),…,Em/𝔽qkm​(xm)E\_{1}/\mathbb{F}\_{q\_{k\_{1}}(x\_{1})},\ldots,E\_{m}/\mathbb{F}\_{q\_{k\_{m}}(x\_{m})} be an MNT cycle with x1,…,xm∈ℤx\_{1},\ldots,x\_{m}\in\mathbb{Z}. Then no two consecutive curves can both have embedding degree 66.

###### Proof.

Suppose to the contrary that ki=ki+1=6k\_{i}=k\_{i+1}=6 for some ii. Then n6​(xi)=q6​(xi+1)n\_{6}(x\_{i})=q\_{6}(x\_{i+1}). From [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), q6​(xi+1)=4​xi+12+1q\_{6}(x\_{i+1})=4x\_{i+1}^{2}+1, and n6​(xi)=4​xi2+2​xi+1n\_{6}(x\_{i})=4x\_{i}^{2}+2x\_{i}+1. Thus 4​xi2+2​xi+1=4​xi+12+14x\_{i}^{2}+2x\_{i}+1=4x\_{i+1}^{2}+1, which implies

|  |  |  |  |
| --- | --- | --- | --- |
|  | 2​xi​(2​xi+1)=(2​xi+1)2.2x\_{i}(2x\_{i}+1)=(2x\_{i+1})^{2}\,. |  | (13) |

This is a contradiction if xi+1≠0x\_{i+1}\neq 0, since the product of two consecutive nonzero integers is not a square. But if xi+1=0x\_{i+1}=0, then q6​(xi+1)=1q\_{6}(x\_{i+1})=1 would not be prime.
∎

We now consider MNT cycles consisting of elliptic curves with alternating
embedding degrees 44 and 66.

###### Lemma 9.

Let Ei/𝔽q4​(xi)E\_{i}/\mathbb{F}\_{q\_{4}}(x\_{i}), Ei+1/𝔽q6​(xi+1)E\_{i+1}/\mathbb{F}\_{q\_{6}}(x\_{i+1})
be consecutive curves in an MNT cycle. Then 2​|xi+1|=|xi|2|x\_{i+1}|=|x\_{i}| or 2​|xi+1|=|xi+1|2|x\_{i+1}|=|x\_{i}+1|.

###### Proof.

We have the condition n4​(xi)=q6​(xi+1)n\_{4}(x\_{i})=q\_{6}(x\_{i+1}). By
[Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), q6​(xi+1)=4​xi+12+1q\_{6}(x\_{i+1})=4x\_{i+1}^{2}+1, and there are
two possibilities for n4​(xi)n\_{4}(x\_{i}).

If n4​(xi)=xi2+2​xi+2n\_{4}(x\_{i})=x\_{i}^{2}+2x\_{i}+2, then xi2+2​xi+2=4​xi+12+1x\_{i}^{2}+2x\_{i}+2=4x\_{i+1}^{2}+1, which we simplify to (xi+1)2=(2​xi+1)2(x\_{i}+1)^{2}=(2x\_{i+1})^{2}. Thus 2​|xi+1|=|xi+1|2|x\_{i+1}|=|x\_{i}+1|.

If instead n4​(xi)=xi2+1n\_{4}(x\_{i})=x\_{i}^{2}+1, then xi2+1=4​xi+12+1x\_{i}^{2}+1=4x\_{i+1}^{2}+1, which we simplify to xi2=(2​xi+1)2x\_{i}^{2}=(2x\_{i+1})^{2}. Thus 2​|xi+1|=|xi|2|x\_{i+1}|=|x\_{i}|.
∎

###### Lemma 10.

Let Ei/𝔽q6​(xi)E\_{i}/\mathbb{F}\_{q\_{6}}(x\_{i}), Ei+1/𝔽q4​(xi+1)E\_{i+1}/\mathbb{F}\_{q\_{4}}(x\_{i+1}) be consecutive curves in an MNT cycle. Then xi+1=2​xix\_{i+1}=2x\_{i}.

###### Proof.

We have the condition n6​(xi)=q4​(xi+1)n\_{6}(x\_{i})=q\_{4}(x\_{i+1}). By [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), this gives 4​xi2+2​xi+1=xi+12+xi+1+14x\_{i}^{2}+2x\_{i}+1=x\_{i+1}^{2}+x\_{i+1}+1, or 2​xi​(2​xi+1)=xi+1​(xi+1+1)2x\_{i}(2x\_{i}+1)=x\_{i+1}(x\_{i+1}+1). This implies xi+1=2​xix\_{i+1}=2x\_{i}.
∎

We now show [Proposition 2](#Thmproposition2 "Proposition 2. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") that all MNT cycles are (6,4)(6,4)-cycles or (6,4,6,4)(6,4,6,4)-cycles.

###### Proof of [Proposition 2](#Thmproposition2 "Proposition 2. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves").

By [Lemma 5](#Thmlemma5 "Lemma 5. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"), [Lemma 7](#Thmlemma7 "Lemma 7. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") and [Lemma 8](#Thmlemma8 "Lemma 8. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"), all MNT cycles consist of curves with embedding degrees alternating between 44 and 66, and have even lengths.

Let E1/𝔽q6​(x1),E2/𝔽q4​(x2),…,E2​m/𝔽q4​(x2​m)E\_{1}/\mathbb{F}\_{q\_{6}}(x\_{1}),E\_{2}/\mathbb{F}\_{q\_{4}}(x\_{2}),\ldots,E\_{2m}/\mathbb{F}\_{q\_{4}}(x\_{2m}) be an MNT cycle. We first observe that [Lemma 9](#Thmlemma9 "Lemma 9. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") and [Lemma 10](#Thmlemma10 "Lemma 10. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") imply that |x1|=|x3|=⋯=|x2​m−1||x\_{1}|=|x\_{3}|=\cdots=|x\_{2m-1}|. Thus q6​(x1)=q6​(x3)=⋯=q6​(x2​m−1)q\_{6}(x\_{1})=q\_{6}(x\_{3})=\cdots=q\_{6}(x\_{2m-1}). As there are only two possibilities for n6​(x1),n6​(x3),…,n6​(x2​m−1)n\_{6}(x\_{1}),n\_{6}(x\_{3}),\ldots,n\_{6}(x\_{2m-1}), for the curves to be distinct we must have m≤4m\leq 4, and if m=4m=4 then we must have x3=−x1x\_{3}=-x\_{1}.

Let x:=x1x:=x\_{1}. Then [Lemma 10](#Thmlemma10 "Lemma 10. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") implies x2=2​xx\_{2}=2x. By [Lemma 9](#Thmlemma9 "Lemma 9. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"), either x3=xx\_{3}=x, in which case we have a (6,4)(6,4)-cycle, or x3=−xx\_{3}=-x. For the latter case, [Lemma 10](#Thmlemma10 "Lemma 10. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") implies that x4=−2​xx\_{4}=-2x, which gives us a (6,4,6,4)(6,4,6,4)-cycle.

By substituting the possible parameter values for xx into the polynomials in [Table 1](#S2.T1 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), we obtain the parametrizations of the possible families of MNT (6,4)(6,4)-cycles in [Table 2](#S4.T2 "In 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") and (6,4,6,4)(6,4,6,4)-cycles in [Table 3](#S4.T3 "In 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"). These cycles can be constructed by substituting integer values of xx and checking if all the n⁡(x)n(x)’s and q⁡(x)q(x)’s are prime.
∎

The MNT (6,4,6,4)(6,4,6,4)-cycles in [Table 3](#S4.T3 "In 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") are unions of two MNT (6,4)(6,4)-cycles. Indeed, the pairs (E1,E2)(E\_{1},E\_{2}) and (E3,E4)(E\_{3},E\_{4}) each form (6,4)(6,4)-cycles. Furthermore, E1,E3E\_{1},E\_{3} are defined over the same finite field. Interestingly, these are the only possible MNT 4-cycles, and no longer cycles consisting of distinct elliptic curves can be obtained by taking unions of MNT 2-cycles.

Table 2: MNT (6,4)(6,4)-cycles.

|  | E1E\_{1} | E2E\_{2} |
| --- | --- | --- |
| kk | 66 | 44 |
| q⁡(x)q(x) | 4​x2+14x^{2}+1 | 4​x2+2​x+14x^{2}+2x+1 |
| n⁡(x)n(x) | 4​x2+2​x+14x^{2}+2x+1 | 4​x2+14x^{2}+1 |
| t⁡(x)t(x) | −2​x+1-2x+1 | 2​x+12x+1 |

Table 3: MNT (6,4,6,4)(6,4,6,4)-cycles.

|  | E1E\_{1} | E2E\_{2} | E3E\_{3} | E4E\_{4} |
| --- | --- | --- | --- | --- |
| kk | 66 | 44 | 66 | 44 |
| q⁡(x)q(x) | 4​x2+14x^{2}+1 | 4​x2+2​x+14x^{2}+2x+1 | 4​x2+14x^{2}+1 | 4​x2−2​x+14x^{2}-2x+1 |
| n⁡(x)n(x) | 4​x2+2​x+14x^{2}+2x+1 | 4​x2+14x^{2}+1 | 4​x2−2​x+14x^{2}-2x+1 | 4​x2+14x^{2}+1 |
| t⁡(x)t(x) | −2​x+1-2x+1 | 2​x+12x+1 | 2​x+12x+1 | −2​x+1-2x+1 |

###### Example 1.

We give an example of an MNT (6,4)(6,4)-cycle, using the parametrization in [Table 2](#S4.T2 "In 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"). If x=1x=1, we check that 4​x2+1=54x^{2}+1=5 and 4​x2−2​x+1=34x^{2}-2x+1=3 are prime. We compute each of the two curves in the cycle using the CM method and Sage [[The17](#bib.bibx36)].

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | | | | |
|  | E1/𝔽5\displaystyle E\_{1}/\mathbb{F}\_{5}\, | :y2=x3+4​x+2,\displaystyle:\,y^{2}=x^{3}+4x+2\,, |  | (14a) |
|  | E2/𝔽3\displaystyle E\_{2}/\mathbb{F}\_{3}\, | :y2=x3+2​x2+1.\displaystyle:\,y^{2}=x^{3}+2x^{2}+1\,. |  | (14b) |

We list all the points of these curves in [Table 5](#A1.T5 "In Appendix A Dual Elliptic Primes ‣ On cycles of pairing-friendly elliptic curves").

###### Example 2.

We give an example of an MNT (6,4,6,4)(6,4,6,4)-cycle, using the parametrization in [Table 3](#S4.T3 "In 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"). If x=3x=3, we check that 4​x2+1=374x^{2}+1=37, 4​x2+2​x+1=434x^{2}+2x+1=43 and 4​x2−2​x+1=314x^{2}-2x+1=31 are all prime. We compute the curves using Sage [[The17](#bib.bibx36)].

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | | | | |
|  | E1/𝔽37\displaystyle E\_{1}/\mathbb{F}\_{37}\, | :y2=x3+24​x+16,\displaystyle:\,y^{2}=x^{3}+24x+16\,, |  | (15a) |
|  | E2/𝔽43\displaystyle E\_{2}/\mathbb{F}\_{43}\, | :y2=x3+36​x+5,\displaystyle:\,y^{2}=x^{3}+36x+5\,, |  | (15b) |
|  | E3/𝔽37\displaystyle E\_{3}/\mathbb{F}\_{37}\, | :y2=x3+22​x+27,\displaystyle:\,y^{2}=x^{3}+22x+27\,, |  | (15c) |
|  | E4/𝔽31\displaystyle E\_{4}/\mathbb{F}\_{31}\, | :y2=x3+26​x+21.\displaystyle:\,y^{2}=x^{3}+26x+21\,. |  | (15d) |

We list all the points of these curves in [Table 6](#A1.T6 "In Appendix A Dual Elliptic Primes ‣ On cycles of pairing-friendly elliptic curves").

5 Two-cycles of specific embedding degrees
------------------------------------------

In this section we prove the following result.

###### Proposition 3.

There are no (5,10)(5,10)-, (8,8)(8,8)-, or (12,12)(12,12)-cycles.

The pairs (5,10),(8,8),(12,12)(5,10),(8,8),(12,12) are precisely the pairs (k1,k2)(k\_{1},k\_{2}) whose cyclotomic polynomials satisfy Φk1​(x)=Φk2​(−x)\Phi\_{k\_{1}}(x)=\Phi\_{k\_{2}}(-x) and deg⁡Φk1​(x)=4\deg\Phi\_{k\_{1}}(x)=4. To prove [Proposition 3](#Thmproposition3 "Proposition 3. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves"), we first use these conditions to reduce from the problem of classifying (k1,k2)(k\_{1},k\_{2})-cycles to that of finding integral points on a few quartic curves, with finitely many exceptions, in [Lemma 12](#Thmlemma12 "Lemma 12. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves"). We then classify all integral points on these quartic curves and the finitely many exceptions using computational tools, yielding no actual (k1,k2)(k\_{1},k\_{2})-cycles.

Note that in the case of 2-cycles, when we require nontrivial embedding degrees, the two curves cannot have equal field sizes.333
Even when allowed, curves E/𝔽qE/\mathbb{F}\_{q} with q=♯​E​(𝔽q)q=\sharp E(\mathbb{F}\_{q}), known as anomalous, are undesirable because discrete logarithms can be computed in polynomial time via the SSSA attack [[Sem98](#bib.bibx31), [Sma99](#bib.bibx33), [SA98](#bib.bibx30)].

We first prove the following more general result, which we hope will also have applications to other kinds of 2-cycles.

###### Lemma 11.

Let (k1,k2)(k\_{1},k\_{2}) satisfy Φk1​(x)=Φk2​(−x)\Phi\_{k\_{1}}(x)=\Phi\_{k\_{2}}(-x). Let E1/𝔽q1,E2/𝔽q2E\_{1}/\mathbb{F}\_{q\_{1}},E\_{2}/\mathbb{F}\_{q\_{2}} be a (k1,k2)(k\_{1},k\_{2})-cycle with q1>q2q\_{1}>q\_{2}, and let c=q1−q2c=q\_{1}-q\_{2}. Then q1​q2|Φk1​(c)q\_{1}q\_{2}\mid\Phi\_{k\_{1}}(c). Additionally, for some integer dd whose prime divisors are all congruent to 1(modk1)1\pmod{k\_{1}}, there is an integer yy such that

|  |  |  |  |
| --- | --- | --- | --- |
|  | y2=c2​d2+4​d​Φk1​(c).y^{2}=c^{2}d^{2}+4d\Phi\_{k\_{1}}(c)\,. |  | (16) |

###### Proof.

By [Lemma 1](#Thmlemma1 "Lemma 1. ‣ 2.1 Elliptic curves and pairings ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves"), the condition that E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}} has embedding degree k1k\_{1} implies that q2|Φk1​(q1)q\_{2}\mid\Phi\_{k\_{1}}(q\_{1}). Then q2|Φk1​(q1−q2)q\_{2}\mid\Phi\_{k\_{1}}(q\_{1}-q\_{2}) as well. Similarly, q1|Φk2​(q2−q1)=Φk1​(q1−q2)q\_{1}\mid\Phi\_{k\_{2}}(q\_{2}-q\_{1})=\Phi\_{k\_{1}}(q\_{1}-q\_{2}). It follows that q1​q2|Φk1​(q1−q2)=Φk1​(c)q\_{1}q\_{2}\mid\Phi\_{k\_{1}}(q\_{1}-q\_{2})=\Phi\_{k\_{1}}(c) as q1q\_{1} and q2q\_{2} are distinct primes.

Then d​q1​q2=Φk1​(c)dq\_{1}q\_{2}=\Phi\_{k\_{1}}(c) for some integer dd. Using q1=q2+cq\_{1}=q\_{2}+c, we can rewrite this as

|  |  |  |  |
| --- | --- | --- | --- |
|  | d​q22+c​d​q2−Φk1​(c)=0.dq\_{2}^{2}+cdq\_{2}-\Phi\_{k\_{1}}(c)=0\,. |  | (17) |

For this quadratic equation in q2q\_{2} to have an integral solution, the discriminant

|  |  |  |  |
| --- | --- | --- | --- |
|  | c2​d2+4​d​Φk1​(c)c^{2}d^{2}+4d\Phi\_{k\_{1}}(c) |  | (18) |

must be a perfect square, so that there is a yy satisfying [Eq. 16](#S5.E16 "In Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves").

Also, for any prime p|dp\mid d, the above relation d​q1​q2=Φk1​(c)dq\_{1}q\_{2}=\Phi\_{k\_{1}}(c) implies that p|Φk1​(c)p\mid\Phi\_{k\_{1}}(c). Hence p≡1(modk1)p\equiv 1\pmod{k\_{1}} by [Lemma 6](#Thmlemma6 "Lemma 6 ([, Proposition 2.10]). ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves").
∎

###### Lemma 12.

In the situation of [Lemma 11](#Thmlemma11 "Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves"), additionally let deg⁡Φk1​(x)=4\deg\Phi\_{k\_{1}}(x)=4. Equivalently, let (k1,k2)∈{(5,10),(8,8),(10,5),(12,12)}(k\_{1},k\_{2})\in\{(5,10),(8,8),(10,5),(12,12)\}. Then c≤82c\leq 82 or 1≤d≤161\leq d\leq 16.

###### Proof.

Let c≥83c\geq 83. Then Φk1​(c)>0\Phi\_{k\_{1}}(c)>0, so the relation d​q1​q2=Φk1​(c)dq\_{1}q\_{2}=\Phi\_{k\_{1}}(c) implies d≥1d\geq 1. Next, because E2/𝔽q2E\_{2}/\mathbb{F}\_{q\_{2}} has q1q\_{1} points, the Hasse bound implies |q1−(q2+1)|≤2​q2|q\_{1}-(q\_{2}+1)|\leq 2\sqrt{q\_{2}}. Substituting c=q1−q2c=q\_{1}-q\_{2} and rearranging shows q2≥(c−1)2/4q\_{2}\geq(c-1)^{2}/4. The same holds for q1q\_{1} since q1>q2q\_{1}>q\_{2}. Then d​q1​q2=Φk1​(c)dq\_{1}q\_{2}=\Phi\_{k\_{1}}(c) implies

|  |  |  |  |
| --- | --- | --- | --- |
|  | d<16​Φk1​(c)(c−1)4.d<16\frac{\Phi\_{k\_{1}}(c)}{(c-1)^{4}}\,. |  | (19) |

For each k1∈{5,8,10,12}k\_{1}\in\{5,8,10,12\}, we find that for c≥83c\geq 83, the right-hand side is at most 17. Thus either c≤82c\leq 82 or 1≤d≤161\leq d\leq 16.
∎

For each (k1,k2)(k\_{1},k\_{2}) listed in [Lemma 12](#Thmlemma12 "Lemma 12. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves"), using the fact q1,q2|Φk1​(c)q\_{1},q\_{2}\mid\Phi\_{k\_{1}}(c) from [Lemma 11](#Thmlemma11 "Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves"), one can see that the case c≤82c\leq 82 yields only finitely many (k1,k2)(k\_{1},k\_{2})-cycles. Also, for each 1≤d≤161\leq d\leq 16 whose prime divisors are congruent to 1(modk1)1\pmod{k\_{1}}, one can show that [Eq. 16](#S5.E16 "In Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves") defines a plane curve of genus 1 in the coordinates (c,y)(c,y). Siegel’s Theorem [[Lan83](#bib.bibx23), Theorem 8.2.4] implies that such a curve has only finitely many integral points, hence there are only finitely many (k1,k2)(k\_{1},k\_{2})-cycles.

We now use computational tools to show that there are in fact no (k1,k2)(k\_{1},k\_{2})-cycles.

###### Proof of [Proposition 3](#Thmproposition3 "Proposition 3. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves").

Using the fact q1,q2|Φk1​(c)q\_{1},q\_{2}\mid\Phi\_{k\_{1}}(c) from [Lemma 11](#Thmlemma11 "Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves"), it is easy to enumerate all (k1,k2)(k\_{1},k\_{2})-cycles which have c≤82c\leq 82, for (k1,k2)∈{(5,10),(8,8),(10,5),(12,12)}(k\_{1},k\_{2})\in\{(5,10),(8,8),(10,5),(12,12)\}. Doing so using Sage [[The17](#bib.bibx36)] reveals no such examples.

We now consider the case d≤16d\leq 16. Restricting to values of dd whose prime factors are all congruent to 1(modk1)1\pmod{k\_{1}}, we are left with the cases shown in [Table 4](#S5.T4 "In Proof of . ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves").

Table 4: Cases ((k1,k2),d)((k\_{1},k\_{2}),d) satisfying [Lemma 12](#Thmlemma12 "Lemma 12. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves") when c≥83c\geq 83.

| (k1,k2)(k\_{1},k\_{2}) | dd |
| --- | --- |
| (5,10)(5,10) | 11 |
| (10,5)(10,5) | 13 |
| (12,12)(12,12) | 13 |

In the case (k1,k2)=(12,12)(k\_{1},k\_{2})=(12,12), d=13d=13, we can enumerate the integral points of [Eq. 16](#S5.E16 "In Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves") using Magma’s IntegralQuarticPoints function [[BCP97](#bib.bibx3)]. Doing so gives no examples with c≥83c\geq 83.

When (k1,k2)=(5,10)(k\_{1},k\_{2})=(5,10) or (10,5)(10,5) and d=11d=11, Sage [[The17](#bib.bibx36)] finds that [Eq. 16](#S5.E16 "In Lemma 11. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves") has no solutions over the ring of integers modulo 16, hence it has no integral solutions. Thus these cases also give no examples.
∎

When deg⁡Φk1​(x)>4\deg\Phi\_{k\_{1}}(x)>4, the bound on dd in [Eq. 19](#S5.E19 "In Proof. ‣ 5 Two-cycles of specific embedding degrees ‣ On cycles of pairing-friendly elliptic curves") no longer converges to a finite value as c→∞c\rightarrow\infty, so we cannot reduce to finding integral points on a finite number of curves as above. It would be interesting to find more general arguments which work for higher-degree cyclotomic polynomials, such as the case of (16,16)(16,16)-cycles, where Φk1​(x)=Φk2​(x)=x8+1\Phi\_{k\_{1}}(x)=\Phi\_{k\_{2}}(x)=x^{8}+1.

6 Cycles with the same discriminant
-----------------------------------

In this section we show that if we construct cycles from elliptic curves of the same discriminant DD, then the length of the cycle must be small. This implies that to construct elliptic curves from polynomial families, we cannot use families with a fixed discriminant. The results in this section are *independent* of the embedding degrees of the elliptic curves.

We first show that any 2-cycle of ordinary elliptic curves consists of curves with the same
discriminant.

###### Proposition 4.

Let E1/𝔽q1,E2/𝔽q2E\_{1}/\mathbb{F}\_{q\_{1}},E\_{2}/\mathbb{F}\_{q\_{2}} be a 2-cycle of ordinary elliptic curves. Then they both have the same discriminant for complex multiplication.

###### Proof.

Let tit\_{i} be the trace of EiE\_{i} for each ii. Then q2=q1+1−t1q\_{2}=q\_{1}+1-t\_{1} and q1=q2+1−t2q\_{1}=q\_{2}+1-t\_{2}. This implies t1+t2=2t\_{1}+t\_{2}=2, and

|  |  |  |
| --- | --- | --- |
|  | 4​q2−t22=4​(q1+1−t1)−(2−t1)2=4​q1−t12.\displaystyle 4q\_{2}-t\_{2}^{2}=4(q\_{1}+1-t\_{1})-(2-t\_{1})^{2}=4q\_{1}-t\_{1}^{2}\,. |  |

The discriminant of EiE\_{i} is the squarefree part of 4​qi−ti24q\_{i}-t\_{i}^{2}, so the two curves have the same discriminant.
∎

The converse is also true if D>3D>3, as shown in [[SS11](#bib.bibx34), Corollary 6.2] and [[BBC+12](#bib.bibx1), Theorem 3.4]. We present an adapted version of the proof below.

###### Proposition 5.

Let D>3D>3 be a squarefree integer such that −D≡0,1(mod4)-D\equiv 0,1\pmod{4}. Suppose that we have an mm-cycle of ordinary elliptic curves E1/𝔽q1,…,Em/𝔽qmE\_{1}/\mathbb{F}\_{q\_{1}},\ldots,E\_{m}/\mathbb{F}\_{q\_{m}} such that each elliptic curve has discriminant DD and q1,…,qmq\_{1},\ldots,q\_{m} are distinct primes. Then m≤2m\leq 2.

###### Proof.

For each i=1,…,mi=1,\ldots,m, let yi∈ℤy\_{i}\in\mathbb{Z} be such that the CM equation 4​qi−ti2=D​yi24q\_{i}-t\_{i}^{2}=Dy\_{i}^{2} is satisfied. Firstly, we note that if we fix qiq\_{i} and DD, the solution (ti,yi)(t\_{i},y\_{i}) to the CM equation is unique up to sign. This follows from the fact that, under our assumptions on DD, the units in the ring of integers of ℚ⁡(−D)\mathbb{Q}(\sqrt{-D}) are ±1\pm 1, hence if two elements have the same norm, then they differ by a multiple of ±1\pm 1.

Now let Ei/𝔽qiE\_{i}/\mathbb{F}\_{q\_{i}}, Ei+1/𝔽qi+1E\_{i+1}/\mathbb{F}\_{q\_{i+1}} be two consecutive curves in the cycle. Since

|  |  |  |  |
| --- | --- | --- | --- |
|  | 4​qi+1−(ti−2)2=4​(qi+1−1+ti)−ti2=4​qi−ti2=D​yi2,\displaystyle 4q\_{i+1}-(t\_{i}-2)^{2}=4(q\_{i+1}-1+t\_{i})-t\_{i}^{2}=4q\_{i}-t\_{i}^{2}=Dy\_{i}^{2}\,, |  | (20) |

thus ti−2=±ti+1t\_{i}-2=\pm t\_{i+1}, and yi=±yi+1y\_{i}=\pm y\_{i+1}, by the uniqueness of the solution to the CM equation.

Suppose that m≥3m\geq 3. Without loss of generality, assume that q2q\_{2} is the smallest prime in the cycle. Then q2<q1,q3q\_{2}<q\_{1},q\_{3}. From the previous paragraph we also have t1−2=±t2t\_{1}-2=\pm t\_{2}. We consider the two cases separately.

If t1−2=t2t\_{1}-2=t\_{2}, then q2=q1−1−t2q\_{2}=q\_{1}-1-t\_{2}. So we have the inequalities q1=q2+1+t2>q2q\_{1}=q\_{2}+1+t\_{2}>q\_{2}, and q3=q2+1−t2>q2q\_{3}=q\_{2}+1-t\_{2}>q\_{2}. Hence 1>t2>−11>t\_{2}>-1 so t2=0t\_{2}=0. But this implies that q1=q3q\_{1}=q\_{3}, which contradicts the assumption that the qiq\_{i}’s are distinct.

If t1−2=−t2t\_{1}-2=-t\_{2}, then q2=q1−1+t2q\_{2}=q\_{1}-1+t\_{2}, so q1=q2+1−t2=q3q\_{1}=q\_{2}+1-t\_{2}=q\_{3}. This again contradicts the assumption that the qiq\_{i}’s are distinct.
∎

For the case where D=3D=3, we cite the following result from [[BBC+12](#bib.bibx1)].

###### Proposition 6 ([[BBC+12](#bib.bibx1), Theorem 3.4]).

Suppose that we have an mm-cycle of ordinary elliptic curves E1/𝔽q1,…,Em/𝔽qmE\_{1}/\mathbb{F}\_{q\_{1}},\ldots,E\_{m}/\mathbb{F}\_{q\_{m}} such that each elliptic curve has discriminant DD and q1,…,qmq\_{1},\ldots,q\_{m} are distinct primes. If m≥3m\geq 3, then m=6m=6 and D=3D=3.

The results in this section show that to construct mm-cycles of elliptic curves with a fixed discriminant DD, either m≤2m\leq 2 or m=6m=6 and D=3D=3. This places a strong restriction on possible cycles, and implies that we cannot construct long cycles from a single family of elliptic curves with a fixed discriminant. For example, the Barreto–Naehrig curves [[BN05](#bib.bibx9)] all have discriminant D=3D=3.

We also note that the results in this section do not depend on the embedding degrees of the elliptic curves. It remains an open question to understand how restricting the embedding degrees places further restrictions on the possible cycles.

7 Cycles with cofactors
-----------------------

Allowing for non-prime orders gives greater flexibility in constructing elliptic curves, while still having relevance to cryptographic applications. While there are few embedding degrees that can be achieved by current constructions of prime-order curves, there are methods that achieve *arbitrary* embedding degrees for composite-order curves [[CP01](#bib.bibx13), [DEM05](#bib.bibx14)]. While composite-order curves tend to be less preferable than prime-order curves in applications, they can still be practical and sometimes even preferable.444
For example, Barreto–Lynn-Scott curves [[BLS02](#bib.bibx8)] are composite-order curves that, thanks to their high embedding degrees, enable efficient implementations at high-security levels. As another example, Edwards curves [[Edw07](#bib.bibx16), [BL07](#bib.bibx7), [BBJ+08](#bib.bibx2)] are composite-order curves that, thanks to their complete formulas for addition, enable efficient implementations that resist various side channels (e.g., [[BDL+11](#bib.bibx5)]).

Nevertheless, we show in this section that allowing for non-prime orders does not give us greater flexibility in constructing cycles. Our arguments in this section rely only on the Hasse bound and the constraints on the orders of the elliptic curves posed by the cycle condition.

###### Definition 5.

An *mm-cycle of elliptic curves with cofactors* consists of mm distinct elliptic curves E1/𝔽q1E\_{1}/\mathbb{F}\_{q\_{1}},…\ldots, Em/𝔽qmE\_{m}/\mathbb{F}\_{q\_{m}} such that for positive integer cofactors h1,…,hmh\_{1},\ldots,h\_{m},

|  |  |  |  |
| --- | --- | --- | --- |
|  | #​E1​(𝔽q1)=h1​q2,…,#​Ei​(𝔽qi)=hi​qi+1,…,#​Em​(𝔽qm)=hm​q1.\displaystyle\#E\_{1}(\mathbb{F}\_{q\_{1}})=h\_{1}q\_{2}\,,\ldots,\#E\_{i}(\mathbb{F}\_{q\_{i}})=h\_{i}q\_{i+1}\,,\ldots,\#E\_{m}(\mathbb{F}\_{q\_{m}})=h\_{m}q\_{1}\,. |  | (21) |

If all the cofactors are 1, then [Definition 5](#Thmdefinition5 "Definition 5. ‣ 7 Cycles with cofactors ‣ On cycles of pairing-friendly elliptic curves") reduces to [Definition 1](#Thmdefinition1 "Definition 1. ‣ 1 Introduction ‣ On cycles of pairing-friendly elliptic curves"). We show that, for any m>1m>1, we cannot have mm-cycles of elliptic curves with any nontrivial cofactor (and large orders). We deduce this by considering only the Hasse bound on the orders of the curves.

###### Proposition 7.

For all m>1m>1, there exists no mm-cycle of elliptic curves having at least one nontrivial cofactor (greater than 11), if q1,…,qm>12​m2q\_{1},\ldots,q\_{m}>12m^{2}.

###### Proof.

We first prove this for the simpler case where m=2m=2. Suppose that we
have a 2-cycle of elliptic curves E1/𝔽q1,E2/𝔽q2E\_{1}/\mathbb{F}\_{q\_{1}},E\_{2}/\mathbb{F}\_{q\_{2}} with cofactors such that #​E1​(𝔽q1)=h1​q2\#E\_{1}(\mathbb{F}\_{q\_{1}})=h\_{1}q\_{2}, #​E2​(𝔽q2)=h2​q1\#E\_{2}(\mathbb{F}\_{q\_{2}})=h\_{2}q\_{1}. The Hasse bound for E1E\_{1} implies

|  |  |  |  |
| --- | --- | --- | --- |
|  | q1+1−2​q1≤h1​q2≤q1+1+2​q1.\displaystyle q\_{1}+1-2\sqrt{q\_{1}}\leq h\_{1}q\_{2}\leq q\_{1}+1+2\sqrt{q\_{1}}\,. |  | (22) |

We can express this as (q1−1)2≤h1​q2≤(q1+1)2(\sqrt{q\_{1}}-1)^{2}\leq h\_{1}q\_{2}\leq(\sqrt{q\_{1}}+1)^{2}. Applying the same argument to E2E\_{2}, we get the following two inequalities

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | q1−1\displaystyle\sqrt{q\_{1}}-1 | ≤h1​q2≤q1+1,\displaystyle\leq\sqrt{h\_{1}q\_{2}}\leq\sqrt{q\_{1}}+1\,, |  | (23) |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | q2−1\displaystyle\sqrt{q\_{2}}-1 | ≤h2​q1≤q2+1.\displaystyle\leq\sqrt{h\_{2}q\_{1}}\leq\sqrt{q\_{2}}+1\,. |  | (24) |

We can then bound q2q\_{2} as follows

|  |  |  |  |
| --- | --- | --- | --- |
|  | h1​q2≤1h2​(q2+1)+1.\displaystyle\sqrt{h\_{1}q\_{2}}\leq\frac{1}{\sqrt{h\_{2}}}(\sqrt{q\_{2}}+1)+1\,. |  | (25) |

If h1>1h\_{1}>1 or h2>1h\_{2}>1, this implies that

|  |  |  |  |
| --- | --- | --- | --- |
|  | q2≤h2+1h1​h2−1≤1+2h1​h2−1<3.\displaystyle\sqrt{q\_{2}}\leq\frac{\sqrt{h\_{2}}+1}{\sqrt{h\_{1}h\_{2}}-1}\leq 1+\frac{2}{\sqrt{h\_{1}h\_{2}}-1}<3\,. |  | (26) |

The same argument applies for bounding q1q\_{1}. Hence for any 2-cycle with nontrivial cofactors, the elliptic curves must have small orders.

We now extend the argument above to mm-cycles with cofactors, for all m>2m>2. Suppose we have an mm-cycle with cofactors #E1(𝔽q1)=h1q2,#E2(𝔽q2)=h2q3,…,#Em(𝔽qm)=hmq1\#E\_{1}(\mathbb{F}\_{q\_{1}})=h\_{1}q\_{2}\,,\#E\_{2}(\mathbb{F}\_{q\_{2}})=h\_{2}q\_{3}\,,\ldots,\#E\_{m}(\mathbb{F}\_{q\_{m}})=h\_{m}q\_{1}. Applying the same argument as before, we have the inequalities

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | | | | |
|  | hm​q1\displaystyle\sqrt{h\_{m}q\_{1}} | ≤qm+1\displaystyle\leq\sqrt{q\_{m}}+1 |  | (27a) |
|  |  | ≤1hm−1​(qm−1+1)+1\displaystyle\leq\frac{1}{\sqrt{h\_{m-1}}}(\sqrt{q\_{m-1}}+1)+1 |  |
|  |  | ≤1hm−1​hm−2​(qm−2+1)+(1+1hm−1)\displaystyle\leq\frac{1}{\sqrt{h\_{m-1}h\_{m-2}}}\left(\sqrt{q\_{m-2}}+1\right)+\left(1+\frac{1}{\sqrt{h\_{m-1}}}\right) |  |
|  |  | |  |
|  |  | ≤1hm−1⋯h1​(q1+1)+(1+1hm−1+⋯+1hm−1⋯h2).\displaystyle\leq\frac{1}{\sqrt{h\_{m-1}\cdots h\_{1}}}\left(\sqrt{q\_{1}}+1\right)+\left(1+\frac{1}{\sqrt{h\_{m-1}}}+\cdots+\frac{1}{\sqrt{h\_{m-1}\cdots h\_{2}}}\right)\,. |  | (27b) |

We simplify this to

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | q1​(1−1hm⋯h1)\displaystyle\sqrt{q\_{1}}\left(1-\frac{1}{\sqrt{h\_{m}\cdots h\_{1}}}\right) | ≤1hm+1hm​hm−1+⋯+1hm⋯h1.\displaystyle\leq\frac{1}{\sqrt{h\_{m}}}+\frac{1}{\sqrt{h\_{m}h\_{m-1}}}+\cdots+\frac{1}{\sqrt{h\_{m}\cdots h\_{1}}}\,. |  | (28) |

If at least one of h1,…,hmh\_{1},\ldots,h\_{m} is greater than 1, then we can bound q1q\_{1} as follows.

|  |  |  |  |
| --- | --- | --- | --- |
|  | q1≤m1−1hm⋯h1≤m1−12=(2+2)​m.\displaystyle\sqrt{q\_{1}}\leq\frac{m}{1-\frac{1}{\sqrt{h\_{m}\cdots h\_{1}}}}\leq\frac{m}{1-\frac{1}{\sqrt{2}}}=(2+\sqrt{2})m\,. |  | (29) |

The above argument applies for q2,…,qmq\_{2},\ldots,q\_{m}, hence qi≤(2+2)2​m2<12​m2q\_{i}\leq(2+\sqrt{2})^{2}m^{2}<12m^{2} for each ii. For cryptographic applications, we would require the elliptic curves to be defined over much larger fields than the size of the cycle, contrary to this bound.
∎

8 Other cycles on parametrized families
---------------------------------------

We have shown in [Section 7](#S7 "7 Cycles with cofactors ‣ On cycles of pairing-friendly elliptic curves") that it is not possible to construct cycles of elliptic curves with nontrivial cofactors (and large orders relative to cycle length). Hence cycles of elliptic curves must be assembled from *prime*-order elliptic curves. At present the only known families of *pairing-friendly* prime-order elliptic curves are the MNT curves for k=3,4,6k=3,4,6, Freeman curves for k=10k=10[[Fre06](#bib.bibx18)], and Barreto–Naehrig curves for k=12k=12[[BN05](#bib.bibx9)]. Now we prove that we cannot construct cycles from just Freeman curves or from just Barreto–Naehrig curves.

###### Proposition 8.

There do not exist cycles consisting only of Freeman curves.

###### Proof.

[Lemma 4](#Thmlemma4 "Lemma 4. ‣ 3 Cycles of pairing-friendly elliptic curves ‣ On cycles of pairing-friendly elliptic curves") poses a restriction on the sum of the traces in a cycle. The trace of the Freeman curves is parametrized by t⁡(x)=10​x2+5​x+3t(x)=10x^{2}+5x+3 (see [Eq. 4](#S2.E4 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")). We note that t⁡(x)>1t(x)>1 for all x∈ℝx\in\mathbb{R}, since the discriminant of t⁡(x)−1t(x)-1 is −55-55. Hence the condition in [Lemma 4](#Thmlemma4 "Lemma 4. ‣ 3 Cycles of pairing-friendly elliptic curves ‣ On cycles of pairing-friendly elliptic curves") cannot be satisfied for cycles consisting only of Freeman curves.
∎

###### Proposition 9.

There do not exist cycles consisting only of Barreto–Naehrig curves.

###### Proof.

We again use [Lemma 4](#Thmlemma4 "Lemma 4. ‣ 3 Cycles of pairing-friendly elliptic curves ‣ On cycles of pairing-friendly elliptic curves"). The trace of the Barreto–Naehrig curves is parametrized by t⁡(x)=6​x2+1t(x)=6x^{2}+1 (see [Eq. 5](#S2.E5 "In 2.2 Families of pairing-friendly elliptic curves ‣ 2 Preliminaries ‣ On cycles of pairing-friendly elliptic curves")), hence if we have a family of elliptic curves consisting only of Barreto–Naehrig curves, then each trace has to be 11. So x=0x=0 and q⁡(x)=n⁡(x)=1q(x)=n(x)=1 for every curve in the cycle, which is impossible since q⁡(x)q(x) and n⁡(x)n(x) have to be prime.
∎

We remark that the proof of [Lemma 5](#Thmlemma5 "Lemma 5. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves") also shows that there do not exist cycles consisting of just Barreto–Naehrig curves and MNT curves of embedding degree 3.

For combinations of MNT, Freeman, and Barreto–Naehrig curves, we did a preliminary investigation using Gröbner bases to find solutions to the following system of polynomial equations in mm variables x1,…,xmx\_{1},\ldots,x\_{m}, where k1,…,km∈{3,4,6,10,12}k\_{1},\ldots,k\_{m}\in\{3,4,6,10,12\}.

|  |  |  |  |
| --- | --- | --- | --- |
|  | nk1(x1)=qk2(x2),nk2(x2)=qk3(x3),…,nkm(xm)=qk1(x1).\displaystyle n\_{k\_{1}}(x\_{1})=q\_{k\_{2}}(x\_{2})\,,n\_{k\_{2}}(x\_{2})=q\_{k\_{3}}(x\_{3}),\ldots,n\_{k\_{m}}(x\_{m})=q\_{k\_{1}}(x\_{1})\,. |  | (30) |

For m≤4m\leq 4 we found that the ideals generated by these polynomials have dimension 00 apart from the MNT cycles in [Proposition 2](#Thmproposition2 "Proposition 2. ‣ 4 MNT cycles ‣ On cycles of pairing-friendly elliptic curves"), implying that we cannot construct other families of cycles of length up to 44. We leave it as an open problem to construct cycles from combinations of these families, or to show that they do not exist.

Appendix A Dual Elliptic Primes
-------------------------------

In [[Mih97](#bib.bibx24), [Mih07](#bib.bibx25)], dual elliptic primes were introduced for applications in primality proving.

###### Definition 6 ([[Mih07](#bib.bibx25), Definition 10]).

Two primes p,qp,q are *dual elliptic primes associated to an order 𝒪⊆ℚ⁡(−D)\mathcal{O}\subseteq\mathbb{Q}(\sqrt{-D})* if there is a prime π∈𝒪\pi\in\mathcal{O} such that p=π​π¯p=\pi\overline{\pi} and q=(π+ε)​(π+ε)¯q=(\pi+\varepsilon)\overline{(\pi+\varepsilon)} with ε=±1\varepsilon=\pm 1.

Dual elliptic primes are equivalent to 2-cycles of ordinary elliptic curves.

###### Proposition 10.

Let p,qp,q be dual elliptic primes associated to an order
𝒪⊆ℚ⁡(−D)\mathcal{O}\subseteq\mathbb{Q}(\sqrt{-D}). Then p,qp,q correspond
bijectively to a 2-cycle of ordinary elliptic curves E1/𝔽p,E2/𝔽qE\_{1}/\mathbb{F}\_{p},E\_{2}/\mathbb{F}\_{q} with complex multiplication by 𝒪\mathcal{O}.

###### Proof.

Let p,qp,q be dual elliptic primes. Then

|  |  |  |  |
| --- | --- | --- | --- |
|  | q=(π+ε)​(π+ε)¯=p+ε⁡(π+π¯)+1.\displaystyle q=(\pi+\varepsilon)\overline{(\pi+\varepsilon)}=p+\varepsilon(\pi+\overline{\pi})+1\,. |  | (31) |

Let t1=−ε⁡(π+π¯)t\_{1}=-\varepsilon(\pi+\overline{\pi}) and t2=ε⁡((π+ε)+(π+ε)¯)t\_{2}=\varepsilon((\pi+\varepsilon)+\overline{(\pi+\varepsilon)}). Then
q=p−t1+1q=p-t\_{1}+1 and t2=−t1+2t\_{2}=-t\_{1}+2, so p=q+1−t2p=q+1-t\_{2}. Thus the
elliptic curves E1/𝔽p,E2/𝔽qE\_{1}/\mathbb{F}\_{p},E\_{2}/\mathbb{F}\_{q} with traces
t1,t2t\_{1},t\_{2} respectively form a 2-cycle. Moreover, since p,qp,q are the norms of principal 𝒪\mathcal{O}-ideals, these elliptic curves have complex multiplication by 𝒪\mathcal{O}.

Conversely, let E1/𝔽p,E2/𝔽qE\_{1}/\mathbb{F}\_{p},E\_{2}/\mathbb{F}\_{q} be a 2-cycle of ordinary elliptic curves with complex multiplication by 𝒪\mathcal{O}. We can write the CM equations as 4​p−t12=y2​D4p-t\_{1}^{2}=y^{2}D and 4​q−t22=y2​D4q-t\_{2}^{2}=y^{2}D. Let λ1\lambda\_{1} be a root of x2−t1​x+px^{2}-t\_{1}x+p, and let λ2\lambda\_{2} be a root of x2−t2​x+qx^{2}-t\_{2}x+q, chosen such that

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | λ1\displaystyle\lambda\_{1} | =t1+y​−D2,\displaystyle=\frac{t\_{1}+y\sqrt{-D}}{2}\,, |  | (32) |
|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | λ2\displaystyle\lambda\_{2} | =t2−y​−D2.\displaystyle=\frac{t\_{2}-y\sqrt{-D}}{2}\,. |  | (33) |

Then p,qp,q are the norms of the principal 𝒪\mathcal{O}-ideals (λ1),(λ2)(\lambda\_{1}),(\lambda\_{2}) respectively. Since t2=2−t1t\_{2}=2-t\_{1}, we have λ2=1−λ1\lambda\_{2}=1-\lambda\_{1}. Let ε=±1\varepsilon=\pm 1 and π=−ε​λ1\pi=-\varepsilon\lambda\_{1}. Then p=λ1​λ1¯=π​π¯p=\lambda\_{1}\overline{\lambda\_{1}}=\pi\overline{\pi}, and q=λ2​λ2¯=(π+ε)​(π+ε)¯q=\lambda\_{2}\overline{\lambda\_{2}}=(\pi+\varepsilon)\overline{(\pi+\varepsilon)}.
∎

Table 5: Example of an MNT (6,4)(6,4)-cycle.

|  |  |  |
| --- | --- | --- |
|  | E1E\_{1} | E2E\_{2} |
|  | y2=x3+4​x+2y^{2}=x^{3}+4x+2 | y2=x3+2​x2+1y^{2}=x^{3}+2x^{2}+1 |
| (q,n,t,k,D)(q,n,t,k,D) | (5,3,3,6,11)(5,3,3,6,11) | (3,5,−1,4,11)(3,5,-1,4,11) |
| points (excluding point at infinity) | (3,1) (3,4) | (0,1) (0,2) (1,1) (1,2) |

Table 6: Example of an MNT (6,4,6,4)(6,4,6,4)-cycle.

|  |  |  |  |  |
| --- | --- | --- | --- | --- |
|  | E1E\_{1} | E2E\_{2} | E3E\_{3} | E4E\_{4} |
|  | y2=x3+24​x+16y^{2}=x^{3}+24x+16 | y2=x3+36​x+5y^{2}=x^{3}+36x+5 | y2=x3+22​x+27y^{2}=x^{3}+22x+27 | y2=x3+26​x+21y^{2}=x^{3}+26x+21 |
| (q,n,t,k,D)(q,n,t,k,D) | (37,43,−5,6,123)(37,43,-5,6,123) | (43,37,7,4,123)(43,37,7,4,123) | (37,31,7,6,11)(37,31,7,6,11) | (31,37,−5,4,11)(31,37,-5,4,11) |
| points (excluding point at infinity) | (0,4) (0,33) (1,2) (1,35) (3,2) (3,35) (4,18) (4,19) (7,3) (7,34) (9,6) (9,31) (12,16) (12,21) (13,3) (13,34) (14,5) (14,32) (17,3) (17,34) (18,8)   (18,29) (23,9) (23,28) (26,7) (26,30) (27,16) (27,21) (28,12) (28,25) (31,10) (31,27) (32,17) (32,20) (33,2) (33,35) (34,18) (34,19) (35,16) (35,21) (36,18) (36,19) | (3,21) (3,22) (4,16) (4,27) (5,3) (5,40) (7,16) (7,27) (8,17) (8,26) (12,12) (12,31) (13,2) (13,41) (18,11) (18,32) (19,18) (19,25)   (23,10) (23,33) (29,5) (29,38) (30,7) (30,36) (31,9) (31,34) (32,16) (32,27) (33,8) (33,35) (38,1) (38,42) (41,21) (41,22) (42,21) (42,22) | (0,8) (0,29) (3,3) (3,34) (5,15) (5,22) (8,7) (8,30) (10,10) (10,27) (11,3) (11,34) (12,13) (12,24) (23,3)   (23,34) (25,12) (25,25) (27,18) (27,19) (28,5) (28,32) (30,14) (30,23) (31,7) (31,30) (35,7) (35,30) (36,2) (36,35) | (2,9) (2,22) (3,8) (3,23) (5,11) (5,20) (7,9) (7,22) (8,11) (8,20) (10,14) (10,17) (13,13) (13,18) (15,2) (15,29) (16,10) (16,21)   (18,11) (18,20) (20,4) (20,27) (21,1) (21,30) (22,9) (22,22) (23,13) (23,18) (26,13) (26,18) (27,15) (27,16) (28,3) (28,28) (30,5) (30,26) |

Acknowledgements
----------------

We thank Bernd Sturmfels for helpful comments. We also thank
François Morain for informing us about the application of cycles
of elliptic curves to primality proving. We thank Pierre Houedry for
pointing out an error in a previous version of our paper. This work was supported by the UC Berkeley Center for Long-Term Cybersecurity and a UC Berkeley University Fellowship.

References
----------

* [BBC+12]

  L. Babinkostova, K. M. Bombardier, M. M. Cole, T. A. Morrell, and C. B.
  Scott.
  Elliptic Reciprocity.
  ArXiv e-prints, December 2012.
* [BBJ+08]

  Daniel J. Bernstein, Peter Birkner, Marc Joye, Tanja Lange, and Christiane
  Peters.
  Twisted Edwards curves.
  In Proceedings of the 1st International Conference on Cryptology
  in Africa, AFRICACRYPT’ 08, pages 389–405, 2008.
* [BCP97]

  Wieb Bosma, John Cannon, and Catherine Playoust.
  The Magma algebra system. I. The user language.
  J. Symbolic Comput., 24(3-4):235–265, 1997.
  Computational algebra and number theory (London, 1993).
* [BCTV14]

  Eli Ben-Sasson, Alessandro Chiesa, Eran Tromer, and Madars Virza.
  Scalable zero knowledge via cycles of elliptic curves.
  In Proceedings of the 34th Annual International Cryptology
  Conference, CRYPTO ’14, pages 276–294, 2014.
  Extended version at <http://eprint.iacr.org/2014/595>.
* [BDL+11]

  Daniel J. Bernstein, Niels Duif, Tanja Lange, Peter Schwabe, and Bo-Yin Yang.
  High-speed high-security signatures.
  In Proceedings of the 13th International Conference on
  Cryptographic Hardware and Embedded Systems, CHES ’11, pages 124–142, 2011.
* [BF03]

  Dan Boneh and Matthew K. Franklin.
  Identity-based encryption from the Weil pairing.
  SIAM Journal on Computing, 32(3):586–615, 2003.
* [BL07]

  Daniel J. Bernstein and Tanja Lange.
  Faster addition and doubling on elliptic curves.
  In Proceedings of the 13th International Conference on the
  Theory and Application of Cryptology and Information Security, ASIACRYPT
  ’07, pages 29–50, 2007.
* [BLS02]

  Paulo SLM Barreto, Ben Lynn, and Michael Scott.
  Constructing elliptic curves with prescribed embedding degrees.
  In International Conference on Security in Communication
  Networks, pages 257–267. Springer, 2002.
* [BN05]

  Paulo SLM Barreto and Michael Naehrig.
  Pairing-friendly elliptic curves of prime order.
  In International Workshop on Selected Areas in Cryptography,
  pages 319–331. Springer, 2005.
* [Bre17]

  Arthur Breitman.
  Scaling Tezos.
  <https://hackernoon.com/scaling-tezo-8de241dd91bd>, 2017.
* [BW05]

  Friederike Brezing and Annegret Weng.
  Elliptic curves suitable for pairing based cryptography.
  Designs, Codes and Cryptography, 37(1):133–141, 2005.
* [Cod18]

  Coda.
  Coda cryptocurrency protocol.
  <https://codaprotocol.com/>, 2018.
* [CP01]

  C. Cocks and R.G.E. Pinch.
  Identity-based cryptosystems based on the Weil pairing.
  Unpublished manuscript, 2001.
* [DEM05]

  Régis Dupont, Andreas Enge, and François Morain.
  Building curves with arbitrary small MOV degree over finite prime
  fields.
  Journal of Cryptology, 18(2):79–89, 2005.
* [Deu41]

  Max Deuring.
  Die Typen der Multiplikatorenringe elliptischer
  Funktionenkörper.
  Abh. Math. Sem. Hansischen Univ., 14:197–272, 1941.
* [Edw07]

  Harold M. Edwards.
  A normal form for elliptic curves.
  Bulletin of the American Mathematical Society, 44(3):393–422,
  2007.
* [FR94]

  Gerhard Frey and Hans-Georg Rück.
  A remark concerning mm-divisibility and the discrete logarithm in
  the divisor class group of curves.
  Math. Comp., 62(206):865–874, 1994.
* [Fre06]

  David Freeman.
  Constructing pairing-friendly elliptic curves with embedding degree
  10.
  In International Algorithmic Number Theory Symposium, pages
  452–465. Springer, 2006.
* [FST10]

  David Freeman, Michael Scott, and Edlyn Teske.
  A taxonomy of pairing-friendly elliptic curves.
  Journal of Cryptology, 23(2):224–280, 2010.
* [Jon13]

  Nathan Jones.
  Elliptic aliquot cycles of fixed length.
  Pacific J. Math., 263(2):353–371, 2013.
* [Jou04]

  Antoine Joux.
  A one round protocol for tripartite Diffie–Hellman.
  Journal of Cryptology, 17(4):263–276, 2004.
* [KT08]

  Koray Karabina and Edlyn Teske.
  On prime-order elliptic curves with embedding degrees
  k = 3, 4, and 6.
  In Alfred J. van der Poorten and Andreas Stein, editors, Algorithmic Number Theory, pages 102–117, Berlin, Heidelberg, 2008.
  Springer Berlin Heidelberg.
* [Lan83]

  Serge Lang.
  Fundamentals of Diophantine Geometry.
  Springer New York, New York, NY, 1983.
* [Mih97]

  Preda Mihǎilescu.
  Cyclotomy of rings & primality testing.
  PhD thesis, ETH Zürich, Zürich, 1997.
* [Mih07]

  Preda Mihǎilescu.
  Dual Elliptic Primes and Applications to Cyclotomy Primality
  Proving.
  ArXiv e-prints, September 2007.
* [MNT01]

  Atsuko Miyaji, Masaki Nakabayashi, and Shunzou Takano.
  New explicit conditions of elliptic curve traces for FR-reduction.
  IEICE Transactions on Fundamentals of Electronics,
  Communications and Computer Sciences, 84(5):1234–1243, 2001.
* [MOV93]

  A. J. Menezes, T. Okamoto, and S. A. Vanstone.
  Reducing elliptic curve logarithms to logarithms in a finite field.
  IEEE Transactions on Information Theory, 39(5):1639–1646, Sep
  1993.
* [Par15]

  James Parks.
  Amicable pairs and aliquot cycles on average.
  Int. J. Number Theory, 11(6):1751–1790, 2015.
* [Par17]

  James Parks.
  An asymptotic for the average number of amicable pairs for elliptic
  curves.
  Mathematical Proceedings of the Cambridge Philosophical
  Society, page 1–27, 2017.
* [SA98]

  Takakazu Satoh and Kiyomichi Araki.
  Fermat quotients and the polynomial time discrete log algorithm for
  anomalous elliptic curves.
  Commentarii mathematici Universitatis Sancti Pauli,
  47(1):81–92, jun 1998.
* [Sem98]

  I. A. Semaev.
  Evaluation of discrete logarithms in a group of p-torsion points of
  an elliptic curve in characteristic p.
  Math. Comput., 67(221):353–356, January 1998.
* [Sil09]

  Joseph H Silverman.
  The arithmetic of elliptic curves, volume 106.
  Springer Science & Business Media, 2009.
* [Sma99]

  N. P. Smart.
  The discrete logarithm problem on elliptic curves of trace one.
  Journal of Cryptology, 12(3):193–196, Jun 1999.
* [SS11]

  Joseph H Silverman and Katherine E Stange.
  Amicable pairs and aliquot cycles for elliptic curves.
  Experimental Mathematics, 20(3):329–357, 2011.
* [Sut12]

  Andrew V. Sutherland.
  Accelerating the CM method.
  LMS J. Comput. Math., 15:172–204, 2012.
* [The17]

  The Sage Developers.
  SageMath, the Sage Mathematics Software System
  (Version 7.5.1), 2017.
  <http://www.sagemath.org>.
* [Was97]

  Lawrence C Washington.
  Introduction to cyclotomic fields, volume 83.
  Springer Science & Business Media, 1997.

Experimental support, please
[view the build logs](./1803.02067v3/__stdout.txt)
for errors. Generated by
[L
A
T
E
xml
![[LOGO]](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAsAAAAOCAYAAAD5YeaVAAAAAXNSR0IArs4c6QAAAAZiS0dEAP8A/wD/oL2nkwAAAAlwSFlzAAALEwAACxMBAJqcGAAAAAd0SU1FB9wKExQZLWTEaOUAAAAddEVYdENvbW1lbnQAQ3JlYXRlZCB3aXRoIFRoZSBHSU1Q72QlbgAAAdpJREFUKM9tkL+L2nAARz9fPZNCKFapUn8kyI0e4iRHSR1Kb8ng0lJw6FYHFwv2LwhOpcWxTjeUunYqOmqd6hEoRDhtDWdA8ApRYsSUCDHNt5ul13vz4w0vWCgUnnEc975arX6ORqN3VqtVZbfbTQC4uEHANM3jSqXymFI6yWazP2KxWAXAL9zCUa1Wy2tXVxheKA9YNoR8Pt+aTqe4FVVVvz05O6MBhqUIBGk8Hn8HAOVy+T+XLJfLS4ZhTiRJgqIoVBRFIoric47jPnmeB1mW/9rr9ZpSSn3Lsmir1fJZlqWlUonKsvwWwD8ymc/nXwVBeLjf7xEKhdBut9Hr9WgmkyGEkJwsy5eHG5vN5g0AKIoCAEgkEkin0wQAfN9/cXPdheu6P33fBwB4ngcAcByHJpPJl+fn54mD3Gg0NrquXxeLRQAAwzAYj8cwTZPwPH9/sVg8PXweDAauqqr2cDjEer1GJBLBZDJBs9mE4zjwfZ85lAGg2+06hmGgXq+j3+/DsixYlgVN03a9Xu8jgCNCyIegIAgx13Vfd7vdu+FweG8YRkjXdWy329+dTgeSJD3ieZ7RNO0VAXAPwDEAO5VKndi2fWrb9jWl9Esul6PZbDY9Go1OZ7PZ9z/lyuD3OozU2wAAAABJRU5ErkJggg==)](https://math.nist.gov/~BMiller/LaTeXML/).

Instructions for reporting errors
---------------------------------

We are continuing to improve HTML versions of papers, and your feedback helps enhance accessibility and mobile
support. To report errors in the HTML that will help us improve conversion and rendering, choose any of the
methods listed below:

* Click the "Report Issue" () button, located in the page header.

**Tip:** You can select the relevant text first, to include it in your report.

Our team has already identified [the following issues](https://github.com/arXiv/html_feedback/issues). We appreciate your time reviewing and reporting rendering errors we
may not have found yet. Your efforts will help us improve the HTML versions for all readers, because disability
should not be a barrier to accessing research. Thank you for your continued support in championing open access for
all.

Have a free development cycle? Help support accessibility at arXiv! Our collaborators at LaTeXML maintain a [list of packages that need conversion](https://github.com/brucemiller/LaTeXML/wiki/Porting-LaTeX-packages-for-LaTeXML), and welcome [developer contributions](https://github.com/brucemiller/LaTeXML/issues).

We gratefully acknowledge support from
our **major funders**,
[**member institutions**](https://info.arxiv.org/about/ourmembers.html), ,
and all contributors.

[About](https://info.arxiv.org/about)
·
[Help](https://info.arxiv.org/help)
·
[Contact](https://info.arxiv.org/help/contact.html)
·
[Subscribe](https://info.arxiv.org/help/subscribe)
·
[Copyright](https://info.arxiv.org/help/license/index.html)
·
[Privacy](https://info.arxiv.org/help/policies/privacy_policy.html)
·
[Accessibility](https://info.arxiv.org/help/web_accessibility.html)
·
[Operational Status (opens in new tab)](https://status.arxiv.org)

Major funding support from

[![Simons Foundation](/static/base/1.0.1/images/funders/simons-foundation.png)](https://www.simonsfoundation.org/)
[![Simons Foundation International](/static/base/1.0.1/images/funders/simons-foundation-international.png)](https://www.sfi.org.bm/)
[![Schmidt Sciences](/static/base/1.0.1/images/funders/schmidt-sciences.png)](https://www.schmidtsciences.org/)