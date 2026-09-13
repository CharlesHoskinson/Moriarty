# Jet discipline, computation slice: 23 Core/4 constructors

Sources: `experiments/moriarty-language/src/successor/financial-expression-v1.ts` (cited as v1.ts), `financial-expression-types-v1.ts` (types.ts), `formal/k/expression-v1.k`, `expression-infer.k`, `expression-types.k`, and `deliverables/language-to-ledger-2026-09-12/k/independent-constructor-inventory-01.json`. Core/5 (`financial-expression-v5.ts`) changes nothing in this slice: the diff touches imports, contract identifiers and the lifecycle binding only.

## Verdict

Of the 23 constructors, 14 are definable from the other 9 with no change in meaning except one: Moriarty's work meter is observable in the result (`workRemaining`, v1.ts:36-37), so any expansion of `And`, `Or` or `Not` into `Select` changes the observable work count by one unit per expansion. A jet discipline for Moriarty therefore needs a rule Simplicity does not need: the cost of a jet is a declared constant, not the cost of its expansion. The nine literal constructors are one construction, `Lit(type, value)`; they share a single dynamic rule in both implementations and differ only in how the type is spelled on the wire. The five arithmetic constructors are the real jet candidates, but there is no single "checked 128-bit add": the result type selects one of six range classes, and K and TypeScript already disagree about which classes exist. That disagreement is the drift the jet discipline is meant to prevent, and it is present today.

## Table

| Constructor | Derivable | Jet candidate | Performance motive | Proof obligation |
|---|---|---|---|---|
| Add | No | Yes, one per range class | High under a bit-blasted core; none in the bigint evaluator | native add of class R returns `n` iff `fits_R(a+b)` and `n = a+b` |
| Sub | No (no negation over unsigned classes) | Yes, per class | High | same shape, for `a-b`; note `NetAmount` is asymmetric (types.ts:156) |
| Mul | No | Yes, per class; Amount×Amount is a 128→256 widening product, a distinct jet | High | native product agrees with `a*b` and with the class the static rule assigns (v1.ts:336-357) |
| FloorDiv | No | Yes | High (division) | `B>0` else `ARITH_DENOMINATOR`; floor toward negative infinity: TS 404 and K `exFloor` 144-146 must agree |
| CeilDiv | Not within Core/4 types (see prose) | Yes | High | TS `q+1 if r != 0` (405) equals K `0 - exFloor(0 - A, B)` (141) on every in-range pair |
| And | Yes: `Select(l, r, Lit(Bool,false))` | No | None | short-circuit preserved; work count differs by 1 |
| Or | Yes: `Select(l, Lit(Bool,true), r)` | No | None | as And |
| Not | Yes: `Select(v, Lit(Bool,false), Lit(Bool,true))` or `Eq(v, Lit(Bool,false))` (K 125 defines it that way) | No | None | work count |
| Eq | No (structural over Record, Text, Enum, Bool, Option) | Marginal | Medium: Text up to 1024 bytes, Records up to 4096 nodes | `canonical(a)===canonical(b)` (TS 408) iff `==K` (K 123): injectivity of the canonical form |
| Lt | No | Marginal | Low-medium | comparison never range-fails; mantissa order is the intended order at equal scale |
| Lte | Yes: `Not(Lt(r, l))` | No | None | totality of the order on bigints |
| Gt | Yes: `Lt(r, l)` | No | None | none beyond Lt |
| Gte | Yes: `Not(Lt(l, r))` | No | None | none beyond Lt |
| Select | No: the only branch in the slice | No | None | K does not admit it at all (inventory: `retainedKAdmitsConstructor: false`) |
| LitUInt | Yes: `Lit(['UInt'+w], v)` | No | None | width in {64,128,256} (v1.ts:222); K admits 64 and 128 only (infer.k:126) |
| LitSInt | Yes: `Lit(['SInt128'], v)` | No | None | none |
| LitBool | Yes: `Lit(['Bool'], v)` | No | None | none |
| LitText | Yes: `Lit(['Text'], v)` | No | None | 1024-byte bound (types.ts:182) |
| LitAmount | Yes: `Lit(['Amount', asset], v)` | No | None | `resolve` checks asset (types.ts:86) |
| LitQuantity | Yes: `Lit(['Quantity', units, scale], mantissa)` | No | None | unit domain and scale 0..18 (types.ts:67-95) |
| LitShares | Yes: `Lit(['Shares', vault, holder], v)` | No | None | vault and party resolve |
| LitRate | Yes: `Lit(['Rate', scale], mantissa)` | No | None | scale bound |
| LitPrice | Yes: `Lit(['Price', base, quote, scale], mantissa)` | No | None | `base != quote` (types.k:55) |

## Analysis

### Arithmetic

All five arithmetic constructors share one dynamic rule per implementation. TypeScript, v1.ts:397-407:

```ts
const a = BigInt(v[0]), b = BigInt(v[1]); let n: bigint;
if (k === 'Add') n = a + b; else if (k === 'Sub') n = a - b; else if (k === 'Mul') n = a * b;
else { if (b <= 0n) fail('ARITH_DENOMINATOR'); let q = a / b, r = a % b; if (r < 0n) { q--; r += b; }
       n = k === 'CeilDiv' && r !== 0n ? q + 1n : q; }
result = String(n); if (!numericFits(type, result)) fail('ARITH_RANGE');
```

K (`expression-v1.k:136-146`) computes the same functions over unbounded `Int` and checks `exFits(T, N)` at line 142. So the value semantics is unbounded arithmetic plus a range check indexed by the result type, and the dimensional algebra lives entirely in the static rule `arithmeticType` (v1.ts:335-362). A jet never sees units, only the range class the static rule assigned.

