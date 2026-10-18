# Proposed production API and public caller trace

UNADOPTED interface specification, not compiled Rust or existing callable API. All types below are normal-path contracts. `SequenceError` and all new admission/refusal/reopen rules are reserved to the designated owner; no substitute variants or error bytes are authored here.

## Library

Recommended location: `mori_core::stage::sequence`, exposed through a proposed non-default `core2-sequential-proposed` feature and forwarded by `mori`. Existing feature names and defaults are untouched. Public developer use has no project-registration or reviewer-record argument.

```rust
pub struct ProgramEntry {
    pub key: String,                 // manifest-local routing label, not identity evidence
    pub program: stage::StagedProgram,
}
pub struct FrameBindings {
    pub states: BTreeMap<String, String>, // local source state -> global Snapshot.state key
    pub duties: BTreeMap<String, String>, // local source duty name -> global liability name
}
pub struct Invocation {
    pub program: String,            // ProgramEntry.key
    pub action: String,
    pub inputs: BTreeMap<String, Value>,
    pub bindings: FrameBindings,
    pub logical_cap: Option<UInt>,  // mandatory JSON key; null retains existing no-local-cap meaning
}
pub struct LogicalAllocation {
    pub ordinary: UInt,
    pub closure_reserve: UInt,
}
pub struct Sequence { /* owns catalog, plan, complete head, outside frame,
                        original pending continuation and every observed prefix */ }

impl Sequence {
    pub fn plan(catalog: Vec<ProgramEntry>, plan: Vec<Invocation>,
                opening: Snapshot, allocation: LogicalAllocation)
        -> Result<Self, SequenceError>;
    pub fn begin(&mut self) -> Result<ObservationIndex, SequenceError>;
    pub fn resume(&mut self, event: stage::StageEvent)
        -> Result<ObservationIndex, SequenceError>;
    pub fn next(&mut self) -> Result<ObservationIndex, SequenceError>;
    pub fn report(&self) -> SequenceRunReport;
}
```

`ObservationIndex` is a local unsigned array index, not a parent/head identity or replay token. `Sequence` has no public constructor from an arbitrary Complete/Execution/Snapshot after planning, and no proposed Deserialize in this slice. This is an in-process ordinary API choice, not a claim of uniqueness, authenticated origin or secret isolation. A separately specified persistent API must preserve the same data and supply its owner-defined validation.

`plan` stores an explicitly provided complete opening head and yields observation0 with no executed call, no manufactured frames, empty effect/call prefixes and zero local consumed work. The sequence plan is a nonempty finite list; numerical invocation/plan/output bounds and their errors must be fixed through the owner-dependent resource contract before implementation. This proposal neither raises existing reader/export bounds nor invents a new supported maximum.

`begin` executes invocation0 through actual `stage::start`. `resume` executes only the active action through actual `stage::resume` and its original continuation. `next` selects the next planned invocation after the current one is actually complete, projecting the owned whole head and calling real `stage::start`. There is no `scenario` or `predecessor` parameter on `next`, no caller-supplied prior accounting, and no auto-start of later programs. The ordinary successful transition ordering and state shape are SC1–SC3 recommendations; concrete misuse/admission/refusal handling belongs to the designated owner.

After a waiting call, progress is `waiting`; after completion with planned successors, it is `between-programs`; after final local completion it is `complete`. A positive residual set does not change this local control status into an incomplete scalar action or into settlement. The `programs_completed` count is separate from paid/delivered/settled external obligations, for which this profile supplies no evidence.

## Production manifest and command

Proposed command: `mori compose PLAN.json --json`. It is additive to the existing CLI and deliberately names the language operation. `--json` prints one full normal-path run report with one final LF. A successfully executed finite list of requested operations returns exit0 whether the resulting local progress is waiting, between programs or complete; actual report status exposes that distinction. New command errors and their publication/exit contract require the designated owner. No compiler/native/prover operation is selected.

The exact root input fields are `profile`, `programs`, `opening`, `invocations`, `allocation`, `operations`; all are mandatory. There are no expected-output fields. Existing bounded public file/strict JSON readers are reused without enlarged caps. All paths are relative to the manifest directory. Input paths are read/frozen before driving the requested calls; expected files can never affect production state.

| Object | Exact fields |
| --- | --- |
| Root | `profile:"moriarty-composition/sequential-proposed/1"`, `programs:ProgramPath[]`, `opening:path`, `invocations:InvocationPath[]`, `allocation:LogicalAllocation`, `operations:Operation[]` |
| ProgramPath | `key:string`, `source:path` |
| InvocationPath | `program:string`, `action:string`, `inputs:path`, `state_bindings:{local:global}`, `duty_bindings:{local:global}`, `logical_cap:UInt|null` |
| LogicalAllocation | `ordinary:UInt`, `closure_reserve:UInt` |
| Operation begin | `kind:"begin"` |
| Operation next | `kind:"next"` |
| Operation resume | `kind:"resume"`, `event:path` |

