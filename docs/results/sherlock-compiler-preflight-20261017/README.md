# Compiler preflight correction — October 17

The designated fallback owner authored the outstanding B04 preflight correction,
its API integration repair and five regression tests. Root applied those exact
afterimages to the draft compiler branch. The private guard module stays private;
the compiler imports the three helpers through the public consumer module.
Fresh combined source review and regression execution remain pending.

The correction moves configuration and global source-count checks before action
selection, and selected input/name extent checks before the unsupported-parameter
diagnostic. The existing parameter-free restriction remains. The earlier
canonical diagnostic-routing edit is already present on this branch. Authorship
and compilation do not close either original review finding.

## Actual checks

| Attempt | Observation | Scope |
|---|---|---|
| [R66](66-compiler-preflight-metadata/RESULT.json) | Metadata exit 1; three private-module import errors | Original owner06 afterimages against compiler base `826cd956262f803a57a2901515296c98dc8f8165` |
| [R70](70-compiler-preflight-metadata/RESULT.json) | Corrected production metadata exit 0 in 2.014 seconds; external regression metadata exit 0 in 0.045 seconds | Exact owner API repair; 129 source/config/test files and 273 dependency/toolchain pins unchanged |
| [R71](71-compiler-integrated-metadata/RESULT.json) | Both metadata outputs emitted; child exits and timings **unrecorded** after collector failure | Changed integrated tree includes reviewed B observer merge `3adfe91aab8d846ce8e4facd411fb13ebe3790e4`; 130 source/config/test files unchanged |

R70 typechecks all five new external test functions; it does not link or run them.
Its production build emitted six existing warnings. No SDK dependency, runtime
evaluation, proof, wallet or ledger action was selected.

R71's post-compilation collector compared lexical paths and stopped on two
`families/../fragments` spellings. The [readback](71-compiler-integrated-metadata/DEPINFO-READBACK.json)
resolves both to pinned source files: 61 compiled production paths and one test
path. Source and dependency/toolchain bytes were unchanged. The collector lost
the child exit/timing values before writing them; no success code is inferred
from artifact existence. Its [failure record](71-compiler-integrated-metadata/ROOT-COLLECTOR-FAILURE.txt)
and original outputs remain intact. No compilation or test was repeated to
replace that record.

The five tests cover configuration/global precedence, selected input/name
extents, in-bounds parameterized refusal and two parameter-free admissions.
Their asserted occurrence counts are owner-authored expectations, awaiting
review; they do not replace the independent compiler oracle.

## Review reliability

A separate B comparison result review returned `SUCCESS` and claimed approval,
but invented material artifact fields, types, sizes and hashes. Root withheld
adoption and PR #173's merge. The [sealed disposition](64-b-pairwise-actual-result-review/REVIEW-DISPOSITION.json)
and [literal contradictions](64-b-pairwise-actual-result-review/LITERAL-READBACK-FINDINGS.json)
are preserved. That failure does not change the separately recorded nine actual
B comparison test passes and supplies no approval for this compiler correction.
New Gemini calls were held while smaller review packets were prepared.

The [membership](membership.json) records 48 byte-identical selected original
receipts. Original absolute paths remain archival references. Binary metadata
artifacts are omitted; their recorded hashes remain. R70's collector source is
saved; R71 ran as a Root heredoc and has no separately saved collector body.
No independent runtime instrumentation is claimed. Full public CLI execution,
the optional observation path and native/stagenet acceptance remain open.
