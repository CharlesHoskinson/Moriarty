# Independent review: `sol-static.k` (MIL/4 K static slice)

This review is based only on reading the file. I did not run `kompile` or `kprove`, and I used no tools. Nothing here proves, adopts or settles anything natively. SP01.6 is still blocked.

## Compile-level defects

These concern how K reads the rules, not what MIL/4 should mean.

**C1. `m4sThresholdCheck` has two `[owise]` rules that overlap.**
- *Counterexample:* `m4sThresholdCheck(0, m4sNoFormulas(), m4sTyped(m4sProp()))`. Both `owise` rules match this term. The result can be either `TYPE_THRESHOLD_BOUND` or `TYPE_FORMULA`, depending on the backend.
- *Consequence:* The function is not deterministic, which contradicts its `[total]` claim. The Haskell backend may report this.
- *Repair:* Make the last rule match only non-`Prop` results:
  `rule m4sThresholdCheck(_,_,m4sTyped(T)) => m4sRejected("TYPE_FORMULA") requires T =/=K m4sProp()`
  Then keep only the bound-failure rule as `owise`, or give each rule an explicit `[priority]`.

**C2. `[total]` is only asserted.** `kompile` does not check it.
- *Counterexample:* `m4sRounded(-5, -1, m4sFloor())` returns `m4sNumber(-5)`. The Floor rule has no guard, so a public helper returns a negative value.
- *Repair:* Guard every `m4sRounded` rule with `requires m4sUnsigned(Q,128) andBool R >=Int 0`, and let `owise` reject everything else. Then make the other numeric helpers private, or check their inputs the same way.

I cannot confirm any other hard compile error without running `kompile`. The nonlinear patterns and the forward references to sorts are legal K.

## Semantic defects and omissions

**S1. The basic conservation formula cannot be typed.**
- *Rule:* `m4sCombine(m4sTyped(T),m4sTyped(T),OP)` only accepts two operands of the same type.
- *Counterexample:* `m4sEqual(m4sPost("bal"), m4sAdd(m4sPre("bal"), m4sHole("d")))`, with `bal : m4sQty("A")` and `d : m4sDelta("A")`, gives `TYPE_MISMATCH`. So "post = pre + delta" cannot be written.
- *Repair:* Add typed mixed rules:
  - `ADD(Qty(a), Delta(a)) → Qty(a)`
  - `SUB(Qty(a), Qty(a)) → Delta(a)`

  Each needs a named checked-evaluation rejection. Delta must not be silently coerced to Qty.

**S2. Qty subtraction is typed as Qty.**
- *Rule:* `m4sArithmetic(m4sQty(_),_) => true` for any operator, including `"SUB"`.
- *Counterexample:* `m4sSub(m4sQtyLit("A",1), m4sQtyLit("A",2))` is typed `Qty(A)`, but `m4sCheckedSub(1,2)` rejects it with `NUMERIC_UNDERFLOW_OR_RANGE`. Nothing says whether a guard whose term underflows is false, is rejected, or blocks settlement.
- *Repair:* Either type `SUB` on Qty as `Delta` (see S1), or state that any term which hits a `m4sNumericReject` rejects the whole formula and is never false. Also replace `m4sArithmetic`'s string operator with a closed `M4SOp` sort, so a misspelt operator such as `"MUL_LITERL"` fails to parse instead of quietly falling into `owise`.

**S3. Delta passes type checking but has no numeric semantics.**
- *Rule:* `m4sTypeValid(A,m4sDelta(ID))` accepts Delta, but every checked helper requires `m4sUnsigned(_,128)`.
- *Counterexample:* A hole `d : Delta(A)` with value −3 has no evaluator path. Every helper rejects it.
- *Repair:* Add `m4sSigned(N,W) := -2^(W-1) <= N < 2^(W-1)` and signed checked add and sub. Until those exist, set `m4sTypeValid(_,m4sDelta(_)) => false`.

**S4. Fuel measures list breadth as well as depth, so reordering changes acceptance.**
- *Rule:* In `m4sFormList(E,m4sFormulaCons(F,REST),P,N)`, `REST` gets `N -Int 1`. Element *i* gets N−i fuel.
- *Counterexample:* Take N=6 and a threshold over [four trivial atoms, one atom of depth 4]. It is rejected with `STATIC_DEPTH`. Put the deep atom first and it is accepted. A 128-element threshold inside another threshold always fails under the 256 cap. The comment says the fuel is a depth bound, but it isn't.
- *Repair:* `m4sFormList(E,Cons(F,REST),P,N) => m4sBoth(m4sForm(E,F,P,N), m4sFormList(E,REST,P,N))`. Keep the size limit as a separate breadth check (count ≤ 128, which already exists), and add a total-node budget if one is intended.

