`deliverables/mil4-k-surge-2026-09-29/sol-static.k` is a specified-only static fragment. It does not discharge M4-C1–C5, it does not admit any of the eight first profiles, and it is not a proof, a U0 exit, or an adoption. The forged-quotient equation inside `m4sCheckedDivMod` is locally coherent. The public static judgment never calls it, and several public success rules accept terms the working decision requires to be rejected.

No `kompile` was run. Compile findings below are equation-shape defects in the pasted file.

## Compile defects

### C1. `m4sCheckedDivMod` has one LHS and four equations

`sol-static.k` lines 262–277. These four rules are the same constructor:

```262:277:deliverables/mil4-k-surge-2026-09-29/sol-static.k
rule m4sCheckedDivMod(_,D,_,_,_,_) => m4sNumericReject("NUMERIC_ZERO_DIVISOR")
  requires D ==Int 0
rule m4sCheckedDivMod(N,D,Q,R,W,_) => m4sNumericReject("NUMERIC_RANGE")
  requires D =/=Int 0 andBool notBool (...)
rule m4sCheckedDivMod(N,D,Q,R,W,_) => m4sNumericReject("NUMERIC_FORGED_QUOTIENT")
  requires ... andBool N =/=Int Q *Int D +Int R
rule m4sCheckedDivMod(N,D,Q,R,W,MODE) => m4sRounded(Q,R,MODE)
  requires ... andBool N ==Int Q *Int D +Int R
```

A K `function` equation is separated from its siblings only by `requires`. The overlap checker does not decide those arithmetic side conditions, so this set is not a well-founded function definition. The same pattern appears at lines 131–134 (`m4sNat`), 201–203 (`m4sNotAtom`), and 280–283 (the two `R >Int 0` ceil rules).

Counterexample of the ordering hazard: `m4sCheckedDivMod(5, 3, 2, 0, 128, m4sExact())`. The range premises hold and `5 =/= 2·3+0`, so the intended result is `NUMERIC_FORGED_QUOTIENT`. An overlap-resolved decision tree that tries the success equation without a proved disjointness check has no structural reason to prefer the forged equation.

Corrected shape: one equation, one classifier.

```k
syntax M4SNumber ::= m4sClassifyDiv(Int, Int, Int, Int, Int, M4SRounding) [function, total]
rule m4sCheckedDivMod(N, D, Q, R, W, MODE) => m4sClassifyDiv(N, D, Q, R, W, MODE)
rule m4sClassifyDiv(_, 0, _, _, _, _) => m4sNumericReject("NUMERIC_ZERO_DIVISOR")
rule m4sClassifyDiv(N, D, Q, R, W, MODE)
  => m4sClassifyQuotient(N, D, Q, R, W, MODE)
  requires D >Int 0 andBool m4sDivRangeOk(N, D, Q, R, W)
rule m4sClassifyDiv(_, D, _, _, _, _) => m4sNumericReject("NUMERIC_RANGE")
  requires D =/=Int 0
```

`m4sClassifyQuotient` then splits on `N ==Int Q *Int D +Int R` versus the complementary case, each with a constructor-level helper rather than a second copy of the same LHS.

### C2. `m4sRounded` success for floor and exact is an unconstrained equation

Lines 278, 280–283, and 284–285. `m4sRounded(Q, 0, m4sCeil())` and `m4sRounded(Q, 0, m4sExact())` match every zero remainder, and `m4sRounded(Q, _, m4sFloor())` matches every floor remainder. Those equations overlap the negative-remainder and out-of-width cases that the final `[owise]` at line 287 is written to catch. Floor is strictly more specific on `m4sFloor()`, so the `[owise]` never sees a bad floor input.

Counterexample: `m4sRounded(2 ^Int 128, 0, m4sExact())`. The `R = 0` exact equation fires and yields `m4sNumber(2 ^Int 128)`. The round-overflow equation at lines 282–283 requires `R >Int 0`, so it does not apply.

