# Current Core r3: captured local UNIT evidence

This snapshot preserves the October 15 source, frozen expectations, operator, actual results, reviews, and root adoption.
Publication copies do not establish a new execution result.
Historical source-only reports retain their original status text.
The later [root adoption](root-records/ADOPTION.json) records the qualified result.

## Read the evidence

- [Qualified root adoption](root-records/ADOPTION.json) records the adopted claims, exclusions, grades, dissent, and HW-24 tally.
- [Opus actual-result review](reviews/actual-result/REPORT.md) explains the observed results and their limits.
- [Opus source review](reviews/source/REPORT.md) preserves the earlier source-phase findings and votes.
- [Candidate source manifest](inputs/resume-source/candidate/SOURCE-MANIFEST.json) pins 126 source packet files.
- [Build manifest](inputs/resume-source/candidate/BUILD-MANIFEST.json) pins 16 compiler build files.
- [Compiler source](inputs/resume-source/candidate/source/mori-r1cs/src/lib.rs) and [tests](inputs/resume-source/candidate/source/mori-r1cs/tests/generic_pipeline.rs) retain exact bytes.
- [Frozen Core closure](inputs/resume-source/candidate/CORE-CLOSURE-PINS.json) identifies the 70 copied Core files.
- [Frozen reviewer packet](inputs/review-frozen/REVIEW-MANIFEST.json), [r2 overlay](inputs/review-r2/R2-MANIFEST.json), and [r3 overlay](inputs/resume-source/review-overlay/R3-MANIFEST.json) identify the test inputs.
- [Actual result manifest](root-records/RESULT-MANIFEST.json) identifies 52 task files and five root records.
- [Actual operator](actual/tools/run_r3.py), [execution recipe](actual/EXECUTION-PROPOSAL.json), and [FREEZE](actual/FREEZE.json) preserve the observed invocation contract.
- [Actual results](actual/run/RESULT.json), [test output](actual/run/E5-r3.stdout), and [analysis](actual/analysis/ANALYSIS-20261015T201805.181231Z.json) preserve the observations.
- [All 71 OPEN rows](OPEN-ROWS.txt) reproduce exact lines from the actual test output.
- [Provenance](PROVENANCE.json) records full identities and reproduction limits.

## Observed scope

Root recorded one attempt, without retry: three controls, 13 steps, and one analysis child.
The attempt ran from 20:17:15Z to 20:18:05Z on October 15, 2026.
Core matched 61 frozen reviewer vectors: 30 accepted and 31 refused.
E4 observed 41 author/internal-agreement tests. E5-r3 observed 14 held-out harness tests.
These counts describe separate units. They are not additive coverage figures.

The source profile is `moriarty/1`, with `moriarty-core/1` semantics.
The test used Rust 1.99.0, x86_64 Linux, debug builds, ark-relations 0.4.0, and BLS12-381 Fr.
`DirectFieldV0` remains test-only. The `groth16` feature compiled only.

The 574 reported rows retain these grades from root adoption:

| Grade | Rows |
|---|---:|
| Independent pre-freeze reviewer oracle | 256 |
| Post-freeze reviewer rows | 53 |
| AUTH author-derived rows | 150 |
| Observation rows | 41 |
| Honest control rows | 3 |
| OPEN rows | 71 |

The OPEN rows remain accounting 35, hwtrace 26, hostile 4, mutations 3, hw25 2, and addendum4 1.
No bare total is a pass rate, security figure, or whole-language coverage measure.

## Preserved judgments and dissent

The following statements quote the Opus actual-result review verbatim:

> DirectFieldV0 is test-only. There is no ABI v1 mapping and no staged `/2` lowering.

> Setup, keys, VK, proof, verification, ledger and Preprod were **not run**.

> HW-01..07 and internal HW-23 have no adversarial internal witness on held-out programs.

> 30 have reserve, DUST, service-fee and recovery planes NOT-REPRESENTED.

> M-14 SAT documents F-04, a satisfiable gap. It is counted OK by design and is never security-closed.

> **M-11b stays OPEN CONTRADICTION.** Its `(lo + r, hi)` is the same field vector as the honest claim.

Opus preserves these additional limits verbatim:

> **Findings and gaps:** F-02, F-03, F-04 (M-14), F-05, F-08/P-5, F-10, F-14.

> **Dissent:** W-OR-01 DISSENT is preserved.
Root adoption preserves these opposing HW-24 positions verbatim:

> FOR corrected HW24 RELATION-PARTIAL, first position only

> CONFIRM frozen HW24 RELATION-EXECUTED scoped to the entry Bool wire v=2, one opposing position

Root recorded no two agreeing votes. It adopted no new HW-24 classification correction.
The original trace labels, Q1 qualification, and all OPEN rows remain unchanged.

The Opus acceptance section retains these open obligations verbatim:

> - the 20 pattern families;
> - general ACTUS (277/18/32/DS01–07) and DeFi (72/DA01–24/held-outs/intent/regression);
> - recursion/PCD, private handoff and multi-parent composition;
> - every U0–U7, M1–M17, MC, SP and G exit.

This snapshot applies to the frozen Core closure at `6c7b1fdaffec98d4fa912c9b16a3c4879f572a47`.
It does not qualify current main `8cede5a0`, Sherlock helpers, or another source tree.
Moriarty remains permissionless for public developers.
Maintainer review and execution records are internal project controls.

## Portable reconstruction

[PORTABLE-LAYOUT.json](PORTABLE-LAYOUT.json) maps 263 preserved files to the original tested relative layout.
The original frozen inputs have 70 Core files, 127 candidate files, and 64 review files.
Two separately recorded generated locks extend that layout to 263 files.

1. Select an empty directory for any separately authorized reproduction preparation.
2. Copy each map entry's `snapshot` bytes to its `testedLayout` path under that directory.
3. Compare each copied identity with its preserved manifest row.
4. Read the original execution recipe and operator before selecting any command.

The publication task did not create or test this reconstructed layout.
The preserved operator contains absolute paths and a once-only placement check.
This packet supplies no modified or relocated operator.

The [separate lock manifest](root-records/REPRO-LOCKS-MANIFEST.json) adds both actual generated `Cargo.lock` bodies.
Root compared their bytes with the identities already recorded in the original `RESULT.json`.
The original 52-file result manifest remains unchanged.
The [preparation qualification](root-records/REPRO-LOCKS-PREPARATION-QUALIFICATION.json) preserves the corrected root metadata check.
This later byte observation adds no runtime result.
The packet also retains lock-check outputs, the 102-crate dependency union, Option A identities, and illustrative lock-control files.
Registry archive/index bodies, toolchain binaries, build artifacts, and caches are omitted.
Exact offline reproduction needs the omitted dependency bodies and the pinned toolchain binaries.

The resume-source manifest declares 207 paths.
This snapshot copies 205 entries and the manifest itself.
Two inherited model wrapper files were excluded by name without access.
The immutable manifest still records their historical identities.
The [root exclusion qualification](root-records/PUBLICATION-EXCLUSION-QUALIFICATION.json) corrects the publication scope.
The publication map does not authorize copying those files.
