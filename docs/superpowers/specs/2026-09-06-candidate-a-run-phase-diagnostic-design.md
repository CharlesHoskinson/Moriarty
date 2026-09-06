# Candidate A case014 public run-phase diagnostic

Status: proposed design only. No observer, launcher, control or native diagnostic
is implemented or authorized by this document. A concrete plan and independent
review must precede execution. The original failed canonical014 stage stays
immutable. Candidate A semantics, runtime and the admitted capture helpers stay
unchanged.

## Question and evidence

Case014 failed with a JavaScript heap OOM: original Node-6, recorder134,
RH002134 and actual tool134, with no ITF and complete unforced cleanup. Root
failed-preservation admission is
`.superpowers/sdd/a4-pilot014-failure-root-admission-20260906.json`, SHA256
`f0676a056c57464478954ecc9ae622acfee14102bfcec8079b6900584dc3f758`.
The failure has no phase trace. Empty stdout and no ITF do not prove that Rust
was never launched or identify the allocation that failed.

The source-only next-diagnostic proposal is
`.superpowers/sdd/a4-pilot014-next-diagnostic-source-proposal-20260906.md`, SHA256
`4258fbfc7cb9b32574f4cda14ad44829dd323dbb50150abf3b8d171cc9cf495a`.
It binds the prior successful aggregate typecheck and twelve native tests to
all104 unchanged Quint files and the same8102-pin runtime map. Aggregate and
standalone entries have different module sets and allocation histories. Those
successes supply prior coverage, not a location for this OOM.

Question: in one newly observed case014 run, does public typecheck complete and
public runSimulator begin before termination? Preserve failure as well as
success. An incomplete observed interval includes observer I/O and Promise
scheduling; it cannot establish the exact function executing at interruption.

## Alternatives and decision proposed

A standalone case014 typecheck is operationally simpler and supplies new
entry-specific frontend feasibility. Its information gain is limited by the
matching aggregate typecheck, and success cannot locate the original failure.

Prefer a bounded public-phase observer using the admitted non-async observation
mechanics with a newly reviewed run profile and launcher. This directly tests
which public intervals complete in the new invocation while retaining the
native command's transformations. Deeper serialization/spawn hooks could be
useful later but need additional context and identity controls. Neither deeper
hooks nor a semantic or representation change belongs to this design.

The A5 no-flatten contrast is separate: CLI run does not call compile or its
full-flatten path. It is not an A4 remedy.

## Narrow observation code

Create new diagnostic-local copies with their original predecessors retained.
Never edit the closed A5 observer or launcher. The run observer's phase list is
exactly load, parse, typecheck, runSimulator and outputResult. Reuse the
non-async wrapper algorithm: preserve receiver, original arguments, exact
return value or Promise identity and thrown/rejected values. Observe settlement
without replacing the Promise. A Left skips downstream stages under the real
CLI's existing chain. Classify actual Either values using the pinned prototype
and own type descriptor. outputResult may return undefined or exit the process;
do not manufacture a completion or install exit/global error handlers.

Keep the reviewed exclusive synchronous JSONL sink, monotonically sequenced
nonnegative clock values, at most32 records and16384 bytes, and diagnostic
failure74 for observer/launcher failure. A signal/OOM that preempts logging is
not relabeled74. The five-phase profile and actual run route need fresh
controls; old compile-control acceptance does not transfer automatically.

The new narrow launcher accepts an immutable invocation file, checks exact
pinned Node/CLI paths, exact intended effective argv and new owned trace/argv
paths, then records actual argv, installs the five hooks and loads the real CLI.
No yargs reconstruction, direct invocation of compiler stages, source rewrite,
field omission, synthetic states or backend substitution is allowed. The plan
must fix its complete validation logic and exact argv inventories.

## Capture and immutable inputs

Use unchanged RH002 around the unchanged integrated recorder, then the new
launcher. The recorder and Node inherit RH002's owned process group. Its
121-source before/after map remains complete and unchanged. Separately freeze
and archive the new observer, launcher, controls, invocation, plan/design and
root decisions before each call, since these are not in that121-source map.
The exact original source/runtime closure, both large external runtime archives,
actual helper/interpreter/tool identities and full tree inventories remain bound.
A reviewed data-only post-cleanup endpoint capture must handle a missing inner
receipt; it cannot reconstruct a missing original receipt or Node exit.

Normal recorder receipts will honestly name the launcher command. They are
not receipts for the exporter's canonical native_command and cannot satisfy
its native acceptance gate. Treat any generated ITF as diagnostic only.

