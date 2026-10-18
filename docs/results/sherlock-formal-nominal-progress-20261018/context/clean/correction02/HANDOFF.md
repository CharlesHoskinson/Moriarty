# Clean B01 correction02: exact record-layout repair and selected tail

Preserves original B01, correction01 and both actual correction01 results. Root's real run passed
CanonicalTypeId and failed ConstructorFields; Semantics/Normal did not run. The recovered declaration
sorry warning in failed parser output establishes no theorem and is not reclassified as a success.

The exact parser errors point to trailing commas at correction01 line64column68 and115column77.
The subsequent fields were dedented relative to each record's first field. Installed Term.lean:354
uses sepByIndent; Extra.lean:202–204 uses withPosition/checkColGe for every record field. This source
inspection resolves the defect class: align continued fields with the first record field.

Only two continuation indents change: line65 from column5 to14; line116 from column9 to22.
No non-whitespace token, theorem body, signature, premise, module import, value or comment changes.
CanonicalTypeId and ConstructorFieldsNormal source copies remain byte-identical. SOURCE.diff is
the entire correction. RECORD-LAYOUT-CHECK.json records all six continued-field record forms in
these three B01 source modules with no remaining text-level dedent mismatch. It is not a Lean parser.
All correction02 subject commands remain UNRUN; Root owns the meaningful complete kernel check.

PROPOSED-ARGV.json has exactly the three remaining ordinary frontend operations: corrected
ConstructorFields, unchanged frozen Semantics, unchanged ConstructorFieldsNormal. The existing
200s/4GiB/swap0/tasks256/network-off/16MiB envelope and Lean4.33.1 stay fixed. Only corrected source
root, new output/scratch root and distinct-r2units change. Each stage is conditional on all prior
selected stages. There is NO CanonicalTypeId stage and no passed-prefix rerun.

REUSE-PASSED-CANONICAL.json pins the actual passed source/result and all five matching generated
CanonicalTypeId module artifacts (.olean/.private/.server/.ir/.ir.sig). Root verifies and byte-copies
only those into the fresh correction02/proposed-local-check/artifacts/Clean/Sherlock directory,
then uses that SINGLE package root for tail checks. Author did not copy/mutate artifacts or execute
any subject. Preserve source and artifact pins before and after Root's operation, capture actual
environment/full stdout/stderr/exit, stop on first failure and retain every prior failure.

DIAGNOSIS-EVIDENCE.json pins exact actual stdout/results and installed parser sources. Original
TOOLCHAIN-PINS.json and all source/correction freezes remain unchanged. No broader source-review,
B02/B03/kernel/constraint/Rust-refinement/native/financial/target acceptance is transferred here.
A separate text-only scan found seven analogous continuation concerns in unreviewed later B03/B04
source; they are recorded, not hidden as dependency passes or repaired as part of this minimal B01 cut.
B04 source remains unexecuted and unfinished while this correction has priority.
