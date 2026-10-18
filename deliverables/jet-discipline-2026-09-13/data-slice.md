# Jet discipline, data slice: the 20 construction, projection, conversion and read constructors

Sources: `experiments/moriarty-language/src/successor/financial-expression-v1.ts` (cited as `v1.ts`), `financial-expression-types-v1.ts` (`types-v1.ts`), `formal/k/expression-v1.k` (`v1.k`), `expression-infer.k`, `spec/successor/semantic-contract.md`, Simplicity's `Core.v` and `Primitive.v`.

## Verdict

All twenty constructors encode over Simplicity's six product and sum formers, and the encoding is free inside the evaluator: field names, enum members and variant tags survive into the dynamic rules only as JSON keys whose every use is a statically resolved lookup or a canonical-order equality that a schema-fixed name-to-position bijection preserves. Names must survive at the boundary, so the encoding needs a codec, not a semantics change. `AccessField`/`AccessIndex` are surface provenance, not a semantic family. The four reads collapse into navigation over one input product, and `ReadPre` with view `post` becomes better typed, not weaker. The real gap is partiality: `ProjectSome`, `ProjectVariant`, `ProjectIndex` and narrowing `ConvertUInt` fail at runtime, which the total nine-constructor core cannot express without `assertl`/`assertr`. Nothing here is a jet in Simplicity's sense today, because Moriarty has no lower definition for a jet to be identical to; the open cost question is EX-D4/EX-D5 work accounting.

## Table

| Constructor | Core encoding | Erases | Jet candidate | Proof obligation |
| --- | --- | --- | --- | --- |
| ConstructRecord | `pair` tree, sorted-name order | field names (codec keeps) | no, basis | order = `valueChildren` sort (types-v1.ts:161), `exSortPairs` (v1.k:117) |
| ProjectField | `take`/`drop` path by schema position | field name | no | path(name) bijective per record type |
| AccessField | same as ProjectField (v1.ts:287,389; v1.k:115) | nothing more | no | EX-D5 work equality |
| ConstructCollection | `pair` tree of `Option<T>`, capacity 128 (v1.ts:256) | nothing; length encoded | yes (packing) | packed-prefix invariant |
| ProjectIndex | `case` tree over 7 index bits, then assert | nothing | yes, strong | `INDEX_RANGE` iff index >= runtime length (v1.ts:391) |
| AccessIndex | same as ProjectIndex (v1.ts:289,390; v1.k:116) | nothing | as above | EX-D5 |
| ConstructSome | `injr` into `Sum Unit T` | nothing | no | value `[v]` (v1.ts:394) |
| ConstructNone | `injl unit` | nothing | no | value `[]` (v1.ts:395) |
| ProjectSome | `case fail iden` | nothing | no | partial; needs assert |
| ConstructVariant | `injl`/`injr` path by tag position | tag name | no | tag order fixed by schema |
| ProjectVariant | `case`, other branches `fail` | tag name | no | partial (`VARIANT_CASE`, v1.ts:382) |
| ConstructEnum | ConstructVariant with `Unit` payloads | member name | no | `enumTypes[e]` ordered (types-v1.ts:58) |
| ConstructAmount | `iden` + static retag | nothing | no-op | range check unreachable (prose) |
| ConstructShares | `iden` + static retag | nothing | no-op | same |
| ConvertUInt | widen `iden`; narrow assert | nothing | yes, given a word type | no bit-level type exists |
| ScalarValue | `quanta`/`mantissa` `iden`; `negative`/`magnitude` SInt128 arithmetic | nothing | yes for sign/abs | no two's-complement definition exists |
| ReadArg | path into input product | name | no | one snapshot validation (v1.ts:489) |
| ReadObs | path into input product | name | no | same |
| ReadLocal | `pair`/`drop` let-threading | name | no | depends on sibling's `Let` |
| ReadPre | `pre`: path into input; `post`: path into `(input, output)` | view becomes a position | no | TYPE_POST_SCOPE becomes ill-typedness |

