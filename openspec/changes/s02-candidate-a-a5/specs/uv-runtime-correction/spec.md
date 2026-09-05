## ADDED Requirements

### Requirement: UV001 retain the real executable

WHEN the unchanged historical test requests uv, the retry SHALL execute the
retained real uv0.12.5 executable through a private single-entry bin directory.
It SHALL not substitute a behavioral shim or broaden PATH to the user's bin.

#### Scenario: Exact executable discovery

- **WHEN** the original test invokes its unchanged uv run python argv
- **THEN** the executable matches the independently pinned real uv bytes
- **AND** its loaded libraries, private copy and supplement archive are retained

### Requirement: UV002 select the admitted offline environment

WHILE uv runs, the launch SHALL disable sync, network, Python downloads,
env-file and user configuration loading, and SHALL select the already-admitted
main Python environment rather than the worktree's distinct virtual environment.

#### Scenario: No dependency acquisition or environment replacement

- **WHEN** the retry starts
- **THEN** controlled UV variables select the pinned main interpreter and prefix
- **AND** the original Python/runtime archives remain unchanged
- **AND** no package sync, download or lock rewrite is authorized

### Requirement: UV003 verify actual child identity

BEFORE each retry command, the launcher SHALL retain an offline no-sync uv
child identity probe and compare its executable, prefix, resolved interpreter
and version with the admitted runtime. A mismatch SHALL stop before the test.

#### Scenario: Probe mismatch stops the sequence

- **WHEN** uv chooses any other Python environment
- **THEN** the launcher records the actual probe result and fails
- **AND** pytest or Core comparison is not executed

### Requirement: UV004 preserve original command evidence

WHEN a retry command runs, it SHALL retain original streams, parent and nested
argv, named controlled environment keys, source/runtime archives, before/after
byte checks and terminal status. Original failed receipts SHALL not change.

#### Scenario: Failure and corrected environment remain distinguishable

- **WHEN** the supplement is admitted
- **THEN** the original missing-uv failure and its JUnit are separately pinned
- **AND** the real supplement has exact regular archive membership
- **AND** no shared helper, original capture, test or fixture is edited

### Requirement: UV005 retain complete sequential coverage

WHEN the corrected sequence runs, fresh collection SHALL equal all 441 prior
ordered node IDs, and runtime SHALL pass all 441 with exact set equality and
no skip or exclusion. Core53 SHALL run only after both gates pass.

#### Scenario: Any mismatch prevents later admission

- **WHEN** collection, runtime, source or environment differs
- **THEN** the sequence stops with original evidence retained
- **AND** no automatic retry or partial Core acceptance is reported

### Requirement: UV006 later gates remain mandatory

BEFORE complete A4 or A5 admission, all expanded A4 tests, the actual 78-case
package and final unrestricted suite SHALL still pass their separate gates.

#### Scenario: Historical retry is a bounded prerequisite

- **WHEN** the original 441 tests and Core53 pass
- **THEN** that result alone does not admit Task2, its pilot, or full A5
- **AND** the remaining corpus, equivalence, samples and reviews remain required

The concrete experimental plan is
`docs/superpowers/plans/2026-09-05-candidate-a-uv-runtime-correction.md`.
This is an execution-environment correction, not a semantic or test-coverage change.