Corrected equations, each with a disjoint remainder pattern and a width premise:

```k
rule m4sRounded(Q, 0, m4sFloor()) => m4sNumber(Q) requires m4sUnsigned(Q, 128)
rule m4sRounded(Q, R, m4sFloor()) => m4sNumericReject("NUMERIC_ROUND_INPUT")
  requires R =/=Int 0
rule m4sRounded(Q, 0, m4sExact()) => m4sNumber(Q) requires m4sUnsigned(Q, 128)
rule m4sRounded(_, R, m4sExact()) => m4sNumericReject("NUMERIC_INEXACT")
  requires R >Int 0
rule m4sRounded(Q, R, m4sCeil()) => m4sNumber(Q +Int 1)
  requires R >Int 0 andBool m4sUnsigned(Q +Int 1, 128)
```

## Semantic defects

### S1. Asset admission is optional on the public inference judgment

Lines 135–144 versus 239–241, and lines 72–75. `m4sStatic` requires `m4sEnvValid`. `m4sInfer`, `m4sForm`, `m4sLookup`, and `m4sKnownAsset` are also public `function`s. A quantity literal asks only `m4sKnownAsset`, which is true for a duplicate id, an empty domain, and a negative decimal.

Counterexample. `A = m4sAsset("A", "", 6, m4sNoAssets())`. `m4sAssetTableValid(A)` is false because the domain is empty. `m4sInfer(m4sEnv(A, m4sNoBindings(), m4sNoBindings()), m4sQtyLit("A", 1), m4sGuard(), 3)` reduces by line 136 to `m4sTyped(m4sQty("A"))`.

Corrected: every exported judgment starts from the validated environment, and lookup rechecks the type.

```k
rule m4sInfer(E, T, P, N) => m4sInferValid(E, T, P, N) requires m4sEnvValid(E)
rule m4sInfer(_, _, _, _) => m4sRejected("STATIC_ENV") [owise]
rule m4sLookup(A, m4sBinding(N, T, _), N) => m4sTyped(T) requires m4sTypeValid(A, T)
rule m4sLookup(A, m4sBinding(N, T, _), N) => m4sRejected("STATIC_BINDING_TYPE")
  requires notBool m4sTypeValid(A, T)
```

`m4sStatic` then reports `STATIC_ENV` for a bad table and `STATIC_BOUND` when `N <=Int 0 orBool N >Int 256`, instead of the single `STATIC_ENV_OR_BOUND` at line 241.

### S2. Domain and decimals are checked, then ignored

Lines 53–54, 72–80, and 139–141. M4-C1 requires `Price<Base,Quote,Scale>` to name both nominal assets and domains, and to reject an implicit reciprocal. The reciprocal half holds: there is no invert constructor, and `BASE =/=String QUOTE` is required. Domain and decimals are not read by `m4sTypeValid` for prices, by `m4sQtyLit`, or by any numeric rule. The result type `m4sPrice(B, Q, S)` has nowhere to put the domains that were required at registration.

Counterexample. Register `m4sAsset("SYN", "cardano", 6, m4sAsset("COL", "ethereum", 18, m4sNoAssets()))`. `m4sTypeValid` accepts `m4sPrice("SYN", "COL", 0)`. Scale `0` matches neither decimal, and the two domains are not part of the typed result. The CDP positive case in the working decision needs `Price<Synthetic,Collateral,S> = 1` at a declared scale with compatible decimals. This price literal satisfies the K rule and misses that case.

Corrected price rule:

```k
syntax M4SType ::= m4sPrice(String, String, String, String, Int)
rule m4sTypeValid(A, m4sPrice(B, QB, Q, QQ, S))
  => m4sAssetMeta(A, B, QB, DB) andBool m4sAssetMeta(A, Q, QQ, DQ)
     andBool B =/=String Q andBool S ==Int DQ andBool DB >=Int 0
```

