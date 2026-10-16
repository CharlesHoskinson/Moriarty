# Sherlock streaming host observations — October 16, 2026

This delivery adds `Machine::visit_steps(max_events, visitor)` to the typed
`sherlock-k-a/1` host interpreter, with public `VisitResult` and `VisitStop`.
It observes each actual `step()` record by reference, drops it before the next
step, and retains no internal history. Zero budget is inert. Terminal
publication takes precedence over an observer stop; later calls resume the
same machine.

The proposed module change exports the API through `mori_core::sherlock`.
The [exact patch](future-wiring.patch) was applied only to the production compile
snapshot for this capture. Root owns applying it to the live checkout.

## Observed evidence

The [result](RESULT.json) records two separate observations. Eight independent
normal tests passed through the actual leaf modules using a standalone stdlib
harness: zero failures, ignored tests or filtered tests. They check complete
N01/N02 records, selected N05 recursive caller states, all N02 observer cuts and
resumption, zero budget, and terminal publication precedence.

A separate fresh production `mori_core` rlib compile used the real `lib.rs`
and projected module exports. It exited zero in 1.62 seconds with empty stderr.
Compiler dep-info lists exactly 26 source files. All five source, manifest,
dependency, toolchain and child postflight checks passed. No test was rerun
through the production `cfg(test)` umbrella, and no rlib consumer was run.
The compiled rlib and dependency binaries are not published here.

## Source dependency and integration

The recorded Git base is `f7ca8ed381daf986be651ba65745310334994bbf`.
The production snapshot also contains the separately delivered
[bounded Core source API](../sherlock-bounded-source-delivery/README.md), which
was not committed at that base. Its `lib.rs` wiring and bounded API are explicit
dependencies in [SOURCE-BINDINGS.json](SOURCE-BINDINGS.json).
Merge that prerequisite first, then compare all captured production source
bytes and the projected `mod.rs` before applying the streaming patch. A Git
merge alone does not establish that source identity.

## Limits and records

This is an unmetered H1 host diagnostic. Event budgets group observations;
callbacks, snapshot cloning, helpers, allocations and native stack use retain
their existing limits and gaps. No untrusted source, CLI, LSP, MCP or network
consumer is connected. A fuel or event budget does not establish host time,
heap or stack admission. The fixed normal cases do not cover every rejection
or exhaustion path through the visitor.

Each child had a 30-second wall clock, a minimal environment, separate process
group and timeout kill. Inventory work in the parent had no time cap. No memory,
disk or process-count bound is claimed. The production compile reused six
selected Rust libraries and two proc macros from the locally trusted artifact
directory. Source authenticity of transitive dependencies is unestablished.

Direct receipts, stdout/stderr, dep-info, driver text, source manifests,
attribution records and scoped review reports are under `records/`. Original
absolute paths remain provenance for those recorded jobs; the relocated records
are not a runnable source snapshot. Complete dependency/tool inventory files
remain in the local runtime. The published inventory summary records their
identical before/after digests and explicitly selected artifact identities.
Provider raw output, compiled objects, cached binaries and private material
are excluded.

K, Quint, Lean, Clean, handwritten-circuit, native and target evidence remain
separate. Native authority is zero. This delivery closes no U, M, MC, SP or G
exit and establishes no whole-language or target acceptance.
