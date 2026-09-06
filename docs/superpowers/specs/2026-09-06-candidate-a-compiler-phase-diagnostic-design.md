# Candidate A compiler phase diagnostic design

Status: proposed diagnostic design. No wrapper or new native experiment is
implemented or authorized by this document. The completed timeout series stays
closed. A separately reviewed implementation and dispatch plan is required.

## Problem and scope

The full original funding-pilot compilation view timed out at 900 seconds with
empty JSON, stderr and GNU-time output. Its four prior validation gates passed,
but those separate invocations do not establish which phase this compilation
reached. Experimental commit `bcdd627afd89d695f261ce850b11cff0ef1491fe` preserves
that result and its limits. H1 remains unresolved.

The next diagnostic should identify the last entered and last completed public
compiler phase in one new invocation. It must preserve the actual model,
compiler transformations, CLI selections and resource limits. It does not
attempt an optimization, a larger budget, a new model, or a solver run.

Context and constraints are already fixed by the adopted compilation-view plan
and its terminal evidence. No additional product preference or visual design is
needed for this diagnostic.

## Considered approaches

1. **Observe the existing public compiler stages.** A small pinned CommonJS
   wrapper records bounded phase markers and invokes the original CLI with its
   original effective arguments. This preserves the selected transformations
   and distinguishes loading, parsing, typechecking, compilation and output.
   Instrumentation adds overhead, and the public compilation interval still
   contains several internal passes. This is the recommended next diagnostic.
2. **Use the existing `--flatten=false` contrast.** This can test whether a new
   invocation completes without full flattening, but it changes the generated
   representation. It cannot replace the required flattened input or locate
   the historical timeout. It is a possible later contrast, not part of this
   design.

Increasing verbosity alone has no source-backed promise of routine phase
markers in this installed JSON compilation path. The inspected implementation
builds the whole JSON string before writing it; empty stdout is inconclusive.

## Architecture and data flow

The new wrapper loads the exact pinned `cliCommands.js` and observes only its
six exports used by the existing compile command: `load`, `parse`, `typecheck`,
`compile`, `outputCompilationTarget`, and `outputResult`. It then loads the
original pinned `cli.js`. The original CLI retains argument parsing, defaults,
stage chaining, error reporting and ordinary process exit behavior.

The wrapper records the real launch argv separately. Before invoking the CLI,
its effective argv must equal the completed original compile argv, including
the original CLI path, original view entry, main module, JSON target,
`pilotSafety`, and verbosity zero. Every callback actually invoked by the
unchanged CLI calls its corresponding original function exactly once with the
same arguments, receiver, mutable objects and return value while observation
remains healthy. Left results may skip later callbacks; the observer must never
force a skipped stage to run or fabricate its markers. A diagnostic logging
failure follows the separate
failure rule below and cannot qualify as a compiler outcome. The
wrapper must be non-async and preserve promise identity and
fulfillment/rejection values; its observing continuation must not introduce an
additional unhandled rejection. No global exception or rejection handler is
added. The wrapper
must not implement its own compiler pipeline or substitute intermediate data.
The implementation review must prove interception works with the installed
CommonJS export bindings and the CLI's actual promise chaining.

Markers go to a separately owned file, never stdout or stderr. Each marker
contains a fixed phase name, sequence number, event kind and monotonic time.
An entry marker is written before the original function runs. A return or
promise-settlement marker distinguishes an ordinary result, a returned error
and a thrown/rejected error using the pinned result representation. No stage
object, source text, environment dump, stack graph or compiler IR is logged.
The trace is bounded to 32 records and 16 KiB. A failed trace-file open, failed
or partial record write, failed result classification, or exceeding either
bound is a diagnostic failure: the wrapper stops with
reserved exit 74 and a bounded diagnostic error message. It must not continue
with missing markers that could falsely resemble a long-running phase. The
outer recorder preserves this authentic exit separately from compiler outcomes
and admits no phase location. The logging-failure path must avoid recursive
logging and must not manufacture an original compiler exit.

