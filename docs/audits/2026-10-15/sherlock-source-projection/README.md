# Core source projection for Sherlock

The actual Rust Core library compiled, the original projection test file compiled
against that library, and all six tests passed. The captured run used a separate
harness with debug information stripped at link, within the unchanged 16 MiB
per-file limit, 4 GiB memory, zero swap, 256 tasks and existing time limits.
[The raw checker capture](core-check.capture.json) binds the actual sources,
compiler, selected dependency artifacts, generated harness and observed test names.
[The root result](root-result.capture.json) records unchanged inputs and empty
scratch. These are captured local results for their recorded bytes.

The [first attempt](first-link-failure.capture.json), including its
[full checker output](first-link-failure-checker.capture.json), compiled Core but hit
SIGXFSZ while linking the test executable; its tests did not run. The new,
separately reviewed encoding strips dependency debug information at link without
changing the source, tests, expectations or resource limits. The failed capture
remains retained. The [exact outer argv](limits-argv.capture.json) records the
4 GiB, zero-swap, 256-task and 16 MiB per-file settings used for the new run.
The [first outer argv](first-link-failure-limits-argv.capture.json) records the
same settings for the retained failed attempt. Cached lexer, procedural-macro and transitive dependency
source correspondence is unestablished; this is a direct rustc observation,
not a Cargo build or target qualification.

## Implemented path

`mori_core::parse` and `validate` feed `sherlock_source::prepare_scalar_action`
and `source_projection_report`. The adapter prepares an existing typed Sherlock
function with ordered dynamic parameters, lets and one final emit value. It
records actual Core expression IDs, action/statement positions and body paths.
It evaluates no expression and executes no emit effect.

The two positive specimens are WeightedTotal and ThresholdCheck. The six tests
check their handwritten target ASTs and complete occurrence paths, JSON report
fields, source action order, an explicit unsupported operator and the
Opus-authored occurrence-budget case. They do not execute those generated
functions: the separate proposed value expectations 45 and false remain unrun.
The test source retains its original pre-execution draft annotation; this dated
capture supplies the actual test disposition.

Only stateless `/1` actions with ordered lets, a final emit value, UInt/Bool,
Add/Sub/Mul/Eq/Lt and Not are projected. An unsupported projection is metadata;
it does not add a compiler refusal. Source functions, branches, recursion,
List/Tree/general ADTs, effects and financial continuation are not supplied by
this adapter. Host symbol strings do not establish frozen K numeric-name
correspondence. Existing `/2` compilation is unchanged.

The backend's `compile_reported` preserves the signed/unsigned compiler choice
and returns the unchanged artifact alongside the projection. The `/1` CLI
serializes `source-projection.json` before publishing the circuits and final
`artifact.json` completion marker, using the existing no-clobber publisher.
This backend/CLI integration is source reviewed; it was not built or executed
by the six Core checks. Its SDK-backed tests and full publication tests remain
unrun. The two Core specimens are not two qualified target compilations.

## Preparation limits and metadata meaning

Core charges at most 9216 occurrences per report, shared across actions, or a
fresh budget for a direct single-action preparation. Exhaustion produces
`projection-occurrence-budget-exhausted`. The backend additionally charges at
most 608256 units: one per path element plus one per occurrence, greedily in source action
order. An action that does not fit becomes `Unsupported` with diagnostic
`projection-path-budget-exhausted`; that withheld action is not charged against
the path budget, so a later smaller action may fit. This diagnostic withholds
metadata and does not change the compiler's supported subset.

The path bound lives in the backend wrapper. Direct Core callers retain the
occurrence bound and can still materialize quadratic let-prefix paths. Neither
bound is a measured circuit cost, a total JSON-byte cap or part of the Target's
`max_artifact_bytes`. The wrapper does not reduce Core's transient preparation
memory. The existing compiler/target limits keep their own scopes.

The JSON envelope binds exact source bytes through `source_hash` and the
compiled program through `compiled_program_hash`. It does not bind the Target,
authorization mode or artifact manifest digest. The manifest does not
reference it. Treat this file as untrusted metadata: it supplies no proof,
signed/unsigned trust, asset authority or permission to execute effects. UInt
literals use Core's canonical decimal-string serialization, including values
above u64. A JSON-only consumer must use the actual preparation API to obtain
the generated function; the report does not embed that function.

## Remaining acceptance work

Backend/CLI build and execution, complete admitted-grammar coverage, actual
source-to-machine values and prefixes, K/Quint/Lean correspondence, bounded B
representation, constraints/export/native proof and required Midnight stagenet
proof/deploy/call/readback remain open. Native authority is zero. No U/M/MC/SP/G
or B-T parent task closes from this source projection and six-test capture.
Independent Muse nonsecurity and Opus security/result reviews completed;
[their bindings](REVIEW-BINDING.json) retain the original R1 finding and its
completed corrections. Opus's R1 erratum narrows the allocation description: a
child path can be allocated before its charge; the accepted occurrence bound
stands. Captured `PENDING` labels are historical pre-review status, superseded
by the completed review bindings. Backend/CLI execution remains unobserved.
