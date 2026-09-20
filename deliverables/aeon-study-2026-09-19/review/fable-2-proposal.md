**Independent advisory proposal: Aeon concepts for Moriarty**

Scope of this note: research advice on which Aeon ideas to bring into Moriarty, in what order, with delivery controls. It is not a release gate, not a Council receipt, and does not touch any Preview, PCD, correspondence or financial obligation in ROADMAP.md, which I treat as fixed.

## Where I disagree with the initial author

The author's headline is "specification-guided synthesis plus trust reporting." I would invert the emphasis and narrow the first step further.

- **Synthesis is the least urgent item.** The reproduced experiment synthesized the constant 97. Moriarty authors already write the full loan lifecycle by hand with 682 passing tests. Holes have no validator worth trusting until Moriarty can discharge an obligation for all admitted inputs, which it cannot do today. Holes therefore depend on refinements, not the reverse.
- **The genuinely missing capability is compile-time discharge of financial obligations over all admitted inputs.** Today, postconditions are enforced at runtime with atomic rejection (PR5), and coverage comes from finite corpora and K agreement. Nothing in the repository proves a postcondition holds for every snapshot. That is the one Aeon concept that adds a new kind of evidence rather than a nicer way to produce the same kind.
- **Trust reporting is mostly a rendering of assumptions Moriarty already knows about**, so it is cheap and safe, but it must not be sold as new assurance.
- **Approach two (Aeon as external engine) and three (substrate) should be rejected now**, not left open for deliberation. Both create a language correspondence obligation that competes with the unresolved MC04 correspondence the roadmap already owes.

## Present versus missing

| Capability | Present in Moriarty | Missing |
| --- | --- | --- |
| Pre/post financial obligations | Runtime checks, atomic rejection, six post reads | Static discharge for all inputs |
| Bounded totality, work ledger | Static action bounds, remaining-work reporting, closure reserve | Nothing from Aeon needed |
| Single consumption of funding | Kernel consumes transfer funding once, ledger uniqueness pending | Compile-time diagnostic only |
| Assumption inventory | Scattered across RESULT.md files and reviews | Machine-emitted per-action frontier |
| Counterexample on failure | Evaluator rejection code, no witness state | Source-local witness |
| Program search | None | Restricted, later |

## Classification

Evidence kinds used below: **T** compile-time theorem about the solver encoding, **R** runtime kernel enforcement, **F** finite tests and corpora, **L** ledger evidence on Preview. No item below produces L.

| Feature | Class | Rationale |
| --- | --- | --- |
| Refinements / VCs | **NOW** (step A1) | Only new evidence kind. Restrict to pure UInt arithmetic postconditions over the 48-constructor runtime. Prereq: pinned solver, written fragment. Non-goal: effects, authority, K. |
| Trust reports | **NOW** (step A0) | Zero semantic risk. Must list snapshot, schema, compiler, kernel, absent K/ledger correspondence, not only native bindings. Non-goal: any claim of verification. |
| Counterexamples / IDE | **NEXT** (A2) | Depends on A1 producing models. Render to `.mori` span with concrete snapshot. Non-goal: LSP parity with Aeon. |
| Typed holes, restricted synthesis | **NEXT** (A3) | Enumeration over approved constructors only, output is ordinary source, validated by A1 plus existing check. Prereq: A1 accepted, corpus of 20 financial tasks. Non-goal: effectful synthesis, holes in admitted programs. |
| Qualifier inference | **DEFER** | Explicit annotations first. Inference makes accepted obligations depend on a qualifier set that reviewers cannot see. Revisit after A1 shows annotation burden. |
| Linear capabilities / typestate | **DEFER** | Kernel already enforces consumption at runtime. A static pass is a diagnostic, and RP01 has not frozen authority objects to attach it to. Revisit after RP01. |
| General ADTs / polymorphism | **DEFER** | Aeon's own entailment lifting skips polymorphic binders. Moriarty is deliberately bounded. Revisit only if MC07 fixtures need it. |
| Totality / work bounds | **REJECT** adoption | Aeon's metric is optional and defaults to true. Moriarty's registered limits are stricter. Keep Moriarty's. |
| Optimization / fitness | **REJECT** for admission | Fees, limits and signed intent are hard constraints. At most, size ranking of already-valid A3 candidates. |
| GP / LLM proposals | **REJECT** now | No measured failure of enumeration exists. Revisit only with A3 corpus data. |
| FFI / native | **REJECT** | Native annotations are promises; unsuitable in the financial kernel. |
| Backend adoption | **REJECT** | Conflicts with exact widths, rounding and the existing K work. |

## Ordered roadmap

**A0. Assumption frontier report.** Add a command to the existing simulation CLI that emits, per action, the assumptions its result depends on. Consumer is the same compiler in `financial-agreement-source-compiler.ts`. Evidence: F. Acceptance is falsifiable: removing a schema trust or changing the compiler pin changes the emitted frontier hash. Non-goal: new checks.