**S5. The time types have no useful arithmetic.**
- *Rule:* `m4sArithmetic` does not list Instant or Duration. Its `owise` only allows `MIN` and `MAX`.
- *Counterexample:* `m4sLess(m4sHole("now"), m4sAdd(m4sPre("start"), m4sHole("ttl")))`, with `Instant("c")` and `Duration("c")`, gives `TYPE_MISMATCH`. Deadlines cannot be expressed. `Duration + Duration` is also rejected.
- *Repair:* Add three typing rules:
  - `ADD(Instant(c), Duration(c)) → Instant(c)`
  - `SUB(Instant(c), Instant(c)) → Duration(c)` (checked, rejects if negative)
  - `ADD(Duration(c), Duration(c)) → Duration(c)`

  Also check clock tags against a registry, just as asset IDs are checked. Today any non-empty string is accepted.

**S6. The numeric layer is not connected to the typing.**
- *Rules:* No typing rule ever invokes `m4sCheckedMul`, `m4sCheckedDivMod` or `m4sReserveUnit`. `m4sVariableProduct` is always rejected, so converting Qty × Price into a quote-asset Qty cannot be written.
- *Counterexample:* A price-conversion guard is inexpressible, yet the file contains rounding and reserve helpers that suggest it is supported.
- *Repair:* Add `m4sConvert(M4STerm, M4STerm, M4SRounding)` with the typing rule `Qty(b) × Price(b,q,s) → Qty(q)`. Its evaluation must be defined through `m4sCheckedMul(_,_,256)` followed by `m4sCheckedDivMod(_,10^s,…)`. Alternatively, label the helpers as unreferenced. Separately, `m4sReserveUnit` only checks that TOTAL = ALLOCATED + RESERVE. It does not enforce the comment's claim that ALLOCATED came from floor rounding. Pass the divmod certificate in, or drop the claim.

**S7. `post(x)` borrows the pre-state binding and ignores mutability.**
- *Rule:* In the outcome phase, `m4sInfer(m4sEnv(_,C,_),m4sPost(ID),m4sOutcome(),N) => m4sLookup(C,ID)`.
- *Counterexample:* If `C` holds an immutable parameter `limit : UInt`, then `post("limit")` is well-typed. That implies the parameter could change.
- *Repair:* Split the environment into state bindings and parameter bindings, and allow `m4sPost` only on state bindings. Otherwise reject with `TYPE_POST_IMMUTABLE`.

**S8. The public entry points skip validation.**
- *Rule:* `m4sInfer` and `m4sForm` are total public functions. Only `m4sStatic` checks `m4sEnvValid`.
- *Counterexample:* `m4sForm(m4sEnv(m4sNoAssets(), m4sBinding("s", m4sShares("X","Y"), m4sNoBindings()), m4sNoBindings()), m4sEqual(m4sPre("s"), m4sPre("s")), m4sGuard(), 4)` returns `m4sTyped(m4sProp())`. That admits Shares, which the static slice is supposed to reject. Relatedly, `m4sComparable(m4sShares(_,_)) => true` contradicts `m4sTypeValid(_,m4sShares(_,_)) => false`.
- *Repair:* Mark `m4sInfer` and `m4sForm` `[private]`, or check `m4sTypeValid` inside `m4sLookup`. Set `m4sComparable(m4sShares(_,_)) => false` until a share registry exists.

## What the module can validly claim

- It is a *specified-only*, fuel-bounded static checker. It returns a type or a named rejection, never a Boolean success flag. Treat it as unkompiled until `kompile` runs clean.
- Asset IDs are nominal and there are no duplicates in the asset table. Decimals and scale are checked metadata only.
- Through `m4sStatic`, formulas typed `Prop` contain only same-type comparisons of valid, non-Shares types. There is no `post` in guards and no negation of non-atoms.
- The helpers are checked unsigned 128-bit helpers with certificate-verified divmod and explicit rounding. They are not tied to evaluation (S6).

## What must remain explicitly rejected

- Oracle premises (`OPEN_ORACLE_PREMISE`), Ω certification (`OPEN_OMEGA_CERTIFICATION`), variable products (`UNSUPPORTED_PHI1`) and open formulas (`OPEN_FORMULA_PREMISE`)
- Shares at every entry point, not just through `m4sStatic` (S8)
- Delta values until signed semantics exist (S3)
- Any soundness or progress claim linking typing to evaluation, because the numeric layer is not connected (S2, S6)
- Any claim of correspondence to source, Core or native code, native settlement, or adoption
- Any claim that `[total]` has been checked