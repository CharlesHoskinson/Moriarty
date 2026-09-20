# A1 — Refinement types, decidability and semantic soundness

Independent advisory proposal, based only on the supplied pinned evidence. No implementation, release approval, or formal Council receipt is claimed.

My recommendation is **a Moriarty-native, advisory verification slice before synthesis**. Borrow Aeon's separation of goals, contexts and assumptions, but do not adopt its verifier as the source of Moriarty guarantees. The initial research correctly rejects substrate replacement; it moves too quickly from one constant-completion example to a synthesis priority. Existing source expressions, financial PRE reads, atomic postconditions, funded repayment, origination, accrual and a complete local lifecycle already supply much of the authoring substrate. The missing increment is a precise static judgment and its trustworthy explanation, not another financial evaluator.

The roadmap records actual Preview loan and swap results, including a later fixed-loan success. Preserve those scoped achievements and their gaps. They do not establish the newer source lifecycle, mandatory PCD, general correspondence or full SP05. The supplied task checklist and roadmap disagree about K progress: reconcile against exact retained results, without either erasing native finite conformance or claiming a theorem from it.

## The judgment to specify first

For profile `P`, typed context `Γ`, expression `e`, input predicate `A` and result predicate `Q`, specify two different judgments:

1. Successful-evaluation safety: every admitted input satisfying `A` that evaluates successfully produces a value satisfying `Q`.
2. Guaranteed defined execution: every admitted input satisfying `A` evaluates successfully within its stated work allowance and satisfies `Q`.

The first must not be displayed as the second. A program that always rejects satisfies the first vacuously. Report precondition feasibility separately; an inconsistent assumption set can prove any postcondition. A static condition about unauthenticated snapshot values remains conditional when execution later authenticates or refreshes them.

Start with a finite, quantifier-free arithmetic fragment over exact widths, with explicit guards and rejection results. Checked addition is not modular addition; wider intermediates, scaling, division, signedness and rounding must follow the existing profile. Encode short-circuit and branch evaluation conditions so dead subexpressions neither invent failures nor hide reachable ones. Exclude operations whose semantics are not faithfully represented. Finite bitvectors make satisfiability decidable in principle, not cheap or guaranteed to complete under a time limit.

A checked `unsat` answer establishes an encoding-level implication subject to the encoder and solver. Calling it a theorem about evaluator behavior additionally needs a stated and discharged encoding adequacy argument. Until then, label it a solver-backed static check, keep runtime checks, and expose dependencies. Independent tests challenge that argument but cannot discharge it universally.

## Feature decisions

| Feature | Decision, exact scope, acceptance and non-goal |
|---|---|
| Refinements / VCs | **NOW:** specify and implement an advisory expression-only VC slice over existing constructors, after fixing the judgment and semantic mapping. Require reviewed per-constructor rules, boundary and rejection controls, and separate definedness/feasibility results. No arbitrary reflection, new financial transition semantics or runtime-check elimination. |
| Trust reports | **NOW:** artifact/action-specific assumption and evidence manifest, including omissions and unsupported dependencies. Include solver/encoder, compiler/profile, schema, state authentication, oracle, native/proof/ledger correspondence and freshness. An injected missing dependency must prevent a completeness claim. This is not a proof that the inventory is complete. |
| Counterexamples / IDE | **NOW** for structured CLI diagnostics and replay; **NEXT** for LSP presentation. Replay a solver model through the pinned evaluator before calling it an executable counterexample. Distinguish encoding-only model, replay mismatch and unavailable model. No automatic claim of a feasible ledger attack. |
| Typed holes / restricted synthesis | **NEXT**, conditional on the VC slice and useful unmet tasks: external draft representation, literals plus a tiny pure typed grammar, explicit input domain and component allowlist. Materialize ordinary `.mori`, then independently check the complete result and rerun relevant obligations. Reject unresolved holes. No authority primitives, effects, candidate-supplied assumptions or weakened specifications. |
| Linear capabilities / typestate | **DEFER** new language machinery; **NEXT** only targeted diagnostics over existing resource and lifecycle rules if duplicate-use mistakes are demonstrated. Require a precise distinction between affine permissions and non-discardable financial duties before new types. Branch-use tests and authenticated replay/conflict tests have different scopes. No claim of distributed uniqueness from local linearity. |
| Totality / work bounds | **NOW:** preserve and expose existing finite bounds, work ledger and closure reserve in reports. Recheck after candidate generation. Bound exhaustion and obligation preservation remain runtime requirements. Accept only versioned bounds that retain cumulative limits across continuation; do not import optional decreasing metrics or general recursion. |
| Qualifier inference | **DEFER:** a finite approved predicate vocabulary may later suggest annotations, always rechecked against the same immutable obligation. Require measured annotation burden and withheld-task improvement. No silent introduction of assumptions, quantified inference or inference-driven weakening of input domains. |
| Optimization | **DEFER:** rank already admitted candidates by a declared static metric, preserving all hard constraints and bounded search. Require equivalent specified behavior and rechecked work bounds. No replacement of fee limits or intent with fitness; no claim of optimality or actual proving cost from AST size. |
| General ADTs / polymorphism | **DEFER:** no new general mechanism justified by this evidence. Reuse existing closed schemas; require a concrete missing financial behavior, canonical encoding, bounded size, matched K/Compact semantics and correspondence scope before extension. Do not infer sound polymorphic refinement support from Aeon's feature list. |
| FFI | **REJECT** unchecked native bindings in the financial semantic kernel or trusted refinement path. Host adapters remain existing explicit trust boundaries, separately inventoried and constrained. No opaque implementation may acquire verified status from an annotation. |
| GP / LLM proposals | **DEFER** as untrusted off-chain candidate generators until a narrow baseline demonstrably fails useful tasks. Any later engine uses the same whole-artifact admission and budget; malformed, effectful and specification-mutating outputs must fail closed. No execution authority or proof standing for generated text. |
| Backend adoption | **REJECT** Aeon as financial substrate, Python execution backend or authority. **DEFER** an external Aeon proposer until native baseline evidence justifies translation cost. An experiment would require a checked translation into ordinary Moriarty plus explicit cross-language semantic obligations; it supplies no Midnight evidence. |

