# Candidate A RH002: measured native invocation evidence

Status: independently reviewed plan with the two required clarifications below.
No native pilot is dispatched.
This refines only the RH002 proposal adopted with RH001 at9cea581. RH001 is now
admitted atb08a2da. The earlier proposal's presence-only completeness fields are
insufficient: a present but empty/truncated resource file must remain incomplete.

## Scope and source ownership

Create receipt-only runner.py and test_runner.py under
`evidence/s02-candidate-a-completion/a4/native-resources/`.
They do not enter the semantic source_pins map. Each invocation archives the
exact runner bytes, launch arguments, original output, GNU time resource output,
terminal record and before/after interpreter/time/runner hashes. Root externally
pins those receipt files. The inner recorder remains unchanged at RH001; its
own original source/runtime/argv receipt is still independently validated.
Do not modify any checker, exporter, Quint model, runtime helper, or core test.

This is a bounded local implementation/test unit before a native pilot. Tests
use short Python children, not Quint or a fabricated Candidate A case. A passing
wrapper unit does not establish native export feasibility or semantic acceptance.

## EARS contracts and OpenSpec scenarios

### RH003: Exclusive invocation evidence

WHEN a capture starts, the runner SHALL require a positive integer wall budget,
a nonempty string argv, an absent dedicated receipt directory and an absent
inner receipt. It SHALL create every output exclusively. It SHALL preserve
original launch failures as terminal evidence, without fabricating an inner
receipt. A reused or symlinked receipt destination SHALL fail before launch.

Scenario: capture into an existing directory or with an existing inner receipt
fails without executing the command; originals remain byte-identical. Invalid
boolean/zero/negative wall budgets fail without creating the destination.

### RH004: Pinned environment and measured scope

WHEN the runner dispatches, it SHALL verify the already-admitted /usr/bin/time
SHA25634ba90f8199989d88f2f02d40ddcee60be27a7f361db9fb7c100578cc8d7bf1e,
pin its own source and resolved interpreter before/after, record the original
parent argv and exact child argv, and preserve its own source bytes. It SHALL
remove PYTHON*, PYTEST*, NODE_* and LD_* overrides without recording their values.
It SHALL set PYTHONNOUSERSITE=1, PYTHONDONTWRITEBYTECODE=1,
NODE_DISABLE_COMPILE_CACHE=1 and LC_ALL=C. PATH remains the admitted task PATH;
the inner recorder resolves and validates its actual tools independently.

Scenario: a short child confirms poison override keys are absent and fixed keys
are present; launch metadata lists removed names only, not secret values.
Mismatch in the expected GNU time executable fails before subprocess execution.

The exact command is /usr/bin/time -v -o <resources.txt> followed by the unchanged
recorder argv. It runs in a new process group. The metric is GNU time maximum
resident set size for the whole recorder/native-descendant invocation, in KiB;
it is NOT simultaneous Node-plus-Rust RSS or evaluator-only RSS. Native Node's
existing default heap is unchanged. Default wall budget is900seconds.

### RH005: Strict resource-content validation

WHEN resource output is evaluated for completeness, the runner SHALL require
the complete C-locale GNU time verbose field inventory exactly once, an intact
command line equal to GNU time's quoted space-joined unchanged recorder argv,
valid nonnegative numeric counters/times, a positive maximum-RSS
value and a parseable terminal exit status. Missing, duplicate, unexpected,
nonfinite, negative or malformed fields SHALL fail completeness. Known leading
GNU time nonzero/signal diagnostic lines may be preserved but SHALL not turn a
failed command into a successful measurement. A resource file larger than64KiB
SHALL be rejected as a resource-format failure without a whole-file read.

Scenario: a successful short process produces parseable original GNU time
output with a positive maximum RSS. Independent malformed/truncated/duplicate/
negative/nonfinite fixtures and a valid23-field report with substituted command
text fail. A genuine test against the original proposed
presence-only resource check fails before the strict parser implementation.

The complete inventory is the installed verbose output's23fields: command,
user/systemseconds,CPUpercentage,walltime,average shared/unshared/stack/total
memory,maximumRSS,averageresidentset,major/minorfaults,voluntary/involuntary
switches,swaps,filesysteminputs/outputs,socketssent/received,signals,pagesize,
exitstatus. Confirm actual field names in the retained short GNU time control;
do not guess a localized or truncated variant. The independent review must
resolve any mistaken field count before implementation dispatch.

### RH006: Timeout and owned-process cleanup

WHEN the wall budget expires, the runner SHALL mark timeout, signal SIGTERM to
the owned process group, wait at most5seconds for group exit, then send SIGKILL
to that group even if its leader has exited. It SHALL bound leader reaping and
the separate group-absence check at5seconds each and record actual cleanup
completion. It SHALL never claim successful cleanup merely from leader exit.

