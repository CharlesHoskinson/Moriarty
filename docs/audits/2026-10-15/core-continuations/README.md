# Core continuations: local reference and static compiler evidence

The selected captured runs contain **88 distinct selected passing Rust tests**: 17 functions have last-closure
captures (CLI 12, compatibility 5), and 71 unchanged selected functions retain
their prior captures. Separately, the first Quint increment has
two normal tests/four one-transition samples, and the lifecycle increment has nine
normal tests/32 sampled traces. This is scoped local evidence for `moriarty/2`,
`moriarty-core/2`, and unsigned static `/4` artifacts. Fresh independent Muse and Opus closure reviews are completed with qualified
local-child-feature approval, as recorded in [FINAL-REVIEW-BINDING.json](FINAL-REVIEW-BINDING.json). All U0–U7, M1–M17, MC/SP/G,
and native/target acceptance gates remain open.

The implementation baseline is
`fe01bc8b7d889427192c304b0e75d0ec719f63db`.
[SOURCE-SCOPE.json](SOURCE-SCOPE.json) and [OBSERVATIONS.json](OBSERVATIONS.json)
remain unchanged historical snapshots of candidate `e9a2b80b3cdf5bc94b11dd4b7a12a0b0c9810629`.
[SOURCE-SCOPE-CURRENT.json](SOURCE-SCOPE-CURRENT.json) and
[OBSERVATIONS-CURRENT.json](OBSERVATIONS-CURRENT.json) remain unchanged historical
78-test snapshots reviewed at `7d6c3965b89d713575ade6a8dddc4f1780b3fdd3`.
[SOURCE-SCOPE-FINAL.json](SOURCE-SCOPE-FINAL.json) and
[OBSERVATIONS-FINAL.json](OBSERVATIONS-FINAL.json) remain byte-exact historical
86-test snapshots reviewed at `dd4e4298322666c140c74343a7827000f578c848`.
[SOURCE-SCOPE-CLOSURE.json](SOURCE-SCOPE-CLOSURE.json) records closure paths and hashes;
each captured command's `start.json.gz` retains its own actual source/input pins.
A current source snapshot does not replace those historical command pins.
[OBSERVATIONS-CLOSURE.json](OBSERVATIONS-CLOSURE.json) binds the closure selected
counts to exact result receipts and explicitly records carried-forward coverage. [EVIDENCE-INVENTORY.json](EVIDENCE-INVENTORY.json) records every
copied artifact's original and stored hashes. Gzip files decompress to the exact
original bytes; nothing in this dossier reruns the product.

## Observed scope

The passing normal Core and public CLI tests exercised source loading, local
start/resume, unchanged false observations, captured local and event values,
ordered effects, positive residual duties, an ordinary business-predicate
rejection, and zero-await actions. The CLI tests also checked source-only
`check`/`inspect`, static compilation, manifest and circuit publication, and
positive artifact reopen. The compiler structure tests checked two circuits
for delivery and three for reserve, original effect ordinals, capture layouts,
and serialized IR. These are actual static compiler outputs, not native proof
or ledger execution.

The normal assertions compare pre/post snapshots, ordered effects, step
indices, step values, intermediate snapshots, captures, residual records and
logical work with independently frozen literals. The retained stdout is the
test harness report, rather than a separate dump of every CLI JSON result.
The financial observations below mean those assertions passed; they are not
independent ledger readback.

## Financial expectations and observed normal assertions

The oracle was frozen before implementation. Its unchanged SHA-256 is
`76075b6350f83ba2776a4466ed9eb4dada7e9d1da0c509a366026e6b98ec9640`.
The exact [oracle JSON](oracle/ORACLE-CONTINUATIONS.json),
[manual derivations](oracle/ORACLE-CONTINUATIONS.md), and three source specimens
are retained. The expectation tables below come from that oracle. The
`core-normal-01` and `cli-normal-05` captures report passing comparisons.

| Delivery invocation | Frozen financial/frame expectation matched by normal assertions | Attempted / committed |
|---|---|---|
| Opening | Marker 77; alice/escrow/bob/observer = 100/0/0/42; release absent | 0 / 0 |
| Start | Transfer 10 alice→escrow, then release duty 0→10; balances 90/10/0/42; site 1; capture amount=10; residual release=10 | 5 / 5 |
| `ready=false` | Same continuation, boundary, captures and residual; `advanced:false`, no segment | 2 / 5 |
| `ready=true` | Transfer 10 escrow→bob, then release duty 10→0 present; balances 90/0/10/42; idle empty frame; residual empty | 6 / 11 |

