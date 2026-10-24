Notes about optimizing emulated pairing (part 2) - HackMD

# Notes about optimizing emulated pairing (part 2)
This note focuses on the techniques used for optimizing the emulated bilinear pairing circuit in [gnark](https://github.com/ConsenSys/gnark) library. This is the part 2 of the [part 1](https://hackmd.io/@ivokub/SyJRV7ye2) by [@ivokub](https://hackmd.io/@ivokub).
:::info
\*Goal:\* Express a pairing on BN254 (and BLS12-381) in a SNARK circuit instantiated on BN254 in as less constraints as possible.
- The pairing curve: BN254 or BLS12-381
- The SNARK curve: BN254
The pairing curve is independent of the choice of the SNARK curve.
:::
#### TL;DR
SNARK curve |pairing curve | 1 pairing (R1CS) | 20 pairings (R1CS) |
|-|-|-|-|
BN254 | BN254 | 1 393 318 | 10 266 245
BN254 | BLS12-381 | 2 088 277 | 14 887 140
#### Motivation
Bilinear pairings have been used in various cryptographic applications and have proven to be a fundamental building block for numerous constructions. In particular, certain Succinct Non-interactive Arguments of Knowledge (SNARKs) have short proofs and fast verification thanks to the use of multi-pairings. This succinctness makes pairing-based SNARKs suitable for proof recursion, where proofs verify other proofs. In such scenarios, it is essential to express multi-pairing computations efficiently as a SNARK arithmetic circuits.
Similar requirements arise in compelling applications such as the verification of Boneh-Lynn-Shacham (BLS) signatures or Kate-Zaverucha-Goldberg (KZG) polynomial commitment opening within a SNARK framework. This opens a wide range of new applications in blockchains. For example, proving efficiently a BLS signatures allows bridging blockchains by proving BLS-based consensus mechanisms.
In Linea, this allows us to support the `ECPAIR` precompile in the zkEVM very efficiently. Any smart contract using pairings to verify BLS signatures, KZG commitments, PlonK or Groth16 proofs or literally any pairing-based protocol would do that seamlessly and efficiently on the Linea zkEVM.
While the literature offers detailed approaches on achieving practical and optimized implementations of pairings in various contexts and target environments, very few have previously addressed the efficient implementation of a pairing as a SNARK arithmetic circuit. In this post we bridge the gap and explain how we achieved \*\*the smallest emulated pairing circuit ever.\*\*
Our academic publication was accepted in the 21st International Conference on Applied Cryptography and Network Security ([ACNS 2023](https://sulab-sever.u-aizu.ac.jp/ACNS2023/program.html)) and presented in Kyoto (Japan) on June, the 20th 2023.
## Background
### SNARK curve vs. pairing curve
Some of the most used SNARKs (e.g. Groth16, PlonK-KZG) use an elliptic curve <font color=red>$E\_1$</font> to instantiate the SNARK construction. This is called, here, the SNARK curve. Independently of the SNARK curve, we define another elliptic curve <font color=blue>$E\_2$</font>. This is called, here, the pairing curve. The curve <font color=blue>$E\_2$</font> can be the same as <font color=red>$E\_1$</font> or completely different.
:::info
In this post, we are interested in proving operations on <font color=blue>$E\_2$</font> in a SNARK instantiated with <font color=red>$E\_1$</font>. In particular, we want to prove a pairing on <font color=blue>$E\_2$ : BN254</font> (or <font color=blue>$E\_2:$ BLS12-381</font>) in a Groth16 SNARK instantiated on <font color=red>$E\_1$ : BN254</font>.
:::
#### BLS12-381
BLS12-381 is a pairing-friendly elliptic curve from the family Barreto-Lynn-Scott with an embedding degree $k=12$ (Construction 6.6 in [[FST06]](https://eprint.iacr.org/2006/372.pdf), case $k\equiv 0 \mod6$ or originally [[BLS02]](https://eprint.iacr.org/2002/088.pdf)). It is defined over a 381-bit field $\mathbb{F}\_q$ and is defined by the equation $Y^2=X^3+4$ and its sextic twist by $Y^2=X^3+4u$ over $\mathbb{F}\_{q^2}[u]=\mathbb{F}\_q/(u^2+1)$. The curve parameters polynomials evaluated at the seed $u=-15132376222941642752$.
Parameter | Polynomial | Value | 2-adicity|
--------- | ---------- | ----- | --------- |
field size $q$ | $(x-1)^2/3\cdot r(x)+x$ | $\texttt{0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab}$ | 1
subgroup order $r$ | $x^4-x^2+1$ | $\texttt{0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001}$ | 32
Besides the sizes of $q$, $r$ and the the low hamming-weight of $u$, which makes the pairing-friendly curve efficient and secure at almost 128-bit level, the high 2-adicity of $r-1$ makes it a fit for SNARK applications.
#### BN254
BN254 is a pairing-friendly elliptic curve from the family Barreto-Naehrig with an embedding degree $k=12$ (Construction 6.2 in [[FST06]](https://eprint.iacr.org/2006/372.pdf), example 6.8 or originally [[BN05]](https://eprint.iacr.org/2005/133.pdf)). It is defined over a 254-bit field $\mathbb{F}\_q$ and is defined by the equation $Y^2=X^3+3$ and its sextic twist by $Y^2=X^3+3/(9+u)$ over $\mathbb{F}\_{q^2}[u]=\mathbb{F}\_q/(u^2+1)$. The curve parameters are polynomials evaluated at the seed $u=4965661367192848881$.
Parameter | Polynomial | Value | 2-adicity|
--------- | ---------- | ----- | --------- |
field size $q$ | $36x^4+36x^3+24x^2+6x+1$ | $\texttt{0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47}$ | 1
subgroup order $r$ | $36x^4+36x^3+18x^2+6x+1$ | $\texttt{0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001}$ | 28
Besides the sizes of $q$, $r$ and the the low hamming-weight of $u$, which makes the pairing-friendly curve efficient and secure at almost 128-bit level, the high 2-adicity of $r-1$ makes it a fit for SNARK applications.
### SNARK-friendly computations
Many SNARK constructions write computations to prove as circuit where the variables are in $\mathbb{F}$, a field where the discrete logarithm is hard. In pairing-based SNARKs the field is chosen to be $\mathbb{F}\_r$, where $r$ is the prime subgroup order on the curve. The size of these variables and particularly the multiplication gates variables is what determines the prover complexity. For example, Groth16 prover complexity is dominated by the multi-scalar-multiplications (in $\mathbb{G}\_1$ and $\mathbb{G}\_2$) of sizes $n$ (the number of multiplication gates). With this in mind, additions and constant-scalar multiplications in $\mathbb{F}\_r$, which are usually expensive in hardware, are essentially free. While more traditional hardware-friendly computations (e.g. XORing 32-bit numbers) are far more costly in SNARKs. The following two observations are the key to lower the number of multiplication gates of a SNARK circuit:
- Additions and multiplications by constants in $\mathbb{F}\_r$ are free and
- the verification can be sometimes simpler than forward computation. The SNARK circuits do not always have to compute the result, but can instead represent a verification algorithm. For example a multiplicative inversion circuit $1/x \stackrel{?}{=} y$ does not have to encode the computation of the inversion $1/x$ but can instead consist of a single multiplication constraint $x · y$ on the value provided (precomputed) by the prover $y$ and the equality check $x · y \stackrel{?}{=} 1$.
This is basically a computation model where inversions cost (almost) as much as multiplications. For the sequel we are interested in implementing efficiently pairings in the R1CS model.
### Bilinear pairings
We briefly recall elementary definitions of pairings and present the computation of two pairings used in practice, the Tate and ate pairings. All elliptic curves discussed below are ordinary (i.e. non-supersingular).
- Let $E$ be an elliptic curve defined over a field $\mathbb{F}\_p$, where $p$ is a prime.
- Let $\pi\_p$ be the Frobenius endomorphism: $(x,y) \mapsto (x^p,y^p)$. Its minimal polynomial is $X^2 - tX + p$ where $t$ is called the $\textit{trace}$.
- Let $r$ be a prime divisor of the curve order $\#E(\mathbb{F}\_p) = p + 1 - t$. The $r$-torsion subgroup of $E$ is denoted $E[r] := \{P \in E(\overline{\mathbb{F}\_p}), [r]P = \mathcal{O}\}$ and has two subgroups of order $r$ (eigenspaces of $\pi\_p$ in $E[r]$) that are useful for pairing applications.
- We define the two groups $\mathbb{G}\_1 = E[r] \cap \ker(\pi\_p - [1])$ with a generator denoted by $G\_1$, and
- $\mathbb{G}\_2 = E[r] \cap \ker(\pi\_p - [p])$ with a generator $G\_2$. The group $\mathbb{G}\_2$ is defined over $\mathbb{F}\_{p^k}$, where the embedding degree $k$ is the smallest integer $k \in \mathbb{N}^\*$ such that $r\mid p^k-1$.
We recall the Tate and ate pairing definitions, based on the same two steps:
- evaluating a function $f\_{s,Q}$ at a point $P$, the Miller loop step, and then
- raising it to the power $(p^k-1)/r$, the final exponentiation step.
#### The Miller loop step
The function $f\_{s,Q}$ has divisor $\text{div}(f\_{s,Q}) = s(Q) - ([s]Q) - (s - 1)(\mathcal{O})$ and
satisfies, for integers $i$ and $j$,
\begin{align\*}
f\_{i+j,Q} = f\_{i,Q}f\_{j,Q}\frac{\ell\_{[i]Q,[j]Q}}{v\_{[i+j]Q}},
\end{align\*}
where $\ell\_{[i]Q,[j]Q}$ and $v\_{[i+j]Q}$ are the two lines needed to compute $[i + j]Q$ from $[i]Q$ and $[j]Q$ ($\ell$ intersecting the two points and $v$ the vertical). We compute $f\_{s,Q}(P)$ with the Miller loop presented in Algorithm 1.
![](https://hackmd.io/\_uploads/BJmHheCIh.png)
The Tate and ate pairings are defined by
\begin{align\*}
Tate(P,Q) &:= f\_{r,P}(Q)^{(p^k-1)/r} \\
ate(P,Q) &:= f\_{t-1,Q}(P)^{(p^k-1)/r}
\end{align\*}
where $P \in \mathbb{G}\_1$ and $Q \in \mathbb{G}\_2$. Usually $t-1$ is smaller than $r$ making the Miller loop shorter. In the sequel, we will focus on the ate pairing for BN and BLS curves.
:::success
\*Well-known optimization [[BKLS02]](https://link.springer.com/chapter/10.1007/3-540-45708-9\_23):\* The final exponentiation kills any element which lives in a strict sub-field of $\mathbb{F}\_{p^k}$. In case the embedding degree $k$ is even, the vertical lines $v\_{S+Q}(P)$ and $v\_{[2]S}(P)$ live in a strict sub-field of $\mathbb{F}\_{p^k}$ so these factors will be eliminated by the final exponentiation. Hence, in this situation we ignore the $\texttt{VerticalLine}$ steps and remove the divisions by $v$ in $\texttt{Update1}$ and $\texttt{Update2}$ steps.
:::
It is also important to recall some results with respect to the complex multiplication (CM) equation $4p=t^2+Dy^2$ with discriminant $-D$ and some integer $y$.
- When $-D=-3$, the curve has CM by $\mathbb{Q}(\omega)$ where $\omega^2+\omega+1=0$. In this case, a twist of degree 6 exists. It has a $j$-invariant $0$ and is of the form $Y^2=X^3+b \, (a=0)$. This is the case of both BN254 and BLS12-381.
- When $E$ has $d$-th order twist for some $d\mid k$, then $\mathbb{G}\_2$ is isomorphic to $E'[r](\mathbb{F}\_{p^{k/d}})$ for some twist $E'$ in this case of the form $Y^2=X^3+b'$. We denote by $\psi$ the twisting isomorphism from $E'$ to $E$.
- When $-D=-3$, there are actually two sextic twists, one with $p + 1 - (-3y + t)/2$ points on it, the other with $p + 1 - (3y + t)/2$, where $y = \sqrt{(4p - t^2)/3}$. Only one of these is the "right" twist, \*i.e.\* has an order divisible by $r$. Let $\nu$ be a quadratic and cubic non-residue in $\mathbb{F}\_{p^{k/d}}$ and $X^6-\nu$ an irreducible polynomial, the "right" twist is either with $b'=b/\nu$ (D-type twist) or $b'=b\nu$ (M-type twist). For the D-type (e.g. BN54), $\psi: E' \rightarrow E : (x,y) \mapsto (\nu^{1/3}x, \nu^{1/2}y)$. For the M-type (e.g. BLS12-381), $\psi: E' \rightarrow E : (x,y) \mapsto (\nu^{2/3}x/\nu, \nu^{1/2}y/\nu)$. For other $d$-twisting $\psi$ formulas, see ["A note on twists for pairing friendly curves"](http://indigo.ie/~mscott/twists.pdf) by Mike Scott.
:::success
\*Optimal ate:\* For BLS12, the optimal (shortest $s$) Miller function is $f\_{x\_0,Q}$ where $x\_0$ is the curve seed (\*i.e.\* the integer in which the curve polynomials $p(x), r(x)$ are evaluated to have the primes $p,r$). For BN, the optimal Miller function is $f\_{6x₀+2,Q} · ℓ\_{[6x₀+2]Q,π(Q)} \cdot ℓ\_{[6x₀+2]Q+π(Q),-π²(Q)}$.
:::
#### The final exponentiation step
An exponentiation in $\mathbb{F}\_{p^k}$ to the fixed $(p^k-1)/r$ exponent is necessary to ensure the output uniqueness of the ate (and Tate) pairings. Many works have tried to speed this computation up by applying vectorial addition chains or lattice-based reduction approaches. It is usually divided into an easy part and a hard part, as follows:
\begin{align}
\frac{p^k-1}{r} &= \underbrace{\frac{p^k-1}{\Phi\_k(p)}}\_{\mbox{easy part}}
\cdot \underbrace{\frac{\Phi\_k(p)}{r}}\_{\mbox{hard part}} \\ \nonumber
&= \underbrace{(p^d-1)\frac{\sum\_{i=0}^{e-1}p^{id}}{\Phi\_k(p)}}\_{\mbox{easy
part}} \cdot \underbrace{\frac{\Phi\_k(p)}{r}}\_{\mbox{hard part}}
\end{align}
where $\Phi\_k$ is the $k$-th cyclotomic polynomial and $k=d\cdot e$. For BLS and BN curves, the easy part is $(p^{k/2}-1)(p^{k/d}+1)$. It is made of Frobenius powers, two multiplications and a single inversion in $\mathbb{F}\_{p^k}$.
:::success
\*SotA implementations:\* The most efficient algorithms for the hard part stem from [[HayHayTer20]]() for BLS12 and [[DuqGha15]](https://eprint.iacr.org/2015/192) for BN.
:::
\*\*TL;DR\*\*
![](https://i.imgur.com/vqVDZAB.jpg)
The map $e$ takes curve points $P, aP \in \mathbb{G}\_1$ (<font color=blue> — </font>) and $Q, bQ \in \mathbb{G}\_2$ (<font color=blue> - - </font>) and sends them to line $\mathbb{G}\_T$ (<font color=green> — </font>). Just keep in mind that:
- $e(aP,bQ)=e(P,Q)^{ab}$ and
- $e(P,Q) \ne 1\_{\mathbb{G}\_T}$ for all $P,Q$
This map is a 2-step computation:
- first, a Miller loop:
```go=
// f\_{s,q}(p)
func MillerLoop(p G1, q G2) GT {
var result GT
result.SetOne()
var l1, l2 line
var prodLines [5]Fp2
for i := len(loopCounter) - 2; i >= 0; i-- {
// f²
result.Square(&result)
// q ← 2\*q and l1 the tangent ℓ passing 2\*q
q.doubleStep(&l1)
// line evaluation at P
l1.r1.Mul(&l1.r1, &p.X)
l1.r2.Mul(&l1.r2, &p.Y)
if loopCounter[i] == 0 {
// ℓ × res (sparse-dense mul)
result.MulByLine(&l1.r0, &l1.r1, &l1.r2)
} else {
// q ← q+Q and
// l2 the line ℓ passing q and Q
q.addStep(&l2, &q)
// line evaluation at P
l2.r1.Mul(&l2.r1, &p.X)
l2.r2.Mul(&l2.r2, &p.Y)
// ℓ × ℓ (sparse-sparse mul)
prodLines = fptower.MulLineByLine(&l2.r0, &l2.r1, &l2.r2, &l1.r0, &l1.r1, &l1.r2)
// (ℓ × ℓ) × result (sparse-somewhatsparse mul)
result.MulByProdLines(&prodLines)
}
}
return result
}
```
- second, a final exponentiation:
```go=
// zᵈ where d = (p¹²-1)/r = (p¹²-1)/Φ₁₂(p) ⋅ Φ₁₂(p)/r
// = (p⁶-1)(p²+1)(p⁴ - p² +1)/r
func FinalExponentiation(z GT) GT {
var result GT
result.Set(z)
var t [3]GT
// Easy part
// (p⁶-1)(p²+1)
t[0].Conjugate(&result)
result.Inverse(&result)
t[0].Mul(&t[0], &result)
result.FrobeniusSquare(&t[0]).
Mul(&result, &t[0])
// Hard part (p⁴ - p² +1)/r
// optimized depending on the curve
// e.g. [HayHayTer20] for BLS12 and [DuqGha15] for BN
// ...
return result
}
```
:::success
The bottleneck of the hard part for both BN254 and BLS12-381 is an exponentiation to the constant $x\_0$. The routine of this exponentiation can be hard-coded via a short addition chain. We use [mmcloughlin/addchain](https://github.com/mmcloughlin/addchain).
:::
## In-circuit optimizations
To compute a pairing in no time flat, we implement in [gnark-crypto](https://github.com/ConsenSys/gnark-crypto) (out-circuit) a collection of state-of-the-art techniques from the literature and some more from our R&D team. However, porting this implementation to [gnark](https://github.com/ConsenSys/gnark-crypto) (in-circuit) would result in billions of constraints, which would make the proof generation impractical.
:::info
In the following we show that a BN254 pairing in a BN254 Groth16 is only \*\*1.3M\*\* constraints and BLS12-381 pairing in a BN254 Groth16 is only \*\*2M\*\* constraints. The difference being mostly related to emulating a bigger field in the latter case.
:::
### Miller loop optimizations
Since inversions cost almost as much as multiplications in R1CS, it is better to use affine coordinates in the Miller loop. For division, instead of computing an inversion and then a multiplication as it is custom, one would compute directly the division in R1CS. A squaring costs as much as a multiplication over $\mathbb{F}\_p$.
The same observations work over extension fields $\mathbb{F}\_{p^e}$ except for squaring where the Karatsuba technique can be specialized.
Point doubling and addition in affine coordinates is as follows:
$[2](x\_S, y\_S) = (x\_{[2]S}, y\_{[2]S})$
\begin{align\*}
\lambda &= 3x\_S^2/2y\_S \\
x\_{[2]S} &= \lambda^2 - 2x\_S \\
y\_{[2]S} &= \lambda(x\_S - x\_{[2]S}) - y\_S
\end{align\*}
$(x\_S, y\_S)+(x\_Q, y\_Q) = (x\_{S+Q}, y\_{S+Q})$
\begin{align\*}
\lambda &= (y\_S-y\_Q)/(x\_S - x\_Q) \\
x\_{S+Q} &= \lambda^2 - x\_S - x\_Q \\
y\_{S+Q} &= \lambda(x\_Q - x\_{S+Q}) - y\_Q
\end{align\*}
For BLS12 and BN curves, $\mathbb{G}\_2$ coordinates are over $\mathbb{F}\_{p^2}$. Note that a doubling is more costly in R1CS than an addition because the tangent slope $\lambda$ requires a squaring and a division instead of just a division. However, it turns out we can do better: when the `loopCounter` bit is 1, a doubling and an addition $[2]S+Q$ is computed but instead we can compute $(S+Q)+S$ trading off a doubling for an addition. Moreover, we can omit the computation of the $y$-coordinate of $S+Q$ as pointed out in a different context in [[EisLauMon03]](https://arxiv.org/abs/math/0208038).
$[2](x\_S, y\_S)+(x\_Q, y\_Q) = (x\_{(S+Q)+S}, y\_{(S+Q)+S})$
\begin{align\*}
\lambda\_1 &= (y\_S-y\_Q)/(x\_S - x\_Q) \\
x\_{S+Q} &= \lambda\_1^2 - x\_S - x\_Q \\
\lambda\_2 &= -\lambda\_1 - 2y\_S / (x\_{S+Q}-x\_S) \\
x\_{(S+Q)+S} &= \lambda\_2^2 - x\_S - x\_{S+Q} \\
y\_{(S+Q)+S} &= \lambda\_2(x\_S - x\_{(S+Q)+S}) - y\_S
\end{align\*}
:::info
\*Note:\* The use of affine coordinates and the merged double-and-add trick is widely known in R1CS optimization.
:::
:::info
\*Note:\* Sometimes the `loopCounter` can have a lower Hamming weight in the signed representation $\{-1,0,1\}$ (2-NAF). We pre-compute the negative point once (for free) and use this representation as subtraction cost the same as addition in R1CS. This is the case of BN254 but not BLS12-381.
:::
The most interesting observation in the Miller loop optimization is in the lines evaluation. A line $\ell$ in $\mathbb{F}\_{p^2}$ is of the form $ay+bx+c=0$. In the Miller loop, we need to compute the lines that go through the untwisted $\mathbb{G}\_2$ points $[2]S$ and $S+Q$ and to evaluate them at $P \in \mathbb{G}\_1$. That is, $\ell\_{\psi([2]S)}(P)$ and $\ell\_{\psi(S+Q)}(P)$ where $\psi: E'(\mathbb{F}\_{p^2}) \rightarrow E(\mathbb{F}\_{p^{12}})$ is the untwisting isomorphism. Both lines are sparse elements in $\mathbb{F}\_{p^{12}}$. In R1CS, we precompute $1/y\_P$ and $x\_P/y\_P$ once and represent the lines by $1 + b' x\_P/y\_P + c'/y\_P$. This does not change the final result because $1/(a\cdot y\_P)$ is in a proper sub-field of $\mathbb{F}\_{p^k}$ and hence will be killed later by the final exponentiation (similar to $\texttt{VerticalLine}$ previously). This makes the specialized multiplication by lines even more sparser and hence saves lots of constraints at each iteration.
Another optimization, from [[Scott19]](https://eprint.iacr.org/2019/077.pdf), is to multiply the lines together before multiplying them by the accumulator. This happens whenever the `loopCounter` bit is 1 but also at each iteration for Multi-Miller loops. We first store the lines and then multiply them 2-by-2 in a second loop fully exploiting any sparsity in the multiplicands. Constraint-wise we implement 3 functions:
- `MulByLine` (sparse-dense mul)
- `MulLineByLine` (sparse-sparse mul)
- `MulByProdLines` (sparse-somewhatsparse mul)
All of these functions are specialized multiplications in $\mathbb{F}\_{p^{12}}$ that cost way less constraints than a plain multiplication in $\mathbb{F}\_{p^{12}}$. The way we represent the lines makes these functions even more optimized since multiplications by $1$ can be discarded.
```go=
for i := len(loopCounter) - 2; i >= 0; i-- {
// mutualize the square among n Miller loops
// (∏ᵢfᵢ)²
res = pr.Square(res)
switch loopCounter[i] {
case 0:
// precompute lines
for k := 0; k < n; k++ {
// Qacc[k] ← 2Qacc[k] and l1 the tangent ℓ passing 2Qacc[k]
Qacc[k], l1[k] = pr.doubleStep(Qacc[k])
// line evaluation at P[k]
l1[k].R0 = \*pr.Mul(&l1[k].R0, xOverY[k])
l1[k].R1 = \*pr.Mul(&l1[k].R1, yInv[k])
}
// if number of lines is odd, mul last line by res
// works for n=1 as well
if n%2 != 0 {
// ℓ × res
res = pr.MulByLine(res, &l1[n-1].R0, &l1[n-1].R1)
}
// mul lines 2-by-2
for k := 1; k < n; k += 2 {
// ℓ × ℓ
prodLines = \*pr.MulLineByLine(&l1[k].R0, &l1[k].R1, &l1[k-1].R0, &l1[k-1].R1)
// (ℓ × ℓ) × res
res = pr.MulByProdLines(res, &prodLines)
}
// ...
```
One last constraint-saving trick is to isolate the first and last iterations (and sometime the second iteration too). The rationale is avoid some unnecessary computations or to "guess" the deterministic result.
- First iteration:
- since we initialize the accumulator by 1 there is no need to square it in the first iteration.
- Also we can just assign the line to the accumulator — the multiplication by 1 is useless.
- In the case of 2 or 3 pairings. We can use `MulLineByLine` and `MulByProdLines` because the accumulator at this point is just the assigned sparse line.
- Second iteration:
- When we use 2-NAF, it might happen that the second bit in the loop is $-1$ e.g. BN254. In this case, at this point, the accumulated point is $S=2Q$, so $2S-Q=3Q$ is equivalent to $S+Q=3Q$. We trade off a `doubleAndAddStep` by just an `addStep`.
- Last iteration:
- At each iteration we compute an accumulated point and an accumulated value in $\mathbb{F}\_{p^{12}}$ but at the end of the Miller loop we only return the accumulated value. Thus, at the last step, we can omit the point arithmetic.
### Final exponentiation optimizations
The final exponentiation for BN and BLS12 curves take place in $\mathbb{F}\_{p^{12}}$ \*i.e.\* $z \mapsto z^{(p^{12}-1)/r}$. It can be seen as a 2-step computation:
- The easy part: $z \mapsto z^{(p^6-1)(p^6+1)}$, which is just a conjugate ($p^6$-Frobenius map), a multiplication and an inverse.
- The hard part: $z \mapsto z^{(p^4-p^2+1)/r}$ which can be developed efficiently using LLL decomposition. It boils down to Frobenius maps and exponentiations by the seed $x\_0$.
The most important optimization we did was to write the whole final exponentiation in $\mathbb{F}\_{p^6}$ instead of $\mathbb{F}\_{p^{12}}$. In fact, after the easy part the element $z$ belongs to the algebraic torus $\mathbb{T}\_{12}(\mathbb{F}\_p)$ and thus to each torus $\mathbb{T}\_{k/d}(\mathbb{F}\_{p^d})$ for $d\mid k$, $d\ne k$ — hence $z \in \mathbb{T}\_{2}(\mathbb{F}\_{p^6})$.
:::info
\*Definition:\*
$$ T\_k(\mathbb{F}\_p) = \{ \alpha \in \mathbb{F}\_{p^k} | \alpha^{(p^k-1)/(p^d-1)} = 1 \}~\text{and}~|T\_k(\mathbb{F}\_p)| = \Phi\_k(p)~.$$
:::
Torus-based cryptography was introduced in [[Rubin-Silververg02]](https://www.math.uci.edu/~asilverb/bibliography/ceilidh.pdf) but was never really used because of its inefficiency. However, in SNARKs, it makes a great fit since inverses are cheap. The typical workflow is:
- 1. Compress elements after the easy part into $\mathbb{F}\_{p^6}$;
- 2. Perform multiplications, squarings and Frobenius maps on the compressed elements;
- 3. Decompress the result back to $\mathbb{F}\_{p^{12}}$
We implement in-circuit the following functions:
\begin{align\*}
\texttt{Compress}&\quad \zeta: ~ \mathbb{F}\_{p^{12}}\setminus\{-1,1\} \rightarrow \mathbb{F}^\*\_{p^6} \\
&m \mapsto \frac{1+m\_0}{m\_1} = g ;\\
\texttt{Decompress}&\quad \zeta^{-1}: ~ \mathbb{F}^\*\_{p^6} \rightarrow \mathbb{F}\_{p^{12}}\setminus\{-1,1\} \\
&g \mapsto \frac{g+w}{g-w}~;\\
\texttt{Multiply}& \quad (g,g') \mapsto \frac{g \cdot g' + \gamma }{g + g'}~;\\
\texttt{Inverse}& \quad g \mapsto -g~.\\
\end{align\*}
We also figure out circuit-efficient computations for squaring and Frobenius maps using hints:
\begin{align\*}
\texttt{Square}& \quad g \mapsto \tfrac{1}{2}(g+\gamma/g) ;\\
\texttt{Frobenius map}& \quad g \mapsto \frac{g^{p^i}}{\gamma^{(p^i-1)/2}}~.
\end{align\*}
The 2 problems we have now are the costs of compression and decompression and the unhandled edge cases $\{-1,1\}$.
We absorb the compression in the easy part, i.e. merge the two at the cost of one since:
\begin{align\*}
m^{p^6-1} &= (m\_0 + w m\_1)^{p^6-1} \\
&= (m\_0 + w m\_1)^{p^6} / (m\_0 + w m\_1) \\
&= (m\_0 - w m\_1) / (m\_0 + w m\_1) \\
&= (-m\_0/m\_1 + w) / (-m\_0/m\_1 - w) \\
&= \zeta^{-1}(-m\_0/m\_1)
\end{align\*}
So we merge the two as:
\begin{align\*}
\zeta(m^{(p^6-1)(p^2+1)}) &= (-m\_0/m\_1)^{p^2+1} \\
&= (-m\_0/m\_1)^{p^2} \cdot (-m\_0/m\_1)\\
\end{align\*}
Which is actually
\begin{align\*}
\texttt{Multiply} \{\texttt{Frobenius map}(-m\_0/m\_1),\,(-m\_0/m\_1)\}
\end{align\*}
For the edge-cases, the Miller loop result is $\ne \{-1,1\}$, otherwise this means that the points are linearly dependant and not from $\mathbb{G}\_1$ and $\mathbb{G}\_2$ respectively. However, for Multi-pairings this might happen. What we do in-circuit is basically: assign a dummy value that does not get the circuit to fail, keep a selector and proceed further. At the end, if the selector is 1 we return 1.
### Fixed-argument pairings
A common scenario in many pairing-based cryptographic protocols is that one argument in the pairing is fixed as a constant parameter in the system. This is for example the case of KZG polynomial commitment. For verifying an opening proof at a single point:
$$ e([f(α)-f(a)+aH(α)]G\_1], \underbrace{G\_2}\_{fixed}).e([-H(α)]G\_1, \underbrace{[α]G\_2}\_{fixed}) \stackrel{?}{=} 1 $$
and for batch verifying a list of opening proofs at different points (as used in PlonK):
$$ e([\sum\_i \lambda\_i(f\_i(α) - f\_i(p\_i) + p\_iH\_i(α))]G\_1, \underbrace{G\_2}\_{fixed}).e([-\sum\_i\lambda\_i[H\_i(α)]G\_1), \underbrace{[α]G\_2}\_{fixed})\stackrel{?}{=}1~.$$
In both cases, we can spot that $G\_2$ and $[\alpha]G\_2$ are fixed points included in the fixed verification key (SRS).
In these situations, the number of constraints in the Miller algorithm can be significantly reduced by storing precomputed values that depend on the fixed argument, prior to the input or existence of the second argument.
We focus on the scenario where the argument $Q \in \mathbb{G}\_2$ is fixed in the case of the ate pairing. This also happens for the BLS signature verification of the minimal-signature-size variant:
$$ e(\sigma, \underbrace{G\_2}\_{fixed})=e(H(m), \texttt{pk}) $$
In this case, we can pre-compute all the lines and only evaluate them $P \in \mathbb{G\_1}$ in the Miller loop. This way we can omit all the $\mathbb{G\_2}$ arithmetic and line computation in-circuit and only square the accumulator and multiply it by the sparse pre-computed lines.
```go=
for i := len(loopCounter) - 2; i >= 0; i-- {
res = pr.Square(res)
if loopCounter[i] == 0 {
// line evaluation at P and ℓ × res
res = pr.MulByLine(res,
pr.Mul(&pr.precompLines[0][i], xOverY),
pr.Mul(&pr.precompLines[1][i], yInv),
)
} else {
// lines evaluations at P
// and ℓ × ℓ
prodLines := \*pr.MulLineByLine(
pr.Mul(&pr.precomLines[0][i], xOverY),
pr.Mul(&pr.precomLines[1][i], yInv),
pr.Mul(&pr.precomLines[2][i], xOverY),
pr.Mul(&pr.precomLines[3][i], yInv),
)
// (ℓ × ℓ) × res
res = pr.MulByProdLines(res, &prodLines)
}
}
```
### Benchmarks
Given a Groth16 SNARK instantiated with BN254, the number of R1CS constraints is:
|curve | 1 pairing | 2 pairings | 4 pairings | 9 pairings | 20 pairings |
|-|-|-|-|-|-|
BN254 | 1 393 318 | 1 872 448 | 2 812 614 | 5 163 662 | 10 266 245
BLS12-381 | 2 088 277 | 2 682 342 | 3 895 167 | 7 184 423 | 14 887 140
In more details, a single pairing cost is:
|curve | Miller loop | Final exponentiation (unsafe) |
|-|-|-|
BN254 | 706 406 | 686 912
BLS12-381 | 898 237 | 1 190 040
For multi-pairings, where the edge-case $\{-1,1\}$ might happen, the final exponentiation (safe) costs:
|curve | Final exponentiation (safe) |
|-|-|
BN254 | 699 729
BLS12-381 | 1 209 247
For a fixed argument pairing where $Q = G\_2$ the canonical generator of $\mathbb{G}\_2$:
|curve | 1 pairing with fixed $Q\in \mathbb{G}\_2$ |
|-|-|
BN254 | 1 218 765
BLS12-381 | 1 868 541
----
#### Useful links
- Linea zkEVM: https://linea.build/
- gnark GitHub: https://github.com/ConsenSys/gnark
- gnark-crypto GitHub: https://github.com/ConsenSys/gnark-crypto
- Academic paper: https://eprint.iacr.org/2022/1162
<style>
r { color: Red }
o { color: Orange }
g { color: Green }
</style>

×

### Sign in

Email

Password




[Forgot password](https://hackmd.io/settings/forgotPassword)

or

[![](/social/google.svg)
Sign in via Google](https://hackmd.io/auth/google)
[![](/social/facebook.svg)
Sign in via Facebook](https://hackmd.io/auth/facebook)
[![](/social/x.svg)
Sign in via X(Twitter)](https://hackmd.io/auth/twitter)
[![](/social/github.svg)
Sign in via GitHub](https://hackmd.io/auth/github)
[![](/social/dropbox.svg)
Sign in via Dropbox](https://hackmd.io/auth/dropbox)
[![](https://hackmd.io/images/wallet.svg)
Sign in with Wallet

Wallet
(
)](#)

Connect another wallet

Continue with a different method

New to HackMD? [Sign up](https://hackmd.io/join)

By signing in, you agree to our [terms of service](https://hackmd.io/s/terms).