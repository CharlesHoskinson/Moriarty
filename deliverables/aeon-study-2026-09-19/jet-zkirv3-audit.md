# Bounded audit: Simplicity-style jets and a certified ZKIRv3 target

Research date: 2026-09-19. No edits to the original Moriarty branch. Source inspection only: no Coq proof build, native tests, circuit synthesis or ledger execution was performed.

Pins: recovered Moriarty `/home/charl/Moriarty` HEAD `84f64b30df53ff0b8ccaf475f57324d3d7c6e594`; Simplicity `/home/charl/zk-langs/simplicity` HEAD `91d89bac54e9411281ee3b6e766a0ea20eb0a3d3`; local Midnight ledger `/home/charl/midnight/midnight-ledger` HEAD `3fa0d1d15a3cfabd41c806546a006b1ce406b7b2`. The latter is a local source pin, not a claim of current deployed Preview alignment. Moriarty status was refreshed read-only; it reports internal campaign resource gaps and no pending transactions.

## Recommendation

Make Midnight ZKIRv3 the explicit required target. Adopt the **jet discipline**—a named reference meaning, an optimized implementation and a checked equivalence contract—as a way to build a certified basis. Do not promise that adopting Simplicity terminology transfers an existing proof to Moriarty, or that reducing the constructor count to fifteen proves the compiler. Preserve the full observable financial semantics and discharge gadget preconditions at composition sites.

## Corrections to the recovered note

### 1. “Nothing is trusted” is unsupported

`wiki/language/jet-discipline.md`, CLM-0972, correctly points to Simplicity `Coq/Simplicity/Primitive.v:247–251`: the jet's denotational interpretation is defined by evaluating its supplied reference expression. But `jet_Parametric` at :239–244 proves a property under a relation class whose mixin already includes the jet preservation condition (:208–215). Neither statement establishes arbitrary machine code equality.

The implementation boundary is visible: `Haskell/Core/Simplicity/CoreJets.hs:2683–2700` matches specification identity hashes and checks types; :2768–2772 selects a separate native implementation. `C/eval.c:316–317` calls the jet function; `C/jets.c` contains that native code. `Coq/Simplicity/Word.v:482–517` has actual arithmetic semantic lemmas, but the inspected material does not establish exhaustive native-code verification or the host compiler/runtime's correctness.

**Required wording:** jet specifications have reference semantics; each adopted implementation needs its own proof or explicitly recorded trust assumption. Hash matching binds identity, not correctness of the code reached through that identity. Proof checker, cryptography, serialization, compiler and runtime remain named dependencies.

### 2. “The Outstanding proof already exists” confuses an executable invariant with a substitution theorem

The recovered source checks `principal + accrued = outstanding` in `financial-lifecycle.ts:854–859`; accrual updates both fields with checked arithmetic at :1791–1825. This supports a conditional value identity on validated reachable states. It is not a mechanized theorem that replaces a financial read with an `Add` expression while preserving all observable behavior.

`financial-expression-v1.ts:493–500` binds a post-state and builds maps without re-admission; :589–592 accepts that post-state at the suffix boundary. Production callers at :728 and :801 pass a prepared kernel result, so the absence of revalidation is not by itself proof of an externally exploitable bypass. It does identify a provenance hypothesis the equivalence theorem must state and prove at every caller.

**Required theorem scope:** validated state and unit binding; lookup success/failure; exact arithmetic domain; evaluation order; work/exhaustion behavior; errors; immutable PRE versus selected POST. A value identity alone does not justify a transparent rewrite.

### 3. Constant jet cost is not the only solution, and can be wrong

The note accurately identifies observable work: `financial-expression-v1.ts:365` charges each entered node; :368–374 implements selective evaluation; :517/:522 returns remaining work. Expanding `And` into `Select` plus a false literal costs an extra reduction on the short-circuit path. A jet that returns the same Boolean may therefore change `WORK_EXHAUSTED` and the published work balance.

However, declaring an arbitrary constant cost does not prove equivalence. A reference expression can have input-dependent costs and different failure prefixes. For an unchanged profile, either preserve the exact logical source-cost trace while accelerating physical execution, or prove equality of the observation relation including charge/failure behavior. A new charging policy needs an explicit version change and requalification, not a transparent optimization label.

The note's contrast with Simplicity also needs scope: the tiny pure denotation has no work balance, but the implementation has cost-based admission. `C/eval.c:568–588` documents CPU budget checks and :718–720 assigns jet-specific cost; :742–743 rejects outside cost limits. Value denotation, committed representation and execution-budget behavior are distinct even there.

### 4. “No backend is named” was already too broad and is superseded

CLM-0984/0993 claims no bit/circuit backend is named. Yet recovered `wiki/decision.md:33` names Compact/ZKIR3 and :39 records early emitted circuit models; `wiki/log.md:188` and :262 also record ZKIR3 experiments. Those historical records do not prove a certified current lowering, but they refute the unrestricted absence claim. The latest user instruction now makes ZKIRv3 mandatory.

A jet needs a precise reference denotation, not necessarily a pre-existing bit-blasted source language. It can relate an exact bounded mathematical operation to a ZKIRv3 gadget once encodings and failure semantics are defined. The missing capability is the certified relation and coverage, not permission to choose a backend.