Delivery preserves marker 77 and observer 42. The frozen complete effect totals
are gross debit 20 (alice 10, escrow 10), fees 0, bob net 10, dust 0, and duty
opening/creation/accrual/discharge/closing = 0/10/0/10/0, with closing duty present.
Successful segment costs are `[5,6]`; one false observation adds attempted work
2 without changing committed work 11. These totals are the oracle's accounting
of the exact matched effects, not a separately executed aggregate ledger check.

| Reserve invocation | Frozen financial/frame expectation matched by normal assertions | Attempted / committed |
|---|---|---|
| Opening | basis=40, allocated=0, marker=77; reserve/target/treasury/observer = 100/0/0/42; existing allocation duty=5 | 0 / 0 |
| Start | Capture extra=2 and held=42; basis=35, allocated=0; balances unchanged; duty=47; site 1 | 11 / 11 |
| `quote=41` | Same site-1 continuation and boundary; no segment or financial effects | 4 / 11 |
| `quote=43` | allocated=42; transfer 42 reserve→target; discharge 42; basis=78; balances 58/42/0/42; duty=5; site 2; captures extra=2, held=42, quote=43 | 15 / 26 |
| `ready=false` | Same site-2 continuation and boundary; no segment or financial effects | 2 / 26 |
| `ready=true` | Fee 2 reserve→treasury, then Emit settled_quote(UInt 43); basis=78, allocated=42; balances 56/42/2/42; duty=5 remains residual; idle empty frame | 10 / 36 |

The complete reserve effect order is state basis, liability accrual, state
allocated, transfer, liability discharge, state basis, fee, then emit. Marker 77
and observer 42 persist. The frozen complete totals are gross debit 44,
target net 42, treasury net 2, reserve net −44, remaining custody 56, fees 2,
dust 0, and duty opening/creation/accrual/discharge/closing = 5/0/42/42/5.
Successful costs are `[11,15,10]`; the two false observations add attempted work
6 while committed work stays 36. Completion retains the positive duty 5.

The separate ordinary business-predicate case accepts `quote=51`, reaching
basis=86, allocated=42, balances 58/42/0/42 and duty 5 at site 2, committed 26.
On `ready=true`, `requires quote <= 50` rejects at original ordinal 9 after
attempted work 6. The normal tests passed the expected error, absence of a
published suffix/continuation, and retention of the prior boundary. The `/1`
compatibility specimen still rejects atomically for false input at step 2,
logical work 6, and completes with work 10 for true input.

## Exact Rust counts and retained failures

| Closure capture | Selected target | Passed | Evidence selection |
|---|---|---:|---|
| `core-normal-01` | Core normal continuations | 5 | Retained unchanged |
| `core-compat-02` | Frozen `/1` wire and identifier byte compatibility | 5 | Last-closure capture |
| `core-static-cost-01` | Opus-authored static cost tests | 13 | Retained unchanged |
| `core-hostile-02` | Expanded Opus-authored stage tests | 22 | Retained unchanged |
| `backend-normal-03` | Static compiler structure; comparator ignored in this run | 5 | Retained unchanged |
| `backend-relation-03` | Relation tests plus H20 reverse decoder case | 10 | Retained unchanged |
| `cli-normal-05` | Public CLI normal and exact refusal cases, including H18 | 12 | Last-closure capture |
| `ledger-source-closure-02` | Source-closure membership tests | 6 | Retained unchanged |
| `ledger-profile-refusals-01` | Exact ledger profile refusal/control cases | 3 | Retained unchanged |
| `backend-frame-01` | Six selected frame units; 17 others filtered out | 6 | Retained unchanged |
| `legacy-byte-comparator-01` | Selected baseline byte comparator | 1 | Retained unchanged `/1` path |
| **Total distinct selected Rust test functions** | | **88** | **17 last-closure + 71 retained unchanged** |