Reserve fresh diagnostic root and inner stage names in the later plan. Do not
reuse `case-014-export`, its raw path, or the prior compiler diagnostic root.
The full effective CLI argv preserves the original row, entry, Rust backend,
seed42, one sample, one trace,27 steps, both noDiagnosticA4/sourceInvariantA4
invariants and completeA4 witness. The sole CLI-argument change is routing
--out-itf to a fresh diagnostic path. Do not add a verbosity or heap flag.
All inherited Node option handling stays identical to the failed invocation;
no explicit default-heap number or hard RSS cap is invented.

Exactly one full observed diagnostic is allowed after independent control
admission and separate root dispatch. RH002's900-second outer limit includes
setup, probes, endpoint snapshots, native execution and retained receipt work,
as in the failed014 invocation. No retry or increased limit. Serialize all
native work with A5. Current HEAD is captured separately from original014's
08e426c7163880b9312f1f1f029a4dde9d0e7593 source commit and the historical
producer/runtime bases. Changes to fixed inputs invalidate dispatch.

## Controls before the full slot

The exact plan must materialize and review the new code first, then retain
fresh normal, throw/reject/Left, Promise identity, receiver/argument and sink
failure controls under the existing15-second/128-MiB mock bounds. Include
open/write/partial/record/byte/classification failures and the genuine five-phase
Right and Left chains. Expected diagnostic failures remain separate from native
outcomes; all owned groups must be terminal and clean.

Use tiny deterministic Quint success and invariant-violation fixtures, each
run directly and through the launcher under120-second outer limits with no
extra native heap override. Both variants of each pair use identical source,
seed and selections, differing only in trace output destination and observation.
Fixtures must export the same selected invariant/witness names and support the
same run argument shape. Success reaches completion; violation occurs after a
real transition and returns the CLI's genuine failed result. The exact plan must
fix both source bodies and expected exits/trace/state/witness predicates from
pinned CLI behavior before execution.

Preserve all original streams and ITFs. Do not demand raw equality of generated
timestamps, source descriptions or runtime-duration summaries. The plan must
name every permitted comparison difference and compare all remaining relevant
fields, full deterministic states and command bindings. No generic normalization
that discards arbitrary fields is acceptable. Public traces must match actual
Right/Left phase order, including runSimulator and outputResult. These controls
establish their own bounded observations, not arbitrary observer equivalence.
A control failure stops before full dispatch and remains preserved.

## Terminal interpretation and required plan

After authentic terminal and proven cleanup, independently inspect the exact
original trace, argv, streams, process/resource records, source/runtime endpoints
and transport responses. NativeOOM/nonzero/timeout can receive failed diagnostic
preservation, never pilot acceptance. A valid trace permits only the new run's
last completed/entered public interval. runSimulator includes expression
resolution, evaluator discovery, request serialization before Rust spawn, Rust
execution, output parsing/conversion and ITF handling. A runSimulator entry
without completion does not discriminate those suboperations.

Malformed/missing trace or observer74 makes phase location unavailable. Preserve
original bytes and real exits; missing original terminals remain missing.
RH002's missing-inner and uncertain-writer limitations remain explicit. Never
hash/admit uncertain live output just because the helper emitted a sidecar hash.
Root owns takeover after transport/host loss and must not restart the command.
Save each returned tool response before the next poll; preserve actual terminal
separately and close indexes only after writers stop.

No diagnostic output substitutes for the failed canonical case, all78 exports,
final115 suite, independent semantic correspondence, model checking, Council
or H1. The later plan must fix exact code, sources, paths, exclusive archive and
dispatch ordering, complete endpoint checks, tiny comparisons, finite trace and
output-intake bounds, full dispatch and all failure paths. Any inability to do
this with unchanged capture interfaces returns to design review.

## Pinned implementation basis

Installed Quint package files are under
`/home/charl/.npm-global/lib/node_modules/@informalsystems/quint/`:

- dist/src/cli.js:308, SHA256 ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501.
- dist/src/cliCommands.js:266–395, SHA256 b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a.
- dist/src/cliHelpers.js:33–45, SHA256 4ad6b6364d2a5d07389f2518aa80013bfd01dbdbab6201b53fd6b042eb8a91dc.
- dist/src/rust/commandWrapper.js:214–259, SHA256 1f26d3f1c31529572b5be32276d2d94c19a3d0298dbf444891802f744e337e1c.

Existing A5 observer source is
`.superpowers/sdd/a5-compiler-phase-diagnostic-20260906/observer.cjs`, SHA256
59f706006a73c795c303d84e7c1e3368acd1cf8a7547bfb0d46821f0d69fbd49.
Its unchanged mechanics are the comparison baseline, not approval of a new profile.
Integrated recorder remains dc30b764e2603dfe3c39ef6e2063a7082d116dc9d3eb9a012072f468beb8f8e9;
RH002 remains d8973d3541269d1be2ce524f2482e5f5b858f15f7d9117e3fe6d5cd2171d660d.
Root read the actual source in tools014458/a276da/9c6074/9db545 and the independent
source proposal in5dd678. This is source/design evidence only.
