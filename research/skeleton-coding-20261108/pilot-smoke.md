# Skeleton-coding pilot: smoke result

Date: 2026-11-09. Protocol: `pilot-protocol.md` (smoke, §5). Run directory:
`research/skeleton-coding-20261108/pilot/runs/smoke/`. The smoke is a harness
proof only; it makes no inferential claim (protocol §8).

## 1. Summary

- All 6 builds and all 4 skeleton lanes ran end to end: lane exit 0, zero
  blind-audit violations, every candidate builds every rule.
- No build has a soundness finding: 0 soundness mutants, 0 static findings
  (lanes read by the reference and not by the candidate), 0 private inputs.
- The 3 `c6_call_bind` builds (Codex) are complete and pass the primary outcome.
- The 3 `c6_admin_publish` builds (Claude) fail the primary outcome only
  because `complete` is false. That is a grader defect (D1), not a candidate
  defect: the reference itself fails the same honest pairs.
- Mutant coverage is very low: 0.10 % to 2.78 % of the mutant space, and every
  grade stopped at the wall budget before the 4000 planned mutants (D2).
- The grader self-test passed in both batches (`grader-selftest.log`,
  3 passed, 914 s in the second batch).

## 2. Outcome per build

Grade settings for every build: `per_rule` 2, `max_mutants` 4000, seed 1,
wall budget 1500 s (cell slot 1800 s; see D2), grader exported from
`626a71393`.

| Build | Built | Complete | Sound | Static | Soundness mutants | Over-constraint | Primary | Instr. cand / ref |
|---|---|---|---|---|---|---|---|---|
| `c6_admin_publish-A-claude-s1` | yes | no (D1) | yes | 0 | 0 | 1 | fail | 2 202 459 / 2 202 420 |
| `c6_admin_publish-B-claude-s1` | yes | no (D1) | yes | 0 | 0 | 1 | fail | 899 093 / 2 202 420 |
| `c6_admin_publish-C-claude-s1` | yes | no (D1) | yes | 0 | 0 | 1 | fail | 2 202 459 / 2 202 420 |
| `c6_call_bind-A-codex-s1` | yes | yes | yes | 0 | 0 | 0 | PASS | 2.20M, 2.50M, 2.50M / 2.20M, 2.50M, 2.20M |
| `c6_call_bind-B-codex-s1` | yes | yes | yes | 0 | 0 | 0 | PASS | 2.20M, 2.50M, 2.50M / 2.20M, 2.50M, 2.20M |
| `c6_call_bind-C-codex-s1` | yes | yes | yes | 0 | 0 | 0 | PASS | 2.51M, 2.81M, 2.51M / 2.20M, 2.50M, 2.20M |

Instruction order for `c6_call_bind`: `KMachineBoundValid`,
`KMachineEnterBind`, `KMachineInvokeFind`. All three Codex builds emit about
300k more instructions than the reference for `KMachineInvokeFind`; build C
emits about 310k more for every rule. Build B of `c6_admin_publish` emits 41 %
of the reference count.

Honest pairs. `c6_admin_publish`: fact(3) step 292 (enabled and disabled
identity accepted by reference and every candidate) and fold step 195
(`synthesis_error` "statement length: expected 4255, got 4297" for the
reference and every candidate; D1). `c6_call_bind`: fact(3) steps 15, 22, 23,
83, 90, 91; all 12 pairs accepted by the reference and every candidate.

Over-constraint. All three `c6_admin_publish` candidates reject one mutant that
the reference accepts: mutant 249, `paired_plus_one` on pre and post IMAGE
lane 5 (control lane `k[3]`, statement lane 408), fact(3) enabled base. The
candidates require control lanes 3 to 16 to be zero for a Return control; the
reference does not constrain these lanes in this step. This is a completeness
deviation on a non-canonical pre-state only. The reference behaviour is a
question for the reference owner, not a pilot finding.

Lane cost (builder lanes):

| Build | Lane wall s | Tokens |
|---|---|---|
| `c6_admin_publish-A-claude-s1` | 100.9 | 770k in / 7.6k out ($0.68) |
| `c6_admin_publish-B-claude-s1` | 231.3 | 904k in / 23.7k out ($1.21) |
| `c6_admin_publish-C-claude-s1` | 133.9 | 746k in / 15.3k out ($0.87) |
| `c6_call_bind-A-codex-s1` | 364.8 | 1824k in / 15.0k out |
| `c6_call_bind-B-codex-s1` | 410.3 | 1841k in / 14.4k out |
| `c6_call_bind-C-codex-s1` | 289.0 | 1257k in / 10.6k out |

