# Frozen K-A Quint counterpart: normal local result

This pair targets `sherlock-k-a/1`, independently of the historical
`universalMachine.qnt` algorithm instance. The model now interprets supplied
finite expression and ordered function tables through ordinary K-A control
transitions. On October 15, 2026, the integrator observed successful typechecking
and all 98 deterministic normal tests passing on the exact source identified
below. This is current-local Quint evidence for the six frozen normal programs
and their finite fuel prefixes.

`kProfileMachineTest.qnt` was written first. Its program tables and expected
snapshots are structural transcriptions of the independent literals in
`../oracles/normal-programs.json` and `../oracles/normal-prefixes.json`.
No candidate interpreter or candidate transition helper computes expected states.
The tests assert all six complete exact-fuel traces and all 92 literal ordinary
fuel-prefix rows: 98 deterministic tests in total. The longest test uses 28
transitions after initialization. Every exact trace asserts the complete state
at each boundary plus the rule line and charge; prefix tests assert complete
published states, including the preserved environment and frames.

The first falsifiable predicate is `n01ExactTraceTest`: after free startup rule
74, `Eval(Literal(UInt(7)))` with fuel 1 becomes `Return(UInt(7))` with fuel 0
under charged rule 102, then empty control and `Done(UInt(7))` under free rule 80.
The integrator observed behavioral RED on the preceding deliberately wrong
stub: initialization succeeded, typecheck succeeded, and the first charged
step produced `Return(UInt(0))` with fuel 0 under rule 102/charge 1, failing the
frozen `Return(UInt(7))` assertion. Initial setup failures were corrected before
that observation. The generic implementation began only after this observed
RED and an explicit integrator handoff. The replacement generic evaluator now
passes this predicate and the full unchanged 98-test normal suite. The RED
receipt remains historical evidence for its deliberately wrong source.

The remaining normal specimens are checked add 2+3, not/conditional selection,
nested lexical shadowing/restoration, named self recursion on true then false,
and ordered list/tree matching. Their exact charged totals are respectively
7, 9, 17, 27, 19; N01 requires 1. The N05 call holds function name 11 and refers to
the original immutable function body. The test does not expand its recursive
call into a new syntax tree or choose a scenario-specific evaluator.

The frozen source is `../k/main.k`, SHA256
`b3746ff51dd7999b046ccb23237595605420adcfeebc0c04c0ad0f1396b11d41`.
The Rust comparison boundary is
`crates/mori-core/src/sherlock/reference_types.rs` and `reference.rs`.
The correctness loop and implementation plan remain controlling source context.
No existing model, historical fixture or oracle file is changed by this pair.

| K-A / typed host element | Explicit Quint projection |
|---|---|
| Immutable expressions and function body references | Finite `ExprId -> Expr` syntax table; identifiers are syntax names, not mutable pointers |
| Ordered function table | `List[Function]`; lookup takes the first source match without sorting or uniqueness admission |
| Source recursion | `Call` keeps the function name and argument ExprIds; dispatch reuses the table |
| UInt128 | Mathematical `int`, admitted endpoints 0 through 340282366920938463463374607431768211455; `inUInt128` names that exact domain |
| Bool | `BoolValue(bool)` |
| Typed UInt list | `UIntList(List[int])`, mapping Nil to empty and Cons to ordered head/tail sequence |
| Typed tree | `UIntTree(List[TreeToken])`, prefix traversal: Leaf(n) maps to TreeLeaf(n), Branch(l,r) maps to TreeBranch followed by l then r |
| Names and environments | Canonical signed-decimal K names map to `int`; environments are maps, without observable map iteration order |
| Frames | Top-first `List[Frame]`; all saved caller and scope environments are retained |
| `uiPush(frame,uiEval(expr),env)` | `Push({frame,expression,environment})`; every frozen K-A push has this continuation shape |
| `.K`, pending/output, fuel and stack limit | `EmptyControl`, `Outcome`, exact integer counters and explicit limit in every snapshot |
| Source transition observation | `lastRule` and `lastCharge`; initial post-startup observation is 74/0 |

Installed Quint 0.32 reserves built-in identifiers, including `Bool` and `next`.
A source-only comparison against its installed `names/base.js` built-in list
and generated `QuintLexer.js` keyword list found those two declaration conflicts
in this pair. The projection constructor is now named `BoolValue`, and the
step-local binding is named `transition`. This is a mechanical naming change:
all `uiB(true/false)` values, oracle literals, programs, expected snapshots, rule
lines, fuel and charges retain their exact meanings. Reversing only
`BoolValue(` to `Bool(` reproduces the preceding test source byte for byte.
The implementation now returns the supplied valid literal rather than the
wrong RED value. The independent test file is frozen at SHA256
`985069bfdfef2f8417a3b7583454b90d84c1764070d77baa608d0add627f6493`.

