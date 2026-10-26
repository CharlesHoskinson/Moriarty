Write a small Quint design-check model of the recursion phase-profile accounting rules. Write only these two files: `profile.qnt` (module `profile`) and `profile_test.qnt` (module `profile_test`), in this folder.

Read first: `../../openspec/changes/recursion-phase-profile/specs/recursion-phase-profile/spec.md`, requirements RPP-003, RPP-004 and RPP-005. Use `pipeline.qnt` in this folder as a style reference.

Model one proof as a total wall interval and a set of phase intervals. Each phase has an owner, a start, an end, an exclusive flag and an optional CPU value. Use integer milliseconds. Use small bounded domains so `quint run` explores quickly.

Encode these as pure predicates, then as invariants of a small state machine. The machine adds phases, samples RSS and closes the profile.
1. The exclusive phases do not overlap. Each interval has exactly one accounting owner.
2. The exclusive sum plus the unexplained bucket equals the total.
3. A profile is valid for coverage only if the discrepancy is at most 5% and unexplained time is at most 5% of the total.
4. The peak delta is max(0, sampled_peak - entry_rss). A phase with fewer than two in-phase samples has boundary-only quality and certifies no instantaneous peak.
5. A counter that is unsupported is "none" with a reason, never zero.
6. Overhead qualification passes only if the upper bound (not the mean) is at most 1%, from at least five pairs. Otherwise it is inconclusive or failed.
7. A full event buffer marks the profile invalid. It does not change the proof outcome.

In `profile_test.qnt`, write `run` tests that pass for the honest cases. Add one seeded mutant per rule 1-7 that breaks it, with a test showing that the predicate catches the mutant.

Then run, from this folder: `quint typecheck profile.qnt`, `quint typecheck profile_test.qnt`, `quint test profile_test.qnt --main profile_test`, and `quint run profile.qnt --main profile --invariant <each invariant> --max-steps 30 --max-samples 1000`. Fix failures until all pass. Write the exact commands and outcomes to `PROFILE-RESULTS.md`.

Rules: do not commit, push or install anything. Do not edit other files. Reply DONE with a three-line summary.
