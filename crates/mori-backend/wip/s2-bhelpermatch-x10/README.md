# Preserved: sherlock-s2 BHelperMatch x10 fragment (not compiled)

Source: `origin/sherlock-s2` commit `dd286467f` ("WIP (unverified): BHelperMatch x10 fragment", 2026-10-25 20:46).

During the 2026-10-26 stranded-lane recovery, `sherlock-s7` (`5d7663275`, "WIP U023 Match helper x5…") and
`sherlock-s2` had each added `c6_match_helper_step.rs` and `c6_match_helper_tests.rs` with the same function
names but different bodies (s7: 455/247 lines, rows entry/arm/occurrence/field/binding; s2: 513/273 lines, ten rows).
The merge keeps s7's version under `src/sherlock_machine/` because it was merged first and carries the consumer
wiring. These are s2's exact bytes, kept outside `src/` so they are not compiled.

Reconcile by running both against the U023 holdout and the C6 fragment tests, then keep the one that passes
(or the ten-row version if both pass). Neither version has an observed passing U023 run.
