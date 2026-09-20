I will deliver the advisory proposal directly as text, since you asked for no tool use.

## Position and challenge to the initial recommendation

I agree with the initial author's boundary: borrow concepts, keep Moriarty's bounded exact semantics and independent Preview acceptance authoritative, and never let a solver result stand in for a Midnight proof. I disagree on emphasis and on one option.

First, the research leads with synthesis. The reproduced evidence for synthesis is one constant, 97, found by SMT literal completion. That is not a signal that synthesis is the valuable import. The valuable import is the verification-condition discipline: an obligation, a declared domain, and a four-way outcome. Moriarty already enforces PRE and POST at runtime with atomic rejection and has finite Source, Core, oracle and K agreement. What it lacks is any compile-time statement that an obligation holds for all admitted inputs. That gap is the prerequisite for holes, counterexamples and trust reports alike, so it must come first and the others follow it.

Second, the research keeps "Aeon as an external proposal engine" open as a plausible approach. I would close it now. It imports a Python trust frontier, an unrefined-native blind spot, a 200 millisecond solver timeout collapsed into a Boolean, and a source correspondence obligation between two languages, in exchange for a proposal engine that a two-page typed grammar over Moriarty's own pure constructors can replace.

Third, the research understates how much is already present. Postconditions, typed PRE reads, source-defined schemas, static action bounds, the runtime work ledger, closure reserve, overflow rejection under domain guards and a 104-case lifecycle corpus with native K agreement all exist at the pinned commit. Any Aeon feature whose Moriarty value is "totality" or "work bounds" is already delivered by different means and should not be re-implemented.

## Evidence classes that must stay separate

- **Compile-time theorem.** A checker verdict of Valid for a named obligation over a declared finite domain, under the assumption that the SMT encoding matches the 48-constructor runtime. It says nothing about kernel effects, authority, K or the ledger.
- **Runtime enforcement.** The existing PRE and POST checks, atomic rejection with typed codes and the work ledger. These remain mandatory regardless of any static verdict until MC04 correspondence exists.
- **Finite tests.** The lifecycle corpus, the 682 language tests, the K differential runs and any new checker-versus-evaluator differential corpus. Passing tests bound confidence; they do not generalize.
- **Ledger evidence.** Preview transactions, finality, readback and independent audits. Nothing in this proposal produces, replaces or reinterprets any of it. SP05, SP09, SP11, SP12, MC and PCD obligations are untouched.

## Feature classification

| Feature | Class | Present in Moriarty | Genuinely missing | Rationale |
| --- | --- | --- | --- | --- |
| Refinements and VCs | NOW | Runtime PRE/POST, finite corpus | Static obligation discharge over all admitted inputs | Highest value, no semantic change, off the Preview critical path |
| Trust reports | NOW | Scattered assumption prose in results | Per-artifact machine-readable assumption inventory | Cheap, directly supports audit discipline already practised |
| Counterexamples and IDE | NEXT | Typed rejection codes with spans | Solver witness rendered as an executable snapshot | Depends on the checker; witness must round-trip through the evaluator |
| Linear capabilities and typestate | NEXT | Kernel uniqueness and ledger consumption at runtime | Static advisory lint for double-consumed funding or authority | Diagnostics only; runtime and ledger checks remain authoritative |
| Typed holes and restricted synthesis | NEXT, after checker | None | Holes in draft source completed into ordinary `.mori` | Only valuable once validation is a real theorem, not a test |
| Totality and work bounds | DEFER | Finite bounds, static work, runtime ledger, closure reserve | Nothing Aeon adds | Aeon's termination metric is optional and trivially true when absent |
| Qualifier inference | DEFER | Not applicable | Annotation-burden relief | Revisit only if measured annotation cost is a real obstacle |
| Optimization and fitness | DEFER | Not applicable | Ranking among already Valid candidates | Fees, limits and signed intent can never be fitness terms |
| General ADTs and polymorphism | DEFER | Bounded exact widths, units, scale | Not wanted in the financial kernel | Conflicts with bounded semantics; Aeon's Horn lifting still skips polymorphic binders |
| GP and LLM proposals | DEFER | None | Proposal engines | Measure failures of enumeration on a financial corpus first |
| FFI and native bindings | REJECT | Deliberately absent | Not a capability, a hole in the trust frontier | Native annotations are promises; unsuitable as financial safety evidence |
| Aeon as backend or substrate | REJECT | Not applicable | Not applicable | Largest migration and proof burden; contradicts the Midnight release gate |

## Ordered roadmap

**A1, static obligation checker (NOW).** Scope: verification-condition generation for the 48 pure financial constructors and for PRE and POST obligations of source-defined actions, discharged by SMT over a documented fragment. Fragment: quantifier-free linear integer arithmetic with exact UInt width side conditions, plus multiplication and division where one operand is a literal or a declared scale constant. Every obligation returns exactly one of Valid, Counterexample, Unknown or Unsupported. Prerequisites: the reviewed 40-constructor expression contract and the reviewed financial pure-expression runtime, both present. Falsifiable acceptance: a differential corpus in which every Counterexample reproduces the documented evaluator rejection code, no corpus case that the evaluator accepts is reported Counterexample, and a mutation control that alters one encoding rule in the checker is detected by the corpus. Non-goals: no effect-level obligations, no kernel or authority encoding, no change to the evaluator, no removal of any runtime check.

**A2, trust and assumption report (NOW, parallel).** Scope: a command that, for a source file, schema and snapshot, lists every assumption class with a status of Verified, Tested or Assumed, with a pointer to the retained evidence. Classes: schema, snapshot, oracle inputs, kernel version, compiler and profile binding, K agreement, proof and ledger correspondence. Acceptance: a class missing from the report is a report failure, and the report for the September 17 lifecycle example must show ledger correspondence as Assumed. Non-goals: it does not compute or infer trust; it renders declared status.

