# Target-required financial arithmetic — proposed successor extension

This is a new SP01 type/overload proposal, not a change to the accepted scope of
expression candidate02. It owns the missing arithmetic required by
[the original AMM fragment](../defi-language-design-2026-09-07/LANGUAGE-DESIGN.md:24).
That fragment remains a target; TYPE_MISMATCH in the current expression table
does not remove it. Financial operation formulas and source expressions must
use the same exact directed rounding and bounded intermediates.

## MA-D1: Explicit finite intermediate types, not unit erasure

Keep Amount<A> unsigned128 asset quanta and Price<B,A,S> as B-per-A at scale S.
Keep the current prohibition on plain division. Use the existing named Mul,
FloorDiv and CeilDiv constructor names with **new versioned overloads** below.
No expression becomes a financial transfer or claim creation by using them.
Each intermediate is a finite pure value, never spendable tokens:

- `AmountProduct<A,B>`: nonnegative m <2^256, dimensional product A×B, with
  asset parameters in increasing identifier order. Equal parameters represent
  A², not an ordinary Amount<A>. Values encode canonical unsigned decimal m.
- `ScaledAmount<A,S>`: nonnegative m <2^256, denoting m/10^S quanta of A;
  S=0..18, distinct from rounded integer Amount<A> even when S=0.
- `SignedScaledAmount<A,S>`: −2^255..2^255−1, the signed counterpart needed by
  negative rate calculations; it is not unsigned spendable cash.
- `SignedAmount<A>`: signed256 quanta, −2^255..2^255−1, for signed cash-flow calculations, not a
  Balance or a nominal Debt record. A negative result never becomes a debit
  by silently flipping its party orientation.
- `NetAmount<A>`: exact signed net in −(2^128−1)..+(2^128−1), computed from
  separately checked UInt128 eligible-incoming and fee counters. This admits
  the full unsigned source domain. It is distinct from both Amount and the
  wider SignedAmount; it does not authorize a cash debit or nominal writeoff.

A proposed mathematical encoding extends candidate02's type arrays with exactly
these five tags and parameters, decimal-string scalar values and one value node
per value. W wrappers and aggregate limits remain candidate02's exact rules.
`AmountProduct` requires ordered asset identifiers; the other tags require a
known asset and scale where present. NetAmount also uses the canonical signed decimal scalar and one value node;
its exact endpoint values ±340282366920938463463374607431768211455 are valid,
while magnitude340282366920938463463374607431768211456 rejects. This needs implementation, independent
boundary cases and new versioning before any schema accepts it. It is not an
alternate optional encoding chosen by the implementation.

## MA-D2: Proposed closed overloads

Evaluate both operands left-to-right, once; apply existing per-node work plus
explicit checked result bounds. No reassociation or multiply/divide fusion may
change rejection behavior. Exact result type follows the table; there is no
implicit conversion between Amount, Shares, Price, Rate or ordinary integers.
A same-number but differently identified asset remains a different type.

| Constructor and input types | Result and rule |
| --- | --- |
| Mul(Amount<A>, UInt128) or reversed | Amount<A>; exact product must fit128. An explicitly128-typed fee numerator/denominator is required; no implicit UInt64 widening. |
| Mul(Amount<A>, Amount<B>) | AmountProduct<sorted(A,B)>; exact product checked256. This product cannot be passed to Transfer. |
| FloorDiv/CeilDiv(AmountProduct<A,B>, Amount<A>) | Amount<B>; positive divisor; exact Euclidean quotient/remainder, round as named and check128. The symmetric A/B cancellation is defined by exact type identity, not field names. A=A returns Amount<A>. |
| Mul(Amount<A>, Price<B,A,S>) or reversed | ScaledAmount<B,S>; exact integer product checked256. Price<A,B,S> does not match this input direction. |
| Mul(Amount<A>, Rate<S>) or reversed | SignedScaledAmount<A,S>; multiply unsigned128 by signed128 exactly and check signed256. Negative rates stay signed. |
| FloorDiv/CeilDiv(ScaledAmount<A,S>, scaleDivisor) | Amount<A>; `scaleDivisor` is a **literal UInt128 exactly10^S** checked statically from S; Euclidean division and round/check128. Other divisors reject TYPE_SCALE_DIVISOR. |
| FloorDiv/CeilDiv(SignedScaledAmount<A,S>, scaleDivisor) | SignedAmount<A>; same literal10^S rule, positive divisor, signed Euclidean division and round/checksigned256. −5/2 floors−3 and ceils−2; no truncation toward zero. |

