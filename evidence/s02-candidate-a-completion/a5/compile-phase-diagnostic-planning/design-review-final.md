# Independent compiler phase diagnostic design final review

**Design verdict: PASS. No remaining concrete design blocker.** This is a
source/design review only. A complete independently reviewed implementation plan,
accepted small controls and a separate actual dispatch remain prerequisites to
the proposed full diagnostic. Nothing was implemented or executed by this review.

Reviewed revised design:
`docs/superpowers/specs/2026-09-06-candidate-a-compiler-phase-diagnostic-design.md`.
SHA256: `19ff70c32004d824ee1b5e34e722425a354412d3548f321a943b6e32fc9ca67a`.

The earlier requested-changes review remains unchanged at
`a5-compiler-phase-design-review-20260906.md`, SHA256
`923af7c50626de40ed2db893662e6715e7ad52b5411ad1b9f11f1c3388207191`.
Its selected-source inspection and runtime-manifest matches remain the basis of
this re-intake; no new compiler/runtime execution or whole-runtime rehash occurred.

Both concrete findings are resolved:

- Exactly-once now applies to each callback actually invoked by the unchanged
  CLI. Left results may skip callbacks, and the observer neither forces calls
  nor fabricates markers. The documented success sequence includes all six
  exports; the compile-only missing-alias sequence correctly omits
  outputCompilationTarget and proceeds to outputResult. This agrees with the
  inspected CLI chain and Either.asyncChain implementation.
- A timeout now identifies only an incomplete observed public-stage interval,
  including observer I/O and promise-reaction margins. The design explicitly
  disclaims bounded I/O latency, active execution of the original function at
  the timeout instant, an internal hotspot or the historical timeout phase.

The revised design also makes the critical preservation conditions explicit:
non-async wrappers return identical promises and preserve fulfillment/rejection
values; observing continuations must not introduce extra unhandled rejections;
no global exception/rejection handlers are added. Argument/receiver/object
identity and single original calls remain concrete implementation-review and
focused-control obligations, not an assertion that an unwritten wrapper works.

Removing the optional process-exit marker eliminates its write-failure ambiguity.
outputResult's entry/input classification remains separate from its authentic
process.exit(0/1). No exit replacement or signal handler is introduced. Diagnostic
open/write/partial-write/classification/bound failures are distinguished by
reserved exit 74 with bounded reporting and no phase admission. The eventual
recorder must still retain actual signal/timeout outcomes if they preempt that
failure path, rather than inventing an exit 74.

The small-control scope remains appropriate: identical frozen successful and
compile-only-error inputs compare actual exits and raw outputs, with byte-exact
successful JSON; synthetic promise/write-failure controls establish observer
properties without changing the model. Exact budgets, fixtures, injection
mechanism, result predicates and invocation count belong to the required complete
implementation plan. This design PASS does not waive that gate.

The sole proposed full diagnostic retains 900 seconds and 4096 MiB, cannot
overlap active A4 recovery tests, and stops at terminal intake without retry,
deeper hooks, factored compilation or solver. Prior exact prefix/sample evidence
is reused for context; diagnostic output is not silently admitted as paired
input or H1 evidence. The immutable 24-file view, two existing import additions,
30 frozen originals, installed runtime, original timeout series and all receipt
boundaries remain protected.

Only this new review file was created. No wrapper, recorder, model, runtime,
native control, compilation, replay or solver was changed or run. H1 remains
unresolved; no A4/A5 completion, Council conclusion or resource increase follows.
