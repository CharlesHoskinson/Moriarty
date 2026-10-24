# Why whole-program type preparation still precedes runtime

The declaration-only producer can populate T1/T2 tables from declaration fields,
but a generic instance may first appear in a function signature or constructor
use. The new `prepare_ext_program_types` keeps one checked builder across these
sources: declarations receive their IDs and FIFO fields first; T3 signature
uses precede entry/function-body uses, and every new key is drained on that same
builder. Finishing one builder and starting another for signatures would lose
identity reuse and the shared constructor/field layout.

The two observed fixtures discriminate that relationship. Chain<UInt> and
Chain<Bool> arise only through T3 uses, whose fields discover Tag specializations;
Ribbon<UInt> is required by a signature even without an entry constructor. Full
names, IDs, tables, T3 records and retained source match independent frozen data.
This is useful actual compiler preparation evidence, recorded in the
[result archive](../../results/sherlock-generic-one-builder-preparation-20261018/README.md).
Assertions ran in memory; their frozen expected JSON is not an actual output dump.

A type graph is still different from a runtime value graph. A constructor must
evaluate its source fields once in the required order, retain pending work and
return context across recursive calls, and allocate the finished nominal value.
A match must resolve source identities, take the first matching arm and retain
binder occurrence paths and fresh bindings while avoiding unselected bodies.
T3 records and declaration constructor order do not execute those behaviors.

UIREQ034's original recursive Schedule scenario remains UNRUN and OPEN: first
ordered arm4, separate structural sum9, repeated Pay4 sum8 and nested recursive
field order still need their own actual generic execution. Optional Schedule
preparation fixtures were excluded from the reviewed/executed two-test batch.
No disputed PD1 traversal policy is adopted by this result. Extended ProgramView
admission, C/P lowering, ConstructorFields, MatchPlan, generic decoding/witness/
boundary representation and formal/native/target correspondence stay separate.
This batch builds no circuit and calls for no circuit website iteration.

See [the public API](../reference/generic-program-preparation-api.md),
[the preparation guide](../how-to/prepare-generic-program-types.md) and
[the earlier declaration boundary](nominal-declaration-boundary.md).


## Canonical TypeId through the existing owner

The [canonical carrier migration](../reference/canonical-runtime-types.md) puts the same TypeId in function and parameter signatures, PairTask, Bound, ReturnF and Scalar.expected. Source sorts still describe the four builtin types. Keeping one identity across these boundaries lets later generic work use a common carrier without claiming generic admission already exists.

Three compiles and ten new tests passed. The four builtin cases follow actual prepared rows into intermediate owner state, bound values and final results; final-value equality alone would miss an unused map. A separate test roundtrips nominal IDs 4, 5 and 17 through the six storage roles. Five V8 tests preserve builtin admission and nominal refusal precedence.

Generic nominal backing/admission, ConstructorFields and declaration-driven ordered MatchPlan execution remain missing. Original UIREQ034 and all 25 refinements stay open. See the [actual evidence](../../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json) and [Root compatibility adoption](../../results/sherlock-canonical-runtime-types-20261018/ROOT-CANONICAL-TYPEID-COMPATIBILITY-ADOPTION.json).
