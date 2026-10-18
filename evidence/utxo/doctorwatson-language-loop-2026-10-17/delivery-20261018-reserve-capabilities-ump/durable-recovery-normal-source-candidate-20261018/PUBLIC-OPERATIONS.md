# Real public inspect/simulate/test operations required in the first delivery

Commands below are **proposed public operations**, not commands that were run or current executable capabilities. Their identities remain documentary markers until owner decisions and a new independent freeze. A new inspection/report profile is selected only with the explicit recovery-module flag; every old command without that flag preserves current behavior/bytes.

## Inspectable source and plan

```bash
mori inspect base.mori --recovery-module recovery.mori
```

Output must equal the complete paired Program/Plan view in `program.inspect.example.json` for the case, including base source profile/semantics, full old Program and filled plan, handler name/kind/site/captures/binders, separate expression arena/raw statements and successful_path_ceiling. Recovery inspect cannot invoke target compilation. A target/signed/native flag combination is outside this local source route and must not silently ignore the companion. Exact argument diagnostics/order need owner review.

## Start, publish, terminate, fresh reopen and recover

For custody-refund, from its example directory:

```bash
mori simulate base.mori --recovery-module recovery.mori \
  --action fund --scenario start.scenario.json --logical-cap 13 \
  --checkpoint-out scratch/pending.json

mori simulate base.mori --recovery-module recovery.mori \
  --reopen scratch/pending.json

mori simulate base.mori --recovery-module recovery.mori \
  --recover scratch/pending.json --recovery-event recover.event.json \
  --checkpoint-out scratch/closed.json

mori simulate base.mori --recovery-module recovery.mori \
  --reopen scratch/closed.json
```

These are separate OS process invocations. The second/third/fourth commands have no access to the first command's runtime object. They must read the published file through the actual checkpoint decoder and retained-source binding path, not rebuild it from expected reports or caller-provided scenarios. Recovery uses the actual captured principal60 from the pending file. Complete expected documentary files are `start.report`, `reopen.report`, `recover-direct.report`, `reopen-closed.report` and their checkpoint counterparts, all suffixed `.example.json`.

The compensation journey is identical with `--action purchase` at start and its own files. The recovery event supplies proposed amount12 plus outcome_known:true; it does not change captured remedy20. Returned terminal duty is8, not silently reset to20 or erased. Supplier retains48 (opening8+payment40) and prior fee3 stays at treasury5. The durable closed file records that forward control is no longer pending in the returned local state.

All new paired state-changing CLI invocations require an explicit new `--checkpoint-out`. The chosen normal-path proposal writes a complete new checkpoint to a new output path and returns the complete report, rather than leaving required durable output optional. Existing pending files are preserved, not overwritten or called globally revoked. The exact publication algorithm, overwrite/error policy, crash durability, destination restrictions and commit-versus-report failure outcome are owner-dependent required decisions; normal examples assume successful publication and do not provide an IO failure oracle.

On the admitted normal path, proposed output is one complete deterministic ASCII pretty JSON report plus final newline, exit0 and empty ordinary stderr. This is a new consumer contract requiring separate byte freeze and actual observation, not inherited evidence from old `/2` outputs. Existing exporter bounds remain. Serialization/write failure cannot be reported as a successful persisted recovery, nor as evidence that financial work did not occur. Its exact known/unknown state disposition and diagnostic semantics must be selected by the owner before implementation.

## Normal false observation across another process boundary

```bash
mori simulate base.mori --recovery-module recovery.mori \
  --resume scratch/pending.json --event normal-false.event.json \
  --checkpoint-out scratch/observed.json

mori simulate base.mori --recovery-module recovery.mori \
  --recover scratch/observed.json --recovery-event recover.event.json \
  --checkpoint-out scratch/closed-after-observation.json
```

The first call uses unchanged StageEvent format and actual base resume semantics, while persisting outer history/T. Embedded legacy Continuation remains identical; new checkpoint C7/T9 includes the complete false invocation. The second call reaches C13/T15 with the same correct final economic state as direct recovery C13/T13. Full documentary outputs are included. Reading/publishing the actual file, not memory, is necessary to preserve the extra2 attempts.

An admitted recover-unknown event with outcome_known:false is also supplied. Its normal waiting report/updated checkpoint preserve boundary and residual duty with attempted2, C7/T9, no successful guard/body segment. The Boolean is unverified local input. Timeout, silence or an unknown outcome does not by itself prove nonexecution or justify external refund.

## Argument/operation matrix proposed

| Mode with `--recovery-module` | Required inputs | Checkpoint output | Excluded alternative operation inputs |
| --- | --- | --- | --- |
| inspect | base source and companion | none | target/native/signed flags |
| start | action, scenario; optional proposed logical cap | required | reopen/resume/recover/event/recovery-event |
| reopen | one complete recovery checkpoint | none; read-only | action/scenario/event/recovery-event/state-changing operation |
| resume | complete recovery checkpoint and normal StageEvent | required | action/scenario/reopen/recover/recovery-event |
| recover | complete recovery checkpoint and RecoveryEvent | required | action/scenario/reopen/resume/normal event |

No mode accepts reviewer metadata as a public prerequisite. Trace/target/signed/native paths do not get a recovery pass. Exact order, missing/conflicting-argument codes, strict decoder rejections and authentic state provenance remain owner decisions; Sol does not supply those checks or hostile tests. Old `--resume` without a companion continues to read the old Core/2 format. Selecting a companion explicitly selects the new durable recovery format; no bare-old-token migration is silently attempted.

## Public `mori test`

```bash
mori test examples/custody-refund
mori test examples/forward-compensation
```

The supplied `mori-tests.json` describes proposed profile `moriarty-tests/recovery/1`. Existing `moriarty-tests/1`, `/2` and disjoint-draft dispatch stay unchanged. A new branch of the actual existing test command handles only fixed language operations inspect/start/reopen/resume/recover with source/module, scenario/event, checkpoint input/output and full expected report paths. It never executes arbitrary manifest shell commands or dispatches providers/models/native work.

Each case includes `process_isolation:"fresh-process-per-operation"`. The proposed actual consumer must invoke the real same public `mori` executable for each listed operation, terminate it, read its actual published bytes, and feed those actual bytes to the next independent operation. It cannot use expected checkpoint content as a next input or keep a library Process alive and label that a fresh reopen. This is a bounded ordinary local test feature in the existing runner, not a daemon/scheduler/framework. Physical process/IO/resource bounds require owner review before execution.

The complete test manifest files cover inspect, start/publication, fresh reopen, recovery/publication and terminal reopen. Optional false-observation manifests must compare full history/T as well as financials. Entire reports, checkpoint bytes, source/IR/plan/trace, stdout/exit/stderr and failed cases must be retained under applicable independent freezes. No one-field assertion or final-balance comparison completes the case. Aggregate output remains a separate consumer with existing bounded complete reporting; public invocation byte goldens do not transfer automatically to it.

The documentary manifest contains identity markers via expected files. It is not a runnable current fixture or a passing test result. First delivery cannot be called complete until real public operations meet the newly independent concrete freeze after the owner decisions. Target proving/ledger/live consumption remain separate and required.