`m4sAssetMeta` returns true only when that id is registered once with that domain and that decimal count. A reciprocal is a different explicit type, never a derived one.

### S3. The type algebra does not match the sorts the file declares

Lines 107–111, 177–179, 83, and 102–104.

`m4sArithmetic` returns true for every operator on `m4sQty`, `m4sDelta`, and `m4sUInt`, including `SUB` and `MUL_LITERAL`. `m4sCombine` then returns that same type only when the two sides are syntactically equal. `m4sShares` is `m4sComparable` and also unconditionally invalid. `m4sInstant` and `m4sDuration` compare and take min/max, and they have no sum.

Counterexamples.

- Loan payment as a delta: `m4sAdd(m4sPre("bal"), m4sPre("paid"))` with `bal : m4sQty("USD")` and `paid : m4sDelta("USD")` reduces by line 179 to `TYPE_MISMATCH`.
- CDP cross-check after a 3/2 requirement: `m4sAtMost(m4sTimesLiteral(m4sQtyLit("SYN", 100), 3), m4sTimesLiteral(m4sQtyLit("COL", 150), 2))` is `TYPE_MISMATCH` because the scaled quantities keep different asset ids. There is no `qty(base) × price(base,quote,s) → qty(quote)` constructor, so the 150-for-100 health comparison cannot be formed in one asset.
- Time: `m4sAdd(m4sPre("now"), m4sPre("period"))` with `m4sInstant("unix")` and `m4sDuration("unix")` is `TYPE_MISMATCH`. A bridge timeout has no well-typed sum, and there is no instant literal.
- Shares bypass: `m4sComparable(m4sShares("S", "U"))` is true at line 102, while line 83 rejects the type in any valid environment. `m4sCompare(m4sTyped(m4sShares("S","U")), m4sTyped(m4sShares("S","U")))` still returns `m4sTyped(m4sProp())` because line 186 only calls `m4sComparable`. The vault pair the comment refuses is a successful comparison on the public `m4sCompare` function.

Corrected combination, with shares refused on the comparison path as well:

```k
rule m4sCombine(m4sTyped(m4sQty(ID)), m4sTyped(m4sDelta(ID)), "ADD")
  => m4sTyped(m4sQty(ID))
rule m4sCombine(m4sTyped(m4sQty(ID)), m4sTyped(m4sQty(ID)), "SUB")
  => m4sTyped(m4sDelta(ID))
rule m4sCombine(m4sTyped(m4sDelta(ID)), m4sTyped(m4sDelta(ID)), OP)
  => m4sTyped(m4sDelta(ID))
  requires OP ==String "ADD" orBool OP ==String "SUB"
rule m4sConvert(m4sTyped(m4sQty(B)), m4sTyped(m4sPrice(B, Q, S)))
  => m4sTyped(m4sQty(Q))
  requires S >=Int 0
rule m4sCombine(m4sTyped(m4sInstant(C)), m4sTyped(m4sDuration(C)), "ADD")
  => m4sTyped(m4sInstant(C))
rule m4sCombine(m4sTyped(m4sDuration(C)), m4sTyped(m4sDuration(C)), "ADD")
  => m4sTyped(m4sDuration(C))
rule m4sComparable(m4sShares(_, _)) => false
rule m4sArithmetic(m4sQty(_), OP) => m4sQtyOp(OP)
```

`m4sQtyOp` is true only for `"ADD"`, `"SUB"`, `"MIN"`, `"MAX"`, and `"MUL_LITERAL"`.

### S4. `m4sNat` erases the nominal asset

Lines 131–132, 107–109, and 186–187. A uint128 literal is a dimensionless `m4sUInt`, and two uints compare. Every profile obligation can be rewritten with `m4sNat` and accepted.

Counterexample. The loan hostile case is “the same debt reduction without the creditor payment.” This guard is accepted:

```k
m4sAnd(m4sEqual(m4sPost("principal"), m4sNat(980)),
       m4sEqual(m4sPost("accrued"), m4sNat(0)))
```