The 58-, 78- and 86-test observation/source snapshots remain byte-exact historical
records. CLI 11 is replaced by 12 and compatibility 4 by 5. Repeated passes,
failed runs and property-generated cases are not additional distinct functions.
The closure selection is eleven capture targets. Production code is unchanged
since `dd4e4298`; only two test files append and two public literal fixtures are
added, with documentation/evidence updates. The [closure pin comparison](execution-inputs/CAPTURE-PIN-COMPARISON-CLOSURE.json)
retains all 1086 pin observations and 22 stream digests over 11 selected captures;
all stream digests match their result receipts, with every pin difference
disclosed. The [closure production projection](execution-inputs/PRODUCTION-SOURCE-PROJECTION-CLOSURE.json)
records unchanged production source since DD. These are root byte observations
separate from independent review.
This is not a rerun of the 71 retained functions or semantic/security acceptance.

The `backend-frame-01` capture remains six passes, no failures/ignores, 17 filtered
out. Its selected call path and the legacy comparator's `/1` path remain unchanged.
Exact counts and retained-evidence qualifications are in
`OBSERVATIONS-CLOSURE.json`; earlier root projections stay historical.

- `cli-normal-01` exited 101 during compilation with an unhandled
  `Statement::Await` match in ledger `relation/mod.rs`. It executed zero tests.
  The exact [Opus statement-root patch](reviews/OPUS-STATEMENT-ROOT-FIX.patch)
  and [report](reviews/OPUS-STATEMENT-ROOT-FIX.md) are retained. The separate
  `cli-normal-02` capture passed seven tests after integration.
- `backend-normal-01` exited 101 with **4 passed, 1 failed, 1 ignored**. Its
  generic scalar specimen used unsupported operator characters and failed at
  source loading before its assertions. The specimen was corrected to the
  existing literal `and`, `or`, and `not =` spellings. The old capture remains.
  The subsequent [Opus delta classification](reviews/OPUS-CHECK-DELTAS.md)
  addresses that edit; `backend-normal-02` passed five with one comparator
  ignored. The old four passes are not added again.

Every selected command's exact start/result/stdout/stderr is retained under
[checks](checks), including the two earlier Rust failures, the failed lifecycle test typecheck,
baseline export and all nine Quint command captures. Result receipts preserve exits, elapsed times and unchanged-pin checks.
The baseline export itself is not a Rust test.

## Legacy byte comparison

The retained [baseline bundle](legacy-baseline) contains only public source,
Target JSON, `/3` artifact JSON and emitted IR. The baseline command used an
already built binary with SHA-256
`89ae66c4f8911adaedfec690a0965a88baffc3601c5765fab137c37244eff28e`, whose recorded
source commit is `95df73558ab4fb0b6d80618a76e88c3a52c38888`, distinct from the
feature baseline. No binary is included here. The
[fixture manifest](execution-inputs/LEGACY-BYTE-FIXTURE-MANIFEST.json) records
exact bidirectional leaf-set/hash equality with the export; the
[postcheck](execution-inputs/LEGACY-BYTE-FIXTURE-POSTCHECK.json) records it unchanged
after the candidate comparator. The one selected comparator passed byte equality
for this artifact and its IR. This is bounded specimen evidence, not a claim that
the entire legacy corpus or all historical CLI usage bytes were compared.

## Two Quint increments, separately counted

The six original prefix draft files remain byte-identical under
[quint-prefix](quint-prefix). Their historical unexecuted labels describe source
preparation time. Captures `quint-prefix-01..04` record both typechecks, the
TypeScript run with four one-transition samples and witness 4/4, and the two
explicit normal example tests. These are unchanged first-increment observations.

The [lifecycle appendix](quint-lifecycle/README.md) now retains standalone relative
imports, the second model, nine corrected normal example tests, exact original
input manifests, old failed test bytes and the two-line pure constructor import
patch. Its `quint-lifecycle-01` model typecheck and `-02` TypeScript run passed.
The run sampled 32 traces with at most four resume steps (lengths min 2/max 5),
with witnesses resume 32, false observation 21, completion 19 and business
rejection 10. The original test typecheck `-03` failed QNT404; `-03b` passed after
two direct imports, and `-04` passed nine tests once each. The passed model
run/typecheck were not repeated. The failed original command is retained and is
not counted as a pass.