No other new overload is implied. No implicit division of Amount by Price, no
plain Rate→UInt conversion and no implicit SignedAmount→Amount. The source must
state the rounding call. A surface helper `floor_price(x,p)` may elaborate to
`FloorDiv(Mul(x,p),LitUInt(128,10^S))` only with exact type-derived S, source spans,
node charges and a source/Core correspondence case. It is bounded syntax sugar,
not a user-defined host function. Rate helpers use the same construction but
return SignedAmount. Positive interest can be admitted into a UInt128 nominal
interest field only through the explicit debt rule's nonnegative/range check;
there is no general cast changing a token value into nominal debt.

## Required equations and independent discriminators

For feeNumerator997, feeDenominator1000, reserveA1,000,000,
reserveB2,000,000 and amountIn10,000, the original expression shape types as:

- adjusted = Amount<A>(9,970,000), using Amount×UInt128;
- numerator = AmountProduct<A,B>(19,940,000,000,000);
- denominator = Amount<A>(1,009,970,000);
- floor output = Amount<B>(19,743), remainder
  162,290,000 in numerator units A×B;
- reserveA'1,010,000; reserveB'1,980,257; exact full balance/effect movement is
  still required by the financial operation rule.

The remainder identity is numerator = output*denominator + remainder. It is not
an extra162,290,000 B payable to a party. A reversed-price or wrong denominator
asset rejects statically. A pre-reserve/adjusted intermediate exceeding128 must
reject even when the later quotient would fit; the example is not permission to
silently widen all Amount multiplication. At the top domain boundary,
(2^128−1)² fits AmountProduct's256 bits, while multiplying Amount(2^128−1)
by UInt128(2) rejects its128-bit overload. Ceil output on the example is19,744;
minimum-receive and exact-output acceptance must not confuse the two directions.

For Price<B,A,4>(19743) and input10,000 A, scaled product197,430,000 divided
by10,000 yields19,743 B. For amount5A and Rate<1>(−5), signed scaled product−25
represents−2.5 A; floor yields−3 A and ceil−2 A. Neither value pays or erases a
debt without a financial transition.

## MA-D3: Shares and principal need explicit, different rules

Do not fake total vault supply as Shares<V,H> belonging to the next depositor.
Vault supply and a holder's entitlement are different schema fields. Proposed
operation-level conversion for a no-fee, already initialized homogeneous vault:

- deposit assets a -> minted shares floor(a*S/Va);
- mint exact shares s -> required assets ceil(s*Va/S);
- withdraw exact assets a -> burned shares ceil(a*S/Va);
- redeem shares s -> assets floor(s*Va/S).

Here S is share supply, Va is the valuation basis explicitly selected by the
rule, and every product is checked unsigned256 before division; a,s,S,Va and
results are checked UInt128. Both S and Va are positive for this subrule. Zero
supply, donation protection, fees, rebasing, illiquidity and external asset value
must have separate explicit equations; no unsupported case falls back to1:1.
Valuation purpose must be named and cannot turn accounting NAV into exit cash.
Example S3/Va10: deposit4 mints1 share; mint1 requires4 assets; withdraw4 burns2;
redeem1 returns3 assets. These differing directions cannot use one rounded rate.
ShareMint/ShareBurn identify holder and require actual funded movements plus
these computations; the share expression value conveys no authority by itself.

**Explicit construction decision:** [ConstructShares](DYNAMIC-SHARES-CONSTRUCTOR.md)
adds a proposed41st pure Core constructor, preserving all40 existing constructors.
The source `shares<V,H>(q)` requires UInt128 q and statically bound V/H; it returns
Shares<V,H>(q) with one own node charge and no financial effect. Vault supply
remains its separately declared UInt128 field, not a holder-indexed share value.
This closes the specific dynamic-value introduction decision. Its new profile,
grammar/representation/elaboration and evaluator/K rules require independent
review and implementation; no published constructor is silently changed.
Product-specific arithmetic still needs independent corpus equations and the
complete numeric rule-body instantiation.

Nominal Debt P/I are unsigned quanta of their declared nominal denomination;
settlement Amount<A> is separate. Interest-first repayment of7 against P100/I10
has dI7,dP0, residual P100/I3. Conversion to actual token payment requires the
specified settlement rule and exact funding receipt; a generic Amount/Price cast
is not repayment. Capitalization P100/I10→P107/I3 preserves outstanding110 but
changes the future interest base. Negative accrual, ACTUS calendars/day counts,
fees and nominal cap treatment require their explicit financial rules.
