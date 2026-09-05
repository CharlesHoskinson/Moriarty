## ADDED Requirements

### Requirement: RH003 exclusive invocation evidence

WHEN capture starts, the runner SHALL require a positive integer wall budget,
nonempty string argv, absent receipt directory and absent inner receipt. It
SHALL create outputs exclusively and preserve launch failures without inventing
inner evidence.

#### Scenario: Existing outputs prevent execution

- **WHEN** a receipt destination or inner receipt already exists
- **THEN** launch fails before command execution and originals stay unchanged
- **AND** invalid boolean or nonpositive budgets also fail before creation

### Requirement: RH004 pinned measured invocation

WHEN dispatching, the runner SHALL verify admitted GNU time bytes, pin its own
source and resolved interpreter before and after, preserve parent/child argv,
sanitize Python/Pytest/Node/loader overrides and set C locale. It SHALL measure
the unchanged recorder invocation in a new owned process group.

#### Scenario: Metric scope is explicit

- **WHEN** the original resource report is retained
- **THEN** maximum RSS is labeled GNU time KiB for the recorder and descendants
- **AND** it is not described as evaluator-only or simultaneous combined RSS
- **AND** removed environment names are retained without their secret values

### Requirement: RH005 strict resource report

WHEN resource completeness is evaluated, the runner SHALL require all 23
installed C-locale verbose fields exactly once, valid finite nonnegative
numeric fields, positive maximum RSS and exact command-content binding to
the unchanged recorder argv. Reports over 64KiB SHALL be rejected boundedly.

#### Scenario: Present invalid data is incomplete

- **WHEN** a report is empty, truncated, duplicated, malformed or nonfinite
- **THEN** it fails resource completeness
- **AND** a structurally valid report with substituted command text also fails
- **AND** a genuine presence-only RED is retained before implementation

### Requirement: RH006 bounded cleanup on every termination path

WHEN timeout or any post-spawn termination occurs, the runner SHALL check its
owned group and apply bounded cleanup if needed. It SHALL not confuse leader
exit with group exit. Timeout cleanup SHALL send TERM, wait at most five
seconds, then send KILL regardless of leader status; leader reaping and group
absence checks SHALL be bounded separately. Forced cleanup SHALL be ineligible.

#### Scenario: Leader exits while child remains

- **WHEN** a leader exits normally or on TERM with a live TERM-ignoring child
- **THEN** the owned child receives bounded group cleanup
- **AND** a remaining zombie/group is recorded as incomplete, not concealed
- **AND** no PID-namespace or detached-descendant isolation is claimed

### Requirement: RH007 eligibility is not semantic acceptance

WHEN capture terminates, it SHALL retain original sidecar hashes, actual exit,
timeout and launch-error status, elapsed time, cleanup result and stable pins.
It SHALL not label inner evidence complete from presence alone. Wrapper
eligibility SHALL require zero exit, valid zero-exit resources, stable pins,
no forced cleanup or remaining group, and a present inner receipt.

#### Scenario: Independent native validation remains required

- **WHEN** a wrapper result is eligible
- **THEN** root still validates the complete inner source/runtime/argv receipt
- **AND** actual raw output, witnesses and invariants remain required
- **AND** resource failures are never semantic mutation kills or counterexamples

### Requirement: RH008 complete local controls

WHEN this unit is admitted, it SHALL retain original behavioral RED/GREEN,
the complete local test inventory, source/runtime bytes and all short-child
artifacts in an exclusive basetemp. It SHALL cover malformed report, exclusive
path, budget, environment, pin, nonzero, missing-inner and both cleanup controls.

#### Scenario: Local tests do not replace the native pilot

- **WHEN** the wrapper tests pass
- **THEN** largest-case native feasibility and the full package remain open
- **AND** no native command is authorized until its other prerequisites pass

Concrete plan: experimental
`docs/superpowers/plans/2026-09-05-candidate-a-native-resource-runner.md`.
The independent plan review required all-path cleanup and exact command binding.
