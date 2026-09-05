## ADDED Requirements

### Requirement: T6S001 complete adversarial inventory

The Task6 suite SHALL retain all 27 named semantic triples, one latest-field
substitution triple, five locator triples, JSON/carrier/path controls and seven
actual-package controls. The final suite SHALL have no skips or deselections.

#### Scenario: Complete final execution

- **WHEN** the complete checker and utility suite executes after native intake
- **THEN** all 115 planned test instances execute, including the package instance
  that runs all seven package mutations
- **AND** the test count alone does not replace evidence for individual controls

### Requirement: T6S002 effective rebound semantic controls

WHEN a semantic mutation is generated, the test SHALL establish an effective
logical and serialized change and successful complete raw linkage before
counting semantic rejection. Resource, parser, hash and process failures SHALL
not count as semantic mutant kills.

#### Scenario: Ineffective or malformed mutation

- **WHEN** a mutation changes no relevant field or fails before semantic comparison
- **THEN** the triple fails instead of reporting a killed mutant

### Requirement: T6S003 corrected and unrelated positive controls

After each rejected mutant, the test SHALL accept a corrected counterpart and
an unrelated honest case through the same bounded semantic and provenance APIs.

#### Scenario: Reject-all checker

- **WHEN** a checker rejects every supplied case
- **THEN** its positive control fails and the mutation triple cannot pass

### Requirement: T6S004 bounded sequential execution

Synthetic triples SHALL execute in separate sequential child processes using
incremental files and one expected case history at a time. Each child SHALL be
reaped before the next triple starts. Native arrays SHALL not be loaded whole.

#### Scenario: Completed synthetic triple

- **WHEN** one triple terminates
- **THEN** the parent retains its paths, report, arguments, original streams and
  terminal record without retaining the child's case objects

### Requirement: T6S005 native package prerequisite

Final acceptance SHALL require separately admitted native shards, case files,
full admission, manifest, original receipts and exact final source pins.

#### Scenario: Missing native package or stale test pins

- **WHEN** native files are absent or the admission predates the final test bytes
- **THEN** actual package admission fails without skipping, self-admitting or
  substituting synthetic files

### Requirement: T6S006 complete package mutation controls

The seven package controls SHALL start from admitted native inputs and separate
small manifest/admission copies. After each rejection, the checker SHALL replay
all 78 unchanged shards again without a semantic-result cache.

#### Scenario: Required package controls

- **WHEN** missing-case, duplicate-case, extra-case, missing-transitive-pin,
  changed-frozen-pin, missing-input or changed-receipt is applied
- **THEN** the changed package is rejected
- **AND** the corrected complete package passes a fresh 78-shard traversal
- **AND** no original native file is modified

### Requirement: T6S007 original behavioral evidence

The recorder SHALL retain exact source/runtime bytes, parent and child arguments,
streams, terminal results, mutation identity and generated control artifacts.
The temporary raw-field omission SHALL import and fail behaviorally; its exact
restoration SHALL precede final validation and native exports.

#### Scenario: Raw-field omission control

- **WHEN** the raw-profile equality guard is disabled under a controlled source window
- **THEN** the substitution test fails because the expected rejection did not occur
- **AND** infrastructure failures are retained separately rather than counted as RED

#### Scenario: Native export source stability

- **WHEN** a recorded producer command is active
- **THEN** checker and test source bytes remain frozen

The experimental implementation plan is adopted at `cd06756`. These requirements
specify future runtime gates; structural validation does not establish a pass.