If `principal` and `accrued` are bound as `m4sUInt`, `m4sStatic` returns `m4sTyped(m4sProp())`. No asset id appears. The same hole types the CDP equality `100 * 3` against `150 * 2` as uints, so the 3/2 check goes through with `SYN` and `COL` deleted.

Corrected: a financial phase rejects a bare natural as a term, and threshold counts stay outside terms, where `m4sThreshold`'s `Int` already is.

```k
rule m4sInfer(_, m4sNat(_), m4sGuard(), N) => m4sRejected("TYPE_BARE_NAT")
  requires N >Int 0
rule m4sInfer(_, m4sNat(_), m4sOutcome(), N) => m4sRejected("TYPE_BARE_NAT")
  requires N >Int 0
```

### S5. A typed quantity is not an unsigned 128-bit value

Lines 153–167 and 181–182. Addition, subtraction, min, max, and literal scaling return the operand type and never call `m4sCheckedAdd`, `m4sCheckedSub`, or `m4sCheckedMul`.

Counterexamples.

- `m4sSub(m4sQtyLit("A", 5), m4sQtyLit("A", 9))` infers `m4sTyped(m4sQty("A"))`. `m4sCheckedSub(5, 9)` would reject.
- `m4sAdd(m4sQtyLit("A", (2 ^Int 128) -Int 1), m4sQtyLit("A", 1))` infers `m4sQty("A")`. The sum does not satisfy `m4sUnsigned(_, 128)`.
- `m4sTimesLiteral(m4sQtyLit("A", 2 ^Int 127), 4)` infers `m4sQty("A")` because line 107 treats `"MUL_LITERAL"` as legal and line 182 only range-checks the coefficient `4`.

Corrected scaling and subtraction for literals, with a width on the type so a 256-bit intermediate can be named:

```k
syntax M4SType ::= m4sQtyW(String, Int) | m4sUIntW(Int)
rule m4sScaleType(m4sTyped(m4sQtyW(ID, 128)), K)
  => m4sQtyFromProduct(ID, V, K)
  requires m4sUnsigned(K, 128)
rule m4sCheckedSub(A, B) => m4sNumericReject("NUMERIC_RANGE")
  requires notBool (m4sUnsigned(A, 128) andBool m4sUnsigned(B, 128))
rule m4sCheckedSub(A, B) => m4sNumericReject("NUMERIC_UNDERFLOW")
  requires m4sUnsigned(A, 128) andBool m4sUnsigned(B, 128) andBool A <Int B
```

The existing `[owise]` on subtract calls both a negative input and a shortfall `NUMERIC_UNDERFLOW_OR_RANGE` (line 257). Those are different failures under `failure_p`.

### S6. Exact arithmetic does not interpret formulas, and the 256-bit lane drops its quotient

Lines 168–173 and 245–277. `M4STerm` has no divisor, quotient, or remainder. `m4sOmega` and `m4sVariableProduct` reject before they read their children. `m4sCheckedMul` allows a 256-bit product of two uint128 operands, then `m4sCheckedDivMod` requires the quotient to satisfy `m4sUnsigned(Q, 128)` even when `W ==Int 256`. A 256-bit `m4sNumber` cannot re-enter `m4sNat` or `m4sQtyLit`, because those literals demand width 128. M4-C3 says host computation alone is insufficient and that one feasible result and one otherwise valid forged quotient must reach the arithmetic rule on the pinned target. A direct call can hit `NUMERIC_FORGED_QUOTIENT`. `m4sStatic` cannot.

Counterexamples.

