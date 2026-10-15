# Lane 10 — Acceptance challenge: generality, recursion, models, and evidence

**Status: recommendation, specified-only; no architecture adopted.** A general-purpose interpreter executes arbitrary programs composed from its published supported grammar through one versioned transition relation. Adding factorial must not add factorial-specific transition rules. Full scope includes recursive ADTs, constructor/pattern operations, recursive and nested calls, and suspension inside a callee with caller frames outstanding; each execution remains finitely bounded. A lifecycle automaton alone demonstrates lifecycle behavior.

**Source facts.** [K's manual](https://kframework.org/docs/user_manual/) defines syntax, configuration, and rewrite rules as separate parts of executable semantics [K: Introduction; Configuration Declaration]. [Matt Might's CEK tutorial](https://matt.might.net/articles/cek-machines/) represents control, environments, and continuation frames explicitly [C: “The CEK machine,” `step` equations]; its lambda-calculus example does not establish Moriarty ADTs or lifecycle behavior. [Quint's property guide](https://quint.sh/docs/checking-properties) distinguishes witness traces, bounded checking, explicit-state checking, and inductive invariants [Q: corresponding sections]. Model results concern the specified transition system.

**Alternatives.** (A) Extend the actual Core execution representation with explicit frames and heap nodes: fewer adapters; reconcile existing semantics. (B) Specify a CEK-style source machine: clearer call contexts; new correspondence obligation. (C) Introduce bytecode: compact dispatch; additional compiler/semantics boundary. Prefer comparing A and B against the same scenarios before freezing; bytecode remains an alternative, not an adopted backend. A CEK machine is not inherently bytecode. Recursive tree values are simple specification terms; indexed immutable heap nodes expose sharing and finite allocation. Neither representation choice proves the other implementation.

**Proposed nonsecurity EARS clauses.** M=model, C=compiler, R=circuit relation, H=host/tooling; these tags identify future work, not completed support.

1. **GEN-01 [M,C,H]** WHEN two different supported programs are loaded, the interpreter SHALL execute both using unchanged transition rules. Scenario: factorial and list sum.
2. **GEN-02 [M,C,H]** WHEN a profile is selected, tooling SHALL enumerate admitted grammar, numeric domains, bounds, and unsupported constructs.
3. **GEN-03 [M,C,R]** WHEN `factorial(5)` executes within bounds, the machine SHALL return 120 through recursive calls and returns.
4. **GEN-04 [M,C,R]** WHEN `sum([1,2,3])` executes, the machine SHALL return 6 through recursive constructor matching.
5. **GEN-05 [M,C,R]** WHEN `sum(Node(2,Leaf(1),Leaf(3)))` executes, the machine SHALL return 6 while preserving both child results.
6. **GEN-06 [M,C,R]** WHEN nested arguments execute, the machine SHALL follow the declared evaluation order and restore each caller environment. Scenario: `add(double(2),increment(3)) = 8`.
7. **GEN-07 [M,C,R]** WHEN admitted mutually recursive functions execute, the machine SHALL preserve their distinct call targets. Scenario: `even(4)=true`, `odd(5)=true`.
8. **GEN-08 [M,C,H,R]** WHEN `10+inner(3)` suspends inside `inner`, suspension SHALL retain its caller frame; resumption returning 4 SHALL complete with 14.
9. **GEN-09 [M,C,H]** WHILE suspended, the machine SHALL expose pending status separately from a completed return value.
10. **GEN-10 [M,R,H]** WHEN a bound is reached, the machine SHALL report the reached bound and resumability policy as a distinct outcome.
11. **GEN-11 [M,C,R,H]** WHEN execution advances, observables SHALL include ordered outputs/effects, residual duties, and resource counters defined by a versioned schedule.
12. **GEN-12 [M,R,H]** WHEN execution suspends and resumes, cumulative logical work SHALL equal the sum of its segments under that schedule; host time, circuit rows, and ledger costs SHALL remain separate measures.
13. **GEN-13 [M,H]** WHEN a K trace is projected into Quint, its adapter SHALL identify matched fields, hidden steps, bounds, and unmatched behavior. Each retained transition SHALL have a named K-rule correspondence obligation.
14. **GEN-14 [M,C,R,H]** WHEN evidence is reported, it SHALL identify whether it establishes a specimen result, bounded model property, compiler correspondence, circuit correspondence, or target observation.

**First demonstrator and counterargument.** Bound integers to 0–128, heap nodes to 15, call depth to 8, and transitions to 256; run the positive specimens above through the same machine. These model limits are not language limits. A lifecycle model is cheaper, but cannot demonstrate recursive evaluation. Candidate K cells: program/control/env/heap/frames/status/effects/duties/work; Quint retains finite projections with an explicit abstraction map. Replay is evidence of selected matching traces; proving a simulation relation and connecting the actual compiler and native circuit remain separate work. New universal dispatch/heap/frame circuit constraints may be required; source-static lowering and host checkpoint tooling alone cannot establish them.

**Actual visual targets inspected:** shared PixelRAG `visual/hit-1-1.jpg` distinguishes CEK/CESK representations; shared primary Lurk `cont_pointer.../tile_0000.jpg` shows tagged continuation representation. Use these to draw caller-frame retention and heap-pointer diagrams. They are design aids, not Moriarty correspondence evidence.
