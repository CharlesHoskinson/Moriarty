# Independent compiler phase diagnostic design review

**Verdict: changes requested on two narrow design statements.** The six-export
approach is source-supported and can preserve the selected CLI pipeline, subject
to the implementation obligations below. No wrapper, control or native command
was executed during this review.

Reviewed design SHA256:
`172690a1c1b70409ec3882d8d4ede299f4c79d9d21c4cb737beec4ef420165fc`.
Reviewed source brief SHA256:
`e62792296758b7cc87cdf18c7d1c13fa0f549d3b096d964cabe3dd86aaf118ee`.

## Required clarifications

1. **Exactly-once applies to actual invocations, not all six exports.** Design
   lines 53–55 say every original function is called exactly once while
   observation is healthy. Installed `cli.js:119–124` threads results through
   `asyncChain`; `@sweet-monads/either/cjs/index.js:121–126` deliberately skips its
   callback on Left. In the genuine compile-only alias failure, the expected
   called-export sequence is load, parse, typecheck, compile, outputResult:
   `outputCompilationTarget` is correctly absent. Earlier Left results skip
   still more callbacks. Require one original call for every wrapper invocation
   the unchanged CLI actually makes, and zero forced calls for skipped stages.
   Spell out success and missing-alias trace orders. Otherwise the literal
   requirement conflicts with the proposed negative control and could induce
   precisely the pipeline substitution the design forbids.

2. **An unmatched entry identifies an incomplete observed interval, not proof
   the original function was still executing.** Design lines 136–138 currently
   narrow a timeout to the public phase. Entry logging occurs before the call;
   settlement logging occurs after fulfillment/rejection, and observation adds
   synchronous I/O plus promise-reaction scheduling. A timeout can interrupt
   those margins, including a stalled marker write before it returns a detectable
   failure. Record/byte bounds limit volume, not I/O latency. Describe only the
   last entered/completed observed public-stage interval, including observer
   work and unsettled callback scheduling. Do not infer that the original phase
   was executing at the timeout instant. Retain the existing prohibition on
   locating the historical run or a specific internal pass. This needs a wording
   correction, not another stage, longer budget or deeper instrumentation.

## Source-supported feasibility and implementation gates

Repository observation: `cliCommands.js:47–56` exports the actual functions
through the CommonJS exports object. `cli.js` requires that object and accesses
its properties when constructing the handler chain. Installing the six wrappers
before loading the original CLI can intercept these references without editing
the installed files. The implementation must check the exact resolved module and
cache identity; source inspection alone is not proof its chosen wrapper does so.

Repository observation: the first five selected functions are async; the JSON
body of `outputCompilationTarget` serializes and calls `process.stdout.write`
before fulfilling. `outputResult` is synchronous and calls process.exit(0/1).
Its entry/input classification and the real child exit are sufficient; a return
marker must not be required for it. Completion of outputCompilationTarget also
does not independently prove stdout was flushed before process.exit.

Implementation requirement: a wrapper returning an original Promise must itself
be non-async and return that identical Promise, rather than `await` it or return
the Promise produced by an observing `.then`. Preserve receiver, argument/value
identity and synchronous throws. Observation necessarily adds reactions and
timing; claim no universal scheduler transparency. Focused controls must check
same Promise identity, identical fulfillment/rejection values and original
single-call counts, including a Left that skips the next stage. They must also
check that an observer-created continuation cannot introduce an additional
unhandled rejection or turn an original throw/rejection into fulfillment. No
global rejection/exception handler may change the original CLI's error behavior.

Implementation requirement: classify Right/Left using the pinned Either
representation without serializing stage objects. A successful async-function
return of a Promise is not the same event as successful Right settlement. On a
Left, mark that phase completed-with-error; on rejection or synchronous throw,
preserve the same failure and distinguish its marker. Do not fabricate markers
for CLI stages skipped by Left propagation.

The reserved exit-74 rule is appropriately separated from compiler results, but
the concrete plan must cover open/write/partial-write/classification failures,
ensure the fatal path cannot recursively log, and retain a bounded best-effort
error message independently of trace success. A signal/timeout that preempts
that path must retain its actual exit, not be relabeled 74. No phase location is
admitted for diagnostic failure or malformed trace. The simplest compatible
choice is to omit the optional process-exit marker. If retained, explicitly
resolve its write-failure interaction with authentic process.exit(0/1); do not
replace process.exit or add signal handlers merely to obtain a marker.

The successful and compile-only alias-error controls are appropriately small and
must compare identical frozen input paths, effective argv, runtime and raw output
bytes; only the instrumented run gains the separate trace. Preserve direct and
instrumented actual terminal exits separately. Synthetic identity/write-failure
controls do not authorize new actual-model or compiler transformations. Their
exact budgets, fault trigger, predicates and invocation count belong in the
separately reviewed implementation plan before any execution.

The full diagnostic remains one later invocation at 900 seconds and 4096 MiB,
after accepted controls and separate root dispatch, without overlap with active
A4 work. Reusing the exact prior prefix/sample evidence for this diagnostic is
properly distinguished from rerunning or admitting the paired H1 experiment.
Successful diagnostic JSON requires separate review before any later use. No
factored compilation, solver, H1 conclusion or resource increase follows.

## Read-only verification performed

Read the design, source brief, installed CLI chain, six function bodies and
Either.asyncChain. Selected installed files matched the admitted runtime
manifest; this was not a whole-runtime rehash:

| Installed file relative to Quint package | SHA256 |
| --- | --- |
| `dist/src/cli.js` | `ac12595b1cb7253feec93c79417615c6eb20fc3b6a3df35c5e3530b24e90a501` |
| `dist/src/cliCommands.js` | `b18672d656782aefde254fe1021dc13fdffdcf8f4ad8709e3126fb5fa293362a` |
| `node_modules/@sweet-monads/either/cjs/index.js` | `4b789da6fbd347942c20977ccc140524dc8297179a3cd1d96a5b36327ad30757` |

Only this review file was created. No wrapper code, model, runtime, receipt,
compiler, simulator or solver was changed or run. The closed timeout observation
and its phase/measurement limits remain unchanged.
