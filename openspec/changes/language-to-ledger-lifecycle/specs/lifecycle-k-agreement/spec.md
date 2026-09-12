## ADDED Requirements

### Requirement: Executable supported Core in K
K SHALL execute the admitted loan lifecycle constructors, financial PRE/POST distinctions, staged publication, explicit rounding and period history. The implementation SHALL extend existing bounded semantics and loaders. It SHALL not replace missing behavior with a host-computed result, constant flag or unchecked projection. Unsupported Core SHALL reject explicitly.

#### Scenario: Actual K execution
- **WHEN** the source lifecycle is elaborated and passed through the checked Core-to-K loader
- **THEN** the pinned K runtime SHALL produce the complete expected successful states and effects.

### Requirement: Complete differential observations
Independent arithmetic, TypeScript evaluation and K SHALL agree on all material state, ordered effects and carried work. Cases SHALL include both rounding modes, duplicate/reversed periods, funding reuse, wrong units, overflow, failed postconditions, work exhaustion and residual debt after partial payment. The corpus SHALL contain a deliberately wrong expected transition that the comparator rejects.

#### Scenario: Incomplete comparison
- **WHEN** final outstanding matches but allowances, principal/accrued split, period cursor, effects or work differ
- **THEN** the differential check SHALL fail.

### Requirement: Explicit proof scope and bounded execution
Evidence SHALL record pinned K/backend identity, loader source, commands, inputs, outputs and resource bounds. Differential agreement SHALL not be labeled a correspondence proof. Required metatheorems and compiler/ledger claims SHALL remain open until discharged. A toolchain or campaign limit SHALL stop only its dependent execution.

#### Scenario: Missing K binary
- **WHEN** K cannot compile or run within the existing admitted toolchain/resources
- **THEN** the stage SHALL remain incomplete with the failed command and concrete blocker; TypeScript tests SHALL not replace it.
