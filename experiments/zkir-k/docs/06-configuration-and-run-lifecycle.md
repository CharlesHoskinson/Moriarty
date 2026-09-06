# Configuration and run lifecycle

## Configuration

The virtual machine (VM) keeps witness computation, gate emission and verdict evaluation in one configuration. The descriptions below follow [`zkir-vm.k`](../semantics/zkir-vm.k), module `ZKIR-VM`; VM symbols belong to that file unless another file is named.

The table gives content sorts, except for the enclosing cell, whose generated sort is `ZkirCell`. The initial content is what each cell holds before the entry point runs. Element types in the role column describe intended contents, not additional K sorts.

| Cell | Sort | Initial content | Role |
|---|---|---|---|
| `<zkir>` | `ZkirCell` | All cells below | Encloses the run. |
| `<k>` | `K` | `$PGM:Job` | Pending computation. |
| `<mem>` | `Map` | `.Map` | Register names (`String`) mapped to `Value`. |
| `<pi>` | `List` | `.List` | Native field elements forming the public-input vector. |
| `<skips>` | `List` | `.List` | `Skip` entries: `skipNone()` for active impacts, `skipSome(n)` for inactive impacts. |
| `<pubInIdx>` | `Int` | `0` | Elements consumed from public transcript inputs. |
| `<pubOutIdx>` | `Int` | `0` | Elements consumed from public transcript outputs. |
| `<privIdx>` | `Int` | `0` | Elements consumed from the private transcript. |
| `<outputs>` | `List` | `.List` | Returned `Value` entries, appended in order. |
| `<outTypes>` | `IrTypes` | `.IrTypes` | Program output signature. |
| `<doComm>` | `Bool` | `false` | Whether the program requires a communications commitment. |
| `<pre>` | `Preimage` | `preimage(.List, 0, noComm(), .List, .List, .List)` | Original inputs, binding input, optional commitment and transcripts. |
| `<constraints>` | `List` | `.List` | Emitted `Constraint` entries, called gates. |
| `<chips>` | `Set` | `.Set` | Chip names selected by `zkir-constraints.k`, `usedChips`. |
| `<verdicts>` | `List` | `.List` | `Verdict` entries produced after execution. |
| `<witnessSpace>` | `Bool` | `false` | Whether the final memory is in the modelled witness space (`witnessSpace`, `zkir-constraints.k`). |
| `<unconstrainedRegs>` | `List` | `.List` | Registers whose assigning relation is `unconstrained`. |
| `<observable>` | `Observable` | `noObs()` | The observable result `obs(status, encoded outputs, public inputs)`, written by `#observable` after `#witnessSpace` (16-compilation-target-contract.md). |
| `<piIdx>` | `Int` | `0` | Next absolute public-input position for gate emission. |
| `<genMode>` | `Bool` | `false` | Enables transcript generation behavior. |
| `<needs>` | `List` | `.List` | Recorded `Need` entries for the harness. |
| `<strictDecode>` | `Bool` | `false` | Selects canonical decoding checks. |
| `<status>` | `Status` | `ok()` | Witness status: `ok()`, `error(String)` or `panic(String)`. |

`<k>` starts with a job, expands into initialization and instructions, and eventually contains `#finish ~> #verdicts`. Termination leaves `.K`, the empty K sequence, after successful runs and after handled errors and panics alike; emptiness says that processing ends, not that the witness succeeds.

The extension surface enables `<strictDecode>` through the `job` and `genJob` rules in `zkir-ext.k`, at priority 30, before ordinary job initialization. See [09-extension-surface.md](09-extension-surface.md) for the decoding difference.

## Entry points and initialization

`ZKIR-VM-SYNTAX` in `zkir-vm.k` declares the entry sort:

```k
  syntax Job ::= job(Program, Preimage)         [symbol(job)]
               | checkedJob(Program, Preimage)  [symbol(checkedJob)]   // wf first
               | genJob(Program, Preimage)      [symbol(genJob)]       // generation mode, see <genMode>
```

`job` executes the raw program without a static check. `checkedJob` places `#wfGate(wf(P))` before `job(P, Pre)`: `wfOk()` disappears, while `wfError(S)` sets the status to `error("well-formedness: " +String S)` and discards the following job. Input loading, gate emission and `#verdicts` then never begin; `<constraints>` and `<verdicts>` stay empty, and every other cell keeps its initial value. The static predicate `wf` belongs to `zkir-syntax.k`; see [10-well-formedness-and-static-checks.md](10-well-formedness-and-static-checks.md).

