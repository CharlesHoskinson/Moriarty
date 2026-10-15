**SOURCE_WRITTEN.** The test source is written; none of it has been run, compiled, verified or accepted. I took no native actions: no Cargo, build, tests, models, keys, deploy, commit or push.

**Written:**
- `crates/mori-core/tests/stage_hostile.rs` is a new file with 10 focused tests. Its sha256 is `56f04e4f…3da`.
- `R/OPUS-CORE-FINISH.patch` contains that new file.
- `R/OPUS-CORE-FINISH-DETAIL.md` is the public report.
- `stage_static_cost.rs` and the independent oracle JSON values are unchanged.

**What the tests check.** Each hostile case changes one field of a real escrow continuation that is waiting at its first await. Every rejection must have attempted 0 and must report the input's committed value back unchanged.
- **Action, format, profile:** a mismatched selected action is refused with `stage_site`; a wrong continuation format or logical profile with `stage_format`.
- **H1/H2:** a one-character change to either the program digest or the plan digest is refused. So is a continuation from an edited program, in both directions.
- **H3:** sites 0, 2 and `u16::MAX`, and an unknown action, are refused with `stage_site`.
- **H4:** a renamed, retyped, missing or extra capture is refused with `stage_capture`.
- **H5:** a wrong event format, action, label, binder name or binder type, a missing binder, or a duplicated binder is refused with `stage_event`.
- **H8:** a forged committed value or a lowered stored cap is refused, and so is a selected cap above the stored cap.
- **Corrected retry:** after a rejection the input is unchanged and a corrected retry completes.
- **H13:** the same continuation resumes twice with identical results. This records that use is not tracked; it is not a replay-protection claim.
- **H25:** emitted continuations stay within 4 MiB, decode back to themselves and resume. A missing `cap` key or an unknown field is refused with `json`.
- **H17/H26:** a property test over literal and input values from 0 to 999. The hand-counted costs are 9 and 6 per segment (total 15), a false guard costs 2, and changing the literal changes both digests.

**Limitation found by the tests:** removing or raising the stored `cap` is accepted. `cap` is not covered by either digest and nothing is authenticated, so the test records this as known behaviour.

**No API corrections were needed.** I checked compatibility by reading the source only. The cost helpers are crate-private, so the integration tests reach them through `load`/`start`/`resume`. They also have their own existing in-module tests.

**Boundary check behaviour:** the resume boundary check keeps the baseline balance rule. Accounts the action transfers from or to must be present, and other balances such as the extra observer 42 are kept rather than refused. This describes what the code does; it makes no ledger-binding claim.

**Still owed:**
- Running both test files. Any failure needs a security classification, not changes to the expected values.
- The rest of the H1–H28 families: hostile resume boundaries, byte limits, and the CLI and backend stage surfaces.
- All U/M/MC/SP/G gates are still open. No approval is implied, and the three earlier timed-out jobs keep their timeout status.
