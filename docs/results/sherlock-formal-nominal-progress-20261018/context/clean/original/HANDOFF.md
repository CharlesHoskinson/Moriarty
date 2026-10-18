# Dedicated Clean source batch 01

This batch contains actual additive Lean source for the existing Fenrir Clean integration, prepared
as exact afterimages only. Root owns destination installation, Git/provider actions and all execution.
No worktree, fork source, Git state or old runtime artifact was changed by this lane.

## Candidate

- `afterimage/clean/fork/Clean/Sherlock/CanonicalTypeId.lean`: one numeric canonical TypeId,
  separate CtorId, fixed builtin row names 0–3, round-trip and identity lemmas. The source follows
  the observed production TypeIdC migration without converting later IDs through SortTag.
- `afterimage/clean/fork/Clean/Sherlock/ConstructorFields.lean`: executable `beginFields`,
  `returnField` and `runReturned` continuation operations. Each pending field saves canonical
  identities, source origin, entire caller environment, evaluated prefix and remaining expressions.
  Build receives the complete ordered value list only after all child values have returned.
- `afterimage/clean/fork/Clean/Sherlock/ConstructorFieldsNormal.lean`: four literal normal theorem
  examples instantiated with the existing reference `Semantics.Expr`, `Value` and `Env`. Cases
  cover one-field-return technical cut, complete two-field construction, nullary Build and no return.

The main falsifiable theorem predicate is `runReturned_complete`: for arbitrary field expressions
and returned values of equal length, executing those returns yields exactly Build with
`prefix ++ values`, unchanged TypeId/CtorId/origin and saved environment. `FieldsEvaluate` states
independent ordered child meanings in that saved environment; `completed_fields_meaning` is the
conditional semantic corollary. `returnField_preserves_context` proves one-return preservation;
`runReturned_append` proves exact-cursor technical split composition. None claims child evaluator
correctness or equal A/B clocks.

## Readset and provenance

`READSET.json` freezes fifteen real source/contract files and copied bytes. The observed active
Fenrir integration is `/home/charl/projectfenrir-sherlock`, whose `.git` is a worktree pointer into
`/home/charl/projectfenrir/.git`; the branch ref read directly is
`feat/sherlock-terminal-control-20261017` at `5b378d117504f87c85b2ec7fc5844be57e92dbbb`.
This is file-level provenance only, without a fresh Git clean-state attestation.
`/home/charl/clean-sherlock` instead points into `clean-fenrir` at the older
`feat/sherlock-step-20261015` branch and has no Sherlock modules in its current source tree.
No repository was cloned, no remote was contacted and no wallet/SDK material was read.

`PREIMAGES.json` names three absent destination files, checked before source creation.
`SOURCE-MEMBERSHIP.json` binds all candidate bytes and records no readset drift at its capture.
Root must recheck absence and every actual source dependency before installation or checking.

## Actual verification and proposed checks

Only source reads, SHA256/copy membership checks and comment-stripped token/line-width inspection
were performed. The three files contain no outside-comment `sorry`, `admit`, `axiom`, `unsafe`,
`partial` or `native_decide` token; this is a text check, not a type/proof/axiom-closure result.
Every new Lean file is within 100 columns. All Lean/model/circuit/native jobs are UNRUN.

`PROPOSED-ARGV.json` contains four sequential direct Lean frontend checks using the existing
`lean-observation-local-check/PROPOSED-ARGV.json` envelope: Lean4.33.1, 200-second timeout,
4GiB memory, zero swap, 256 tasks, network isolation and a 16MiB output file limit. It substitutes
only stage unit, source root/file and fresh runtime artifact/scratch paths. No Lake build, dependency
fetch, interpreter evaluation, heartbeat/recursion override, circuit or native operation is requested.
The single fresh artifact root avoids the previously documented split-Clean-package lookup issue.
The Semantics import is freshly checked from its frozen source, rather than silently trusting a
historical cached artifact. Root creates the listed output directories and selects/audits execution;
the author has not created a runner or launched any of these commands.

## Assumptions and open boundaries

The pure continuation accepts semantic Expr/Value/Env carriers; it has no Rust adapter yet. In the
frozen real `shared.rs`, FrameRowC still has exactly nine core variants and ConstructorFields is
explicitly refused. The new source does not change that fact or claim installed generic admission.
The old pure Semantics.Value still has dedicated UInt/list/tree variants. The parametric continuation
can represent a generic nominal carrier but has not extended or replaced that old machine.

`FieldsEvaluate` assumes each child has completed with its stated semantic value. Neither that
relation nor matching lengths proves child correctness, source termination or finite B work.
Full declaration membership, constructor field typing/arity, ordered MatchPlan selection, actual
Append/Build helper steps, allocation/capacity checks, complete enclosing frames, financial state,
B clocks/cuts, MachineImage/StepInv and physical representation are absent from this slice.
It ends at Build input, before value allocation. No bounded Clean constraint component,
constraint soundness/completeness, Rust/K/Quint refinement, native SDK consumer compilation,
proof, stagenet transaction or settlement is established. Security assessments stay with Opus.
The original35 UIREQ,25 additive refinements and every native/stagenet/formal/financial gate remain.

## Next source work

Await Root-selected kernel feedback and current generic constructor owner interface. Extend this
same module with an actual nominal Value/constructor-layout carrier and ordered MatchPlan semantic
slice only against the adopted interface/current implementation. An independent oracle owner has
received declaration-only continuation API/scenario metadata; no populated independent expectations
were read. Current constructor runtime adoption and full B schedule must be supplied by the owner,
not inferred from these proofs. Keep semantic completion, observed checking and product acceptance
separate as that source advances.