`genJob` sets `<genMode>` to `true` and becomes `job`. `tools/zkir_run.py` exposes the three constructors as the default run, `--checked` and `--gen`; `Runner.run` selects generation before checked execution when both flags are supplied, since there is no combined checked-generation constructor.

The `preimage` constructor orders its fields as `inputs`, `binding_input`, `communications_commitment`, `private_transcript`, `public_transcript_inputs` and `public_transcript_outputs`. Its `CommOpt` field is either `noComm()` or `comm(commitment, opening)`. `preimage_term` in `tools/zkir_run.py` converts the optional two-element commitment array to that constructor and checks preimage integers through `fr`. See [04-values-and-encoding.md](04-values-and-encoding.md) for field encodings.

The ordinary job rule stores the preimage, output types, commitment flag and `usedChips(P)`, then schedules:

```k
#loadInputs(#preInputs(Pre), Ins, 0) ~> #seedPi(Ins) ~> Is ~> #verdicts
```

`#loadInputs` processes declarations in order: it checks that enough raw elements remain, decodes exactly `encodedLen(T)` through `decodeStrict` in `zkir-values.k`, and schedules `#put` before the next declaration. `#put` stores a successful value by map update, overwriting any existing destination, so a later decoding failure preserves earlier writes. Finishing the declarations also checks for surplus raw inputs.

`#seedPi` places the binding input at position zero and emits `bindGate(0)`. With commitments enabled it also places the supplied commitment at position one and emits `commGate(1, Ins, Rand)`. A missing commitment places only the binding input in `<pi>`, still emits `bindGate(0)` and `commGate(1, Ins, 0)` with opening randomness zero, sets `<piIdx>` to two, and raises `Expected communications commitment`. That gate later evaluates to `unknown("public input vector incomplete")` because `<pi>` has one element. After an input-loading failure, seeding still emits the gates and sets `<piIdx>` to one or two, without populating `<pi>`.

## Sequencing and failure

The instruction-list rule exposes the next instruction through `~>`:

```k
  rule <k> (I:Instr ; Is:Instrs) => I ~> Is ... </k>
  rule <k> .Instrs => #finish ... </k>
```

An ordinary instruction appends `gate(I)` and becomes `#exec(I)`. `isSpecialEmit` selects two exceptions: `impact` emits `guardGate` and its `piGate` entries, while `output` emits `outputGate` carrying `<outTypes>`. Impact advances `<piIdx>` during emission, independently of witness progress.

Witness entry rules match `<status> ok() </status>`, a cell-pattern condition rather than a global restriction on every rule. `#fail` sets `error(S)`; `#panicNow` sets `panic(S)`. Subsequent `#exec` terms disappear under either failure status; memory, outputs, public inputs and transcript cursors keep their partial state, gate emission continues, and `#verdicts` still runs. A failed static check differs because it prevents job initialization.

The ordering follows the explicit continuation and rule conditions, not the textual order of the rules. Helpers such as `#put` need no separate status test because only the live witness path schedules them.

## Operand resolution and transcript reads

`resolve` turns `imm(I)` into `vOk(native(I))`, looks up `var(X)` in `<mem>`, or returns `vErr` for a missing variable. `resolveNatives` uses `#rn` to resolve and convert each operand before proceeding. A non-native value therefore fails before a later missing operand can replace its error.

Other continuations encode instruction-specific precedence: `#lt` through `#lt4` check the first operand before resolving the second; `#recon` through `#recon4` check the modulus operand before the divisor; `#output` appends successful values one at a time after `#exec(output(...))` checks arity. These are explicit sequencing helpers, not an instruction-level `strict` attribute. See [07-instruction-reference.md](07-instruction-reference.md) for individual operations.

`publicInput` reads **public transcript outputs**, through `#prePubOut`; `privateInput` reads `#prePrivate`. Their `#exec` rules resolve the guard through `#guardActive` before `#input` selects a branch. A false guard stores `defaultValue(T)` without consuming transcript elements. An active read consumes `encodedLen(T)` elements; when the slice exists, the cursor advances before `#put` decodes, so a decoding failure can leave an advanced cursor. A short transcript in ordinary mode instead schedules `#panicNow` without advancing the cursor.