- Feasible AMM case from the working decision: input `11` A, reserves `1000/2200`, fee `3/1000`, floor output `23` B. The term `m4sVariableProduct(m4sQtyLit("A", 11), m4sQtyLit("B", 2200))` reduces by line 172 to `UNSUPPORTED_PHI1`. The inner asset check never runs.
- Forged but otherwise in range: `m4sCheckedDivMod(5, 3, 2, 0, 128, m4sExact())` does reduce to `NUMERIC_FORGED_QUOTIENT`. That call is not a subterm of `m4sForm`.
- Width: `m4sCheckedDivMod(2 ^Int 128, 1, 2 ^Int 128, 0, 256, m4sExact())` meets `N == Q·D+R` and is rejected by lines 264–267 as `NUMERIC_RANGE`, because `2 ^Int 128 <Int 2 ^Int 128` is false. `W = 256` does not widen the quotient.
- Option identity from the working decision: `1500000·(3123456789−3000000000) = 185185183·1000000 + 500000`. Floor is `185185183`. Nothing in `m4sForm` constructs this divmod, and the one-micro-USDC underpayment (`185185182`) is a successful `m4sCheckedSub` of two unrelated integers.

Corrected term form that reaches one classifier:

```k
syntax M4STerm ::= m4sDivMod(M4STerm, M4STerm, M4STerm, M4STerm, Int, M4SRounding)
rule m4sInfer(E, m4sDivMod(N, D, Q, R, W, MODE), P, Fuel)
  => m4sAfterDiv(m4sCheckedDivMod(n, d, q, r, W, MODE), TQ)
  requires Fuel >Int 0 andBool m4sLiteralInt(m4sInfer(E, N, P, Fuel -Int 1), TN, n)
         andBool m4sSameWidth(TN, TD, TQ, W)
```

`m4sAfterDiv` returns `m4sTyped(TQ)` only from `m4sNumber(q)` after the classifier succeeds, and it carries the remainder out as a tagged residue (S7). `m4sOmega("amm-cp/1")` stays rejected until a certificate names this divmod; it does not become a silent host call.

### S7. `m4sReserveUnit` credits a product remainder

Lines 289–294, against the comment immediately above them and M4-C2. The rule returns `m4sNumber(RESERVE)` whenever three uint128 integers add. It does not take an asset, a beneficiary, a whole-unit tag, or a divmod residue.

Counterexample from the vault trace. `500·1999 = 999·1000 + 500`, divisor `1000`, accepted shares `999`, product remainder `500`. `m4sCheckedDivMod(999500, 1000, 999, 500, 128, m4sFloor())` returns `m4sNumber(999)` and drops `500`. `m4sReserveUnit(999500, 999000, 500)` then returns `m4sNumber(500)`. The working decision says that `500` is in product units and is neither a share payment nor an asset credit. The hostile vault witness `Q = 998`, `R = 1500` fails `R <Int D` and is reported as `NUMERIC_RANGE` (lines 264–267), the same code as a bad width or a negative input, so the trace does not reach a remainder rule.

Corrected:

```k
syntax M4SResidue ::= m4sProductRemainder(Int) | m4sWholeUnit(String, Int)
rule m4sReserveUnit(m4sProductRemainder(_), _, _, _)
  => m4sNumericReject("NUMERIC_PRODUCT_RESIDUE")
rule m4sReserveUnit(m4sWholeUnit(ID, RESERVE), TOTAL, ALLOCATED)
  => m4sReserveCredit(ID, "protocol-reserve", RESERVE)
  requires m4sUnsigned(RESERVE, 128) andBool RESERVE >Int 0
         andBool TOTAL ==Int ALLOCATED +Int RESERVE
rule m4sCheckedDivMod(999500, 1000, 998, 1500, 128, _)
  => m4sNumericReject("NUMERIC_NONCANONICAL_REMAINDER")
```

The last equation is the instance; the general rule is `R >=Int D` after the product equation has already matched, reported as `NUMERIC_NONCANONICAL_REMAINDER` rather than `NUMERIC_RANGE`.

### S8. Floor and ceil succeed with no W-D4 policy, beneficiary, or spare effect

