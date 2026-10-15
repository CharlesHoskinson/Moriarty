# Core continuations: local reference and static compiler evidence

The selected captured runs contain **58 passing Rust tests**, plus a separate
first Quint model increment with two normal example tests and four sampled
one-transition traces. This is scoped local evidence for `moriarty/2`,
`moriarty-core/2`, and unsigned static `/4` artifacts. Fresh full-candidate and
actual-result Muse and Opus reviews are **PENDING**. All U0–U7, M1–M17, MC/SP/G,
and native/target acceptance gates remain open.

The implementation baseline is
`fe01bc8b7d889427192c304b0e75d0ec719f63db`.
[SOURCE-SCOPE.json](SOURCE-SCOPE.json) records relative paths and current hashes;
each captured command's `start.json.gz` retains its own actual source/input pins.
A current source snapshot does not replace those historical command pins.
[OBSERVATIONS.json](OBSERVATIONS.json) binds the selected counts to their exact
result receipts. [EVIDENCE-INVENTORY.json](EVIDENCE-INVENTORY.json) records every
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
`core-normal-01` and `cli-normal-02` captures report passing comparisons.

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

| Capture | Selected target | Passed |
|---|---|---:|
| `core-normal-01` | Core normal continuations | 5 |
| `core-compat-01` | Frozen `/1` wire compatibility | 4 |
| `core-static-cost-01` | Opus-authored static cost tests | 13 |
| `core-hostile-01` | Opus-authored stage tests | 10 |
| `cli-normal-02` | Public CLI normal continuations | 7 |
| `backend-normal-02` | Static compiler structure; one comparator ignored in this run | 5 |
| `backend-relation-01` | Opus-authored relation tests | 4 |
| `ledger-source-closure-01` | Source-closure membership tests | 3 |
| `legacy-byte-comparator-01` | Explicitly selected baseline byte comparator | 1 |
| `backend-frame-01` | Six selected Opus-authored frame unit tests; 17 other tests filtered out | 6 |
| **Total selected successful Rust tests** | | **58** |

The final `backend-frame-01` capture passed all six exact
`stage::frame::tests::` tests, with 0 failed, 0 ignored and 17 filtered out. Its
result receipt records elapsed time 10.72677956699954 seconds, unchanged source
pins and native-execution directory entries, and native actions requested 0.
The [frame postcheck](execution-inputs/FRAME-POSTCHECK.json) also records unchanged
embedded fixtures and baseline dependencies, absent Cargo configuration and no
Core property seed files. The prior selected total was 52 before this separate
check; all original captures are retained unchanged. The earlier Core-only summary is retained as a historical 32-test record; its then-current
“backend_cli_execution: not performed” field is superseded by the later captures,
not edited. Property-generated cases are not counted as additional Rust tests.

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
[checks](checks), including both failed runs, baseline export and all four Quint
commands. Result receipts preserve exits, elapsed times and unchanged-pin checks.
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

## First Quint increment, separately counted

The six original draft files are copied unchanged under [quint-prefix](quint-prefix).
Their historical “unexecuted” labels describe their creation time. The four
`quint-prefix-01` through `-04` captures record the subsequent successful model
typecheck, TypeScript run, test typecheck and TypeScript test. The run used
`--max-steps=1 --max-samples=4 --seed=20261015`: `firstBoundaryReached` was witnessed
in four of four sampled traces. The two explicit tests passed one escrow prefix
and one reserve prefix example. Two tests and four witnesses are separate from
the 58 Rust tests, and are not an exhaustive model-checking result.

The [tree digest summary](quint-prefix/EXECUTION-TREE-DIGESTS.json) retains the
original full inventory hash, Node hash, package-tree digests and file counts;
the 488 KB package-member path inventory and package/cache contents are omitted.
Per-command result receipts record tool/input/cache/directory postchecks. The
[Opus classification](reviews/OPUS-QUINT-CHECK-CLASSIFICATION.md) states:
“The model uses mathematical `int`. It does not model UInt128, serialization,
ledger state, proofs or adversaries.” A later lifecycle increment remains draft
outside this dossier and is not included as completed evidence.

## Reviews and qualification limits

The frozen design precedence, Sol adoption and final Muse design approval are
retained under [design](design). Completed Opus source/check-classification reports,
model-identity JSON, adapters/configurations and exact start/terminal receipts are
retained under [reviews](reviews), [review-adapters](review-adapters), and
[review-receipts](review-receipts). These preserve each report's stated scope,
missing reads/pins, sandbox restrictions and any missing guarded startup. For
example, the [delta result](reviews/OPUS-CHECK-DELTAS-RESULT.md) states that the
permission layer refused its `status --json` refresh. A terminal timeout is not a
completed review: the core-foundation, core-admit, backend-source and inspect
source jobs' timeout receipts remain visible. Files left by those jobs do not
supply returned model identity or approval.

Opus's [CLI classification](reviews/OPUS-CLI-CHECK-CLASSIFICATION.md) says that its
verdict “is not candidate acceptance”; the source/lowering bodies were not all
read line by line. Its [Core classification](reviews/OPUS-CORE-CHECK-CLASSIFICATION.md)
likewise distinguishes effects from correctness. The retained classifications
and delta reports apply only to their exact commands and premises. The
[frame classification](reviews/OPUS-FRAME-CHECK-CLASSIFICATION-RESULT.md) preserves
its missing startup: “I couldn't load the develop/AGENTS status because the shell
refused the combined command.” It also states that the six tests are
“structural checks I wrote, with no independent oracle, so a pass doesn't show
the frames are sound.” The exact proposed argv, classification, returned identity,
terminal/start/owner metadata, run captures and postcheck are retained. This dossier
adds no security interpretation or substitute approval.

The [frozen Opus contract](design/DESIGN-OPUS.md) requires these output labels:
`qualification: local-reference-stage`, `authentication: none`,
`consumption: untracked`. Its P1 states: “Unsigned `/2` stages do not authenticate
event origin, correlation, finality or the resumer.” Its P3 states: “The local
evaluator is a pure function of untrusted files.” The [A1 amendment](design/DESIGN-OPUS-FINAL.md)
requires all ten `/4` gates to remain `unperformed`. Those are the controlling
qualified limitations; static outputs and sampled models establish no native
constraint soundness, ledger settlement, authority, confidentiality or consumption
acceptance. Native authority remains zero.

Required evidence still absent: fresh full-candidate Muse nonsecurity audit,
fresh actual-results Muse audit, and Opus review of the full integrated source,
tests and observed results. Later lifecycle-model draft coverage,
full legacy usage-byte comparison, general native proof/target
correspondence and every retained full parent exit remain open. No commit, merge,
release or parent acceptance is asserted by this dossier.