**A3, counterexample diagnostics (NEXT).** Scope: render the A1 witness as a concrete snapshot and source span in the check command. Acceptance: running the simulation CLI on the emitted snapshot produces the same rejection. Prerequisite: A1 accepted.

**A4, linear and typestate lint (NEXT).** Scope: a static pass over multi-action source that flags a funding or authorization capability used more than once on any path, with branch usage comparison as in Aeon's linearity pass. Acceptance: the lint flags a deliberately duplicated funding use in the lifecycle corpus and stays silent on all accepted corpus cases. Non-goal: no claim about replay, distributed uniqueness or ledger consumption.

**A5, typed holes with restricted synthesis (NEXT, after A1 and A3).** Scope: a draft-only hole syntax in a separate file profile, a grammar containing only in-scope values and pure constructors, enumeration ranked by term size, and A1 as the sole validator. Output is ordinary `.mori` source that then passes through the unchanged check and evaluate commands. Acceptance: on a corpus of at least twenty fee-inclusive repayment tasks with independent expected bodies, every accepted candidate is Valid under A1 and equivalent to the expected body on the corpus. Non-goals: no effectful synthesis, no new constructors, no fitness on fees or limits, no GP or LLM backends.

Everything in DEFER has no scheduled work. Everything in REJECT gets a written rejection record so it is not relitigated.

## EARS requirements

- **AEO-REQ-01.** The static obligation checker shall report exactly one of Valid, Counterexample, Unknown or Unsupported per obligation, and shall never map Unknown or Unsupported to either Valid or Counterexample.
- **AEO-REQ-02.** When the checker reports Counterexample, it shall emit a snapshot that, when evaluated by the existing evaluator, produces the documented rejection code for that obligation.
- **AEO-REQ-03.** The checker shall encode the exact UInt widths, overflow rejection, scale and rounding of the financial pure-expression runtime, and a mutation control on any single encoding rule shall be detected by the differential corpus.
- **AEO-REQ-04.** Where the checker reports Valid, the runtime enforcement of that obligation shall remain in place until MC04 compiler and ledger correspondence is accepted.
- **AEO-REQ-05.** While a source file contains an unresolved hole, the check command shall reject admission with a typed diagnostic.
- **AEO-REQ-06.** If a synthesized candidate satisfies any fitness measure but fails validation, then the tool shall discard the candidate and record the failure.
- **AEO-REQ-07.** The trust report shall list every declared assumption class for an artifact with a status of Verified, Tested or Assumed and an evidence pointer, and an absent class shall be a report failure.
- **AEO-REQ-08.** The synthesis tool shall emit only ordinary source in an existing profile and shall not extend the grammar, runtime or lowering.

## Foreman Pel delivery controls

Each of A1 through A5 is one Pel task from an approved OpenSpec change artifact. The implementer role produces a candidate in an isolated worktree on the immutable base. Permitted write paths are limited to the language experiment source, its tests and a new deliverables directory. The `candidate-full` gate runs the existing 682 language tests plus the new differential corpus and mutation control. The reviewer role reviews independently under the standard policy. No task carries publication authority, touches the ledger plugin, dispatches a campaign, edits any MC or SP checkbox, or writes to Preview evidence directories. A run that exits with an unresolved action is not a delivery.

## Three highest risks

1. **A third semantics.** The SMT encoding, the evaluator and K can drift. Mitigation is the differential corpus with mutation control and the rule that only the evaluator's rejection codes count.
2. **Verdict inflation.** A Valid verdict gets cited as ledger correctness or used to argue an SP gate closed. Mitigation is AEO-REQ-04, the trust report showing ledger correspondence as Assumed, and no roadmap edits from these tasks.
3. **Nondeterministic Unknown.** Solver timeouts make check results flap between Valid and Unknown across machines, breaking reproducible review. Mitigation is a pinned solver, a generous fixed budget, and treating Unknown as a warning that never gates admission.

## Strongest counter-position

The hard release gate is end-to-end Preview settlement. Moriarty already has runtime postconditions, a finite corpus and K agreement. A compile-time checker adds a new component to keep aligned, delays task 5.x work on authenticated Docker and Preview, and its Valid verdicts will be misread. Spend the capacity on SP05 and SP09 instead. I take this seriously. My answer is that A1 and A2 are pure, off the Preview path, consume no admitted attempts and are validated against what already exists. But if implementer capacity is a single track, the counter-position wins and this whole roadmap becomes NEXT behind task 5.4.

## What would change my opinion

- If more than about a third of corpus obligations fall outside the declared fragment because interest and rate arithmetic requires variable-by-variable multiplication at wide widths, the checker degrades to bounded model checking and its value drops. I would then move A1 to DEFER and keep only A2.
- If the differential corpus finds the evaluator and the encoding disagree on a rounding or overflow case that is not a checker bug, that is a finding about the runtime and must be routed to SP02, not patched in the encoding.
- If a reviewed correspondence relation between source, kernel effects and Compact lowering arrives earlier than expected, effectful obligations and synthesis move up.

## Conditions for endorsing a shared plan

I would endorse a consensus plan if it states that no Aeon code is imported, that A1 precedes any hole or synthesis work, that all eight requirement IDs above or equivalents are carried into an OpenSpec change with falsifiable acceptance, that the four evidence classes are named separately in every result, that runtime enforcement is never removed on the strength of a static verdict, that the REJECT items have written records, and that no task in the plan edits, closes or reinterprets any existing MC, SP, PCD, correspondence or Preview obligation. This is research advice and not a release gate or a Council runtime receipt.