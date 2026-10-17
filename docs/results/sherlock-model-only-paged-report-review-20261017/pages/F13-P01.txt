# Bounded universal-interpreter companion — source draft

Author: gpt-6.1-sol, delegated lane 09. Status: source-only; no Quint command,
model evaluation, build, native tool, prover or backend has run in this lane.
Root owns per-source Opus classification and any subsequent ordinary checks.
These files remain runtime-only design artifacts and establish no product gate.

The independent arithmetic literals in `expected-normal.json` were written
before the assertion module. They follow 3×2×1=6, 1+2+3=6 and 4+5=9, without
using model/evaluator output. Scalar bound 15 deliberately accommodates tree9;
it replaces lane 09's earlier proposed 0–7 bound. Factorial4 produces 24 and
therefore encounters this finite instance's ordinary scalar-bound refusal.

## Source membership and bounds

`universalMachine.qnt` owns one grouped `Machine` state, initial states, pure
functional transitions, thin actions, invariants and reachability predicates.
`universalMachineTest.qnt` imports only `./universalMachine` and contains eight
named `run` scenarios ending `Test`. There are no external imports, recursive
operators/types, hooks or proof/authentication predicates. `expected-normal.json`
and this README are documentation, not executable dependencies.

The finite instance has six initial scenarios and three algorithm identifiers;
argument 0–4, scalar 0–15, eight immutable node slots 0–7, at most eight frames,
fuel 2 or 32, at most one checkpoint/yield, at most one false observation and
one accepted event. The model contains no unbounded nondeterministic range and
no uninstantiated constant. Nondeterministic initialization picks only a member
of the explicit scenario set. Later transitions are deterministic except the
choice between the modeled ready/not-ready observations.

The common heap stores a three-element list at reference3 and a two-leaf tree
at reference7. Reference0 is an actual Nil node, not a missing-reference value.
All references target existing nodes; every structural child has a smaller
identifier. Node identity is not a source-language observable. This fixed arena
is immutable, acyclic and contains no allocation model. Frames distinguish
factorial multiplication, list-head addition, visiting the right subtree and
adding the left result; no recursive Quint definition is required.

## Actual modeled behavior

`Evaluate` pushes explicit frames or produces a value. `ReturnValue` pops a frame
and performs checked arithmetic, or finishes with a terminal result. Tree
traversal retains the right reference while computing the left value, then
retains the left value while computing the right. Factorial3 and list123 each
require eight ticks; tree45 requires six. These step counts are hand-traced
source expectations, not observed executions.

Each tick with remaining fuel consumes one logical work unit and increments
attempted work. An attempted tick at zero transitions to Exhausted, incrementing
attempted but not consumed work. Thus total=remaining+consumed and
attempted=consumed+1 exactly in Exhausted. Ready/not-ready observations and
terminal stutters consume no logical interpreter work. These are abstract work
units; they are not circuit rows, DUST, financial fees or native resource costs.

The yield fixture suspends before applying a pending return frame. Its snapshot
retains accumulator1, all three factorial frames and consumed-work5. A false
observation leaves control, frames, snapshot and fuel unchanged. A ready event
restores Running with the retained return value and frames. The concrete
environment permits at most one false observation; a ready event remains
enabled afterwards. This finite input bound does not establish external-event
availability, real time, unconditional progress or authentication.

`boundary` is exclusively the local interpreter suspension/checkpoint boundary.
It is NOT a financial commit or a proof-chunk commitment. No financial effects,
working store, accumulated duties, ledger phases or financial commit operation
are modeled. Core/2 successful-prefix await semantics are unchanged and cannot
be inferred from this companion. Exact future Commit/Yield opcode design is open.

Rejected represents ordinary constructor/reference/finite-domain refusal.
The included rejection scenario is ordinary arithmetic exceeding scalar15;
there are no hostile, tampering, cryptographic or security fixtures. Terminal
states expose explicit result/reason fields and permit only exact stuttering.

## Properties and nonvacuity

