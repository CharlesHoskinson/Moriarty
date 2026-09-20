# Independent proposal A3: resources, financial effects and delivery

Advisory research based only on the supplied evidence packet. No implementation, fresh validation, release approval or Council runtime receipt is claimed.

My recommendation is to borrow Aeon's obligation presentation and restricted checking concepts, but put effect semantics and evidence classification before synthesis. Moriarty's difficult resource is not a variable consumed once: it is persistent financial authority and liability carried across authenticated state transitions, failure, restart and settlement. A language feature earns priority when it exposes or preserves that boundary.

## What exists, and what is missing

The supplied roadmap reports implemented funded repayment, source schemas, named actions, PRE reads, atomic financial postconditions, funded origination, bounded accrual and the complete local loan lifecycle. Native K execution reportedly matches 125 expression cases, 104 lifecycle cases and six counter probes. These are meaningful implementations and finite conformance evidence, not a source/Core/K metatheorem. The older task checklist still demands recovery of an unavailable original binary; reconcile that wording with the newer recovered-source execution without claiming exact historical reproduction.

Historical Preview loan and swap operations, including corrected runs, also exist. They must retain their precise lineage, accounting and process-exit limitations. Authenticated Docker/Preview execution of the newer source-defined lifecycle is still open. Neither those historical operations nor new arithmetic checks establish mandatory PCD, compiler-to-ledger correspondence, private composition or full ACTUS/DeFi coverage.

The missing authoring capability is a versioned account of what an expression or action can establish, what resources it requires, what assumptions it trusts and what evidence supports it. Do not redescribe existing runtime postconditions as an absent refinement system and rebuild them under a new name.

## Feature decisions

Each acceptance criterion below is proposed and falsifiable, not an observed result.

| Feature | Decision, exact scope and prerequisite | Acceptance and non-goal |
|---|---|---|
| Refinements/VCs | **NOW**, design and bounded implementation of pure arithmetic obligations over one existing profile. Pin width, denomination, scale, rounding, overflow, partial operations and path guards first. | Boundary cases and deliberately wrong encodings must expose overflow, underflow, zero division and rounding differences; a solver result has explicit assumptions. No effect-level or ledger theorem follows. |
| Typed holes/restricted synthesis | **NEXT**, one draft-only pure expression hole, finite approved grammar, closed effect-free context, explicit term/search bounds. Requires the exact checker and corpus below. | Every returned source passes the ordinary full source checker and independent semantic checks; no solution and timeout remain outcomes. No holes in admitted source; no authority acquisition, state writes or effectful synthesis. |
| Trust reports | **NOW**, per-action/artifact inventory covering native operations, compiler/profile, solver encoding, snapshots, oracle/time/authority and proof/backend dependencies, including unrefined dependencies. | Injected unrefined native and stale observation dependencies appear; unknown dependency classification prevents a completeness claim. Report is an inventory, not verified trustworthiness. |
| Counterexamples/IDE | **NOW** structured CLI results and replayable witnesses; **NEXT** editor display using the same format. Depends on source-span and solver-variable mapping. | SAT witness replays under exact semantics or is explicitly marked an encoding discrepancy. UNKNOWN/timeout/unsupported never render as a counterexample. No bespoke LSP prerequisite. |
| Linear capabilities/typestate | **NOW** specify resource/effect signatures for existing protected operations; **NEXT** an optional static diagnostic pass over that subset. Requires explicit mint, consume, borrow, escape and failure rules. | Duplicate use, alias escape, branch misuse and dropped residual obligations are rejected; valid branch alternatives and partial payment remain expressible. No claim of distributed uniqueness or authenticated authority from typing. |
| Totality/work bounds | **NOW** expose and test preservation of existing bounds, work ledger and closure reserve. Additions require compositional cost rules. | Continuation/split/migration and failure probes cannot reset promised global work or strand closure. Search budget and solver time are separate. Do not import optional termination metrics as a replacement. |
| Qualifier inference | **DEFER**, until handwritten obligations show repeated annotation burden and the supported logic is stable. | Reconsider with a fixed qualifier corpus where inferred predicates pass explicit checking and improve coverage without hiding assumptions. No arbitrary predicate invention. |
| Optimization | **DEFER**, until enough valid candidates justify ranking. | Reconsider with deterministic ranking of independently valid candidates by existing static work/size metrics and unchanged financial predicates. No soft fees, liability limits or intent constraints; no circuit-cost claim from CPU fitness. |
| General ADTs/polymorphism | **DEFER** general facilities. Closed, bounded result/resource variants may be separately justified by actual lifecycle needs. | Require a concrete corpus blocker, versioned semantics, size rules and correspondence extension. No wholesale import of Aeon's general type system. |
| FFI | **REJECT** Aeon native/Python execution in the trusted financial kernel. Existing external boundaries remain explicit obligations. | Admitted source cannot gain a new opaque financial primitive through an annotation. Off-chain tools may propose ordinary source, without becoming execution authority. |
| GP/LLM proposals | **DEFER** as bounded untrusted source producers behind the same validator. | Reconsider only if restricted enumeration demonstrably fails useful tasks and measured yield justifies cost; malformed and malicious proposals must fail closed. No provider output as evidence of validity. |
| Backend adoption | **REJECT** Aeon as execution substrate or replacement financial evaluator. **DEFER** external proposal-engine experiments. | Reconsider only on a pinned comparative task corpus with exact translation validation and a smaller demonstrated maintenance/trust burden. No diversion of Midnight, PCD or correspondence obligations. |

## The resource boundary