Skeleton lanes (Claude): admin publish B 92.8 s ($0.50), C 105.3 s ($0.47);
call bind B 185.6 s ($0.83), C 85.2 s ($0.45). All 0 blind violations. The
first two admin publish skeletons were void (sandbox without the included
files) and are kept in `runs/smoke/void-missing-includes/`; the fix is commit
`2924f1a1d`.

## 3. Declared coverage (SAS-014)

Coverage is below 1 for every build. `run` excludes mutants identical to the
base. "Tested" = a verdict from both emitters (agree, over-constraint or
soundness). Untested mutants in `c6_admin_publish` are the fold-base mutants
(D1).

| Build | Run / planned | Space | Coverage | Tested | Untested | Identical | Lanes mutated / 4255 | s per mutant |
|---|---|---|---|---|---|---|---|---|
| `c6_admin_publish-A-claude-s1` | 1186 / 4000 | 76 584 | 1.55 % | 584 | 602 | 191 | 798 | 1.27 |
| `c6_admin_publish-B-claude-s1` | 2131 / 4000 | 76 584 | 2.78 % | 1047 | 1084 | 362 | 1215 | 0.70 |
| `c6_admin_publish-C-claude-s1` | 1175 / 4000 | 76 584 | 1.53 % | 579 | 596 | 190 | 794 | 1.28 |
| `c6_call_bind-A-codex-s1` | 391 / 4000 | 229 752 | 0.17 % | 391 | 0 | 57 | 274 | 3.84 |
| `c6_call_bind-B-codex-s1` | 325 / 4000 | 229 752 | 0.14 % | 325 | 0 | 52 | 229 | 4.63 |
| `c6_call_bind-C-codex-s1` | 231 / 4000 | 229 752 | 0.10 % | 231 | 0 | 37 | 166 | 6.53 |

Every one of the 15 statement sections (enable, schema, carrier version, pre
and post META, PROG, CTX, IMAGE, DATA, HELPER) got mutants in every build. Per
section counts are in each `grade.json` under `mutants.by_section`. The static
check covers all lanes in every build.

## 4. Receipts

For each build `<b>` under `runs/smoke/`:

- `<b>/grade/grade.json`: summary, verdict, honest pairs, static check, coverage.
- `<b>/grade/mutants.jsonl`: one record per mutant run (class, kind, section, lane, both verdicts).
- `<b>/grade/heavy.log`: grader build and test output.
- `<b>/grade/install/`: the candidate and adaptor files the grader compiled.
- `<b>/candidate.rs`, `<b>/cell.json`, `<b>/audit.json`, `<b>/sandbox.json`, `<b>/lane/`.

Batch records: `runs/smoke/grade-batch.log` (second batch: 5 builds,
heavy run `20261109T181127Z-SKP-batch-5`, wall 8516 s, peak RSS 15.9 GB),
`runs/smoke/grader-selftest.log`. Build A of `c6_admin_publish` was graded in
an earlier batch (09:37 to 10:02) that the host killed during the next build;
its `cell.json` was then updated by hand from its receipt.

## 5. Harness defects (fix before wave 1, protocol change rule (a))

D1. Completeness uses honest steps from programs with a different layout. With
`per_rule` 2, `honest_steps` can take the second step from another program
(here fold after fact(3)). `build_all` builds one circuit per rule, from the
first honest step's layout, so the second step's statement does not fit
(`synthesis_error` for the reference too). `complete` then is false for every
build, and about half of the sampled mutants are untested. The self-test uses
`per_rule` 1 and does not see this. Fix: key circuits by (rule, program) or by
layout, and make the grade fail when the reference does not accept every honest
pair. Wave-1 components with steps in more than one program are exposed to the
same defect.

D2. The wall budget sets the coverage, and the coverage depends on the
candidate. The debug-profile grader takes 0.7 to 6.5 s per mutant. No grade
reached 4000 mutants. A smaller candidate gets more mutants in the same budget
(admin publish B: 2131 against 1186), so a wall budget confounds the primary
outcome with circuit size. The smoke used a 1800 s slot, not the 3600 s of
protocol §6. Fix options: grade with an optimized test profile; fix the count
of mutants (same seeded prefix for every build of a component) and let the
slot follow; or compare builds only on the mutant ids that every build of the
component ran.

D3. Peak RSS of the batch was 15.9 GB against the 16 GiB (17.2 GB) per-run cap,
7 % headroom. The earlier estimate was 11.5 GB.

D4. `grade-batch` writes `cell.json` records only after the whole batch, so a
killed batch leaves stale records although receipts exist.

D5. The batch reruns the grader self-test (914 s) every time. One self-test per
grader commit is enough.

D6. Builder and skeleton lanes run `cargo check` outside `scripts/doctorwatson
heavy`. Under the host rule of 2026-11-09 (all heavy work in heavy.slice) they
must go through it or be shown to be light.
