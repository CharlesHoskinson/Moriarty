# A5 compiler phase implementation plan: independent source review

Spec verdict: REQUEST CHANGES.
Quality verdict: REQUEST CHANGES.
One bounded sequencing defect prevents adoption of the reviewed bytes. The
remaining reviewed design and interface obligations have no additional blocking
finding. This is plan review, not Council, executable validation, or dispatch.

Reviewed plan:
`docs/superpowers/plans/2026-09-06-candidate-a-compiler-phase-diagnostic.md`

Reviewed SHA256:
`6a861022ed8641e2674f36f5725636c79550fffe3550928012694331cc0d76e4`

Adopted design:
`docs/superpowers/specs/2026-09-06-candidate-a-compiler-phase-diagnostic-design.md`

Design SHA256:
`19ff70c32004d824ee1b5e34e722425a354412d3548f321a943b6e32fc9ca67a`

## Finding CP-R1: raw/content failures are checked after later native launches

Priority: P2, blocking this plan's stated stop-on-failure contract.

Locators in the reviewed plan: `record.py` block lines 652–699; CP003 line 933.
The latter states, "A failure stops the sequence; no repeat is supplied."

The child loop performs all four native invocations before reaching the raw
stdout/stderr comparisons at lines 689–692. For example, if success-observed
has the expected exit and valid phase trace but different generated JSON,
error-direct and error-observed still launch before the known pair mismatch
is checked. Successful JSON shape/declarations/empty stderr and the genuine
missing-alias baseline text are also checked only after all four runs, at
lines 693–699. A malformed success-direct result with exit zero can therefore
launch the remaining three children before its content predicate fails.

Required correction: place each direct baseline's content predicates after
that child's terminal and before the next native launch. Place the pair's
raw stdout/stderr equality checks immediately after its observed child and
before proceeding to the next pair. Preserve the exact four-child order,
limits, inputs, final predicates and existing original failure receipts.
Failure must escape the loop into its current terminal/finalization path;
do not weaken the predicates or add retries. Re-review the changed plan bytes
before materialization.

## Other reviewed obligations

- The six CommonJS bindings are real writable exports. Installed `cli.js`
  lines 59–60 and 119–124 construct the chain from that same exports object;
  installed `cliCommands.js` lines 47–56 export the functions. The pinned
  Either implementation's `asyncChain` returns a resolved Left without calling
  the next callback. The observer therefore intercepts the actual callbacks
  and does not force Left-skipped work.
- `install` supplies non-async functions, calls originals with `Reflect.apply`,
  and returns the original result or Promise. Its observing fulfillment and
  rejection handlers return undefined, preserving original settlement values
  without forwarding a rejection into the unused observer Promise. The mock
  source checks receiver, argument/mutable-object identity, call count,
  synchronous result/throw identity, Promise identity and both CLI chain shapes.
  These checks are specified, not executed results.
- Result classification uses the pinned Either prototype and its own `type`
  data property. Open/write/partial-write/classification/record/byte failures
  call the bounded exit-74 path. The production observer has no signal, exit,
  global exception or rejection handler and does not replace process.exit.
  The installed outputResult exits zero or one; its entry without return is
  handled explicitly. Trace interpretation retains I/O and scheduling margins
  and cannot locate the historical timeout or an internal compiler hot spot.
- `launch.cjs` receives exactly one invocation JSON path. The basename-derived
  child folder now matches `execute`'s sibling invocation and exclusive child
  directory. Recorded real argv, restored effective argv, main, invariant,
  JSON target and verbosity agree with the three historical input receipts
  inspected directly. The full command preserves default init and step.
- Preparation's exact four-block extraction, six-entry initial directory
  inventory and transport's initial command write agree. The source freeze
  retains the 24 view copies, two import additions, 30 originals, four alias
  control inputs, reviewed source references and runtime metadata. Actual
  preparation/full dispatch heads remain distinct from the historical view
  dispatch and runtime bootstrap. No helper globals are reassigned.
- The A4 recorder provides the actual `pin`, exclusive `write`, duplicate-key
  rejecting `read`, and source-only `archive` interfaces used by this plan.
  The resource runner provides the actual `cleanup_group`, `group_exists`, and
  bounded `parse_resource` interfaces used here. The source hashes rechecked
  in this review match the plan's immutable utility pins. No helper was imported.
- Native process ownership uses a new session/group. The reviewed post-Popen
  paths run cleanup before artifact finalization; uncertain ownership blocks
  artifact hashes and requires root takeover. Actual child code, timeout,
  monitor/signal issues and recorder preservation status remain distinct.
  The full timeout path permits preservation success without compiler success;
  exit 74 invalidates phase location. Runtime/source drift blocks final intake.
- Incremental original tool responses stay in `transport/` outside finalized
  child stages. The next response is requested only after preserving the
  previous one. Missing terminal/session/transport-save outcomes require
  takeover; the plan supplies no fabricated terminal or retry. Intake excludes
  its own growing transport and requires preserving that terminal separately.
- Root implementation review, mock adoption, alias adoption and separate full
  dispatch are distinct gates. The A4 gate uses its existing terminal fields
  and `args`/`responses` outer receipt interface and requires terminal, clean
  owned groups. It does not falsely require or infer A4 semantic success.
  The full slot is exclusive, once only, at 900 seconds and 4096-MiB Node/JVM
  settings. It authorizes no factored compile, solver, resource increase or H1
  admission. Root still must inspect all original records at each stated gate.

## Review evidence and limits

Read the entire 1046-line frozen plan, the adopted design, relevant installed
CLI/Either source, immutable helper definitions and the three historical
compile input records. Rechecked the plan/design, design-adoption, two utility
and three historical-input small-file hashes. Those checked bindings agree
with the plan. This review did not hash runtime archives or other large
artifacts, materialize executable blocks, import helpers, execute native/mock/
test code, modify the reviewed plan, or change frozen inputs/runtime/receipts.

Only this review file was written. Its verdict applies to the exact reviewed
SHA above; a corrected plan needs a new review record and root adoption.