ON every post-spawn termination path, including early leader exit and monitor
exception, the runner SHALL check the owned group and apply the same bounded
cleanup if it remains. Forced cleanup SHALL make measurement ineligible even
when the group eventually disappears. A cleanup exception SHALL preserve a
terminal incomplete result; it SHALL not erase the original failure.

Scenario: a short leader exits on SIGTERM while a child ignores SIGTERM. The
test proves the group SIGKILL path executes and the child cannot continue; a
remaining zombie/group is recorded as cleanup-incomplete, not hidden. A separate
simple timeout test returns124 with incomplete resource/inner evidence. Tests
may shorten grace/cleanup intervals explicitly; real capture defaults stay5s.
An independent control covers a normally exited leader with a live descendant,
without waiting for the command timeout; it requires cleanup and ineligibility.

Fixed native commands must not daemonize or escape their group. This is not a
PID-namespace isolation claim or a guarantee for detached hostile descendants.
No host-wide process scans/signals or unrelated process termination is allowed.

### RH007: Evidence status and failure taxonomy

WHEN capture terminates, it SHALL retain stdout/stderr/resource/launch/runner
hashes, elapsed wall time, actual child exit, timeout/launch-error status,
group-cleanup status, inner receipt presence/hash and source/runtime stability.
It SHALL not label an inner receipt complete from presence alone. The terminal
record SHALL state that independent inner receipt validation is still required.

Resource-format completeness requires a valid full report and no timeout.
Wrapper eligibility additionally requires outer exit0, resource exit0, stable
pins, no forced cleanup or lingering group and a present inner receipt. Eligibility is NOT native
acceptance: root separately validates the original inner receipt's argv, exact
source/runtime closure, terminal status, full raw shard, witnesses and invariants.

Scenario: a nonzero child retains actual exit7 and fails eligibility even when
resource output is structurally complete. A child that produces no inner
receipt is ineligible despite outer exit0. Timeout and malformed resource
output are resource/incomplete evidence, never semantic counterexamples.

The CLI returns124 on timeout; otherwise2 for unstable/incomplete evidence;
otherwise the actual nonzero command status (negative signal codes normalized
to128+signal), or0 only when eligible. terminal.json is externally pinned and
has no self-hash. Launch errors use explicit type/message and return2.

### RH008: Complete local controls and original evidence

WHEN this unit is proposed for admission, it SHALL retain the genuine
presence-only completeness RED, unchanged failing test at GREEN, full local
test collection and terminal outputs, runner/test before/after source bytes,
exact interpreter/time/runtime references, and all subprocess artifacts under
an exclusive pytest basetemp. No test or subprocess artifact is deleted.

Required controls: original completeness RED; honest measured child; exact
sidecar hash verification; absent inner receipt; nonzero child; malformed,
truncated, duplicated and nonfinite resource records; exclusive targets; invalid
wall types; environment cleanup; tool-pin mismatch; simple timeout; leader-exit
plus SIGTERM-ignoring child both on timeout and ordinary early exit; substituted
resource command; bounded large resource-file rejection. Source
tampering can be a deterministic pin-check unit, not actual concurrent editing
of the running script. No native or package test exclusion counts as acceptance.

## Implementation sequence and admission

1. Independently review this plan and the installed GNU time23-field inventory.
   Record corresponding OpenSpec RH003–008 scenarios on main.
2. Add local tests plus the minimal proposed presence-only resource predicate.
   Preserve one actual assertion RED on a present empty resource file.
3. Implement strict resource parsing and bounded capture/CLI. Keep the tests
   unchanged except additional planned controls; preserve all original bytes.
4. Run the complete local test module under admitted Python with -B/fresh-X
   and an exclusive basetemp; record exact test names/count and all outputs.
   Use a small dedicated receipt recorder, not the active A4 semantic recorder
   while the exporter edits its pinned sources. Archive that recorder too.
5. Obtain nonauthor source/evidence review, independently check the original
   archives/runtime/terminal results, and commit only this root-owned unit.
6. Only after exporter and Task6 source admission, separately dispatch the
   largest-case native pilot with explicit unchanged recorder argv, wall900,
   root-pinned outer receipts and independent complete native validation.

Any discovered process or resource parsing gap needs a retained failing control
and reviewed correction. No successful pilot, native package, model-checking
gate, cross-provider Council result or Foreman repair is implied by this plan.

Independent plan review: a0_final_review confirmed all23installed fields and
required command-content binding plus cleanup on every post-spawn path. Both
requirements are incorporated above. This is a local plan review, not Council.
