## ADDED Requirements

### Requirement: NW001 literal native driver binding

WHEN a native wrapper is generated, the renderer SHALL preserve the shared
driver body except its module name, case-table import and two typed pure case
and ordinal bindings. The shared template SHALL remain in the complete source
closure through the static aggregate. No authority/action body SHALL be specialized.

#### Scenario: Every generated wrapper matches its template

- **WHEN** the renderer validates all 78 final wrappers
- **THEN** reversing only the declared header/import/binding substitutions
  reproduces the shared template exactly for every wrapper
- **AND** wrapper032 binds ordinal32 in final accepted source

### Requirement: NW002 exact observed native variable order

WHEN native ITF metadata is read, the producer and checker SHALL require exactly
`authorityState,caseIndex,cursor,latestEvent` in that order. The state field set
and its meanings SHALL remain unchanged.

#### Scenario: Observed native order is accepted

- **WHEN** the actual native-order fixture is checked against the old declaration-order expectation
- **THEN** the two small honest-shard tests fail at the exact raw-vars predicate
- **AND** after the sole expectation correction all existing 79 tests pass
- **AND** this transport control is not counted as a semantic mutant kill

### Requirement: NW003 renewed complete wrapper verification

After literal wrappers replace parameterized instances, the producer SHALL repeat
aggregate closure/size preflight and native typecheck coverage for all 78 final
wrappers. The prior one-driver parse measurement SHALL not describe the new body layout.

#### Scenario: Final producer validation

- **WHEN** the corrected driver suite is admitted
- **THEN** both intended verification and ordinal controls have original behavioral failures
- **AND** all 12 corrected Quint tests and all generated-wrapper comparisons pass
- **AND** the largest-case pilot and final native exports remain separately required

The amendment is adopted experimentally at `4465863`, based on retained native
toy-probe evidence. The toy probe is not Candidate A lifecycle or package evidence.