Distinguish an affine permission, which may be unused, from a durable liability, which cannot silently disappear, and from a consumed ledger object, whose uniqueness depends on authenticated acceptance. A blanket “everything is linear” rule is wrong for optional authority, while an affine rule for debt is unsound.

For repayment, specify a transition relation connecting PRE state, authorized intent, funding, effects and POST state. It must track asset identity, recipient, gross debit, fee-inclusive net result, interest-before-principal allocation, residual debt, retained history and cumulative work. A local consumed handle can yield a successor handle carrying residual state; consuming the old handle is not discharging the underlying duty. Failure preserves the relevant pre-state under the existing atomic rules. Borrowing observational PRE data grants no spending authority.

The static judgment should summarize a relation over effects, not issue a broad “financially safe” type. Splitting resources requires an explicit conservation rule; private branches, shared-state interleaving and asynchronous settlement need different rules. Initially these unsupported extensions stay outside the new diagnostic subset and remain open roadmap obligations.

## Ordered delivery

1. **Package A: scope, obligations and evidence.** Create a proposed OpenSpec change that maps every new requirement to existing MC/SP identities. Reconcile local/K/historical Preview evidence and unfinished lifecycle delivery. Define the solver fragment, resource signatures and structured obligation/trust schema. Acceptance requires reviewable examples including partial repayment, atomic failure and stale authorization; do not wait for a new synthesizer.
2. **Package B: exact pure checker and CLI diagnostics.** Implement only one current numeric profile, with positive, negative, boundary and encoding-mutation controls. Preserve the existing evaluator as the execution authority. Track “proved relative to encoding” separately from correspondence evidence. The independently expected financial corpus must include more than the constant 97 example.
3. **Package C: restricted draft synthesis.** After B passes, add one-hole arithmetic completion into ordinary source. Measure useful task completion, search exhaustion and validation failures on a frozen corpus. Resource diagnostics may proceed separately once A's signatures are reviewed; they must not block the unchanged existing profile.
4. **Retained delivery track.** Continue already approved authenticated lifecycle, PCD, compiler/ledger and Preview work under their existing admissions. Effectful synthesis, broader type-system extensions and optimization need separate proposals after demonstrated need and reviewed semantic correspondence. Authoring improvements cannot close MC03–MC08 or the wider SP obligations.

For each implementation package, Pel should bind an immutable base, approved specification, bounded write paths, exact provider/transport qualifications, credential references and the full host verification command with its environment. Use isolated worktrees and the documented implement → verify → independent-review flow. Preserve immutable candidate and evidence references; any repair requires fresh verification and review of the resulting candidate. No model substitution, fixtures as transport qualification, or execution on null campaign IDs. A pending action or admission rejection is not delivery. Publication needs its own authority; an unknown external outcome requires reconciliation before retry. Stop at failed required controls or resource ceilings. The six advisers' consensus supplies design input, not those implementation receipts.

## Proposed EARS requirements

- **AE-MOR-001:** When an obligation is submitted, the authoring checker shall bind its result to the program hash, semantic profile, input domain, predicate, encoding version and explicit assumptions.
- **AE-MOR-002:** When solving ends, the checker shall report proved-relative-to-encoding, refuted, unknown, timeout, unsupported or error distinctly, and shall issue no success certificate for the latter four outcomes.
- **AE-MOR-003:** When a candidate replaces a draft hole, the tool shall recheck the complete resulting ordinary source, reject disallowed effects and unresolved holes, and retain the candidate's validation evidence before presenting it as admissible source.
- **AE-MOR-004:** When a dependency contributes to an action's behavior or claimed guarantee, the report shall identify its assumption class and evidence status, including opaque unrefined operations and external observations.
- **AE-MOR-005:** When an action partially discharges a liability, its transition shall preserve residual liability, remaining authority and cumulative work under the existing financial semantics; rejection shall satisfy the specified atomic rollback behavior.
- **AE-MOR-006:** When a capability-bearing action is checked, static acceptance shall require the defined resource-flow rules, while runtime and ledger acceptance shall independently enforce authenticated authority, currentness and unique consumption.
- **AE-MOR-007:** When a generated artifact is displayed or recorded, the tool shall distinguish compile-time claims, runtime checks, finite conformance results and finalized ledger evidence, retaining each claim's exact scope and lineage.
- **AE-MOR-008:** When a Pel candidate changes after verification or review, delivery shall require new candidate-bound verification and independent review without reusing the previous candidate's acceptance.

A compile-time theorem needs a stated domain and justified semantics; solver UNSAT alone establishes the encoded formula relative to the solver and encoding. Runtime rejection applies to executions actually checked. Finite tests sample behavior. Ledger records establish particular accepted transactions under their acceptance path. None upgrades itself into another category.

## Risks, dissent and endorsement

The three largest risks are arithmetic/effect encoding drift; conflating local resource use with durable authority or debt preservation; and spending delivery capacity on attractive synthesis while correspondence and authenticated lifecycle remain unfinished.

The strongest counter-position is to defer every new language mechanism and complete authenticated lifecycle/PCD first. I would accept that position if the team cannot fund an independently bounded authoring track. Conversely, a measured corpus showing arithmetic authoring errors dominate delivery time would justify accelerating Package B and then C. Evidence that resource diagnostics require broad semantics changes would move their implementation to DEFER; evidence of a narrow existing IR hook and real duplicate-use defects would support NEXT.

I can endorse a shared plan that leads with exact obligation/evidence reporting, places synthesis after its validator, preserves durable residual duties and all MC/SP/PCD/Preview requirements, names existing implementations accurately, and funds every accepted increment with candidate-bound Pel controls. Consensus must retain these acceptance conditions and unresolved disagreements, rather than count feature votes.
