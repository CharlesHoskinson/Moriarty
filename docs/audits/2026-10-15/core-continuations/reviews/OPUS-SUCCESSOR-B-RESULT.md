The scope-B review is **incomplete overall**. I accept the backend source and the backend capture results, but six required coverage areas went unread, so this is not an approval of the whole feature or anything native. The report is at `R/OPUS-SUCCESSOR-B.md`. I'm a fresh Opus 5.5 reviewer (`claude-opus-5-5`, high effort) and I wrote none of the reviewed work. I ran only the guarded status startup and nothing was built, run or changed. Authority stays 0, counters are unchanged, and all ten `/4` gates remain unperformed.

**SOURCE: ACCEPT (scope B, local source-static `/2` only).**
- **What I read in full:** all five backend stage modules, the entire `lower_action_segment` in `lib.rs` (lines 777–1228) plus the Builder, public-input and range-check helpers it reaches, the proving and signed-relation refusal changes, every test body in `stages.rs` and `stage_relation.rs`, and the six frame unit tests.
- **Matches the frozen design:** role positions, padding, typed wires, effect ordinals, N+1 circuits, the zero-await `null` case, the 29-element domain-separated context, and decoder rules 1–8.
- **F1 closed:** fresh output is now validated at the byte-count fixed point before it is returned (`lower.rs:125-128`).
- **T1 closed:** a signed Target is refused before the source is loaded, with an exact message (`lower.rs:23-28`). The context-perturbation and domain-separation tests are in place.
- **T2 closed, with a caveat:** I couldn't find the prior B report that defines T2, so I took it to be the 14 exact-diagnostic forgeries. I traced each one through the decoder's check order and each hits its own refusal.
- **Optional findings only, no required ones:**
  - B-O1 (Low): the `/4` decoder doesn't range-check or role-check the effect, balance and liability descriptor positions; it only checks they don't overlap the pending positions.
  - B-O2 (Info): no event or provenance binding; this is the stated limit and those gates remain unperformed.
  - B-O3 (Info): the context-perturbation test doesn't cover every Target field or frame term.

**ACTUAL-RESULTS: ACCEPT for the backend captures, subject to gap G1.**
- **Fresh runs:** backend-normal-03 is 5 passed and 1 ignored; backend-relation-02 is 9 passed.
- **Carried forward:** backend-frame-01 (6) and legacy-byte-comparator-01 (1) don't need reruns. Each lists four changed pins, and none is on their call paths:
  - the `lower.rs` change sits inside the staged compiler, which neither test calls;
  - `stage_relation.rs` and `stage_hostile.rs` are separate test binaries;
  - the README is documentation.
- **Hashes:** all 417 SHA256SUMS entries verify, and the current backend file hashes match the current scope file and the fresh captures' start pins.

**DOCS: INCOMPLETE, not reviewed.**

**Unread coverage, listed in the report:**
- **G1:** the harness blocked decompressing the retained `.gz` captures. I read the raw runtime copies instead and bound them only by stream hashes plus the SHA256SUMS check.
- **G2:** legacy baseline manifest, postcheck, export capture and the `95df…eb2577` projection.
- **G3:** all Quint prefix and lifecycle material.
- **G4:** OBSERVATIONS-CURRENT, the README and all normative docs, and the 84-path manifest count.
- **G5:** a test-by-test check against the full hostile-test list in the earlier design documents.
- **G6:** the other eight capture-pin rows.

Scope A (Core/CLI internals) is excluded.