Model counts are separate from the 88 Rust functions and from each other.
Sampled witnesses are not exhaustive checking or Rust/compiler correspondence.
The [tree summary](quint-prefix/EXECUTION-TREE-DIGESTS.json) and per-command receipts
retain pinned tools and unchanged-directory observations without copying the
488 KB package path list, Node binary, package tree or cache contents. The
[Opus classification](reviews/OPUS-QUINT-CHECK-CLASSIFICATION.md) states:
“The model uses mathematical `int`. It does not model UInt128, serialization,
ledger state, proofs or adversaries.”

## Repairs, review history and qualification limits

The original full and completion reports are copied unchanged under
[reviews](reviews), with exact requested configurations, canonical returned model
usage and terminal/start/owner receipts. Their findings remain historical, with
no automatic approval upgrade:

| Review of frozen `e9a2b80b` | Original disposition retained |
|---|---|
| [Muse full source/results](reviews/MUSE-FULL-CANDIDATE-REVIEW.md) | SOURCE APPROVE and ACTUAL-RESULTS APPROVE within its nonsecurity scope; DOCS F1 NEEDS_CHANGES |
| [Initial full Opus](reviews/OPUS-FULL-CANDIDATE-REVIEW.md) | Source and actual results NOT APPROVED; incomplete |
| [Opus completion A, Core](reviews/OPUS-COMPLETION-CORE.md) | NEEDS_CHANGES, test-only coverage |
| [Opus completion B, backend](reviews/OPUS-COMPLETION-BACKEND.md) | INCOMPLETE; source F1 and T1/T2 coverage |
| [Opus completion C, consumers](reviews/OPUS-COMPLETION-CONSUMERS.md) | NEEDS_CHANGES; coverage test F1 and remaining gaps |

The [backend repair integration](execution-inputs/BACKEND-REPAIR-INTEGRATION.json)
records exact Opus payload copies for the lowerer and relation tests without
adaptation. The [backend author report](reviews/OPUS-BACKEND-REPAIR-RESULT.md)
states that freshly compiled artifacts now pass the existing shape validator
before return, and supplies five additional relation tests. Its new captures are
5 backend normal passes, 9 relation passes and 7 CLI passes. The
[ledger coverage author report](reviews/OPUS-LEDGER-COVERAGE-REPAIR-RESULT.md)
describes expansion of the source-directory coverage test; its capture passes 6.
These reports and captures are evidence of work performed, not final security
acceptance.

The initial Core hostile-test source job timed out at 410 seconds, with no report
or returned identity. Its exact terminal receipt is retained. The completed
[Core finisher/adoption report](reviews/OPUS-CORE-HOSTILE-FINISH-RESULT.md) identifies
Opus 5.5 as **author-finisher and classifier, not independent reviewer**, and
adopts test hash `e8d3af1257d447aad9f14b566b6eb9b3005a929f26887cbe3863a546f93054a7`.
The adapter's generic `opus-independent-security-review` report tag does not
change that stated role. `core-hostile-02` captured 22 passes. Before its child
was launched, an initial precheck stopped on only an auxiliary examples README
pin; no product test ran in that precheck. The successful start metadata retains
`auxiliary_doc_delta`; this is not another product failure or extra test run.