`outputResult` calls `process.exit`, so it is not required to return. Its input
classification and entry marker are distinct from the authentic outer child
exit. No process-exit marker, exit replacement or signal handler is added.
Exit 74 is reserved for observation failure as defined above.
A timeout may leave only an entry marker. That is a valid partial diagnostic,
not a successful phase or successful compilation.

## Source and receipt boundaries

- Keep the 24-file compilation view, its two import additions and all 30 frozen
  originals unchanged. Preserve the original failed series and its receipts.
- Keep the installed Quint 0.32.0, Node 24.18.1 and shared runtime bytes
  unchanged. No package installation, compiler patch or Foreman edit.
- New code and run outputs use a new, exclusive diagnostic directory. Before
  dispatch, pin the wrapper, recording code, exact controls, view, reviewed
  plan, compiler modules and full shared runtime references. Archive their
  before/after bindings and record the actual dispatch HEAD.
- Preserve separate stdout, stderr, phase trace, resource file, actual argv,
  child PID/group, original child exit, timeout status and original outer tool
  terminal. Preserve incremental outer transport records outside files that
  the recorder finalizes. Missing transport metadata remains explicitly absent.
- Reuse reviewed owned-process-group cleanup. An incomplete group or uncertain
  writer blocks artifact finalization. No existing receipt directory is reused.

## Acceptance sequence

The implementation plan must supply all code, exact invocations and receipt
predicates before any execution. Each gate has independent review.

1. **Wrapper controls.** Use the already preserved small alias-control inputs:
   a successful compile and the genuine compile-only missing-alias failure.
   Compare direct and instrumented execution on each identical input under the
   same pinned runtime. Require unchanged actual exit and raw stdout/stderr;
   require byte-identical successful generated JSON. The successful callback
   order is load, parse, typecheck, compile, outputCompilationTarget,
   outputResult. The missing-alias order omits outputCompilationTarget after
   compile returns Left. Check these exact phase orders,
   left/right or thrown-error classification, bounded trace, unmodified argv,
   unchanged source/runtime pins and authentic terminal records. Add focused
   controls for promise identity/values and trace-write failure so the observer
   cannot convert a failure into a success. These are wrapper controls only.
2. **One full original-pilot diagnostic.** Only after accepted controls and a
   separately recorded root dispatch, invoke the same full original view once
   with the original 900-second wall limit and 4,096-MiB Node/JVM settings.
   Do not overlap this heavyweight command with the active A4 recovery tests.
   Do not repeat the unchanged typecheck/prefix/sample gates merely to collect
   a new trace: their exact source-bound receipts remain available, and this
   invocation supplies diagnostic evidence only.
3. **Terminal intake.** Preserve and inspect all actual records even on timeout
   or failure. Validate each marker against the frozen wrapper and native exit.
   Report only the last entered and completed phases of this new invocation.
   Missing or malformed markers make phase location unavailable. Stop without
   a retry, resource increase, deeper hook, factored compile or solver run.

If the instrumented command happens to produce valid JSON, retain it but do
not silently promote the diagnostic to the paired-input/H1 acceptance gate.
Any later use requires explicit independent review of the instrumentation and
generated predicates. A completed `compile` marker also does not imply that
serialization or stdout output completed.

## Interpretation and next decision

A timeout after an entry without its matching completion identifies an
incomplete observed public-stage interval, subject to the reviewed wrapper and
trace integrity. That interval includes observer I/O and promise-reaction
scheduling margins; bounded log volume does not bound I/O latency. It does not
prove the original function was executing at the timeout instant, identify an
internal hot spot, or locate the historical timeout's phase. Instrumentation
timing is not an uninstrumented benchmark.
Any deeper diagnostic or remedy is a new reviewed proposal informed by the
actual result. A4 completion, full A5 checking, Council and architecture
selection remain outside this design.

## Design self-review

- [x] Existing code path, terminal evidence and exact constraints inspected.
- [x] Two approaches compared; recommendation has a bounded diagnostic purpose.
- [x] Argument/object/promise preservation, failure handling and exit behavior
  are explicit implementation-review obligations.
- [x] Controls, exclusive ownership, limits and non-overlap requirements are
  specified without claiming they have run.
- [x] No placeholder, resource escalation, model change or H1 conclusion.
- [ ] Separate design/implementation-plan review and concrete dispatch.