### 5. “Roughly fifteen primitives” is a research candidate, not a completed basis

Simplicity `Core.v:5–32` has nine typed structural constructors and total evaluation. Failures/assertions, primitives, encodings, native jets and runtime budgets live outside those lines. Counting that file against all financial source constructs conflates abstraction levels.

Moriarty dimensional arithmetic, partial lookup/projection, errors, effect construction, observable work, scope phases, canonical codecs and signed commitments still need semantics even if fewer AST constructors represent them. Combining `Lit` variants changes neither the number of range obligations nor the need to prove serialization faithful. Calling output construction “outside Core” relocates its proof burden; it does not remove it.

### 6. Syntactic scale restrictions are not automatically an obstruction

`financial-expression-v1.ts:347` requires a literal power-of-ten denominator for scaled division. That is a real syntactic static rule. But an optimization performed after original typechecking can retain its certified derivation; there is no need to weaken the public type system merely so jet syntax passes it. An explicitly typed internal constant or proof-carrying rewrite can preserve the rule's conclusion. Semantic equality alone is insufficient if phase, source diagnostic or commitment behavior changes.

### 7. The historical K mismatch remains a coverage limit

At the recovered pin, `financial-expression-v1.ts:222` admits UInt256 while `formal/k/expression-infer.k:126` restricts UInt literals to 64/128 and `expression-types.k:18–21` lacks the extended range families. This establishes different implemented domains, not necessarily an unsound same-domain theorem: the existing reports already qualify the narrower K scope. The certified basis must track constructor/type-domain coverage and reject promotion beyond it. Reuse old evidence only within its exact domain.

## ZKIRv3 composition obligations

The local target defines `Assert`/`CondSelect` with Boolean preconditions in `zkir-v3/src/ir.rs:334–356` and exposes bit-range/equality constraints at :360–378. `Add` and `Mul` operate over target field/group types (:646–675), which are not automatically Moriarty checked integers. `Inv` (:693–704) is field inversion, not integer floor division. These API facts are enough to identify obligations; they are not proof that constraints are correctly synthesized in this pin.

For each supported gadget record:

1. Reference operation and semantic-profile hash; exact input/output types and units.
2. Canonical encoding and decoding, range/Boolean/sign/carry conditions, and required state invariants.
3. Complete success and rejection semantics, including zero denominators, overflow and branch behavior.
4. Exact ZKIRv3 instruction sequence or generator version, target library and proof-system parameters.
5. Soundness: any satisfying assignment under stated preconditions decodes to an allowed reference result/effect. Constraints must not admit an alternative malicious witness.
6. Completeness for the supported domain: valid source behavior has a target witness, preventing an always-rejecting “correct” compiler.
7. Logical work accounting and separate physical circuit/prover resource bounds.
8. A caller obligation establishing every precondition and constraining/consuming every correctness-relevant output.

Composition requires an actual theorem relating output encodings and invariants to the next gadget's inputs, plus frame/effect noninterference and witness/public-input bindings. A field equality only establishes integer equality with sufficient range/no-wrap constraints. A helper computing a comparison bit does not enforce the comparison unless the caller constrains its required value. A correct witness generator does not exclude another witness that an underconstrained circuit accepts.

For short-circuit operations, a target circuit cannot blindly evaluate and enforce an invalid dead branch. It needs a proved guarded relation and witness construction respecting inactive branches, while preserving the source's selected-branch work policy. “Select” as an opcode name is not that proof.

## Bounded delivery recommendation

**First slice:** freeze the full observable relation and certification metadata; prove one checked unsigned addition and one conditional selection from the supported Moriarty domain to exact ZKIRv3 constraints. Include overflow, unconstrained carry/Boolean, witness mutation, dead-branch error and work-exhaustion controls. Retain one genuine positive witness. No reduction of all sixty source constructs is required to begin.

**Next:** add typed scalar conversion and floor/ceil division with explicit quotient/remainder bounds; then validated financial reads. Delay invariant-dependent `Outstanding` substitution until reachable-state and post-state provenance obligations are established. Do not hide cryptographic gadgets under generic primitive declarations; list their trust/proof coverage separately.

**Integration:** connect certified fragments through the actual Compact/ZKIRv3 generation route selected for Moriarty. If Compact is an intermediate compiler, its transformation and generated artifacts remain part of the correspondence obligation. Preserve exact source/profile/program/intent and complete effect binding through the actual Midnight verifier; local target instructions alone do not establish ledger acceptance.

**Permissionlessness:** certification is technical evidence for a semantic rule or artifact, not a committee granting developers deployment licenses. A new application composed from supported certified rules must be validated by objective rules without named model reviewers, RP03 or a central program allowlist.

Suggested panel conditions: endorse mandatory ZKIRv3 and a certified reference/optimized basis; reject “nothing trusted,” “proof already exists” and “fifteen obligations settle the compiler”; require observable work and caller-precondition coverage; preserve private-state, financial and actual proof-to-ledger obligations. This recommendation is specified-only and makes no claim of a built certified backend.