`allInv` combines finite-state bounds, immutable well-founded heap references,
frame payload bounds, work accounting, waiting-frame/pending-value agreement,
terminal result consistency and checkpoint accounting. Named witnesses cover
recursive depth≥3, four list-node visits including Nil, the exact tree-node set,
Waiting with three retained frames, resume after a false observation,
exhaustion after positive work and scalar-bound refusal. Tests first require
the corresponding witness to be false at init and then force a nonempty trace.
Mid-trace assertions inspect frames and cursor, so terminal flags alone cannot
satisfy the normal scenarios. `terminalStutterTest` checks unchanged terminal
results/work; `yieldPendingReturnTest` checks snapshot preservation across both
observations and eventual halt. Transition-preservation claims beyond those
specific scenarios remain obligations, not exhaustive results.

## K projection and remaining gaps

This companion is intentionally unequal to the K draft's generic first-order
expression evaluator. Its program identifier selects one of three finite
algorithm transitions; it does not load a K function table, arbitrary code or
AST. There is no lexical environment, caller-local restoration, general let,
Bool expression evaluation, mutual recursion, or arbitrary match dispatch.
The frame tags are algorithm-specific abstractions, not general K return PCs.
Math integers are guarded by scalar15, not UInt128 or target field arithmetic.

A proposed projection relates K expression/control, continuation stack and fuel
to Control, frames, accumulator and work on the common factorial/list/tree
subset. It must define heap-to-structural-ADT correspondence, frame translation,
evaluation order and which K administrative rewrites stutter. Current tick
counts are not asserted to equal K fuel costs. Heap references are absent from
K structural identity, so erasing node addresses is another explicit obligation.

The K first fragment leaves waiting/finance unmodeled. Therefore this Quint
waiting/checkpoint extension has no established K projection. Any later adapter
must separately preserve Core/2 successful-prefix effects, residual duties and
checkpoint atomicity. The local ready boolean says only whether the modeled
event condition has occurred; it supplies no proof, signature or authorization.
No semantic refinement, execution equivalence, financial correctness, native
correspondence or universal interpreter capacity follows from this draft.

## Planned root checks only

After exact-source classification, root may typecheck both files, test module
`universalMachineTest` using `--match 'Test$' --backend typescript`, and sample
module `universalMachine` using `--invariant allInv --backend typescript
--max-samples 32 --max-steps 32 --seed 1`. No command has been run by this author.
No `verify`, Apalache, Rust backend or temporal/exhaustive checking is planned.
Root must apply the execution boundary's containment, offline, output and
source-digest conditions as reconciled with higher-priority developer rules;
the author neither changes HOME nor creates tool caches or model artifacts.

## Initial frontend diagnostic and syntax-only repair

Root's initial C2 test-module typecheck failed while loading the imported main
module; no test or sampled-run stage followed it. Its first listener diagnostic
was `violated grammar of annotated params return type`. Installed Quint 0.32's
AnnotatedOperDef grammar requires a return type after annotated parameters;
the two parameterized actions lacked `: bool`. The repaired source adds those
two annotations only. Expected values, domain bounds and transition bodies are
unchanged. Original source/manifest bytes are retained outside this model's
source membership, under `R/diagnostics/quint-C2-original/`. Root's original
stdout/stderr remain under `R/checks/C2.*`. The changed digest needs fresh
classification and a root check; no successful typecheck or execution is claimed.

Root's second C2 attempt parsed further and rejected the local name `next`,
which is in Quint 0.32's builtin inventory. Before another candidate, the author
changed approach to a complete source-static declaration/binder scan against
the installed builtin/lexer inventories, plus return-annotation/import checks.
The audit is retained at `R/diagnostics/quint-source-static-audit.md`, with its
script, full inventory hashes and per-binding JSON. It found only that binding
collision; record labels `head`/`tail` and wildcard `_` are not declarations.
The candidate now consistently calls that local value `chargedMachine`.
No function body behavior, expected literal, scenario or finite bound changed.
Second-candidate bytes are preserved at `R/diagnostics/quint-v2-original/` and
its failure remains in `R/checks/quint-v2/C2.stderr`. The static scan is not a
Quint parse, type/effect check, simulation or acceptance result. Fresh source
classification and root checks remain required.
