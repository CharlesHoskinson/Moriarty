# DoctorWatson local CI

Superseded 2026-10-24: review routing, adoption votes, consumed-attempt and no-retry rules, permanent lanes and campaign queues in this file are retired. The process is the factory loop in [AGENTS.md](../AGENTS.md); technical descriptions below stand as history of the legacy tooling.

Run the local ordinary QA consumer:

```bash
./scripts/doctorwatson.sh
```

The [entrypoint](../scripts/doctorwatson.sh) resolves its own checkout root, so it
can also be called by absolute path from another directory. It runs these commands
in order and stops on the first nonzero exit:

```bash
python3 models/sherlock/qa/check.py
python3 -m unittest discover -s models/sherlock/qa -p 'test_*.py' -v
python3 -m unittest discover -s models/doctorwatson -p 'test_*.py' -v
```

The explicit `--execute`, `--inspect` and `--test-selected` delivery
routes never fall through to this default QA path; see
[DOCTORWATSON-DELIVERY.md](DOCTORWATSON-DELIVERY.md).

An existing Python 3.10 or newer interpreter and its standard library suffice. The
default entrypoint installs no packages and invokes no network services, GitHub Actions,
Cargo workspace build, backend, SDK, K executor, Quint engine, Lean/Clean build,
prover, key generation, native verifier or deployment. There is no hosted Sherlock
workflow. The existing [site workflow](../.github/workflows/site.yml) has only a
manual `workflow_dispatch` trigger; pushes and pull requests start no hosted CI.
Manual site publication remains a separate, explicit action and is unrun here.

## Explicit Sherlock host build

```bash
DOCTORWATSON_RUSTC=/absolute/path/to/rustc \
DOCTORWATSON_OUTPUT_DIR=/absolute/path/to/new-attempt \
  ./scripts/doctorwatson.sh --host
```

Use the installed Rust compiler matching `rust-toolchain.toml`. The output
directory must be a new attempt, preserving earlier results. This mode first
runs ordinary QA, then the explicit host wrapper. It compiles the actual Sherlock
reference and fixed-input observation producer, builds and runs the bounded
stepping tests, executes the producer, and compares every complete observation
against the independent N01–N08 corpus. Each child command has a 30-second limit;
no failed stage enables its dependent stage. No tool installation, Cargo/SDK
build, native circuit, proof or ledger command is selected.

The producer calls `Machine::run_steps`, so this lane consumes new executable
Sherlock behavior. The comparison covers all 216 fuel runs, including initial
post-startup state, every complete charged record, free publication, terminal
state and absence of a further step. Width 1/2/4/8, zero-event cuts and continuation
have separate normal tests. Diagnostic K text is a declared common-domain
projection, not a canonical machine codec or typed structural equality proof.

The wrapper retains source/compiler/artifact identities, commands, raw outputs,
terminal statuses and comparisons. A separately supplied JSON file establishes
artifact comparison only; the checker does not authenticate its producer or
claim it executed Rust. The wrapper's own build/run receipt supplies the actual
local observation. The original K startup edge, K execution, interpreter Quint,
Lean/Clean, handwritten constraints, native and Midnight stagenet evidence stay
separate. Source input extraction and the new tests are included in consumed
source membership; the existing frozen expectations are unchanged.

`--host` and `--design` are separate explicit modes. The default remains ordinary
Python QA. A host pass completes this local host job, not full Sherlock or any
Moriarty roadmap exit.

## Explicit stream observation job

The existing host wrapper has a separate stream realization. It is selected
through the wrapper; `scripts/doctorwatson.sh` continues to accept only its
existing `--host` and `--design` modes.

```bash
python3 -I -B models/sherlock/qa/run_host.py \
  --rustc /absolute/path/to/1.99.0/bin/rustc \
  --output-dir /absolute/path/to/new-stream-attempt \
  --realization stream
```

Use a fresh attempt directory and the same existing scoped execution conditions
as the host job. This fixed SDK-free producer calls real `Machine::visit_steps`,
cycles observer widths 1/2/4/8, and compares each complete normal run against the
independent N01–N08 corpus. Semantic fuel is not refilled between observation
calls. The job compiles and runs eight named normal stream tests, including
zero-budget calls, observer break/resume, terminal precedence and zero-fuel free
publication. Constructor startup remains outside this post-startup comparison.
It shares the wrapper's 30-second child limits and exact compiler/source receipts.

On October 17 the integrated ordinary shell passed 241 Python tests (exit 0,
10.508 seconds). A separate actual stream job completed every stage with exit 0,
passed all eight selected Rust tests and matched all 216 complete normal runs,
with no first difference. The compiler read eight producer inputs and nine test
inputs; the full comparison/source snapshot contained 26 files. These are distinct
memberships. Both actual checker helpers were included in that snapshot.
The retained local receipts are currently unpublished; historical captures above
keep their original source bytes and counts.

The stream producer deliberately retains callback records for full comparison.
No memory, allocation or performance improvement was measured. The checker still
reports its own execution layers as `not-run` and producer authenticity as
`not-established`: it compares supplied bytes. The wrapper's separate compiler,
Rust-test and producer receipts establish this actual local host execution.
Neither result supplies K/Quint/Lean/Clean, constraints, native or target evidence.