Lines 44 and 278–286. M4-C2 leaves per-primitive direction, the reserve cell, and whole-unit identification open. The AMM note says `3/1000` inside the quote yields `23` B, while an upfront one-unit fee yields `21` B, and `30/10000` is an encoding comparison rather than another rate. `m4sFloor()` returns the quotient and discards `R`. There is no rational constructor, no gcd check, and no beneficiary cell.

Counterexample. `m4sCheckedDivMod(10, 4, 2, 2, 128, m4sFloor())` meets `10 == 2·4+2` and returns `m4sNumber(2)`. The residue `2` is gone, so the result cannot name the integer effects M4-C2's discriminator requires. `m4sCeil()` on the same witness returns `3` and also emits no reserve line. Either success selects a rounding policy the working decision says is undisposed.

Corrected: bare rounding is an open premise, and an admitted mode is a named policy with both outputs.

```k
rule m4sRounded(Q, R, m4sFloor()) => m4sNumericReject("OPEN_ROUNDING_POLICY")
  requires m4sUnsigned(Q, 128) andBool R >=Int 0
syntax M4SRounding ::= m4sPolicyFloor(String, String) // policy-id, beneficiary-cell
rule m4sRounded(Q, R, m4sPolicyFloor(PID, BEN))
  => m4sQuotientResidue(PID, BEN, Q, R)
  requires m4sUnsigned(Q, 128) andBool R >=Int 0
```

Until a W-D4 disposition names `PID`, profiles keep the reject rule.

### S9. Open premises and price authority are split in opposite directions

Lines 147–152 and 168–173, against M4-C4 and the `obs/1` and `cdp/1` rows. A hole in either phase looks up `H` and can be typed. `m4sOracle`, `m4sOmega`, and `m4sVariableProduct` reject without reading their payload. A price literal is accepted with no round, epoch, or grant.

Counterexample. Assets `A` and `B` are registered, `H` binds `"p"` at `m4sPrice("A","B",6)`, and the guard is `m4sEqual(m4sHole("p"), m4sPriceLit("A","B",6,1))`. At fuel `4`, `m4sStatic` returns `m4sTyped(m4sProp())`. The observation that the `obs/1` positive case needs, `m4sOracle("selected-round")`, returns `OPEN_ORACLE_PREMISE`. So does the hostile still-fresh nonselected round `m4sOracle("older-round")`. The two `obs/1` traces are the same judgment. Deleting the hole binding changes the result to `UNKNOWN_NAME`, which is a different rule from “resolved footprint missing,” so the M4-C4 discriminator is not what the reject implements.

Corrected:

```k
rule m4sInfer(_, m4sHole(_), _, N) => m4sRejected("STATIC_UNRESOLVED_HOLE")
  requires N >Int 0
rule m4sInfer(_, m4sPriceLit(_, _, _, _), _, N)
  => m4sRejected("UNAUTHENTICATED_PRICE")
  requires N >Int 0
syntax M4STerm ::= m4sSelectedRound(String, Int, String) // feed, epoch, digest
rule m4sInfer(_, m4sOracle(_), _, N) => m4sRejected("OPEN_ORACLE_TUPLE")
  requires N >Int 0
```

`m4sSelectedRound` is the only observation term, and it still needs the verifier premise from M4-C5 before a later rule may return `m4sTyped(m4sPrice(...))`. That later rule is not this file's to add as if a verifier existed.

### S10. Profile hostile traces typecheck, or they have no term

Line 239 reduces a valid environment to `m4sForm`, which only checks types.

Counterexamples that return `m4sTyped(m4sProp())` whenever the named cells are `m4sQty` of one asset:

