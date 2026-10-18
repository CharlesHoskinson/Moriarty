# Nominal constructor continuation companion

Source-only candidate. No Quint frontend, test or simulation has been executed
by this author. Fresh independent non-author review and selected root execution
are required. This adds a separate finite semantic profile; it does not change
frozen K-A, frozen algorithm Quint-A, A39/B96, a circuit schema or the public API.

The gap is SCON-016.R02: generic construction must evaluate fields and retain
its continuation through recursive calls. A materializer accepting already
evaluated fields does not cover this gap. The model reads ordinary immutable
AST/declaration tables and dispatches by expression variants; there is no
factorial/list/tree/fixture selector in the evaluator.

Values use structural preorder tokens to avoid recursive Quint types. Each
constructor token carries exact nominal type/constructor identity and source
field count. Ordered declaration field types drive structural validation.
The continuation retains type/constructor identity, occurrence, source expression,
completed fields, remaining expressions and saved environment. Lower frames
retain eventual return context. Constructor allocation is a separate Build
transition after all fields finish. Calls start with parameters only and restore
caller environment on return. Matches try source arms in order and run only the
first match body, with arm scope restored afterwards.

The main finite input constructs Schedule(make(2),caller9). make recursively
constructs Link(2,Link(1,End)); this exercises recursion in Schedule's first field
and Link's last field. inspect shadows the caller name, skips a nonmatching arm,
selects the first of two matching arms, and returns its second field. The caller
adds its restored9. Independent literal result18, structural values, build order
and selected arm were frozen before evaluator authoring. Separate ordinary inputs
construct equal-looking Pay4 values at different occurrences and a value through
mutually recursive type30/type31 declarations. These are additional model inputs,
not the retained UIREQ034 Schedule4/sum9/repeatedPay8 product acceptance suite.

Eight tests propose final-value/environment checks, recursive first/last-field
frame inspection, occurrence separation, mutually recursive construction,
post-field allocation, all-prefix pure Snapshot save/resume consistency and
explicit terminal-charge adaptation. All-prefix consistency computes actual
successors on both sides and is an internal model property, not an independent
prefix oracle. Full independently specified prefix/frame/work checks are a
separate follow-up with withheld non-author expectations. No outcome or work
total is generated into expected-normal.json by the model.

The finite instances use scalar0..255, at most32 frames and initial fuel192,
128 or32. Programs contain finite maps, and recursion proceeds through named
calls in those maps. All operations are pure except one grouped machine state,
initialization and guarded tick/stutter actions. The model owns step; the test
module supplies concrete init for the main program. No external imports,
uninstantiated constants, recursive Quint definitions/types, network or backend
are needed. The proposed bounded run is deterministic; a seed identifies the
command but supplies no random coverage.

Adapters are explicit:

| Concern | This profile | Other profiles / limitation |
|---|---|---|
| Profile | sherlock-quint-nominal-construction/1 | No K/B event-tag substitution |
| Terminal publication | one charged pending transition; zero fuel exhausts first | frozen K-A/A/B publication is free and precedes exhaustion |
| Fields | structural preorder values; declared source order | no native row/UInt128/field encoding |
| Source occurrences | expression ID plus distinct occurrence ID on constructors/builds | repeated evaluation of one occurrence remains one source site |
| Frames/env | top-first; full saved maps and pending field prefixes | no production ABI or canonical serialization |
| Helper work | zero; pure folds normalized inside transitions | B entry/iteration/exit work/visits/tasks/fuel not modeled or equated |
| Allocation | constructor node and ordered field slots at Build | no scalar/environment/physical arena accounting claim |

OutsideDomain is a model-domain disposition, not a new production reject code.
The inputs are closed, monomorphic and binder-linear. The profile does not
replace legacy duplicate Cons/Branch binder-name match-miss semantics. Nested
constructor patterns use explicit nested Match expressions; compilation of
pattern matrices and admission are outside this model. Frame/scalar limits are
finite model restrictions, not security evidence or native budgets.

This model omits complete financial Core/2 F01/F02 state, effects and await
prefixes; persistent canonical export/import; physical row release; B helper
refinement; all39/full96; source parsing/lowering; Clean/K/circuit correspondence;
proofs, cryptography, native execution and ledger settlement. Pure Snapshot
value save/resume is not production persistent boundary evidence. Finite runs
cannot establish exhaustive validity or close UIREQ034/SCON-016.

Normative sources: docs/sherlock/design.md section revision03 reconciliation,
SCON-016.R02 in openspec/changes/sherlock-completion-convergence/specs/sherlock-completion/spec.md,
and openspec/changes/sherlock-universal-interpreter-design/interfaces.md section
Declaration-driven ADT extension and financial boundary. No tag, width, failure
charge or production schema is selected by these additions.