`impact` uses public transcript inputs. An inactive impact appends zeros and `skipSome(n)` without resolving its value operands or consuming transcript inputs. An active impact appends native values through `#impactPush` and `#impactOne`, which advance `<pubInIdx>` one at a time, then appends `skipNone()` and compares the pushed values with the expected transcript in `#impactCheck`; missing expected values follow the mismatch error path.

## Completion and generation

In ordinary mode, live `#finish` requires each transcript cursor to equal its transcript length, otherwise raising `Transcripts not fully consumed`. Only then does `#commCheck` compare the supplied commitment with `transientCommit(#preInputs(Pre) #encodeAll(Os), Rand)`, where `#preInputs(Pre)` is the raw input list of the preimage, `#encodeAll(Os)` in `zkir-constraints.k` is the concatenated encoding of the output values, and juxtaposition is K list concatenation. `transientCommit` in `zkir-hash.k` hashes the opening followed by that list.

`#verdicts` runs after `#finish` and invokes `verdicts` in `zkir-constraints.k` with the final memory, public inputs, chips, outputs and binding input. It does not change `<status>`. The commitment gate re-encodes input registers, whereas the witness commitment check uses raw inputs. See [08-constraints-and-verdicts.md](08-constraints-and-verdicts.md) for outcomes and [13-known-divergences.md](13-known-divergences.md) for consequences of this distinction.

Under `error` or `panic`, `#finish` disappears, preserving the first failure instead of replacing it with an exhaustion or commitment error. Gates retain operands and register names, not snapshots of intermediate memory, so raw programs that overwrite registers can make earlier gates observe later values; the single-assignment check of `wf` in `zkir-syntax.k` matters when interpreting these verdicts.

Generation changes transcript handling through `<genMode>`. A short active read supplies `defaultValue(T)`, advances the cursor and records `needPubOut(T)` or `needPriv(T)`, and an active impact records `needPubIn(value)` instead of comparing expectations. Live `#finish` skips exhaustion checks and, when commitments are enabled, records `needComm(value)`. Generation still requires a commitment pair at seeding and can fail on other witness operations.

`build_preimage` in `tools/diff_test.py` supplies a provisional commitment pair when needed, runs generation, fills the requested transcript values, and replaces expected public transcript inputs and the commitment from the recorded needs. It repeats within a bounded loop because generated values can change later guards. `need` in `tools/zkir_run.py` exposes these records as `pubOut`, `priv`, `pubIn` and `comm`. Since live `#finish` skips the exhaustion and commitment checks, a `--gen` run can report `ok`, even `holds()` on every gate, for a preimage an ordinary run rejects. Apply the `needs` list and rerun without `--gen` before reading the result as an ordinary verdict.

## Runner statuses

`Runner.run` in `tools/zkir_run.py` checks the returned `<k>` before interpreting `<status>`.

| Reported status | Condition |
|---|---|
| `ok` | Empty `<k>` and `ok()` status. |
| `error` | Empty `<k>` and `error(S)` status. |
| `panic` | Empty `<k>` and `panic(S)` status. |
| `stuck` | Nonempty `<k>` and no `--depth` given. |
| `depth-exhausted` | Nonempty `<k>` and `--depth N` given. |

A stuck configuration retains a computation, such as a `#exec` or helper application, possibly alongside `<status> ok()`. No result follows from that alone, so the runner treats any residual computation as failure and reports the full term in `k_cell` and its first 300 characters in `error`. `depth-exhausted` records that a bound was given, not that it was reached: the runner does not distinguish bound exhaustion from a run that is stuck. The traced program below with `--depth 3` yields the residual

```
#loadInputs ( ListItem ( 2 ) , .TypedIds , 1 ) ~> #seedPi ( typedId ( "%x" , native ( ) ) , .TypedIds ) ~> add ( var ( "%x" ) , imm ( 3 ) , "%y" ) ; impact ( imm ( 1 ) , var ( "%y" ) , .Operands ) ; output ( var ( "%y" ) , .Operands ) ; .Instrs ~> #verdicts ~> .K
```

with `%x` already in `memory` and `constraints` 0: the `<k>` cell after `#put` and before the loader finishes. A failing `krun` process separately produces `krun-failed`.

These statuses appear in the returned JavaScript Object Notation (JSON) object. `main` prints it and returns zero for every reported status, so callers inspect `status` and the verdicts rather than the exit code. Two failures precede any run and exit with code 2 and no JSON: a program the loader rejects prints `format error: ...`, and a preimage integer that is not a canonical field element, caught by `fr`, prints `preimage error: ...`. See [11-tooling-reference.md](11-tooling-reference.md).

