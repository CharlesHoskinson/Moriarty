# Cofactor-torsion attacks on hinted scalar multiplications in SNARK circuits
> Source: https://yelhousni.github.io/publication/preprint/preprint26-1776/
> Collected: 2026-10-26
> Published: Unknown

Cofactor-torsion attacks on hinted scalar multiplications in SNARK circuits | يوسف


Search
======

[يوسف](/)




* [Home](/#about)
* [Work](/#experience)
* [Publications](/#publications)
* [Talks](/#talks)
* [Media](/#media)
* [Blog](/#posts)
* [Contact](/#contact)

Cofactor-torsion attacks on hinted scalar multiplications in SNARK circuits
===========================================================================

[Youssef El Housni](/authors/admin/)

August 2026

[PDF](https://eprint.iacr.org/2026/1776.pdf)

### Abstract

Proving an elliptic curve scalar multiplication [k]P= Q ∈ E(Fp) inside a SNARK is much cheaper when the output Q is hinted by the prover and only verified in-circuit, rather than recomputed. The recent scalar multiplication techniques of Eagen, El Housni, Masson and Piellard (Latincrypt 2025) certify a hinted Q with a short lattice reduction (fraction) decomposition of the scalar and a single group identity, and yield the fastest known in-circuit scalar multiplications. We observe that the soundness of this whole family rests on an implicit prime-order hypothesis= the certifying identity is checked over the full group E(Fp), and only when E(Fp) has trivial cofactor does it force Q into the prime-order subgroup. On cofactor curves the certification is unsound. We give two concrete forgery classes against these hinted gadgets= an any-scalar attack that, for a target scalar fixed by the statement, adapts the decomposition so a small rational torsion point cancels from the identity; and a chosen-scalar attack that vanishes the output-side coefficients modulo a small cofactor prime and solves for the scalar afterwards. Both make the gadget accept Q′ = [k]P + T for a non-zero torsion point T as if it were [k]P. We quantify the reachable torsion in terms of the sub-scalar range bound and validate the attacks on widely deployed curves such as BLS12-381, BN254 and BW6-761. The straightforward fix is to check that Q lies in the subgroup but is expensive. We propose a cheaper one that binds the hinted output through a hinted preimage, with the minimal such constant that suffices against each attack model.

Type

[Preprint](/publication/#3)

Publication

Pre-print

![Avatar](/authors/admin/avatar_hu_856abe718a5f72db.jpg)

##### [Youssef El Housni](https://yelhousni.github.io/)

###### Staff Cryptography Researcher at Consensys (USA)

My research interests include applied cryptography for blockchain applications.

2026 Youssef El Housni ©

##### Cite

×

[Copy](#)
[Download](#)