Tree tokens describe recursive immutable value structure. They have no address,
sharing, heap identity, allocator or pointer semantics. The intended common
domain requires a complete well-formed prefix tree with exact UInt128 leaves;
`validTree` checks this literal projection using a finite pending-child fold;
constructor, equality and pattern helpers operate on that structural form. List element
order and tree branch order remain semantic. ExprId assignment may identify
equal immutable syntax; source expression identity is not an observable value.

The input API accepts finite concrete program tables and finite lists, with
explicit concrete fuel and stack limits. This source contains no uninstantiated
constants, recursive Quint operators or unbounded nondeterministic selection.
The 98 tests supply finite syntax, fuel 0 through exact completion plus 1 and
stack limit 8. The exact UInt128 endpoint is not a claim that the full domain has
been sampled or exhausted. There is no implicit scalar 15 restriction.

The implementation dispatches every frozen K-A expression form, all nine
frame forms, invoke/enter/bind, ordered pattern arms and primitive completion.
Function lookup is a left fold retaining its first source match. Binding is a
right fold over finite parameter indices, preserving the source helper's
tail-first duplicate/type checks. Arguments append in source order, calls begin
with fresh parameter bindings, and explicit return/scope frames restore saved
environments. Frame replacement retains depth; source pop rules remove one
frame. Failed conditional/return-type checks and failed pushes preserve the
original frames and environment. All charged transitions identify their exact
pinned K rule line. Free return/error publication precedes exhaustion.

Scalar add/sub/mul use mathematical integers followed by the exact UInt128
range check; comparison and structural equality retain their declared types.
List constructors and patterns use ordered sequences. Tree leaf/branch
construction and equality use prefix tokens; branch matching finds its first
child's extent through a finite fold and returns both ordered child values.
Normal literal validation checks UInt128 leaves and complete tree structure.
These are A helper normalizations inside their charged machine rules, with no
additional A fuel events and no measured helper work claim.

The declared common input domain is a finite closed immutable AST table whose
expression references unfold to finite syntax, with canonical signed-decimal
names, finite function/parameter/arm lists and nonnegative fuel/stack limits.
Equal immutable syntax may share an ExprId; syntax cycles or missing ExprIds
have no K expression counterpart and no fabricated semantic rejection code.
Named function recursion, including mutual references, reuses the same table
and does not require source unrolling. Trees must be complete single prefix
trees; empty/trailing-token encodings are outside the typed K tree projection.
This is a source-domain declaration, not a general admission implementation or
a claim of totality on arbitrary `Snapshot` or `Program` records.

All six exact traces and 92 prefix tests remain independent and unchanged, and
all 98 now have observed passing local results. Broader input programs, the
frozen eight fixtures, remaining D01–D36, full grammar/negative coverage and
actual cross-layer comparisons remain unobserved for this candidate. No
security negative-case or theorem evidence is supplied here. This pair defines
no B helper profile, image policy, admission policy, security predicate,
theorem, native interface, circuit or cross-layer correspondence result.

The complete local import closure is exactly `kProfileMachineTest.qnt`
importing `./kProfileMachine.qnt`. The model has no imports or provider/network
calls. The observed tool was the installed Quint 0.32.0 TypeScript backend.
The exact tested model SHA256 is
`c62c12dd3a973bcb54b549f45768a356b0fb3a154b193cd76ac80d2d80fa95ca`;
the frozen test SHA256 is
`985069bfdfef2f8417a3b7583454b90d84c1764070d77baa608d0add627f6493`.
The integrator publishes the separate Quint-only receipts under
`docs/audits/2026-10-15/sherlock-k-profile-quint/`.

Recorded Quint arguments from the checkout root were:

```bash
quint typecheck models/sherlock/quint/kProfileMachineTest.qnt
quint test models/sherlock/quint/kProfileMachineTest.qnt --main kProfileMachineTest --match '^n0[1-6].*Test$' --backend typescript --max-samples 1 --seed 1
```

Each command ran under a 60-second timeout with a 5-second termination grace,
4 GB memory ceiling, swap disabled, task limit 256 and network disabled.
The Node heap ceiling was 3072 MB. Typecheck exited 0 in approximately 3.96
seconds. The normal suite exited 0 in approximately 3.96 seconds overall;
Quint reported `98 passing (163ms)` and the captured stderr was empty.
The seed was 1 and max samples was 1; these are deterministic test results,
not a random simulation or exhaustive model-checking result. The source author
performed no additional model execution while updating this account.

Implementation source, observed model evidence and acceptance remain separate.
These normal model observations grant no native authority and close no retained
roadmap exit. Security review, full grammar and fixture coverage, formal
correspondence, circuit checks, proofs and native/target evidence remain
separate and open.
