# Lane 08 — explicit CEK-style K evaluator candidate

Status: research proposal, specified only. Falsifiable predicate for later work: every admitted demo reaches its independently specified result, exact remaining fuel and empty continuation; bounded failures return the specified envelope. No K compiler, parser, backend, model checker or proof tool ran.

**Source facts.** Official FUN uses constructor values and ordered pattern cases; closures retain an environment, and application restores the caller environment [S1, “Functions and Closures”]. LAMBDA++ demonstrates that saving only computation loses environment context, and gives environment-carrying recursive closures [S2, `cc(Map,K)` / `muclosure`]. The evaluation-order lesson distinguishes deterministic `seqstrict` from nondeterministic `strict` [S3]. These tutorials motivate mechanisms; they do not specify Moriarty.

**Recommendation.** Define one explicit, first-order CEK-style machine, with immutable global code and a typed frame stack. Suggested configuration:

```k
<k> step </k>
<control> eval(Main) </control> <env> .Map </env>
<kont> .Frames </kont> <code> Functions </code>
<types> Datatypes </types> <fuel> InitialFuel </fuel>
<status> running </status> <out> pending </out>
```

Proposed syntax: `Expr ::= lit(Value) | var(Id) | let(Id,Expr,Expr) | prim(Op,Expr,Expr) | if(Expr,Expr,Expr) | call(FunId,Exprs) | ctor(TypeId,Tag,Exprs) | match(Expr,Cases)`. Use `Value ::= uint(Width,Int) | bool(Bool) | data(TypeId,Tag,Values)`; nominal recursive datatype declarations refer to names, e.g. `Tree = Leaf(UInt128) | Node(Tree,Tree)`. Admit finite trees with separate node/depth/byte limits; recursion in the declaration does not admit infinite values.

Every running transition consumes exactly one fuel unit and has `fuel > 0`; an otherwise enabled nonterminal transition at zero returns `FuelExhausted`. Terminal value plus empty stack succeeds without requiring another unit. Label rules and freeze which machine transitions count. Do not use K rewrite counts as semantic fuel. Evaluation contexts are explicit frames, avoiding invisible heating costs: evaluating `prim(op,A,B)` pushes `binL(op,B,rho)` and evaluates A; returning A pushes `binR(op,A)` and evaluates B under saved rho; returning B computes a checked result. All arguments and constructor fields evaluate left-to-right. `if` evaluates only its selected branch.

Global code maps unique function IDs to parameter types, result type and body; calls evaluate arguments, check arity, push `return(savedEnv)`, install a fresh parameter environment and enter the body. Recursive references resolve through the unchanged code map, avoiding substitution and self-containing values. Later higher-order support requires explicit closure IDs/captured environments. Checked UInt arithmetic uses K integers with `0 <= n < 2^width`; addition/multiplication overflow and subtraction underflow return explicit errors, never implicit wrapping. Match cases are tested in source order with deterministic structural matching; bindings reject duplicates, and mismatch advances once. Nonexhaustive matching is an explicit result even if static admission normally rejects it.

**Bounded demonstrators.** Specify `fact(n)=if n==0 then 1 else n*fact(n-1)`; `fact(3)=6` must retain pending multiplication frames. Specify `sum(Leaf(n))=n; sum(Node(l,r))=sum(l)+sum(r)`; `sum(Node(Leaf(2),Node(Leaf(3),Leaf(4))))=9` exercises non-tail branching recursion, saved right subtrees and pattern binding. Include zero fuel, UInt overflow, missing function, bad tag/arity and unmatched-case witnesses. Exact fuel expectations must be independently enumerated after rule freeze.

**Alternatives and gaps.** A bounded node heap with typed handles maps more directly to fixed circuit storage, but adds allocation, lookup and graph-validity obligations; pure value trees make the first reference semantics easier to inspect. Counterargument: explicit frames duplicate K’s native computation stack and environment copies can dominate state size. Accept that clarity initially; postpone representation optimization until a concrete trace exists. Existing expression/lifecycle K files show modular declarations, staged admission and failure envelopes, but are older Core/4 experiments, not universal correspondence evidence.

The pure fragment omits financial effects, intent/policy evaluation, residual duties and durable suspension. Extension needs staged writes/effects, explicit commit/reject boundaries, and a versioned serializable suspension containing control, frames, environments, fuel and code identity. Circuit implementation separately needs transition selectors, value/frame encodings, checked arithmetic and state/code commitments; frontend/codegen work alone establishes none of those constraints.

Later ordinary validation should inspect and classify syntax/type checks before authorization, then use concrete complete-state traces and negative witnesses. Determinism, preservation/progress and frontend/circuit correspondence require separately stated claims and proofs; passing examples are finite conformance witnesses.

S1: https://kframework.org/k-distribution/pl-tutorial/2_languages/3_fun/1_untyped/1_environment/fun-untyped/
S2: https://kframework.org/k-distribution/pl-tutorial/1_k/3_lambda%2B%2B/lesson_5/
S3: https://kframework.org/k-distribution/k-tutorial/1_basic/14_evaluation_order/
