# First public Sherlock compiler core — October17

This is a work-in-progress source increment. The production library now prepares and admits the two supplied parameter-free Core/1 programs through the existing B owner, preserving their lets/operators and returning typed HostOnly artifacts. The candidate adds an explicit optional observation branch and CLI integration source. The public CLI binary and observation branch have not been built/executed by this result.

Actual observations against the recorded source:

- Full production core metadata/type/borrow/derive check: exit0 in28.7567seconds; all113 Rust inventory pins unchanged; the dep-info identifies60 compiled Rust source inputs.
- Production core rlib build: exit0 in45.6020seconds, the same113-file pinned inventory (60 compiled source inputs) and six completed normal dependency rlibs, no cfg(test), defaultopt0 and omitted debug symbols. Six existing warnings remain. No SDK/backend/ledger dependency was selected.
- External seven-test harness compile: exit0 in3.2433seconds. Exact seven listed tests passed separately. Both FixedQuote and FixedEligibility reached actual preparation/admission with observe=false; format/mapping/digest assertions passed. These are positive API checks, not complete event/image comparisons. A later actual capture and independent selected-field comparison is recorded below.

The earlier30second Cargo check and180second Cargo build timeouts remain preserved. Cold normal dependencies completed and core compilation began; no source error was reported before timeout. A focused direct metadata check and then direct rlib build reused only completed normal dependency artifacts. No unfinished core artifact was reused. Source, dependencies, commands, compiler and artifacts have their recorded identities; cache authenticity is not proved by hashes.

The [fresh source review](REQUEST-CHANGES.json) requests two mandatory corrections: enforce existing bounds/config checks before action selection and parameterized diagnostics, and use canonical escaped diagnostic routing. Both remain OPEN at this cut. The [anchor check](REVIEW-ANCHOR-CHECK.json) verifies all eight quote bodies and records one off-by-one line locator. Neither positive tests nor successful compilation approves those guards. Original source and review records remain intact while corrections are prepared.

The real public CLI, explicit B run observation, complete independent artifact/runtime comparison, general source/recursion/ADT support, persistent boundaries, witness/constraints/export, K/Quint/Lean-Clean correspondence, financial effects, native proof/independent verification, stagenet settlement and full Sherlock acceptance remain separate work. No SDK relation, VM, circuit, key/proof, ledger, wallet or transaction action ran in this compiler batch. Existing native holds and acceptance obligations remain unchanged.

Receipts retain exact local paths. Binary artifacts are retained locally rather than committed. The [source candidate](SOURCE-CANDIDATE.json), [positive test inputs](POSITIVE-TEST-SOURCE.json), and [membership](membership.json) bind this evidence cut. SOURCE-CANDIDATE.json preserves the author’s pre-integration status fields; the later build/test/capture receipts establish the observed state for those exact source hashes. Later changed source needs affected checks and a fresh applicable review.

## Independent actual program-table comparison

The [actual capture](actual-capture/RESULT.json) compiled a small client against that exact production rlib and called compile_host with observe=false on the same two public source/config inputs and explicit HostOnly target descriptor. Both calls exited0 and retained their complete JSON artifacts: [Quote](actual-capture/quote.stdout), [Eligibility](actual-capture/eligibility.stdout). The source has no expected program cells or evaluator substitution.

The [independent selected comparisons](actual-capture/COMPARISON-RESULT.json) matched every one of the twelve ProgramView fields and the selected source/config/target identities, admission identity/layout and compile-only envelope. Quote is exact; Eligibility uses only the [sealed bijective hygienic-name adapter](independent/R20-compiler-oracle-correction02/EXPECTED-ALPHA-ADAPTER.json), retaining raw and normalized tables and the explicit mapping. The [comparator](independent/26-compiler-compare-source/compare_compile_only.py) rejects missing selected fields, type/value/order/cardinality differences. It deliberately reports comparison_complete=false: admission bounds, full preparation report values, some wrapper evidence and all runtime/event/image/native outcomes remain unselected. Its37 synthetic checks are separate from this actual capture.

The original expected runtime event schedule contained extra Push/Scope events; the independent review requested changes before runtime execution. The [correction02 recheck](independent/25-compiler-oracle-review/ADDENDUM-02.md) closed that specific semantic defect while preserving the original oracle and nine explicit limits. The twelve program tables were unchanged by that correction. Golden expectations predate the positive tests; comparator source froze after those tests but before captured artifacts were supplied to its author. The public metadata pointers were selected from typed field declarations before capture. No output-derived expected cell was used.

The archived comparator can read these exact captures without compiling Moriarty or invoking any SDK:

```sh
python3 -I -B docs/results/sherlock-source-compile-20261017/independent/26-compiler-compare-source/compare_compile_only.py docs/results/sherlock-source-compile-20261017/actual-capture/quote.stdout --case quote
python3 -I -B docs/results/sherlock-source-compile-20261017/independent/26-compiler-compare-source/compare_compile_only.py docs/results/sherlock-source-compile-20261017/actual-capture/eligibility.stdout --case eligibility
```

Relocation is archival packaging, not another execution. Original receipts/client include paths remain tied to their recorded checkout; the committed source/config fixtures are under crates/mori-core/tests/sherlock_compile_artifact_positive. Mandatory source-review corrections remain OPEN at this cut.

The [ordinary result review](independent/32-compiler-ordinary-result-review/REVIEW.md) independently checked the selected values from retained JSON and supports this limited publication. It records the capture executable and dep-info hashes at audit time: the original capture result did not seal them at invocation time. That provenance limit is retained; no run is backdated or repeated. The review is advisory and does not approve the pending guard corrections.