**A1. Decidable obligation fragment and VC discharge.** Write the fragment as a document: UInt widths, scale, overflow as rejection, integer division, no quantifiers. Encode postconditions of the source loan lifecycle as bit-vector obligations. Report four outcomes: valid, refuted with model, unknown, unsupported. Evidence: T for the encoding, F for the differential control. Acceptance: every one of the nine retained rejection cases yields refuted or runtime rejection consistently, no valid obligation disagrees with the 682 tests, and a seeded encoding bug that drops overflow is caught by the differential run. Non-goal: proving anything about K or Midnight.

**A2. Counterexample rendering.** Turn A1 models into a concrete snapshot plus source span. Acceptance: for each refuted obligation, running the simulation CLI on the rendered snapshot reproduces the rejection.

**A3. Restricted holes.** Only after A1 and A2 are accepted and the task corpus exists. Acceptance: at least the corpus positive tasks complete with valid obligations, no candidate references authority-bearing primitives, and the output compiles through the unchanged frontend.

Everything else waits for RP01 or is rejected.

## EARS requirements

- **AEON-MOR-R01.** The authoring tool shall emit an assumption frontier for each action that names snapshot, schema, compiler pin, kernel version, and each unproved correspondence, and shall label the report as non-evidence for PCD.
- **AEON-MOR-R02.** The obligation checker shall accept only obligations within the published decidable fragment and shall report any other obligation as unsupported.
- **AEON-MOR-R03.** When the solver returns a satisfying model for a negated obligation, the checker shall report refuted with the model. When it returns unknown or times out, the checker shall report unknown. The two shall never share a code.
- **AEON-MOR-R04.** If a statically valid obligation is later rejected by the kernel on any admitted snapshot, then the checker build shall be marked unsound and the differential corpus shall fail.
- **AEON-MOR-R05.** Where synthesis is enabled, the tool shall search only over an explicit constructor allowlist, shall produce ordinary `.mori` text, and shall never emit a program containing an unresolved hole.
- **AEON-MOR-R06.** While a synthesis or check run is active, it shall not invoke any ledger, plugin, custody or Preview path, and shall write only under its declared workspace.
- **AEON-MOR-R07.** The tool shall not use fitness, examples or sampled properties to accept a candidate; acceptance shall come only from the fragment checker and the existing frontend.
- **AEON-MOR-R08.** Every result shall record solver name, version, timeout and fragment document hash so an independent reviewer can rerun it.

## Foreman Pel delivery controls

Use the standard implement, verify, review chain with no changes to role assignments. Register `candidate-full` as the existing language test plus the A1 differential corpus. Permitted write paths: the successor language source tree, its tests, the fragment document, and a new deliverables directory. Any nonignored change under plugin, ledger, custody, `openspec/sprints`, or `deliverables/preview-*` fails admission. The reviewer receives the fragment document and the seeded-bug control result, not only the diff. Exit 2 on admission failure and exit 3 on unresolved actions are the only permitted non-success outcomes. No publication authority is granted. Grok timeout handling stays as the roadmap states: a missing review keeps the gate open.

## Three highest risks

1. **Silent second authority.** The VC encoding drifts from kernel semantics on overflow, rounding or scale, and reviewers begin citing "valid" instead of the evaluator. Mitigation is R04 and the seeded-bug control.
2. **Outcome conflation.** Aeon's own `smt_valid` collapses refutation, unknown and timeout. A copied design would let a 200 ms timeout look like a counterexample or, worse, an unknown look like proved. Mitigation is R03.
3. **Roadmap contamination.** A0 text or an A1 valid mark gets quoted as progress on SP09 or MC04, or Pel capacity is taken from the open Preview and K tasks. Mitigation is explicit labelling and the write-path fence.

## Strongest counter-position

Do nothing from Aeon until the hard Preview gate closes. All Aeon value is off-chain ergonomics. Task 5 in the lifecycle package and the loan raw-exit gap are unblocked by none of this, and every reviewer hour spent on a solver fragment is an hour not spent on Docker and Preview evidence. This is a serious position. My answer is that A0 and A1 are small, fenced, and A1 produces the only new evidence kind that MC04 correspondence will eventually need. But if the team cannot fund A1 without slowing task 5, the counter-position wins and everything becomes DEFER.

## What would change my opinion

- If A1 shows the kernel's exact arithmetic cannot be encoded faithfully in a decidable fragment, refinements drop to DEFER and counterexamples with them.
- If A3's corpus shows enumeration fails on most realistic tasks, GP or LLM proposals move from REJECT to DEFER behind the same validator.
- If RP01 freezes authority objects sooner than expected, linear diagnostics move to NEXT.
- If independent review capacity stays constrained by the Grok timeouts, I would cut A2 and A3 entirely.

## Conditions for endorsing a shared plan

I would sign a joint plan that meets all of the following:

- Approaches two and three are recorded as rejected, not deferred.
- A1 is bounded to pure arithmetic postconditions with a written fragment and four-way outcomes before any hole work starts.
- Every artifact states which evidence kind it is, and none is placed under an SP05, SP09, MC04 or PCD checkbox.
- Pel admission fences the write paths and the verify gate includes the seeded-bug control.
- The consensus document lists what each reviewer would need to see to change their vote, so it reads as a shared falsifiable position rather than a tally.