## Ordered OpenSpec and Pel delivery

**1. Reconcile and specify.** Create a small proposed OpenSpec change, `moriarty-static-obligations`, cross-referenced to existing MC01/SP02 and MC08 developer-flow requirements. Preserve MC/SP identities, historical receipts, financial corpus obligations and current ownership. Freeze the two judgments, operation subset, result schema, assumptions and independent arithmetic fixtures. An authoring add-on must not delay already admitted lifecycle work or become a substitute for it.

**2. Deliver the advisory checker and evidence report.** Implement only that subset through the existing frontend. Verification includes exact-width overflow, zero divisors, rounding, intermediate bounds, branch reachability, inconsistent assumptions, stale artifact binding and deliberately incorrect encodings. Compare tiny exhaustive domains and full-width adversarial cases with an independent expected-result source; retain finite-evidence labels. A replay mismatch blocks that supported operation until resolved. The deliverable is useful even if synthesis is never adopted.

**3. Trial draft completion.** Predeclare a small withheld corpus of practical pure financial expressions, baseline authoring effort, rejection criteria and search budgets. Require a parameterized case over its complete declared domain; reproducing `97` is insufficient. Continue only if independently checked successful completions improve the declared baseline without narrowing specifications. Keep effectful synthesis outside this change.

For each implementation package, Pel consumes the approved immutable OpenSpec artifact in an isolated worktree. Pin repository/base, permitted paths, provider identities, transport qualification, environment, limits and the full `candidate-full` command. Use implement → host verify → independent review; a repair changes the candidate and requires verification and a fresh candidate-bound review. Retain hashes, raw failures and unresolved actions. Pel parsing success is not admission, and review is not publication authorization. No reviewer substitution follows automatically from availability. This advisory discussion does not dispatch Pel or a financial campaign. Existing RP03, Docker-before-Preview and exact-candidate requirements remain applicable to their separate execution stages.

## Proposed EARS requirements

- **AEON-VC-001:** When a static obligation is requested, the checker shall bind its judgment, predicate, complete input domain, profile, source and compiler hashes, arithmetic rules and assumptions in the result.
- **AEON-VC-002:** When an obligation completes, the checker shall distinguish established encoding implication, validated counterexample, unsupported fragment, timeout, unknown and checker error; only the first shall receive an established status, with its trust qualification.
- **AEON-VC-003:** If input assumptions are inconsistent or feasibility is unestablished, the checker shall expose that status and shall not present successful-evaluation safety as guaranteed defined execution.
- **AEON-TRUST-001:** When a report is rendered, the tool shall separate static claims, runtime enforcement, finite test evidence and ledger evidence, and shall identify incomplete dependency coverage.
- **AEON-CEX-001:** When a model is presented as an executable counterexample, the tool shall retain successful replay against the bound evaluator and the violated obligation; otherwise it shall label the model unvalidated.
- **AEON-SYN-001:** When a candidate is proposed, the authoring tool shall preserve the frozen specification, emit hole-free ordinary source and require whole-program checking, supported static obligations and applicable bounds before acceptance.
- **AEON-BOUND-001:** While an authoring extension is enabled, the admitted profile shall retain its runtime checks, exact rejection behavior, cumulative work limits, closure reserve and persistent financial obligations.
- **AEON-EVID-001:** When delivery status is updated, the register shall close only requirements supported by current candidate-bound evidence and shall retain all outstanding financial, PCD, correspondence and actual Preview acceptance requirements.

Each requirement needs positive and negative scenarios in OpenSpec; unavailable evidence leaves a requirement open.

## Risks, dissent and endorsement

The three largest risks are **semantic mismatch** between exact checked arithmetic and SMT; **vacuous or mis-scoped assurance** from inconsistent assumptions, runtime rejection or a single snapshot; and **delivery displacement**, where attractive authoring work obscures PCD and real settlement obligations.

The strongest counter-position is to defer even the checker and complete correspondence and mandatory ledger acceptance first: runtime postconditions already catch concrete failures, while another encoder enlarges the trusted base. I would adopt that position if no repeated authoring errors justify the slice, if constructor mapping cannot be reviewed independently, or if the work competes for the same scarce delivery capacity. Conversely, evidence of frequent preventable arithmetic mistakes and a narrowly adequate encoding would justify the proposed order. Demonstrated native-baseline failure on valuable held-out tasks could justify an external proposer later.

I can endorse a shared plan that keeps the checker advisory until semantic adequacy is established, makes synthesis conditional and pure, preserves runtime and ledger authority, reconciles existing achievements honestly, binds every claim to its evidence class, and leaves all current financial/PCD/Preview gates intact. I cannot endorse presenting six advisers' agreement as technical validation or release acceptance.