## Three-instruction trace

Take a base-surface program with native input `%x`, one native output and communications commitment disabled. Its instructions, in `zkir-syntax.k` constructor notation, are:

```k
add(var("%x"), imm(3), "%y") ;
impact(imm(1), (var("%y"), .Operands)) ;
output((var("%y"), .Operands)) ; .Instrs
```

The same program as a JSON artifact, saved as `trace.zkir`:

```json
{"version":{"major":3,"minor":0},
 "inputs":[{"name":"%x","type":"Scalar<BLS12-381>"}],
 "outputs":["Scalar<BLS12-381>"],
 "do_communications_commitment":false,
 "instructions":[{"op":"add","a":"%x","b":"0x03","output":"%y"},
                 {"op":"impact","guard":"0x01","inputs":["%y"]},
                 {"op":"output","vals":["%y"]}]}
```

Use this preimage, saved as `trace-pre.json`; `tools/zkir_run.py`, `preimage_term`, supplies empty omitted transcripts and `noComm()`:

```json
{"inputs":["2"],"binding_input":"9","public_transcript_inputs":["5"]}
```

From the repository root:

```sh
uv run --group zkir-k python experiments/zkir-k/tools/zkir_run.py trace.zkir trace-pre.json
```

The milestones below group helper rewrites, not individual backend steps. Brackets abbreviate K lists, braces abbreviate maps. `<k>` shows the next computation; its continuation holds the remaining instructions and then `#verdicts`. Gate entries are appended to the existing list.

| After | Next in `<k>` | `<mem>` | `<pi>` | `<skips>` | Cursors: public input, public output, private | `<piIdx>` | `<outputs>` | Gates appended |
|---|---|---|---|---|---|---:|---|---|
| Input loading | `#seedPi` | `{%x: native(2)}` | `[]` | `[]` | `(0,0,0)` | 0 | `[]` | None |
| Seeding | `add` | `{%x: native(2)}` | `[9]` | `[]` | `(0,0,0)` | 1 | `[]` | `bindGate(0)` |
| `add` witness | `impact` | `{%x: native(2), %y: native(5)}` | `[9]` | `[]` | `(0,0,0)` | 1 | `[]` | `gate(add(var("%x"), imm(3), "%y"))` |
| `impact` witness | `output` | `{%x: native(2), %y: native(5)}` | `[9,5]` | `[skipNone()]` | `(1,0,0)` | 2 | `[]` | `guardGate(imm(1))`, `piGate(1, imm(1), var("%y"))` |
| `output` witness | `.Instrs`, then `#finish` | `{%x: native(2), %y: native(5)}` | `[9,5]` | `[skipNone()]` | `(1,0,0)` | 2 | `[native(5)]` | `outputGate((var("%y"), .Operands), (native(), .IrTypes))` |
| `#finish` | `#verdicts` | `{%x: native(2), %y: native(5)}` | `[9,5]` | `[skipNone()]` | `(1,0,0)` | 2 | `[native(5)]` | None |
| `#verdicts` | `.K` | `{%x: native(2), %y: native(5)}` | `[9,5]` | `[skipNone()]` | `(1,0,0)` | 2 | `[native(5)]` | None |

Throughout, `<status>` is `ok()`, `<chips>` is `.Set`, `<outTypes>` is `(native(), .IrTypes)`, and `<pre>` holds the supplied preimage. `<doComm>`, `<genMode>` and `<strictDecode>` remain false; `<needs>` remains empty. `<verdicts>` stays empty until the final step, when each of the five gates receives `holds()`.

`#exec(add(...))` schedules `#put` through `#bin`; `zkir-ops.k`, `addV`, computes native addition. The impact reserves position one at emission, then pushes five and compares it with the supplied transcript entry. Output appends `native(5)`. Transcript lengths match at `#finish`, leaving verdict evaluation as the last computation.

The runner's JSON confirms the last row: `status` `ok`, `memory` with `%x` encoded `["2"]` and `%y` encoded `["5"]`, `pis` `["9", "5"]`, `pi_skips` `[null]`, `cursors` `[1, 0, 0]`, `k_cell` `.K`, `outputs` `["Native:5"]`, and five `holds` entries in `all_verdicts`. Gates are pretty-printed with list cons cells flattened, so the output gate appears as `outputGate ( var ( "%y" ) , .Operands , native ( ) , .IrTypes )`.