But there is no single "checked 128-bit add". `numericFits` (types.ts:152-157) defines six classes: unsigned 64, 128 and 256, signed 128 and 256, and the asymmetric `NetAmount`. A jet for `Add` is six jets, keyed on the static rule's output, and each carries the obligation: the native operation of class R returns a value exactly when `fits_R(a op b)`, and then returns `a op b`.

The repository shows what happens without that obligation. K's `exFits` (`expression-types.k:18-21`) has three classes and no 256-bit one; `exArithmeticType` (infer.k:130) lacks the Amount×Price, Amount×Rate and AmountProduct÷Amount cases of v1.ts:336-350; the inventory records "Width256 is not admitted by retained K". The two definitions have drifted exactly where a jet's proof obligation would have caught it.

`CeilDiv` is defined by different formulas in the two implementations (TS adds one on nonzero remainder; K negates a floor of the negation, line 141). They agree on unbounded integers, but `CeilDiv` cannot be derived inside Core/4 with identical failure behaviour: `-floor(-a/b)` needs `0 - a`, which fails `ARITH_RANGE` for positive unsigned `a`, and `floor((a+b-1)/b)` overflows at `a = 2^128-1, b = 2` where the primitive succeeds. It stays primitive.

One static rule is hostile to any jet scheme. v1.ts:347 requires the divisor of a `ScaledAmount` to be syntactically a `LitUInt` node:

```ts
if(!same(right,['UInt128']) || rightNode?.constructor!=='LitUInt' || rightNode.operands.width!=='128' || ...) fail('TYPE_SCALE_DIVISOR');
```

A jet wrapping that literal is rejected, though under Merkle-root recognition it is the same term. The rule must become a property of the divisor's value before jets or a single `Lit` can be introduced.

### Boolean, comparison, control

`And` and `Or` are the slice's only non-strict constructors: v1.ts:368-370 returns the left value without touching the right when it decides, and K 95-99 does the same (comment at K 94: "Entry is charged once"). `Select` (v1.ts:372-375) evaluates the condition and one branch, so `And(l, r) = Select(l, r, Lit(Bool,false))` preserves short-circuiting exactly, including that `And(false, FloorDiv(x, 0))` never raises `ARITH_DENOMINATOR`. It does not preserve the work meter: the expansion charges one extra unit for the `LitBool` branch when taken. Since `workRemaining` is published and `WORK_EXHAUSTED` is an observable rejection, deleting the jet changes the output. Simplicity has no observable cost inside the language, so its no-op property is free; Moriarty's has to be stipulated by attaching a constant work charge to each jet and proving the interpreter debits exactly that.

K already defines `Not` as equality with `false` (`expression-v1.k:125`). The four orderings reduce to `Lt` by swapping and one negation; both implementations compare mantissas (v1.ts:410, K 148-151) and the static rule (v1.ts:300-302) forces equal types, hence equal scales. `Eq` is not derivable: line 302 exempts it from the numeric requirement, so it applies to Records, Text and Options and compares canonical serialisations no Core/4 constructor can fold over. Its obligation, injectivity of `canonical` on well-typed values, is asserted by key sorting and nowhere proved.

`Select` is the slice's one branch, the analogue of Simplicity's `case`. The inventory records that retained K does not admit it at all. The basis below rests on a constructor that exists today in one implementation only.

### Literals

The nine literals have one dynamic rule per implementation: v1.ts:378 `result = this.literal(n).value` and K 109 `exLiteralValue(K,O)`. They have one static rule: v1.ts:264-265 builds the type, then calls `resolve`, `valueDomain` and `valueBound`. The `literal` switch at 219-233 is a table from wire operand names to a `ValueType` and a value:

```ts
case 'LitAmount':   return { type: ['Amount', o.asset], value: o.value };
case 'LitQuantity': return { type: ['Quantity', o.units, o.scale], value: o.mantissa };
case 'LitPrice':    return { type: ['Price', o.base, o.quote, o.scale], value: o.mantissa };
```

The `mantissa`/`value` split is naming (K 121-122 splits the same way). The nine are one construction, `Lit: [['type','type'],['value',...]]`, using the `type` operand role `ConstructSome` and `ConstructCollection` already use (v1.ts:63-64, checked by `typeShape` at 209); the value's shape follows from the type via `valueShape` (types.ts:166-178), and `LitUInt`'s width check (222) is subsumed by `typeShape`.

Two consequences need a decision, not an assumption. `resolve` accepts derived types (`AmountProduct`, `ScaledAmount`, `SignedAmount`, `NetAmount`, `UInt256`; types.ts:96-105) that have no literal today, so a generic `Lit` would admit them unless restricted. And v1.ts:347 dispatches on the string `'LitUInt'`. Nothing in either implementation says whether derived-type literals are wanted.

## Minimal basis for this slice

Primitive: `Lit`, `Select`, `Eq`, `Lt`, `Add`, `Sub`, `Mul`, `FloorDiv`, `CeilDiv`. That is 9 of 23. Derived: the other eight literals into `Lit`; `Not`, `And`, `Or` from `Select` and `Lit`; `Lte`, `Gt`, `Gte` from `Lt` and `Not`. Of the nine primitives, five (the arithmetic) are jet candidates with a strong motive under a bit-blasted core and a per-range-class obligation; `Eq` and `Lt` are marginal; `Lit` and `Select` are core, not jets. Whether the bit-blasted core is the intended execution target cannot be determined from the repository; the TypeScript evaluator is already native bigint, and nothing in `formal/k` or the deliverables names a circuit or bit-level backend.
