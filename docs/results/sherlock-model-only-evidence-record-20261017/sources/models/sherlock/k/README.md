# Moriarty universal-interpreter K candidate: pure first-order fragment

Source-only draft for K 7.1.337. No K parser, compiler, evaluator or prover was run by the author. No fixture result or trace is observed. Root owns per-source classification and any later Kore frontend check. Successful Kore emission would establish source well-formedness only.

The author hand-froze `expected-results.json` before writing `main.k` or the fixture terms. Each normal fixture is a direct `uiRun(code, expression, 1000, 64)` term under `fixtures/`. Its function table is input data; all eight fixtures use the same evaluator rules. The expectations are literal independently derived result values. There is no candidate evaluator used to generate expectations, no assertion runner, and no expected consumed-fuel oracle. All fixtures are specified-only.

| Fixture | Independent expected value | Capability |
|---|---|---|
| uiFactorial3.ui | uiU(6) | non-tail named recursion with pending multiplication |
| uiSum3.ui | uiU(6) | distinct recursive program under the same semantics |
| uiListsum123.ui | uiU(6) | recursive immutable UInt list, ordered matching |
| uiTreesum45.ui | uiU(9) | non-tail branching tree fold; pending right call |
| uiBoolLetOrder.ui | uiU(7) | bool negation, selected branch, let scope |
| uiCallerLocal.ui | uiU(12) | callee/caller parameter name overlap; caller restoration |
| uiMutualEven4.ui | uiB(true) | mutual recursion through unchanged global code |
| uiConstructList.ui | uiCons(uiU(1),uiCons(uiU(2),uiNil())) | strict recursive constructor expressions |

## Machine definition

`MORIARTY-UI-SYNTAX` declares the program language. `MORIARTY-UI` adds explicit machine control and frame sorts. The only required file is `domains.md`; direct builtin imports are `DOMAINS-SYNTAX` and `DOMAINS`. The only other import is the candidate's own syntax module. No I/O, reflection, substitution, fresh allocation, hooks, strings, external code or whole-evaluator recursive function is declared. No strictness attributes, rule priorities or otherwise rules select control transitions.

The `<k>` cell contains one `UIControl` term after startup. `<env>` is an immutable finite binding map. `<code>` is the input finite named function table. `<stack>` is an explicit inductive frame list; `<depth>` counts its frames, and `<stackLimit>` bounds every push. Frames specify evaluation of the left operand before the right, arguments left to right, the chosen branch only, typed function return, lexical scopes and match continuation. Function bodies run with only their declared argument bindings. Return frames restore the caller map; argument evaluation restores the caller map before invocation. Let and pattern-bound scopes restore their saved map after the body.

Names are integer identifiers. Functions have typed parameter lists and a declared return type. Lookup chooses the first equal function identifier; the intended admitted source profile requires unique function identifiers and function bodies closed over their arguments/let/pattern bindings. The draft does not implement an independent whole-program static checker. Runtime binding checks enforce arity, parameter types and distinct parameter identifiers. Return types are checked. Equality is structural across the value sum; unequal variants compare false.

Values are extensional, finite immutable K terms: UInt128, Bool, lists of UInt128, and binary trees with UInt128 leaves. Recursive list/tree constructors have dedicated sorts, preventing lists containing trees and other ill-shaped combinations. Patterns are variable, nil, cons(head,tail), leaf(value), branch(left,right); nested patterns are expressed by nested matches. Cases advance in source order. A failed match yields a defined result code. Generic nominal datatypes, records, polymorphism and higher-order closures are not modeled.

## Fuel and numeric meaning

Startup and terminal publication cost zero. Every machine rule between the machine-section comment and pure-helper-section comment decrements strictly positive fuel by one. This includes scheduling a frame push, performing the push, operand return, binding/lookup stages and match-case stages. A final value with empty stack succeeds even at zero fuel; an explicit error publishes even at zero. Any other control at zero fuel publishes `uiExhausted()`. Publication stops computation. Depth increases only on a successful push and decreases on pop; let-frame replacement preserves depth.

Pure helpers structurally recurse over finite program/data/list terms. Their internal K rewrites are deliberately not semantic fuel events. Thus finite fuel bounds machine transitions; it does not alone establish a host time/memory bound, circuit row bound or constant helper cost. Input node/depth/byte limits and metered helper refinement remain unimplemented. K Int supplies mathematical intermediate integers; UInt values and checked add/sub/mul results must be in `[0,2^128)`. No modular wraparound, circuit field arithmetic, division or rounding is modeled.

Result codes are 1 initial negative fuel/stack limit; 2 invalid UInt literal; 3 unbound variable; 4 stack limit; 5 non-Bool conditional; 6 result type mismatch; 7 missing function; 8 argument binding mismatch; 9 exhausted match cases; 10 arithmetic range; 11 primitive operand type mismatch. These describe the fragment, not Moriarty's frozen diagnostic ABI. Rejection/exhaustion retain diagnostic machine cells; only `<out>` is the terminal result, and no financial state exists.

## Explicitly unmodeled

Financial writes/effects, funding/debit/fee accounting, intents, policies, mandatory proof claims, residual duties and financial acceptance are UNMODELED. Durable checkpoints, yield/await/event handling, nested suspension, accepted-event observations, continuation serialization, resume binding, multi-segment work accounting and atomic financial commit/rejection are UNMODELED. Requirements remain pending. Fuel exhaustion is a terminal result, not suspension.

There is no addressed heap, pointer identity, cycles, cryptographic digest, circuit constraint system, code commitment or native proof integration. A hash-consed or bounded addressed implementation needs its own representation relation and witnesses. The draft neither changes nor corresponds by assertion to existing Core/4 K experiments, Rust Core/2, Source, ZKIR or ledger semantics. No language/version acceptance gate is closed.

Later evidence should distinguish ordinary frontend source checking, complete-state concrete execution witnesses, exact hand-counted fuel witnesses, bounded abstraction checks and actual metatheoretic/cross-layer proofs. The present source and normal expected terms supply only a reviewable candidate.
