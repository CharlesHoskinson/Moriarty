# DoctorWatson local observation

Source baseline `9bcbca52bbfbc58443f6bdb287d73afb7d71f1ab`; changed candidate
files are bound in [local-result.json](local-result.json).

Root ran `./scripts/sherlock-qa.sh` locally on Python 3.14.4, with exit 0.
The checker validated eight frozen program records, 208 state rows, 200 charged
edges, eight publications and 216 fuel prefixes; both committed Rust oracle
literal transcriptions matched. All 31 tests passed. The complete
[stdout](local.stdout.json) and [test output](local.stderr.txt) are retained.

Six new regressions and their changes were authored by Opus 5.5 high, confirmed
by provider `modelUsage`. They address duplicate JSON members, nonfinite numbers,
deep nesting, malformed diagnostic overlays, byte-exact Rust readback and the
declared observation family. The [pre-fix test output](security-red.stderr.txt)
records 31 tests, eight failure entries (including subtests) and two errors.
Root applied the source patch only after that reproduction; the existing
independent oracle files were unchanged.

This is source/corpus integrity and literal comparison evidence. The Python
comparison does not establish agreement with the Rust typed comparator.
No interpreter, K, Quint, Lean/Clean, circuit, compiler, SDK, proof, native or
target execution occurred in this run. Missing adapters remain explicit gaps;
no parent roadmap exit or stagenet requirement is completed.

The user requires local CI. GitHub source publication is separate from execution;
the sole existing Actions workflow now has only manual site publication. No
automatic hosted CI is introduced. The new command performs no installation or
network call. Full candidate/result reviews are recorded separately after review.

The first run predates the user naming the factory DoctorWatson; its recorded
command/path identities remain unchanged as history. The current entrypoint is
`./scripts/doctorwatson.sh`. Later observations are recorded separately.

## Workflow design check

The predecessor [actual entrypoint record](quint/design-02.json) has exit 0 and matching
before/after source hashes. `./scripts/doctorwatson.sh --design` passed 31 Python
tests, 28 deterministic Quint scenarios and a TypeScript run requesting 10,000
samples, 60 steps and seed 1. The simulator reported no invariant violation in
25.538 seconds. The whole entrypoint took 30.272 seconds across three separately
bounded model commands; no individual command exceeded its 30-second limit.

Twenty of 25 stochastic witnesses were positive. Rebase, integration, dependency
change, scoped completion and source-only merge had zero random hits; those
paths have separate positive deterministic scenarios. The random run remains
explicitly incomplete coverage, not an exhaustive check or proof of real agents.

The [reproduced model defect](quint/16-late-attempt-RED.stdout) accepted a late
completion from a prior attempt of the same candidate. The archived RED sources
are retained as gzip files here. `JobToken` now binds candidate and attempt;
the [successor deterministic output](quint/19-terminal-guard-fix-1.stdout) records
the corrected scenario. This changed the design's result-binding requirement.

The earlier [entrypoint timeout](quint/design-01.json) remains exit 124 even though
its stdout printed a no-violation summary. The initial model timeout and later
performance improvement are retained separately. No timeout, sample count,
step ceiling or Node heap limit was increased. See the
[model result index](quint/model-result.json),
[entrypoint index](quint/entrypoint-result.json), and
[performance review](quint/performance-review.md).

This is a DoctorWatson workflow model, distinct from Sherlock's required semantic
models. No Sherlock interpreter, circuit, native proof or target execution is
established by these observations. Early development attempts 01–09 retained
hashes/output without full source snapshots; that reproducibility limit remains
disclosed in the model documentation.

The [design-03 predecessor](quint/design-03.json) added the separate directed
walk and passed all four then-current model commands. Its
[directed record](quint/directed-result.json) retains those six positive witnesses.

## Current corrected candidate

The current [design-04 entrypoint](quint/design-04.json) exited 0 with matching
before/after hashes for all seven executable inputs. Node 24.21.0 and Quint
0.32.0 were observed. The complete [stdout](quint/design-04.stdout) and
[test/shell output](quint/design-04.stderr) retain:

- 33 Python tests and the unchanged eight-program corpus counts;
- 30 main Quint scenarios and one directed-completion test;
- 10,000 default samples, 60-step ceiling, seed 1, no invariant violation in
  26.788 seconds;
- one separate 47-operation directed walk, 50-step ceiling, seed 1, all six
  witnesses positive in 64 ms;
- overall 35.243 seconds across five individually bounded model commands.

The current default random result has **seven zero-hit warnings**: reviewed,
rebased, integrated, dependenciesChanged, layerRecorded, completed and sourceOnly.
Eighteen of 25 witnesses are positive. Separate deterministic and directed
observations do not change those counts or establish exhaustive coverage.

Independent review identified upstream edits reviving cancelled/blocked tasks;
[the first RED](quint/22-terminal-dependent-RED.stdout) reproduced both phases.
A [second RED](quint/25-terminal-history-RED.stdout) reproduced lost blocked
administration history. The corrected update preserves terminal dispositions and
history; the [30-scenario GREEN](quint/26-terminal-history-GREEN-1.stdout)
passes. The [history record](quint/TERMINAL-HISTORY-CORRECTION.json) binds sources.

A [disabled-step negative control](quint/24-disabled-driver-proof-2.json)
showed that `quint run --witnesses` could still exit 0 after the walk stopped
early. The new deterministic test failed on that isolated copy and passes for
the committed walk. The isolated RED sources are retained as gzip files.

Opus 5.5 high supplied two further checker corrections and regressions. The
[actual RED](hardening-RED.stderr) recorded four failed subtests and one error
among 33 tests; the [corrected local command](hardening-GREEN.json) exited 0
with all 33 tests passing. The helper's exact verified bytes are now pinned
before execution, and invalid observed indices produce no reconstructed state.
These are the author's scoped fixes; imports, interpreter and a consistent
whole-checkout snapshot remain outside that pin's guarantee.

The [complete kit review](QUINT-REVIEW.md) distinguishes modeled properties,
constructed/trusted assumptions, deterministic reachability and sampled warnings.
Earlier positive results and timeouts retain their original source scope.
On this host `timeout` is uutils: the separately observed KILL timeout returns
124. The predecessor audit's GNU-specific 137 prediction does not describe this
host; no exit was relabeled.
