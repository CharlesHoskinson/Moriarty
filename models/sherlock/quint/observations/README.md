# Supplied Quint K-A observations

This is a source-prepared observation driver and ordinary Python artifact adapter
for `sherlock-k-a/1`. New Quint typechecking, driver execution and corpus comparison
are **not run** here. The historical 98 and 126 model tests remain evidence only
for their recorded sources. Eleven normal Python checks exercise serializer
fixtures and source selection; their fixture-shaped traces are not model output.

`normalObservationTest.qnt` selectively imports `n01Program` through `n08Program`
from the existing independent test modules and calls the unchanged model's
`initProgram` and `step`. It references no expected state, expected outcome or
transition helper. Its 216 named tests select the committed ordinary budgets in
N01–N08 order and perform exactly `fuel + 1` actual model steps per run. That is
enough to capture the free exhaustion/return publication on the normal corpus.
The surplus-fuel run also captures a terminal stutter. Passing this driver alone
would show execution, not agreement with the independent corpus.

The full local source closure is:

- `observations/normalObservationTest.qnt`;
- `kProfileMachine.qnt` (no imports);
- `kProfileMachineTest.qnt`, importing `kProfileMachine.qnt`;
- `kProfileExtraNormalTest.qnt`, importing `kProfileMachine.qnt`.

The imported test modules contain expected literals, but the observation driver
imports only their program declarations. Expected values never construct emitted
states. No original model, test, oracle, shared checker or execution script changes.

After separate integrator preflight, the proposed TypeScript-only commands from
the repository root are shown below. These are source preparation, not permission
to execute. The integrator owns actual bounded command execution and its receipt.
There is no native/Rust backend or `verify` request.

```bash
quint typecheck models/sherlock/quint/observations/normalObservationTest.qnt
quint test models/sherlock/quint/observations/normalObservationTest.qnt --main normalObservationTest --match '^n0[1-8]Fuel[0-9]+ObservationTest$' --backend typescript --max-samples 1 --seed 1 --out-itf '<capture-directory>/{test}.itf.json'
```

Copy `normal-observation-manifest.json` next to the actual captures. Each ITF name
is relative to that manifest. The manifest fixes selection and declares the pinned
K source identity; it neither contains observations nor authenticates their origin.
Then the ordinary supplied-artifact adapter can run independently:

```bash
python3 -I -B models/sherlock/qa/quint_observations.py '<capture-directory>/normal-observation-manifest.json' > '<capture-directory>/quint-observations.json'
```

The adapter imports Python's standard library only. Its public functions are
`decode_itf(value)`, `project_snapshot(decoded_snapshot)`,
`adapt_itf_trace(case_id, itf_document)` and
`observation_bundle([(case_id, itf_document), ...], source_sha256=...)`.
`canonical_bytes(value)` produces deterministic JSON for artifact digesting.
It does not evaluate expressions, bind parameters, match patterns, select rules,
compute primitive results, replay states or open the independent oracle files.
It renders supplied syntax, values, frames and helper results.

The top-level JSON has `format: sherlock-quint-itf-observations/1` and:

| Member | Meaning |
| --- | --- |
| `observations` | Common HOST-OBSERVATIONS shape: producer, profile, declared K source hash, boundary, ordered program identities and complete runs |
| `raw_traces` | Ordered `{case_id,itf}` documents, retaining all parsed original ITF fields, metadata, map order, expression IDs and states |
| `raw_trace_references` | Ordered case IDs and `canonical_itf_sha256`; CLI also adds original file `path` and `file_sha256` |
| `projection` | Explicit profile, boundary, startup/terminal interpretation, per-run stutter counts and projection gaps |
| `evidence` | `execution_receipt: not-supplied`, `producer_authenticity: not-established`, `adapter_execution: supplied-artifact-only` |

`observations` follows `qa/HOST-OBSERVATIONS.md`, with declared producer
`sherlock-quint-k-profile`. A comparator must expect that identity explicitly;
Quint output must never be labelled host execution. Program/code, entry, control,
environment, all top-first frames and their saved environments, stack limit,
initial/remaining fuel, raw outcome and work appear in every projected snapshot.
The incoming raw `lastRule` and `lastCharge` determine each record. The adapter
retains actual free terminal publication and refuses a truncated pending trace.
The shared checker owns complete 8-program/216-run membership comparison; the
adapter also supports a deliberately smaller supplied trace for diagnosis.

The installed Quint 0.32 encoder at `dist/src/itf.js` uses `{tag,value}` variants,
`{"#tup":[]}` unit payloads, `{"#bigint":"..."}` integers and `{"#map":[...]}`
maps. State variable names may be module-qualified; the adapter requires one
unambiguous `machine`, `lastRule` and `lastCharge` at each state.

The raw/projection differences are deliberate and visible:

- `lastRule=74,lastCharge=0` marks the initial post-startup state; this does not
  execute or observe a separate K constructor-startup edge.
- `Snapshot` has no independent depth or work cell. The adapter derives depth
  from raw frame-list length and work from raw `initial - remaining`; it checks
  each raw charge against that difference. It does not measure helper work.
- K syntax expands ExprIds. Equal syntax sharing and unused syntax-table rows
  remain in raw ITF. Calls retain names; the renderer never unfolds recursion.
- Map rendering uses canonical decimal names sorted as strings, matching the
  host diagnostic renderer; raw ITF map encoding and order remain available.
- `step` stays enabled after publication: `advance` emits unchanged state with
  rule 0 and charge 0. Those actual terminal stutters are retained raw and
  counted per run. They are excluded from K-A semantic records. The shared
  `terminal_has_next=false` means no further K-A semantic edge from a published
  outcome, **not** an observed probe that the Quint action is disabled.
- Raw `Pending`, `Done`, `Rejected` and `Exhausted` outcomes retain their K
  spellings. Free publication is preserved separately from charged rules.

Diagnostic K text is not a canonical codec, state admission API or authenticated
receipt. Parsing, hashing and Python fixture checks establish no model execution,
refinement, exhaustive numeric/grammar coverage, B/helper metering, Lean/Clean,
circuit, financial, theorem, native or Midnight target result. All retained
roadmap exits and native limits remain unchanged.
