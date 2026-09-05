## ADDED Requirements

### Requirement: OP001 complete historical Python inventory

WHEN A5 Task2 runs its original full Python regression checkpoint, the runner
SHALL select all 23 unchanged test files from commit
`28d35d82d9f404844d05e0e6bd0ce2dc86b8c311` by exact path and SHA256. It SHALL
retain the unchanged pytest configuration and use no test filters or skips.

#### Scenario: Exact original coverage is retained

- **WHEN** the historical suite is collected and executed
- **THEN** fresh collection contains 441 unique node IDs
- **AND** all 441 runtime JUnit cases pass and match the collected names
- **AND** no historical node-ID receipt is invented
- **AND** any count, source, configuration or result mismatch fails the checkpoint

### Requirement: OP002 original fixtures and independent Core comparison

WHEN the historical checkpoint runs, its recorder SHALL preserve the admitted
2673-path fixture selection at current original bytes, external fixtures, all
14 original ITFs and the complete 53-case input. Source, input and runtime pins
SHALL remain unchanged during each recorded command.

#### Scenario: Complete independent comparison

- **WHEN** the original Core comparison executes
- **THEN** all 53 records are compared without allow-subset
- **AND** a successful terminal result and complete source/input archive are retained
- **AND** new A4 sources are not substituted for original fixtures

### Requirement: OP003 expanded gates remain mandatory

BEFORE full A4 or A5 admission, the expanded A4 tests, all 78 native cases,
1557-event package replay and final unrestricted Python suite SHALL pass with
their complete fresh source/input closures.

#### Scenario: Historical checkpoint does not admit later work

- **WHEN** the original 441-test checkpoint passes
- **THEN** it does not admit the new A4 tests or missing native package
- **AND** the actual-package test remains required with no skip
- **AND** a pilot still requires completion and review of its other prerequisites

The exact runner and source inventory are adopted experimentally at `9cea581`.
This specification changes sequencing, not formal semantics or final coverage.