## 1. Encodings and what they erase

Core/4 runtime values are plain JSON: a record is an object keyed by field name (v1.ts:392), an option is `[]` or `[v]` (v1.ts:394-395), a variant is `{tag, value}` (v1.ts:381), an enum is its member string (v1.ts:393), a collection is an array (v1.ts:396), every number a decimal string. So names do reach the dynamic semantics. Three things use them:

1. Lookup `v[0][o.field]` (v1.ts:389; v1.k:115). `o.field` is a static `id` operand (v1.ts:59) already resolved against `s.recordTypes` at v1.ts:288. It is a path fixed at check time.
2. `Eq` compares `canonical(v[0]) === canonical(v[1])` (v1.ts:408). Canonical JSON sorts keys, and both `valueChildren` (types-v1.ts:161) and the K record rule (`exSortPairs`, v1.k:117) commit to that order, so a pair tree in sorted order makes structural equality coincide with canonical equality. Enum equality on member strings coincides with index equality because `enumTypes[e]` is an ordered array of distinct identifiers (types-v1.ts:58).
3. The boundary: `bindSnapshots` admits `Pre`/`Args`/`Obs` name-keyed (v1.ts:487-491); `post = {...this.pre, ...this.writes}` (v1.ts:520) and each `Emit` descriptor (v1.ts:416) leave name-keyed. That is the ledger wire format.

Names are load-bearing only at the boundary. Inside, the encoding is free given one codec `Schema -> (name <-> position)` shared by both implementations. Collections erase nothing but must encode length, since `INDEX_RANGE` checks runtime length, not capacity (v1.ts:391; `static-semantics.md:110-113`); the packed-prefix invariant is the obligation.

## 2. Partiality is the real gap

`ProjectSome` fails `OPTION_NONE` (v1.ts:383), `ProjectVariant` `VARIANT_CASE` (v1.ts:382), `ProjectIndex` `INDEX_RANGE` (v1.ts:391), narrowing `ConvertUInt` `ARITH_RANGE` (v1.ts:379-380). Simplicity's nine constructors (`Core.v:5-15`) are total; failure enters only with `assertl`/`assertr`/`fail` outside the nine. A basis for this slice must include one assertion former, reconciled with the sibling slice's `Require`.

## 3. AccessField versus ProjectField

Not semantic. One static branch (v1.ts:287-288), one dynamic branch (v1.ts:389), one K rule each with a disjunctive side condition (v1.k:115-116; infer.k:87-88), identical operand tables (v1.ts:59-60). EX-D5 says it outright: "AccessField/AccessIndex are exact aliases of ProjectField/ProjectIndex, including errors and work" (`semantic-contract.md:38`).

What separates them is surface spelling. Postfix `.field` lowers to `ProjectField` (`expression-source-lower.ts:51`), `[i]` to `ProjectIndex` (`financial-expression-source-types.ts:122`); the call forms `access_field(r,"f")` and `access_index(c,i)` lower to the `Access*` names (`financial-expression-source-types.ts:177-186`). EX-D5's only content is that an optimizer may not delete the node charge: a cost rule, not a meaning, and the only thing left to decide.

## 4. The four reads

The evaluator holds five environments: `pre`, `args`, `obs` bound together by `bindSnapshots` (v1.ts:490); `locals` written by `Let` (v1.ts:413); `writes` by `NextWrite` (v1.ts:414). K has cells `<pre>`, `<args>`, `<obs>`, `<locals>`, `<writes>` (v1.k:110-114, 127-128).

`ReadArg`, `ReadObs` and `ReadPre` with view `pre` read three components of one immutable input, validated by the same `validateSnapshot`. They collapse without loss into `take`/`drop` paths over `Prod Pre (Prod Args Obs)`. `ReadLocal` reads a `Let` binding; Simplicity has no `let`, and the standard encoding threads the value through the input with `pair` and reads it with `drop`. This holds only if the sibling slice makes `Let` a threading form.

