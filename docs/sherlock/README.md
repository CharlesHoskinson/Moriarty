# Sherlock documentation

Sherlock is Moriarty’s universal interpreter and its handwritten Midnight realization. This corpus follows [Diátaxis](https://diataxis.fr/): tutorials teach through an exercise, how-to guides solve a task, reference describes interfaces, and explanation develops the design.

## Tutorial

- [First local observations](tutorials/first-local-observations.md)

## How-to guides

- [Compare complete observations](how-to/compare-complete-observations.md)
- [Find the first difference](how-to/find-first-difference.md)
- [Add a normal case](how-to/add-normal-case.md)
- [Check the stream realization](how-to/check-stream-realization.md)
- [Prepare a compile-only HostOnly source artifact](how-to/prepare-host-source-artifact.md)
- [Compile dedicated draft fixtures to HostOnly artifacts](how-to/compile-draft-program.md)
- [Observe a Core2 stage financial result](how-to/observe-stage-financial-result.md)
- [Prepare checked staged financial source](how-to/prepare-staged-financial-source.md)
- [Use typed word arithmetic](how-to/use-typed-word-arithmetic.md)
- [Prepare owned generic program types](how-to/prepare-generic-program-types.md)
- [Observe a declared recursive nominal list through the public Rust API](how-to/observe-declared-recursive-list.md)
- [Select a retained Push configuration family](how-to/select-retained-push-family.md)

## Reference

- [Components](reference/components.md)
- [Nominal declaration preparation API](reference/nominal-declaration-api.md)
- [Whole-program generic type preparation API](reference/generic-program-preparation-api.md)
- [Profiles and results](reference/profiles-results.md)
- [Resource manifest report API](reference/resource-manifest-api.md)
- [Host machine](reference/host-machine.md)
- [Core2 financial observation adapter](reference/financial-observation-adapter.md)
- [Financial carrier and public staged source producer](reference/financial-carrier-binding.md)
- [Typed word arithmetic](reference/typed-word-arithmetic.md)
- [K-A host event grouping](reference/host-grouping.md)
- [HostOnly source compiler API and artifact](reference/host-source-compiler.md)
- [Dedicated draft source compiler API and artifact](reference/draft-source-compiler.md)
- [HostOnly configuration and exact fixture](reference/host-source-config.md)
- [Image layout](reference/image-layout.md)
- [C6 physical host codecs and nominal DATA-row representation](reference/c6-physical-host-codecs.md)
- [Retained Push family and checked positive K context projection](reference/c6-retained-push-family-and-context.md)
- [Bridges and ledger](reference/bridges-ledger.md)
- [DoctorWatson](reference/doctorwatson.md)

## Explanation

- [Universal interpreter](explanation/universal-interpreter.md)
- [Observation boundaries](explanation/observation-boundaries.md)
- [Resource reporting boundaries](explanation/resource-reporting-boundaries.md)
- [Source occurrence identity](explanation/source-occurrence-identity.md)
- [Nominal declaration preparation and runtime](explanation/nominal-declaration-boundary.md)
- [Whole-program type preparation and runtime](explanation/whole-program-types-and-runtime.md)
- [Relations, witnesses and ledger](explanation/relations-witnesses-ledger.md)
- [Evidence and acceptance](explanation/evidence-and-acceptance.md)
- [Chunks and financial boundaries](explanation/chunks-and-financial-boundaries.md)
- [Decisions and lessons](explanation/decisions-and-lessons.md)

## Maintain the corpus during implementation

Every implementation batch identifies the affected tutorial, guide, reference and explanation pages. Update interfaces, examples and observed results alongside their owning code; if a category is unaffected, record that disposition in the batch result. Bind examples and claims to the source/profile/consumer actually checked. Validate relative links and named API anchors before publication. Preserve earlier results with their dates and qualifications.

The [OpenSpec](../../openspec/changes/sherlock-universal-interpreter-design/requirements.md), [correctness loop](../SHERLOCK-CORRECTNESS-LOOP.md) and [DoctorWatson local commands](../DOCTORWATSON-CI.md) retain their separate acceptance predicates. A proposed API, source implementation, ordinary host observation, formal result and native target observation each have their own status.

## Design and implementation

[Revision03](design.md), [OpenSpec/EARS adoption](../../openspec/changes/sherlock-completion-convergence/ADOPTION.md), [implementation plan](../SHERLOCK-IMPLEMENTATION-PLAN.md), [loop contract](implementation-loop.md) and [independent reviews](reviews/README.md) define the current work. [Actual stream results](../results/sherlock-stream-20261017/README.md) and [composed333 results](../results/sherlock-composed333-20261017/README.md) retain their exact observed scope.

At the October17 integration cut, [finite Quint validation](../results/sherlock-quint-design-20261017/README.md) records44 passing tests, five passing directed/sampled runs and38 reached witnesses. The independent result review closes that ordinary entry condition. Root subsequently armed the new full implementation goal (creation epoch1792260805); this dated observation does not attest that a worker is running when this page is read. Compiler implementation and native/target acceptance remain separate work.

The [first compiler-core result](../results/sherlock-source-compile-20261017/README.md) adds actual production-library compilation and seven positive tests, including two source-to-B preparation/admission cases. At that historical cut, mandatory guard/diagnostic corrections and complete independent artifact comparison remain open. This is a source increment; public CLI and native/target acceptance are separate.

The [dedicated draft compiler source](reference/draft-source-compiler.md) adds a
public bounded preparation route for closed entries with typed functions,
nested calls, list/tree constructors and a named correspondence report. The
[2026-10-18 observed result](../results/sherlock-public-draft-compiler-20261018/README.md)
records current Core compilation, external public consumer compilation and two
passing public fixture tests, plus seven passing bounded-copy tests. All four
sealed comparison fields (including all twelve program tables), the full
26-obligation correspondence object and selected artifact metadata match. Fresh
source review and corrected final result review are complete; original
UIREQ-022 is adopted. At that compile-only cut, runtime 7/8 values were
unobserved; the later public runtime result below is separate. The CLI, guide’s
file-writing example, semantic proof and native/target evidence remain unobserved.

The [Core2 financial observation result](../results/sherlock-financial-observation-20261018/README.md)
records the positive start/false-await/true-resume observation test and later
canonical compilation without another test run. Its scoped result and canonical promotion reviews are complete, with the
original opinions and factual corrections retained. The read-only adapter
does not establish a Sherlock financial state/step producer or close the general
UIREQ-017 clause, UIREQ-018 lowering or UIREQ-019 later local failure.


Original UIREQ-022 was subsequently adopted after corrected final result review.
See the [Root adoption](../results/sherlock-public-draft-compiler-20261018/ROOT-ORIGINAL022-ADOPTION.json).
This accepts the original named-mapping report clause. All 26 semantic
correspondence obligations remain unproved; runtime, CLI, witness/constraint,
native and target evidence retain their separate requirements.


The [supported-profile field](reference/draft-source-compiler.md#supported-profile-report)
adds an explicit grammar/domain/schedule/extents report using the admitted
invocation's config. The public API arguments are unchanged. R256 passed normal
Core compilation, public consumer compilation, listing and both new exact tests,
with 723 selected inputs unchanged. R257 approved the supplied source and exact
plan. After withholding the inaccurate R263 reply, Root adopted original
UIREQ-002 following fresh R272 literal-result review. The current evidence
check at that adoption cut recorded eleven accepted originals and 24 open. See the
[profile result and adoption](../results/sherlock-supported-profile-20261018/README.md).
The passing report tests were not rerun to repair the review reply. Use the
[inspection guide](how-to/compile-draft-program.md#inspect-the-supported-profile-field)
without treating its unrun file-writing example as a captured printout.

The installed [public runtime API](reference/draft-source-compiler.md#public-draft-runtime-api)
now has [two observed host cases](../results/sherlock-public-runtime-20261018/README.md).
R279 passed external consumer compilation, listing and exact ChainTotal/BranchTotal
tests, reusing the unchanged Core build with all 636 selected pins unchanged.
The tests assert actual UInt 7/8, complete ordered tables, call order/counts and
relational event/counter/resource contracts. Raw stdout contains test status;
full traces and exact independent B-clock totals are absent. Successful R281/R286
reviews preceded [Root's scoped adoption](../results/sherlock-public-runtime-20261018/ROOT-ADOPTION.json)
and exact three-source/seven-test-file installation. Historical capacity/listing
failures and R265 terminal ERROR remain preserved, not approval.

PR187 merged the six-file supported-profile increment and original UIREQ-002
adoption; the retained baseline is eleven adopted originals. The runtime result
adds no new EARS row. Source-reference compatibility work retains those acceptance
records rather than inventing a fresh scenario run. CLI/file publication, full
semantic correspondence, physical H1, native/target evidence and broader language
coverage remain separate. No new circuit or website iteration occurred.


The installed [simulate command/report](reference/draft-source-compiler.md#simulate-source-integration-observed-consumers-cli-unrun)
source is installed as fourteen exact files following R300/R311 review. R299
passed seventeen phases and its one-test/24-row shared production argument
consumer; R301 compiled the report composition with its additive metadata
corrections retained. R310 then passed three phases and one new test looping two
programs through real run/report/Serialize/JSON, checking independent UInt7/8 and
same-run report bindings with 457 pins unchanged. See the
[intended command shape](how-to/compile-draft-program.md#pending-command-shape-unrun).
R322 later passed compilation, two direct real selected-handler processes and a
saved-files-only verifier. Genuine Args, production source/config/target reads and
JSON export/printing produced complete 59,814/64,160-byte reports; frozen R324
predicates checked independent UInt 7/8 and declared bindings without a second
owner run. Full root CLI/global dispatch and file publication remain unrun,
without an exact B-clock/full trace.
R315 returned SUCCESS and Root installed all six consumer files, verified all
twenty source/test files and applied the reviewed R316 reporting-reference update.
The existing reader is CURRENT11/35 with no findings, 24 originals and all 25
refinements open. The old CURRENT10/stale025 negative remains preserved; no new
EARS adoption or reader run is claimed by this docs update. B04 stays unchecked.
See the [source and receipt archive](../results/sherlock-simulate-report-20261018/README.md).


## Canonical runtime type identities

The canonical TypeId migration passed three compiles and ten new tests. Read the [interface reference](reference/canonical-runtime-types.md), [inspection guide](how-to/inspect-canonical-runtime-types.md), [builtin walkthrough](tutorials/first-local-observations.md#follow-one-builtin-identity-across-six-carriers) and [runtime explanation](explanation/whole-program-types-and-runtime.md#canonical-typeid-through-the-existing-owner). [Actual receipts](../results/sherlock-canonical-runtime-types-20261018/ACTUAL-TEN-TESTS-SCOPE.json) cover four builtin live-carrier cases, five V8 cases and nominal storage only. Generic nominal runtime, original UIREQ034 and all 25 refinements remain open.


## Offline K capture utilities

- How-to: [Assemble observations from an existing F03 capture set](how-to/assemble-offline-f03-observations.md).
- Reference: [Offline K capture adapters](reference/offline-k-capture-adapters.md).

These pages describe supplied-file assembly and comparison. The archived eleven guards and four installed-decoder regressions are separate from producer execution and native/target evidence.


## Recursive child, parent and terminal handoff — source cut October 19

- Reference: [exact child/parent/terminal interfaces and 16+12 field order](reference/recursive-child-parent-terminal.md).
- How-to: [inspect the handoff before a local rehearsal](how-to/inspect-recursive-proof-handoff.md).
- Explanation: [why the parent, terminal and anchor remain separate](explanation/recursive-proof-boundaries.md).

These pages accompany actual source/results publication. They add no accepted recursive proof, deployed target, new EARS closure or separate execution route. A new tutorial is deferred until a connected actual path exists; no fictitious runnable command is introduced.


## Declared recursive nominal list — observed October 20

The [public Rust API guide](how-to/observe-declared-recursive-list.md) follows one actual `ListUInt128` source through body typing, owner admission/genesis and complete owner records. [The retained evidence](../results/sherlock-original007-recursive-nominal-list-20261020/README.md) contains 70 events, one Terminal and all nine outputs, including 846,469 raw trace bytes. This source-aware finite result does not close UIREQ007 or its binder/formal/full-consumer/refinement obligations. The [layered explanation](explanation/nominal-text-to-admission.md#observed-body-typed-owner-path-october-20) keeps K/Quint/Clean, DATA constraints, native proof and target acceptance separate.


## Complete positive META counterpart — October 20

[The B38 public source/result](../../models/sherlock/quint/c6-positive-metadata-20261020/README.md), merged in [PR326](https://github.com/CharlesHoskinson/Moriarty/pull/326), records one frontend and two directed tests passing. Its full six-field metadata observations were compared internally; no actual vector JSON or Rust codec comparison is claimed. Six public modules need a withheld seventh expected-data module to run the test entry. The [retained-family reference](reference/c6-retained-push-family-and-context.md#later-positive-meta-evidence--2026-10-20) and [inspection guide](how-to/select-retained-push-family.md#inspect-the-later-positive-meta-result-without-changing-its-scope) preserve those limits and keep the later K META compile-only result, local Fin proof, pending handwritten source and native work separate. No full EARS, recursive-proof or target acceptance follows.