- AMM hostile output: `m4sEqual(m4sPost("out"), m4sQtyLit("B", 24))` with the trader floor at `20` B. The working decision says `24` B is rejected. Static acceptance has no quotient and no fee-policy id.
- Loan hostile discharge: `m4sAnd(m4sEqual(m4sPost("principal"), m4sQtyLit("USD", 980)), m4sEqual(m4sPost("accrued"), m4sQtyLit("USD", 0)))`. The creditor credit of `30` is absent. Line 186 still returns a proposition.
- Bridge: a source lock of `100` and a destination credit of `99` can be two equalities on two quantity cells. The unexplained unit, the nonreceipt premise, and “refund while destination nonreceipt is unknown” have no constructor. `m4sOracle("dest-nonreceipt")` collapses to the same open reject as a successful delivery.

Corrected static success is a formation result with an empty footprint obligation, not an effect:

```k
syntax M4SCheck ::= m4sFormed(M4SType, M4SFootprint)
syntax M4SFootprint ::= m4sFoot(M4STypes)
rule m4sStatic(E, F, P, N) => m4sFormedOnly(m4sForm(E, F, P, N))
  requires m4sEnvValid(E) andBool N >Int 0 andBool N <=Int 256
rule m4sFormedOnly(m4sTyped(m4sProp())) => m4sRejected("STATIC_NO_EFFECT_LINE")
```

A later effect judgment, absent here, is what may accept a formed formula. That keeps `m4sTyped(m4sProp())` from being read as the loan, AMM, or bridge positive case.

### S11. Named rejection is coarser than the failure the decision asks for

Lines 137–138, 142–144, 226–227, 232–238, and 220–223.

- `STATIC_ASSET_OR_RANGE` covers an unknown id with value `1` and a known id with value `-1`.
- `STATIC_PRICE_OR_RANGE` likewise covers a self-price `m4sPrice("A","A",6)`, a scale of `19`, and a magnitude of `2 ^Int 128`.
- `m4sFormList(E, m4sNoFormulas(), m4sGuard(), 0)` returns `m4sTyped(m4sProp())` at line 226. Fuel `0` still succeeds for the empty list, against the exhaustion comment at line 129.
- `m4sThreshold(2, cons(eq, cons(eq, m4sNoFormulas())))` is `m4sTyped(m4sProp())` when `eq` is well typed. The count is `2` because line 225 counts constructors, so one approval formula is two votes. The `gov/1` row requires a distinct approval set.
- `m4sBoth` keeps the left rejection and drops the right one (line 220). `m4sAnd(m4sEqual(m4sOracle("r"), m4sNat(1)), m4sEqual(m4sQtyLit("NO", 1), m4sQtyLit("NO", 1)))` reports `OPEN_ORACLE_PREMISE` and hides `STATIC_ASSET_OR_RANGE`. Swapping the conjuncts hides the oracle failure instead.

Corrected threshold and asset literal rules:

```k
rule m4sInfer(m4sEnv(A, _, _), m4sQtyLit(ID, V), _, N)
  => m4sRejected("STATIC_UNKNOWN_ASSET")
  requires N >Int 0 andBool notBool m4sKnownAsset(A, ID)
rule m4sInfer(m4sEnv(A, _, _), m4sQtyLit(ID, V), _, N)
  => m4sRejected("STATIC_LITERAL_RANGE")
  requires N >Int 0 andBool m4sKnownAsset(A, ID) andBool notBool m4sUnsigned(V, 128)
rule m4sFormList(_, m4sNoFormulas(), _, N) => m4sRejected("STATIC_DEPTH")
  requires N <=Int 0
rule m4sThresholdCheck(K, FS, m4sTyped(m4sProp())) => m4sTyped(m4sProp())
  requires K >=Int 1 andBool K <=Int m4sDistinctCount(FS)
         andBool m4sDistinctCount(FS) ==Int m4sFormulaCount(FS)
         andBool m4sFormulaCount(FS) <=Int 128
```

`m4sDistinctCount` walks `m4sFormulaCons` and rejects a repeated syntactic child with `TYPE_THRESHOLD_DISTINCT`.

### S12. Cell and hole environments are interchangeable positionally