`ReadPre` with view `post` is the soundness-critical case and it comes out stronger. Today two static rules guard it, `TYPE_NEXT_READ` and `TYPE_POST_SCOPE` (v1.ts:284-285; infer.k:86), the second admitting `post` only inside `Ensure`, which the action check confines to the suffix (v1.ts:480-481). Dynamically it reads `writes` overlaid on `pre` (v1.ts:388; v1.k:114 `exOverlay(Pre,W)`), and since every `NextWrite` precedes every `Ensure`, that overlay is exactly the `post` of v1.ts:520. In the navigation encoding the action is a function `Input -> Output` and the ensure suffix is a check over `Prod Input Output`. A `post` read is a `drop` into the second component, which exists only in the suffix's input type; a `post` read in the prefix is not a scope violation but an ill-typed term, and `next` has no component at all. The view operand becomes a position in a type, which satisfies EX-D3 ("Read-view is part of Core encoding, never inferred from a name") by construction. This is not a shorter unsafe spelling.

What breaks is observable, not semantic: `TYPE_POST_SCOPE` becomes `TYPE_MISMATCH`, `nodePath` values in rejections change, and one read becomes several nodes under EX-D4's per-node work charge. All three are pinned by the frozen conformance cases.

## 5. ConvertUInt and ScalarValue

`ConstructAmount`, `ConstructShares` and `ConvertUInt` share one dynamic rule: `result=v[0]; if(!numericFits(type,result)) fail('ARITH_RANGE')` (v1.ts:379-380). The value passes through unchanged; only the type changes. For `ConstructAmount`/`ConstructShares` the check is unreachable: the child must be `UInt128` (v1.ts:267) and `numericFits` for `Amount`/`Shares` is the same 128-bit bound (types-v1.ts:157). They are type ascriptions with no runtime content.

`ConvertUInt` is `iden` when widening and a range assertion when narrowing (v1.ts:275 admits any of 64/128/256 in either direction). The dynamic rule reads the statically inferred type (`this.types.get(n)`, v1.ts:366) for the bound; it is not self-contained.

`ScalarValue` is four operations under one name (v1.ts:384): `quanta` and `mantissa` return `v[0]` unchanged; `negative` is a sign test and `magnitude` an absolute value on the decimal-string `SInt128`. The first two belong with `ConstructAmount`; the last two belong to the arithmetic slice.

Jets over a bit-level definition? No such definition exists. Every Core/4 number is a decimal string handled through `BigInt`; there is no word type and no two's-complement definition to wrap. Simplicity's jet is `Jet.jet A B t p := t (PrimitivePrimSem M)` (`Primitive.v:248`), which needs a `t`. The repository has not decided whether it wants a bit-level numeric core; until it does, "jet" here can only mean a native fast path for a decimal-string function, which both implementations already are.

## 6. Minimal basis for this slice

Six term formers plus one non-term form replace the twenty:

1. `pair`: ConstructRecord; ConstructCollection as a product of options
2. `path`: ProjectField, AccessField, ReadArg, ReadObs, both views of ReadPre, ReadLocal if `Let` threads
3. `inject`: ConstructSome, ConstructNone, ConstructVariant, ConstructEnum
4. `case`: ProjectVariant, ProjectSome, index dispatch in ProjectIndex/AccessIndex
5. `assert`: the partial projections and narrowing ConvertUInt; shared with the sibling's `Require`
6. sign/abs on SInt128: ScalarValue `negative`/`magnitude`; belongs to the arithmetic slice
7. type ascription, not a term: ConstructAmount, ConstructShares, widening ConvertUInt, ScalarValue `quanta`/`mantissa`

Count: 20 constructors to 6 term formers, two of them shared with other slices, so 4 belong to this slice alone. The two candidates that earn a jet in Simplicity's sense are `ProjectIndex` (a 7-level case tree over a 128-slot product is what jets exist for) and `ScalarValue` sign/abs, the latter conditional on a bit-level numeric definition that does not exist. Every cost of the change sits in observables the frozen cases pin: error codes, node paths and EX-D4 work counts.