Opening is the complete existing `Snapshot` DTO, with global state/duty keys. Each inputs file is exactly a map of existing typed Values for that invocation's fresh action parameters. Each event file is the current complete `StageEvent` DTO. Programs load through the actual `stage::load`, yielding the original Program/filled StagePlan and existing owner-defined program/plan digests. No new sequence digest, authenticated head ID or source-admission identity rule is specified here.

The bindings name every declared local state and every source duty name relevant to its declared local view; they do not map assets/accounts, inject values, supply missing history or rename the original nested reports. Ordinary projection equations are in DESIGN.md. Exact admissibility of aliases, registry identities and input/state schemas, including the existing R7 distinctions, is explicitly an owner handoff. The positive examples use straightforward one-to-one maps and currently supported scalar statements.

## One real call trace

1. CLI parser selects the new `Compose` arm in `commands::run`. `compose::load_plan` reads the public manifest/opening/inputs/events using `core_io`, then calls actual `stage::load` for each source. It constructs `ProgramEntry`, `Invocation` and the complete opening once. A test adapter may call this same loader; neither adapter performs financial execution.
2. Production `Sequence::plan` builds the owned session and opening observation. It stores real admitted Programs/Plans and the declared bindings. Design/admission preparation does not pretend any program ran.
3. The driver calls `begin`. Sequence projects G0 into P's local Scenario, retaining F0, and invokes existing `stage::start(P, actionP, actual_projection, capP)`. The existing runtime supplies original Steps/effects/frames/work. Sequence stores that full outcome and the lifted whole snapshots. If P waits, the driver must supply declared resume operations; Q does not start.
4. After P completes, Sequence retains its actual whole head GP and every earlier P segment. A report exposes this boundary before Q starts. The driver calls `next` with **no Q state parameter**. Sequence projects GP under Q's own bindings and calls existing `stage::start(Q, actionQ, inputsQ + actual_projection, capQ)`. A source initializer in Q cannot overwrite GP.
5. Q's success updates the selected projection, preserves every outside row, appends actual original outcomes/effects, and carries gross debit/fees/residual/work prefixes forward. Any Q awaits use Q's exact returned continuation. The production report retains both original programs' records.
6. `compose::run` renders the typed full `SequenceRunReport`. No expected output exists on this path. The whole semantic JSON and exact public bytes require independently frozen expectations before a test.

No host or test creates a Q Snapshot, concatenates two arbitrary executions, synthesizes a continuation, resets global counters or substitutes a final-state summary. Projection/accumulation are actual production behavior under review.

## Existing test consumer integration

Proposed `moriarty-tests/sequential-proposed/1` root: exactly `{profile,cases}`. Each case is exactly `{name,plan,expected_observations,expected_report}`. `plan` is a path to the same production manifest. `expected_observations` is an ordered list of paths, one complete expanded PrefixObservation for opening and each requested operation. `expected_report` names the whole independently expected run report. The suite preloads all expected values before production calls. It executes the same plan driver and compares complete values; it does not derive runtime state from expectations or perform its own projection/accounting.

The adapter uses the existing aggregate reporting behavior: every actual observation and each comparison flag remains visible; an ordinary expectation mismatch yields overall failed/exit1 while later otherwise valid requested operations and cases are still observed. It keeps the production run tables intact and names complete prefixes, never truncations. The separate public binary regression runs actual `mori compose`, compares full report values/bytes, and separately exercises `mori test` for complete comparison/aggregate behavior. Proposed names and required goldens are in EXAMPLES-AND-TESTS.md; no command/test is callable at the pinned source yet.

## Exact production file boundary for later integration

New: `crates/mori-core/src/stage/sequence.rs`, `crates/mori/src/commands/compose.rs`, `crates/mori/src/commands/test/sequence_test.rs`. Shared edits: `stage/mod.rs`, `crates/mori-core/Cargo.toml`, `crates/mori/Cargo.toml`, `crates/mori/src/cli.rs`, `commands/mod.rs`, `commands/test.rs`. Public tests/fixtures are additive. Existing parser, scalar evaluator, stage DTOs/digests/admission, Pair source and old goldens stay intact. If owner decisions require changes to those existing rules, their complete changed source and affected correspondence become a separately reviewed extension, not an invisible prerequisite repair.
