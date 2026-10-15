# Votes: evidence-repair resume (literal)

Voter: `claude-opus-5-5`, source author of this resume, 2026-10-15. All earlier voters in this lineage are the same model, so independence is context-only. Nothing below is adoption, and nothing becomes a decision without the second substantive vote that the repository rule requires (AGENTS.md: two agreeing substantive votes).

## P-1 / R-01 (coverage wording)

- **First position (existing, reviewer):** result review `FIX-PROPOSALS.md` P-1 (`d7d3e402…`): replace the `COVERAGE.md` refusal-semantics sentence with the exact quoted text, and change `coverage::REFUSALS` rows `mori::core::bound` and `mori::core::liability (scenario identity differs)` from `Support::Constant` to wording that names pre-state authenticity and frame completeness. No constraint change.
- **My vote: AGREE, implemented.** The quoted sentence appears verbatim in `candidate/COVERAGE.md`. Both rows are `Support::TargetPremise` with that wording. A third `TargetPremise` row covers scenario asset/balance/liability row refusals (undeclared asset, invalid or duplicate row), which the reviewer's quote names. Only `coverage.rs` data and the BUILD pin change (`DIFFS/candidate/`). The text came from the terminated partial and is carried forward byte-identical.
- **Result:** two agreeing substantive positions on P-1, both context-only, same model.

## P-2 / R-04 (build-identity disclosure)

- **First position (existing, reviewer):** P-2 offers two options. Option 1 discloses that `compiler_build_digest` covers the `origin_build_manifest_sha256` and `vs_origin_r2` annotation bytes. Option 2 defines `mori-r1cs-build-manifest/2`.
- **Root/user direction:** choose the disclosure option; do not redesign the format.
- **My vote: DISCLOSURE, implemented.** `candidate/BUILD-IDENTITY-DELTA.json` `r04_disclosure` states the effect and the unchanged `/1` format. It also states that byte identity establishes no provenance, semantic equivalence or security property. Option 2 is not done.
- **Result:** reviewer (either option acceptable) + this vote (disclosure) agree on the disclosure option.

## New clauses (first votes only; each needs a second substantive vote)

| ID | Clause | My vote |
|---|---|---|
| NV-1 | Typed outcome classes SAT, UNSAT, NOT-ENCODABLE, NOT-DECODABLE, ASSIGNMENT-SHAPE and CIRCUIT-ERROR. Circuit, synthesis and `is_satisfied` errors FAIL. PREMISE, contradiction, target-gap and observation rows are OPEN and never OK | FOR |
| NV-2 | Inherited AUTH addendum 3 unchanged (H1-R11-r3 / H2-R5-r3 re-encoded in Core wire; HW-25-r3; M-11b contradiction; phase/work method). New dated AUTH addendum 4 for the additions and for corrections of addendum-3 prose | FOR |
| NV-3 | S-9 represented by FrameCells::Include (addendum 3) plus Core-valid in-memory programs that exceed the states, duties and trace caps (addendum 4). No compiler change | FOR |
| NV-4 | Booleanity isolation through the existing `Tamper` test hook: entry wire and public field both set to 2 on M-10 and HW-23 bases, with the first unsatisfied constraint index asserted (0, entry region) | FOR |
| NV-5 | HW-14-r3 (H1' public vector under H1), HW-15-r3 (ASSIGNMENT-SHAPE, explicitly not relation evidence), HW-18-r3 (W header constant), HW-26-r3 unsorted half; HW-26 padding NOT-APPLICABLE (OPEN) | FOR |
| NV-6 | Second relation per accepted vector with `AccountingBinding::GrossDebitAndFees` and the vector's own cap binding. Bound public sums must equal the frozen gross/fees, with +1 tamper rows and a frame variant. `gross_debit_excluding_self` is compared with Core only | FOR |
| NV-7 | Frozen phase/work_attempted observed by `simulate_metered` (cap or `u128::MAX`), with an unmetered method control. AUTH observation method | FOR |
| NV-8 | HW traceability v2 with machine `{table,row}` refs. The harness prints every HW entry as OPEN; the post-run parser resolves refs; no class is upgraded by OK refs | FOR |
| NV-9 | One parser (`libtest_rows.py`) for RESULT.json and the analysis, header split before classification; dated O_EXCL analyses, never overwriting; parser-only operator diff | FOR |
| NV-10 | E0r3 copy tool = r2 copy tool plus the r3 overlay and `r3_manifest` key (no other change) | FOR |
| NV-11 | Resource block reused unchanged (Rust 1.99 four pins, `env -i` 11 variables, Option A 102-crate union, AS 8 GiB, CPU 60/900, wall 1800, jobs 2, netns, one attempt), with a fresh scratch name | FOR (reuse of an already two-vote block; the new scratch path and step list need a second vote) |

## Kept open, not voted here

- **P-5 / F-08 (caller-supplied `CompilerBuild`):** remains the reviewer's one-vote proposal. Out of scope for this repair. I cast no vote on it.
- **M-11b v2** (addendum 3 `proposed_v2_not_adopted`): needs the reviewer's adoption. It is not executed.

## Dissent recorded

- From reading "HW-NN RELATION-EXECUTED" (inherited draft HW-05, HW-14, HW-15) as full execution: HW-05 and HW-15 are RELATION-PARTIAL in v2. HW-14 now has an executed counterpart (HW-14-r3).
- From reading the public-only M-10 UNSAT as booleanity evidence: the binding equality refuses that edit. M-10/wire isolates booleanity.