## Source-read report

The ordinary checker's `source_reference_facts` reports bytes explicitly recorded
as inputs by that checker: sorted paths, SHA-256, byte lengths, read counts and
any changed snapshots between repeated reads. In the current default corpus check
it has eight unique paths and nine read events because the existing serializer is
read once for each oracle family. These counts describe this recorded input set;
they are not a new acceptance limit.

This is not the Python import/dependency closure, compiler membership, supplied
observation-file inventory, reviewer read extent or execution evidence. Those
are separately reported or unestablished. The helper itself opens no files and
runs no subject. See the [QA report details](../models/sherlock/qa/README.md#source-reference-facts).

## Workflow design commands

```bash
./scripts/doctorwatson.sh --design
```

This first checks that the required tools already exist, then runs ordinary QA
and typechecks, tests and samples the
[DoctorWatson workflow model](../models/doctorwatson/README.md) with the already
installed Quint CLI and Node. It selects Quint's TypeScript backend explicitly;
there is no installation, Rust backend, Apalache/`verify`, SDK or target command.
A fourth model command requires the directed walk to complete all 47 guarded
steps in a deterministic test. A fifth runs that separate directed reachability
model (one sample, 50 steps, seed 1), reporting the rare integration paths without
altering random sampling. Each model command has a 30-second wall limit and Node's old-space limit of
256 MiB; that is not a total process-memory limit. Failures and timeouts propagate,
with no silent fallback. The simulation requests 10,000 samples, 60 steps and seed 1.

Node and Quint resolve from the current PATH; `DOCTORWATSON_NODE` and
`DOCTORWATSON_QUINT` can name trusted existing executables/CLI files. Missing tools
fail before checks rather than triggering an install. Relative overrides resolve
against the checkout root, including when called from another directory.
Child model commands receive
only PATH, LANG and TMPDIR. Unknown arguments fail with usage status 2.

This workflow model checks task/review/integration behavior. It does not execute
Sherlock's semantics or replace its required K, Quint and Lean/Clean QA. The
default command remains Python-only. A sampled pass means no counterexample
observed under the recorded bounds, not exhaustive verification or a proof of the
real agents, shell, Git, circuit or interpreter implementation.

Its falsifiable predicate is that the frozen normal corpus, committed Rust oracle
literal readback and supplied complete comparisons pass the two commands above.
This checks the actual committed readback through the QA consumer. It does not
execute the Rust evaluator. Passing supplies only the observations made by those
checks for the current source.

## Evidence tiers

Current beta operator policy follows the [canonical automation override](../raw/assignments/sherlock-beta-automated-provisioning-2026-10-18.md), which removes human-only operation/tier approval steps. This ordinary entrypoint’s observed scope remains separate from authorized beta setup and native/target evidence; historical zero-authority wording does not override the current assignment.

| Tier | Consumer and required evidence | Included here |
|---|---|---|
| Ordinary QA | Frozen normal expectations, supplied complete prefix/state comparisons and committed oracle readback through the commands above | Yes; Python standard library only |
| Sherlock host | Actual source compilation, fourteen bounded normal tests, fixed-input execution and all complete prefix observations | Only explicit `--host`; post-startup normal A-profile domain |
| Sherlock stream | Actual `visit_steps` producer/test build, eight normal Rust tests and complete independent normal-run comparison | Only wrapper `--realization stream`; post-startup A-profile, retained QA records |
| DoctorWatson design | Workflow state machine, deterministic scenarios and bounded sampled invariants | Only with explicit `--design`; separate from interpreter semantic QA |
| Developer semantic checks | Required K execution and explicit finite Quint projection, with actual traces, bounds, clocks and first divergences | No; unrun by this entrypoint |
| Lean/Clean and handwritten circuits | Required reference/component observations, separate witness and constraint dispositions, and cross-layer correspondence evidence | No; unrun by this entrypoint; existing private Fenrir integration remains separate |
| Native and target qualification | Exact SDK/relation, keys, proofs, verifier and actual qualified target receipts | No native/target observation is supplied by this ordinary entrypoint; beta operator policy follows the current override |

The developer tiers remain required by the
[Sherlock correctness loop](SHERLOCK-CORRECTNESS-LOOP.md). Ordinary fixture and
readback agreement supplies no formal refinement, constrained circuit result or
Midnight target observation. Supplied observations retain their source/profile
identity; missing layers remain gaps. Existing resource ceilings, counters,
terminal failures, C4, security ownership and every U0–U7, M1–M17 and MC/SP/G
acceptance requirement remain in force. This entrypoint adds no native execution
authority and no public developer approval requirement.

Retain the tested source, Python version, exact commands, outputs and terminal
status when citing a run. Source and result reviews retain the current repository
routing and exact candidate scope. Running this script locally supplies no
evidence about execution in another environment.

On October 16, 2026, the entrypoint was run by absolute path from `/tmp` with
Python 3.14.4. The checker passed eight programs, 208 literal states, 200 charged
edges, eight publications and 216 fuel prefixes; all 25 QA unit tests passed.
A separate temporary fixture copy with a changed frozen JSON file returned exit
2 from the real checker. The shell trace showed that the unit-test command was
not invoked after that failure. Repository fixtures were unchanged. These are
local ordinary QA observations only; the other tiers above were unrun.

## Separate local site checks

For an already prepared site environment with Node 24 or newer and its existing
dependencies, run these existing package commands from `site/`:

```bash
npm run check
npm test
npm run build
```

The site's `npm run verify` combines those same commands. They do not include
the separate browser checks against a served build. Site checks, browser setup,
browser execution and manual publication were not run for this entrypoint change.

A later Opus-owned correction added six malformed-input and exact-byte readback
regressions. Root reproduced the failures before applying the fixes, then ran
the local entrypoint: all 31 tests passed with the same corpus counts. The
[recorded results](audits/2026-10-16/doctorwatson/README.md) retain both
RED and GREEN observations and exact tested source hashes.

Historical `design-03` ran all four then-current bounded model commands through the actual script
from another directory, with unchanged input hashes and exit 0. It passed 31
Python tests and 28 Quint scenarios, reported no invariant violation in 10,000
default samples, and reached all six directed witnesses. The five default random
zero-hit warnings remain separate. Total wall time was 34.185 seconds; each model
command retained its own 30-second cap. See the
[final entrypoint capture](audits/2026-10-16/doctorwatson/quint/design-03.json).

The predecessor [design-04 capture](audits/2026-10-16/doctorwatson/quint/design-04.json)
includes the subsequent fixes and all five model commands, with unchanged source
hashes and exit 0 in 35.243 seconds. It passed 33 Python tests, 30 main Quint
scenarios and the directed-completion test. Default sampling reported no violation
in 26.788 seconds, with seven zero-hit witnesses; directed sampling reached its
six witnesses in 64 ms. The [kit review](audits/2026-10-16/doctorwatson/QUINT-REVIEW.md)
lists the current warnings and model abstractions. The command caps remain tight
on slower hosts; timeouts stay failures, with no bounds or sample-count fallback.

The [delivery follow-up](audits/2026-10-16/doctorwatson-delivery/README.md) records
the actual `--host` build and execution: 79 Python tests, seven compiled normal
Rust tests and 216 complete prefix comparisons passed. The final host command
exited 0 in 3.611 seconds with unchanged consumed source bytes. Its predecessor
failed compilation specifically because `run_steps` was absent; all dependent
stages remained unrun. The final `--design` command passed 79 Python tests,
34 model scenarios, its directed-completion test and both simulations in
31.733 seconds. Eight random witnesses had zero hits; those warnings remain
separate from positive deterministic/directed observations. All per-command
timeouts and model bounds were preserved.


The additional flat-ID host realization uses the same inspected wrapper with a
separate fresh attempt directory:

```bash
python3 -I -B models/sherlock/qa/run_host.py \
  --rustc /absolute/path/to/1.99.0/bin/rustc \
  --output-dir /absolute/path/to/new-image-attempt \
  --realization image
```

It compiles and executes the existing builder-produced image interpreter on the
same eight normal inputs and compares all 216 complete observations. Its test
stages are explicitly `not-selected`; they are not reported as passing tests.
Actual row tables, expression/frame origins and runtime Debug values remain
separate from the common structural projection. This is an A-profile host
proposal, not B adoption, canonical image admission or a circuit/native result.
The command requires the current scoped preflight; source preparation is not an
execution receipt. Default `--host` retains its own fourteen-test realization.


## Current EARS evidence report

The ordinary DoctorWatson checker call explicitly selects
`openspec/changes/sherlock-completion-convergence/current-evidence.json` with
`--requirement-evidence`. It reports all 35 original UIREQ rows (37 named
scenarios), the separate 25 refinement rows (68 named scenarios), and the
declared retained-family inventory. These are distinct inventories, not one
completion denominator. Each row retains its owner-declared implementation,
validation, acceptance, scope, review/evidence references and next concrete check.

The checker validates declared membership, named scenario completeness, file
hashes and reference line spans. Computed currentness is separate from semantic
assessment: matching files do not authenticate a reviewer or close a requirement.
Original and refinement summaries stay separate. Pending Root semantic adoption
cannot enter an adopted-closed subtotal; proposals remain visible as proposals.
A changed source/evidence/review reference preserves its historical declaration
but reports stale/not-current and cannot count as currently closed. Undeclared
dependency closure remains unestablished. Required evidence classes are declared
per original clause; this report adds no blanket native prerequisite.

A complete current report may contain open, partial or unrun obligations and
return exit 0. A stale/missing/invalid selected row returns mismatch exit 1 with
row diagnostics; malformed top-level input returns input-error exit 2. This
validates the declared report, not the requirements themselves. The existing
shell fail-stop propagates those failures. A bare `check.py` invocation without
the selector retains the existing corpus/source-only behavior and reads no
requirement sidecar, preserving the fixed small host/image/stream snapshots.
