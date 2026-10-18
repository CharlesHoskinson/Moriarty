# Existing intended contract and exact expectation derivation

This is a regression for already implemented public testing behavior, not a proposal to change composition semantics.

`docs/NATIVE-CORE.md`, current lines234–241, says:

> The JSON report retains every case in manifest order, including its full actual execution or error.

Its next clause states that, when every case is exportable, expectation mismatches retain all results and produce status failed with exit1; matching suites produce passed/exit0. The complete pinned document is in current-source/. The separate staged test documentation, lines127–136, specifies that a mismatch retains its actual report and continues through later actual events and cases. The existing staged-profile test models ordinary expectation transcription errors. This regression exercises the corresponding existing disjoint public consumer, not that other manifest branch.

The current disjoint implementation directly realizes this contract: `run_case` compares actual whole Pair reports before prefix referencing, folds each comparison into case passed, continues every event, and returns every stage/table. `run` processes both cases regardless of passed=false, counts failed cases, prints the complete aggregate, and returns ExitCode::FAILURE if any case mismatches. Thus failure-wrapper expectations follow an explicit existing contract and source path; they are not measured output from a new producer.

## Inputs and original independent observations

The exact nine original source/scenario/event/manifest inputs and all ten complete Pair goldens remain byte-unchanged. The original public aggregate is839110bytes, SHA256 `0f51b2c2c1eb3d259d3c5b69689924761822e7f9b50809a4497450b2037153f5`. These public fixtures match the original independent custody with freeze SHA256 `12135cbac3e899da1f063ba2f8e31a75f9ca1fa4d51d62fdb8dff635bfce3848`. No new finance/work values or evaluator outputs were created.

Before its one public CLI invocation, the test reads the whole original aggregate and all ten complete stage goldens. It then changes only a temporary expected comparison file: left-first-2.json `/post/state/marker/value` becomes string78 instead of77. The runtime never uses this expected value as state, input, source or continuation. It remains an ordinary wrong test observation.

Required aggregate differences are exactly:

- `/status`: passed→failed.
- `/cases/0/passed`: true→false.
- `/cases/0/stages/2/passed`: true→false.

All actual financial frames, captures, original envelopes/Steps, effects/history, counters, other comparison flags, later stages and second case stay exactly the independent baseline. `DERIVED-FAILED-PUBLIC-REPORT.json` is the complete839112byte derivative with exactly those three wrapper changes. `EXPECTATION-DERIVATION.json` records its provenance and hashes. It is not newly independent custody. The test derives the same literal wrapper change in memory before invocation, never from actual output.

At the selected mismatch stage, actual marker remains77, phase1, allocated42, basis78; both members are still pending at site2 with their original full captures. All eight balance rows and both residual duty5 rows remain. At both case completions, marker77 and both observers42/11 remain; balances include reserve56,target42,reserve-treasury2 and alice10,bob10,other-treasury0; allocation5 and duty5 remain positive. Caps16/36, committed16/36, total attempted52, six full history rows and13 ordered effects remain. The complete ten reports, not these descriptive examples, are the comparison oracle.

Both case-local history/effect prefix representations are expanded through the already existing test helper and compared against each original complete Pair report. Explicit case/stage length checks prevent a truncated sequence from satisfying a zip comparison. Whole aggregate JSON and exact bytes independently preserve every remaining field.

Expected per-case exportability is an explicit premise. Data-only measurements of the complete frozen-derived cases are375179/374395 pretty ASCII bytes, depth14 and8556/8534 value nodes; each fits unchanged1MiB/depth32/65536-node case bounds. This is not an actual exporter run or resource adequacy judgment. No report cap or output limit changes.

Original oracle qualifications remain: financial/work values were independently source-derived with disclosed related-source exposure and a narrow finite Core premise; full wire is conditional on the authored codec, not independent compiler-wire proof. Literal wrapper flags and serialization do not strengthen that claim. This test establishes no new financial scenario, StageRejection behavior, security/authentication, shared head, native/proof or full operator acceptance.
