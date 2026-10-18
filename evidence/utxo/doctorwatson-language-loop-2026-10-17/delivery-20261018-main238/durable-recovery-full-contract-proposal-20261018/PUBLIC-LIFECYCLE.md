# Real public paired-source lifecycle required in the first delivery

All commands here are proposed and UNRUN. Current main has no recovery companion/record route. Normal first-delivery success requires actual implementation of these public paths together, not a report viewer, mocked validator or library Process kept alive.

## Public command matrix

| Mode | Concrete public call | Actual inputs | Required ordinary output |
| --- | --- | --- | --- |
| Inspect | `mori inspect base.mori --recovery-module recovery.mori` | Actual paired source | Complete paired base Program/filled plan, recovery module/arenas/handlers/captures and path-cost plan |
| Start | `mori simulate base.mori --recovery-module recovery.mori --action fund --scenario start.scenario.json --logical-cap 13 --checkpoint-out scratch/pending.json` | Actual source pair and starting scenario | Actual pending or forward-complete complete checkpoint published and full report printed |
| Reopen | `mori simulate base.mori --recovery-module recovery.mori --reopen scratch/pending.json` | Actual source pair plus actual complete saved checkpoint | Event-free whole reopened state/report, C/T/history/effects/control unchanged; no language invocation |
| Normal resume | `mori simulate base.mori --recovery-module recovery.mori --resume scratch/pending.json --event normal-false.event.json --checkpoint-out scratch/observed.json` | Actual opened record and unchanged-format StageEvent | Actual updated complete pending/forward-complete record and baseline history |
| Recover | `mori simulate base.mori --recovery-module recovery.mori --recover scratch/pending.json --recovery-event recover.event.json --checkpoint-out scratch/closed.json` | Actual opened record, typed recovery event selecting actual handler | Actual recovery-complete record, or normal not-taken pending record, with whole history/C/T/effects/residual |
| Terminal reopen | `mori simulate base.mori --recovery-module recovery.mori --reopen scratch/closed.json` | Actual closed file and retained source pair | Complete closed state including residual duties and original history; no manufactured pending suffix |
| Test | `mori test examples/custody-refund` | New manifest, actual source/event/scenario files and independent expected files | Complete aggregate of actual public fresh-process results and comparisons |

For compensation the start action is purchase. No command depends on project reviewer metadata, provider credentials or maintainer permission. Source/module inspect is local source processing; it must not dispatch target compilation or conceal unsupported target/native behavior. Exact argument refusals and ordering remain designated-owner work. Normal operation selectors are mutually exclusive; start requires action/scenario, reopen requires no event or checkpoint output, state-changing paired modes require a new checkpoint output. Old no-companion CLI behavior stays unchanged, including its bare-token event-driven resume.

## Actual reader→transition→writer→fresh reader connection

The source pair is read through the real CLI. `load_pair` constructs the actual pair/plan and the designated-owner combined admission result. Source-only inspect renders that whole result. Start delegates to the actual paired language operation and retains its actual normal baseline start output inside the new record. It must not separately reconstruct a financial state from expected values.

For every reopen/resume/recover, the CLI bounded reader consumes the actual checkpoint file and the actual decoder/source-bound reopening API. Reopen returns that complete selected state without evaluating a segment. Resume/recover then consume that opened state in the actual language engine. No unreviewed automatic old-token migration, scenario override or host-side sum is part of the ordinary path.

The resulting actual Transition supplies a complete checkpoint and invocation. Serialization operates on that actual checkpoint; the real local file publication operation receives those bytes; complete stdout comes from the actual report of the same transition. On the normal successful path, the published file's parsed checkpoint equals the report.checkpoint in every field. A later new process reads that actual file. Exact low-level algorithm, durability/commit/write/print order and all error/crash dispositions are O-IO/O-FAIL owner outputs, not chosen by this ordinary design. Public success cannot claim a persisted state when actual publication did not complete. That positive predicate does not imply a failure means no financial work.

Actual normal files must retain full state/rows, source/captures, prefix effects/fees/duties, C/T, complete observation history and control. Checkpoint publication is mandatory for paired transitions. Existing files are not labelled globally revoked by creation of a new closed record. Live once-only/old-copy/race/late-event behavior is owner/target work, not supplied by file names or local labels.

## Exact recommended ordinary library linkage

These are API contracts, not code or adopted Rust definitions:

