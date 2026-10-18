# Whole-program generic type preparation — October18,2026

Four ordinary operations passed: normal Core compilation (3.970211877s), external
consumer compilation (0.412264438s), and the two exact preparation tests
`generic_fifo_after_signature_full_preparation` (0.005430591s) and
`signature_only_reuse_full_preparation` (0.004151716s). Each test passed one
selected case with the other filtered out. All427 declared input pins matched
whole-attempt pre/post checks; no continuous or per-phase immutability is claimed.

The two cases compare complete prepared names, UTF8 totals, declaration NameIds,
type/type-argument/constructor/field vectors and metadata, exact T3 records and
retained owned source against independently frozen expectations. These comparisons
occurred in memory. Stdout contains test status; no actual PreparedExtProgramSource
JSON was persisted. The packaged JSON goldens are expected data, not captures.
Normal Core stderr retains all11 warnings, including two in the candidate T3 leaf
(unused imports and unused helper); compilation had no errors.

- [Exact operations and results](evidence/actual-four-one-builder-01/RESULT.json)
  and [full four argv](evidence/PROPOSED-OPERATIONS.json); phase receipts and all
  eight rawstreams are alongside the result.
- [Source review](reviews/source/REPORT.md) and
  [same-reviewer actual-result/installation/compatibility review](reviews/postresult/REPORT.md)
  are complete original reports, with public terminal/identity/pin receipts.
  Both are one independent reviewer context, not two independent votes. The
  actual reviewer was Gemini4ArgonHIGH under the standing observed Opus outage.
- [Seven production afterimages](source/production/AFTERIMAGES.json) and
  [six reviewed primary consumer files](source/consumer/PRIMARY-CONSUMER-CLOSURE.json)
  preserve exact source. Historical SOURCE_ONLY/UNRUN headers are unchanged;
  their later actual result is the separate receipt above.
- [Independent oracle freeze](oracle/EXPECTATION-FREEZE.json) and
  [derivation](oracle/DERIVATION-AND-LIMITATIONS.md) distinguish expected values
  from results. The historical freeze also pins optional Schedule artifacts;
  their payloads are excluded here. No optional Schedule test was reviewed or run.
- [Source-body preservation](metadata/SOURCE-BODY-PRESERVATION.json),
  [all59 reference changes](metadata/COMPLETE-59-REFERENCE-CHANGES.json) and
  [editorial qualifications](EDITORIAL-QUALIFICATIONS.json) retain compatibility
  scope and correct descriptive aliases without rewriting raw records.

The public API is `shared::prepare_ext_program_types`, returning the complete
source-bearing `PreparedExtProgramSource`. T1/T2 handle declaration registration
and FIFO fields. T3 visits signature types first, then entry/function source uses,
sharing one checked builder and draining newly discovered keys after each use.

UIREQ034 remains OPEN; the original recursive Schedule runtime scenario is UNRUN.
This is partial preparation evidence, with no admitted generic ProgramView,
Pass C/P lowering, ConstructorFields lifecycle, first-arm MatchPlan execution,
runtime values/work trace, financial extension or native/target result. Previous
closed originals retain their separately reviewed compatibility scope. No binary,
rlib, SDK payload, provider input/NDJSON/private reasoning or dependency source
cache is packaged. Gzip files reconstruct exact immutable original JSON; the
copy map records original and compressed hashes. Absolute runtime paths inside
historical records remain verbatim. Rebuilding elsewhere requires its own pinned
toolchain/dependency environment; this archive is an evidence/source package.

Root writes the separate `ROOT-PREPARATION-FEATURE-ADOPTION.json` and any final
installation/currentness record when integrating this prepared public tree.