The later successor reviews are retained verbatim, not used to erase earlier
incomplete reports. [Muse's source completion](reviews/MUSE-SUCCESSOR-SOURCE-COMPLETION.md)
approved the qualified local child-feature source at `7d6c3965`, retaining its
prior scoped results/docs approvals. [Opus A completion](reviews/OPUS-SUCCESSOR-A-COMPLETION.md)
retained required A-C1/H19 ledger controls, A-C2/H16 CLI controls and A-D1 cap
disclosure despite its conditional headline. [Opus B completion](reviews/OPUS-SUCCESSOR-B-COMPLETION.md)
accepted its source/results scope, required T3/H20 and duplicate T4/H19, and left
required normative documentation reads incomplete. These are not whole-feature
approval of the final successor.

The completed [final A source-author report](reviews/OPUS-FINAL-A-REPAIR-RESULT.md)
and [exact integration record](execution-inputs/FINAL-A-REPAIR-INTEGRATION.json)
retain four authoritative proposed payloads. The public CLI target now passed 11
and the new ledger profile target passed 3, with unchanged pins and no fixture
leftovers. Opus authored these tests and the advisory stored-cap disclosure; this
is source authorship/classification, not independent approval. Its assumption
that H20 came from a live B reviewer is not adopted: the root integration record
explicitly acknowledges the separate completed H20 author payload.

The [H20 author report](reviews/OPUS-H20-REPAIR-RESULT.md) and
[integration](execution-inputs/H20-REPAIR-INTEGRATION.json) preserve the original
nine tests and add one reverse `/4`→`/3` refusal case. The byte path pins the `/3`
format diagnostic; the `serde_json::Value` path pins the `/3` ABI diagnostic due
to key order, exactly as that report describes. `backend-relation-03` passed 10.
The [legacy source projection](execution-inputs/LEGACY-SOURCE-PROJECTION-CURRENT.json)
is a retained root public-source observation, not a rebuilt baseline executable.

The completed [dd4 Muse final review](reviews/MUSE-FINAL-INTEGRATION.md) approved
its qualified source/results/docs scope. The completed
[dd4 Opus final review](reviews/OPUS-FINAL-INTEGRATION.md) approved actual results
and docs, but marked SOURCE and INTEGRATED **NEEDS_CHANGES** for exactly F-1/H18
and F-2/H15. It completed the full H1–H28 mapping and previously unread normative
coverage and reports no production defect. Its prior finding closures and two
remaining findings remain verbatim. The
[historical dd4 binding](historical/dd4e4298/FINAL-REVIEW-BINDING.json) is explicitly
a NEEDS_CHANGES record, not current whole-feature approval.

The completed [last-two source-author report](reviews/OPUS-LAST-TWO-TESTS-RESULT.md)
and [integration record](execution-inputs/LAST-TWO-TESTS-INTEGRATION.json) retain
four exact proposed files. For F-1/H18, the added public compile test pins the
signed-Target refusal and absence of an output directory. For F-2/H15, the new
compatibility test compares Program and Execution bytes for `/1` identifiers
`await` and `when` against hand-typed literal JSON. The
[freeze note](repairs/last-two-tests/ORACLE-FREEZE-NOTE.md) records the 881-byte
Program and 2092-byte Execution fixtures frozen before assertions; their exact
payload bytes remain unchanged. CLI 12 and compatibility 5 then passed with
unchanged pins and no public fixture leftovers. Expected literals were not
regenerated from candidate output. Authorship/classification and passing captures
are not independent closure acceptance.

[REVIEW-STATUS.json](REVIEW-STATUS.json) records completed independent closure
reviews; exact dispositions and limits remain in the verbatim reports. No earlier report's
disposition is silently upgraded. The objective
[plain-capture count correction](execution-inputs/PUBLIC-CAPTURE-COUNT-CORRECTION.json)
retains the historical 108 files/27 captures while preserving B's 100/25 statement
verbatim, without inferring that the reviewer independently checked all 108.

The [whitespace observation](execution-inputs/WHITESPACE-CHECK-OBSERVATION.json)
retains the full-branch check's raw patch-context whitespace findings and the
separate source/document check excluding immutable captured `.patch` files.
Raw patches were not rewritten to make that observation disappear.

The [frozen Opus contract](design/DESIGN-OPUS.md) requires output labels
`qualification: local-reference-stage`, `authentication: none`,
`consumption: untracked`. P1 states: “Unsigned `/2` stages do not authenticate
event origin, correlation, finality or the resumer.” P3 states: “The local
evaluator is a pure function of untrusted files.” The [A1 amendment](design/DESIGN-OPUS-FINAL.md)
requires all ten `/4` gates to remain `unperformed`. This dossier supplies no
substitute security judgment. Native authority remains zero; all full U0–U7,
M1–M17 and MC/SP/G gates remain open. No native proof, ledger/target qualification,
release, merge or whole-language acceptance is asserted.

## Final closure review publication

The reviewed source is `0e42392d691d474e5a343765fba318b621f4b2bc`, tree `7cccf9c45a7a446e18bd5237cdf2aad6436176ca`. The [Muse report](reviews/MUSE-CLOSURE.md) and [Opus report](reviews/OPUS-CLOSURE.md), canonical identities and completed terminal receipts are copied verbatim. Root confirmed their qualified closure approval before this mechanical metadata append. This publication changes only review records, maintained dossier status, the external source binding, inventory and checksums. Frozen source/observation snapshots remain byte-identical. The metadata commit is bound externally after creation; no source self-reference is inserted. No new product execution, native/target qualification or parent exit is claimed.