Line 9 and lines 96–98 and 145–152. `m4sEnv(M4SAssets, M4STypes, M4STypes)` uses the second list for both `m4sPre` and `m4sPost`, and the third for `m4sHole`. `m4sEnvValid` checks the two lists the same way. It does not require the names to be disjoint, and a swap is still a valid environment.

Counterexample. `C` binds `"x"` at `m4sQty("A")` and `H` binds `"x"` at `m4sUInt()`. Both `m4sEnv(A, C, H)` and `m4sEnv(A, H, C)` pass `m4sEnvValid`. In the first, `m4sPre("x")` is a quantity. In the second, `m4sPre("x")` is a uint. The formula text is unchanged.

Corrected:

```k
syntax M4SEnv ::= m4sEnv(m4sAssets(M4SAssets), m4sCells(M4STypes), m4sHoles(M4STypes))
rule m4sEnvValid(m4sEnv(m4sAssets(A), m4sCells(C), m4sHoles(H)))
  => m4sAssetTableValid(A) andBool m4sBindingsValid(A, C)
     andBool m4sBindingsValid(A, H) andBool m4sDisjoint(C, H)
rule m4sInfer(m4sEnv(_, m4sCells(C), _), m4sPre(ID), _, N) => m4sLookup(A, C, ID)
  requires N >Int 0
```

## Unimplemented scope

These are absent from the syntax, not failed equations. Adding them would be a new specified slice, not a patch on a passing gate. The working decision already says W-D0–W-D3, the S0 part of W-D4, M3-D, M3-R, and M3-N stay open, and that a version rename inherits no pass.

- The six obligations `stage_p`, `intent_p`, `effect_p`, `authority_p`, `history_p`, and `failure_p` have no judgment. There is no predecessor head, no replay consumption, and no “unpaid duty selects `failure_p`.”
- Effect lines, derived footprints, conservation, and post-state writes. `m4sPost` reuses the pre cell's type, which is the right static reading of a stable cell, and it still does not compute a post value.
- M4-C5's signature contract: scheme, bytes, digest, verifier locus, public inputs, grant, budget, issuer, epoch, and revocation. `gov/1` “reject execution after grant revocation with unchanged signed bytes” has no term. Threshold is only a typed list.
- Canonical bytes, `/3` versus `/4`, unknown-tag rejection, and migration (M4-C1). Asset identity is raw `String` equality. `"USDC"` and `"USDC "` are distinct nonempty ids.
- Certified Ω for `amm-cp/1`. The file implements the defer-all alternative by `OPEN_OMEGA_CERTIFICATION` and `UNSUPPORTED_PHI1`. It has no selected-program certificate and no exact-rational fee (`0.033` A is not a term; `3/1000` versus `30/10000` cannot be compared).
- S0 AccrualFirst. The split “payment `30` clears accrued `10` and principal `20`, creditor receives `30`” has no waterfall constructor. Originate and accrue are outside this file, as the decision says they are outside S0.
- `cdp/1` ceiling cell, issue grant, quota, haircut, and authenticated health. `loan-coll/1` close factor, seizure, shortfall, and loss waterfall.
- `opt-capped/1` instrument id, premium, selected fixing, exercise, cap as a numeric min, and residual reclaim of `564814817`.
- `obs/1` observation tuple, key epoch, authenticated time, and branch-independent admission. `bridge-pair/1` claim, nullifier, installment, remaining entitlement, representation ratio, partial receipt as a state distinct from nonreceipt, and refund premise. The decision already keeps this profile specified-only when no qualified foreign verifier exists. This K adds no verifier.
- `vault/1` share-issue authority, virtual-offset ownership, donation beneficiary, and custody versus surplus. Withdrawal, rewards, and slash are later profiles in the decision and have no constructors here.
- Source, Core, native, and Preview ledger correspondence. The file header disclaims them. This audit does not rescore the 23 September U0 exit.

The local divmod equation check can stay as a helper under S6's single classifier. It is not, by itself, a profile semantics.