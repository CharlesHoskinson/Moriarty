## ADDED Requirements

### Requirement: UA001 exact alias relation

BEFORE and after each command, validation SHALL require the selected main
environment's bin/python3 path to be a symlink with exact target text python,
resolving to the admitted interpreter. It SHALL not use an alias allowlist.

#### Scenario: Any other alias fails

- **WHEN** alias kind, target text or resolved interpreter differs
- **THEN** the command sequence stops and retains the actual observation

### Requirement: UA002 unchanged identity checks

WHEN checking the uv child probe, only the expected executable spelling SHALL
change to the exact verified python3 alias. Prefix, resolved interpreter,
version, offline controls and archive checks SHALL remain unchanged.

#### Scenario: Same interpreter with verified lexical identity

- **WHEN** the exact alias and all identity fields match
- **THEN** the unchanged requested test command may execute
- **AND** the prior failed probe remains distinct from a pytest failure

### Requirement: UA003 new receipts preserve old evidence

WHEN retrying, receipts SHALL pin the two new companion sources, separate alias
dispatch and complete original failed-probe evidence. Old sources, supplement
archives and command receipts SHALL not change.

#### Scenario: No rewritten failure or runtime

- **WHEN** the alias supplement is adopted
- **THEN** no new runtime prepare occurs and both prior failures remain intact
- **AND** fresh exclusive stages distinguish the corrected alias invocation

### Requirement: UA004 unchanged complete sequence

WHEN the alias sequence runs, collection SHALL match all 441 original ordered
IDs, runtime SHALL pass all 441 exact JUnit cases, and Core53 SHALL run only
after both pass. Any mismatch SHALL stop without implicit retry.

#### Scenario: No partial suite acceptance

- **WHEN** any command, source, fixture or runtime check fails
- **THEN** later commands remain unexecuted and original outputs are preserved

### Requirement: UA005 no expanded authority

WHILE applying this amendment, the implementation SHALL not change tests or
requested child argv, bypass uv, prepare another runtime, launch native jobs
or close expanded A4, final unrestricted suite or full A5 gates.

#### Scenario: Alias correction is a bounded prerequisite

- **WHEN** the corrected sequence passes
- **THEN** complete Task2 still requires independent source and evidence review
- **AND** all later native and acceptance obligations remain required
