# Skeleton-coding pilot: protocol amendment 1

Date: 2026-11-09. Amends `pilot-protocol.md`. Source: the smoke result,
`pilot-smoke.md` §5 (harness defects D1 to D6).

This amendment is made under protocol change rule (a): after the smoke, to fix
harness defects and the mutant budget, before wave 1. It must be committed
before the first wave-1 build. After the first wave-1 build, nothing in it
changes.

## 1. D1: one circuit per (rule, program)

Defect: with `per_rule` 2, `honest_steps` takes the second step of a rule from
another program when the first program has only one (for `c6_admin_publish`:
fact(3) step 292, then fold step 195). `build_all` built one circuit per rule
from the first step's layout. The fold statement did not fit that circuit
("statement length: expected 4255, got 4297"), for the reference too, so
`complete` was false for every build and the fold-base mutants were untested.

Fix (grader `skeleton_pilot.rs`):

- A circuit key is (rule, program) (`Honest::key`, for example
  `KAdminPublishValue@fold`). `build_all` builds one reference circuit and one
  candidate circuit per key, from the layout of that program. Every honest pair
  and every mutant is checked against the circuit built for its own layout.
- The mutant space is built per base, each base with its own statement layout
  (`honest_mutant_space`). Coverage per section reports `lanes` (the largest
  section length over the base layouts) and `lane_slots` (the sum over the
  base layouts), with the mutated count of each.
- The static check runs per key; a finding names its circuit key.
- Harness check: the reference must accept every honest enabled pair and every
  disabled identity pair. If it does not, the grade is a harness error:
  `verdict.harness_ok` is false, `verdict.primary_pass` is null, no mutant
  runs, and `harness.errors` names each refused pair. A harness error is not a
  candidate fail. It is not counted in the analysis; the lead fixes the harness
  and regrades.
- The self-test `grader_finds_dropped_constraint_and_clears_reference` now
  grades the reference against itself with `per_rule` 2 (fact(3) and fold) and
  requires the reference to accept every honest pair, with zero untested
  mutants. The old grader fails this test. It also grades a broken reference (the
  weakened emitter as reference) and requires a harness error.

## 2. D2: a fixed mutant prefix per component

Defect: the wall budget set the number of mutants, and a smaller candidate ran
more mutants in the same budget. Coverage then depended on the candidate, which
confounds the primary outcome with circuit size.

Fix (grader and `scripts/qa/skeleton_pilot.py`):

- Every build of a component is graded on the same mutants: the first N
  mutants of the stratified sample (protocol §6), with N fixed per component.
  The sample order does not depend on N, so the sample for N is a prefix of
  every longer sample, and mutant id i is the same mutant in every build.
- The sample seed is per component, not per build: the driver passes
  `SKELETON_PILOT_SEED=20261108` for every build. This replaces the sentence
  "the seed also seeds the mutant sample" in protocol §5. The build seed still
  names the repeated lane run.
- A `random` mutant draws its value from a generator seeded by the sample seed
  and the mutant id, so its value does not depend on which mutants ran before
  it.
- N is `SKELETON_PILOT_MUTANTS` (default 400) or a per-component value in
  `MUTANTS_BY_COMPONENT`. The lead confirms N for each wave-1 component before
  the first wave-1 build and records it in the wave-1 run log. When the mutant
  space is smaller than N, all mutants run. For scale: at the slowest smoke
  rate (6.5 s per mutant), 400 mutants take about 2600 s.
- The wall budget stays only as a safety stop. If it ends before the grade
  reaches all N planned mutants, the grade is harness-incomplete:
  `verdict.harness_ok` is false, `primary_pass` is null and `harness.errors`
  records how far it got. The grade does not report a lower coverage as a
  result. The lead regrades it with a longer slot; N is never lowered for one
  build.
- The receipt schema is `skeleton-pilot-grade/2`. New fields: `harness`,
  `mutants.reached`, `mutants.all_planned_ran`, `verdict.harness_ok`, and
  `circuit` in the honest and mutant records.
- Shared-id helper: `skeleton_pilot.py compare CELL...` compares the builds of
  one component on the mutant ids that every build reached, by class, and lists
  the soundness ids. It refuses grades with different plans (component, seed,
  N, `per_rule`). With the fixed prefix, every complete grade reaches all N ids,
  so the helper is a check, not a correction.

## 3. D4: cell records per cell

`grade-batch` marks every cell of the batch `grade_exit: "pending"` (and
removes the old verdict) before the heavy run. The batch script runs
`skeleton_pilot.py record-cell CELL --exit RC` after each cell, so each
`cell.json` is written as soon as its grade ends. A cell that a killed batch did
not reach stays "pending".

## 4. Smoke regrade before wave 1

The 6 smoke builds (`runs/smoke/`) were graded with the defective grader. They
must be regraded with the fixed grader (same N and seed rule as wave 1) before
wave 1 starts. The smoke stays a harness proof with no inferential claim
(protocol §8). The regrade must show, for every build: `harness_ok` true, every
honest pair accepted by the reference, all N planned mutants reached, and zero
untested mutants that come from a layout mismatch. The old receipts are kept;
the regrade writes new receipts.

## 5. Wave-1 check: independent Gemini Quint lane

Each graded wave-1 candidate circuit also gets one independent Gemini lane:

- Runner: `agy` with `gemini-4-argon`, started with `argon-run`, in its own
  detached worktree under `~/.cache/gemini-lanes/`, wrapped in `heavy-run`.
- Task: model the component's constraints (the candidate emitter) and the
  reference semantics (the reference machine's step logic for the component
  rules) in Quint, with the `quint-modeling` and `quint-lang` skills, and
  report a SOUND verdict (every statement the candidate accepts is a reference
  step or a disabled identity) and a COMPLETE verdict (every reference step and
  disabled identity is accepted) per rule, with a counterexample for each "no".
- The lane reads source only. It runs no cargo, no proof and no `quint`
  command (headless agy cannot run commands). The lead runs `quint typecheck`,
  `quint test` and `quint run --invariant` on each model and appends the
  results to the lane report.
- Report: `qa/gemini-quint/skeleton-pilot/<cell>/REPORT.md`. The lane never
  commits; the lead reviews and commits the report.
- The lane starts only after the build's grade is written, and its output does
  not change the grade.
- Status: the lane report is an input, not evidence. It does not change
  `primary_pass`, the analysis (protocol §8) or the decision rule (§9). A
  disagreement between a lane verdict and the grade is listed in the wave-1
  result for review by the lead. It is not a pilot outcome.

## 6. Not changed

Protocol §1 to §4 and §7 to §10 do not change, except as stated above.
`per_rule` stays 2. D3 (batch peak RSS near the 16 GiB cap), D5 (the batch
self-test runs every time) and D6 (lane `cargo check` outside
`scripts/doctorwatson heavy`) stay open. They do not change a grade, but D3 and
D6 must be handled before wave-1 lanes and grades run on this host.