- `load_pair(base_source:&str,recovery_source:&str) -> Result<RecoveryProgram,RecoveryLoadFailure>`
- `inspect_pair(program:&RecoveryProgram) -> RecoveryProgramReport`
- `start(program:&RecoveryProgram,action:&str,scenario:Scenario,cap:Option<UInt>) -> Result<RecoveryTransition,RecoveryFailure>`
- `reopen(program:&RecoveryProgram,checkpoint_bytes:&[u8]) -> Result<ReopenedRecovery,RecoveryFailure>`
- `resume(program:&RecoveryProgram,opened:ReopenedRecovery,event:StageEvent,cap:Option<UInt>) -> Result<RecoveryTransition,RecoveryFailure>`
- `recover(program:&RecoveryProgram,opened:ReopenedRecovery,event:RecoveryEvent,cap:Option<UInt>) -> Result<RecoveryTransition,RecoveryFailure>`
- `checkpoint_bytes(checkpoint:&RecoveryCheckpoint) -> Result<Vec<u8>,RecoveryExportFailure>`
- `outcome_report(transition:&RecoveryTransition) -> RecoveryReport`
- `reopen_report(opened:&ReopenedRecovery) -> RecoveryReport`

RecoveryTransition retains the actual checkpoint and actual invocation; ReopenedRecovery exposes its actual complete checkpoint. Neither constructor is a public proof certificate. RecoveryFailure, RecoveryLoadFailure and RecoveryExportFailure denote mandatory designated-owner concrete results, not invented old StageRejection wrappers or lost partial effects. B would move these methods onto an owned Process but must return/serialize the same complete records and meet the same public flow.

The implementation belongs in the existing mori-core staged language concern and existing mori inspect/simulate/test consumers. Source pair parsing/planning, versioned DTO/reader, actual normal transition integration, report rendering and public file/process integration need exact candidate reviews. There is no new verification framework, scheduler, campaign or application-specific recovery helper.

## Public `mori test` is part of this first delivery

Each supplied `mori-tests.json` uses proposed profile moriarty-tests/recovery/1, with six isolated cases: direct recovery, false normal observation then recovery, direct normal forward completion, false normal observation then normal forward completion, not-taken recovery then successful recovery, and not-taken recovery then normal forward completion. Each begins from a new actual start, so no record is reused for competing terminal branches. Existing profiles remain unchanged.

Operations are fixed inspect/start/reopen/resume/recover, not arbitrary commands. Each operation runs the same actual public executable in a new OS process. A case has its own actual scratch files; the next operation consumes its prior actual published path. Expected JSON never becomes a checkpoint input. The runner terminates each process, retains full stdout/stderr/exit/report and actual read/published records, compares entire expected reports, and preserves operation/case order and whole failures in the aggregate. It cannot replace this with repeated library calls to a retained Process and claim fresh-process reopen. Owner must supply resource/file/path/child-process envelopes and exact error retention rules before execution.

Recommended aggregate profile is moriarty-local-recovery-test-report/1. Root fields: format, qualification, authentication, consumption, status, cases. Each case retains name, matched and complete ordered operations. Each operation retains operation, report, checkpoint_in (full actual record or null), checkpoint_out (full actual record or null), matched and process:{exit_code,stdout,stderr}. status is passed or failed for normal complete comparisons; exit0 iff all complete comparisons match, exit1 for mismatches. This is an ordinary comparison result, not security/ledger success. Exact process failures and bounded aggregate handling remain owner-dependent and must not truncate a case, prefix or failure into a passing summary. Aggregate byte conformance has its own freeze; per-invocation bytes do not transfer.

## Qualified completion boundary

Finite first-delivery evidence must include all normal paths above and every designated-owner scoped non-normal outcome after that owner's full rules/tests and independent reviews exist. Local authenticated/once-only/ledger claims remain unavailable unless separately established. Full original joins, private composition, late success/recovery races, per-stage consent, mandatory proofs, lowering/ledger phases, all financial/conformance/target/release requirements remain mandatory with separate evidence. No normal-only local implementation, doc test profile, design vote or merge waives them.

## Exact B alternative surface for the decision comparison

B would expose `RecoveryProcess::start(program:RecoveryProgram, action:&str, scenario:Scenario, cap:Option<UInt>) -> Result<RecoveryProcess,RecoveryFailure>` and `RecoveryProcess::reopen(program:RecoveryProgram, checkpoint_bytes:&[u8]) -> Result<RecoveryProcess,RecoveryFailure>`. Its instance operations would be `resume(&mut self,event:StageEvent,cap:Option<UInt>) -> Result<Invocation,RecoveryFailure>`, `recover(&mut self,event:RecoveryEvent,cap:Option<UInt>) -> Result<Invocation,RecoveryFailure>`, `checkpoint(&self) -> &RecoveryCheckpoint` and `report(&self) -> RecoveryReport`. Non-normal mutation/retention behavior still requires the exact owner contract; these signatures supply no failure policy. Each CLI invocation must still open actual bytes into a new Process and publish the same complete actual record before a later process reopens it. No Process object may cross the OS-process boundary in the public test. Thus B changes API ownership and reconstruction, not required data or acceptance. A is recommended for the public contract.
